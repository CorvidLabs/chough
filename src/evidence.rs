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
use crate::records::{
    local_path, now, print_json, read_bounded_record, valid_name
};
use crate::{cases, leaks};
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use clap::ValueEnum;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::{Component, Path};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// The 0.5.x evidence folder; nothing is written there in 0.6.0. Readers that report
/// the observations phase name it (timing.rs).
pub(super) const EVIDENCE_DIR: &str = ".chough/evidence";
/// The retired ledger's vault shard name (attachments.rs reads it while the vault
/// shards still exist; removed with step 8).
pub(super) const PROVENANCE: &str =
    "unauthenticated observation; not proof of human approval, test execution, or deployment";

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
        let mut body = format!(
            "**{}.**\n\nRecorded as {} in {} at revision {}, observed {}.",
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
                &self.cases.iter().map(|case| case.id.clone()).collect::<Vec<_>>(),
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
                                vec![("path".into(), path.clone()), ("digest".into(), digest.clone())]
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
        let actor = record
            .maps("actor")
            .first()
            .and_then(|map| crate::actor::Actor::from_front(&markdown::Value::Maps(vec![map.clone()])));
        Ok(Evidence {
            id: need("id")?,
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
                    let digest = fields.remove("digest").context("artifact map lacks digest")?;
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
                content: scalar("content"),
                input_inventory: None,
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

/// Write an entry by create-new (REQ-records-001), its ID equal to the file stem,
/// refusing a same-name different-content creation with no retry (REQ-cli-023), then
/// register it in the session: the `observations` front matter list, which the session
/// writer links in What was verified (REQ-records-004).
pub(super) fn record(
    root: &Path,
    evidence: &Evidence,
    screenshot: Option<(String, Vec<u8>)>,
) -> Result<String> {
    validate_record(evidence, &evidence.id)?;
    let session_id = active_session(root)?;
    let dir = evidence_dir(&session_id);
    // The screenshot is named by the entry's final ID, so it is written inside the
    // name allocation; a refused entry write leaves no screenshot behind (it is new
    // and uncommitted, REQ-records-012).
    let written_shot = std::cell::Cell::new(None);
    let name = identity::create_named(
        root,
        &dir,
        &evidence.id,
        ".md",
        markdown::RECORD_BOUND,
        |name| {
            let mut entry = evidence.clone();
            if let Some((extension, bytes)) = &screenshot {
                let file = write_screenshot(root, &session_id, name, extension, bytes)?;
                written_shot.set(Some(format!("{dir}/{file}")));
                entry.screenshot = Some(file);
            }
            let mut record = entry.to_record(name);
            if leaks::redacting() {
                record.redact_leaks(leaks::Local::current());
            } else {
                record.check_leaks(leaks::Local::current())?;
            }
            Ok(record.render())
        },
    );
    let name = match name {
        Ok(name) => name,
        Err(error) => {
            if let Some(shot) = written_shot.take() {
                let _ = fs::remove_file(local_path(root, &shot)?);
            }
            return Err(error);
        }
    };

    let mut session = match crate::checkpoint::read_session(root)? {
        crate::checkpoint::Session::V2(session) => *session,
    };
    if !session.observations.contains(&name) {
        session.observations.push(name.clone());
    }
    crate::checkpoint::save_checkpoint(root, &mut session)?;

    Ok(format!("{dir}/{name}.md"))
}

// ---------------------------------------------------------------------------
// `evidence add`.

#[derive(clap::Args)]
pub(super) struct EvidenceArgs {
    /// The entry's name; a date and slug not in use. Omit for automatic naming.
    id: Option<String>,
    #[arg(long = "criterion", required = true)]
    criteria: Vec<String>,
    #[arg(long = "requirement", required = true)]
    requirements: Vec<String>,
    #[arg(long = "contract", required = true)]
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
    #[arg(long)]
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
    #[arg(long)]
    cases: Vec<String>,
    #[arg(long)]
    method: Option<String>,
    #[arg(long)]
    track: Option<String>,
    #[arg(long)]
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

/// Key failing lines within the REQ-records-005 bounds: at most 20 lines and 2,048
/// bytes, escape sequences removed (cases runs strip with their own adapter rules),
/// redacted never refused.
pub(super) fn key_lines(text: &str) -> (Vec<String>, usize) {
    let mut lines: Vec<String> = text
        .replace("\r\n", "\n")
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect();
    if lines.len() > 20 {
        lines = lines[lines.len() - 20..].to_vec();
    }
    let mut bytes: usize = lines.iter().map(|line| line.len() + 1).sum();
    while bytes > 2_048 {
        let first = lines.remove(0);
        bytes -= first.len() + 1;
    }
    let mut redacted = 0;
    let local = leaks::Local::current();
    let lines = lines
        .into_iter()
        .map(|line| {
            let (clean, count) = leaks::redact(local, &line);
            redacted += count;
            clean
        })
        .collect();
    (lines, redacted)
}

/// Fingerprint an `--artifact` path: absolute paths outside the project are
/// fingerprinted in place and named by a neutral ID; project-relative paths keep their
/// place and digest. Returns (external ID, project-local fingerprints).
pub(super) fn bind_artifacts(
    root: &Path,
    artifacts: &[String],
) -> Result<(Option<String>, BTreeMap<String, String>)> {
    let mut external = None;
    let mut local = BTreeMap::new();
    for artifact in artifacts {
        if Path::new(artifact).is_absolute() {
            ensure!(
                external.is_none(),
                "one external artifact per entry; record another with its own entry"
            );
            external = Some(bind_external(root, artifact)?);
        } else {
            ensure!(
                canonical_path(artifact).as_deref() == Some(artifact.as_str()),
                "artifact path must be spelled canonically (relative, `/`-separated, no empty, `.` or `..` parts, no trailing `/`): {artifact}"
            );
            ensure!(
                local_path(root, artifact)?.is_file(),
                "missing artifact: {artifact}"
            );
            local.insert(artifact.clone(), crate::records::file_digest(root, artifact)?);
        }
    }
    ensure!(
        local.len() + external.iter().count() == artifacts.len(),
        "duplicate artifact paths"
    );
    Ok((external, local))
}

/// Fingerprint an external file in place and record it locally (REQ-records-005).
fn bind_external(root: &Path, path: &str) -> Result<String> {
    let target = Path::new(path);
    ensure!(
        target.exists() && !target.is_dir() && !target.is_symlink(),
        "external artifact cannot be read (missing, a directory or a symlink)"
    );
    ensure!(
        target.metadata()?.len() <= 64 * 1_048_576,
        "external artifact exceeds the 64 MiB bound"
    );
    let bytes = fs::read(target).context("reading the external artifact")?;
    let fingerprint = format!("sha256:{}", crate::records::digest_bytes(&bytes));
    crate::local::record_external(root, path, &fingerprint)
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
    validate_requirements(root, &args.contracts, &args.requirements)?
        .map_err(|miss| miss.for_evidence())?;
    let mut case_refs = vec![];
    for id in &args.cases {
        let digest = cases::current_digest(root, id)?
            .with_context(|| format!("unknown or retired case: {id}"))?;
        case_refs.push(CaseRef {
            id: id.clone(),
            digest,
        });
    }
    let (external, artifact_fingerprints) = bind_artifacts(root, &args.artifacts)?;
    let screenshot_data = match &args.screenshot {
        Some(path) => Some(read_screenshot(path)?),
        None => None,
    };
    let date = identity::utc_date(now()?);
    let source = args
        .cases
        .first()
        .cloned()
        .unwrap_or_else(|| args.summary.clone());
    let base = identity::dated_name(&date, &source, "evidence");
    let id = args.id.clone().unwrap_or(base);
    let (key_lines, redacted) = match &args.failing_lines {
        Some(text) if args.outcome != Outcome::Passed => key_lines(text),
        _ => (vec![], 0),
    };
    let mut limitations = args.limitations.clone();
    if let Some(note) = crate::local::toolchain_limitation(root) {
        limitations.push(note);
    }
    let evidence = Evidence {
        id,
        aliases: vec![],
        criteria: args.criteria,
        requirements: args.requirements,
        contracts: args.contracts,
        kind: args.kind,
        outcome: args.outcome,
        environment: args.environment,
        summary: args.summary,
        artifacts: args.artifacts,
        artifact_fingerprints,
        external,
        screenshot: None,
        fingerprint: None,
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
    let name = record(root, &evidence, screenshot_data)?;
    // The stored ID, so automation never parses it out of `recorded` (#60).
    print_json(&json!({
        "schema_version": 1,
        "observation": evidence.id,
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
        ensure!(crate::records::is_digest(digest), "invalid artifact digest in {name}");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Key screenshots (REQ-records-013).

const SCREENSHOT_BOUND: u64 = 524_288;
const SCREENSHOTS_PER_SESSION: usize = 5;
const SCREENSHOTS_SESSION_BYTES: u64 = 2_097_152;

/// Strip a PNG's metadata chunks (tEXt, zTXt, iTXt, tIME, eXIf).
fn strip_png(bytes: &[u8]) -> Result<Vec<u8>> {
    const PNG_SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    ensure!(bytes.starts_with(PNG_SIGNATURE), "not a PNG");
    let mut out = PNG_SIGNATURE.to_vec();
    let mut at = PNG_SIGNATURE.len();
    while at + 8 <= bytes.len() {
        let length = u32::from_be_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
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
        let length = u16::from_be_bytes(bytes[at + 2..at + 4].try_into().unwrap()) as usize;
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
        "the screenshot cannot be read (missing, a directory or a symlink)"
    );
    let bytes = fs::read(target).context("reading the screenshot")?;
    ensure!(
        bytes.len() as u64 <= SCREENSHOT_BOUND,
        "the screenshot is over the 524288-byte bound; crop or compress it, or record it by fingerprint with --artifact"
    );
    if bytes.starts_with(b"\x89PNG") {
        Ok(("png".into(), strip_png(&bytes)?))
    } else if bytes.starts_with(b"\xff\xd8") {
        Ok(("jpg".into(), strip_jpeg(&bytes)?))
    } else {
        bail!("the screenshot is not a PNG or JPEG; convert it, or record it by fingerprint with --artifact")
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
fn write_screenshot(
    root: &Path,
    session_id: &str,
    evidence_id: &str,
    extension: &str,
    bytes: &[u8],
) -> Result<String> {
    let (count, total) = screenshot_use(root, session_id);
    ensure!(
        count < SCREENSHOTS_PER_SESSION,
        "a session holds at most {SCREENSHOTS_PER_SESSION} screenshots; crop or compress, or record by fingerprint with --artifact"
    );
    ensure!(
        total + bytes.len() as u64 <= SCREENSHOTS_SESSION_BYTES,
        "the session's screenshots would pass 2097152 bytes in all; crop or compress, or record by fingerprint with --artifact"
    );
    let relative = format!("{}/{evidence_id}.{extension}", evidence_dir(session_id));
    let created = crate::records::write_new_bytes(root, &relative, bytes)
        .with_context(|| format!("writing the screenshot for {evidence_id}"))?;
    ensure!(created, "the screenshot {relative} already exists");
    Ok(format!("{evidence_id}.{extension}"))
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

/// One entry by ID or alias, across the open sessions (REQ-records-003).
pub(super) fn load(root: &Path, id: &str) -> Result<Evidence> {
    for session_id in crate::sessions::open_ids(root)? {
        let dir = evidence_dir(&session_id);
        if let Some((_, record)) = markdown::resolve(root, &dir, id)? {
            return Evidence::from_record(&record);
        }
    }
    bail!("evidence not found: {id}")
}

/// Whether an entry with this ID or alias exists in an open session.
pub(super) fn is_present(root: &Path, id: &str) -> Result<bool> {
    if !valid_name(id) {
        return Ok(false);
    }
    for session_id in crate::sessions::open_ids(root)? {
        let dir = evidence_dir(&session_id);
        if markdown::resolve(root, &dir, id)?.is_some() {
            return Ok(true);
        }
    }
    Ok(false)
}





/// The JSON view of an entry, as `status` and the case reports show it.
impl Evidence {
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
            "provenance": PROVENANCE,
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
    input_inventory: Option<InputInventory>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct InputInventory {
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

    /// The refusal for `evidence add`, which names its contracts.
    fn for_evidence(&self) -> anyhow::Error {
        self.render(|module| format!("add --contract specs/{module}/{module}.spec.md"))
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
    let mut text = String::new();
    let mut searched = vec![];
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
        text.push_str(
            &crate::records::read_spec_file(&local_path(root, companion)?)
                .context("contract requires its native requirements.md companion")?,
        );
        text.push('\n');
        // Named with `/` on every platform, as contracts and messages spell paths.
        #[cfg(windows)]
        searched.push(companion.replace('\\', "/"));
        #[cfg(not(windows))]
        searched.push(companion.to_string());
    }
    for requirement in requirements {
        if !(requirement.starts_with("REQ-") && holds_token(&text, requirement)) {
            return Ok(Err(RequirementMiss {
                id: requirement.clone(),
                owner: owning_module(root, requirement, &searched),
                searched,
            }));
        }
    }
    Ok(Ok(()))
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
}

impl<'a> Reads<'a> {
    pub(super) fn new(root: &'a Path) -> Self {
        Self {
            root,
            cases: cases::Digests::new(root),
            artifact_time: std::cell::Cell::new(Duration::ZERO),
        }
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
                        let actual = fs::read(&record.path)
                            .ok()
                            .map(|bytes| format!("sha256:{}", crate::records::digest_bytes(&bytes)));
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
        for evidence in load_all(self.root)? {
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

/// Labels observation references current, historical or unavailable.
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
        if !is_present(self.reads.root, id)? {
            return Ok("unavailable");
        }
        let evidence = load(self.reads.root, id)?;
        Ok(
            match self.reads.freshness(&evidence, &self.snapshot, environment)? {
                "current-observation" => "current",
                "environment-unchecked" => "current",
                "stale-revision-or-inputs" => "stale",
                "stale-artifact" => "stale",
                "stale-case-definition" => "stale-case-definition",
                "stale-environment" => "stale-environment",
                _ => "unknown",
            },
        )
    }

}

impl Reads<'_> {
    fn label_reference(
        &self,
        id: &str,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<&'static str> {
        if !is_present(self.root, id)? {
            return Ok("unavailable");
        }
        let evidence = load(self.root, id)?;
        Ok(label_for(
            self.freshness(&evidence, current, environment)?,
            environment,
        ))
    }

    /// Every observation an open session or a bug references, labelled.
    pub(super) fn reference_labels(
        &self,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<Vec<Value>> {
        let mut values = vec![];
        for (id, sources) in references_from(self.root)? {
            let label = self.label_reference(&id, current, environment)?;
            values.push(json!({"id": id, "label": label, "referenced_by": sources}));
        }
        Ok(values)
    }
}

/// Every observation an open session's front matter or a bug's record references.
fn references_from(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for session_id in crate::sessions::open_ids(root)? {
        let file = crate::sessions::session_file(&session_id);
        let text = read_bounded_record(&local_path(root, &file)?, markdown::SESSION_BOUND)?;
        let record = markdown::parse(&file, &text)?;
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

/// The evidence entries of one session as receipt values for its archived story:
/// id, result, claim, observed, revision, content, fingerprint, raw_log, case,
/// duration_seconds and the actor, each only when known (REQ-cli-017, REQ-history-006;
/// duration_seconds per the plan's delta correction 4). Front-matter-native maps.
#[cfg_attr(not(test), allow(dead_code))] // step 4's seam; wired in sessions.rs
pub(super) fn receipts_of(
    root: &Path,
    session_id: &str,
) -> Result<Vec<Vec<(String, String)>>> {
    let mut receipts = vec![];
    for evidence in load_session(root, session_id)? {
        let mut map = vec![
            ("id".into(), evidence.id.clone()),
            ("result".into(), evidence.outcome.name().to_string()),
            ("claim".into(), evidence.summary.clone()),
            (
                "observed".into(),
                iso_time(evidence.observed_at_unix_seconds),
            ),
            ("revision".into(), evidence.snapshot.revision.clone()),
        ];
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
        }
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
#[cfg_attr(not(test), allow(dead_code))] // wired in sessions.rs at the archive point
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

// ---------------------------------------------------------------------------
// Compatibility shims for files other steps own (stage.rs, sessions.rs,
// library.rs, timing.rs, main.rs, validate.rs, bugs.rs). They write nothing and go
// when steps 3, 8 and 9 land their removals of the retired retention (REQ-cli-033).

// removed at integration (t382)
pub(super) fn auto_prune(_root: &Path) -> Value {
    json!({"status": "skipped", "reason": "retention is removed in 0.6.0 (REQ-cli-033)"})
}

// removed at integration (t382)
pub(super) fn stale_holds(_root: &Path) -> Result<Vec<String>> {
    Ok(vec![])
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
                input_inventory: None,
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

/// Where an observation record was found.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(not(test), allow(dead_code))] // step 4 seam
pub(super) enum Found {
    /// An open session's evidence folder.
    Session(String),
    /// An archived story's receipts list.
    Story(String),
    /// A bug's receipts list.
    Bug(String),
}

/// One observation record, wherever it lives, for reports (timing) and bug
/// verification: id, aliases, case, result, observed (Unix seconds), environment,
/// revision, run time and where it was found.
#[derive(Clone, Debug)]
#[cfg_attr(not(test), allow(dead_code))] // step 4 seam
pub(super) struct ObservationView {
    pub(super) id: String,
    pub(super) aliases: Vec<String>,
    pub(super) case: Option<String>,
    pub(super) result: String,
    pub(super) observed: Option<u64>,
    pub(super) environment: String,
    pub(super) revision: String,
    pub(super) duration_seconds: Option<u64>,
    pub(super) found: Found,
}

/// A receipt map (a story's or a bug's `receipts:` entry) as a view.
fn receipt_view(map: &[(String, String)], found: Found) -> ObservationView {
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
        environment: get("environment").unwrap_or_default(),
        revision: get("revision").unwrap_or_default(),
        duration_seconds: get("duration_seconds").and_then(|text| text.parse().ok()),
        found,
    }
}

/// An open session's entry as a view.
fn entry_view(evidence: &Evidence, session_id: &str) -> ObservationView {
    ObservationView {
        id: evidence.id.clone(),
        aliases: evidence.aliases.clone(),
        case: evidence.cases.first().map(|case| case.id.clone()),
        result: evidence.outcome.name().to_string(),
        observed: Some(evidence.observed_at_unix_seconds),
        environment: evidence.environment.clone(),
        revision: evidence.snapshot.revision.clone(),
        duration_seconds: evidence.duration_seconds,
        found: Found::Session(session_id.to_string()),
    }
}

/// Every observation record: each open session's entries, then each archived story's
/// and each bug's receipts. Read-only.
#[cfg_attr(not(test), allow(dead_code))] // step 4 seam
pub(super) fn every_observation(root: &Path) -> Result<Vec<ObservationView>> {
    let mut views = vec![];
    for session_id in crate::sessions::open_ids(root)? {
        for evidence in load_session(root, &session_id)? {
            views.push(entry_view(&evidence, &session_id));
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
            let story = name.trim_end_matches(".md").to_string();
            let text = read_bounded_record(&archive.join(&name), markdown::SESSION_BOUND)?;
            let record = markdown::parse(&name, &text)?;
            for map in record.maps("receipts") {
                views.push(receipt_view(&map, Found::Story(story.clone())));
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
                    views.push(receipt_view(&map, Found::Bug(name.trim_end_matches(".md").into())));
                }
            }
        }
    }
    Ok(views)
}

/// Resolve an evidence ID (a bare ID, a 0.5.x alias or the qualified
/// `<session-id>/<evidence-id>` form) through the open sessions; two records holding
/// it refuse as ambiguous, naming the qualified form (the t38b/t382 agreement).
#[cfg_attr(not(test), allow(dead_code))] // step 4 seam
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
    match found.len() {
        0 => Ok(None),
        _ => {
            let first = found.pop().expect("one entry");
            if found.is_empty() {
                Ok(Some((format!("{}/{}", first.0, first.1.id), first.1)))
            } else {
                bail!(
                    "refused: {id} names an evidence entry in more than one session; qualify it as <session-id>/<evidence-id>"
                )
            }
        }
    }
}

/// The one entry for a validation run's evidence record shape (REQ-cli-023, plan
/// delta correction 4): written by validate.rs.
#[cfg_attr(not(test), allow(dead_code))] // step 4 seam
pub(super) struct ValidationRun {
    pub(super) id: String,
    pub(super) case: String,
    pub(super) environment: String,
    pub(super) state: String,
    pub(super) started: u64,
    pub(super) completed: Option<u64>,
    pub(super) failures: u64,
    pub(super) not_run: u64,
    pub(super) elapsed_seconds: u64,
    /// One attempt: started, observed, result and duration_seconds.
    pub(super) attempts: Vec<Vec<(String, String)>>,
    pub(super) summary: String,
}

/// Write a repeated validation run as one evidence entry (REQ-cli-023).
#[cfg_attr(not(test), allow(dead_code))] // step 4 seam
pub(super) fn record_validation_run(root: &Path, run: &ValidationRun) -> Result<String> {
    let session_id = active_session(root)?;
    let mut record = Record::new(&format!(
        "**{}.**\n\nA repeated validation run of {}: {}, {} failed, {} not run, {} seconds.\n",
        run.summary,
        run.case,
        run.state,
        run.failures,
        run.not_run,
        run.elapsed_seconds
    ));
    let set = |record: &mut Record, key: &str, value: &str| {
        record.set_scalar(key, value).expect("a scalar value")
    };
    set(&mut record, "kind", "evidence");
    set(&mut record, "id", &run.id);
    set(&mut record, "validation_run", "true");
    set(&mut record, "case", &run.case);
    set(&mut record, "environment", &run.environment);
    set(&mut record, "state", &run.state);
    set(&mut record, "started", &iso_time(run.started));
    if let Some(completed) = run.completed {
        set(&mut record, "completed", &iso_time(completed));
    }
    set(&mut record, "failures", &run.failures.to_string());
    set(&mut record, "not_run", &run.not_run.to_string());
    set(&mut record, "elapsed_seconds", &run.elapsed_seconds.to_string());
    if !run.attempts.is_empty() {
        record.set("attempts", markdown::Value::Maps(run.attempts.clone()))?;
    }
    set(&mut record, "claim", &run.summary);
    set(&mut record, "result", "not-run");
    set(&mut record, "evidence_kind", "structural");
    // HEAD, so freshness works for a run as for an entry; empty outside a repository.
    set(
        &mut record,
        "revision",
        &crate::records::head_revision(root).unwrap_or_default(),
    );
    let session_id_dir = evidence_dir(&session_id);
    let name = identity::create_named(
        root,
        &session_id_dir,
        &run.id,
        ".md",
        markdown::RECORD_BOUND,
        |name| {
            let mut record = record.clone();
            record.set_scalar("id", name).expect("the id");
            if leaks::redacting() {
                record.redact_leaks(leaks::Local::current());
            } else {
                record.check_leaks(leaks::Local::current())?;
            }
            Ok(record.render())
        },
    )?;
    Ok(format!("{session_id_dir}/{name}.md"))
}
