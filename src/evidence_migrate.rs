//! Mapper 3 of `chough migrate` (REQ-records-009): each 0.5.x observation and
//! validation run becomes an evidence entry of the open session that references it, a
//! receipt of the archived story or bug that references it, or is removed with why.
//!
//! - Placement: an open session's reference wins, and of several open sessions the one
//!   whose new ID sorts first holds the entry; the others resolve it by its alias. A
//!   reference from an archived session or a bug makes a receipt (REQ-history-006).
//! - Names: the observed date and the first case or the claim, as `evidence add` names
//!   an entry (REQ-records-003); a validation run is named from its case ID and `run`
//!   (REQ-cli-023). A clash takes the first 6 hexadecimal digits of the SHA-256 of the
//!   old ID, so a preview and its apply name everything alike. Every old ID is an alias.
//! - Artifacts: a raw log under `.chough/artifacts/` is removed and its recorded digest is
//!   the fingerprint (REQ-records-005); a PNG or JPEG is stripped of its metadata and kept
//!   beside its session or story within REQ-records-013's bounds; an artifact elsewhere in
//!   the project stays the project's and keeps its path and digest on an entry.
//! - What is not carried is one counted gap line per field.
//!
//! It reads and returns only: the framework redacts, writes and removes.

use crate::evidence::{Evidence, ValidationRun};
use crate::migrate::{Binary, Carried, ObservationHomes, Receipt, SessionHome, Written};
use crate::records::{local_path, read_bounded_record};
use anyhow::{Context, Result, bail, ensure};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

// MARK: - Constants

const EVIDENCE_DIR: &str = ".chough/evidence";
const VALIDATION_DIR: &str = ".chough/validation";
const ARTIFACTS_DIR: &str = ".chough/artifacts";
/// The 0.5.x bound of one record file.
const LEGACY_BOUND: u64 = 1_048_576;
/// REQ-records-013's bounds: per screenshot after stripping, and per session.
const SCREENSHOT_BYTES: u64 = 524_288;
const SCREENSHOTS_PER_SESSION: usize = 5;
const SCREENSHOT_BYTES_PER_SESSION: u64 = 2_097_152;

// MARK: - Model

/// Where a 0.5.x record goes.
#[derive(Clone, Debug, PartialEq, Eq)]
enum Home {
    /// An entry of this open session.
    Entry(String),
    /// A receipt, beside this archived story when one references it, and in these bugs.
    Receipt {
        story: Option<String>,
        bugs: Vec<String>,
    },
    Removed,
}

/// A 0.5.x observation as read, with its file and, for one the 0.5.x roll-up moved into
/// the vault, its `<library-dir>/attachments/<folder>`, where its artifacts moved too.
struct Observation {
    source: String,
    evidence: Evidence,
    attachment: Option<String>,
}

impl Observation {
    /// Where an artifact the record names is now: a rolled observation's
    /// `.chough/artifacts/<rest>` moved to `<attachment>/artifacts/<rest>`.
    fn locate(&self, root: &Path, artifact: &str) -> Result<String> {
        if let (Some(base), Some(rest)) = (&self.attachment, artifact.strip_prefix(".chough/")) {
            let moved = format!("{base}/{rest}");
            if local_path(root, &moved)?.is_file() {
                return Ok(moved);
            }
        }
        Ok(artifact.to_string())
    }
}

/// A 0.5.x validation run as read, with its file.
struct Run {
    source: String,
    run: crate::validate::LegacyRun,
}

