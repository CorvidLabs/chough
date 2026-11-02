//! Observations bound to Git content, environment and, for cases, definition digests.

use crate::cases::{self, ARTIFACTS_DIR};
use crate::checkpoint::{Session, read_session};
use crate::context::hi_export;
use crate::identity;
use crate::records::{
    append_line, bounded, file_digest, head_revision, is_digest, local_path, native_json, now,
    print_json, read_record, record_names, valid_name, write_new,
};
use crate::{bugs, prefs, validate};
use anyhow::{Context, Result, bail, ensure};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path};
use std::process::{Command, Output};

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
    let output = Command::new("git")
        .args(arguments)
        .current_dir(root)
        .output()
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
    let dirty_output = git(root, &["status", "--porcelain", "--untracked-files=all"])?;
    let dirty = String::from_utf8(dirty_output.stdout)?.lines().any(|line| {
        line.get(3..)
            .is_some_and(|name| fingerprint_key(name) != Key::Skip)
    });
    Ok(Snapshot {
        revision,
        dirty,
        fingerprint: format!("{:x}", legacy.finalize()),
        content: Some(format!("{:x}", content.finalize())),
    })
}

/// Whether `ancestor` is an ancestor of (or equal to) the current HEAD.
pub(super) fn is_ancestor_of_head(root: &Path, ancestor: &str) -> bool {
    Command::new("git")
        .args(["merge-base", "--is-ancestor", ancestor, "HEAD"])
        .current_dir(root)
        .output()
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
            let export = hi_export(root, family);
            ensure!(
                export.state == "observed",
                "cannot validate criteria: {}",
                export.content
            );
            let value: Value = serde_json::from_str(&export.content)
                .context("native HI family export is invalid or exceeds 16384 bytes")?;
            exports.insert(family.to_string(), value);
        }
        ensure!(
            find_id(&exports[family], criterion),
            "unknown or retired criterion: {criterion}"
        );
    }
    Ok(())
}

/// Whether `text` holds `id` as a whole token of ID characters.
fn holds_token(text: &str, id: &str) -> bool {
    text.split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
        .any(|part| part == id)
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
/// module name whose bounded `requirements.md` is not among those searched and holds
/// the ID as a token. Reads neither the checkpoint nor native deltas.
fn owning_module(root: &Path, id: &str, searched: &[String]) -> Option<String> {
    let (module, _) = id.strip_prefix("REQ-")?.rsplit_once('-')?;
    let companion = format!("specs/{module}/requirements.md");
    if !valid_name(module) || searched.contains(&companion) {
        return None;
    }
    let path = local_path(root, &companion).ok()?;
    let text = read_record(&path).ok()?;
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
            &read_record(&local_path(root, companion)?)
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

fn validate_record(evidence: &Evidence, name: &str) -> Result<()> {
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
            valid_name(&case.id) && is_digest(&case.digest),
            "invalid case binding: {name}"
        );
    }
    Ok(())
}

