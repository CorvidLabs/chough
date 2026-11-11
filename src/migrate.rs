//! `chough migrate` (REQ-records-009, REQ-records-010): carry a project's 0.5.x records
//! into the 0.6.0 tree. A preview writes nothing and ends with the plan's digest; an
//! apply needs that digest, no open native change, the human's consent and every file
//! it removes committed at HEAD. It writes by create-new only, reads each file back,
//! writes a receipt, and only then removes the 0.5.x files, restoring them if an
//! archive that read valid no longer does.
//!
//! Each record kind has a mapper, run in this order because each needs the one before:
//!
//! 1. sessions (t382, [`sessions_0_5`]): open sessions become session folders and
//!    archived ones (the rolled ones in `knowledge/attachments/` included) archived
//!    stories, each timeline holding its selected history lineage in order, the legacy
//!    log's entries in the session that adopted them; inbox entries move to
//!    `agent/inbox/`; shelved 0.5.x sessions are removed. It also says which
//!    observations each session references ([`SessionHome`]).
//! 2. bug links (t38b, [`bug_links_0_5`]): each 0.5.x bug's linked observation IDs.
//! 3. evidence (t37d, [`evidence_0_5`]): an observation an open session references
//!    becomes one of its evidence entries; one an archived session or a bug references
//!    becomes a receipt; others are removed. A PNG or JPEG artifact of a carried
//!    observation within the bounds of REQ-records-013 is kept beside its session or
//!    story; other artifacts are removed.
//! 4. bugs (t38b, [`bugs_0_5`]): Markdown bugs named by date and slug, every 0.5.x ID an
//!    alias, linked observations as receipts or `removed`, an open bug's fixture kept
//!    byte for byte.
//! 5. knowledge (t37j, [`knowledge_0_5`]): the generated and copied vault content of
//!    REQ-cli-044 removed with why.
//!
//! A mapper only reads and returns a [`Carried`]; it writes nothing and redacts
//! nothing. The framework redacts every written file (REQ-records-006), counting each
//! redaction by file, field and kind, builds the plan and its digest, and applies it.

use crate::leaks::{self, Local};
use crate::records::{local_path, print_json};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// One 0.6.0 file the migration writes, create-new.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Written {
    /// Project-relative path.
    pub(crate) path: String,
    /// The bytes, before the framework's redaction.
    pub(crate) bytes: String,
}

/// What one mapper carries into 0.6.0.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct Carried {
    /// The 0.5.x files this mapper consumed; each is removed after the apply, once its
    /// bytes still match the plan.
    pub(crate) sources: BTreeSet<String>,
    /// The 0.6.0 files written.
    pub(crate) written: Vec<Written>,
    /// Each 0.5.x ID with the project-relative path of the record that now holds it as
    /// an alias (REQ-records-003), or `removed` when it was not carried.
    pub(crate) aliases: BTreeMap<String, String>,
    /// 0.5.x files removed without being carried, each with why.
    pub(crate) removed: BTreeMap<String, String>,
    /// Files kept byte for byte that hold a leak: listed as `leak` findings, never
    /// changed (REQ-records-009), each with the kinds found.
    pub(crate) kept_with_leaks: BTreeMap<String, Vec<String>>,
}

impl Carried {
    /// Add `other` to this one; a file two mappers write is refused.
    fn absorb(&mut self, other: Carried) -> Result<()> {
        for written in other.written {
            if self.written.iter().any(|known| known.path == written.path) {
                bail!(
                    "two mappers write {}, so the plan is refused; this is a Chough bug",
                    written.path
                );
            }
            self.written.push(written);
        }
        self.sources.extend(other.sources);
        self.aliases.extend(other.aliases);
        self.removed.extend(other.removed);
        self.kept_with_leaks.extend(other.kept_with_leaks);
        Ok(())
    }
}

/// Where a 0.5.x session went, by its 0.5.x key (the file stem, or `legacy` for
/// `.chough/session.json`).
#[derive(Clone, Debug, Serialize)]
pub(crate) struct SessionHome {
    /// The 0.6.0 session ID (`<date>-<slug>`).
    pub(crate) id: String,
    /// Open sessions keep a folder; archived ones become a story.
    pub(crate) archived: bool,
    /// The observation IDs its checkpoint references.
    pub(crate) observations: Vec<String>,
}