/// A record the plan carries, entry or receipt, before its name is chosen.
enum Item<'a> {
    Observation(&'a Observation),
    Run(&'a Run),
}

impl Item<'_> {
    fn old_id(&self) -> &str {
        match self {
            Item::Observation(observation) => &observation.evidence.id,
            Item::Run(run) => &run.run.id,
        }
    }

    /// When it happened, which orders naming and screenshot bounds.
    fn observed(&self) -> u64 {
        match self {
            Item::Observation(observation) => observation.evidence.observed_at_unix_seconds,
            Item::Run(run) => run.run.started_at_unix_seconds,
        }
    }

    /// The name `evidence add` would give it on its observed date (REQ-records-003,
    /// REQ-cli-023).
    fn base_name(&self) -> String {
        let date = crate::identity::utc_date(self.observed());
        match self {
            Item::Observation(observation) => {
                let evidence = &observation.evidence;
                let source = evidence
                    .cases
                    .first()
                    .map(|case| case.id.clone())
                    .unwrap_or_else(|| evidence.summary.clone());
                crate::identity::dated_name(&date, &source, "evidence")
            }
            Item::Run(run) => {
                crate::identity::dated_name(&date, &format!("{} run", run.run.case), "evidence")
            }
        }
    }
}

/// What not carrying a field cost, counted once per field for the gap lines.
#[derive(Default)]
struct Uncarried {
    snapshot_fingerprints: usize,
    dirty: usize,
    provenance: usize,
    receipt_inventories: usize,
    receipt_extra_cases: usize,
    receipt_artifact_paths: usize,
    receipt_fields: usize,
}

impl Uncarried {
    fn count(&mut self, evidence: &Evidence, receipt: bool) {
        self.snapshot_fingerprints += 1;
        self.provenance += 1;
        if evidence.snapshot.dirty {
            self.dirty += 1;
        }
        if !receipt {
            return;
        }
        self.receipt_fields += 1;
        if evidence.snapshot.input_inventory.is_some() {
            self.receipt_inventories += 1;
        }
        if evidence.cases.len() > 1 {
            self.receipt_extra_cases += 1;
        }
        if evidence
            .artifacts
            .iter()
            .any(|artifact| !artifact.starts_with(&format!("{ARTIFACTS_DIR}/")))
        {
            self.receipt_artifact_paths += 1;
        }
    }

    fn lines(&self) -> Vec<String> {
        let mut lines = vec![];
        let mut add = |count: usize, text: String| {
            if count > 0 {
                lines.push(text);
            }
        };
        let observations = |count: usize| plural(count, "observation");
        let receipts = |count: usize| plural(count, "receipt");
        add(
            self.snapshot_fingerprints,
            format!(
                "the 0.5.x snapshot fingerprint (a revision-bound identity) of {} is not carried; 0.6.0 compares the content identity, and Git keeps the 0.5.x files",
                observations(self.snapshot_fingerprints)
            ),
        );
        add(
            self.dirty,
            format!(
                "{} were recorded on a dirty tree; that flag is not carried, and the content identity covers the dirty files",
                observations(self.dirty)
            ),
        );
        add(
            self.provenance,
            format!(
                "provenance of {} is not carried; Git keeps it in the 0.5.x files",
                observations(self.provenance)
            ),
        );
        add(
            self.receipt_fields,
            format!(
                "the criteria, requirements, contracts, kind, method, track, exit code, tests run and limitations of {} are not carried, since REQ-history-006 lists a receipt's fields; Git keeps them in the 0.5.x files",
                receipts(self.receipt_fields)
            ),
        );
        add(
            self.receipt_inventories,
            format!(
                "the input inventory of {} is not carried; Git keeps it in the 0.5.x files",
                receipts(self.receipt_inventories)
            ),
        );
        add(
            self.receipt_extra_cases,
            format!(
                "the cases after the first of {} are not carried; Git keeps them in the 0.5.x files",
                receipts(self.receipt_extra_cases)
            ),
        );
        add(
            self.receipt_artifact_paths,
            format!(
                "the artifact paths of {} are not carried; each receipt's fingerprint is its first artifact's digest",
                receipts(self.receipt_artifact_paths)
            ),
        );
        lines
    }
}

fn plural(count: usize, noun: &str) -> String {
    if count == 1 {
        format!("1 {noun}")
    } else {
        format!("{count} {noun}s")
    }
}

// MARK: - The mapper

