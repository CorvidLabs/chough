//! Evidence entries (REQ-cli-017): one Markdown record per observation at
//! `.chough/sessions/<session>/evidence/<id>.md`, named by date and slug
//! (REQ-records-001, REQ-records-003). The raw log of an executed case stays in
//! `.chough/local/logs/` (REQ-records-005); the entry carries the exact command, the
//! capture's fingerprint and the key failing lines. An external artifact is
//! fingerprinted in place and named by a neutral ID, never its path, and one small key
//! screenshot sits beside its entry (REQ-records-013). The retired retention (the
//! ledger, `evidence prune`, the protection walks) is gone (REQ-cli-033).
//!
//! Compatibility shims for files other steps own are at the bottom; they write nothing
//! and go when steps 3, 8 and 9 land their removals.

use crate::identity;
use crate::markdown::{self, Record};
use crate::records::{local_path, now, print_json, read_bounded_record, valid_name};
use crate::{cases, leaks};
use anyhow::{Context, Result, bail, ensure};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Component, Path};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// What every evidence view says an observation is, and is not: a manual entry, which
/// Chough did not run, and the sentence a 0.5.x record holds.
pub(super) const PROVENANCE: &str =
    "unauthenticated observation; not proof of human approval, test execution, or deployment";

/// What a case run Chough executed says it is, and is not (F12g): the test did run, so it
/// does not deny the execution.
pub(super) const EXECUTED_PROVENANCE: &str =
    "unauthenticated observation; not proof of human approval or deployment";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub(super) enum Kind {
    ProductTest,
    Structural,
    HumanObservation,
    DeliveryObservation,
    PublicationObservation,
    InstallationObservation,
    DeploymentObservation,
    LiveHealthObservation,
    DryRun,
}

impl Kind {
    fn name(&self) -> &'static str {
        match self {
            Kind::ProductTest => "product-test",
            Kind::Structural => "structural",
            Kind::HumanObservation => "human-observation",
            Kind::DeliveryObservation => "delivery-observation",
            Kind::PublicationObservation => "publication-observation",
            Kind::InstallationObservation => "installation-observation",
            Kind::DeploymentObservation => "deployment-observation",
            Kind::LiveHealthObservation => "live-health-observation",
            Kind::DryRun => "dry-run",
        }
    }

    fn parse(text: &str) -> Result<Kind> {
        Kind::from_str(text, true).map_err(|_| anyhow::anyhow!("unknown evidence kind: {text}"))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub(super) enum Outcome {
    Passed,
    Failed,
    NotRun,
}

impl Outcome {
    pub(super) fn name(&self) -> &'static str {
        match self {
            Outcome::Passed => "passed",
            Outcome::Failed => "failed",
            Outcome::NotRun => "not-run",
        }
    }

    fn parse(text: &str) -> Result<Outcome> {
        Outcome::from_str(text, true).map_err(|_| anyhow::anyhow!("unknown result: {text}"))
    }
}

// The value enums' FromStr.
// The entry.

/// One evidence entry (REQ-cli-017).
#[derive(Clone, Debug)]
pub(super) struct Evidence {
    pub(super) id: String,
    pub(super) aliases: Vec<String>,
    pub(super) criteria: Vec<String>,
    pub(super) requirements: Vec<String>,
    pub(super) contracts: Vec<String>,
    pub(super) kind: Kind,
    pub(super) outcome: Outcome,
    pub(super) environment: String,
    /// The one-line claim.
    pub(super) summary: String,
    /// The ID was supplied explicitly: a clash refuses, never suffixes (REQ-cli-023).
    pub(super) explicit_id: bool,
    /// Project-local artifacts, fingerprinted (paths are project-relative).
    pub(super) artifacts: Vec<String>,
    pub(super) artifact_fingerprints: BTreeMap<String, String>,
    /// An external artifact's neutral ID (REQ-records-005); its path is local-only.
    pub(super) external: Option<String>,
    /// The screenshot beside the entry (REQ-records-013).
    pub(super) screenshot: Option<String>,
    /// The SHA-256 of the captured output, the external file or the committed
    /// screenshot bytes, as `sha256:<64 hex>`.
    pub(super) fingerprint: Option<String>,
    /// The CI run that holds the raw log, when one does.
    pub(super) raw_log: Option<String>,
    pub(super) observed_at_unix_seconds: u64,
    pub(super) snapshot: Snapshot,
    pub(super) cases: Vec<CaseRef>,
    pub(super) method: Option<String>,
    pub(super) track: Option<String>,
    pub(super) argv: Vec<String>,
    pub(super) exit_code: Option<i32>,
    pub(super) tests_run: Option<u64>,
    pub(super) limitations: Vec<String>,
    /// Key failing lines kept in the body (REQ-records-005).
    pub(super) key_lines: Vec<String>,
    /// How many recorded lines were redacted (REQ-records-006).
    pub(super) redacted: usize,
    pub(super) actor: Option<crate::actor::Actor>,
    pub(super) duration_seconds: Option<u64>,
}

impl Evidence {
    /// The record for this entry; `name` is the file name the ID matches.
    fn to_record(&self, name: &str) -> Record {
        let stop = if self.summary.ends_with(['.', '!', '?']) {
            ""
        } else {
            "."
        };
        let mut body = format!(
            "**{}{stop}**\n\nRecorded as {} in {} at revision {}, observed {}.",
            self.summary,
            self.outcome.name(),
            self.environment,
            &self.snapshot.revision[..self.snapshot.revision.len().min(12)],
            iso_time(self.observed_at_unix_seconds),
        );
        if self.redacted > 0 {
            body.push_str(&format!(
                " {} recorded line{} redacted.",
                self.redacted,
                if self.redacted == 1 { " was" } else { "s were" }
            ));
        }
        if self.raw_log.is_some() {
            body.push_str(" The raw log is the named CI run's artifact.");
        } else if self.argv.is_empty() {
            body.push('\n');
        } else {
            body.push_str(" The raw log stays in the machine-local store.\n");
        }
        if !self.key_lines.is_empty() {
            body.push_str("\n## Key failing lines\n\n");
            for line in &self.key_lines {
                body.push_str(&format!("- {line}\n"));
            }
        }
        let mut record = Record::new(&body);
        let set = |record: &mut Record, key: &str, value: &str| {
            record.set_scalar(key, value).expect("a scalar value")
        };
        let set_list = |record: &mut Record, key: &str, values: &[String]| {
            record
                .set(key, markdown::Value::List(values.to_vec()))
                .expect("a list value")
        };
        set(&mut record, "kind", "evidence");
        set(&mut record, "id", name);
        set_list(&mut record, "aliases", &self.aliases);
        set(&mut record, "claim", &self.summary);
        set(&mut record, "result", self.outcome.name());
        set(&mut record, "evidence_kind", self.kind.name());
        if let Some(method) = &self.method {
            set(&mut record, "method", method);
        }
        if let Some(track) = &self.track {
            set(&mut record, "track", track);
        }
        if !self.cases.is_empty() {
            set_list(
                &mut record,
                "case",
                &self
                    .cases
                    .iter()
                    .map(|case| case.id.clone())
                    .collect::<Vec<_>>(),
            );
            set_list(
                &mut record,
                "case_digest",
                &self
                    .cases
                    .iter()
                    .map(|case| case.digest.clone())
                    .collect::<Vec<_>>(),
            );
        }
        if !self.argv.is_empty() {
            set_list(&mut record, "command", &self.argv);
        }
        if let Some(code) = self.exit_code {
            set(&mut record, "exit_code", &code.to_string());
        }
        if let Some(count) = self.tests_run {
            set(&mut record, "tests_run", &count.to_string());
        }
        if let Some(duration) = self.duration_seconds {
            set(&mut record, "duration_seconds", &duration.to_string());
        }
        set(&mut record, "revision", &self.snapshot.revision);
        if let Some(content) = &self.snapshot.content {
            set(&mut record, "content", content);
        }
        if let Some(inventory) = &self.snapshot.input_inventory {
            inventory.to_record(&mut record);
        }
        set(&mut record, "environment", &self.environment);
        if let Some(fingerprint) = &self.fingerprint {
            set(&mut record, "fingerprint", fingerprint);
        }
        if let Some(raw_log) = &self.raw_log {
            set(&mut record, "raw_log", raw_log);
        }
        if let Some(external) = &self.external {
            set(&mut record, "external", external);
        }
        if let Some(screenshot) = &self.screenshot {
            set(&mut record, "screenshot", screenshot);
        }
        if !self.artifacts.is_empty() {
            set_list(&mut record, "artifacts", &self.artifacts);
            record
                .set(
                    "artifact_fingerprints",
                    markdown::Value::Maps(
                        self.artifact_fingerprints
                            .iter()
                            .map(|(path, digest)| {
                                vec![
                                    ("path".into(), path.clone()),
                                    ("digest".into(), digest.clone()),
                                ]
                            })
                            .collect(),
                    ),
                )
                .expect("a maps value");
        }
        set_list(&mut record, "criteria", &self.criteria);
        set_list(&mut record, "requirements", &self.requirements);
        set_list(&mut record, "contracts", &self.contracts);
        set_list(&mut record, "limitations", &self.limitations);
        set(
            &mut record,
            "observed",
            &iso_time(self.observed_at_unix_seconds),
        );
        if let Some(actor) = &self.actor {
            record
                .set("actor", actor.front_value())
                .expect("the actor map");
        }
        record
    }

    /// Read an entry back from its record.
    fn from_record(record: &Record) -> Result<Evidence> {
        let scalar = |key: &str| record.scalar(key).map(str::to_string);
        let need = |key: &str| scalar(key).with_context(|| format!("evidence entry lacks {key}"));
        let number = |key: &str| -> Result<Option<u64>> {
            scalar(key)
                .map(|text| {
                    text.parse::<u64>()
                        .with_context(|| format!("evidence entry's {key} is not a number"))
                })
                .transpose()
        };
        let observed = crate::session_md::parse_time(&need("observed")?)
            .context("evidence entry's observed time")?;
        // The working content identity is a SHA-256 digest when present (REQ-cli-017),
        // as the writer spells it: 64 lowercase hexadecimal digits.
        let content = scalar("content");
        if let Some(content) = &content {
            ensure!(
                content.len() == 64
                    && content
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
                "invalid evidence entry {}: content must be a SHA-256 digest (64 lowercase hexadecimal digits)",
                scalar("id").unwrap_or_default()
            );
        }
        let actor = record.maps("actor").first().and_then(|map| {
            crate::actor::Actor::from_front(&markdown::Value::Maps(vec![map.clone()]))
        });
        Ok(Evidence {
            id: need("id")?,
            explicit_id: false,
            aliases: record.list("aliases"),
            criteria: record.list("criteria"),
            requirements: record.list("requirements"),
            contracts: record.list("contracts"),
            kind: Kind::parse(&need("evidence_kind")?)?,
            outcome: Outcome::parse(&need("result")?)?,
            environment: need("environment")?,
            summary: need("claim")?,
            artifacts: record.list("artifacts"),
            artifact_fingerprints: record
                .maps("artifact_fingerprints")
                .into_iter()
                .map(|map| {
                    let mut fields = map.into_iter().collect::<BTreeMap<_, _>>();
                    let path = fields.remove("path").context("artifact map lacks path")?;
                    let digest = fields
                        .remove("digest")
                        .context("artifact map lacks digest")?;
                    Ok((path, digest))
                })
                .collect::<Result<BTreeMap<_, _>>>()?,
            external: scalar("external"),
            screenshot: scalar("screenshot"),
            fingerprint: scalar("fingerprint"),
            raw_log: scalar("raw_log"),
            observed_at_unix_seconds: observed,
            snapshot: Snapshot {
                revision: need("revision")?,
                dirty: false,
                fingerprint: String::new(),
                content,
                input_inventory: InputInventory::from_record(record)?,
            },
            cases: {
                let ids = record.list("case");
                let digests = record.list("case_digest");
                ensure!(
                    ids.len() == digests.len(),
                    "evidence entry's case and case_digest lists differ in length"
                );
                ids.into_iter()
                    .zip(digests)
                    .map(|(id, digest)| CaseRef { id, digest })
                    .collect()
            },
            method: scalar("method"),
            track: scalar("track"),
            argv: record.list("command"),
            exit_code: number("exit_code")?.map(|code| code as i32),
            tests_run: number("tests_run")?,
            limitations: record.list("limitations"),
            key_lines: record
                .section("Key failing lines")
                .map(|section| {
                    section
                        .lines()
                        .filter_map(|line| line.strip_prefix("- ").map(str::to_string))
                        .collect()
                })
                .unwrap_or_default(),
            redacted: 0,
            actor,
            duration_seconds: number("duration_seconds")?,
        })
    }
}

/// `YYYY-MM-DDTHH:MM:SSZ` for a Unix time.
fn iso_time(secs: u64) -> String {
    crate::session_md::iso_time(secs)
}

// ---------------------------------------------------------------------------
// Paths and writing.

/// The evidence folder of a session.
fn evidence_dir(session_id: &str) -> String {
    format!(".chough/sessions/{session_id}/evidence")
}