/// The receipt map of one carried observation, as a story's or a bug's `receipts:`
/// list holds it (REQ-history-006, with `observed` and `duration_seconds`).
pub(crate) type Receipt = Vec<(String, String)>;

/// Where each 0.5.x observation went: an open session's evidence entry (its path), or
/// a receipt (its map), by observation ID. An observation in neither was removed.
#[derive(Clone, Debug, Default)]
// Read by the bug mapper (t38b) when it lands; removed at integration (t382).
#[allow(dead_code)]
pub(crate) struct ObservationHomes {
    pub(crate) entries: BTreeMap<String, String>,
    pub(crate) receipts: BTreeMap<String, Receipt>,
}

/// The 0.5.x record locations whose presence makes a tree unmigrated (REQ-records-010):
/// files, folders whose contents count, and the folders whose `*.json` files count.
const LEGACY_FILES: [&str; 3] = [
    ".chough/session.json",
    ".chough/transitions.jsonl",
    ".chough/pruned.jsonl",
];
const LEGACY_FOLDERS: [&str; 3] = [
    ".chough/sessions/archive",
    ".chough/history",
    ".chough/evidence",
];
const LEGACY_JSON_IN: [&str; 2] = [".chough/sessions", ".chough/bugs"];

/// Every 0.5.x record path in the project, sorted (REQ-records-010).
pub(crate) fn legacy_paths(root: &Path) -> Result<Vec<String>> {
    let mut found = BTreeSet::new();
    for file in LEGACY_FILES {
        if local_path(root, file)?.is_file() {
            found.insert(file.to_string());
        }
    }
    for folder in LEGACY_FOLDERS {
        walk_files(root, folder, &mut found)?;
    }
    for folder in LEGACY_JSON_IN {
        let Ok(entries) = std::fs::read_dir(local_path(root, folder)?) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".json") && entry.path().is_file() {
                found.insert(format!("{folder}/{name}"));
            }
        }
    }
    Ok(found.into_iter().collect())
}

/// Every file under `relative`, as project-relative paths.
fn walk_files(root: &Path, relative: &str, found: &mut BTreeSet<String>) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(local_path(root, relative)?) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            walk_files(root, &name, found)?;
        } else if kind.is_file() {
            found.insert(name);
        }
    }
    Ok(())
}

#[derive(clap::Args)]
pub(super) struct MigrateArgs {
    /// Apply the plan whose digest a preview printed; without it, the plan is only
    /// printed and nothing is written.
    #[arg(long)]
    apply: Option<String>,
    /// Who consented to the migration, their words and where, recorded in the receipt
    /// as a relayed claim; or a standing `migration` preference gives it.
    #[arg(long, requires = "apply")]
    consent: Option<String>,
}

/// One redaction the plan makes: the file, the field and the kind, never the text.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct Redaction {
    file: String,
    field: String,
    kind: String,
}

/// The whole plan: what is read, written, redacted, kept and removed.
struct Plan {
    /// Every 0.5.x file with its SHA-256.
    sources: BTreeMap<String, String>,
    carried: Carried,
    redactions: Vec<Redaction>,
    follow_ups: Vec<String>,
}

impl Plan {
    /// The plan as printed, without its digest.
    fn view(&self) -> Value {
        let written: Vec<Value> = self
            .carried
            .written
            .iter()
            .map(|written| {
                json!({"path": written.path, "sha256": sha256(written.bytes.as_bytes()), "bytes": written.bytes.len()})
            })
            .collect();
        json!({
            "schema_version": 1,
            "sources": self.sources,
            "writes": written,
            "redactions": self.redactions,
            "removed": self.carried.removed,
            "kept_with_leaks": self.carried.kept_with_leaks,
            "aliases": self.carried.aliases,
            "follow_ups": self.follow_ups,
            "git_history": "Git history still holds what earlier commits recorded; this migration does not rewrite it.",
        })
    }