/// Mapper 3 (REQ-records-009), given where each 0.5.x session went and each 0.5.x
/// bug's linked observations.
pub(super) fn migrate_0_5(
    root: &Path,
    sessions: &BTreeMap<String, SessionHome>,
    bug_links: &BTreeMap<String, Vec<String>>,
) -> Result<(Carried, ObservationHomes)> {
    let mut carried = Carried::default();
    let mut homes = ObservationHomes::default();
    let observations = read_observations(root, &mut carried)?;
    let runs = read_runs(root, &mut carried)?;
    let bug_paths = crate::bugs::paths_0_5(root)?;
    // An open bug's fixtures stay byte for byte, so nothing here lists them as removed.
    let fixtures = crate::bugs::open_fixtures_0_5(root)?;
    let taken = existing_names(root)?;

    // Place, then name in observed-then-old-ID order.
    let mut items: Vec<(Item, Home)> = vec![];
    for observation in &observations {
        let home = place(&observation.evidence.id, sessions, bug_links);
        items.push((Item::Observation(observation), home));
    }
    for run in &runs {
        let home = place(&run.run.id, sessions, bug_links);
        items.push((Item::Run(run), home));
    }
    items.sort_by(|(left, _), (right, _)| {
        (left.observed(), left.old_id()).cmp(&(right.observed(), right.old_id()))
    });
    let mut planned: BTreeSet<String> = BTreeSet::new();
    let mut screenshots = Screenshots::default();
    let mut uncarried = Uncarried::default();
    for (item, home) in &items {
        let source = match item {
            Item::Observation(observation) => &observation.source,
            Item::Run(run) => &run.source,
        };
        if *home == Home::Removed {
            // A rolled observation in the vault is the knowledge mapper's to remove.
            if source.starts_with(".chough/") {
                carried.removed.insert(
                    source.clone(),
                    "no open session, archived session or bug references it (REQ-records-009); Git keeps its bytes".into(),
                );
            }
            carried
                .aliases
                .insert(item.old_id().to_string(), "removed".into());
            if let Item::Observation(observation) = item {
                remove_artifacts(root, observation, &fixtures, &mut carried)?;
            }
            continue;
        }
        let name = choose_name(item, &taken, &planned)?;
        planned.insert(name.clone());
        carried.sources.insert(source.clone());
        match item {
            Item::Observation(observation) => carry_observation(
                root,
                observation,
                home,
                &name,
                &bug_paths,
                &fixtures,
                &mut screenshots,
                &mut uncarried,
                &mut carried,
                &mut homes,
            )?,
            Item::Run(run) => carry_run(
                run,
                home,
                &name,
                &bug_paths,
                &observations,
                &mut carried,
                &mut homes,
            )?,
        }
    }
    screenshots.settle(&mut carried, &mut homes)?;
    // Every other file under `.chough/artifacts/` is removed; an open bug's fixture is
    // kept by the framework whatever is listed here.
    for path in files_under(root, ARTIFACTS_DIR)? {
        if !carried.sources.contains(&path)
            && !carried.removed.contains_key(&path)
            && !fixtures.contains(&path)
        {
            carried.removed.insert(
                path,
                "a 0.5.x artifact no carried observation keeps (REQ-records-009); Git keeps its bytes".into(),
            );
        }
    }
    carried.gaps.extend(uncarried.lines());
    Ok((carried, homes))
}

/// Where the record with this old ID goes (REQ-records-009).
fn place(
    id: &str,
    sessions: &BTreeMap<String, SessionHome>,
    bug_links: &BTreeMap<String, Vec<String>>,
) -> Home {
    let references = |home: &&SessionHome| home.observations.iter().any(|held| held == id);
    if let Some(open) = sessions
        .values()
        .filter(|home| !home.archived)
        .filter(references)
        .map(|home| home.id.clone())
        .min()
    {
        return Home::Entry(open);
    }
    let story = sessions
        .values()
        .filter(|home| home.archived)
        .filter(references)
        .map(|home| home.id.clone())
        .min();
    let bugs: Vec<String> = bug_links
        .iter()
        .filter(|(_, linked)| linked.iter().any(|held| held == id))
        .map(|(bug, _)| bug.clone())
        .collect();
    if story.is_none() && bugs.is_empty() {
        return Home::Removed;
    }
    Home::Receipt { story, bugs }
}