/// The active session's ID, or a refusal naming how to start one.
fn active_session(root: &Path) -> Result<String> {
    let relative = crate::sessions::checkpoint_path(root)?;
    crate::sessions::id_of(&relative).with_context(|| format!("no session at {relative}"))
}

/// Write an entry by create-new (REQ-records-001), its ID equal to the file stem, then
/// its side files, each new: the key screenshot beside it (REQ-records-013), the raw
/// log under `.chough/local/logs/` and an external artifact's local record
/// (REQ-records-005). The name holds for all of them: an automatic name taken by an
/// entry, a log or a screenshot gets one clash suffix, and that name taken too
/// refuses; an explicit name taken refuses (REQ-cli-023). A refusal writes nothing,
/// and a side file that fails removes what this write made. The entry is then
/// registered in the session's `observations` list, which the session writer links
/// in What was verified (REQ-records-004). Returns the entry's path.
pub(super) fn record(root: &Path, evidence: &Evidence, files: EntryFiles) -> Result<String> {
    validate_record(evidence, &evidence.id)?;
    let session_id = active_session(root)?;
    let dir = evidence_dir(&session_id);
    let mut entry = evidence.clone();
    if let Some((_, bytes)) = &files.screenshot {
        ensure!(
            entry.external.is_none() && entry.fingerprint.is_none(),
            "an entry holds one fingerprint: record the screenshot and the external artifact as separate entries"
        );
        screenshot_room(root, &session_id, bytes)?;
        entry.fingerprint = Some(format!("sha256:{}", crate::records::digest_bytes(bytes)));
    }
    let taken = |name: &str| -> Result<bool> {
        let mut paths = vec![format!("{dir}/{name}.md")];
        if files.log.is_some() {
            paths.push(crate::local::log_path(name));
        }
        if let Some((extension, _)) = &files.screenshot {
            paths.push(format!("{dir}/{name}.{extension}"));
        }
        for path in paths {
            if local_path(root, &path)?.exists() {
                return Ok(true);
            }
        }
        Ok(false)
    };
    let name = if evidence.explicit_id {
        ensure!(
            !taken(&evidence.id)?,
            "observation identity collision: {} already exists; record a new observation or omit the ID",
            evidence.id
        );
        evidence.id.clone()
    } else if !taken(&evidence.id)? {
        evidence.id.clone()
    } else {
        let suffixed = identity::suffixed(&evidence.id)?;
        ensure!(
            !taken(&suffixed)?,
            "refused: {dir}/{suffixed}.md or its log or screenshot also exists; nothing was written and no other name was drawn"
        );
        suffixed
    };
    if let Some((extension, _)) = &files.screenshot {
        entry.screenshot = Some(format!("{name}.{extension}"));
    }
    let mut record = entry.to_record(&name);
    if leaks::redacting() {
        record.redact_leaks(leaks::Local::current());
    } else {
        record.check_leaks(leaks::Local::current())?;
    }
    let text = record.render_within(markdown::Kind::Evidence)?;
    let target = format!("{dir}/{name}.md");
    ensure!(
        crate::records::write_new_bounded(root, &target, &text, markdown::RECORD_BOUND)?,
        "observation identity collision: {name} already exists; nothing was written"
    );
    let mut written = vec![target.clone()];
    if let Err(error) = write_side_files(root, &dir, &name, &files, &mut written) {
        for path in written.iter().rev() {
            if let Ok(path) = local_path(root, path) {
                let _ = fs::remove_file(path);
            }
        }
        return Err(error);
    }

    let mut session = match crate::checkpoint::read_session(root)? {
        crate::checkpoint::Session::V2(session) => *session,
    };
    if !session.observations.contains(&name) {
        session.observations.push(name.clone());
    }
    crate::checkpoint::save_checkpoint(root, &mut session)?;
    Ok(target)
}

/// The side files of a written entry, each by create-new, adding each path written.
fn write_side_files(
    root: &Path,
    dir: &str,
    name: &str,
    files: &EntryFiles,
    written: &mut Vec<String>,
) -> Result<()> {
    if let Some((extension, bytes)) = &files.screenshot {
        let relative = format!("{dir}/{name}.{extension}");
        ensure!(
            crate::records::write_new_bytes(root, &relative, bytes)?,
            "the screenshot {relative} already exists; nothing was written"
        );
        written.push(relative);
    }
    if let Some(log) = &files.log {
        written.push(crate::local::write_log(root, name, log)?);
    }
    if let Some(external) = &files.external {
        crate::local::save_external(root, &external.id, &external.path, &external.fingerprint)?;
    }
    Ok(())
}

/// The ID an entry's recorded path names: its file stem (REQ-cli-017).
pub(super) fn recorded_id(path: &str) -> String {
    path.rsplit('/')
        .next()
        .unwrap_or(path)
        .trim_end_matches(".md")
        .to_string()
}

// ---------------------------------------------------------------------------
// `evidence add`.

#[derive(clap::Args)]
pub(super) struct EvidenceArgs {
    /// The entry's name; a date and slug not in use. Omit for automatic naming.
    id: Option<String>,
    /// The HI criteria the observation shows; at least one.
    #[arg(long = "criterion", required = true)]
    criteria: Vec<String>,
    /// A requirement the observation shows, validated when given: held by a companion,
    /// or added or modified by the active session's native change in its approved delta.
    #[arg(long = "requirement")]
    requirements: Vec<String>,
    /// A native contract, `specs/<module>/<module>.spec.md`, validated when given.
    #[arg(long = "contract")]
    contracts: Vec<String>,
    #[arg(long)]
    kind: Kind,
    #[arg(long)]
    outcome: Outcome,
    #[arg(long)]
    environment: String,
    #[arg(long)]
    summary: String,
    /// A project-local artifact to fingerprint, or an absolute path outside the
    /// project, fingerprinted in place and never copied (REQ-records-005).
    #[arg(long = "artifact")]
    artifacts: Vec<String>,
    /// Key failing lines of a manual failure, within the bounds of REQ-records-005.
    #[arg(long)]
    failing_lines: Option<String>,
    /// One small key screenshot committed beside the entry (REQ-records-013).
    #[arg(long)]
    screenshot: Option<String>,
    /// The CI run holding the raw log, when one does.
    #[arg(long)]
    raw_log: Option<String>,
    /// Link a test-case definition; its current digest is bound to the observation.
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long)]
    method: Option<String>,
    #[arg(long)]
    track: Option<String>,
    #[arg(long = "limitation")]
    limitations: Vec<String>,
    /// Replace values that would leak with `<redacted:<kind>>` (REQ-records-006).
    #[arg(long)]
    redact: bool,
}

#[derive(clap::Subcommand)]
pub(super) enum EvidenceCommand {
    Add(Box<EvidenceArgs>),
}

pub(super) fn run(root: &Path, command: EvidenceCommand) -> Result<()> {
    match command {
        EvidenceCommand::Add(args) => add(root, *args),
    }
}

/// Key failing lines within the REQ-records-005 bounds, as a case run keeps them:
/// complete terminal escape sequences removed, blank lines dropped, each match of
/// REQ-records-006 redacted (never refused) and counted, then the last lines that fit
/// in 20 lines and 2,048 bytes.
pub(super) fn key_lines(text: &str) -> (Vec<String>, usize) {
    let local = leaks::Local::current();
    let mut redacted = 0;
    let lines = text_lines(text)
        .into_iter()
        .map(|line| {
            let (clean, count) = leaks::redact(local, &line);
            redacted += count;
            clean
        })
        .collect();
    (bounded_tail(lines), redacted)
}

/// `evidence add --failing-lines`: the same lines and bounds, the kept text checked as
/// REQ-records-006 checks other text: a leak refuses, naming the field and the kind,
/// or under `--redact` is redacted and counted.
fn manual_key_lines(text: &str) -> Result<(Vec<String>, usize)> {
    if leaks::redacting() {
        return Ok(key_lines(text));
    }
    let kept = bounded_tail(text_lines(text));
    for line in &kept {
        leaks::check(leaks::Local::current(), "--failing-lines", line)?;
    }
    Ok((kept, 0))
}

/// The non-blank lines of `text`, escape sequences removed.
fn text_lines(text: &str) -> Vec<String> {
    let (visible, _) = cases::strip_escapes(text.replace("\r\n", "\n").as_bytes());
    String::from_utf8_lossy(&visible)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}

/// The last of `lines` that fit in 20 lines and 2,048 bytes.
fn bounded_tail(mut lines: Vec<String>) -> Vec<String> {
    if lines.len() > 20 {
        lines.drain(..lines.len() - 20);
    }
    let mut bytes: usize = lines.iter().map(|line| line.len() + 1).sum();
    while bytes > 2_048 && !lines.is_empty() {
        bytes -= lines.remove(0).len() + 1;
    }
    lines
}

/// Fingerprint an `--artifact` path: absolute paths outside the project are
/// fingerprinted in place and named by a neutral ID; project-relative paths keep their
/// place and digest. Returns (external ID, project-local fingerprints).
pub(super) fn bind_artifacts(root: &Path, artifacts: &[String]) -> Result<BoundArtifacts> {
    let mut bound = BoundArtifacts::default();
    for artifact in artifacts {
        if Path::new(artifact).is_absolute() {
            ensure!(
                bound.external.is_none(),
                "one external artifact per entry; record another with its own entry"
            );
            // The path is kept one quoted line in the local map; a control character
            // would break that record and every later read of it.
            ensure!(
                !artifact.chars().any(char::is_control),
                "external artifact {artifact:?} has a control character in its path; rename or move it, then record it; nothing was recorded"
            );
            bound.external = Some(bind_external(root, artifact)?);
        } else {
            ensure!(
                canonical_path(artifact).as_deref() == Some(artifact.as_str()),
                "artifact path must be spelled canonically (relative, `/`-separated, no empty, `.` or `..` parts, no trailing `/`): {artifact}"
            );
            ensure!(
                local_path(root, artifact)?.is_file(),
                "missing artifact: {artifact}"
            );
            ensure!(
                !bound.fingerprints.contains_key(artifact),
                "duplicate artifact paths"
            );
            bound.fingerprints.insert(
                artifact.clone(),
                crate::records::file_digest(root, artifact)?,
            );
            bound.paths.push(artifact.clone());
        }
    }
    Ok(bound)
}

/// An entry's artifacts: the project-local paths in their order with their digests,
/// and the one external artifact, which the entry names only by its neutral ID.
#[derive(Default)]
pub(super) struct BoundArtifacts {
    pub(super) paths: Vec<String>,
    pub(super) fingerprints: BTreeMap<String, String>,
    pub(super) external: Option<ExternalBinding>,
}

/// An external artifact fingerprinted in place (REQ-records-005): its neutral
/// `ext-<6 hex>` ID, its machine-local path and `sha256:<64 hex>`. The local record is
/// written only after the entry.
#[derive(Clone)]
pub(super) struct ExternalBinding {
    pub(super) id: String,
    pub(super) path: String,
    pub(super) fingerprint: String,
}

/// Fingerprint an external file in place and draw its neutral ID; a missing path, a
/// directory, a symlink or a file over 64 MiB refuses, naming the path on standard
/// error only (REQ-records-005).
fn bind_external(root: &Path, path: &str) -> Result<ExternalBinding> {
    let target = Path::new(path);
    ensure!(
        target.exists() && !target.is_dir() && !target.is_symlink(),
        "external artifact {path} cannot be read (missing, a directory or a symlink); nothing was recorded"
    );
    ensure!(
        target.metadata()?.len() <= 64 * 1_048_576,
        "external artifact {path} exceeds the 64 MiB bound; nothing was recorded"
    );
    let bytes =
        fs::read(target).with_context(|| format!("reading the external artifact {path}"))?;
    Ok(ExternalBinding {
        id: crate::local::draw_external_id(root)?,
        path: path.to_string(),
        fingerprint: format!("sha256:{}", crate::records::digest_bytes(&bytes)),
    })
}