    /// The SHA-256 of the printed plan, which `--apply` must name.
    fn digest(&self) -> String {
        sha256(self.view().to_string().as_bytes())
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn run(root: &Path, args: MigrateArgs) -> Result<()> {
    let plan = plan(root)?;
    let digest = plan.digest();
    match args.apply {
        None => {
            let mut view = plan.view();
            view["plan_digest"] = json!(digest);
            view["apply"] = json!(format!(
                "chough migrate --apply {digest} --consent \"<who consented, their words and where>\""
            ));
            print_json(&view)
        }
        Some(_) => bail!(
            "`chough migrate --apply` is being built in plan step 10; nothing was written. Run `chough migrate` to preview the plan."
        ),
    }
}

/// Build the plan: run every mapper in order, redact what they write and list every
/// 0.5.x file read.
fn plan(root: &Path) -> Result<Plan> {
    let mut carried = Carried::default();
    let (sessions, homes) = sessions_0_5(root)?;
    carried.absorb(sessions)?;
    let links = bug_links_0_5(root)?;
    let (evidence, observations) = evidence_0_5(root, &homes, &links)?;
    carried.absorb(evidence)?;
    carried.absorb(bugs_0_5(root, &observations)?)?;
    carried.absorb(knowledge_0_5(root)?)?;
    // A file a mapper carried is never `removed without carrying`, whatever another
    // mapper listed.
    let carried_sources = carried.sources.clone();
    carried
        .removed
        .retain(|path, _| !carried_sources.contains(path));
    carried
        .written
        .sort_by(|left, right| left.path.cmp(&right.path));
    let local = Local::current();
    let mut redactions = vec![];
    for written in &mut carried.written {
        redactions.extend(redact(local, written)?);
    }
    redactions.sort();
    let mut sources = BTreeMap::new();
    for path in legacy_paths(root)?
        .into_iter()
        .chain(carried.sources.iter().cloned())
    {
        let bytes = std::fs::read(local_path(root, &path)?)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        sources.insert(path, sha256(&bytes));
    }
    // REQ-records-014: the plan lists the missing verification exclusions as a
    // follow-up; Chough never writes SpecSync's policy.
    let follow_ups = crate::doctor::exclusions(root)
        .advice()
        .into_iter()
        .collect::<Vec<_>>();
    Ok(Plan {
        sources,
        carried,
        redactions,
        follow_ups,
    })
}

/// Redact every leak in a written file (REQ-records-006), returning each redaction by
/// field and kind. A Markdown record is redacted field by field and in its body; any
/// other file as a whole.
fn redact(local: &Local, written: &mut Written) -> Result<Vec<Redaction>> {
    let mut found = vec![];
    let mut note = |field: &str, text: &str| {
        for matched in leaks::find(local, text) {
            found.push(Redaction {
                file: written.path.clone(),
                field: field.to_string(),
                kind: matched.kind.name().to_string(),
            });
        }
    };
    if written.path.ends_with(".md")
        && let Ok(mut record) = crate::markdown::parse(&written.path, &written.bytes)
    {
        for (field, value) in record.fields() {
            note(&field, &value);
        }
        if !found.is_empty() {
            record.redact_leaks(local);
            written.bytes = record.render();
        }
    } else {
        note("content", &written.bytes.clone());
        if !found.is_empty() {
            written.bytes = leaks::redact(local, &written.bytes).0;
        }
    }
    Ok(found)
}

// Called by the bug mapper (t38b) and the apply; removed at integration (t382).
#[allow(dead_code)]
/// The migration receipt's path, fixed when the plan is built: an apply on another UTC
/// day rebuilds a plan with another digest and asks for a fresh preview.
pub(crate) fn receipt_path() -> Result<String> {
    Ok(format!(
        ".chough/archive/migrations/{}-from-0-5.md",
        crate::identity::utc_date(crate::records::now()?)
    ))
}

/// A deterministic six-hex-digit suffix or entry ID from `seed`, so the plan an apply
/// rebuilds has the previewed digest (REQ-records-009).
fn hex6(seed: &str) -> String {
    sha256(seed.as_bytes())[..6].to_string()
}

/// The 0.5.x history cursor of a session (`history_cursor`).
#[derive(serde::Deserialize, Clone, Default)]
struct Cursor {
    cycle: String,
    head: Option<String>,
    #[serde(default)]
    legacy: Option<Value>,
}

/// One 0.5.x history event.
#[derive(serde::Deserialize)]
struct Event {
    parent: Option<String>,
    previous_cycle: Option<Cursor>,
    transition: crate::stage::Transition,
}

/// A session's selected history lineage, oldest first, as 0.5.x selected it: from the
/// cursor's head through each parent and previous cycle, then the legacy log when the
/// walk reaches its anchor. Returns the transitions, the files read, and what could not
/// be read (a pruned or missing event ends the walk there).
fn lineage(
    root: &Path,
    cursor: &Cursor,
) -> Result<(Vec<crate::stage::Transition>, Vec<String>, Vec<String>)> {
    let mut transitions = vec![];
    let mut files = vec![];
    let mut missing = vec![];
    let mut current = cursor.clone();
    let mut seen = BTreeSet::new();
    while let Some(id) = current.head.clone() {
        if !seen.insert((current.cycle.clone(), id.clone())) {
            missing.push(format!("a parent loop at {id}"));
            break;
        }
        let path = format!(".chough/history/cycles/{}/{id}.json", current.cycle);
        let Ok(bytes) = std::fs::read(local_path(root, &path)?) else {
            missing.push(format!("{path} (pruned or missing)"));
            break;
        };
        let event: Event = serde_json::from_slice(&bytes)
            .with_context(|| format!("{path} is not a 0.5.x history event"))?;
        files.push(path);
        transitions.push(event.transition);
        current.head = event.parent;
        if current.head.is_none()
            && let Some(previous) = event.previous_cycle
        {
            current = previous;
        }
    }
    if current.legacy.is_some() && missing.is_empty() {
        let legacy = ".chough/transitions.jsonl";
        if let Ok(text) = std::fs::read_to_string(local_path(root, legacy)?) {
            for line in text.lines().rev().filter(|line| !line.trim().is_empty()) {
                transitions.push(
                    serde_json::from_str(line).with_context(|| {
                        format!("{legacy} holds a line that is not a transition")
                    })?,
                );
            }
            files.push(legacy.to_string());
        } else {
            missing.push(format!("{legacy} (the adopted legacy log)"));
        }
    }
    transitions.reverse();
    Ok((transitions, files, missing))
}

/// The newest commit time of `relative`, when Git has one.
fn committed_at(root: &Path, relative: &str) -> Option<u64> {
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%ct", "--", relative])
        .current_dir(root)
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().parse().ok()
}

/// Mapper 1, sessions (t382): each 0.5.x session becomes a session folder (open) or an
/// archived story, its timeline the selected lineage; unfolded inbox entries move to
/// `agent/inbox/`; shelved sessions are removed.
fn sessions_0_5(root: &Path) -> Result<(Carried, BTreeMap<String, SessionHome>)> {
    let mut carried = Carried::default();
    let mut homes = BTreeMap::new();
    let mut found: Vec<(String, String, bool)> = vec![];
    if local_path(root, ".chough/session.json")?.is_file() {
        found.push(("legacy".into(), ".chough/session.json".into(), false));
    }
    // Sessions the 0.5.x roll-up moved into the vault are archived stories too
    // (`<library-dir>/attachments/<folder>/sessions/<key>.json`).
    let library = crate::prefs::effective(root, "library-dir")
        .map(|(dir, _)| dir)
        .unwrap_or_else(|_| "knowledge".into());
    let mut folders: Vec<(String, bool)> = vec![
        (".chough/sessions".into(), false),
        (".chough/sessions/archive".into(), true),
    ];
    let attachments = format!("{}/attachments", library.trim_end_matches('/'));
    if let Ok(entries) = std::fs::read_dir(local_path(root, &attachments)?) {
        let mut rolled: Vec<String> = entries
            .flatten()
            .filter(|entry| entry.path().join("sessions").is_dir())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .map(|folder| format!("{attachments}/{folder}/sessions"))
            .collect();
        rolled.sort();
        folders.extend(rolled.into_iter().map(|folder| (folder, true)));
    }
    for (folder, archived) in folders {
        let folder = folder.as_str();
        let mut names: Vec<String> = std::fs::read_dir(local_path(root, folder)?)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|entry| entry.path().is_file())
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .filter(|name| name.ends_with(".json"))
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        for name in names {
            let key = name.trim_end_matches(".json").to_string();
            found.push((key, format!("{folder}/{name}"), archived));
        }
    }
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut lineage_files: BTreeSet<String> = BTreeSet::new();
    for (key, path, archived) in found {
        let text = std::fs::read_to_string(local_path(root, &path)?)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        let mut value: Value = serde_json::from_str(&text)
            .with_context(|| format!("{path} is not a 0.5.x session record"))?;
        let cursor = value
            .as_object_mut()
            .and_then(|map| map.remove("history_cursor"))
            .filter(|cursor| !cursor.is_null());
        let mut session: crate::checkpoint::Checkpoint = serde_json::from_value(value)
            .with_context(|| format!("{path} is not a 0.5.x session record"))?;
        let (transitions, files, missing) = match cursor {
            Some(cursor) => lineage(root, &serde_json::from_value(cursor)?)?,
            // A legacy single checkpoint before history events: its log is its history.
            None if key == "legacy" => lineage(
                root,
                &Cursor {
                    legacy: Some(Value::Bool(true)),
                    ..Cursor::default()
                },
            )?,
            None => (vec![], vec![], vec![]),
        };
        lineage_files.extend(files.iter().cloned());
        let at = transitions
            .iter()
            .map(|transition| transition.at_unix_seconds)
            .filter(|at| *at > 0)
            .min()
            .or_else(|| committed_at(root, &path))
            .unwrap_or(crate::records::now()?);
        let last = transitions
            .iter()
            .map(|transition| transition.at_unix_seconds)
            .max()
            .unwrap_or(at);
        let source = if key == "legacy" {
            session.objective.clone()
        } else {
            key.clone()
        };
        let mut id =
            crate::identity::dated_name(&crate::identity::utc_date(at), &source, "session");
        if !taken.insert(id.clone()) {
            id = format!("{id}-{}", hex6(&path));
            taken.insert(id.clone());
        }
        session.meta = crate::session_md::Meta {
            id: id.clone(),
            aliases: if key == "legacy" {
                vec![]
            } else {
                vec![key.clone()]
            },
            pr: String::new(),
            gates: vec![],
        };
        session.pending = transitions;
        let record = crate::session_md::render_with(&session, last, &mut |index, attempt| {
            Ok(format!("t-{}", hex6(&format!("{id}#{index}#{attempt}"))))
        })?;
        let mut record = record;
        let (target, text) = if archived {
            let date = crate::identity::utc_date(last);
            let outcome = if session.stage.current == "done" {
                "done"
            } else {
                "closed-without-done"
            };
            record.set_scalar("closed", &date)?;
            record.set_scalar("outcome", outcome)?;
            record.set("receipts", crate::markdown::Value::Maps(vec![]))?;
            let mut text = record.render();
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&format!("\nClosed {date}: {outcome}.\n"));
            (format!(".chough/archive/sessions/{id}.md"), text)
        } else {
            (format!(".chough/sessions/{id}/session.md"), record.render())
        };
        for gap in missing {
            carried.removed.insert(
                format!("{path} history"),
                format!("not carried into {target}: {gap}"),
            );
        }
        carried.written.push(Written {
            path: target.clone(),
            bytes: text,
        });
        carried.sources.insert(path.clone());
        carried.sources.extend(files);
        if key != "legacy" {
            carried.aliases.insert(key.clone(), target.clone());
        }
        // Unfolded inbox entries of an open session move to its agent/inbox/.
        let inbox = format!(".chough/sessions/{key}.inbox");
        let mut entries: Vec<String> = std::fs::read_dir(local_path(root, &inbox)?)
            .map(|entries| {
                entries
                    .flatten()
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .filter(|name| name.ends_with(".json"))
                    .collect()
            })
            .unwrap_or_default();
        entries.sort();
        for name in entries {
            let entry_path = format!("{inbox}/{name}");
            if archived {
                carried
                    .removed
                    .insert(entry_path, "an inbox entry of an archived session".into());
                continue;
            }
            let entry: Value =
                serde_json::from_str(&std::fs::read_to_string(local_path(root, &entry_path)?)?)
                    .with_context(|| format!("{entry_path} is not a 0.5.x inbox entry"))?;
            let text = |field: &str| entry[field].as_str().unwrap_or_default().to_string();
            let kind = <crate::sessions::Kind as clap::ValueEnum>::from_str(&text("kind"), true)
                .map_err(|_| anyhow::anyhow!("{entry_path} has an unknown inbox kind"))?;
            let added = entry["recorded_at_unix_seconds"].as_u64().unwrap_or(at);
            let name = crate::identity::dated_name(
                &crate::identity::utc_date(added),
                &text("text"),
                "entry",
            );
            let rendered = crate::sessions::render_entry(
                &name,
                &id,
                &text("from"),
                kind,
                &text("text"),
                None,
                &crate::session_md::iso_time(added),
            )?;
            carried.written.push(Written {
                path: format!(".chough/sessions/{id}/agent/inbox/{name}.md"),
                bytes: rendered,
            });
            carried.sources.insert(entry_path.clone());
            carried.aliases.insert(text("id"), entry_path);
        }
        homes.insert(
            key,
            SessionHome {
                id,
                archived,
                observations: session.observations.clone(),
            },
        );
    }
    // What no session's lineage selected, shelved sessions and 0.5.x-only files.
    let mut leftovers = BTreeSet::new();
    walk_files(root, ".chough/history", &mut leftovers)?;
    walk_files(root, ".chough/sessions/shelved", &mut leftovers)?;
    for path in [".chough/transitions.jsonl", ".chough/sessions/.gitignore"] {
        if local_path(root, path)?.is_file() {
            leftovers.insert(path.to_string());
        }
    }
    for path in leftovers {
        if lineage_files.contains(&path) || carried.sources.contains(&path) {
            continue;
        }
        let why = if path.starts_with(".chough/sessions/shelved") {
            "a shelved 0.5.x session is not carried"
        } else if path.ends_with(".gitignore") {
            "0.5.x temporary-file ignore; 0.6.0 keeps local state in .chough/local/"
        } else if path.contains("/pruned/") {
            "a 0.5.x retention receipt; there is no retention in 0.6.0"
        } else {
            "history no session's selected lineage holds"
        };
        carried.removed.insert(path, why.into());
    }
    Ok((carried, homes))
}