/// The record's name: its base name, or with the clash suffix of its old ID when another
/// record already holds that name (REQ-records-003). A record that holds the old ID as an
/// alias is this plan's own write from an interrupted apply, so it takes nothing.
fn choose_name(
    item: &Item,
    taken: &BTreeMap<String, Vec<String>>,
    planned: &BTreeSet<String>,
) -> Result<String> {
    let old = item.old_id();
    let free = |name: &str| {
        !planned.contains(name)
            && taken
                .get(name)
                .is_none_or(|aliases| aliases.iter().any(|alias| alias == old))
    };
    let base = item.base_name();
    if free(&base) {
        return Ok(base);
    }
    let suffix = &format!("{:x}", Sha256::digest(old.as_bytes()))[..6];
    let name = format!("{base}-{suffix}");
    ensure!(
        free(&name),
        "refused: {old} and another record both migrate to the name {name}, so no plan was made"
    );
    Ok(name)
}

/// The IDs 0.6.0 records already hold in a mixed tree, each with its aliases.
fn existing_names(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    Ok(crate::evidence::every_observation(root)?
        .into_iter()
        .map(|view| (view.id, view.aliases))
        .collect())
}

// MARK: - Reading 0.5.x records

/// Every 0.5.x observation under `.chough/evidence/`, sorted by file; any other file
/// there is removed with why. A file that does not read as an observation refuses the
/// plan, naming it.
fn read_observations(root: &Path, carried: &mut Carried) -> Result<Vec<Observation>> {
    let mut found = vec![];
    for path in files_under(root, EVIDENCE_DIR)? {
        let top_level_json = path
            .strip_prefix(&format!("{EVIDENCE_DIR}/"))
            .is_some_and(|name| !name.contains('/') && name.ends_with(".json"));
        if !top_level_json {
            carried.removed.insert(
                path,
                "not a 0.5.x observation record; 0.6.0 keeps evidence in session folders".into(),
            );
            continue;
        }
        let text = read_bounded_record(&local_path(root, &path)?, LEGACY_BOUND)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        let evidence = crate::evidence::legacy_observation(&path, &text)
            .with_context(|| format!("{path} is not a 0.5.x observation, so no plan was made"))?;
        found.push(Observation {
            source: path,
            evidence,
            attachment: None,
        });
    }
    // Rolled observations: `<library-dir>/attachments/<folder>/evidence/<id>.json`. A
    // second copy of an ID already read is left to the knowledge mapper.
    let library = crate::prefs::effective(root, "library-dir")?.0;
    let attachments = format!("{library}/attachments");
    let Ok(folders) = std::fs::read_dir(local_path(root, &attachments)?) else {
        return Ok(found);
    };
    let mut folders: Vec<String> = folders
        .flatten()
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    folders.sort();
    for folder in folders {
        let base = format!("{attachments}/{folder}");
        let evidence_dir = format!("{base}/evidence");
        for path in files_under(root, &evidence_dir)? {
            let top_level_json = path
                .strip_prefix(&format!("{evidence_dir}/"))
                .is_some_and(|name| !name.contains('/') && name.ends_with(".json"));
            if !top_level_json {
                continue;
            }
            let text = read_bounded_record(&local_path(root, &path)?, LEGACY_BOUND)
                .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
            let evidence =
                crate::evidence::legacy_observation(&path, &text).with_context(|| {
                    format!("{path} is not a 0.5.x observation, so no plan was made")
                })?;
            if found.iter().any(|known| known.evidence.id == evidence.id) {
                continue;
            }
            found.push(Observation {
                source: path,
                evidence,
                attachment: Some(base.clone()),
            });
        }
    }
    Ok(found)
}