fn add(root: &Path, args: EvidenceArgs) -> Result<()> {
    let _redact_guard = RedactGuard::new(args.redact);
    crate::checkpoint::read_session(root)?;
    if let Some(id) = &args.id {
        ensure!(valid_name(id), "invalid evidence ID");
        identity::require_explicit_observation_id(id)?;
    }
    ensure!(
        !args.environment.trim().is_empty() && !args.summary.trim().is_empty(),
        "environment and summary must not be blank"
    );
    validate_criteria(root, &args.criteria)?;
    validate_evidence_links(root, &args.contracts, &args.requirements)?;
    let mut case_refs = vec![];
    for id in &args.cases {
        let digest = cases::current_digest(root, id)?
            .with_context(|| format!("unknown or retired case: {id}"))?;
        case_refs.push(CaseRef {
            id: id.clone(),
            digest,
        });
    }
    let bound = bind_artifacts(root, &args.artifacts)?;
    let screenshot_data = match &args.screenshot {
        Some(path) => Some(read_screenshot(path)?),
        None => None,
    };
    ensure!(
        bound.external.is_none() || screenshot_data.is_none(),
        "an entry holds one fingerprint: record the screenshot and the external artifact as separate entries"
    );
    let date = identity::utc_date(now()?);
    let source = args
        .cases
        .first()
        .cloned()
        .unwrap_or_else(|| args.summary.clone());
    let base = identity::dated_name(&date, &source, "evidence");
    let id = args.id.clone().unwrap_or(base);
    let (key_lines, redacted) = match &args.failing_lines {
        Some(text) if args.outcome != Outcome::Passed => manual_key_lines(text)?,
        _ => (vec![], 0),
    };
    let mut limitations = args.limitations.clone();
    if let Some(note) = crate::local::toolchain_limitation(root) {
        limitations.push(note);
    }
    let evidence = Evidence {
        id,
        explicit_id: args.id.is_some(),
        aliases: vec![],
        criteria: args.criteria,
        requirements: args.requirements,
        contracts: args.contracts,
        kind: args.kind,
        outcome: args.outcome,
        environment: args.environment,
        summary: args.summary,
        artifacts: bound.paths,
        artifact_fingerprints: bound.fingerprints,
        external: bound.external.as_ref().map(|external| external.id.clone()),
        screenshot: None,
        fingerprint: bound
            .external
            .as_ref()
            .map(|external| external.fingerprint.clone()),
        raw_log: args.raw_log,
        observed_at_unix_seconds: now()?,
        snapshot: snapshot(root)?,
        cases: case_refs,
        method: args.method,
        track: args.track,
        argv: vec![],
        exit_code: None,
        tests_run: None,
        limitations,
        key_lines,
        redacted,
        actor: crate::actor::Actor::current(),
        duration_seconds: None,
    };
    let name = record(
        root,
        &evidence,
        EntryFiles {
            screenshot: screenshot_data,
            log: None,
            external: bound.external,
        },
    )?;
    // The stored ID, with any clash suffix, so automation never parses it out of
    // `recorded` (#60, REQ-cli-017).
    print_json(&json!({
        "schema_version": 1,
        "observation": recorded_id(&name),
        "recorded": name,
        "provenance": PROVENANCE,
    }))
}

/// Redact for the command when asked (REQ-records-006), restored at its end.
struct RedactGuard(bool);

impl RedactGuard {
    fn new(on: bool) -> Self {
        if on {
            leaks::set_redacting(true);
        }
        RedactGuard(on)
    }
}

impl Drop for RedactGuard {
    fn drop(&mut self) {
        if self.0 {
            leaks::set_redacting(false);
        }
    }
}

pub(super) fn validate_record(evidence: &Evidence, name: &str) -> Result<()> {
    ensure!(valid_name(&evidence.id), "invalid evidence ID: {name}");
    ensure!(
        evidence
            .criteria
            .iter()
            .all(|id| crate::records::valid_criterion_id(id)),
        "an evidence criterion is not a criterion ID: {name}"
    );
    ensure!(
        evidence
            .requirements
            .iter()
            .all(|id| crate::records::valid_reference_id(id)),
        "an evidence requirement is not a requirement ID: {name}"
    );
    for case in &evidence.cases {
        ensure!(
            !case.digest.is_empty() && crate::records::is_digest(&case.digest),
            "evidence case {} has no definition digest: {name}",
            case.id
        );
    }
    for digest in evidence.artifact_fingerprints.values() {
        ensure!(
            crate::records::is_digest(digest),
            "invalid artifact digest in {name}"
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Key screenshots (REQ-records-013).

const SCREENSHOT_BOUND: u64 = 524_288;
/// The bounds and remedies every screenshot refusal names (REQ-records-013).
const SCREENSHOT_REMEDY: &str = "a key screenshot is a PNG or JPEG of at most 524,288 bytes, at most 5 and 2,097,152 bytes per session; crop or compress it, or record it by fingerprint with --artifact";
const SCREENSHOTS_PER_SESSION: usize = 5;
const SCREENSHOTS_SESSION_BYTES: u64 = 2_097_152;

/// Strip a PNG's metadata chunks (tEXt, zTXt, iTXt, tIME, eXIf).
fn strip_png(bytes: &[u8]) -> Result<Vec<u8>> {
    const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    ensure!(bytes.starts_with(PNG_SIGNATURE), "not a PNG");
    let mut out = PNG_SIGNATURE.to_vec();
    let mut at = PNG_SIGNATURE.len();
    while at + 8 <= bytes.len() {
        let length =
            u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]]) as usize;
        let kind = &bytes[at + 4..at + 8];
        let end = at + 8 + length + 4;
        ensure!(end <= bytes.len(), "a PNG chunk overruns the file");
        if ![b"tEXt".as_slice(), b"zTXt", b"iTXt", b"tIME", b"eXIf"].contains(&kind) {
            out.extend_from_slice(&bytes[at..end]);
        }
        at = end;
    }
    Ok(out)
}

/// Strip a JPEG's APP1..APP15 and COM segments.
fn strip_jpeg(bytes: &[u8]) -> Result<Vec<u8>> {
    ensure!(bytes.starts_with(b"\xff\xd8"), "not a JPEG");
    let mut out = b"\xff\xd8".to_vec();
    let mut at = 2;
    while at + 4 <= bytes.len() {
        ensure!(bytes[at] == 0xff, "a JPEG segment is malformed");
        let marker = bytes[at + 1];
        // Start of scan: the image data is copied as it is.
        if marker == 0xda {
            out.extend_from_slice(&bytes[at..]);
            return Ok(out);
        }
        if marker == 0xd8 || marker == 0x01 || (0xd0..=0xd7).contains(&marker) {
            out.extend_from_slice(&bytes[at..at + 2]);
            at += 2;
            continue;
        }
        let length = u16::from_be_bytes([bytes[at + 2], bytes[at + 3]]) as usize;
        let end = at + 2 + length;
        ensure!(end <= bytes.len(), "a JPEG segment overruns the file");
        let metadata = (0xe1..=0xef).contains(&marker) || marker == 0xfe;
        if !metadata {
            out.extend_from_slice(&bytes[at..end]);
        }
        at = end;
    }
    bail!("a JPEG with no image data")
}

/// Read, strip and stage a screenshot for an entry; the committed write happens with
/// the entry's name in `record`. Returns (extension, stripped bytes).
fn read_screenshot(path: &str) -> Result<(String, Vec<u8>)> {
    let target = Path::new(path);
    ensure!(
        target.exists() && !target.is_dir() && !target.is_symlink(),
        "the screenshot cannot be read (missing, a directory or a symlink); {SCREENSHOT_REMEDY}"
    );
    let bytes = fs::read(target).context("reading the screenshot")?;
    ensure!(
        bytes.len() as u64 <= SCREENSHOT_BOUND,
        "the screenshot is over the 524,288-byte bound; {SCREENSHOT_REMEDY}"
    );
    if bytes.starts_with(b"\x89PNG") {
        Ok(("png".into(), strip_png(&bytes)?))
    } else if bytes.starts_with(b"\xff\xd8") {
        Ok(("jpg".into(), strip_jpeg(&bytes)?))
    } else {
        bail!("the screenshot is not a PNG or JPEG; {SCREENSHOT_REMEDY}")
    }
}

/// The session's screenshot count and total bytes so far.
fn screenshot_use(root: &Path, session_id: &str) -> (usize, u64) {
    let dir = match local_path(root, &evidence_dir(session_id)) {
        Ok(dir) => dir,
        Err(_) => return (0, 0),
    };
    let mut count = 0;
    let mut bytes = 0;
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with(".png") || name.ends_with(".jpg") {
                count += 1;
                bytes += entry.metadata().map(|meta| meta.len()).unwrap_or(0);
            }
        }
    }
    (count, bytes)
}

