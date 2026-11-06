//! Observations bound to Git content, environment and, for cases, definition digests.

use crate::cases::{self, ARTIFACTS_DIR};
use crate::checkpoint::{Session, read_session};
use crate::context::hi_export_json;
use crate::identity;
use crate::records::{
    append_line, bounded, file_digest, head_revision, is_digest, local_path, native_json, now,
    print_json, read_native_text, read_record, read_spec_file, record_names, valid_name, write_new,
};
use crate::records::{digest_bytes, valid_module_name, valid_reference_id};
use crate::{bugs, prefs, validate};
use anyhow::{Context, Result, bail, ensure};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

pub(super) const EVIDENCE_DIR: &str = ".chough/evidence";
pub(super) const PROVENANCE: &str =
    "unauthenticated observation; not proof of human approval, test execution, or deployment";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize, ValueEnum)]
#[serde(rename_all = "kebab-case")]
pub(super) enum Outcome {
    Passed,
    Failed,
    NotRun,
}

#[derive(Clone, Deserialize, Serialize)]
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
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct CaseRef {
    pub(super) id: String,
    pub(super) digest: String,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Evidence {
    pub(super) schema_version: u32,
    pub(super) id: String,
    pub(super) criteria: Vec<String>,
    pub(super) requirements: Vec<String>,
    pub(super) contracts: Vec<String>,
    pub(super) kind: Kind,
    pub(super) outcome: Outcome,
    pub(super) environment: String,
    pub(super) summary: String,
    pub(super) artifacts: Vec<String>,
    pub(super) artifact_fingerprints: BTreeMap<String, String>,
    pub(super) observed_at_unix_seconds: u64,
    pub(super) snapshot: Snapshot,
    pub(super) provenance: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) cases: Vec<CaseRef>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) method: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) track: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) argv: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) exit_code: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) tests_run: Option<u64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(super) limitations: Vec<String>,
    /// Wall-clock seconds an explicit case run took; absent on manual records and on
    /// records written before it was kept, whose run time stays unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) duration_seconds: Option<u64>,
}

#[derive(clap::Args)]
pub(super) struct EvidenceArgs {
    id: Option<String>,
    #[arg(long = "criterion", required = true)]
    criteria: Vec<String>,
    #[arg(long = "requirement", required = true)]
    requirements: Vec<String>,
    #[arg(long = "contract", required = true)]
    contracts: Vec<String>,
    #[arg(long, value_enum)]
    kind: Kind,
    #[arg(long, value_enum)]
    outcome: Outcome,
    #[arg(long)]
    environment: String,
    #[arg(long)]
    summary: String,
    #[arg(long = "artifact")]
    artifacts: Vec<String>,
    /// Link a test-case definition; its current digest is bound to the observation.
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long, value_parser = ["automated", "manual", "agent-assisted"])]
    method: Option<String>,
    #[arg(long, value_parser = ["functional", "copy-localization", "design-qa"])]
    track: Option<String>,
    /// A known limit of this observation, such as a check that was not performed.
    #[arg(long = "limitation")]
    limitations: Vec<String>,
}