/// Every 0.5.x validation run under `.chough/validation/`, sorted by file; any other
/// file there is removed with why.
fn read_runs(root: &Path, carried: &mut Carried) -> Result<Vec<Run>> {
    let mut found = vec![];
    for path in files_under(root, VALIDATION_DIR)? {
        let top_level_json = path
            .strip_prefix(&format!("{VALIDATION_DIR}/"))
            .is_some_and(|name| !name.contains('/') && name.ends_with(".json"));
        if !top_level_json {
            carried.removed.insert(
                path,
                "not a 0.5.x validation run; 0.6.0 records a run as one evidence entry".into(),
            );
            continue;
        }
        let text = read_bounded_record(&local_path(root, &path)?, LEGACY_BOUND)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        let run = crate::validate::legacy_run(&path, &text).with_context(|| {
            format!("{path} is not a 0.5.x validation run, so no plan was made")
        })?;
        found.push(Run { source: path, run });
    }
    Ok(found)
}

/// Every file under `relative`, project-relative and sorted.
fn files_under(root: &Path, relative: &str) -> Result<Vec<String>> {
    fn walk(root: &Path, relative: &str, found: &mut Vec<String>) -> Result<()> {
        let Ok(entries) = std::fs::read_dir(local_path(root, relative)?) else {
            return Ok(());
        };
        for entry in entries {
            let entry = entry?;
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            let kind = entry.file_type()?;
            if kind.is_dir() {
                walk(root, &name, found)?;
            } else if kind.is_file() {
                found.push(name);
            }
        }
        Ok(())
    }
    let mut found = vec![];
    walk(root, relative, &mut found)?;
    found.sort();
    Ok(found)
}

// MARK: - Carrying

/// What an observation's artifacts become: the raw log's digest, a key screenshot to
/// place, and the in-project artifacts it keeps; Chough's other artifacts are removed.
struct Artifacts {
    log_digest: Option<String>,
    image: Option<(String, Vec<u8>)>,
    kept: Vec<String>,
    kept_fingerprints: BTreeMap<String, String>,
}

/// Sort an observation's artifacts (REQ-records-005, REQ-records-013). A rolled
/// observation's artifacts are read where the roll-up moved them; the knowledge mapper
/// removes what is not carried from the vault, so only `.chough/artifacts/` is listed
/// here, never an open bug's fixture.
fn sort_artifacts(
    root: &Path,
    observation: &Observation,
    fixtures: &BTreeSet<String>,
    carried: &mut Carried,
) -> Result<Artifacts> {
    let evidence = &observation.evidence;
    let mut sorted = Artifacts {
        log_digest: None,
        image: None,
        kept: vec![],
        kept_fingerprints: BTreeMap::new(),
    };
    for artifact in &evidence.artifacts {
        let digest = evidence.artifact_fingerprints.get(artifact).cloned();
        if !artifact.starts_with(&format!("{ARTIFACTS_DIR}/")) {
            sorted.kept.push(artifact.clone());
            if let Some(digest) = digest {
                sorted.kept_fingerprints.insert(artifact.clone(), digest);
            }
            continue;
        }
        let located = observation.locate(root, artifact)?;
        let exists = local_path(root, &located)?.is_file();
        let listed = exists
            && located.starts_with(&format!("{ARTIFACTS_DIR}/"))
            && !fixtures.contains(&located);
        let lower = artifact.to_ascii_lowercase();
        if lower.ends_with(".log") {
            if sorted.log_digest.is_none() {
                sorted.log_digest = digest;
            }
            if listed {
                carried.removed.insert(
                    located,
                    "a raw log is never committed (REQ-records-005); its SHA-256 stays the entry's fingerprint, and Git keeps its bytes".into(),
                );
            }
        } else if sorted.image.is_none()
            && exists
            && [".png", ".jpg", ".jpeg"]
                .iter()
                .any(|extension| lower.ends_with(extension))
        {
            let bytes = std::fs::read(local_path(root, &located)?)
                .with_context(|| format!("{located} cannot be read, so no plan was made"))?;
            sorted.image = Some((located, bytes));
        } else if listed {
            carried.removed.insert(
                located,
                "a 0.5.x artifact other than a raw log or a key screenshot is not carried (REQ-records-009); Git keeps its bytes".into(),
            );
        }
    }
    Ok(sorted)
}