/// Write the screenshot beside its entry, named by the entry's ID, by create-new;
/// nothing is written when a bound is exceeded.
/// Whether the session has room for one more screenshot of these bytes: at most 5 and
/// 2,097,152 bytes in all (REQ-records-013), checked before anything is written.
fn screenshot_room(root: &Path, session_id: &str, bytes: &[u8]) -> Result<()> {
    let (count, total) = screenshot_use(root, session_id);
    ensure!(
        count < SCREENSHOTS_PER_SESSION,
        "a session holds at most {SCREENSHOTS_PER_SESSION} screenshots; {SCREENSHOT_REMEDY}"
    );
    ensure!(
        total + bytes.len() as u64 <= SCREENSHOTS_SESSION_BYTES,
        "the session's screenshots would pass 2,097,152 bytes in all; {SCREENSHOT_REMEDY}"
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// Reading.

/// Every evidence entry of every open session, in folder and name order.
pub(super) fn load_all(root: &Path) -> Result<Vec<Evidence>> {
    let mut found = vec![];
    for session_id in crate::sessions::open_ids(root)? {
        found.extend(load_session(root, &session_id)?);
    }
    Ok(found)
}

/// The evidence entries of one session.
fn load_session(root: &Path, session_id: &str) -> Result<Vec<Evidence>> {
    let dir = evidence_dir(session_id);
    let mut found = vec![];
    let folder = local_path(root, &dir)?;
    let Ok(entries) = fs::read_dir(&folder) else {
        return Ok(found);
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md"))
        .collect();
    names.sort();
    for name in names {
        let relative = format!("{dir}/{name}");
        let text = read_bounded_record(&folder.join(&name), markdown::RECORD_BOUND)
            .with_context(|| format!("reading {relative}"))?;
        found.push(Evidence::from_record(&markdown::parse(&relative, &text)?)?);
    }
    Ok(found)
}

/// A receipt resolved from an archived story or a bug, as a partial entry: id,
/// aliases, result, observed and revision are what a receipt carries (the t38b/t382
/// agreement); freshness of a receipt stays revision-bound (REQ-cli-017).
fn evidence_from_receipt(map: &[(String, String)]) -> Result<Evidence> {
    let get = |key: &str| {
        map.iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
    };
    let id = get("id").context("a receipt lacks its id")?;
    Ok(Evidence {
        id,
        explicit_id: false,
        aliases: get("aliases")
            .map(|text| text.split(", ").map(str::to_string).collect())
            .unwrap_or_default(),
        criteria: vec![],
        requirements: vec![],
        contracts: vec![],
        kind: Kind::Structural,
        outcome: Outcome::parse(&get("result").context("a receipt lacks its result")?)?,
        environment: get("environment").unwrap_or_default(),
        summary: get("claim").unwrap_or_default(),
        artifacts: vec![],
        artifact_fingerprints: BTreeMap::new(),
        external: None,
        screenshot: None,
        fingerprint: get("fingerprint"),
        raw_log: get("raw_log"),
        observed_at_unix_seconds: get("observed")
            .map(|text| crate::session_md::parse_time(&text))
            .transpose()
            .context("a receipt's observed time")?
            .unwrap_or(0),
        snapshot: Snapshot {
            revision: get("revision").unwrap_or_default(),
            dirty: false,
            fingerprint: String::new(),
            content: get("content"),
            input_inventory: None,
        },
        cases: vec![],
        method: None,
        track: None,
        argv: vec![],
        exit_code: None,
        tests_run: None,
        limitations: vec![],
        key_lines: vec![],
        redacted: 0,
        actor: None,
        duration_seconds: get("duration_seconds").and_then(|text| text.parse().ok()),
    })
}

/// A `receipts:` map with the name of the record holding it.
type ReceiptMaps = Vec<(String, Vec<(String, String)>)>;

/// Every `receipts:` map in the Markdown records under a folder, with the record name.
/// Where archived session stories are (REQ-history-006).
const STORIES_DIR: &str = ".chough/archive/sessions";

fn receipt_maps(root: &Path, dir: &str, key: &str) -> Result<ReceiptMaps> {
    let folder = local_path(root, dir)?;
    let mut found = vec![];
    let Ok(entries) = fs::read_dir(&folder) else {
        return Ok(found);
    };
    let mut names: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md"))
        .collect();
    names.sort();
    for name in names {
        let text = read_bounded_record(&folder.join(&name), markdown::SESSION_BOUND)?;
        let record = markdown::parse(&name, &text)?;
        // REQ-history-006: an archived story's receipts are its `## Receipts` section's,
        // or an older story's front matter's.
        let maps = if dir == STORIES_DIR {
            record.story_receipts(&name)?
        } else {
            record.maps(key)
        };
        for map in maps {
            found.push((name.trim_end_matches(".md").to_string(), map));
        }
    }
    Ok(found)
}

/// The migration receipt a tombstone names: a receipt map `{id, removed: <receipt>}`
/// for an observation the 0.5.x migration removed without carrying (REQ-records-009).
/// A tombstone is never a receipt.
fn tombstone_of(map: &[(String, String)]) -> Option<&str> {
    map.iter()
        .find(|(key, _)| key == "removed")
        .map(|(_, receipt)| receipt.as_str())
}

/// The names a receipt map answers to: its ID, then each alias.
fn receipt_names(map: &[(String, String)]) -> Vec<String> {
    let get = |key: &str| {
        map.iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    };
    let mut names: Vec<String> = get("id").into_iter().map(str::to_string).collect();
    names.extend(
        get("aliases")
            .into_iter()
            .flat_map(|aliases| aliases.split(", "))
            .filter(|alias| !alias.is_empty())
            .map(str::to_string),
    );
    names
}

/// The folder of the migration receipts (REQ-records-009).
const MIGRATIONS_DIR: &str = ".chough/archive/migrations";

/// What an evidence ID names (REQ-records-003, REQ-records-009).
pub(super) enum Resolved {
    /// An open session's entry, or an archived story's or a bug's receipt read as one.
    Present(Box<Evidence>),
    /// An observation the 0.5.x migration removed, with the receipt that records it.
    Removed(String),
    /// Nothing holds it.
    Missing,
}

/// One record that can hold an evidence ID.
struct Holder {
    /// The session of an entry, or the story or bug of a receipt.
    place: String,
    names: Vec<String>,
    held: Held,
}

enum Held {
    Entry(Box<Record>),
    Receipt(Vec<(String, String)>),
}

impl Holder {
    fn evidence(&self) -> Result<Evidence> {
        match &self.held {
            Held::Entry(record) => Evidence::from_record(record),
            Held::Receipt(map) => evidence_from_receipt(map),
        }
    }
}

/// Every record an evidence ID can name, read once per command: the open sessions'
/// entries, the archived stories' and the bugs' receipts, and the IDs the 0.5.x
/// migration removed, from the tombstones and the migration receipts' aliases. An
/// entry or receipt is read as evidence only when an ID names it.
#[derive(Default)]
pub(super) struct Index {
    /// Entries and story receipts, which resolve first.
    first: Vec<Holder>,
    /// Bug receipts, which resolve after them.
    bugs: Vec<Holder>,
    by_name: BTreeMap<String, Vec<(bool, usize)>>,
    removed: BTreeMap<String, String>,
}

impl Index {
    pub(super) fn build(root: &Path) -> Result<Index> {
        let mut index = Index::default();
        for session_id in crate::sessions::open_ids(root)? {
            let dir = evidence_dir(&session_id);
            let folder = local_path(root, &dir)?;
            let Ok(entries) = fs::read_dir(&folder) else {
                continue;
            };
            let mut names: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .filter(|name| name.ends_with(".md"))
                .collect();
            names.sort();
            for name in names {
                let relative = format!("{dir}/{name}");
                let text = read_bounded_record(&folder.join(&name), markdown::RECORD_BOUND)
                    .with_context(|| format!("reading {relative}"))?;
                let record = markdown::parse(&relative, &text)?;
                let mut names: Vec<String> = record
                    .scalar("id")
                    .into_iter()
                    .map(str::to_string)
                    .collect();
                names.extend(record.list("aliases"));
                index.first.push(Holder {
                    place: session_id.clone(),
                    names,
                    held: Held::Entry(Box::new(record)),
                });
            }
        }
        let bugs_dir = local_path(root, crate::bugs::BUGS_DIR)?;
        let bug_maps = if bugs_dir.is_dir() {
            receipt_maps(root, crate::bugs::BUGS_DIR, "receipts")?
        } else {
            vec![]
        };
        for (in_bugs, maps) in [
            (false, receipt_maps(root, STORIES_DIR, "receipts")?),
            (true, bug_maps),
        ] {
            for (place, map) in maps {
                let names = receipt_names(&map);
                if let Some(receipt) = tombstone_of(&map) {
                    for name in names {
                        index
                            .removed
                            .entry(name)
                            .or_insert_with(|| receipt.to_string());
                    }
                    continue;
                }
                let holder = Holder {
                    place,
                    names,
                    held: Held::Receipt(map),
                };
                if in_bugs {
                    index.bugs.push(holder);
                } else {
                    index.first.push(holder);
                }
            }
        }
        index.read_migration_aliases(root)?;
        for (in_bugs, holders) in [(false, &index.first), (true, &index.bugs)] {
            for (position, holder) in holders.iter().enumerate() {
                for name in &holder.names {
                    index
                        .by_name
                        .entry(name.clone())
                        .or_default()
                        .push((in_bugs, position));
                }
            }
        }
        Ok(index)
    }

    /// The IDs each migration receipt's `## Aliases` maps to `removed`.
    fn read_migration_aliases(&mut self, root: &Path) -> Result<()> {
        let folder = local_path(root, MIGRATIONS_DIR)?;
        let Ok(entries) = fs::read_dir(&folder) else {
            return Ok(());
        };
        let mut names: Vec<String> = entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| name.ends_with(".md"))
            .collect();
        names.sort();
        for name in names {
            let relative = format!("{MIGRATIONS_DIR}/{name}");
            let text = read_bounded_record(&folder.join(&name), markdown::Kind::Receipt.bound())
                .with_context(|| format!("reading {relative}"))?;
            let record = markdown::parse(&relative, &text)?;
            let Some(aliases) = record.section("Aliases") else {
                continue;
            };
            for line in aliases.lines() {
                let mut quoted = line.split('`').skip(1).step_by(2);
                if let (Some(old), Some("removed")) = (quoted.next(), quoted.next()) {
                    self.removed
                        .entry(old.to_string())
                        .or_insert_with(|| relative.clone());
                }
            }
        }
        Ok(())
    }

    /// The holders named `name` in one tier, kept to `place` when the ID is qualified.
    fn named(&self, name: &str, place: Option<&str>, in_bugs: bool) -> Vec<&Holder> {
        let tier = if in_bugs { &self.bugs } else { &self.first };
        self.by_name
            .get(name)
            .into_iter()
            .flatten()
            .filter(|(bugs, _)| *bugs == in_bugs)
            .map(|(_, position)| &tier[*position])
            .filter(|holder| place.is_none_or(|place| holder.place == place))
            .collect()
    }

    /// Whether any entry or receipt holds the ID.
    pub(super) fn holds(&self, id: &str) -> bool {
        let (place, name) = split_qualified(id);
        !self.named(name, place, false).is_empty() || !self.named(name, place, true).is_empty()
    }

    /// Resolve an ID (bare, a 0.5.x alias, or qualified as `<place>/<id>`) as bugs.rs
    /// does: an open session's entry or an archived story's receipt first, where two
    /// holding it are ambiguous; then the bugs' receipts, where copies of one receipt
    /// are one; then the migration's removals.
    pub(super) fn resolve(&self, id: &str) -> Result<Resolved> {
        match self.lookup(id)? {
            Ok(resolved) => Ok(resolved),
            Err(ambiguous) => bail!("{}", ambiguous.refusal),
        }
    }

    /// As [`Index::resolve`], with an ambiguous ID as a value naming its places, so a
    /// report can label it instead of refusing (REQ-cli-023).
    fn lookup(&self, id: &str) -> Result<std::result::Result<Resolved, Ambiguous>> {
        let (place, name) = split_qualified(id);
        let places = |holders: &[&Holder]| -> Vec<String> {
            holders
                .iter()
                .map(|holder| format!("{}/{name}", holder.place))
                .collect()
        };
        match self.named(name, place, false).as_slice() {
            [one] => return Ok(Ok(Resolved::Present(Box::new(one.evidence()?)))),
            [] => {}
            many => {
                return Ok(Err(Ambiguous {
                    refusal: format!(
                        "refused: {id} names evidence in more than one place ({}); qualify it as <session-id>/<evidence-id>",
                        many.iter()
                            .map(|holder| holder.place.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    places: places(many),
                }));
            }
        }
        let bugs = self.named(name, place, true);
        if let Some(first) = bugs.first() {
            let same = bugs.iter().all(|holder| match (&holder.held, &first.held) {
                (Held::Receipt(map), Held::Receipt(other)) => map == other,
                _ => false,
            });
            if !same {
                return Ok(Err(Ambiguous {
                    refusal: format!(
                        "refused: {id} names differing receipts in bugs {}; qualify it as <bug-id>/<evidence-id>",
                        bugs.iter()
                            .map(|holder| holder.place.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                    places: places(&bugs),
                }));
            }
            return Ok(Ok(Resolved::Present(Box::new(first.evidence()?))));
        }
        if place.is_none()
            && let Some(receipt) = self.removed.get(name)
        {
            return Ok(Ok(Resolved::Removed(receipt.clone())));
        }
        Ok(Ok(Resolved::Missing))
    }
}

/// An ID more than one record holds: the refusal naming the qualified form, and each
/// place as `<place>/<id>`.
struct Ambiguous {
    refusal: String,
    places: Vec<String>,
}

/// One reference's label (REQ-cli-021), with the receipt that records a removed one
/// (REQ-records-009) and the places of an ambiguous one (REQ-cli-023).
struct ReferenceLabel {
    label: &'static str,
    receipt: Option<String>,
    places: Vec<String>,
}

/// `<place>/<id>` as (place, id); a bare ID has no place.
fn split_qualified(id: &str) -> (Option<&str>, &str) {
    match id.split_once('/') {
        Some((place, name)) => (Some(place), name),
        None => (None, id),
    }
}

/// One entry by ID or alias, resolved as [`Index::resolve`] resolves it; a removed or
/// missing ID is an error.
pub(super) fn load(root: &Path, id: &str) -> Result<Evidence> {
    match Index::build(root)?.resolve(id)? {
        Resolved::Present(evidence) => Ok(*evidence),
        Resolved::Removed(receipt) => {
            bail!("evidence {id} was removed by the 0.5.x migration, as {receipt} records")
        }
        Resolved::Missing => bail!("evidence not found: {id}"),
    }
}

/// Whether an entry or a receipt with this ID or alias exists.
pub(super) fn is_present(root: &Path, id: &str) -> Result<bool> {
    let (place, name) = split_qualified(id);
    if !valid_name(name) || place.is_some_and(|place| !valid_name(place)) {
        return Ok(false);
    }
    Ok(Index::build(root)?.holds(id))
}

/// The JSON view of an entry, as `status` and the case reports show it.
impl Evidence {
    /// What the entry is, and is not: an entry that holds the command Chough ran is an
    /// executed case run; any other is a manual entry.
    pub(super) fn provenance(&self) -> &'static str {
        if self.argv.is_empty() {
            PROVENANCE
        } else {
            EXECUTED_PROVENANCE
        }
    }

    pub(super) fn to_json(&self) -> Value {
        json!({
            "schema_version": 2,
            "id": self.id,
            "aliases": self.aliases,
            "criteria": self.criteria,
            "requirements": self.requirements,
            "contracts": self.contracts,
            "kind": self.kind.name(),
            "outcome": self.outcome.name(),
            "environment": self.environment,
            "summary": self.summary,
            "artifacts": self.artifacts,
            "artifact_fingerprints": self.artifact_fingerprints,
            "external": self.external,
            "screenshot": self.screenshot,
            "fingerprint": self.fingerprint,
            "raw_log": self.raw_log,
            "observed_at_unix_seconds": self.observed_at_unix_seconds,
            "revision": self.snapshot.revision,
            "content": self.snapshot.content,
            "input_inventory": self.snapshot.input_inventory,
            "cases": self.cases.iter().map(|case| json!({"id": case.id, "digest": case.digest})).collect::<Vec<_>>(),
            "method": self.method,
            "track": self.track,
            "argv": self.argv,
            "exit_code": self.exit_code,
            "tests_run": self.tests_run,
            "limitations": self.limitations,
            "duration_seconds": self.duration_seconds,
            "actor": self.actor.as_ref().and_then(|actor| match actor.front_value() {
                markdown::Value::Maps(maps) => maps.first().cloned(),
                _ => None,
            }).map(|map| map.into_iter().collect::<BTreeMap<_, _>>()),
            "provenance": self.provenance(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Snapshot {
    /// HEAD when observed: shown and used for regression ancestry, not for freshness
    /// when `content` is recorded.
    pub(super) revision: String,
    pub(super) dirty: bool,
    /// Legacy identity: the revision plus every listed file except `.chough/` and
    /// `target/`. Freshness compares it only for records without `content`.
    pub(super) fingerprint: String,
    /// Working content identity without the revision, Chough records, build output or
    /// SpecSync lifecycle bookkeeping (REQ-cli-004, REQ-cli-017).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) content: Option<String>,
    /// Retained diagnostic metadata only; aggregate identities remain authoritative.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) input_inventory: Option<InputInventory>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct InputInventory {
    version: u32,
    complete: bool,
    entries: Vec<InputIdentity>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct InputIdentity {
    domain: u8,
    key: String,
    path: String,
    digest: String,
}

/// #203's bounds on a diagnostic input inventory.
const INPUT_INVENTORY_ENTRIES: usize = 128;
const INPUT_INVENTORY_BYTES: usize = 32_768;

impl InputInventory {
    /// #203's rule (REQ-cli-004): version 1, at most 128 entries and 32,768 serialized
    /// bytes, and each entry a canonical, distinct path with a SHA-256 digest and the
    /// domain and key the content identity gives that path. Chough records, build
    /// output and SpecSync bookkeeping, which the identity skips, are refused.
    fn validate(&self, name: &str) -> Result<()> {
        ensure!(
            self.version == 1,
            "unsupported input inventory version: {name}"
        );
        ensure!(
            self.entries.len() <= INPUT_INVENTORY_ENTRIES
                && serde_json::to_vec(self)?.len() <= INPUT_INVENTORY_BYTES,
            "invalid input inventory bounds: {name}"
        );
        let mut paths = BTreeSet::new();
        for entry in &self.entries {
            ensure!(
                canonical_path(&entry.path).as_deref() == Some(entry.path.as_str()),
                "invalid input inventory entry: {name}: path must be canonical"
            );
            let expected = match fingerprint_key(&entry.path) {
                Key::Path(key) => Some((0, key)),
                Key::Change { key, .. } => Some((1, key)),
                Key::Skip => None,
            };
            ensure!(
                crate::records::is_digest(&entry.digest)
                    && paths.insert(entry.path.as_str())
                    && expected == Some((entry.domain, entry.key.clone())),
                "invalid input inventory entry: {name}"
            );
        }
        Ok(())
    }

    /// The entry's front matter form: `input_inventory_version`,
    /// `input_inventory_complete` and the `input_inventory` maps.
    fn to_record(&self, record: &mut Record) {
        record
            .set_scalar("input_inventory_version", &self.version.to_string())
            .expect("a scalar value");
        record
            .set_scalar("input_inventory_complete", &self.complete.to_string())
            .expect("a scalar value");
        record
            .set(
                "input_inventory",
                markdown::Value::Maps(
                    self.entries
                        .iter()
                        .map(|entry| {
                            vec![
                                ("domain".into(), entry.domain.to_string()),
                                ("key".into(), entry.key.clone()),
                                ("path".into(), entry.path.clone()),
                                ("digest".into(), entry.digest.clone()),
                            ]
                        })
                        .collect(),
                ),
            )
            .expect("a maps value");
    }

    /// The inventory an entry carries, validated; none without `input_inventory_version`.
    fn from_record(record: &Record) -> Result<Option<InputInventory>> {
        let Some(version) = record.scalar("input_inventory_version") else {
            return Ok(None);
        };
        let name = format!(
            "invalid evidence entry {}: input inventory",
            record.scalar("id").unwrap_or_default()
        );
        let invalid = || name.clone();
        let complete = match record.scalar("input_inventory_complete") {
            Some("true") => true,
            Some("false") => false,
            _ => bail!("{name}"),
        };
        let mut entries = vec![];
        for map in record.maps("input_inventory") {
            let fields: BTreeMap<String, String> = map.into_iter().collect();
            ensure!(fields.len() == 4, "{name}");
            let field = |key: &str| fields.get(key).cloned().with_context(invalid);
            entries.push(InputIdentity {
                domain: field("domain")?.parse().with_context(invalid)?,
                key: field("key")?,
                path: field("path")?,
                digest: field("digest")?,
            });
        }
        let inventory = InputInventory {
            version: version.parse().with_context(invalid)?,
            complete,
            entries,
        };
        inventory.validate(&name)?;
        Ok(Some(inventory))
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CaseRef {
    pub(super) id: String,
    pub(super) digest: String,
}

pub(super) fn snapshot(root: &Path) -> Result<Snapshot> {
    let phase = Instant::now();
    // A project may live in a subdirectory of its repository (REQ-cli-056, DRIVE-18):
    // the snapshot then covers the files under it. At the top level the prefix is empty
    // and every byte is as before.
    let top = git(root, &["rev-parse", "--show-toplevel"])?;
    let prefix = String::from_utf8(git(root, &["rev-parse", "--show-prefix"])?.stdout)?
        .trim_end_matches(['\n', '\r'])
        .to_string();
    ensure!(
        Path::new(std::str::from_utf8(&top.stdout)?.trim())
            .canonicalize()?
            .join(&prefix)
            .canonicalize()?
            == root,
        "{} is not where Git places this project; run chough from the project directory",
        root.display()
    );
    let revision = String::from_utf8(git(root, &["rev-parse", "HEAD"])?.stdout)?
        .trim()
        .to_string();
    let list = git(
        root,
        &[
            "ls-files",
            "--cached",
            "--others",
            "--exclude-standard",
            "-z",
        ],
    )?;
    crate::records::trace_phase("snapshot.git-enumeration", phase);
    let phase = Instant::now();
    // The generated knowledge directory (the librarian's index, notes and vault
    // attachments) is Chough's own generated output: rebuilding it must not make
    // the observations it lists stale (#136, REQ-cli-004). The legacy fingerprint
    // keeps its exact byte stream for older records, so the exclusion applies only
    // to the content identity and the dirty flag.
    let library_dir = crate::prefs::effective(root, "library-dir")
        .map_or_else(|_| "knowledge".to_string(), |(value, _)| value);
    let in_library =
        |name: &str| name == library_dir || name.starts_with(&format!("{library_dir}/"));
    let mut files = BTreeSet::new();
    for bytes in list
        .stdout
        .split(|byte| *byte == 0)
        .filter(|bytes| !bytes.is_empty())
    {
        let name =
            std::str::from_utf8(bytes).context("non-UTF-8 repository paths are unsupported")?;
        if !name.starts_with(".chough/") && !name.starts_with("target/") {
            files.insert(name);
        }
    }
    // (domain, key, path, change.md): change artifacts are a separate key domain, so no
    // ordinary path can collide with one.
    let mut entries = vec![];
    for name in &files {
        if in_library(name) {
            continue;
        }
        match fingerprint_key(name) {
            Key::Skip => {}
            Key::Path(key) => entries.push((0_u8, key, name.to_string(), false)),
            Key::Change { key, change_md } => {
                entries.push((1_u8, key, name.to_string(), change_md))
            }
        }
    }
    entries.sort();
    let (legacy, content) = std::thread::scope(|scope| -> Result<(String, String)> {
        let legacy = scope.spawn(|| -> Result<String> {
            // The legacy fingerprint keeps its exact byte stream for older records.
            let mut legacy = Sha256::new();
            legacy.update(revision.as_bytes());
            for name in files {
                if let Some(parent) = Path::new(&name)
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                {
                    local_path(root, parent.to_str().context("non-UTF-8 source parent")?)?;
                }
                legacy.update((name.len() as u64).to_le_bytes());
                legacy.update(name.as_bytes());
                if !hash_input(&mut legacy, root, name, None, false)? {
                    legacy.update(b"missing");
                }
            }
            Ok(format!("{:x}", legacy.finalize()))
        });
        let content = scope.spawn(|| -> Result<String> {
            let mut content = Sha256::new();
            for (domain, key, name, change_md) in &entries {
                // An absent tracked path contributes nothing, as after its deletion is committed.
                hash_input(&mut content, root, name, Some((*domain, key)), *change_md)?;
            }
            Ok(format!("{:x}", content.finalize()))
        });
        let legacy = legacy
            .join()
            .map_err(|_| anyhow::anyhow!("legacy fingerprint worker panicked"))??;
        let content = content
            .join()
            .map_err(|_| anyhow::anyhow!("content fingerprint worker panicked"))??;
        Ok((legacy, content))
    })?;
    crate::records::trace_phase("snapshot.hash", phase);
    let phase = Instant::now();
    let dirty_output = if prefix.is_empty() {
        git(root, &["status", "--porcelain", "--untracked-files=all"])?
    } else {
        git(
            root,
            &["status", "--porcelain", "--untracked-files=all", "--", "."],
        )?
    };
    // Porcelain paths are relative to the repository root; strip the project's prefix.
    let dirty = String::from_utf8(dirty_output.stdout)?.lines().any(|line| {
        line.get(3..)
            .and_then(|name| name.strip_prefix(prefix.as_str()))
            .is_some_and(|name| !in_library(name) && fingerprint_key(name) != Key::Skip)
    });
    crate::records::trace_phase("snapshot.git-status", phase);
    Ok(Snapshot {
        revision,
        dirty,
        fingerprint: legacy,
        content: Some(content),
        input_inventory: None,
    })
}

/// Whether `ancestor` is an ancestor of (or equal to) the current HEAD.
pub(super) fn is_ancestor_of_head(root: &Path, ancestor: &str) -> bool {
    crate::records::run_owned(
        Command::new("git")
            .args(["merge-base", "--is-ancestor", ancestor, "HEAD"])
            .current_dir(root),
        None,
    )
    .is_ok_and(|output| output.status.success())
}

/// Confirm each criterion is active in native HI export.
pub(super) fn validate_criteria(root: &Path, criteria: &[String]) -> Result<()> {
    let mut exports = BTreeMap::new();
    for criterion in criteria {
        let family = criterion
            .split_once('-')
            .context("criterion must be a HI ID")?
            .0;
        ensure!(valid_name(family), "invalid criterion family");
        if !exports.contains_key(family) {
            // The complete export, not the 16 KiB display section: a family's intent can
            // be any size, and a Chough bound must never decide this check (LIFE-15).
            let export = crate::context::hi_export_json(root, family);
            let value = export.value.with_context(|| {
                format!(
                    "cannot validate criteria: {} was {}: {}",
                    export.section.source, export.section.state, export.section.content
                )
            })?;
            exports.insert(family.to_string(), value);
        }
        ensure!(
            find_id(&exports[family], criterion),
            "unknown or retired criterion: {criterion}"
        );
    }
    Ok(())
}

/// A requirement link that none of the searched companions holds. Callers render it,
/// because the fix for a requirement of another module differs between a cases file
/// and `evidence add`.
pub(super) struct RequirementMiss {
    id: String,
    searched: Vec<String>,
    /// The other module whose companion holds the ID, when one does.
    owner: Option<String>,
}

impl RequirementMiss {
    fn render(&self, fix: impl Fn(&str) -> String) -> anyhow::Error {
        let found = format!(
            "requirement {} not found in {}",
            self.id,
            self.searched.join(", ")
        );
        match &self.owner {
            Some(module) => anyhow::anyhow!(
                "{found}; {} belongs to module {module} (specs/{module}/requirements.md): {}",
                self.id,
                fix(module)
            ),
            None => anyhow::anyhow!("{found}; {UNRESOLVED_HINT}"),
        }
    }

    /// The refusal for a case definition, which binds its file's one module.
    pub(super) fn for_cases(&self) -> anyhow::Error {
        self.render(|module| {
            format!(
                "a cases file binds one module's contract, so move the case to .chough/cases/{module}.cases.toml"
            )
        })
    }

    /// The refusal for `evidence add`, which names its contracts and, when the active
    /// session's native change was consulted, that change, its deltas searched and why
    /// none resolved the link (REQ-cli-005).
    fn for_evidence(&self, consulted: Option<(&str, &[(String, String)])>) -> anyhow::Error {
        let fix = |module: &str| format!("add --contract specs/{module}/{module}.spec.md");
        let Some((change, deltas)) = consulted else {
            return self.render(fix);
        };
        let found = format!(
            "requirement {} not found in {} or in {}",
            self.id,
            self.searched.join(", "),
            deltas
                .iter()
                .map(|(path, _)| path.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
        if let Some(module) = &self.owner {
            return anyhow::anyhow!(
                "{found}; {} belongs to module {module} (specs/{module}/requirements.md): {}",
                self.id,
                fix(module)
            );
        }
        let mut why: Vec<&str> = deltas.iter().map(|(_, reason)| reason.as_str()).collect();
        why.dedup();
        anyhow::anyhow!(
            "{found}; the active session's native change {change} was consulted and did not resolve it: {}; it may be a typo",
            why.join("; ")
        )
    }
}

/// Confirm each requirement ID appears in the contracts' native requirements companions.
/// The outer error is a contract that cannot be read; the inner one is the first link
/// that no searched companion holds, which is always a refusal.
pub(super) fn validate_requirements(
    root: &Path,
    contracts: &[String],
    requirements: &[String],
) -> Result<std::result::Result<(), RequirementMiss>> {
    let companions = contract_companions(root, contracts)?;
    find_requirements(root, &companions, requirements)
}

/// Each contract's `requirements.md` companion, named with `/` on every platform, as
/// contracts and messages spell paths; each contract must be a native spec that exists.
fn contract_companions(root: &Path, contracts: &[String]) -> Result<Vec<String>> {
    let mut companions = vec![];
    for contract in contracts {
        ensure!(
            contract.starts_with("specs/") && contract.ends_with(".spec.md"),
            "contract must be a native specs/*.spec.md path"
        );
        let path = local_path(root, contract)?;
        ensure!(path.is_file(), "missing contract: {contract}");
        let companion = Path::new(contract)
            .parent()
            .context("contract parent missing")?
            .join("requirements.md");
        let companion = companion.to_str().context("non-UTF-8 contract companion")?;
        #[cfg(windows)]
        companions.push(companion.replace('\\', "/"));
        #[cfg(not(windows))]
        companions.push(companion.to_string());
    }
    Ok(companions)
}

/// The first of `requirements` that none of `companions` holds, as a miss naming them.
fn find_requirements(
    root: &Path,
    companions: &[String],
    requirements: &[String],
) -> Result<std::result::Result<(), RequirementMiss>> {
    let mut text = String::new();
    for companion in companions {
        // On a module's first change, `specsync new` has scaffolded no requirements.md
        // and the approved delta adds the requirement: a companion that does not exist
        // yet holds no requirement, as an empty one would, so the link is a miss the
        // caller decides like any other (cli.md, Approved delta links). One that exists
        // but cannot be read is still refused.
        let path = local_path(root, companion)?;
        if path.exists() {
            text.push_str(
                &crate::records::read_spec_file(&path)
                    .context("contract requires its native requirements.md companion")?,
            );
            text.push('\n');
        }
    }
    for requirement in requirements {
        if !(requirement.starts_with("REQ-") && holds_token(&text, requirement)) {
            return Ok(Err(RequirementMiss {
                id: requirement.clone(),
                owner: owning_module(root, requirement, companions),
                searched: companions.to_vec(),
            }));
        }
    }
    Ok(Ok(()))
}

/// `evidence add`'s requirement and contract links (REQ-cli-005): each contract given
/// must be a native spec that exists, and each requirement must be held by a searched
/// companion, the given contracts' or, with none given, its own module's, or be added or
/// modified by the active session's native change in that module's approved delta, as a
/// case link is (REQ-cli-009), so evidence can be recorded before `specsync change
/// check` materializes it.
fn validate_evidence_links(
    root: &Path,
    contracts: &[String],
    requirements: &[String],
) -> Result<()> {
    let given = contract_companions(root, contracts)?;
    let deltas = cases::DeltaLinks::new(root);
    'requirements: for requirement in requirements {
        let companions = if contracts.is_empty() {
            let module = requirement
                .strip_prefix("REQ-")
                .and_then(|rest| rest.rsplit_once('-'))
                .map(|(module, _)| module)
                .filter(|module| crate::records::valid_module_name(module))
                .with_context(|| {
                    format!(
                        "requirement {requirement} is not a REQ-<module>-<number> ID, so no module's requirements.md can hold it"
                    )
                })?;
            vec![format!("specs/{module}/requirements.md")]
        } else {
            given.clone()
        };
        let Err(miss) = find_requirements(root, &companions, std::slice::from_ref(requirement))?
        else {
            continue;
        };
        let mut change = None;
        let mut searched = vec![];
        for companion in &companions {
            let module = companion
                .strip_prefix("specs/")
                .and_then(|rest| rest.strip_suffix("/requirements.md"))
                .unwrap_or(companion);
            match deltas.in_delta(module, requirement)? {
                cases::InDelta::NoChange => return Err(miss.for_evidence(None)),
                cases::InDelta::Held(_) => continue 'requirements,
                cases::InDelta::Not(consulted, reason) => {
                    searched.push((
                        format!(".specsync/changes/{consulted}/deltas/{module}.md"),
                        reason,
                    ));
                    change = Some(consulted);
                }
            }
        }
        return Err(miss.for_evidence(
            change
                .as_deref()
                .map(|change| (change, searched.as_slice())),
        ));
    }
    Ok(())
}

fn git(root: &Path, arguments: &[&str]) -> Result<Output> {
    let output =
        crate::records::run_owned(Command::new("git").args(arguments).current_dir(root), None)
            .context("git is required for evidence")?;
    ensure!(
        output.status.success(),
        "git {} failed: {}",
        arguments.join(" "),
        crate::records::bounded(&output.stderr)
    );
    Ok(output)
}

/// How the content identity treats one listed path.
#[derive(Debug, PartialEq)]
enum Key {
    /// Chough records, build output and SpecSync lifecycle bookkeeping.
    Skip,
    /// An ordinary file, keyed by its path.
    Path(String),
    /// A native change artifact, keyed by change ID and its path inside the change, so
    /// moving the change into the archive keeps its key. `change.md` has the value of
    /// its front-matter `state:` line blanked.
    Change { key: String, change_md: bool },
}

fn fingerprint_key(name: &str) -> Key {
    if name.starts_with(".chough/") || name.starts_with("target/") {
        return Key::Skip;
    }
    let change = name
        .strip_prefix(".specsync/changes/")
        .and_then(|rest| rest.split_once('/'))
        .or_else(|| {
            let (directory, rest) = name
                .strip_prefix(".specsync/archive/changes/")?
                .split_once('/')?;
            Some((archived_id(directory)?, rest))
        })
        .filter(|(id, rest)| !id.is_empty() && !rest.is_empty());
    match change {
        Some((_, rest)) if BOOKKEEPING.contains(&rest) => Key::Skip,
        Some((id, rest)) => Key::Change {
            key: format!("specsync-change/{id}/{rest}"),
            change_md: rest == "change.md",
        },
        None => Key::Path(name.to_string()),
    }
}

/// Hash `label` (a key domain and key) and one listed file's kind, mode bits, length and
/// bytes. Returns false, hashing nothing, when the path is absent from the working tree.
fn hash_input(
    hash: &mut Sha256,
    root: &Path,
    name: &str,
    label: Option<(u8, &str)>,
    change_md: bool,
) -> Result<bool> {
    let path = root.join(name);
    let metadata = match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        result => result?,
    };
    if let Some((domain, label)) = label {
        hash.update([domain]);
        hash.update((label.len() as u64).to_le_bytes());
        hash.update(label.as_bytes());
    }
    if metadata.file_type().is_symlink() {
        hash.update(b"symlink");
        hash.update(fs::read_link(&path)?.as_os_str().as_encoded_bytes());
        return Ok(true);
    }
    ensure!(metadata.is_file(), "unsupported Git input: {name}");
    hash.update(b"file");
    hash.update([u8::from(metadata.permissions().readonly())]);
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        hash.update((metadata.permissions().mode() & 0o111).to_le_bytes());
    }
    if change_md {
        let bytes = fs::read(&path)?;
        let bytes = match String::from_utf8(bytes) {
            Ok(text) => blank_front_matter_state(&text).into_bytes(),
            Err(error) => error.into_bytes(),
        };
        hash.update((bytes.len() as u64).to_le_bytes());
        hash.update(&bytes);
        return Ok(true);
    }
    hash.update(metadata.len().to_le_bytes());
    let mut file = File::open(&path)?;
    let mut buffer = [0_u8; 8192];
    loop {
        let size = file.read(&mut buffer)?;
        if size == 0 {
            break;
        }
        hash.update(&buffer[..size]);
    }
    Ok(true)
}

/// A relative path rebuilt from its components and joined with `/`, so `a//b`, `a/b/`
/// and (on Windows) `a\b` share one spelling. None for empty or traversing paths.
fn canonical_path(path: &str) -> Option<String> {
    let mut parts = vec![];
    for component in Path::new(path).components() {
        match component {
            Component::Normal(part) => parts.push(part.to_str()?),
            _ => return None,
        }
    }
    (!parts.is_empty()).then(|| parts.join("/"))
}

// ---------------------------------------------------------------------------
// Freshness (REQ-cli-004).

/// Observations with freshness, sharing this request's case definitions.
pub(super) struct Reads<'a> {
    root: &'a Path,
    cases: cases::Digests<'a>,
    artifact_time: std::cell::Cell<Duration>,
    /// The evidence an ID can name, read once per request (REQ-cli-021).
    index: std::cell::OnceCell<Index>,
}

impl<'a> Reads<'a> {
    pub(super) fn new(root: &'a Path) -> Self {
        Self {
            root,
            cases: cases::Digests::new(root),
            artifact_time: std::cell::Cell::new(Duration::ZERO),
            index: std::cell::OnceCell::new(),
        }
    }

    /// This request's evidence index, read on first use.
    fn index(&self) -> Result<&Index> {
        if let Some(index) = self.index.get() {
            return Ok(index);
        }
        let built = Index::build(self.root)?;
        Ok(self.index.get_or_init(|| built))
    }

    /// Every open session's entry, read from this request's index, in folder and name
    /// order; one that does not read as an entry refuses, as [`load_all`] does.
    fn entries(&self) -> Result<Vec<Evidence>> {
        self.index()?
            .first
            .iter()
            .filter(|holder| matches!(holder.held, Held::Entry(_)))
            .map(Holder::evidence)
            .collect()
    }

    /// How many open sessions' entries no ID in `referenced` names, by ID or alias.
    pub(super) fn unreferenced_entries(&self, referenced: &BTreeSet<&str>) -> Result<usize> {
        Ok(self
            .index()?
            .first
            .iter()
            .filter(|holder| matches!(holder.held, Held::Entry(_)))
            .filter(|holder| {
                !holder
                    .names
                    .iter()
                    .any(|name| referenced.contains(name.as_str()))
            })
            .count())
    }

    /// (all current, an external artifact pending) for one entry.
    fn artifacts_current(&self, evidence: &Evidence) -> Result<(bool, bool)> {
        let started = Instant::now();
        let result = (|| {
            let mut current = true;
            let mut external_pending = false;
            for artifact in &evidence.artifacts {
                local_path(self.root, artifact)?;
                let expected = evidence
                    .artifact_fingerprints
                    .get(artifact)
                    .context("missing artifact binding")?;
                ensure!(
                    crate::records::is_digest(expected),
                    "invalid artifact digest"
                );
                if crate::records::file_digest(self.root, artifact)
                    .as_ref()
                    .ok()
                    != Some(expected)
                {
                    current = false;
                }
            }
            if let Some(external) = &evidence.external {
                match crate::local::external(self.root, external)? {
                    Some(record) => {
                        let actual = fs::read(&record.path).ok().map(|bytes| {
                            format!("sha256:{}", crate::records::digest_bytes(&bytes))
                        });
                        if actual.as_ref() != Some(&record.fingerprint) {
                            current = false;
                        }
                    }
                    None => external_pending = true,
                }
            }
            Ok((current, external_pending))
        })();
        self.artifact_time
            .set(self.artifact_time.get() + started.elapsed());
        result
    }

    fn freshness(
        &self,
        evidence: &Evidence,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<&'static str> {
        let (artifacts_current, _) = self.artifacts_current(evidence)?;
        let mut cases_current = true;
        for case in &evidence.cases {
            if self.cases.current(&case.id)?.as_deref() != Some(case.digest.as_str()) {
                cases_current = false;
            }
        }
        Ok(match current {
            Err(_) => "unknown-revision",
            Ok(snapshot) if !same_inputs(snapshot, evidence) => "stale-revision-or-inputs",
            Ok(_) if !artifacts_current => "stale-artifact",
            Ok(_) if !cases_current => "stale-case-definition",
            Ok(_) if environment.is_none() => "environment-unchecked",
            Ok(_) if environment != Some(evidence.environment.as_str()) => "stale-environment",
            Ok(_) => "current-observation",
        })
    }

    /// Observations with freshness, sharing this request's case definitions.
    pub(super) fn observations(
        &self,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<Vec<Value>> {
        let mut values = vec![];
        for evidence in self.entries()? {
            let state = self.freshness(&evidence, current, environment)?;
            let (artifacts_current, external_pending) = self.artifacts_current(&evidence)?;
            let mut view = json!({"record": evidence.to_json(), "freshness": state, "label": label_for(state, environment)});
            if !artifacts_current {
                view["artifact"] = json!("changed");
            } else if external_pending {
                view["artifact"] = json!("external, not checkable here");
            }
            values.push(view);
        }
        Ok(values)
    }
}

impl Drop for Reads<'_> {
    fn drop(&mut self) {
        crate::records::trace(&format!(
            "phase freshness.artifacts elapsed_ms={}",
            self.artifact_time.get().as_millis()
        ));
    }
}

/// The same inputs as an entry was recorded with: its content identity when it has
/// one, else its revision against HEAD with the tree clean (revision-bound,
/// TC-cli-0073).
fn same_inputs(current: &Snapshot, evidence: &Evidence) -> bool {
    match (&current.content, &evidence.snapshot.content) {
        (Some(now), Some(then)) => now == then,
        _ => {
            !current.dirty
                && !evidence.snapshot.revision.is_empty()
                && current.revision == evidence.snapshot.revision
        }
    }
}

/// A label for one freshness state and environment match (LIFE-7).
fn label_for(state: &str, environment: Option<&str>) -> &'static str {
    match (state, environment) {
        ("current-observation", Some(_)) => "current",
        ("current-observation", None) => "current (environment unverified)",
        ("environment-unchecked", _) => "current (environment unverified)",
        ("unknown-revision", _) => "unknown",
        ("stale-revision-or-inputs", _) => "stale",
        ("stale-artifact", _) => "stale",
        ("stale-case-definition", _) => "stale-case-definition",
        ("stale-environment", _) => "stale-environment",
        _ => "unknown",
    }
}

/// Labels observation references current, historical, removed or unavailable.
pub(super) struct Labels<'a> {
    reads: Reads<'a>,
    snapshot: Result<Snapshot>,
}

impl<'a> Labels<'a> {
    pub(super) fn new(root: &'a Path) -> Self {
        Self {
            reads: Reads::new(root),
            snapshot: snapshot(root),
        }
    }

    /// `environment` None accepts the record's own environment.
    pub(super) fn label(
        &self,
        root: &Path,
        id: &str,
        environment: Option<&str>,
    ) -> Result<&'static str> {
        ensure!(
            root == self.reads.root,
            "observation labels belong to another project root"
        );
        Ok(self
            .reads
            .label_reference(id, &self.snapshot, environment)?
            .label)
    }
}

/// A referenced observation's label (REQ-cli-021): current when present with matching
/// inputs, its environment unchecked when no environment is given; otherwise
/// historical, present but stale. Removed and unavailable come from the index.
fn reference_label(state: &str, environment: Option<&str>) -> &'static str {
    match (state, environment) {
        ("current-observation", _) => "current",
        ("environment-unchecked", None) => "current",
        _ => "historical",
    }
}

impl Reads<'_> {
    /// One reference's label: current, historical, removed (naming the receipt),
    /// unavailable, or ambiguous (naming its places), never a refusal of the report.
    fn label_reference(
        &self,
        id: &str,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<ReferenceLabel> {
        let label = |label, receipt, places| ReferenceLabel {
            label,
            receipt,
            places,
        };
        Ok(match self.index()?.lookup(id)? {
            Ok(Resolved::Present(evidence)) => label(
                reference_label(
                    self.freshness(&evidence, current, environment)?,
                    environment,
                ),
                None,
                vec![],
            ),
            Ok(Resolved::Removed(receipt)) => label("removed", Some(receipt), vec![]),
            Ok(Resolved::Missing) => label("unavailable", None, vec![]),
            Err(ambiguous) => label("ambiguous", None, ambiguous.places),
        })
    }

    /// Every observation an open session or a bug references, labelled, one row per ID.
    /// When the bare ID is ambiguous because two sessions hold same-named entries
    /// (REQ-cli-023), a session's own reference resolves in that session, as
    /// `<session>/<id>`, and the ID is one row per group of sources resolving alike,
    /// each naming the form it resolved through as `resolved`. A reference that stays
    /// ambiguous, such as a bare link a bug held before the merge, reads `ambiguous`,
    /// naming its places.
    pub(super) fn reference_labels(
        &self,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<Vec<Value>> {
        let index = self.index()?;
        let mut values = vec![];
        for (id, sources) in references_from(self.root)? {
            let mut groups: Vec<(String, Vec<String>)> = vec![];
            if index.lookup(&id)?.is_ok() {
                groups.push((id.clone(), sources));
            } else {
                for source in sources {
                    let own = source
                        .strip_prefix(&format!("{}/", crate::sessions::DIR))
                        .and_then(|rest| rest.strip_suffix("/session.md"))
                        .map(|session| format!("{session}/{id}"))
                        .filter(|qualified| index.holds(qualified));
                    let key = own.unwrap_or_else(|| id.clone());
                    match groups.iter_mut().find(|(known, _)| *known == key) {
                        Some((_, by)) => by.push(source),
                        None => groups.push((key, vec![source])),
                    }
                }
            }
            for (key, by) in groups {
                let found = self.label_reference(&key, current, environment)?;
                let mut value = json!({"id": id, "label": found.label, "referenced_by": by});
                if key != id {
                    value["resolved"] = json!(key);
                }
                if let Some(receipt) = found.receipt {
                    value["receipt"] = json!(receipt);
                }
                if !found.places.is_empty() {
                    value["places"] = json!(found.places);
                }
                values.push(value);
            }
        }
        Ok(values)
    }
}

/// Every observation an open session's front matter or a bug's record references.
fn references_from(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for session_id in crate::sessions::open_ids(root)? {
        let file = crate::sessions::session_file(&session_id);
        // An unreadable session never stops another session's read (DRIVE-14): skip it
        // here; `session list` still reports it.
        let Ok(text) = read_bounded_record(&local_path(root, &file)?, markdown::SESSION_BOUND)
        else {
            continue;
        };
        let Ok(record) = markdown::parse(&file, &text) else {
            continue;
        };
        for id in record.list("observations") {
            found.entry(id).or_default().push(file.clone());
        }
    }
    let bugs = local_path(root, crate::bugs::BUGS_DIR)?;
    if bugs.is_dir() {
        for entry in fs::read_dir(&bugs)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if !name.ends_with(".json") && !name.ends_with(".md") {
                continue;
            }
            let text = read_bounded_record(&entry.path(), markdown::RECORD_BOUND)?;
            let ids: Vec<String> = if name.ends_with(".json") {
                serde_json::from_str::<Value>(&text)?
                    .get("observations")
                    .and_then(Value::as_array)
                    .map(|all| {
                        all.iter()
                            .filter_map(Value::as_str)
                            .map(str::to_string)
                            .collect()
                    })
                    .unwrap_or_default()
            } else {
                markdown::parse(&name, &text)?.list("evidence")
            };
            for id in ids {
                found
                    .entry(id)
                    .or_default()
                    .push(format!("{}/{}", crate::bugs::BUGS_DIR, name));
            }
        }
    }
    for sources in found.values_mut() {
        sources.sort();
    }
    Ok(found)
}

// ---------------------------------------------------------------------------
// The seam for the archived story (plan step 4, wired by the step 3 owner):
// a session's entries as `receipts:` values.

/// The evidence entries of one session as receipt values for its archived story, in
/// the order and shape the migration writes them (REQ-history-006, REQ-cli-017):
/// id; aliases as one scalar, comma-and-space separated; claim; result; command, the
/// executed argument vector joined by spaces; revision; environment; content;
/// fingerprint; raw_log; case and case_digest; screenshot; observed; duration_seconds;
/// and the actor. Each field is written only when known. Front-matter-native maps.
pub(super) fn receipts_of(root: &Path, session_id: &str) -> Result<Vec<Vec<(String, String)>>> {
    let mut receipts = vec![];
    // REQ-history-006 (t38j): receipts in observation-time order, oldest first, then by
    // ID, whatever order the entries' names sort in.
    let mut entries = load_session(root, session_id)?;
    entries.sort_by(|left, right| {
        left.observed_at_unix_seconds
            .cmp(&right.observed_at_unix_seconds)
            .then_with(|| left.id.cmp(&right.id))
    });
    for evidence in entries {
        let mut map: Vec<(String, String)> = vec![("id".into(), evidence.id.clone())];
        if !evidence.aliases.is_empty() {
            map.push(("aliases".into(), evidence.aliases.join(", ")));
        }
        map.push(("claim".into(), evidence.summary.clone()));
        map.push(("result".into(), evidence.outcome.name().to_string()));
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
        if let Some(raw_log) = &evidence.raw_log {
            map.push(("raw_log".into(), raw_log.clone()));
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
            iso_time(evidence.observed_at_unix_seconds),
        ));
        if let Some(duration) = evidence.duration_seconds {
            map.push(("duration_seconds".into(), duration.to_string()));
        }
        if let Some(actor) = &evidence.actor {
            map.push(("actor".into(), actor.display()));
        }
        receipts.push(map);
    }
    Ok(receipts)
}

/// Move a finishing session's screenshots to the folder beside its story,
/// `.chough/archive/sessions/<id>/`, created only when it has one (REQ-records-013).
/// Only bytes committed at HEAD move (REQ-records-012); an uncommitted screenshot
/// stays and is named in the returned list. Call with the story already written.
pub(super) fn move_screenshots_at_finish(root: &Path, session_id: &str) -> Result<Vec<String>> {
    let from = local_path(root, &evidence_dir(session_id))?;
    let mut kept = vec![];
    let Ok(entries) = fs::read_dir(&from) else {
        return Ok(kept);
    };
    let target_dir = format!(".chough/archive/sessions/{session_id}");
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.ends_with(".png") && !name.ends_with(".jpg") {
            continue;
        }
        let bytes = fs::read(entry.path())?;
        let target = format!("{target_dir}/{name}");
        let committed = crate::records::write_new_bytes(root, &target, &bytes)?;
        ensure!(committed, "{target} already exists with other bytes");
        let relative = format!("{}/{name}", evidence_dir(session_id));
        if committed_at_head(root, &relative) {
            fs::remove_file(entry.path())?;
        } else {
            kept.push(relative);
        }
    }
    Ok(kept)
}

/// One archived change as the removed post-archive summary saw it.
#[allow(dead_code)] // fields read again when step 8 lands the vault rewiring
pub(super) struct ListedArchive {
    pub(super) id: String,
    pub(super) hot: bool,
    pub(super) paths: Vec<String>,
    /// The archive's finalization timestamp (library.rs builds these; unused by the
    /// removed summary, kept so its construction compiles until step 8 lands).
    pub(super) finalized: u64,
}

/// Top-level files SpecSync 6.1.1 writes into a change directory to record lifecycle
/// state; `lesson-bundle.md` is generated by finalize from records already hashed.
const BOOKKEEPING: [&str; 9] = [
    "state.json",
    "approvals.json",
    "verification.json",
    "verification-attempts.json",
    "review.json",
    "review-attempts.json",
    "finalization.json",
    "accepted-state.json",
    "lesson-bundle.md",
];

/// `<id>` of an archive directory named `<YYYY-MM-DD>-<id>`.
fn archived_id(directory: &str) -> Option<&str> {
    let (date, id) = (
        directory.get(..10)?,
        directory.get(10..)?.strip_prefix('-')?,
    );
    let dated = date.bytes().enumerate().all(|(index, byte)| {
        if index == 4 || index == 7 {
            byte == b'-'
        } else {
            byte.is_ascii_digit()
        }
    });
    (dated && !id.is_empty()).then_some(id)
}

const UNRESOLVED_HINT: &str = "it may be a typo, or a requirement an approved native change adds that is not yet materialized: run specsync change check <change> before running or recording cases that link it";

fn find_id(value: &Value, id: &str) -> bool {
    match value {
        Value::Object(object) => {
            object.get("id").and_then(Value::as_str) == Some(id)
                || object
                    .iter()
                    .filter(|(key, _)| key.as_str() != "retired")
                    .any(|(_, value)| find_id(value, id))
        }
        Value::Array(values) => values.iter().any(|value| find_id(value, id)),
        _ => false,
    }
}

/// Whether `text` holds `id` as a whole token of ID characters. A dot belongs to the token
/// so a dotted module's `REQ-chat.service-1` is one token, except a sentence-ending dot.
fn holds_token(text: &str, id: &str) -> bool {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '.'))
        .any(|part| part == id || part.trim_end_matches('.') == id)
}

/// The module another companion would hold `id` for: `REQ-<module>-<part>`, a valid
/// module name whose `requirements.md` (read at any size) is not among those searched and holds
/// the ID as a token. Reads neither the checkpoint nor native deltas.
fn owning_module(root: &Path, id: &str, searched: &[String]) -> Option<String> {
    let (module, _) = id.strip_prefix("REQ-")?.rsplit_once('-')?;
    let companion = format!("specs/{module}/requirements.md");
    if !crate::records::valid_module_name(module) || searched.contains(&companion) {
        return None;
    }
    let path = local_path(root, &companion).ok()?;
    let text = crate::records::read_spec_file(&path).ok()?;
    holds_token(&text, id).then(|| module.to_string())
}

/// `change.md` with the value of the `state:` line in its leading `---` front matter
/// blanked. A body line, or text without closed front matter, is kept as written.
fn blank_front_matter_state(text: &str) -> String {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let bare = |line: &str| line.trim_end_matches(['\r', '\n']).to_string();
    if lines.first().map(|line| bare(line)) != Some("---".into()) {
        return text.to_string();
    }
    let Some(close) = lines.iter().skip(1).position(|line| bare(line) == "---") else {
        return text.to_string();
    };
    let mut out = String::with_capacity(text.len());
    for (index, line) in lines.iter().enumerate() {
        let trimmed = bare(line);
        if index >= 1 && index <= close && trimmed.starts_with("state:") {
            out.push_str("state:");
            out.push_str(&line[trimmed.len()..]);
        } else {
            out.push_str(line);
        }
    }
    out
}

/// Read a 0.5.x JSON observation into the entry model (the retired protection walk in
/// history.rs reads them, and the migration of step 10 will).
impl<'de> Deserialize<'de> for Evidence {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Evidence, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[allow(dead_code)] // schema_version/provenance are read only to parse a 0.5.x record whole
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Legacy {
            schema_version: u32,
            id: String,
            criteria: Vec<String>,
            requirements: Vec<String>,
            contracts: Vec<String>,
            kind: Kind,
            outcome: Outcome,
            environment: String,
            summary: String,
            artifacts: Vec<String>,
            artifact_fingerprints: BTreeMap<String, String>,
            observed_at_unix_seconds: u64,
            snapshot: LegacySnapshot,
            provenance: String,
            #[serde(default)]
            cases: Vec<CaseRef>,
            method: Option<String>,
            track: Option<String>,
            #[serde(default)]
            argv: Vec<String>,
            exit_code: Option<i32>,
            tests_run: Option<u64>,
            #[serde(default)]
            limitations: Vec<String>,
            duration_seconds: Option<u64>,
        }
        let legacy = Legacy::deserialize(deserializer)?;
        Ok(Evidence {
            id: legacy.id,
            explicit_id: false,
            aliases: vec![],
            criteria: legacy.criteria,
            requirements: legacy.requirements,
            contracts: legacy.contracts,
            kind: legacy.kind,
            outcome: legacy.outcome,
            environment: legacy.environment,
            summary: legacy.summary,
            artifacts: legacy.artifacts,
            artifact_fingerprints: legacy.artifact_fingerprints,
            external: None,
            screenshot: None,
            fingerprint: None,
            raw_log: None,
            observed_at_unix_seconds: legacy.observed_at_unix_seconds,
            snapshot: Snapshot {
                revision: legacy.snapshot.revision,
                dirty: legacy.snapshot.dirty,
                fingerprint: legacy.snapshot.fingerprint,
                content: legacy.snapshot.content,
                input_inventory: legacy.snapshot.input_inventory,
            },
            cases: legacy.cases,
            method: legacy.method,
            track: legacy.track,
            argv: legacy.argv,
            exit_code: legacy.exit_code,
            tests_run: legacy.tests_run,
            limitations: legacy.limitations,
            key_lines: vec![],
            redacted: 0,
            actor: None,
            duration_seconds: legacy.duration_seconds,
        })
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct LegacySnapshot {
    revision: String,
    dirty: bool,
    fingerprint: String,
    content: Option<String>,
    /// #203's diagnostic inventory: absent, null or valid, never authoritative.
    #[serde(default)]
    input_inventory: Option<InputInventory>,
}

/// Whether the file's bytes are committed at HEAD (as Git stores them), the
/// REQ-records-012 check; the same rule sessions.rs's remove_committed uses.
fn committed_at_head(root: &Path, relative: &str) -> bool {
    let path = match local_path(root, relative) {
        Ok(path) => path,
        Err(_) => return false,
    };
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(_) => return false,
    };
    // The blob as Git stores it at HEAD, compared byte for byte.
    let show = std::process::Command::new("git")
        .args(["show", &format!("HEAD:{relative}")])
        .current_dir(root)
        .output();
    match show {
        Ok(output) => output.status.success() && output.stdout == bytes,
        Err(_) => false,
    }
}

// ---------------------------------------------------------------------------
// One reader over every entry and receipt (plan delta correction 4; timing and
// bugs both call it).

/// One observation record, wherever it lives, as the gates read it: id, aliases,
/// case, result and observed (Unix seconds).
#[derive(Clone, Debug)]
pub(super) struct ObservationView {
    pub(super) id: String,
    pub(super) aliases: Vec<String>,
    pub(super) case: Option<String>,
    pub(super) result: String,
    pub(super) observed: Option<u64>,
}

/// A receipt map (a story's or a bug's `receipts:` entry) as a view.
fn receipt_view(map: &[(String, String)]) -> ObservationView {
    let get = |key: &str| {
        map.iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
    };
    ObservationView {
        id: get("id").unwrap_or_default(),
        aliases: get("aliases")
            .map(|text| text.split(", ").map(str::to_string).collect())
            .unwrap_or_default(),
        case: get("case"),
        result: get("result").unwrap_or_default(),
        observed: get("observed").and_then(|text| crate::session_md::parse_time(&text).ok()),
    }
}

/// An open session's entry as a view.
fn entry_view(evidence: &Evidence) -> ObservationView {
    ObservationView {
        id: evidence.id.clone(),
        aliases: evidence.aliases.clone(),
        case: evidence.cases.first().map(|case| case.id.clone()),
        result: evidence.outcome.name().to_string(),
        observed: Some(evidence.observed_at_unix_seconds),
    }
}

/// Every observation record: each open session's entries, then each archived story's
/// and each bug's receipts. Read-only.
pub(super) fn every_observation(root: &Path) -> Result<Vec<ObservationView>> {
    let mut views = vec![];
    for session_id in crate::sessions::open_ids(root)? {
        for evidence in load_session(root, &session_id)? {
            views.push(entry_view(&evidence));
        }
    }
    let archive = local_path(root, ".chough/archive/sessions")?;
    if archive.is_dir() {
        let mut names: Vec<String> = fs::read_dir(&archive)?
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| name.ends_with(".md"))
            .collect();
        names.sort();
        for name in names {
            let text = read_bounded_record(&archive.join(&name), markdown::SESSION_BOUND)?;
            let record = markdown::parse(&name, &text)?;
            for map in record.story_receipts(&name)? {
                if tombstone_of(&map).is_none() {
                    views.push(receipt_view(&map));
                }
            }
        }
    }
    let bugs = local_path(root, crate::bugs::BUGS_DIR)?;
    if bugs.is_dir() {
        for entry in fs::read_dir(&bugs)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            if name.ends_with(".md") {
                let text = read_bounded_record(&entry.path(), markdown::RECORD_BOUND)?;
                let record = markdown::parse(&name, &text)?;
                for map in record.maps("receipts") {
                    if tombstone_of(&map).is_none() {
                        views.push(receipt_view(&map));
                    }
                }
            }
        }
    }
    Ok(views)
}

/// Resolve an evidence ID (a bare ID, a 0.5.x alias or the qualified
/// `<session-id>/<evidence-id>` form) through the open sessions; two records holding
/// it refuse as ambiguous, naming the qualified form (the t38b/t382 agreement).
pub(super) fn resolve_entry(root: &Path, id: &str) -> Result<Option<(String, Evidence)>> {
    if let Some((session, entry)) = id.split_once('/') {
        let record = load_session(root, session)?
            .into_iter()
            .find(|evidence| evidence.id == entry || evidence.aliases.iter().any(|a| a == entry));
        return Ok(record.map(|evidence| (id.to_string(), evidence)));
    }
    let mut found = vec![];
    for session_id in crate::sessions::open_ids(root)? {
        let dir = evidence_dir(&session_id);
        if let Some((_, record)) = markdown::resolve(root, &dir, id)? {
            found.push((session_id, Evidence::from_record(&record)?));
        }
    }
    let Some(first) = found.pop() else {
        return Ok(None);
    };
    ensure!(
        found.is_empty(),
        "refused: {id} names an evidence entry in more than one session; qualify it as <session-id>/<evidence-id>"
    );
    Ok(Some((format!("{}/{}", first.0, first.1.id), first.1)))
}

/// The files written beside an entry: an optional screenshot (extension, bytes) and
/// the raw log of an executed case (bytes, written machine-local, never committed).
#[derive(Default)]
pub(super) struct EntryFiles {
    pub(super) screenshot: Option<(String, Vec<u8>)>,
    pub(super) log: Option<Vec<u8>>,
    /// An external artifact's local record, written after the entry.
    pub(super) external: Option<ExternalBinding>,
}

// ---------------------------------------------------------------------------
// The seams of the 0.5.x evidence mapper (src/evidence_migrate.rs, plan step 10), wired
// for t382: thin wrappers over this module's reader, entry writer and screenshot
// stripping, for t37d to keep or replace in place.

/// Read a 0.5.x JSON observation into the entry model, naming the file on refusal
/// (REQ-records-009). A `snapshot.input_inventory` is read with #203's rule: absent or
/// null is none, a valid one is carried as diagnostic metadata that never changes
/// freshness, and an invalid one refuses (REQ-cli-004).
pub(super) fn legacy_observation(path: &str, text: &str) -> Result<Evidence> {
    let evidence: Evidence =
        serde_json::from_str(text).with_context(|| format!("{path}: not a 0.5.x observation"))?;
    if let Some(inventory) = &evidence.snapshot.input_inventory {
        inventory.validate(path)?;
    }
    Ok(evidence)
}

/// An entry's record text, named by its ID, within the entry bound; the migration's
/// framework redacts it (REQ-records-006).
pub(super) fn entry_text(evidence: &Evidence) -> Result<String> {
    evidence
        .to_record(&evidence.id)
        .render_within(markdown::Kind::Evidence)
}

/// A key screenshot's committed bytes: a PNG (`png`) or JPEG (`jpg`) without its
/// metadata (REQ-records-013).
pub(super) fn strip_screenshot(bytes: &[u8], extension: &str) -> Result<Vec<u8>> {
    match extension {
        "png" => strip_png(bytes),
        _ => strip_jpeg(bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A receipt map's lines under `receipts:`, as a story or a bug writes them.
    fn receipt(id: &str, aliases: &str, claim: &str) -> String {
        let mut text = format!("  - id: {id}\n");
        if !aliases.is_empty() {
            text.push_str(&format!("    aliases: {aliases}\n"));
        }
        text.push_str(&format!(
            "    claim: {claim}\n    result: passed\n    revision: {}\n    environment: local\n    observed: 2026-01-01T00:00:00Z\n",
            "1".repeat(40)
        ));
        text
    }

    fn write(root: &Path, name: &str, text: &str) {
        let path = root.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    }

    fn holder(root: &Path, name: &str, receipts: &[String]) {
        write(
            root,
            name,
            &format!("---\nreceipts:\n{}---\n\n# A record\n", receipts.concat()),
        );
    }

    const MIGRATION: &str = ".chough/archive/migrations/2026-01-05-from-0-5.md";

    /// Two stories and two bugs holding receipts, a tombstone and a migration receipt.
    fn project() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        let root_path = root.path();
        let tombstone = format!("  - id: OBS-tombstoned\n    removed: {MIGRATION}\n");
        holder(
            root_path,
            ".chough/archive/sessions/2026-01-01-story-a.md",
            &[receipt(
                "2026-01-01-x",
                "OBS-shared",
                "SYNTHETIC in story a",
            )],
        );
        holder(
            root_path,
            ".chough/archive/sessions/2026-01-02-story-b.md",
            &[receipt("2026-01-01-x", "", "SYNTHETIC in story b")],
        );
        holder(
            root_path,
            ".chough/bugs/2026-01-01-bug-a.md",
            &[
                receipt("2026-01-03-y", "", "SYNTHETIC in bug a"),
                receipt("2026-01-04-z", "", "SYNTHETIC copy"),
                tombstone,
            ],
        );
        holder(
            root_path,
            ".chough/bugs/2026-01-02-bug-b.md",
            &[
                receipt("2026-01-03-y", "", "SYNTHETIC in bug b"),
                receipt("2026-01-04-z", "", "SYNTHETIC copy"),
            ],
        );
        write(
            root_path,
            MIGRATION,
            "---\nkind: receipt\n---\n\n# Migration from Chough 0.5.x\n\n## Aliases\n\n- `OBS-carried` → `.chough/archive/sessions/2026-01-01-story-a.md`\n- `OBS-gone` → `removed`\n",
        );
        root
    }

    fn present(resolved: Resolved) -> Evidence {
        match resolved {
            Resolved::Present(evidence) => *evidence,
            Resolved::Removed(receipt) => panic!("removed, as {receipt} records"),
            Resolved::Missing => panic!("missing"),
        }
    }

    /// t38b's R2 (a): a qualified `<story>/<id>` or `<bug>/<id>` resolves where the bare
    /// ID is ambiguous, and names the place's own receipt.
    #[test]
    fn qualified_story_and_bug_ids_resolve_where_the_bare_id_is_ambiguous() {
        let root = project();
        let index = Index::build(root.path()).unwrap();
        let error = index.resolve("2026-01-01-x").err().unwrap().to_string();
        assert!(
            error.contains("names evidence in more than one place")
                && error.contains("2026-01-01-story-a")
                && error.contains("2026-01-02-story-b"),
            "{error}"
        );
        for (place, claim) in [
            ("2026-01-01-story-a", "SYNTHETIC in story a"),
            ("2026-01-02-story-b", "SYNTHETIC in story b"),
        ] {
            let found = present(index.resolve(&format!("{place}/2026-01-01-x")).unwrap());
            assert_eq!(found.summary, claim);
        }
        // An alias resolves too, bare or qualified.
        assert_eq!(
            present(index.resolve("OBS-shared").unwrap()).summary,
            "SYNTHETIC in story a"
        );
        let found = present(index.resolve("2026-01-02-bug-b/2026-01-03-y").unwrap());
        assert_eq!(found.summary, "SYNTHETIC in bug b");
        assert!(index.holds("2026-01-01-bug-a/2026-01-03-y"));
        assert!(matches!(
            index.resolve("2026-01-02-story-b/2026-01-03-y").unwrap(),
            Resolved::Missing
        ));
    }

    /// t38b's R2 (b): two bugs holding differing receipts for one ID refuse, naming the
    /// bugs and the qualified form; identical copies are one receipt.
    #[test]
    fn differing_bug_receipts_refuse_and_identical_copies_are_one() {
        let root = project();
        let index = Index::build(root.path()).unwrap();
        let error = index.resolve("2026-01-03-y").err().unwrap().to_string();
        assert!(
            error.contains("names differing receipts in bugs 2026-01-01-bug-a, 2026-01-02-bug-b")
                && error.contains("<bug-id>/<evidence-id>"),
            "{error}"
        );
        assert_eq!(
            present(index.resolve("2026-01-04-z").unwrap()).summary,
            "SYNTHETIC copy"
        );
    }

    /// t38b's R2 (c), and E1 and E2: an ID the migration removed resolves as removed,
    /// naming its receipt, from a tombstone or the receipt's aliases; `load` refuses it
    /// with that receipt; a tombstone is never a receipt; an ID held nowhere is missing.
    #[test]
    fn removed_ids_name_their_receipt_and_load_refuses_them() {
        let root = project();
        let index = Index::build(root.path()).unwrap();
        for id in ["OBS-gone", "OBS-tombstoned"] {
            match index.resolve(id).unwrap() {
                Resolved::Removed(receipt) => assert_eq!(receipt, MIGRATION, "{id}"),
                _ => panic!("{id} is not removed"),
            }
            assert!(!index.holds(id), "{id}");
            let error = load(root.path(), id).err().unwrap().to_string();
            assert_eq!(
                error,
                format!("evidence {id} was removed by the 0.5.x migration, as {MIGRATION} records")
            );
        }
        // An alias the migration carried is not removed.
        assert!(matches!(
            index.resolve("OBS-carried").unwrap(),
            Resolved::Missing
        ));
        assert!(matches!(
            index.resolve("never-recorded").unwrap(),
            Resolved::Missing
        ));
        let error = load(root.path(), "never-recorded")
            .err()
            .unwrap()
            .to_string();
        assert_eq!(error, "evidence not found: never-recorded");
        assert!(!is_present(root.path(), "OBS-gone").unwrap());
        assert!(is_present(root.path(), "2026-01-04-z").unwrap());
    }
}