fn git(root: &Path, arguments: &[&str]) -> Result<Output> {
    let output =
        crate::records::run_owned(Command::new("git").args(arguments).current_dir(root), None)
            .context("git is required for evidence")?;
    ensure!(
        output.status.success(),
        "git {} failed: {}",
        arguments.join(" "),
        bounded(&output.stderr)
    );
    Ok(output)
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

pub(super) fn snapshot(root: &Path) -> Result<Snapshot> {
    let phase = Instant::now();
    let actual_root = git(root, &["rev-parse", "--show-toplevel"])?;
    ensure!(
        Path::new(std::str::from_utf8(&actual_root.stdout)?.trim()).canonicalize()? == root,
        "--root must be the Git repository root for evidence"
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
    // The legacy fingerprint keeps its exact byte stream, so records without a content
    // identity compare as before.
    let mut legacy = Sha256::new();
    legacy.update(revision.as_bytes());
    // (domain, key, path, change.md): change artifacts are a separate key domain, so no
    // ordinary path can collide with one.
    let mut entries = vec![];
    for name in files {
        if let Some(parent) = Path::new(name)
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
        match fingerprint_key(name) {
            Key::Skip => {}
            Key::Path(key) => entries.push((0_u8, key, name, false)),
            Key::Change { key, change_md } => entries.push((1_u8, key, name, change_md)),
        }
    }
    entries.sort();
    let mut content = Sha256::new();
    for (domain, key, name, change_md) in &entries {
        // An absent tracked path contributes nothing, as after its deletion is committed.
        hash_input(&mut content, root, name, Some((*domain, key)), *change_md)?;
    }
    crate::records::trace_phase("snapshot.hash", phase);
    let phase = Instant::now();
    let dirty_output = git(root, &["status", "--porcelain", "--untracked-files=all"])?;
    let dirty = String::from_utf8(dirty_output.stdout)?.lines().any(|line| {
        line.get(3..)
            .is_some_and(|name| fingerprint_key(name) != Key::Skip)
    });
    crate::records::trace_phase("snapshot.git-status", phase);
    Ok(Snapshot {
        revision,
        dirty,
        fingerprint: format!("{:x}", legacy.finalize()),
        content: Some(format!("{:x}", content.finalize())),
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
            let export = hi_export_json(root, family);
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

/// Whether `text` holds `id` as a whole token of ID characters. A dot belongs to the token
/// so a dotted module's `REQ-chat.service-1` is one token, except a sentence-ending dot.
fn holds_token(text: &str, id: &str) -> bool {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-' || c == '.'))
        .any(|part| part == id || part.trim_end_matches('.') == id)
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

const UNRESOLVED_HINT: &str = "it may be a typo, or a requirement an approved native change adds that is not yet materialized: run specsync change check <change> before running or recording cases that link it";

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

/// The module another companion would hold `id` for: `REQ-<module>-<part>`, a valid
/// module name whose `requirements.md` (read at any size) is not among those searched and holds
/// the ID as a token. Reads neither the checkpoint nor native deltas.
fn owning_module(root: &Path, id: &str, searched: &[String]) -> Option<String> {
    let (module, _) = id.strip_prefix("REQ-")?.rsplit_once('-')?;
    let companion = format!("specs/{module}/requirements.md");
    if !valid_module_name(module) || searched.contains(&companion) {
        return None;
    }
    let path = local_path(root, &companion).ok()?;
    let text = read_spec_file(&path).ok()?;
    holds_token(&text, id).then(|| module.to_string())
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
            &read_spec_file(&local_path(root, companion)?)
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

/// Record a new observation; never replaces an existing ID.
pub(super) fn record(root: &Path, evidence: &Evidence) -> Result<String> {
    validate_record(evidence, &format!("{}.json", evidence.id))?;
    let name = format!("{EVIDENCE_DIR}/{}.json", evidence.id);
    let content = serde_json::to_string_pretty(evidence)? + "\n";
    let created = match write_new(root, &name, &content) {
        Ok(created) => created,
        Err(error)
            if error
                .downcast_ref::<std::io::Error>()
                .is_some_and(|error| error.kind() == std::io::ErrorKind::AlreadyExists) =>
        {
            false
        }
        Err(error) => return Err(error),
    };
    if !created {
        let existing = read_record(&local_path(root, &name)?)?;
        ensure!(
            existing != content,
            "evidence ID already exists; record a new observation"
        );
        bail!(
            "observation identity collision: {} already exists with different content; refusing to overwrite or retry",
            evidence.id
        );
    }
    Ok(name)
}

pub(super) fn artifact_fingerprints(
    root: &Path,
    artifacts: &[String],
) -> Result<BTreeMap<String, String>> {
    let mut fingerprints = BTreeMap::new();
    for artifact in artifacts {
        ensure!(
            local_path(root, artifact)?.is_file(),
            "missing artifact: {artifact}"
        );
        fingerprints.insert(artifact.clone(), file_digest(root, artifact)?);
    }
    ensure!(
        fingerprints.len() == artifacts.len(),
        "duplicate artifact paths"
    );
    Ok(fingerprints)
}

pub(super) fn add(root: &Path, args: EvidenceArgs) -> Result<()> {
    read_session(root)?;
    if let Some(id) = &args.id {
        ensure!(valid_name(id), "invalid evidence ID");
        identity::require_explicit_observation_id(id)?;
    }
    ensure!(
        !args.environment.trim().is_empty() && !args.summary.trim().is_empty(),
        "environment and summary must not be blank"
    );
    for artifact in &args.artifacts {
        ensure!(
            canonical_path(artifact).as_deref() == Some(artifact.as_str()),
            "artifact path must be spelled canonically (relative, `/`-separated, no empty, `.` or `..` parts, no trailing `/`): {artifact}"
        );
    }
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
    let artifact_fingerprints = artifact_fingerprints(root, &args.artifacts)?;
    let id = match args.id {
        Some(id) => id,
        None => {
            let source = args.cases.first().cloned().unwrap_or_else(|| {
                args.kind
                    .to_possible_value()
                    .expect("evidence kind has a clap value")
                    .get_name()
                    .to_string()
            });
            identity::observation(&source)?
        }
    };
    let evidence = Evidence {
        schema_version: 2,
        id,
        criteria: args.criteria,
        requirements: args.requirements,
        contracts: args.contracts,
        kind: args.kind,
        outcome: args.outcome,
        environment: args.environment,
        summary: args.summary,
        artifacts: args.artifacts,
        artifact_fingerprints,
        observed_at_unix_seconds: now()?,
        snapshot: snapshot(root)?,
        provenance: PROVENANCE.into(),
        cases: case_refs,
        method: args.method,
        track: args.track,
        argv: vec![],
        exit_code: None,
        tests_run: None,
        limitations: args.limitations,
        duration_seconds: None,
    };
    let name = record(root, &evidence)?;
    print_json(&json!({"schema_version": 1, "recorded": name, "provenance": evidence.provenance}))
}

pub(super) fn validate_record(evidence: &Evidence, name: &str) -> Result<()> {
    ensure!(
        matches!(evidence.schema_version, 1 | 2),
        "unsupported evidence schema_version: {name}"
    );
    ensure!(
        name == format!("{}.json", evidence.id)
            && valid_name(&evidence.id)
            && !evidence.criteria.is_empty()
            && !evidence.requirements.is_empty()
            && !evidence.contracts.is_empty()
            && !evidence.environment.trim().is_empty()
            && !evidence.summary.trim().is_empty()
            && is_digest(&evidence.snapshot.fingerprint),
        "invalid evidence fields: {name}"
    );
    ensure!(
        evidence.snapshot.content.as_deref().is_none_or(is_digest),
        "invalid evidence fields: {name}: snapshot content must be a SHA-256 digest"
    );
    ensure!(
        evidence.artifacts.len() == evidence.artifact_fingerprints.len(),
        "invalid artifact bindings: {name}"
    );
    if evidence.schema_version == 1 {
        ensure!(
            matches!(
                evidence.kind,
                Kind::ProductTest
                    | Kind::Structural
                    | Kind::HumanObservation
                    | Kind::DeliveryObservation
            ) && evidence.outcome != Outcome::NotRun
                && evidence.cases.is_empty()
                && evidence.method.is_none()
                && evidence.track.is_none()
                && evidence.argv.is_empty()
                && evidence.exit_code.is_none()
                && evidence.tests_run.is_none()
                && evidence.limitations.is_empty()
                && evidence.duration_seconds.is_none(),
            "schema 1 evidence uses only schema 1 fields: {name}"
        );
    }
    for case in &evidence.cases {
        ensure!(
            valid_reference_id(&case.id) && is_digest(&case.digest),
            "invalid case binding: {name}"
        );
    }
    Ok(())
}

pub(super) fn load_all(root: &Path) -> Result<Vec<Evidence>> {
    let phase = Instant::now();
    let mut records = vec![];
    for name in record_names(root, EVIDENCE_DIR)? {
        let path = local_path(root, &format!("{EVIDENCE_DIR}/{name}"))?;
        let evidence: Evidence = serde_json::from_str(&read_record(&path)?)
            .with_context(|| format!("invalid evidence record: {name}"))?;
        validate_record(&evidence, &name)?;
        records.push(evidence);
    }
    crate::records::trace_phase("evidence.load", phase);
    Ok(records)
}

pub(super) fn load(root: &Path, id: &str) -> Result<Evidence> {
    ensure!(valid_name(id), "invalid evidence ID");
    let name = format!("{id}.json");
    let path = local_path(root, &format!("{EVIDENCE_DIR}/{name}"))?;
    let evidence: Evidence = serde_json::from_str(
        &read_record(&path).with_context(|| format!("unknown observation: {id}"))?,
    )
    .with_context(|| format!("invalid evidence record: {name}"))?;
    validate_record(&evidence, &name)?;
    Ok(evidence)
}

/// Content identities are compared when a schema 2 record has one. A schema 1 record,
/// whose schema has no content identity, and a record without one keep the
/// revision-bound legacy comparison, so they go stale after any new commit.
fn same_inputs(current: &Snapshot, recorded: &Evidence) -> bool {
    match &recorded.snapshot.content {
        Some(content) if recorded.schema_version >= 2 => current.content.as_ref() == Some(content),
        _ => current.fingerprint == recorded.snapshot.fingerprint,
    }
}

/// One report's root-bound freshness reads. It shares only case definitions;
/// artifact bytes and environment checks are still evaluated for every observation.
pub(super) struct Reads<'a> {
    root: &'a Path,
    cases: cases::Digests<'a>,
    artifact_time: Cell<Duration>,
}

impl<'a> Reads<'a> {
    pub(super) fn new(root: &'a Path) -> Self {
        Self {
            root,
            cases: cases::Digests::new(root),
            artifact_time: Cell::new(Duration::ZERO),
        }
    }

    fn artifacts_current(&self, evidence: &Evidence) -> Result<bool> {
        let started = Instant::now();
        let result = (|| {
            let mut current = true;
            for artifact in &evidence.artifacts {
                local_path(self.root, artifact)?;
                let expected = evidence
                    .artifact_fingerprints
                    .get(artifact)
                    .context("missing artifact binding")?;
                ensure!(is_digest(expected), "invalid artifact digest");
                if file_digest(self.root, artifact).as_ref().ok() != Some(expected) {
                    current = false;
                }
            }
            Ok(current)
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
        let artifacts_current = self.artifacts_current(evidence)?;
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
            values.push(
                json!({"record": evidence, "freshness": state, "label": label_for(state, environment)}),
            );
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

// ---------------------------------------------------------------------------
// Observation labels and reference-aware retention (REQ-history-001).

/// Append-only record of every pruned observation set.
pub(super) const LEDGER: &str = ".chough/pruned.jsonl";
const POLICY: &str = "chough-retention/1";
const KEEP_NEWEST: usize = 3;
const MIN_AGE_DAYS: u64 = 14;
const DAY_SECONDS: u64 = 86_400;
const WARN_BYTES: u64 = 50 * 1_048_576;
const LEDGER_LIMIT: u64 = 16 * 1_048_576;
const ELIGIBLE_REASON: &str = "routine automated run older than 14 days outside every protected set (checkpoint, bugs, native manifests, newest 3 and newest failure per case and environment, validation failures and first and last passes)";

/// Current only when present and its bound inputs still match; when no environment was
/// requested, the record's own environment is accepted.
fn label_for(freshness: &str, environment: Option<&str>) -> &'static str {
    match freshness {
        "current-observation" => "current",
        "environment-unchecked" if environment.is_none() => "current",
        _ => "historical",
    }
}

/// Whether the observation record exists; a pruned or missing one does not.
pub(super) fn is_present(root: &Path, id: &str) -> Result<bool> {
    if !valid_name(id) {
        return Ok(false);
    }
    Ok(local_path(root, &format!("{EVIDENCE_DIR}/{id}.json"))?.exists())
}

/// Observation IDs listed in the retention ledger.
fn read_ledger(root: &Path) -> Result<BTreeSet<String>> {
    let path = local_path(root, LEDGER)?;
    if !path.exists() {
        return Ok(BTreeSet::new());
    }
    ensure!(path.is_file(), "{LEDGER} must be a file");
    ensure!(
        fs::metadata(&path)?.len() <= LEDGER_LIMIT,
        "{LEDGER} exceeds {LEDGER_LIMIT} bytes"
    );
    let mut ids = BTreeSet::new();
    for line in fs::read_to_string(&path)?.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: Value = serde_json::from_str(line).context("invalid retention ledger line")?;
        let id = entry["id"]
            .as_str()
            .context("retention ledger line without an id")?
            .to_string();
        // A `restored` event puts a removed observation back; it is no longer pruned.
        if entry["event"] == "restored" {
            ids.remove(&id);
        } else {
            ids.insert(id);
        }
    }
    Ok(ids)
}

/// What the retention ledger recorded about one pruned observation.
pub(super) struct PrunedEntry {
    pub(super) observed_at_unix_seconds: Option<u64>,
    pub(super) outcome: Option<String>,
}

/// Each pruned observation's recorded time and outcome, by ID, read within the
/// ledger's bound; an unreadable ledger is an error the caller reports.
pub(super) fn ledger_entries(root: &Path) -> Result<BTreeMap<String, PrunedEntry>> {
    let path = local_path(root, LEDGER)?;
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    ensure!(path.is_file(), "{LEDGER} must be a file");
    ensure!(
        fs::metadata(&path)?.len() <= LEDGER_LIMIT,
        "{LEDGER} exceeds {LEDGER_LIMIT} bytes"
    );
    let mut entries = BTreeMap::new();
    for line in fs::read_to_string(&path)?.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: Value = serde_json::from_str(line).context("invalid retention ledger line")?;
        let id = entry["id"]
            .as_str()
            .context("retention ledger line without an id")?
            .to_string();
        if entry["event"] == "restored" {
            entries.remove(&id);
            continue;
        }
        entries.insert(
            id,
            PrunedEntry {
                observed_at_unix_seconds: entry["observed_at_unix_seconds"].as_u64(),
                outcome: entry["outcome"].as_str().map(str::to_string),
            },
        );
    }
    Ok(entries)
}

/// Labels observation references current, historical, pruned or unavailable.
pub(super) struct Labels<'a> {
    reads: Reads<'a>,
    snapshot: Result<Snapshot>,
    pruned: BTreeSet<String>,
}

impl<'a> Labels<'a> {
    pub(super) fn new(root: &'a Path) -> Self {
        Self {
            reads: Reads::new(root),
            snapshot: snapshot(root),
            // An unreadable ledger leaves absent observations unavailable, never current.
            pruned: read_ledger(root).unwrap_or_default(),
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
        self.reads
            .label_reference(id, &self.snapshot, environment, &self.pruned)
    }
}

impl Reads<'_> {
    fn label_reference(
        &self,
        id: &str,
        current: &Result<Snapshot>,
        environment: Option<&str>,
        pruned: &BTreeSet<String>,
    ) -> Result<&'static str> {
        if !is_present(self.root, id)? {
            return Ok(if pruned.contains(id) {
                "pruned"
            } else {
                "unavailable"
            });
        }
        let evidence = load(self.root, id)?;
        Ok(label_for(
            self.freshness(&evidence, current, environment)?,
            environment,
        ))
    }
}

/// How [`crate::sessions::every_checkpoint`] labels an archived session:
/// `archived session <key>`.
const ARCHIVED_SESSION: &str = "archived session ";

/// Files named directly, keyed by [`file_key`], with what names them: bug fixtures of
/// any status and the design and file references of every checkpoint that is not an
/// archived session. An archived session is done, and done closed its cycle, so its
/// references hold no proof (REQ-cli-045, KNOW-21).
fn file_references(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut add = |path: &str, source: String| {
        let sources = map.entry(file_key(path)).or_default();
        if !sources.contains(&source) {
            sources.push(source);
        }
    };
    for (label, checkpoint) in crate::sessions::every_checkpoint(root)? {
        if label.starts_with(ARCHIVED_SESSION) {
            continue;
        }
        for path in checkpoint
            .references
            .design
            .iter()
            .chain(&checkpoint.references.files)
        {
            add(path, label.clone());
        }
    }
    for (bug, path) in bugs::fixture_references(root)? {
        add(&path, bug);
    }
    Ok(map)
}

/// Every referenced observation ID with what references it: the checkpoint, bugs of
/// any status and stored validation runs.
fn references(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut add = |id: &str, source: String| {
        let sources = map.entry(id.to_string()).or_default();
        if !sources.contains(&source) {
            sources.push(source);
        }
    };
    for (label, checkpoint) in crate::sessions::every_checkpoint(root)? {
        for id in &checkpoint.observations {
            add(id, label.clone());
        }
    }
    for (bug, ids) in bugs::observation_references(root)? {
        for id in &ids {
            add(id, bug.clone());
        }
    }
    for (run, ids) in validate::run_references(root)? {
        for id in &ids {
            add(id, format!("validation {run}"));
        }
    }
    Ok(map)
}

/// Whether a protection reason is a closed bug's link: `<bug-id> <status>` with status
/// verified, duplicate or wont-fix.
fn closed_bug_reason(reason: &str) -> bool {
    reason.starts_with("BUG-")
        && reason
            .split_whitespace()
            .nth(1)
            .is_some_and(bugs::is_closed_status)
}

/// The key items of closed bugs (KNOW-22), each with the bug whose vault folder it moves
/// into: by observation ID, the earliest linked failed observation and, for a verified
/// bug, the first linked passed observation after it; by [`file_key`], a fixture under
/// `.chough/`. A linked record that is gone (already trimmed) or unreadable is not a key
/// item; an observation two bugs name goes with the bug first by ID.
fn closed_bug_key_items(
    root: &Path,
) -> Result<(BTreeMap<String, String>, BTreeMap<String, String>)> {
    let mut observations = BTreeMap::new();
    let mut fixtures = BTreeMap::new();
    for closed in bugs::closed_links(root)? {
        let bug = closed.id;
        let records: Vec<Evidence> = closed
            .linked
            .iter()
            .filter_map(|id| load(root, id).ok())
            .collect();
        let earliest = |outcome: Outcome, after: Option<u64>| {
            records
                .iter()
                .filter(|evidence| evidence.outcome == outcome)
                .filter(|evidence| after.is_none_or(|at| evidence.observed_at_unix_seconds > at))
                .min_by(|left, right| {
                    (left.observed_at_unix_seconds, &left.id)
                        .cmp(&(right.observed_at_unix_seconds, &right.id))
                })
        };
        if let Some(failure) = earliest(Outcome::Failed, None) {
            observations
                .entry(failure.id.clone())
                .or_insert_with(|| bug.clone());
            if closed.status == "verified"
                && let Some(fix) = earliest(Outcome::Passed, Some(failure.observed_at_unix_seconds))
            {
                observations
                    .entry(fix.id.clone())
                    .or_insert_with(|| bug.clone());
            }
        }
        if let Some(path) = closed.fixture.filter(|path| path.starts_with(".chough/")) {
            fixtures
                .entry(file_key(&path))
                .or_insert_with(|| bug.clone());
        }
    }
    Ok((observations, fixtures))
}

impl Reads<'_> {
    /// Referenced labels share the same case index as this request's observations.
    pub(super) fn reference_labels(
        &self,
        current: &Result<Snapshot>,
        environment: Option<&str>,
    ) -> Result<Vec<Value>> {
        let pruned = read_ledger(self.root).unwrap_or_default();
        let mut values = vec![];
        for (id, sources) in references(self.root)? {
            let label = self.label_reference(&id, current, environment, &pruned)?;
            values.push(json!({"id": id, "label": label, "referenced_by": sources}));
        }
        Ok(values)
    }
}

/// Native manifests that protect observation sets, or why they could not be read.
#[derive(Clone)]
struct Native {
    state: &'static str,
    detail: String,
    /// (reason, listed paths)
    manifests: Vec<(String, BTreeSet<String>)>,
}

impl Native {
    fn unavailable(detail: String) -> Self {
        Self {
            state: "unavailable",
            detail,
            manifests: vec![],
        }
    }
}

/// Change directories, rejecting links and names that are not plain change IDs.
fn change_dirs(root: &Path, directory: &str) -> Result<Vec<String>, String> {
    let path = local_path(root, directory).map_err(|error| error.to_string())?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let entries = fs::read_dir(&path).map_err(|error| format!("{directory}: {error}"))?;
    let mut names = vec![];
    for entry in entries {
        let entry = entry.map_err(|error| format!("{directory}: {error}"))?;
        let kind = entry
            .file_type()
            .map_err(|error| format!("{directory}: {error}"))?;
        let name = entry
            .file_name()
            .to_str()
            .map(str::to_string)
            .ok_or_else(|| format!("{directory}: non-UTF-8 entry"))?;
        if kind.is_symlink() {
            return Err(format!("{directory}/{name} is a symlink"));
        }
        if kind.is_dir() {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

fn change_id_ok(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 255
        && !id.starts_with('-')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

/// The paths a change's acceptance manifest lists, read with `specsync change status`.
fn manifest(root: &Path, id: &str) -> Result<BTreeSet<String>, String> {
    if !change_id_ok(id) {
        return Err(format!("unexpected native change ID {id:?}"));
    }
    let response = native_json(root, "specsync", &["change", "status", id, "--json"]);
    let value = response.value.ok_or_else(|| {
        format!(
            "{} was {}: {}",
            response.section.source, response.section.state, response.section.content
        )
    })?;
    let entries = value
        .get("acceptance_entries")
        .and_then(Value::as_array)
        .ok_or_else(|| format!("specsync change status {id} reported no acceptance_entries"))?;
    let mut paths = BTreeSet::new();
    for entry in entries {
        paths.insert(
            entry
                .get("path")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("an acceptance entry of {id} has no path"))?
                .to_string(),
        );
    }
    Ok(paths)
}

/// The most recent archived change: the highest finalization timestamp.
fn latest_archive(root: &Path) -> Result<Option<String>, String> {
    let mut latest: Option<(u64, String, String)> = None;
    for directory in change_dirs(root, ".specsync/archive/changes")? {
        let name = format!(".specsync/archive/changes/{directory}/finalization.json");
        // A native SpecSync file: read within SpecSync's bound, not Chough's (LIFE-15).
        let text = local_path(root, &name)
            .and_then(|path| read_native_text(&path))
            .map_err(|error| format!("{name}: {error}"))?;
        let value: Value =
            serde_json::from_str(&text).map_err(|error| format!("{name}: {error}"))?;
        let id = value["change_id"]
            .as_str()
            .filter(|id| directory.ends_with(id))
            .ok_or_else(|| format!("{name} has no change_id matching its directory"))?;
        let timestamp = value["timestamp"]
            .as_u64()
            .ok_or_else(|| format!("{name} has no timestamp"))?;
        let candidate = (timestamp, directory.clone(), id.to_string());
        if latest.as_ref().is_none_or(|current| candidate > *current) {
            latest = Some(candidate);
        }
    }
    Ok(latest.map(|(_, _, id)| id))
}

fn native_manifests(root: &Path) -> Result<Native> {
    if !local_path(root, ".specsync/config.toml")?.is_file()
        && !local_path(root, ".specsync/config.json")?.is_file()
    {
        return Ok(Native {
            state: "not-configured",
            detail: "No native SpecSync configuration; no native manifest protects a set.".into(),
            manifests: vec![],
        });
    }
    let read = || -> Result<Vec<(String, BTreeSet<String>)>, String> {
        let mut manifests = vec![];
        for id in change_dirs(root, ".specsync/changes")? {
            let paths = manifest(root, &id)?;
            manifests.push((format!("native active {id}"), paths));
        }
        if let Some(id) = latest_archive(root)? {
            let paths = manifest(root, &id)?;
            manifests.push((format!("native archive {id}"), paths));
        }
        Ok(manifests)
    };
    Ok(match read() {
        Ok(manifests) => Native {
            state: "observed",
            detail: "Active change manifests and the most recent archived manifest were read with `specsync change status <id> --json`.".into(),
            manifests,
        },
        Err(detail) => Native::unavailable(format!(
            "{detail}. Every set a native manifest could list stays protected."
        )),
    })
}

/// Native acceptance paths shared with history retention. Unavailable native
/// protection is an error, so a maintenance preview cannot invent eligibility.
pub(super) fn retention_protection(root: &Path) -> Result<Vec<String>> {
    if !local_path(root, ".specsync/config.toml")?.is_file()
        && !local_path(root, ".specsync/config.json")?.is_file()
    {
        return Ok(vec![]);
    }
    let mut ids: BTreeSet<String> = change_dirs(root, ".specsync/changes")
        .map_err(anyhow::Error::msg)?
        .into_iter()
        .collect();
    for directory in change_dirs(root, ".specsync/archive/changes").map_err(anyhow::Error::msg)? {
        let path = format!(".specsync/archive/changes/{directory}/finalization.json");
        let value: Value = serde_json::from_str(&read_native_text(&local_path(root, &path)?)?)?;
        let id = value["change_id"]
            .as_str()
            .filter(|id| directory.ends_with(id))
            .context("unavailable archived manifest identity")?;
        ids.insert(id.to_string());
    }
    let mut paths = BTreeSet::new();
    for id in ids {
        paths.extend(manifest(root, &id).map_err(anyhow::Error::msg)?);
    }
    Ok(paths.into_iter().collect())
}

/// Why a record is not a routine automated run, if it is not.
fn non_routine(evidence: &Evidence) -> Option<String> {
    if !matches!(
        evidence.kind,
        Kind::ProductTest | Kind::Structural | Kind::DryRun
    ) {
        return serde_json::to_value(&evidence.kind)
            .ok()
            .and_then(|value| value.as_str().map(str::to_string));
    }
    match evidence.method.as_deref() {
        Some("automated") => None,
        Some(method) => Some(method.to_string()),
        None => Some("unspecified-method".into()),
    }
}

fn case_ids(evidence: &Evidence) -> Vec<&str> {
    evidence.cases.iter().map(|case| case.id.as_str()).collect()
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

/// The key two spellings of one file share: the canonical spelling with case folded,
/// since macOS and Windows filesystems usually ignore case. On a case-sensitive
/// filesystem folding can only join distinct files, which protects more, never less.
fn file_key(path: &str) -> String {
    canonical_path(path)
        .unwrap_or_else(|| path.to_string())
        .to_lowercase()
}

/// What `map` records for `key` and for each folder that contains it: the case-folded
/// canonical keys of [`file_key`], cut at `/`, so `a/b` names `a/b/c` but not `a/bc`
/// (#109 review: a bug fixture or checkpoint reference may name a folder).
fn naming<'a, V>(map: &'a BTreeMap<String, V>, key: &str) -> impl Iterator<Item = &'a V> {
    let mut ancestors = vec![key.to_string()];
    let mut rest = key;
    while let Some((parent, _)) = rest.rsplit_once('/') {
        ancestors.push(parent.to_string());
        rest = parent;
    }
    ancestors
        .into_iter()
        .filter_map(move |ancestor| map.get(&ancestor))
}

fn file_size(path: &Path) -> u64 {
    fs::symlink_metadata(path)
        .ok()
        .filter(|metadata| metadata.is_file())
        .map_or(0, |metadata| metadata.len())
}

fn directory_bytes(path: &Path) -> u64 {
    let Ok(entries) = fs::read_dir(path) else {
        return 0;
    };
    entries
        .flatten()
        .map(|entry| match entry.file_type() {
            Ok(kind) if kind.is_dir() => directory_bytes(&entry.path()),
            Ok(kind) if kind.is_file() => file_size(&entry.path()),
            _ => 0,
        })
        .sum()
}

/// One observation set: a record plus the artifacts only it references.
struct ObservationSet {
    evidence: Evidence,
    removable: Vec<String>,
    bytes: u64,
    age_days: u64,
    reasons: Vec<String>,
}

struct Classification {
    sets: Vec<ObservationSet>,
    native: Native,
    total_bytes: u64,
    /// The newest routine run of each case and environment (KNOW-17).
    newest_run: BTreeSet<String>,
}

fn classify(root: &Path, at: u64) -> Result<Classification> {
    classify_with(root, at, None)
}

/// [`classify`], optionally reusing native manifests already read, so a re-check after
/// native probes reads every local protection without starting another process.
fn classify_with(root: &Path, at: u64, native: Option<Native>) -> Result<Classification> {
    let records = load_all(root)?;
    let references = references(root)?;
    let named_files = file_references(root)?;
    let native = match native {
        Some(native) => native,
        None => native_manifests(root)?,
    };
    let manifest_keys: Vec<(&String, BTreeSet<String>)> = native
        .manifests
        .iter()
        .map(|(reason, paths)| (reason, paths.iter().map(|path| file_key(path)).collect()))
        .collect();

    // Records using each file, counted by one key per file however it is spelled.
    let mut artifact_users: BTreeMap<String, usize> = BTreeMap::new();
    for evidence in &records {
        let unique: BTreeSet<String> = evidence
            .artifacts
            .iter()
            .map(|artifact| file_key(artifact))
            .collect();
        for key in unique {
            *artifact_users.entry(key).or_default() += 1;
        }
    }

    // Newest three routine runs and the newest routine failure per case and environment.
    // A record bound to several cases belongs to the group of each; one with none to "".
    let mut groups: BTreeMap<(&str, &str), Vec<&Evidence>> = BTreeMap::new();
    for evidence in records
        .iter()
        .filter(|evidence| non_routine(evidence).is_none())
    {
        let mut cases = case_ids(evidence);
        if cases.is_empty() {
            cases.push("");
        }
        cases.sort_unstable();
        cases.dedup();
        for case in cases {
            groups
                .entry((case, evidence.environment.as_str()))
                .or_default()
                .push(evidence);
        }
    }
    let mut newest = BTreeSet::new();
    let mut newest_run = BTreeSet::new();
    let mut last_failure = BTreeSet::new();
    for group in groups.values_mut() {
        group.sort_by(|left, right| {
            (right.observed_at_unix_seconds, &right.id)
                .cmp(&(left.observed_at_unix_seconds, &left.id))
        });
        newest_run.extend(group.first().map(|evidence| evidence.id.clone()));
        newest.extend(
            group
                .iter()
                .take(KEEP_NEWEST)
                .map(|evidence| evidence.id.clone()),
        );
        if let Some(failure) = group
            .iter()
            .find(|evidence| evidence.outcome == Outcome::Failed)
        {
            last_failure.insert(failure.id.clone());
        }
    }

    // Failures and the first and last passes of each validation run.
    let outcomes: BTreeMap<String, Outcome> = records
        .iter()
        .map(|evidence| (evidence.id.clone(), evidence.outcome.clone()))
        .collect();
    let mut validation: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for (run, ids) in validate::run_references(root)? {
        let passes: Vec<&String> = ids
            .iter()
            .filter(|id| outcomes.get(id.as_str()) == Some(&Outcome::Passed))
            .collect();
        let mut protected: BTreeSet<&String> = ids
            .iter()
            .filter(|id| outcomes.get(id.as_str()) == Some(&Outcome::Failed))
            .collect();
        protected.extend(passes.first());
        protected.extend(passes.last());
        for id in protected {
            validation
                .entry(id.clone())
                .or_default()
                .push(format!("validation {run}"));
        }
    }

    let mut sets = vec![];
    for evidence in records {
        let record_path = format!("{EVIDENCE_DIR}/{}.json", evidence.id);
        let mut reasons = vec![];
        if let Some(sources) = references.get(&evidence.id) {
            reasons.extend(
                sources
                    .iter()
                    .filter(|source| !source.starts_with("validation "))
                    .cloned(),
            );
        }
        if native.state == "unavailable" {
            reasons.push("native unavailable".into());
        }
        let mut keys = vec![file_key(&record_path)];
        keys.extend(evidence.artifacts.iter().map(|artifact| file_key(artifact)));
        for (reason, paths) in &manifest_keys {
            if keys.iter().any(|key| paths.contains(key)) {
                reasons.push((*reason).clone());
            }
        }
        for key in &keys {
            for source in naming(&named_files, key).flatten() {
                if !reasons.contains(source) {
                    reasons.push(source.clone());
                }
            }
        }
        if let Some(reason) = non_routine(&evidence) {
            reasons.push(reason);
        }
        if newest.contains(&evidence.id) {
            reasons.push("newest".into());
        }
        if last_failure.contains(&evidence.id) {
            reasons.push("last failure".into());
        }
        if let Some(runs) = validation.get(&evidence.id) {
            reasons.extend(runs.iter().cloned());
        }
        let age_seconds = at.saturating_sub(evidence.observed_at_unix_seconds);
        if age_seconds < MIN_AGE_DAYS * DAY_SECONDS {
            reasons.push("recent".into());
        }
        let mut removable = vec![];
        let mut removable_keys = BTreeSet::new();
        let mut bytes = file_size(&local_path(root, &record_path)?);
        for artifact in &evidence.artifacts {
            let key = file_key(artifact);
            // The case-preserving spelling must lie under the artifacts directory, so a
            // differently cased directory on a case-sensitive filesystem is never touched.
            let inside = canonical_path(artifact)
                .is_some_and(|path| path.starts_with(&format!("{ARTIFACTS_DIR}/")));
            if !inside
                || artifact_users.get(&key) != Some(&1)
                || naming(&named_files, &key).next().is_some()
                || !removable_keys.insert(key)
            {
                continue;
            }
            let path = local_path(root, artifact)?;
            if path.is_file() {
                bytes += file_size(&path);
                removable.push(artifact.clone());
            }
        }
        sets.push(ObservationSet {
            age_days: age_seconds / DAY_SECONDS,
            evidence,
            removable,
            bytes,
            reasons,
        });
    }
    let total_bytes = directory_bytes(&local_path(root, EVIDENCE_DIR)?)
        + directory_bytes(&local_path(root, ARTIFACTS_DIR)?);
    Ok(Classification {
        sets,
        native,
        total_bytes,
        newest_run,
    })
}

fn policy(authority: Option<&str>) -> Value {
    json!({
        "name": POLICY,
        "keep_newest": KEEP_NEWEST,
        "keep_newest_failure": true,
        "min_age_days": MIN_AGE_DAYS,
        "warn_bytes": WARN_BYTES,
        "authority": authority,
    })
}

/// Classify, and with an authority remove eligible sets, appending each to the ledger.
fn retention(root: &Path, authority: Option<&str>) -> Result<Value> {
    let _budget = authority.map(|_| crate::history::maintenance_budget());
    let at = now()?;
    let classification = classify(root, at)?;
    let mut eligible = vec![];
    let mut protected = vec![];
    let mut reclaimable = 0;
    for set in &classification.sets {
        let evidence = &set.evidence;
        if set.reasons.is_empty() {
            reclaimable += set.bytes;
            let mut files = vec![format!("{EVIDENCE_DIR}/{}.json", evidence.id)];
            files.extend(set.removable.iter().cloned());
            eligible.push(json!({
                "id": evidence.id,
                "cases": case_ids(evidence),
                "environment": evidence.environment,
                "age_days": set.age_days,
                "outcome": evidence.outcome,
                "bytes": set.bytes,
                "files": files,
                "reason": ELIGIBLE_REASON,
            }));
        } else {
            protected.push(json!({
                "id": evidence.id,
                "cases": case_ids(evidence),
                "environment": evidence.environment,
                "age_days": set.age_days,
                "outcome": evidence.outcome,
                "reasons": set.reasons,
            }));
        }
    }
    let warning = (classification.total_bytes > WARN_BYTES).then(|| {
        format!(
            "Evidence and artifacts hold {} bytes, above the {WARN_BYTES}-byte (50 MiB) warning; review this preview and ask the human before pruning.",
            classification.total_bytes
        )
    });
    let mut pruned = vec![];
    let mut skipped = vec![];
    if let Some(authority) = authority {
        let head = head_revision(root);
        for set in classification
            .sets
            .iter()
            .filter(|set| set.reasons.is_empty())
        {
            let evidence = &set.evidence;
            let fresh = classify(root, now()?)?;
            if !fresh.sets.iter().any(|current| {
                current.evidence.id == evidence.id
                    && current.reasons.is_empty()
                    && current.removable == set.removable
            }) {
                skipped.push(json!({"id":evidence.id,"reason":"protection or shared artifact references changed"}));
                continue;
            }
            let mut candidate_changed = false;
            for (artifact, expected) in &evidence.artifact_fingerprints {
                if !file_digest(root, artifact).is_ok_and(|actual| actual == *expected) {
                    candidate_changed = true;
                    break;
                }
            }
            if candidate_changed {
                skipped.push(json!({"id":evidence.id,"reason":"artifact is changed, missing or unsafe; observation retained"}));
                continue;
            }
            let protection = crate::history::protection_fingerprint(root)?;
            let record_path = format!("{EVIDENCE_DIR}/{}.json", evidence.id);
            let record_digest = file_digest(root, &record_path)?;
            let mut removed = vec![record_path.clone()];
            removed.extend(set.removable.iter().cloned());
            let entry = json!({
                "schema_version": 1,
                "id": evidence.id,
                "cases": case_ids(evidence),
                "environment": evidence.environment,
                "outcome": evidence.outcome,
                "kind": evidence.kind,
                "revision": evidence.snapshot.revision,
                "observed_at_unix_seconds": evidence.observed_at_unix_seconds,
                "artifacts": evidence.artifact_fingerprints,
                "record_digest": record_digest,
                "removed": removed,
                "reason": ELIGIBLE_REASON,
                "policy": policy(Some(authority)),
                "head": head,
                "pruned_at_unix_seconds": at,
            });
            // The ledger line comes first, so a removal is never unexplained.
            append_line(root, LEDGER, &serde_json::to_string(&entry)?)?;
            for artifact in &set.removable {
                ensure!(
                    crate::history::protection_fingerprint(root)? == protection,
                    "references changed during cleanup; remaining files retained; inspect the partial ledger entry for {}",
                    evidence.id
                );
                ensure!(
                    file_digest(root, artifact)? == evidence.artifact_fingerprints[artifact],
                    "artifact changed during cleanup; remaining files retained"
                );
                remove_if_present(root, artifact).context("partial evidence cleanup; observation retained and ledger explains attempted removals")?;
            }
            ensure!(
                crate::history::protection_fingerprint(root)? == protection
                    && file_digest(root, &record_path)? == record_digest,
                "observation or protection changed during cleanup; remaining files retained"
            );
            remove_if_present(root, &record_path).context(
                "partial evidence cleanup; inspect the existing record and retention ledger",
            )?;
            pruned.push(evidence.id.clone());
        }
    }
    Ok(json!({
        "schema_version": 1,
        "applied": authority.is_some(),
        "authority": authority,
        "policy": policy(authority),
        "native": {"state": classification.native.state, "detail": classification.native.detail},
        "eligible": eligible,
        "protected": protected,
        "totals": {
            "sets": classification.sets.len(),
            "eligible": eligible.len(),
            "protected": protected.len(),
            "evidence_bytes": classification.total_bytes,
            "reclaimable_bytes": reclaimable,
        },
        "warning": warning,
        "pruned": pruned,
        "skipped": skipped,
        "ledger": LEDGER,
        "note": "Files are removed from the working tree only; committed bytes stay recoverable from Git history. Native archives, ignore rules and history are never changed.",
    }))
}

/// The newest ledger line per observation ID, with `restored` events undoing a removal.
pub(super) fn ledger_records(root: &Path) -> Result<BTreeMap<String, Value>> {
    let path = local_path(root, LEDGER)?;
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    ensure!(
        path.is_file() && fs::metadata(&path)?.len() <= LEDGER_LIMIT,
        "{LEDGER} must be a file within {LEDGER_LIMIT} bytes"
    );
    let mut records = BTreeMap::new();
    for line in fs::read_to_string(&path)?.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let entry: Value = serde_json::from_str(line).context("invalid retention ledger line")?;
        let id = entry["id"]
            .as_str()
            .context("retention ledger line without an id")?
            .to_string();
        if entry["event"] == "restored" {
            records.remove(&id);
        } else {
            records.insert(id, entry);
        }
    }
    Ok(records)
}

/// When an observation record was observed: None when it has no record, and an error
/// when a record exists but cannot be read (review of #108), so callers never treat an
/// unreadable record as a missing one.
pub(super) fn observed_at(root: &Path, id: &str) -> Result<Option<u64>> {
    if !is_present(root, id)? {
        return Ok(None);
    }
    Ok(Some(load(root, id)?.observed_at_unix_seconds))
}

/// Outcome, environment and cases of a present observation, for knowledge entries.
pub(super) fn observation_summary(root: &Path, id: &str) -> Option<(String, String, Vec<String>)> {
    if !is_present(root, id).ok()? {
        return None;
    }
    let evidence = load(root, id).ok()?;
    let outcome = serde_json::to_value(&evidence.outcome)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".into());
    let cases = case_ids(&evidence)
        .into_iter()
        .map(str::to_string)
        .collect();
    Some((outcome, evidence.environment.clone(), cases))
}

/// One archived change as the post-archive summary sees it: its ID, whether it is one
/// of the hot (newest) archives, and the paths its verification manifest lists.
pub(super) struct ListedArchive {
    pub(super) id: String,
    pub(super) hot: bool,
    pub(super) paths: BTreeSet<String>,
    /// When the change was finalized, which ends its cycle (KNOW-20).
    pub(super) finalized: u64,
}

/// The archived change whose cycle `observed_at` falls in: the first finalized at or
/// after it (KNOW-20). None when it was observed after the newest archive (current work).
fn owner_by_cycle(archives: &[ListedArchive], observed_at: u64) -> Option<&ListedArchive> {
    archives
        .iter()
        .filter(|archive| archive.finalized >= observed_at)
        .min_by(|left, right| (left.finalized, &left.id).cmp(&(right.finalized, &right.id)))
}

/// For each archived change, the present observation records no archive lists that were
/// observed in its cycle (KNOW-20), so its knowledge entry can summarize them.
pub(super) fn attributed_records(
    root: &Path,
    archives: &[ListedArchive],
) -> Result<BTreeMap<String, Vec<String>>> {
    let listed: BTreeSet<String> = archives
        .iter()
        .flat_map(|archive| archive.paths.iter().map(|path| file_key(path)))
        .collect();
    let mut found: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for evidence in load_all(root)? {
        let record = file_key(&format!("{EVIDENCE_DIR}/{}.json", evidence.id));
        if listed.contains(&record) {
            continue;
        }
        if let Some(owner) = owner_by_cycle(archives, evidence.observed_at_unix_seconds) {
            found
                .entry(owner.id.clone())
                .or_default()
                .push(evidence.id.clone());
        }
    }
    for ids in found.values_mut() {
        ids.sort();
    }
    Ok(found)
}

const SUMMARY_REASON: &str = "archived-summary: routine automated proof listed only by cold archived changes (outside the newest 3), summarized in the knowledge directory and committed identically at HEAD; bytes stay recoverable from Git";

/// Eligible candidates, protected sets, sets not at HEAD, and the native read state.
struct SummaryPlan {
    eligible: Vec<SummaryCandidate>,
    protected: Vec<Value>,
    uncommitted: Vec<Value>,
    native: Native,
}

struct SummaryCandidate {
    id: String,
    files: Vec<String>,
    /// The cold archives that list the set, or the one that owns it by cycle time.
    archives: Vec<String>,
    /// The archives whose manifests list a file of the set; only their validity can change.
    listed_by: Vec<String>,
    /// Owned by cycle time rather than listed by a manifest (KNOW-20).
    attributed: bool,
    /// A loose artifact group rather than an observation set (KNOW-18).
    loose: bool,
    /// A non-routine set whose only protection was being non-routine: it moves whole
    /// into the vault (KNOW-19).
    move_all: bool,
    /// A failed run: its artifacts move into the vault (KNOW-19).
    move_artifacts: bool,
    /// Files that are key items of a closed bug, with that bug: they move into the
    /// bug's vault folder instead of an archive's (KNOW-22).
    bug_files: BTreeMap<String, String>,
    bytes: u64,
}

/// The files of a candidate that move into the vault instead of being removed, with
/// their attachment paths (KNOW-19): a non-routine set whole, a failed run's artifacts,
/// screenshots and failure-first logs.
fn candidate_moves(candidate: &SummaryCandidate, knowledge_dir: &str) -> BTreeMap<String, String> {
    candidate
        .files
        .iter()
        .enumerate()
        .filter(|(index, file)| {
            candidate.move_all
                || candidate.bug_files.contains_key(*file)
                || (candidate.move_artifacts && (candidate.loose || *index > 0))
                || crate::attachments::is_key_item(file)
        })
        .map(|(_, file)| {
            let folder = candidate
                .bug_files
                .get(file)
                .unwrap_or(&candidate.archives[0]);
            (
                file.clone(),
                crate::attachments::attachment(knowledge_dir, folder, file),
            )
        })
        .collect()
}

const LOOSE_REASON: &str = "archived-summary: loose artifacts no observation record references, owned by a cold archived change (outside the newest 3), summarized in the knowledge directory and committed identically at HEAD; key items move into its attachments; bytes stay recoverable from Git";

/// The first commit time of every file under `.chough/artifacts/`, read once per project
/// per process: an apply makes no commit, so it cannot change during one (KNOW-20).
fn first_commits(root: &Path) -> BTreeMap<String, u64> {
    use std::sync::{Mutex, OnceLock};
    static CACHE: OnceLock<Mutex<BTreeMap<std::path::PathBuf, BTreeMap<String, u64>>>> =
        OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut cache = cache
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    cache
        .entry(root.to_path_buf())
        .or_insert_with(|| {
            git(
                root,
                &[
                    "log",
                    "--diff-filter=A",
                    "--format=@%ct",
                    "--name-only",
                    "--",
                    crate::attachments::ARTIFACTS_DIR,
                ],
            )
            .map(|output| {
                crate::attachments::parse_first_commits(&String::from_utf8_lossy(&output.stdout))
            })
            .unwrap_or_default()
        })
        .clone()
}

/// The blob each of `files` has in HEAD's tree; untracked files are absent.
fn head_blobs(root: &Path, files: &[String]) -> Result<BTreeMap<String, String>> {
    let mut tracked = BTreeMap::new();
    for chunk in files.chunks(256) {
        let mut arguments = vec!["ls-tree", "HEAD", "--"];
        arguments.extend(chunk.iter().map(String::as_str));
        let Ok(listed) = git(root, &arguments) else {
            return Ok(BTreeMap::new());
        };
        for line in String::from_utf8_lossy(&listed.stdout).lines() {
            if let Some((meta, path)) = line.split_once('\t')
                && let Some(blob) = meta.split_whitespace().nth(2)
            {
                tracked.insert(path.to_string(), blob.to_string());
            }
        }
    }
    Ok(tracked)
}

/// Files whose working-tree bytes equal the blob tracked at HEAD. Hashing uses
/// `--no-filters`, so the exact bytes on disk are compared: with `core.autocrlf` or
/// other clean filters a file that differs from HEAD byte for byte is never identical.
fn identical_at_head(root: &Path, files: &[String]) -> Result<BTreeSet<String>> {
    let tracked = head_blobs(root, files)?;
    let mut identical = BTreeSet::new();
    for chunk in files.chunks(256) {
        let present: Vec<&String> = chunk
            .iter()
            .filter(|file| local_path(root, file).is_ok_and(|path| path.is_file()))
            .collect();
        if present.is_empty() {
            continue;
        }
        let mut arguments = vec!["hash-object", "--no-filters", "--"];
        arguments.extend(present.iter().map(|file| file.as_str()));
        let hashed = git(root, &arguments)?;
        for (file, blob) in present
            .iter()
            .zip(String::from_utf8_lossy(&hashed.stdout).lines())
        {
            if tracked.get(file.as_str()).map(String::as_str) == Some(blob.trim()) {
                identical.insert((*file).clone());
            }
        }
    }
    Ok(identical)
}

/// The Git blob ID of exactly `bytes`, with no filters applied.
fn blob_of(root: &Path, bytes: &[u8]) -> Result<String> {
    let output = crate::records::run_owned(
        Command::new("git")
            .args(["hash-object", "--no-filters", "--stdin"])
            .current_dir(root),
        Some(bytes),
    )
    .context("git is required for evidence")?;
    ensure!(
        output.status.success(),
        "git hash-object --no-filters --stdin failed: {}",
        bounded(&output.stderr)
    );
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Where `--apply` holds a file between moving it out of place and deleting it. Holds sit
/// outside every listed record directory, so a stale one never breaks a record listing.
pub(super) const HOLD_DIR: &str = ".chough/removing";

/// Holds left in [`HOLD_DIR`] by an apply that stopped partway or found a conflict.
pub(super) fn stale_holds(root: &Path) -> Result<Vec<String>> {
    let mut names = vec![];
    let path = local_path(root, HOLD_DIR)?;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            names.push(format!(
                "{HOLD_DIR}/{}",
                entry?.file_name().to_string_lossy()
            ));
        }
    }
    names.sort();
    Ok(names)
}

/// Put `hold` back at `path` only if nothing is there: a hard link fails when the target
/// exists, so a file a writer recreated meanwhile is never replaced.
fn put_back(hold: &Path, path: &Path) -> std::io::Result<()> {
    fs::hard_link(hold, path)?;
    fs::remove_file(hold)
}

/// Where [`remove_verified`] holds `relative` while it compares the bytes: always a file
/// directly in [`HOLD_DIR`], named by a digest of the path, never beside the record, so a
/// hold left behind cannot sit in a directory that readers list.
fn hold_path(relative: &str) -> String {
    format!(
        "{HOLD_DIR}/{}.hold",
        &crate::records::digest_bytes(relative.as_bytes())[..24]
    )
}

/// Remove `relative` only while it still holds `expected`. The file is first moved into
/// [`HOLD_DIR`], so later writes to its path cannot be lost, then compared and deleted at
/// once. Bytes that differ are put back without replacing anything and refused.
fn remove_verified(root: &Path, relative: &str, expected: &[u8]) -> Result<()> {
    remove_verified_as(root, relative, expected, None)
}

/// [`remove_verified`], also requiring `permissions` when given: a vault copy a refused
/// apply created is removed only while it still holds the bytes and mode it was written
/// with, so a copy another writer changed is kept (#110).
fn remove_verified_as(
    root: &Path,
    relative: &str,
    expected: &[u8],
    permissions: Option<&fs::Permissions>,
) -> Result<()> {
    let path = local_path(root, relative)?;
    let holds = local_path(root, HOLD_DIR)?;
    fs::create_dir_all(&holds)?;
    let hold = local_path(root, &hold_path(relative))?;
    ensure!(
        !hold.exists(),
        "{} already exists; nothing was removed",
        hold.display()
    );
    fs::rename(&path, &hold).with_context(|| format!("removing {relative}"))?;
    let actual = fs::read(&hold).with_context(|| format!("re-reading {relative}"));
    let same_mode = permissions.is_none_or(|expected| {
        fs::symlink_metadata(&hold).is_ok_and(|metadata| metadata.permissions() == *expected)
    });
    if same_mode && actual.as_deref().ok() == Some(expected) {
        fs::remove_file(&hold).with_context(|| format!("removing {relative}"))?;
        let _ = fs::remove_dir(&holds);
        return Ok(());
    }
    match put_back(&hold, &path) {
        Ok(()) => {
            let _ = fs::remove_dir(&holds);
            bail!("{relative} changed during the apply; its bytes were kept and it was not removed")
        }
        Err(error) => bail!(
            "{relative} changed during the apply and could not be put back without replacing a file written meanwhile ({error}); its changed bytes are kept at {}",
            hold.display()
        ),
    }
}

/// What [`place_copy`] did at a vault destination.
#[derive(Debug, PartialEq, Eq)]
enum Placed {
    /// This apply created the copy; a refused apply removes it while it is unchanged.
    Created,
    /// The same bytes were already there; they are not this apply's to remove.
    AlreadyThere,
}

/// Put `content` at `path` without ever replacing what is there. The copy is written and
/// given its mode (`finish`) in a private staging file beside `path`, then hard-linked into
/// place, which fails rather than replace a file. A failure at any step removes only the
/// staging file, so another writer's bytes at `path` are never deleted (#115 review).
/// Different bytes already at `path` are refused and kept.
fn place_copy(
    path: &Path,
    content: &[u8],
    finish: impl FnOnce(&Path) -> std::io::Result<()>,
) -> Result<Placed> {
    use std::sync::atomic::{AtomicU64, Ordering};
    static STAGED: AtomicU64 = AtomicU64::new(0);
    let name = path
        .file_name()
        .context("a vault copy needs a file name")?
        .to_string_lossy()
        .into_owned();
    let staged = path.with_file_name(format!(
        ".{name}.chough-copy-{}-{}.tmp",
        std::process::id(),
        STAGED.fetch_add(1, Ordering::Relaxed)
    ));
    let staged_copy = (|| -> std::io::Result<()> {
        use std::io::Write;
        let mut copy = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&staged)?;
        copy.write_all(content)?;
        drop(copy);
        finish(&staged)
    })();
    if let Err(error) = staged_copy {
        let _ = fs::remove_file(&staged);
        return Err(error.into());
    }
    let linked = fs::hard_link(&staged, path);
    let _ = fs::remove_file(&staged);
    match linked {
        Ok(()) => Ok(Placed::Created),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let existing = fs::read(path).with_context(|| {
                format!(
                    "reading {}, which appeared during the apply",
                    path.display()
                )
            })?;
            ensure!(
                existing == content,
                "{} appeared during the apply with different bytes; a move never overwrites",
                path.display()
            );
            Ok(Placed::AlreadyThere)
        }
        Err(error) => Err(error.into()),
    }
}

/// Classify observation sets that archived manifests list: candidates (listed only by
/// cold archives, no protection left once age and the latest-archive reason are set
/// aside, every file committed identically at HEAD), protected sets with reasons, and
/// sets held back because their bytes are not at HEAD.
fn summary_plan(
    root: &Path,
    archives: &[ListedArchive],
    native: Option<Native>,
    knowledge_dir: &str,
) -> Result<SummaryPlan> {
    let (pending, mut protected, native) = summary_protections(root, archives, native, None)?;
    // A key item whose attachment path would be too long for a Windows checkout stays,
    // and so does its set: nothing of it moves or is removed (REQ-cli-046, #109).
    let (pending, too_long): (Vec<SummaryCandidate>, Vec<SummaryCandidate>) =
        pending.into_iter().partition(|candidate| {
            candidate_moves(candidate, knowledge_dir)
                .values()
                .all(|target| target.chars().count() <= crate::attachments::MAX_ATTACHMENT_PATH)
        });
    // A move never overwrites (#109 review): an attachment already holding other bytes
    // keeps both files and its set; one holding the same bytes needs no copy.
    let (pending, occupied): (Vec<SummaryCandidate>, Vec<(SummaryCandidate, Vec<String>)>) = {
        let mut free = vec![];
        let mut occupied = vec![];
        for candidate in pending {
            let mut differing = vec![];
            for (file, target) in candidate_moves(&candidate, knowledge_dir) {
                let target_path = local_path(root, &target)?;
                if fs::symlink_metadata(&target_path).is_err() {
                    continue;
                }
                let same = fs::read(&target_path).ok().is_some_and(|existing| {
                    local_path(root, &file)
                        .ok()
                        .and_then(|source| fs::read(source).ok())
                        .is_some_and(|source| source == existing)
                });
                if !same {
                    differing.push(target);
                }
            }
            if differing.is_empty() {
                free.push(candidate);
            } else {
                occupied.push((candidate, differing));
            }
        }
        (free, occupied)
    };
    for (candidate, differing) in occupied {
        protected.push(json!({
            "id": candidate.id,
            "kind": if candidate.loose { "loose-artifact" } else { "observation" },
            "archives": candidate.archives,
            "reasons": [format!(
                "its attachment already holds different bytes, and a move never overwrites: {}",
                differing.join(", ")
            )],
        }));
    }
    for candidate in too_long {
        let long: Vec<String> = candidate_moves(&candidate, knowledge_dir)
            .into_values()
            .filter(|target| target.chars().count() > crate::attachments::MAX_ATTACHMENT_PATH)
            .collect();
        protected.push(json!({
            "id": candidate.id,
            "kind": if candidate.loose { "loose-artifact" } else { "observation" },
            "archives": candidate.archives,
            "reasons": [format!(
                "attachment path longer than {} characters, which a Windows checkout cannot hold: {}",
                crate::attachments::MAX_ATTACHMENT_PATH,
                long.join(", ")
            )],
        }));
    }
    let all: Vec<String> = pending
        .iter()
        .flat_map(|candidate| candidate.files.iter().cloned())
        .collect();
    let identical = identical_at_head(root, &all)?;
    let mut eligible = vec![];
    let mut uncommitted = vec![];
    for candidate in pending {
        let differing: Vec<&String> = candidate
            .files
            .iter()
            .filter(|file| !identical.contains(*file))
            .collect();
        if differing.is_empty() {
            eligible.push(candidate);
        } else {
            uncommitted.push(json!({
                "id": candidate.id,
                "files": differing,
                "reason": "not committed identically at HEAD; commit it before it can be summarized",
            }));
        }
    }
    Ok(SummaryPlan {
        eligible,
        protected,
        uncommitted,
        native,
    })
}

/// The protection rules of [`summary_plan`], from local state and the given manifests:
/// sets listed only by cold archives with no protection left (pending the HEAD check),
/// and protected sets with their reasons.
fn summary_protections(
    root: &Path,
    archives: &[ListedArchive],
    native: Option<Native>,
    loose_only: Option<&BTreeSet<String>>,
) -> Result<(Vec<SummaryCandidate>, Vec<Value>, Native)> {
    let classification = classify_with(root, now()?, native)?;
    let keyed: Vec<(&ListedArchive, BTreeSet<String>)> = archives
        .iter()
        .map(|archive| {
            (
                archive,
                archive.paths.iter().map(|path| file_key(path)).collect(),
            )
        })
        .collect();
    let (bug_observations, bug_fixtures) = closed_bug_key_items(root)?;
    let mut pending = vec![];
    let mut protected = vec![];
    for set in &classification.sets {
        let evidence = &set.evidence;
        let record_path = format!("{EVIDENCE_DIR}/{}.json", evidence.id);
        let mut keys = vec![file_key(&record_path)];
        keys.extend(evidence.artifacts.iter().map(|artifact| file_key(artifact)));
        let listing: Vec<&ListedArchive> = keyed
            .iter()
            .filter(|(_, paths)| keys.iter().any(|key| paths.contains(key)))
            .map(|(archive, _)| *archive)
            .collect();
        // Proof no archive lists belongs to the change whose cycle it was observed in
        // (KNOW-20); proof observed after the newest archive is current work.
        let attributed = listing.is_empty();
        let owners: Vec<&ListedArchive> = if attributed {
            match owner_by_cycle(archives, evidence.observed_at_unix_seconds) {
                Some(owner) => vec![owner],
                None => continue,
            }
        } else {
            listing.clone()
        };
        let superseded = !classification.newest_run.contains(&evidence.id);
        // A non-routine set (manual, agent-assisted, a human observation) is a key item:
        // when that is its only protection it moves into the vault (KNOW-19).
        let key_item = non_routine(evidence);
        let mut reasons: Vec<String> = set
            .reasons
            .iter()
            .filter(|reason| *reason != "recent" && !reason.starts_with("native archive "))
            // A closed bug holds none of its observations here: its key items move into
            // the vault under the bug, and the rest follows the cold rule (KNOW-22).
            // `evidence prune`, which moves nothing, still keeps them.
            .filter(|reason| !closed_bug_reason(reason))
            .filter(|reason| Some(*reason) != key_item.as_ref())
            // Only the newest run of a case and environment stays protected as newest
            // (KNOW-17); the newest failure keeps its own protection.
            .filter(|reason| !(superseded && *reason == "newest"))
            .cloned()
            .collect();
        reasons.extend(
            owners
                .iter()
                .filter(|archive| archive.hot)
                .map(|archive| format!("hot archive {}", archive.id)),
        );
        let cold: Vec<String> = owners
            .iter()
            .filter(|archive| !archive.hot)
            .map(|archive| archive.id.clone())
            .collect();
        if reasons.is_empty() {
            let mut files = vec![record_path];
            files.extend(set.removable.iter().cloned());
            // A closed bug's key item moves whole, with its artifacts, under the bug.
            let bug_files: BTreeMap<String, String> = bug_observations
                .get(&evidence.id)
                .map(|bug| {
                    files
                        .iter()
                        .map(|file| (file.clone(), bug.clone()))
                        .collect()
                })
                .unwrap_or_default();
            pending.push(SummaryCandidate {
                id: evidence.id.clone(),
                files,
                listed_by: listing.iter().map(|archive| archive.id.clone()).collect(),
                archives: cold,
                attributed,
                loose: false,
                move_all: key_item.is_some(),
                move_artifacts: evidence.outcome == Outcome::Failed,
                bug_files,
                bytes: set.bytes,
            });
        } else {
            protected.push(json!({
                "id": evidence.id,
                "cases": case_ids(evidence),
                "archives": cold,
                "reasons": reasons,
            }));
        }
    }
    // Loose artifacts no record references belong to the cold change that lists them or
    // whose cycle their first commit falls in (KNOW-18, KNOW-20).
    let referenced: BTreeSet<String> = classification
        .sets
        .iter()
        .flat_map(|set| {
            set.evidence
                .artifacts
                .iter()
                .map(|artifact| file_key(artifact))
        })
        .collect();
    let is_referenced = |file: &str| {
        let key = file_key(file);
        referenced.contains(&key)
            || referenced
                .iter()
                .any(|folder| key.starts_with(&format!("{folder}/")))
    };
    let named = file_references(root)?;
    let held: BTreeMap<String, String> = crate::attachments::open_bug_fixtures(root)?
        .into_iter()
        .map(|(bug, fixture)| (file_key(&fixture), format!("{bug} open")))
        .collect();
    let first = first_commits(root);
    // The re-check of a loose group judges only its planned files: a file an earlier
    // removal released into the folder is never removed, so its own age or names cannot
    // protect the planned ones (#109 review).
    let considered = |file: &str| {
        is_referenced(file) || loose_only.is_some_and(|planned| !planned.contains(file))
    };
    for group in crate::attachments::loose_groups(root, &considered)? {
        let keys: Vec<String> = group.files.iter().map(|file| file_key(file)).collect();
        let mut reasons: Vec<String> = vec![];
        for key in &keys {
            for source in naming(&named, key).flatten() {
                if !source.starts_with("BUG-") && !reasons.contains(source) {
                    reasons.push(source.clone());
                }
            }
            for bug in naming(&held, key) {
                if !reasons.contains(bug) {
                    reasons.push(bug.clone());
                }
            }
        }
        // Each file is owned on its own: by the archives that list it, or else by the
        // cycle its own first commit falls in (#109 review: one cold-listed file must not
        // make a current sibling cold).
        let mut listing: Vec<&ListedArchive> = vec![];
        let mut owners: Vec<&ListedArchive> = vec![];
        let mut attributed = false;
        let mut current: Vec<&String> = vec![];
        for (file, key) in group.files.iter().zip(&keys) {
            let listed: Vec<&ListedArchive> = keyed
                .iter()
                .filter(|(_, paths)| paths.contains(key))
                .map(|(archive, _)| *archive)
                .collect();
            let owned: Vec<&ListedArchive> = if listed.is_empty() {
                match first
                    .get(file)
                    .and_then(|added| owner_by_cycle(archives, *added))
                {
                    Some(owner) => {
                        attributed = true;
                        vec![owner]
                    }
                    None => {
                        current.push(file);
                        vec![]
                    }
                }
            } else {
                listed.clone()
            };
            for archive in listed {
                if !listing.iter().any(|known| known.id == archive.id) {
                    listing.push(archive);
                }
            }
            for archive in owned {
                if !owners.iter().any(|known| known.id == archive.id) {
                    owners.push(archive);
                }
            }
        }
        // All current or uncommitted: current work, left alone as before. Some: the folder
        // stays whole, and says why.
        if current.len() == group.files.len() {
            continue;
        }
        if !current.is_empty() {
            reasons.push(format!(
                "current work: first committed after the newest archive, or not committed: {}",
                current
                    .iter()
                    .map(|file| file.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        reasons.extend(
            owners
                .iter()
                .filter(|archive| archive.hot)
                .map(|archive| format!("hot archive {}", archive.id)),
        );
        let cold: Vec<String> = owners
            .iter()
            .filter(|archive| !archive.hot)
            .map(|archive| archive.id.clone())
            .collect();
        if reasons.is_empty() {
            // A closed bug's fixture is a key item: it moves under the bug (KNOW-22).
            let bug_files: BTreeMap<String, String> = group
                .files
                .iter()
                .filter_map(|file| {
                    naming(&bug_fixtures, &file_key(file))
                        .next()
                        .map(|bug| (file.clone(), bug.clone()))
                })
                .collect();
            pending.push(SummaryCandidate {
                id: group.id,
                files: group.files,
                listed_by: listing.iter().map(|archive| archive.id.clone()).collect(),
                archives: cold,
                attributed,
                loose: true,
                move_all: false,
                move_artifacts: false,
                bug_files,
                bytes: group.bytes,
            });
        } else {
            protected.push(json!({
                "id": group.id,
                "kind": "loose-artifact",
                "archives": cold,
                "reasons": reasons,
            }));
        }
    }
    Ok((pending, protected, classification.native))
}

/// Re-apply the protection rules right before removing one of `candidate`'s files: no
/// native change is open, and the set is still unprotected with no file beyond the plan. The
/// bytes themselves are checked against the plan by [`remove_verified`].
fn still_unprotected(
    root: &Path,
    archives: &[ListedArchive],
    native: &Native,
    candidate: &SummaryCandidate,
) -> Result<()> {
    let open = change_dirs(root, ".specsync/changes").map_err(anyhow::Error::msg)?;
    ensure!(
        open.is_empty(),
        "a native change opened during removal ({}); {} was not removed",
        open.join(", "),
        candidate.id
    );
    let planned: BTreeSet<String> = candidate.files.iter().cloned().collect();
    let (pending, protected, _) = summary_protections(
        root,
        archives,
        Some(native.clone()),
        candidate.loose.then_some(&planned),
    )?;
    // The first planned file (an observation set's record) is removed last, so it is still
    // present here. Every planned file still present must still belong to this set alone;
    // files this apply already removed drop out. The set may gain a file an earlier removal
    // released from a shared artifact: only planned files are ever removed, so the gained
    // one simply stays. A loose group lists its files by path, so a gained file can sort
    // before its planned first file (#109 closeout); only an observation set must keep it.
    if pending.iter().any(|current| {
        current.id == candidate.id
            && (candidate.loose || current.files.first() == candidate.files.first())
            && candidate
                .files
                .iter()
                .filter(|file| local_path(root, file).is_ok_and(|path| path.exists()))
                .all(|file| current.files.contains(file))
    }) {
        return Ok(());
    }
    let now = protected
        .iter()
        .find(|entry| entry["id"] == candidate.id.as_str())
        .map_or_else(
            || "its files changed".to_string(),
            |entry| format!("now protected by {}", entry["reasons"]),
        );
    bail!(
        "protections changed during removal ({}: {now}); its files were not removed",
        candidate.id
    )
}

fn one_line_bounded(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(200)
        .collect()
}

/// Terminal evidence validity of each archive, as `specsync change status` reports it.
/// Fails closed: an archive whose validity cannot be read is an error, never a value.
fn validity(root: &Path, ids: &BTreeSet<String>) -> Result<BTreeMap<String, String>> {
    // Each read costs tens of seconds inside SpecSync (spec-sync#831), so they run in
    // parallel, at most 8 at once; errors are reported in ID order, not completion order.
    let ids: Vec<&String> = ids.iter().collect();
    let workers = std::thread::available_parallelism()
        .map_or(2, std::num::NonZeroUsize::get)
        .clamp(2, 8)
        .min(ids.len().max(1));
    let next = std::sync::atomic::AtomicUsize::new(0);
    let mut results: Vec<(usize, Result<String>)> = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..workers)
            .map(|_| {
                scope.spawn(|| {
                    let mut done = vec![];
                    loop {
                        let index = next.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                        let Some(id) = ids.get(index) else {
                            break;
                        };
                        done.push((index, validity_of(root, id)));
                    }
                    done
                })
            })
            .collect();
        handles
            .into_iter()
            .flat_map(|handle| handle.join().unwrap_or_default())
            .collect()
    });
    results.sort_by_key(|(index, _)| *index);
    let mut found = BTreeMap::new();
    for (index, state) in results {
        found.insert(ids[index].clone(), state?);
    }
    Ok(found)
}

fn validity_of(root: &Path, id: &str) -> Result<String> {
    let response = native_json(root, "specsync", &["change", "status", id, "--json"]);
    let Some(state) = response
        .value
        .as_ref()
        .and_then(|value| value["summary"]["terminal_evidence"]["validity"].as_str())
    else {
        bail!(
            "`specsync change status {id} --json` did not report terminal evidence validity ({}: {}); nothing was removed without a native validity check",
            response.section.state,
            one_line_bounded(&response.section.content)
        );
    };
    Ok(state.to_string())
}

/// `chough library archive [--apply]`: preview the post-archive summary, or with an
/// authority remove the eligible proof after `write_entries` summarized it. Ledger lines
/// come first; a removal that changes any affected archive's validity is restored.
pub(super) fn archive_summary(
    root: &Path,
    archives: &[ListedArchive],
    knowledge_dir: &str,
    authority: Option<&str>,
    write_entries: &dyn Fn(&Path) -> Result<()>,
) -> Result<Value> {
    let _budget = authority.map(|_| crate::history::maintenance_budget());
    let SummaryPlan {
        eligible,
        protected,
        uncommitted,
        native,
    } = summary_plan(root, archives, None, knowledge_dir)?;
    let entry_path = |archive: &str| format!("{knowledge_dir}/archives/{archive}.md");
    let listed: Vec<Value> = eligible
        .iter()
        .map(|candidate| {
            json!({
                "id": candidate.id,
                "kind": if candidate.loose { "loose-artifact" } else { "observation" },
                "files": candidate.files,
                "moves": candidate_moves(candidate, knowledge_dir),
                "archives": candidate.archives,
                "bytes": candidate.bytes,
                "knowledge": entry_path(&candidate.archives[0]),
            })
        })
        .collect();
    let reclaimable: u64 = eligible.iter().map(|candidate| candidate.bytes).sum();
    let mut pruned = vec![];
    if let Some(authority) = authority {
        let holds = stale_holds(root)?;
        ensure!(
            holds.is_empty(),
            "an earlier apply left files in {HOLD_DIR} ({}); inspect them and move each back or delete it before applying again",
            holds.join(", ")
        );
        let open = change_dirs(root, ".specsync/changes").map_err(anyhow::Error::msg)?;
        ensure!(
            open.is_empty(),
            "a native change is open ({}); summarize archived proof only between changes, after it is archived or abandoned",
            open.join(", ")
        );
        ensure!(
            native.state != "unavailable",
            "native manifests are unavailable: {}",
            native.detail
        );
        let affected: BTreeSet<String> = eligible
            .iter()
            .flat_map(|candidate| candidate.listed_by.iter().cloned())
            .collect();
        if !eligible.is_empty() {
            let before = validity(root, &affected)?;
            // The validity probes run external processes, so protections may have
            // changed meanwhile (a checkpoint reference, a bug, an open change). Re-plan
            // from local state only, reusing the manifests already read, and refuse
            // unless every eligible set is still eligible with the same files.
            let open = change_dirs(root, ".specsync/changes").map_err(anyhow::Error::msg)?;
            ensure!(
                open.is_empty(),
                "a native change opened during the validity check ({}); nothing was removed",
                open.join(", ")
            );
            let fresh = summary_plan(root, archives, Some(native.clone()), knowledge_dir)?;
            let gained: Vec<String> = eligible
                .iter()
                .filter(|candidate| {
                    !fresh.eligible.iter().any(|current| {
                        current.id == candidate.id && current.files == candidate.files
                    })
                })
                .map(|candidate| {
                    let now = fresh
                        .protected
                        .iter()
                        .chain(&fresh.uncommitted)
                        .find(|entry| entry["id"] == candidate.id.as_str());
                    match now {
                        Some(entry) if !entry["reasons"].is_null() => {
                            format!("{}: now protected by {}", candidate.id, entry["reasons"])
                        }
                        Some(entry) => format!(
                            "{}: no longer committed identically at HEAD {}",
                            candidate.id, entry["files"]
                        ),
                        None => format!("{}: its files changed", candidate.id),
                    }
                })
                .collect();
            ensure!(
                gained.is_empty(),
                "protections or committed bytes changed during the native validity check ({}); nothing was removed. Preview again",
                gained.join("; ")
            );
            // Read everything removal needs before the first ledger line is written.
            let mut prepared = vec![];
            for candidate in &eligible {
                let evidence = if candidate.loose {
                    None
                } else {
                    Some(load(root, &candidate.id)?)
                };
                let record_digest = file_digest(root, &candidate.files[0])?;
                let mut bytes = vec![];
                for file in &candidate.files {
                    bytes.push((file.clone(), fs::read(local_path(root, file)?)?));
                }
                let moves = candidate_moves(candidate, knowledge_dir);
                prepared.push((candidate, evidence, record_digest, bytes, moves));
            }
            // The native validity check runs external processes; a file may have changed
            // since the plan found it identical at HEAD. Check the exact bytes held for
            // removal against HEAD before anything is written, and refuse otherwise.
            let files: Vec<String> = eligible
                .iter()
                .flat_map(|candidate| candidate.files.iter().cloned())
                .collect();
            let tracked = head_blobs(root, &files)?;
            let mut changed = vec![];
            for (_, _, _, bytes, _) in &prepared {
                for (file, content) in bytes {
                    if tracked.get(file) != Some(&blob_of(root, content)?) {
                        changed.push(file.clone());
                    }
                }
            }
            ensure!(
                changed.is_empty(),
                "files changed after they were found committed identically at HEAD ({}); nothing was removed. Commit or restore them and preview again",
                changed.join(", ")
            );
            write_entries(root)?;
            let head = head_revision(root);
            let at = now()?;
            let mut held: Vec<(String, Vec<u8>, fs::Permissions)> = vec![];
            let mut copies: Vec<(String, Vec<u8>, fs::Permissions)> = vec![];
            let mut recorded: Vec<String> = vec![];
            let removal: Result<()> = (|| {
                for (candidate, evidence, record_digest, bytes, moves) in &prepared {
                    let entry = if let Some(evidence) = evidence {
                        let mut entry = json!({
                        "schema_version": 1,
                        "id": candidate.id,
                        "cases": case_ids(evidence),
                        "environment": evidence.environment,
                        "outcome": evidence.outcome,
                        "kind": evidence.kind,
                        "revision": evidence.snapshot.revision,
                        "observed_at_unix_seconds": evidence.observed_at_unix_seconds,
                        "artifacts": evidence.artifact_fingerprints,
                        "record_digest": record_digest,
                        "removed": candidate.files,
                        "archives": candidate.archives,
                        "attributed": candidate.attributed.then_some("cycle time"),
                        "knowledge": entry_path(&candidate.archives[0]),
                        "reason": SUMMARY_REASON,
                        "policy": policy(Some(authority)),
                        "head": head,
                        "pruned_at_unix_seconds": at,
                        });
                        if !moves.is_empty() {
                            entry["moved"] = json!(moves);
                        }
                        entry
                    } else {
                        let mut digests = BTreeMap::new();
                        for (file, content) in bytes {
                            digests.insert(file.clone(), digest_bytes(content));
                        }
                        json!({
                            "schema_version": 1,
                            "kind": "loose-artifact",
                            "id": candidate.id,
                            "artifacts": digests,
                            "record_digest": record_digest,
                            "removed": candidate.files,
                            "moved": moves,
                            "archives": candidate.archives,
                            "attributed": candidate.attributed.then_some("cycle time"),
                            "knowledge": entry_path(&candidate.archives[0]),
                            "reason": LOOSE_REASON,
                            "policy": policy(Some(authority)),
                            "head": head,
                            "pruned_at_unix_seconds": at,
                        })
                    };
                    append_line(root, LEDGER, &serde_json::to_string(&entry)?)?;
                    recorded.push(candidate.id.clone());
                    // Artifacts first and the record last, so the set stays classifiable.
                    for (file, content) in bytes.iter().skip(1).chain(bytes.iter().take(1)) {
                        // Protections can change at any point in the apply (another agent
                        // updating the checkpoint, a bug link, an open change), so they are
                        // checked again right before every removal.
                        still_unprotected(root, archives, &native, candidate)?;
                        // Its permissions go with the bytes, to the vault copy and to a
                        // restore (KNOW-10, #109 review).
                        let permissions = fs::symlink_metadata(local_path(root, file)?)
                            .with_context(|| format!("reading the permissions of {file}"))?
                            .permissions();
                        // A key item is copied into the vault before its original goes. The
                        // copy is create-only: what is already there is never overwritten,
                        // and only a copy this apply created is removed by a restore.
                        if let Some(target) = moves.get(file) {
                            let path = local_path(root, target)?;
                            if let Some(parent) = path.parent() {
                                fs::create_dir_all(parent)?;
                            }
                            let placed = place_copy(&path, content, |staged| {
                                fs::set_permissions(staged, permissions.clone())
                            })
                            .with_context(|| format!("moving {file} to {target}"))?;
                            if placed == Placed::Created {
                                copies.push((target.clone(), content.clone(), permissions.clone()));
                            }
                        }
                        remove_verified(root, file, content)?;
                        held.push((file.clone(), content.clone(), permissions));
                    }
                }
                Ok(())
            })();
            let failure = match removal {
                Err(error) => Some(format!("removal failed partway ({error:#})")),
                Ok(()) => match validity(root, &affected) {
                    Err(error) => Some(format!(
                        "terminal evidence validity could not be re-read after removal ({error:#})"
                    )),
                    Ok(after) if after != before => Some(format!(
                        "removing archived proof changed terminal evidence validity ({})",
                        before
                            .iter()
                            .filter(|(id, state)| after.get(*id) != Some(*state))
                            .map(|(id, state)| format!(
                                "{id}: {state} -> {}",
                                after.get(id).map_or("missing", String::as_str)
                            ))
                            .collect::<Vec<_>>()
                            .join("; ")
                    )),
                    Ok(_) => None,
                },
            };
            if let Some(failure) = failure {
                // Restore with create-new: a file a writer recreated meanwhile is kept, and
                // the removed bytes stay recoverable from HEAD, where they were verified.
                let mut kept = vec![];
                for (file, bytes, permissions) in &held {
                    let path = local_path(root, file)?;
                    if let Some(parent) = path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    match fs::OpenOptions::new()
                        .write(true)
                        .create_new(true)
                        .open(&path)
                    {
                        Ok(mut target) => {
                            use std::io::Write;
                            target
                                .write_all(bytes)
                                .with_context(|| format!("restoring {file}"))?;
                            fs::set_permissions(&path, permissions.clone())
                                .with_context(|| format!("restoring {file}"))?;
                        }
                        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                            kept.push(file.clone());
                        }
                        Err(error) => {
                            return Err(error).with_context(|| format!("restoring {file}"));
                        }
                    }
                }
                // The vault copies of a refused apply are removed again, each only while it
                // still holds the bytes and mode this apply wrote: a copy another writer
                // changed meanwhile is kept as written (#110).
                let mut changed_copies = vec![];
                for (copy, content, permissions) in &copies {
                    if remove_verified_as(root, copy, content, Some(permissions)).is_err() {
                        changed_copies.push(copy.clone());
                    }
                }
                let failure = if kept.is_empty() {
                    failure
                } else {
                    format!(
                        "{failure}; not restored because a file was written there meanwhile, kept as written, removed bytes recoverable from Git HEAD: {}",
                        kept.join(", ")
                    )
                };
                let failure = if changed_copies.is_empty() {
                    failure
                } else {
                    format!(
                        "{failure}; vault copies another writer changed after this apply created them were kept as written: {}",
                        changed_copies.join(", ")
                    )
                };
                for id in &recorded {
                    append_line(
                        root,
                        LEDGER,
                        &serde_json::to_string(&json!({
                            "schema_version": 1,
                            "event": "restored",
                            "id": id,
                            "reason": failure,
                            "restored_at_unix_seconds": now()?,
                        }))?,
                    )?;
                }
                write_entries(root)?;
                bail!(
                    "{failure}; restored {} files and recorded the restore in {LEDGER}",
                    held.len()
                );
            }
            pruned = eligible
                .iter()
                .map(|candidate| candidate.id.clone())
                .collect();
            write_entries(root)?;
        }
    }
    let hot: Vec<&str> = archives
        .iter()
        .filter(|archive| archive.hot)
        .map(|archive| archive.id.as_str())
        .collect();
    let cold: Vec<&str> = archives
        .iter()
        .filter(|archive| !archive.hot)
        .map(|archive| archive.id.as_str())
        .collect();
    Ok(json!({
        "schema_version": 1,
        "applied": authority.is_some(),
        "authority": authority,
        "hot": hot,
        "cold": cold,
        "native": {"state": native.state, "detail": native.detail},
        "eligible": listed,
        "protected": protected,
        "uncommitted": uncommitted,
        "stale_holds": stale_holds(root)?,
        "totals": {
            "eligible": listed.len(),
            "protected": protected.len(),
            "uncommitted": uncommitted.len(),
            "reclaimable_bytes": reclaimable,
        },
        "pruned": pruned,
        "ledger": LEDGER,
        "note": "Preview by default. Applying needs retention authority and no open native change; it keeps one knowledge entry per archive, removes only files committed identically at HEAD (recoverable from Git), and never edits native archives or Git history.",
    }))
}

/// The retention authority for an apply, or the refusal a human must resolve.
pub(super) fn retention_authority(root: &Path, command: &str) -> Result<String> {
    let session = read_session(root)?;
    let authorized = match &session {
        Session::V2(checkpoint) => prefs::authorized(root, checkpoint, "retention")?,
        Session::V1(_) => {
            let (value, origin) = prefs::effective(root, "retention")?;
            (value == "auto").then_some(origin)
        }
    };
    match authorized {
        Some(authority) => Ok(authority),
        None => {
            let (value, origin) = prefs::effective(root, "retention")?;
            bail!(
                "{command} is not authorized (retention = {value}, {origin}). Preview first, then ask the human; record their answer with `chough consent grant retention --source <their words>`"
            )
        }
    }
}

fn remove_if_present(root: &Path, relative: &str) -> Result<()> {
    match fs::remove_file(local_path(root, relative)?) {
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
            Err(error).with_context(|| format!("removing {relative}"))
        }
        _ => Ok(()),
    }
}

/// `chough evidence prune [--apply]`: preview by default.
pub(super) fn prune(root: &Path, apply: bool) -> Result<()> {
    let session = read_session(root)?;
    let authority = if apply {
        let authorized = match &session {
            Session::V2(checkpoint) => prefs::authorized(root, checkpoint, "retention")?,
            Session::V1(_) => {
                let (value, origin) = prefs::effective(root, "retention")?;
                (value == "auto").then_some(origin)
            }
        };
        if authorized.is_none() {
            let (value, origin) = prefs::effective(root, "retention")?;
            bail!(
                "pruning evidence is not authorized (retention = {value}, {origin}). Preview with `chough evidence prune`, then ask the human; record their answer with `chough consent grant retention --source <their words>` for this objective, or `chough pref set retention auto --source <...>` only if they asked for retention to apply automatically"
            );
        }
        authorized
    } else {
        None
    };
    print_json(&retention(root, authority.as_deref())?)
}

/// After `cases run` and `validate run`: prune with the standing preference
/// `retention auto`, else nothing. Failures are reported, never hidden.
pub(super) fn auto_prune(root: &Path) -> Value {
    let result = (|| -> Result<Value> {
        let (value, origin) = prefs::effective(root, "retention")?;
        if value != "auto" {
            return Ok(Value::Null);
        }
        let report = retention(root, Some(&origin))?;
        Ok(json!({
            "authority": report["authority"],
            "pruned": report["pruned"],
            "totals": report["totals"],
            "native": report["native"],
            "warning": report["warning"],
            "ledger": LEDGER,
        }))
    })();
    result
        .unwrap_or_else(|error| json!({"error": format!("automatic retention failed: {error:#}")}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_hold_never_sits_in_the_removed_file_directory() {
        let records = [
            ".chough/evidence/a.json",
            ".chough/artifacts/x.log",
            ".chough/artifacts/nested/deep/y.log",
            ".chough/library/snapshots/SNAP-a/manifest.json",
        ];
        let mut seen = std::collections::BTreeSet::new();
        for record in records {
            let hold = hold_path(record);
            let hold = Path::new(&hold);
            assert_eq!(hold.parent(), Some(Path::new(HOLD_DIR)), "{record}");
            assert_ne!(hold.parent(), Path::new(record).parent(), "{record}");
            let name = Path::new(record).file_name().unwrap().to_str().unwrap();
            assert!(
                !hold.to_str().unwrap().contains(name),
                "{record}: {}",
                hold.display()
            );
            assert!(seen.insert(hold.to_path_buf()), "{record}");
        }
    }

    #[test]
    fn a_failed_copy_never_deletes_another_writers_bytes() {
        // #115 review P1: another writer replaces the attachment just before setting the
        // copy's mode fails; the refusal must keep that writer's bytes.
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("attachments/a2/evidence/manual.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let error = place_copy(&path, b"ours\n", |_| {
            fs::write(&path, "another writer\n")?;
            Err(std::io::Error::other("set_permissions failed"))
        })
        .unwrap_err();
        assert!(
            format!("{error:#}").contains("set_permissions failed"),
            "{error:#}"
        );
        assert_eq!(fs::read_to_string(&path).unwrap(), "another writer\n");
        let left: Vec<_> = fs::read_dir(path.parent().unwrap()).unwrap().collect();
        assert_eq!(left.len(), 1, "no staged copy is left behind");

        // A failure before anyone else writes leaves nothing behind.
        let other = root.path().join("attachments/a2/evidence/other.json");
        place_copy(&other, b"ours\n", |_| {
            Err(std::io::Error::other("set_permissions failed"))
        })
        .unwrap_err();
        assert!(!other.exists());
        assert_eq!(fs::read_dir(other.parent().unwrap()).unwrap().count(), 1);

        // A finished copy is created; identical bytes already there are not ours.
        assert_eq!(
            place_copy(&other, b"ours\n", |_| Ok(())).unwrap(),
            Placed::Created
        );
        assert_eq!(fs::read_to_string(&other).unwrap(), "ours\n");
        assert_eq!(
            place_copy(&other, b"ours\n", |_| Ok(())).unwrap(),
            Placed::AlreadyThere
        );
        assert!(place_copy(&other, b"theirs\n", |_| Ok(())).is_err());
        assert_eq!(fs::read_to_string(&other).unwrap(), "ours\n");
    }

    #[test]
    fn remove_verified_puts_changed_bytes_back_and_leaves_no_hold() {
        let root = tempfile::tempdir().unwrap();
        fs::create_dir_all(root.path().join(EVIDENCE_DIR)).unwrap();
        let file = format!("{EVIDENCE_DIR}/a.json");
        fs::write(root.path().join(&file), "changed\n").unwrap();
        let error = remove_verified(root.path(), &file, b"verified\n").unwrap_err();
        assert!(error.to_string().contains("not removed"), "{error:#}");
        assert_eq!(
            fs::read_to_string(root.path().join(&file)).unwrap(),
            "changed\n"
        );
        assert!(!root.path().join(HOLD_DIR).exists());
        remove_verified(root.path(), &file, b"changed\n").unwrap();
        assert!(!root.path().join(&file).exists());
        assert!(!root.path().join(HOLD_DIR).exists());
    }

    #[test]
    fn put_back_never_replaces_a_file_written_meanwhile() {
        let root = tempfile::tempdir().unwrap();
        let hold = root.path().join("hold");
        let path = root.path().join("path");
        fs::write(&hold, "held\n").unwrap();
        fs::write(&path, "written meanwhile\n").unwrap();
        let error = put_back(&hold, &path).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::AlreadyExists);
        assert_eq!(fs::read_to_string(&path).unwrap(), "written meanwhile\n");
        assert_eq!(fs::read_to_string(&hold).unwrap(), "held\n");
        fs::remove_file(&path).unwrap();
        put_back(&hold, &path).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), "held\n");
        assert!(!hold.exists());
    }

    fn observation(id: String) -> Evidence {
        Evidence {
            schema_version: 2,
            id,
            criteria: vec!["LIFE-1".into()],
            requirements: vec!["REQ-cli-001".into()],
            contracts: vec!["specs/cli/cli.spec.md".into()],
            kind: Kind::ProductTest,
            outcome: Outcome::Passed,
            environment: "local".into(),
            summary: "original".into(),
            artifacts: vec![],
            artifact_fingerprints: BTreeMap::new(),
            observed_at_unix_seconds: 1,
            snapshot: Snapshot {
                revision: "0".repeat(40),
                dirty: false,
                fingerprint: "0".repeat(64),
                content: Some("0".repeat(64)),
            },
            provenance: PROVENANCE.into(),
            cases: vec![],
            method: None,
            track: None,
            argv: vec![],
            exit_code: None,
            tests_run: None,
            limitations: vec![],
            duration_seconds: None,
        }
    }

    #[test]
    fn a_forced_observation_collision_preserves_bytes_without_another_random_draw() {
        let root = tempfile::tempdir().unwrap();
        identity::with_random_results(vec![Ok([7; 12])], || {
            let id = identity::observation("TC-cli-0001").unwrap();
            let mut evidence = observation(id.clone());
            record(root.path(), &evidence).unwrap();
            let name = format!(".chough/evidence/{id}.json");
            let path = root.path().join(name);
            let original = fs::read(&path).unwrap();
            evidence.summary = "different content".into();
            let error = record(root.path(), &evidence).unwrap_err();
            assert!(
                error.to_string().contains("identity collision"),
                "{error:#}"
            );
            assert!(error.to_string().contains(&id), "{error:#}");
            assert_eq!(fs::read(path).unwrap(), original);
            assert_eq!(identity::random_draws(), 1);
            assert_eq!(record_names(root.path(), EVIDENCE_DIR).unwrap().len(), 1);
        });
    }

    #[test]
    fn a_random_source_failure_preserves_the_existing_record_tree() {
        let root = tempfile::tempdir().unwrap();
        let directory = root.path().join(EVIDENCE_DIR);
        fs::create_dir_all(&directory).unwrap();
        let existing = directory.join("legacy.json");
        fs::write(&existing, b"existing bytes\n").unwrap();
        let error = identity::with_random_results(vec![Err("injected random failure")], || {
            identity::observation("TC-cli-0001").unwrap_err()
        });
        assert!(error.to_string().contains("injected random failure"));
        assert_eq!(identity::random_draws(), 1);
        assert_eq!(fs::read(existing).unwrap(), b"existing bytes\n");
        assert_eq!(fs::read_dir(directory).unwrap().count(), 1);
    }
}