/// Remove the artifacts under `.chough/artifacts/` of an observation that is not
/// carried, an open bug's fixture excepted.
fn remove_artifacts(
    root: &Path,
    observation: &Observation,
    fixtures: &BTreeSet<String>,
    carried: &mut Carried,
) -> Result<()> {
    for artifact in &observation.evidence.artifacts {
        let located = observation.locate(root, artifact)?;
        if located.starts_with(&format!("{ARTIFACTS_DIR}/"))
            && !fixtures.contains(&located)
            && local_path(root, &located)?.is_file()
        {
            carried.removed.insert(
                located,
                format!(
                    "an artifact of {}, which is not carried (REQ-records-009); Git keeps its bytes",
                    observation.evidence.id
                ),
            );
        }
    }
    Ok(())
}

/// Carry one observation as an entry or a receipt.
#[allow(clippy::too_many_arguments)]
fn carry_observation(
    root: &Path,
    observation: &Observation,
    home: &Home,
    name: &str,
    bug_paths: &BTreeMap<String, String>,
    fixtures: &BTreeSet<String>,
    screenshots: &mut Screenshots,
    uncarried: &mut Uncarried,
    carried: &mut Carried,
    homes: &mut ObservationHomes,
) -> Result<()> {
    let old = observation.evidence.id.clone();
    let artifacts = sort_artifacts(root, observation, fixtures, carried)?;
    let mut evidence = observation.evidence.clone();
    evidence.id = name.to_string();
    evidence.aliases = vec![old.clone()];
    evidence.artifacts = artifacts.kept.clone();
    evidence.artifact_fingerprints = artifacts.kept_fingerprints.clone();
    evidence.fingerprint = artifacts.log_digest.clone();
    match home {
        Home::Entry(session) => {
            uncarried.count(&observation.evidence, false);
            let path = format!(".chough/sessions/{session}/evidence/{name}.md");
            carried.aliases.insert(old.clone(), path.clone());
            homes.entries.insert(old.clone(), path.clone());
            screenshots.offer(Placed {
                home: session.clone(),
                folder: format!(".chough/sessions/{session}/evidence"),
                entry: true,
                evidence,
                image: artifacts.image,
            });
        }
        Home::Receipt { story, bugs } => {
            uncarried.count(&observation.evidence, true);
            if evidence.fingerprint.is_none() {
                evidence.fingerprint = observation
                    .evidence
                    .artifacts
                    .iter()
                    .find(|artifact| !artifact.starts_with(&format!("{ARTIFACTS_DIR}/")))
                    .and_then(|artifact| observation.evidence.artifact_fingerprints.get(artifact))
                    .cloned();
            }
            let alias = match story {
                Some(story) => format!(".chough/archive/sessions/{story}.md"),
                None => bugs
                    .iter()
                    .find_map(|bug| bug_paths.get(bug).cloned())
                    .unwrap_or_else(|| "removed".into()),
            };
            carried.aliases.insert(old.clone(), alias);
            match story {
                Some(story) => screenshots.offer(Placed {
                    home: story.clone(),
                    folder: format!(".chough/archive/sessions/{story}"),
                    entry: false,
                    evidence,
                    image: artifacts.image,
                }),
                None => {
                    if let Some((image, _)) = &artifacts.image {
                        carried.removed.insert(
                            image.clone(),
                            "a key screenshot of an observation only a bug references; a screenshot is kept beside a session or story (REQ-records-013), and Git keeps its bytes".into(),
                        );
                    }
                    homes.receipts.insert(old, receipt_map(&evidence));
                }
            }
        }
        Home::Removed => bail!("a removed observation is never carried; this is a Chough bug"),
    }
    Ok(())
}