pub(super) fn load_all(root: &Path) -> Result<Vec<Evidence>> {
    let mut records = vec![];
    for name in record_names(root, EVIDENCE_DIR)? {
        let path = local_path(root, &format!("{EVIDENCE_DIR}/{name}"))?;
        let evidence: Evidence = serde_json::from_str(&read_record(&path)?)
            .with_context(|| format!("invalid evidence record: {name}"))?;
        validate_record(&evidence, &name)?;
        records.push(evidence);
    }
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

pub(super) fn freshness(
    root: &Path,
    evidence: &Evidence,
    current: &Result<Snapshot>,
    environment: Option<&str>,
) -> Result<&'static str> {
    let mut artifacts_current = true;
    for artifact in &evidence.artifacts {
        local_path(root, artifact)?;
        let expected = evidence
            .artifact_fingerprints
            .get(artifact)
            .context("missing artifact binding")?;
        ensure!(is_digest(expected), "invalid artifact digest");
        if file_digest(root, artifact).as_ref().ok() != Some(expected) {
            artifacts_current = false;
        }
    }
    let mut cases_current = true;
    for case in &evidence.cases {
        if cases::current_digest(root, &case.id)?.as_deref() != Some(case.digest.as_str()) {
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

/// Observations with freshness, for status and next.
pub(super) fn observations(
    root: &Path,
    current: &Result<Snapshot>,
    environment: Option<&str>,
) -> Result<Vec<Value>> {
    let mut values = vec![];
    for evidence in load_all(root)? {
        let state = freshness(root, &evidence, current, environment)?;
        values.push(
            json!({"record": evidence, "freshness": state, "label": label_for(state, environment)}),
        );
    }
    Ok(values)
}

// ---------------------------------------------------------------------------
// Observation labels and reference-aware retention (REQ-cli-018).

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
        ids.insert(
            entry["id"]
                .as_str()
                .context("retention ledger line without an id")?
                .to_string(),
        );
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
pub(super) struct Labels {
    snapshot: Result<Snapshot>,
    pruned: BTreeSet<String>,
}

impl Labels {
    pub(super) fn new(root: &Path) -> Self {
        Self {
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
        label_reference(root, id, &self.snapshot, environment, &self.pruned)
    }
}

fn label_reference(
    root: &Path,
    id: &str,
    current: &Result<Snapshot>,
    environment: Option<&str>,
    pruned: &BTreeSet<String>,
) -> Result<&'static str> {
    if !is_present(root, id)? {
        return Ok(if pruned.contains(id) {
            "pruned"
        } else {
            "unavailable"
        });
    }
    let evidence = load(root, id)?;
    Ok(label_for(
        freshness(root, &evidence, current, environment)?,
        environment,
    ))
}

/// Files named directly, keyed by [`file_key`], with what names them: bug fixtures of
/// any status and the checkpoint's design and file references.
fn file_references(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    let mut map: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut add = |path: &str, source: String| {
        let sources = map.entry(file_key(path)).or_default();
        if !sources.contains(&source) {
            sources.push(source);
        }
    };
    if let Session::V2(checkpoint) = read_session(root)? {
        for path in checkpoint
            .references
            .design
            .iter()
            .chain(&checkpoint.references.files)
        {
            add(path, "checkpoint".into());
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
    if let Session::V2(checkpoint) = read_session(root)? {
        for id in &checkpoint.observations {
            add(id, "checkpoint".into());
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

/// Referenced observations with their labels, for status and next.
pub(super) fn reference_labels(
    root: &Path,
    current: &Result<Snapshot>,
    environment: Option<&str>,
) -> Result<Vec<Value>> {
    let pruned = read_ledger(root).unwrap_or_default();
    let mut values = vec![];
    for (id, sources) in references(root)? {
        let label = label_reference(root, &id, current, environment, &pruned)?;
        values.push(json!({"id": id, "label": label, "referenced_by": sources}));
    }
    Ok(values)
}

/// Native manifests that protect observation sets, or why they could not be read.
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
        let text = local_path(root, &name)
            .and_then(|path| read_record(&path))
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
}

fn classify(root: &Path, at: u64) -> Result<Classification> {
    let records = load_all(root)?;
    let references = references(root)?;
    let named_files = file_references(root)?;
    let native = native_manifests(root)?;
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
    let mut last_failure = BTreeSet::new();
    for group in groups.values_mut() {
        group.sort_by(|left, right| {
            (right.observed_at_unix_seconds, &right.id)
                .cmp(&(left.observed_at_unix_seconds, &left.id))
        });
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
            for source in named_files.get(key).into_iter().flatten() {
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
                || named_files.contains_key(&key)
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
    if let Some(authority) = authority {
        let head = head_revision(root);
        for set in classification
            .sets
            .iter()
            .filter(|set| set.reasons.is_empty())
        {
            let evidence = &set.evidence;
            let record_path = format!("{EVIDENCE_DIR}/{}.json", evidence.id);
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
                "record_digest": file_digest(root, &record_path)?,
                "removed": removed,
                "reason": ELIGIBLE_REASON,
                "policy": policy(Some(authority)),
                "head": head,
                "pruned_at_unix_seconds": at,
            });
            // The ledger line comes first, so a removal is never unexplained.
            append_line(root, LEDGER, &serde_json::to_string(&entry)?)?;
            for artifact in &set.removable {
                remove_if_present(root, artifact)?;
            }
            remove_if_present(root, &record_path)?;
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
        "ledger": LEDGER,
        "note": "Files are removed from the working tree only; committed bytes stay recoverable from Git history. Native archives, ignore rules and history are never changed.",
    }))
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