/// Mapper 2, each 0.5.x bug's linked observation IDs (t38b): replace this body with a
/// call to `crate::bugs::links_0_5(root)`.
fn bug_links_0_5(_root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    Ok(BTreeMap::new())
}

/// Mapper 3, evidence (t37d): replace this body with a call to
/// `crate::evidence::migrate_0_5(root, sessions, bug_links)`.
fn evidence_0_5(
    _root: &Path,
    _sessions: &BTreeMap<String, SessionHome>,
    _bug_links: &BTreeMap<String, Vec<String>>,
) -> Result<(Carried, ObservationHomes)> {
    Ok((Carried::default(), ObservationHomes::default()))
}

/// Mapper 4, bugs (t38b): replace this body with a call to
/// `crate::bugs::migrate_0_5(root, observations)`.
fn bugs_0_5(_root: &Path, _observations: &ObservationHomes) -> Result<Carried> {
    Ok(Carried::default())
}

/// Mapper 5, knowledge (t37j): replace this body with a call to
/// `crate::library::migrate_0_5(root)`.
fn knowledge_0_5(_root: &Path) -> Result<Carried> {
    Ok(Carried::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_written_record_is_redacted_field_by_field_and_counted() {
        let local = Local::new(Some("SYNTHETIC-user"), Some("SYNTHETIC-host"));
        let mut written = Written {
            path: ".chough/bugs/2026-11-11-x.md".into(),
            bytes: "---\nkind: bug\nid: 2026-11-11-x\ntitle: Broken at /Users/SYNTHETIC-user/p\n---\n# x\n".into(),
        };
        let found = redact(&local, &mut written).unwrap();
        assert_eq!(
            found,
            vec![Redaction {
                file: ".chough/bugs/2026-11-11-x.md".into(),
                field: "title".into(),
                kind: "home-path".into(),
            }]
        );
        assert!(
            written.bytes.contains("<redacted:home-path>"),
            "{}",
            written.bytes
        );
        assert!(!written.bytes.contains("/Users/"));
    }

    /// Every release fixture's sessions map to 0.6.0 records that read back strictly,
    /// with every selected transition on the timeline, and the same plan twice.
    #[test]
    fn every_release_fixture_maps_its_sessions_to_readable_records() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases");
        for tag in ["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"] {
            let root = fixtures.join(tag);
            let (carried, homes) = sessions_0_5(&root).unwrap();
            let (again, _) = sessions_0_5(&root).unwrap();
            assert_eq!(carried.written, again.written, "{tag}: not deterministic");
            assert!(!carried.written.is_empty(), "{tag}: no session carried");
            let events = {
                let mut found = BTreeSet::new();
                walk_files(&root, ".chough/history", &mut found).unwrap();
                found.len()
            };
            let legacy = std::fs::read_to_string(root.join(".chough/transitions.jsonl"))
                .map(|text| text.lines().filter(|line| !line.trim().is_empty()).count())
                .unwrap_or(0);
            let mut entries = 0;
            for written in &carried.written {
                if written.path.ends_with("/session.md")
                    || written.path.starts_with(".chough/archive/sessions/")
                {
                    let session = crate::session_md::read(&written.path, &written.bytes)
                        .unwrap_or_else(|error| panic!("{tag} {}: {error:#}", written.path));
                    let record = session.source.as_ref().unwrap();
                    entries += crate::session_md::timeline(record).unwrap().len();
                    assert!(
                        homes.values().any(|home| written.path.contains(&home.id)),
                        "{tag}: {} has no home",
                        written.path
                    );
                }
            }
            assert_eq!(
                entries,
                events + legacy,
                "{tag}: transitions lost or duplicated"
            );
        }
    }

    /// A session the 0.5.x roll-up moved into the vault becomes an archived story, its
    /// history read through its cursor as for any archived session.
    #[test]
    fn a_rolled_session_in_the_vault_becomes_an_archived_story() {
        fn copy(from: &Path, to: &Path) {
            std::fs::create_dir_all(to).unwrap();
            for entry in std::fs::read_dir(from).unwrap().flatten() {
                let target = to.join(entry.file_name());
                if entry.path().is_dir() {
                    copy(&entry.path(), &target);
                } else {
                    std::fs::copy(entry.path(), target).unwrap();
                }
            }
        }
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases/v0.5.0");
        let root = tempfile::tempdir().unwrap();
        copy(&fixture, root.path());
        let rolled = root
            .path()
            .join("knowledge/attachments/fixture-second/sessions");
        std::fs::create_dir_all(&rolled).unwrap();
        std::fs::rename(
            root.path()
                .join(".chough/sessions/archive/fixture-second.json"),
            rolled.join("fixture-second.json"),
        )
        .unwrap();
        let (carried, homes) = sessions_0_5(root.path()).unwrap();
        assert!(homes["fixture-second"].archived);
        let story = &carried.aliases["fixture-second"];
        assert!(story.starts_with(".chough/archive/sessions/"), "{story}");
        assert!(
            carried
                .sources
                .contains("knowledge/attachments/fixture-second/sessions/fixture-second.json")
        );
        let written = carried
            .written
            .iter()
            .find(|written| &written.path == story)
            .unwrap();
        crate::session_md::read(&written.path, &written.bytes).unwrap();
    }

    #[test]
    fn two_mappers_writing_one_file_are_refused() {
        let mut one = Carried::default();
        let file = Written {
            path: "a.md".into(),
            bytes: String::new(),
        };
        one.written.push(file.clone());
        let mut two = Carried::default();
        two.written.push(file);
        assert!(one.absorb(two).is_err());
    }
}
