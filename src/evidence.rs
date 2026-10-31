//! Observations bound to Git content, environment and, for cases, definition digests.

use crate::cases;
use crate::checkpoint::read_session;
use crate::context::hi_export;
use crate::records::{
    bounded, file_digest, is_digest, local_path, now, print_json, read_record, record_names,
    valid_name, write_new,
};
use anyhow::{Context, Result, bail, ensure};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;
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
    pub(super) revision: String,
    pub(super) dirty: bool,
    pub(super) fingerprint: String,
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
}

#[derive(clap::Args)]
pub(super) struct EvidenceArgs {
    id: String,
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
    let mut hash = Sha256::new();
    hash.update(revision.as_bytes());
    for name in files {
        hash.update((name.len() as u64).to_le_bytes());
        hash.update(name.as_bytes());
        let path = root.join(name);
        if let Some(parent) = Path::new(name)
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
        {
            local_path(root, parent.to_str().context("non-UTF-8 source parent")?)?;
        }
        match fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                hash.update(b"symlink");
                hash.update(fs::read_link(&path)?.as_os_str().as_encoded_bytes());
            }
            Ok(metadata) if metadata.is_file() => {
                hash.update(b"file");
                hash.update([u8::from(metadata.permissions().readonly())]);
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    hash.update((metadata.permissions().mode() & 0o111).to_le_bytes());
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
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => hash.update(b"missing"),
            Ok(_) => bail!("unsupported Git input: {name}"),
            Err(error) => return Err(error.into()),
        }
    }
    let dirty_output = git(root, &["status", "--porcelain", "--untracked-files=all"])?;
    let dirty = String::from_utf8(dirty_output.stdout)?.lines().any(|line| {
        line.get(3..)
            .is_some_and(|name| !name.starts_with(".chough/") && !name.starts_with("target/"))
    });
    Ok(Snapshot {
        revision,
        dirty,
        fingerprint: format!("{:x}", hash.finalize()),
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

/// Confirm each requirement ID appears in the contracts' native requirements companions.
pub(super) fn validate_requirements(
    root: &Path,
    contracts: &[String],
    requirements: &[String],
) -> Result<()> {
    let mut text = String::new();
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
        let companion = local_path(
            root,
            companion.to_str().context("non-UTF-8 contract companion")?,
        )?;
        text.push_str(
            &read_record(&companion)
                .context("contract requires its native requirements.md companion")?,
        );
        text.push('\n');
    }
    for requirement in requirements {
        ensure!(
            requirement.starts_with("REQ-")
                && text
                    .split(|c: char| !(c.is_ascii_alphanumeric() || c == '-'))
                    .any(|part| part == requirement),
            "requirement not found in contract companions: {requirement}"
        );
    }
    Ok(())
}

/// Record a new observation; never replaces an existing ID.
pub(super) fn record(root: &Path, evidence: &Evidence) -> Result<String> {
    validate_record(evidence, &format!("{}.json", evidence.id))?;
    let name = format!("{EVIDENCE_DIR}/{}.json", evidence.id);
    ensure!(
        !local_path(root, &name)?.exists(),
        "evidence ID already exists; record a new observation"
    );
    ensure!(
        write_new(
            root,
            &name,
            &(serde_json::to_string_pretty(evidence)? + "\n")
        )?,
        "evidence ID already exists; record a new observation"
    );
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
    ensure!(valid_name(&args.id), "invalid evidence ID");
    ensure!(
        !args.environment.trim().is_empty() && !args.summary.trim().is_empty(),
        "environment and summary must not be blank"
    );
    validate_criteria(root, &args.criteria)?;
    validate_requirements(root, &args.contracts, &args.requirements)?;
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
    let evidence = Evidence {
        schema_version: 2,
        id: args.id.clone(),
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
                && evidence.limitations.is_empty(),
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
        Ok(snapshot) if snapshot.fingerprint != evidence.snapshot.fingerprint => {
            "stale-revision-or-inputs"
        }
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
        values.push(json!({"record": evidence, "freshness": state}));
    }
    Ok(values)
}