/// A receipt map: exactly REQ-history-006's fields, with `observed` and
/// `duration_seconds` (the plan's delta corrections 2 and 4), each only when known.
fn receipt_map(evidence: &Evidence) -> Receipt {
    let mut map: Receipt = vec![
        ("id".into(), evidence.id.clone()),
        ("aliases".into(), evidence.aliases.join(", ")),
        ("claim".into(), evidence.summary.clone()),
        ("result".into(), evidence.outcome.name().to_string()),
    ];
    if !evidence.argv.is_empty() {
        map.push(("command".into(), evidence.argv.join(" ")));
    }
    map.push(("revision".into(), evidence.snapshot.revision.clone()));
    map.push(("environment".into(), evidence.environment.clone()));
    if let Some(content) = &evidence.snapshot.content {
        map.push(("content".into(), content.clone()));
    }
    if let Some(fingerprint) = &evidence.fingerprint {
        map.push(("fingerprint".into(), fingerprint.clone()));
    }
    if let Some(case) = evidence.cases.first() {
        map.push(("case".into(), case.id.clone()));
        map.push(("case_digest".into(), case.digest.clone()));
    }
    if let Some(screenshot) = &evidence.screenshot {
        map.push(("screenshot".into(), screenshot.clone()));
    }
    map.push((
        "observed".into(),
        crate::session_md::iso_time(evidence.observed_at_unix_seconds),
    ));
    if let Some(duration) = evidence.duration_seconds {
        map.push(("duration_seconds".into(), duration.to_string()));
    }
    map
}

/// Carry one validation run as one entry holding each attempt, or as a receipt
/// (REQ-cli-023).
fn carry_run(
    run: &Run,
    home: &Home,
    name: &str,
    bug_paths: &BTreeMap<String, String>,
    observations: &[Observation],
    carried: &mut Carried,
    homes: &mut ObservationHomes,
) -> Result<()> {
    let legacy = &run.run;
    let attempts: Vec<Vec<(String, String)>> = legacy
        .observations
        .iter()
        .filter_map(|id| {
            observations
                .iter()
                .find(|observation| observation.evidence.id == *id)
        })
        .map(|observation| {
            let evidence = &observation.evidence;
            let mut attempt = vec![
                (
                    "observed".to_string(),
                    crate::session_md::iso_time(evidence.observed_at_unix_seconds),
                ),
                ("result".to_string(), evidence.outcome.name().to_string()),
            ];
            if let Some(duration) = evidence.duration_seconds {
                attempt.push(("duration_seconds".to_string(), duration.to_string()));
            }
            attempt
        })
        .collect();
    let completed = (legacy.state == "complete")
        .then(|| legacy.started_at_unix_seconds + legacy.elapsed_seconds);
    let entry = ValidationRun {
        id: name.to_string(),
        case: legacy.case.clone(),
        environment: legacy.environment.clone(),
        state: legacy.state.clone(),
        started: legacy.started_at_unix_seconds,
        completed,
        failures: u64::from(legacy.failures),
        not_run: u64::from(legacy.not_run),
        elapsed_seconds: legacy.elapsed_seconds,
        attempts,
        summary: format!(
            "A 0.5.x repeated validation run of {}: {} of {} attempts",
            legacy.case, legacy.completed, legacy.times
        ),
    };
    let old = legacy.id.clone();
    match home {
        Home::Entry(session) => {
            let path = format!(".chough/sessions/{session}/evidence/{name}.md");
            let text = crate::evidence::validation_run_text(&entry, std::slice::from_ref(&old))?;
            carried.written.push(Written {
                path: path.clone(),
                bytes: text,
            });
            carried.aliases.insert(old.clone(), path.clone());
            homes.entries.insert(old, path);
        }
        Home::Receipt { story, bugs } => {
            let alias = match story {
                Some(story) => format!(".chough/archive/sessions/{story}.md"),
                None => bugs
                    .iter()
                    .find_map(|bug| bug_paths.get(bug).cloned())
                    .unwrap_or_else(|| "removed".into()),
            };
            carried.aliases.insert(old.clone(), alias);
            let mut map: Receipt = vec![
                ("id".into(), name.to_string()),
                ("aliases".into(), old.clone()),
                ("claim".into(), entry.summary.clone()),
                ("result".into(), "not-run".into()),
                ("environment".into(), entry.environment.clone()),
                ("case".into(), entry.case.clone()),
                (
                    "observed".into(),
                    crate::session_md::iso_time(entry.started),
                ),
            ];
            map.push(("duration_seconds".into(), entry.elapsed_seconds.to_string()));
            homes.receipts.insert(old, map);
        }
        Home::Removed => bail!("a removed validation run is never carried; this is a Chough bug"),
    }
    Ok(())
}

// MARK: - Key screenshots

/// A carried observation waiting for its screenshot to be settled: an entry of an open
/// session, or a receipt of an archived story.
struct Placed {
    /// The open session's or the story's ID, whose bounds the screenshot counts toward.
    home: String,
    /// Where its screenshot goes: the session's `evidence/` folder or the folder beside
    /// the story (REQ-records-013).
    folder: String,
    entry: bool,
    evidence: Evidence,
    image: Option<(String, Vec<u8>)>,
}

/// The carried observations with their candidate screenshots, settled in the order
/// offered (observed, then old ID) so REQ-records-013's per-session bounds hold.
#[derive(Default)]
struct Screenshots {
    placed: Vec<Placed>,
}

impl Screenshots {
    fn offer(&mut self, placed: Placed) {
        self.placed.push(placed);
    }

    /// Keep each screenshot that fits, then write every entry and receipt.
    fn settle(self, carried: &mut Carried, homes: &mut ObservationHomes) -> Result<()> {
        let mut kept: BTreeMap<String, (usize, u64)> = BTreeMap::new();
        for placed in self.placed {
            let mut evidence = placed.evidence;
            if let Some((source, bytes)) = placed.image {
                let extension = if source.to_ascii_lowercase().ends_with(".png") {
                    "png"
                } else {
                    "jpg"
                };
                let stripped =
                    crate::evidence::strip_screenshot(&bytes, extension).with_context(|| {
                        format!("{source} is not a readable key screenshot, so no plan was made")
                    })?;
                let size = stripped.len() as u64;
                let (count, total) = kept.get(&placed.home).copied().unwrap_or((0, 0));
                let why = if size > SCREENSHOT_BYTES {
                    Some(format!(
                        "a key screenshot of {size} bytes once its metadata is stripped, over the {SCREENSHOT_BYTES} bytes one may hold (REQ-records-013); Git keeps its bytes"
                    ))
                } else if count >= SCREENSHOTS_PER_SESSION {
                    Some(format!(
                        "a key screenshot past the {SCREENSHOTS_PER_SESSION} a session keeps (REQ-records-013); Git keeps its bytes"
                    ))
                } else if total + size > SCREENSHOT_BYTES_PER_SESSION {
                    Some(format!(
                        "a key screenshot past the {SCREENSHOT_BYTES_PER_SESSION} bytes a session keeps (REQ-records-013); Git keeps its bytes"
                    ))
                } else {
                    None
                };
                if let Some(why) = why {
                    carried.removed.insert(source, why);
                } else {
                    kept.insert(placed.home.clone(), (count + 1, total + size));
                    let file = format!("{}.{extension}", evidence.id);
                    if evidence.fingerprint.is_none() {
                        evidence.fingerprint = Some(format!("{:x}", Sha256::digest(&stripped)));
                    }
                    evidence.screenshot = Some(file.clone());
                    carried.sources.insert(source);
                    carried.binaries.push(Binary {
                        path: format!("{}/{file}", placed.folder),
                        bytes: stripped,
                    });
                }
            }
            if placed.entry {
                carried.written.push(Written {
                    path: format!("{}/{}.md", placed.folder, evidence.id),
                    bytes: crate::evidence::entry_text(&evidence)?,
                });
            } else {
                let old = evidence.aliases.first().cloned().unwrap_or_default();
                homes.receipts.insert(old, receipt_map(&evidence));
            }
        }
        Ok(())
    }
}
