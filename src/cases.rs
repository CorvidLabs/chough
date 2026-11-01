//! Test-case definitions (`chough-testing/1`), focused selection and explicit execution.

use crate::evidence::{self, CaseRef, Evidence, Kind, Outcome, PROVENANCE};
use crate::records::{
    Captured, Limits, digest_bytes, local_path, now, print_json, run_bounded, valid_name,
    write_new_bytes,
};
use crate::{checkpoint, prefs};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::Read;
use std::path::{Component, Path};
use std::time::{Duration, Instant};

pub(super) const CASES_DIR: &str = ".chough/cases";
pub(super) const ARTIFACTS_DIR: &str = ".chough/artifacts";
const PROFILE: &str = "chough-testing/1";
const FILE_LIMIT: u64 = 262_144;
const OUTPUT_LIMIT: usize = 1_048_576;
const ENV_ENTRIES: usize = 32;
const ENV_NAME_LIMIT: usize = 128;
const ENV_VALUE_LIMIT: usize = 4_096;
/// Variables that choose what `cargo test` builds or runs, refused for cargo-test in
/// any ASCII case, besides the names under `CARGO_REFUSED_PREFIXES`.
const CARGO_REFUSED: [&str; 12] = [
    "PATH",
    "CARGO",
    "CARGO_HOME",
    "RUSTC",
    "RUSTDOC",
    "RUSTC_WRAPPER",
    "RUSTC_WORKSPACE_WRAPPER",
    "RUSTFLAGS",
    "CARGO_ENCODED_RUSTFLAGS",
    "RUSTDOCFLAGS",
    "CARGO_ENCODED_RUSTDOCFLAGS",
    "RUSTC_BOOTSTRAP",
];
/// Prefixes of names that choose the toolchain, build settings, per-target runner,
/// linker or flags (which can carry `-C linker=...`), or unstable cargo behavior.
/// `CARGO_TARGET_DIR` only chooses where cargo writes and stays allowed.
const CARGO_REFUSED_PREFIXES: [&str; 6] = [
    "RUSTUP_",
    "CARGO_BUILD_",
    "CARGO_TARGET_",
    "CARGO_HOST_",
    "CARGO_PROFILE_",
    "CARGO_UNSTABLE_",
];
const INHERITED_ENVIRONMENT: &str = "inherited environment variables were not recorded";
const ESCAPES_REMOVED: &str =
    "terminal escape sequences were removed before matching; the artifact keeps the raw output";
const ESC: u8 = 0x1b;
const TRACKS: [&str; 3] = ["functional", "copy-localization", "design-qa"];
const METHODS: [&str; 3] = ["automated", "manual", "agent-assisted"];
const ARTIFACT_TYPES: [&str; 6] = [
    "test-log",
    "report-json",
    "screenshot",
    "recording",
    "transcript",
    "reviewer-note",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct CaseFile {
    profile: String,
    module: String,
    #[serde(default, rename = "case")]
    cases: Vec<Case>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Case {
    pub(super) id: String,
    pub(super) title: String,
    pub(super) status: String,
    pub(super) track: String,
    pub(super) method: String,
    pub(super) criteria: Vec<String>,
    pub(super) requirements: Vec<String>,
    #[serde(default)]
    pub(super) paths: Vec<String>,
    pub(super) environments: Vec<String>,
    pub(super) artifacts: Vec<String>,
    #[serde(default)]
    pub(super) setup: Vec<String>,
    #[serde(default)]
    pub(super) inputs: BTreeMap<String, String>,
    #[serde(default)]
    pub(super) steps: Vec<String>,
    pub(super) expected: Vec<String>,
    #[serde(default)]
    pub(super) notes: Option<String>,
    #[serde(default)]
    pub(super) bind: Option<Binding>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Binding {
    pub(super) adapter: String,
    #[serde(default)]
    pub(super) test_target: Option<String>,
    #[serde(default)]
    pub(super) test: Option<String>,
    #[serde(default)]
    pub(super) task: Option<String>,
    #[serde(default)]
    pub(super) argv: Vec<String>,
    #[serde(default)]
    pub(super) expect_output: Vec<String>,
    #[serde(default)]
    pub(super) timeout_seconds: Option<u64>,
    /// Variables set for the executed command on top of the inherited environment.
    /// Omitted when empty, so definitions without it keep their digests.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub(super) env: BTreeMap<String, String>,
}

/// A parsed case with where it came from.
pub(super) struct Located {
    pub(super) module: String,
    pub(super) file: String,
    pub(super) case: Case,
}

impl Located {
    pub(super) fn digest(&self) -> Result<String> {
        let mut identity = self.case.clone();
        identity.title = String::new();
        identity.notes = None;
        Ok(digest_bytes(&serde_json::to_vec(&identity)?))
    }

    fn contract(&self) -> String {
        format!("specs/{0}/{0}.spec.md", self.module)
    }
}

#[derive(Subcommand)]
pub(super) enum CasesCommand {
    /// Validate every definition, including links to HI and native requirements.
    Check,
    /// List definitions with their digests.
    List,
    /// Select the cases affected by criteria, requirements or paths.
    Select {
        #[arg(long = "criterion")]
        criteria: Vec<String>,
        #[arg(long = "requirement")]
        requirements: Vec<String>,
        #[arg(long = "path")]
        paths: Vec<String>,
        /// Add a relevant optional track: copy-localization or design-qa.
        #[arg(long = "track", value_parser = ["copy-localization", "design-qa"])]
        tracks: Vec<String>,
    },
    /// Execute an automated case's binding and record the observation.
    Run {
        id: String,
        #[arg(long)]
        environment: String,
        #[arg(long)]
        observation_id: Option<String>,
    },
    /// Record a manual or agent-assisted execution with a verdict for every expected outcome.
    Record(RecordArgs),
}

#[derive(clap::Args)]
pub(super) struct RecordArgs {
    id: String,
    #[arg(long)]
    environment: String,
    #[arg(long, value_parser = ["manual", "agent-assisted"])]
    method: String,
    /// INDEX=pass|fail for each expected outcome, counting from 1.
    #[arg(long = "verdict", required = true)]
    verdicts: Vec<String>,
    #[arg(long)]
    summary: String,
    #[arg(long = "artifact")]
    artifacts: Vec<String>,
    #[arg(long = "limitation")]
    limitations: Vec<String>,
    #[arg(long)]
    observation_id: Option<String>,
}

fn read_bounded(path: &Path) -> Result<String> {
    let file = File::open(path)?;
    let mut content = String::new();
    file.take(FILE_LIMIT + 1).read_to_string(&mut content)?;
    ensure!(
        content.len() as u64 <= FILE_LIMIT,
        "case file exceeds {FILE_LIMIT} bytes: {}",
        path.display()
    );
    Ok(content)
}

/// Parse every definition file structurally, without consulting native tools.
pub(super) fn load(root: &Path) -> Result<Vec<Located>> {
    let directory = local_path(root, CASES_DIR)?;
    if !directory.exists() {
        return Ok(vec![]);
    }
    let mut names = vec![];
    for entry in fs::read_dir(directory)? {
        let name = entry?
            .file_name()
            .to_str()
            .context("invalid case filename")?
            .to_string();
        ensure!(
            name.ends_with(".cases.toml"),
            "unexpected file in {CASES_DIR}: {name}"
        );
        names.push(name);
    }
    names.sort();
    let mut located = vec![];
    let mut ids = BTreeSet::new();
    for name in names {
        let file = format!("{CASES_DIR}/{name}");
        let parsed: CaseFile = toml::from_str(&read_bounded(&local_path(root, &file)?)?)
            .with_context(|| format!("{file}: invalid chough-testing definition"))?;
        ensure!(
            parsed.profile == PROFILE,
            "{file}: unsupported profile {}; expected {PROFILE}",
            parsed.profile
        );
        let stem = name.trim_end_matches(".cases.toml");
        ensure!(
            parsed.module == stem && valid_name(stem),
            "{file}: module must equal the file stem"
        );
        for case in parsed.cases {
            validate_shape(&parsed.module, &case)
                .with_context(|| format!("{file}: case {}", case.id))?;
            ensure!(
                ids.insert(case.id.clone()),
                "{file}: duplicate case ID {}",
                case.id
            );
            located.push(Located {
                module: parsed.module.clone(),
                file: file.clone(),
                case,
            });
        }
    }
    Ok(located)
}

fn non_blank(label: &str, values: &[String]) -> Result<()> {
    ensure!(!values.is_empty(), "{label} must not be empty");
    ensure!(
        values.iter().all(|value| !value.trim().is_empty()),
        "{label} entries must not be blank"
    );
    Ok(())
}

fn validate_shape(module: &str, case: &Case) -> Result<()> {
    let prefix = format!("TC-{module}-");
    ensure!(
        case.id.strip_prefix(&prefix).is_some_and(
            |number| number.len() == 4 && number.bytes().all(|byte| byte.is_ascii_digit())
        ),
        "id must look like {prefix}0001"
    );
    ensure!(!case.title.trim().is_empty(), "title must not be blank");
    ensure!(
        matches!(case.status.as_str(), "active" | "retired"),
        "status must be active or retired"
    );
    ensure!(
        TRACKS.contains(&case.track.as_str()),
        "track must be one of {}",
        TRACKS.join(", ")
    );
    ensure!(
        METHODS.contains(&case.method.as_str()),
        "method must be one of {}",
        METHODS.join(", ")
    );
    non_blank("criteria", &case.criteria)?;
    non_blank("requirements", &case.requirements)?;
    non_blank("environments", &case.environments)?;
    non_blank("expected", &case.expected)?;
    ensure!(!case.artifacts.is_empty(), "artifacts must not be empty");
    for artifact in &case.artifacts {
        ensure!(
            ARTIFACT_TYPES.contains(&artifact.as_str()),
            "artifact type {artifact} is not one of {}",
            ARTIFACT_TYPES.join(", ")
        );
    }
    for path in &case.paths {
        ensure!(
            !path.is_empty()
                && Path::new(path)
                    .components()
                    .all(|part| matches!(part, Component::Normal(_))),
            "paths must be project-relative without traversal: {path}"
        );
    }
    if case.method != "automated" {
        non_blank("setup", &case.setup)?;
        non_blank("steps", &case.steps)?;
    }
    match (&case.method[..], &case.bind) {
        ("automated", None) => bail!("an automated case needs a [case.bind] binding"),
        ("manual", Some(_)) => bail!("a manual case cannot have a binding"),
        (_, Some(binding)) => validate_binding(binding)?,
        _ => {}
    }
    Ok(())
}

/// Whether `name` matches `[A-Za-z_][A-Za-z0-9_]*`.
fn valid_env_name(name: &str) -> bool {
    let mut bytes = name.bytes();
    bytes
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == b'_')
        && bytes.all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
}

/// Whether a cargo-test binding may not declare `name`, because it chooses what cargo
/// builds or runs.
fn chooses_what_cargo_runs(name: &str) -> bool {
    let upper = name.to_ascii_uppercase();
    CARGO_REFUSED.contains(&upper.as_str())
        || (upper != "CARGO_TARGET_DIR"
            && CARGO_REFUSED_PREFIXES
                .iter()
                .any(|prefix| upper.starts_with(prefix)))
}

fn validate_env(binding: &Binding) -> Result<()> {
    ensure!(
        binding.env.len() <= ENV_ENTRIES,
        "env holds at most {ENV_ENTRIES} entries; {} are declared",
        binding.env.len()
    );
    let mut folded = BTreeMap::new();
    for (name, value) in &binding.env {
        ensure!(
            valid_env_name(name),
            "env name {name} must match [A-Za-z_][A-Za-z0-9_]*"
        );
        ensure!(
            name.len() <= ENV_NAME_LIMIT,
            "env name {name} exceeds {ENV_NAME_LIMIT} bytes"
        );
        ensure!(!value.contains('\0'), "env value of {name} contains NUL");
        ensure!(
            value.len() <= ENV_VALUE_LIMIT,
            "env value of {name} exceeds {ENV_VALUE_LIMIT} bytes"
        );
        if let Some(other) = folded.insert(name.to_ascii_uppercase(), name) {
            bail!(
                "env names {other} and {name} are equal ignoring ASCII case, and Windows treats them as one variable"
            );
        }
        ensure!(
            binding.adapter != "cargo-test" || !chooses_what_cargo_runs(name),
            "env name {name} chooses what cargo builds or runs, so a cargo-test binding cannot declare it"
        );
    }
    Ok(())
}

fn validate_binding(binding: &Binding) -> Result<()> {
    if let Some(timeout) = binding.timeout_seconds {
        ensure!(
            (1..=3_600).contains(&timeout),
            "timeout_seconds must be between 1 and 3600"
        );
    }
    ensure!(
        binding
            .expect_output
            .iter()
            .all(|fragment| !fragment.is_empty()),
        "expect_output fragments must not be empty"
    );
    if let Some(fragment) = binding
        .expect_output
        .iter()
        .find(|fragment| fragment.contains('\u{1b}'))
    {
        bail!(
            "expect_output fragment {fragment:?} contains ESC (U+001B); output is matched with terminal escape sequences removed, so it could never match"
        );
    }
    validate_env(binding)?;
    match binding.adapter.as_str() {
        "cargo-test" => {
            ensure!(
                binding.test_target.as_deref().is_some_and(valid_name),
                "cargo-test needs test_target"
            );
            ensure!(
                binding.test.as_deref().is_some_and(|test| !test.is_empty()
                    && test
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b':'))),
                "cargo-test needs test as a Rust test path"
            );
            ensure!(
                binding.task.is_none() && binding.argv.is_empty(),
                "cargo-test uses only test_target, test, expect_output, env and timeout_seconds"
            );
        }
        "fledge-task" => {
            ensure!(
                binding.task.as_deref().is_some_and(valid_name),
                "fledge-task needs task"
            );
            ensure!(
                binding.test_target.is_none() && binding.test.is_none() && binding.argv.is_empty(),
                "fledge-task uses only task, expect_output, env and timeout_seconds"
            );
            ensure!(
                !binding.expect_output.is_empty(),
                "fledge-task needs at least one expect_output fragment; an exit code alone is not an assertion"
            );
        }
        "command" => {
            non_blank("argv", &binding.argv)?;
            ensure!(
                binding.test_target.is_none() && binding.test.is_none() && binding.task.is_none(),
                "command uses only argv, expect_output, env and timeout_seconds"
            );
            ensure!(
                !binding.expect_output.is_empty(),
                "command needs at least one expect_output fragment; an exit code alone is not an assertion"
            );
        }
        other => bail!("unknown adapter {other}; use cargo-test, fledge-task or command"),
    }
    Ok(())
}

fn fledge_tasks(root: &Path) -> Result<BTreeSet<String>> {
    let path = local_path(root, "fledge.toml")?;
    if !path.is_file() {
        return Ok(BTreeSet::new());
    }
    let value: toml::Value =
        toml::from_str(&read_bounded(&path)?).context("fledge.toml is not valid TOML")?;
    Ok(value
        .get("tasks")
        .and_then(toml::Value::as_table)
        .map(|table| table.keys().cloned().collect())
        .unwrap_or_default())
}

/// Full validation, including native HI and requirement links for active cases.
pub(super) fn check(root: &Path) -> Result<Vec<Located>> {
    let cases = load(root)?;
    let tasks = fledge_tasks(root)?;
    for located in &cases {
        check_links(root, located, &tasks)?;
    }
    Ok(cases)
}

/// Validate a case's Fledge task, HI criteria and native requirements.
fn check_links(root: &Path, located: &Located, tasks: &BTreeSet<String>) -> Result<()> {
    let case = &located.case;
    let context = || format!("{}: case {}", located.file, case.id);
    if let Some(binding) = &case.bind
        && let Some(task) = &binding.task
    {
        ensure!(
            tasks.contains(task),
            "{}: fledge task {task} is not defined in fledge.toml",
            context()
        );
    }
    if case.status == "retired" {
        return Ok(());
    }
    evidence::validate_criteria(root, &case.criteria).with_context(context)?;
    evidence::validate_requirements(root, &[located.contract()], &case.requirements)
        .with_context(context)?
        .map_err(|miss| miss.for_cases())
        .with_context(context)
}

/// Find an active case and confirm its links before it produces an observation.
fn find_checked(root: &Path, id: &str) -> Result<Located> {
    let located = find(root, id)?;
    ensure!(located.case.status == "active", "{id} is retired");
    check_links(root, &located, &fledge_tasks(root)?)?;
    Ok(located)
}

/// The current digest of an active case, or None when it is missing or retired.
pub(super) fn current_digest(root: &Path, id: &str) -> Result<Option<String>> {
    for located in load(root)? {
        if located.case.id == id {
            return if located.case.status == "active" {
                Ok(Some(located.digest()?))
            } else {
                Ok(None)
            };
        }
    }
    Ok(None)
}

pub(super) fn find(root: &Path, id: &str) -> Result<Located> {
    load(root)?
        .into_iter()
        .find(|located| located.case.id == id)
        .with_context(|| format!("unknown case: {id}"))
}

fn summary(located: &Located) -> Result<Value> {
    Ok(json!({
        "id": located.case.id,
        "title": located.case.title,
        "file": located.file,
        "status": located.case.status,
        "track": located.case.track,
        "method": located.case.method,
        "criteria": located.case.criteria,
        "requirements": located.case.requirements,
        "digest": located.digest()?,
    }))
}

fn select(
    root: &Path,
    criteria: &[String],
    requirements: &[String],
    paths: &[String],
    tracks: &[String],
) -> Result<Value> {
    ensure!(
        !(criteria.is_empty() && requirements.is_empty() && paths.is_empty()),
        "name at least one affected --criterion, --requirement or --path"
    );
    let mut selected = vec![];
    let mut omitted = vec![];
    for located in check(root)? {
        let case = &located.case;
        let linked = case.criteria.iter().any(|id| criteria.contains(id))
            || case.requirements.iter().any(|id| requirements.contains(id))
            || case.paths.iter().any(|path| {
                paths.iter().any(|changed| {
                    changed.starts_with(path.as_str()) || path.starts_with(changed.as_str())
                })
            });
        let reason = if case.status == "retired" {
            Some("retired".to_string())
        } else if !linked {
            Some("not linked to the affected criteria, requirements or paths".into())
        } else if case.track != "functional" && !tracks.contains(&case.track) {
            Some(format!(
                "{} track not requested as relevant to this change",
                case.track
            ))
        } else {
            None
        };
        match reason {
            None => selected.push(summary(&located)?),
            Some(reason) => {
                omitted.push(json!({"id": case.id, "track": case.track, "reason": reason}));
            }
        }
    }
    let (lane, origin) = prefs::effective(root, "required-lane")?;
    Ok(json!({
        "schema_version": 1,
        "affected": {"criteria": criteria, "requirements": requirements, "paths": paths, "tracks": tracks},
        "selected": selected,
        "omitted": omitted,
        "required_lane": if lane.is_empty() { json!(null) } else { json!({"lane": lane, "origin": origin, "command": format!("fledge lanes run {lane}")}) },
        "note": "Functional cases linked to the affected scope are always selected; unselected cases are not retested by default.",
    }))
}

/// Run a binding without a shell through the bounded runner, with its declared
/// variables set on top of the inherited environment and each stream capped at the
/// artifact bound while reading.
fn execute(
    root: &Path,
    argv: &[String],
    env: &[(String, String)],
    timeout: Duration,
) -> Result<Captured> {
    let limits = Limits {
        stdout_cap: OUTPUT_LIMIT,
        stderr_cap: OUTPUT_LIMIT,
        deadline: timeout,
    };
    run_bounded(root, argv, env, &limits).with_context(|| format!("cannot start {}", argv[0]))
}

/// Remove complete terminal escape sequences, copying everything else unchanged: CSI
/// (`ESC [`, parameter bytes 0x30 to 0x3F, intermediate bytes 0x20 to 0x2F, one final
/// byte 0x40 to 0x7E), OSC (`ESC ]` up to BEL or `ESC \`, before any line break) and
/// `ESC` with one byte from 0x40 to 0x5F. A sequence that does not complete, before a
/// line break or the end of the input, is kept. No removed byte is a line break, so
/// every line keeps its start and the text before its first ESC. Linear: an OSC that
/// fails to complete fails at the next line break or the end, and so does every OSC
/// that starts before that point. Returns whether anything was removed.
fn strip_escapes(input: &[u8]) -> (Vec<u8>, bool) {
    let mut output = Vec::with_capacity(input.len());
    let mut removed = false;
    let mut unterminated_until = 0;
    let mut index = 0;
    while index < input.len() {
        if input[index] == ESC {
            let length = match input.get(index + 1) {
                Some(b'[') => csi_length(&input[index..]),
                Some(b']') if index >= unterminated_until => {
                    let found = osc_length(&input[index..]);
                    if let Err(stopped) = found {
                        unterminated_until = index + stopped;
                    }
                    found.ok()
                }
                Some(0x40..=0x5f) if !matches!(input[index + 1], b'[' | b']') => Some(2),
                _ => None,
            };
            if let Some(length) = length {
                index += length;
                removed = true;
                continue;
            }
        }
        output.push(input[index]);
        index += 1;
    }
    (output, removed)
}

/// The length of a complete CSI sequence at the start of `bytes`.
fn csi_length(bytes: &[u8]) -> Option<usize> {
    let mut index = 2;
    while matches!(bytes.get(index)?, 0x30..=0x3f) {
        index += 1;
    }
    while matches!(bytes.get(index)?, 0x20..=0x2f) {
        index += 1;
    }
    matches!(bytes.get(index)?, 0x40..=0x7e).then_some(index + 1)
}

/// The length of a complete OSC sequence at the start of `bytes`, or the offset of the
/// line break or end of input that stopped it.
fn osc_length(bytes: &[u8]) -> std::result::Result<usize, usize> {
    let mut index = 2;
    loop {
        match bytes.get(index) {
            None | Some(b'\n' | b'\r') => return Err(index),
            Some(0x07) => return Ok(index + 1),
            Some(&ESC) if bytes.get(index + 1) == Some(&b'\\') => return Ok(index + 2),
            Some(_) => index += 1,
        }
    }
}

/// The artifact: both streams within the 1 MiB total, then a note for each stream
/// whose bytes are not all present. Also returns the text the assertions read, which
/// excludes those notes and has complete terminal escape sequences removed, and
/// whether any were.
fn artifact(execution: &Captured) -> (Vec<u8>, String, bool) {
    const STDOUT_HEADER: &[u8] = b"--- stdout ---\n";
    const STDERR_HEADER: &[u8] = b"\n--- stderr ---\n";
    let mut output = STDOUT_HEADER.to_vec();
    output.extend(&execution.stdout);
    output.extend(STDERR_HEADER);
    let stderr_start = output.len();
    output.extend(&execution.stderr);
    let mut present = [execution.stdout.len(), execution.stderr.len()];
    if output.len() > OUTPUT_LIMIT {
        present[0] = present[0].min(OUTPUT_LIMIT.saturating_sub(STDOUT_HEADER.len()));
        present[1] = present[1].min(OUTPUT_LIMIT.saturating_sub(stderr_start));
        output.truncate(OUTPUT_LIMIT);
        output.extend(b"\n[truncated: captured output is limited to 1048576 bytes]\n");
    }
    let (visible, escapes_removed) = strip_escapes(&output);
    let text = String::from_utf8_lossy(&visible).into_owned();
    for (name, total, present) in [
        ("stdout", execution.stdout_total, present[0] as u64),
        ("stderr", execution.stderr_total, present[1] as u64),
    ] {
        if total > present {
            if !output.ends_with(b"\n") {
                output.push(b'\n');
            }
            output.extend(
                format!(
                    "[truncated: {name} wrote {total} bytes; {} discarded]\n",
                    total - present
                )
                .as_bytes(),
            );
        }
    }
    (output, text, escapes_removed)
}

/// Sum "N passed" and "N failed" across cargo's "test result:" lines.
fn cargo_counts(output: &str) -> (u64, u64) {
    let mut passed = 0;
    let mut failed = 0;
    for line in output
        .lines()
        .filter(|line| line.starts_with("test result:"))
    {
        let words: Vec<&str> = line
            .split(|c: char| c.is_whitespace() || c == ';' || c == '.')
            .filter(|word| !word.is_empty())
            .collect();
        for pair in words.windows(2) {
            if let Ok(count) = pair[0].parse::<u64>() {
                match pair[1] {
                    "passed" => passed += count,
                    "failed" => failed += count,
                    _ => {}
                }
            }
        }
    }
    (passed, failed)
}

/// The counts the cargo-test adapter judges a run by: the larger of the raw and the
/// cleaned count, for passed and for failed. Cleaning reveals counts that colour
/// splits, but a CSI sequence's parameter and intermediate bytes can also swallow a
/// count the raw text shows, so neither text alone can lower a count.
fn matched_counts(raw: &str, cleaned: &str) -> (u64, u64) {
    let raw = cargo_counts(raw);
    let cleaned = cargo_counts(cleaned);
    (raw.0.max(cleaned.0), raw.1.max(cleaned.1))
}

fn argv_for(binding: &Binding) -> Vec<String> {
    match binding.adapter.as_str() {
        "cargo-test" => vec![
            "cargo".into(),
            "test".into(),
            "--locked".into(),
            "--test".into(),
            binding.test_target.clone().unwrap_or_default(),
            "--".into(),
            "--exact".into(),
            binding.test.clone().unwrap_or_default(),
        ],
        "fledge-task" => vec![
            "fledge".into(),
            "run".into(),
            binding.task.clone().unwrap_or_default(),
        ],
        _ => binding.argv.clone(),
    }
}

fn observation_id(root: &Path, case: &str, requested: Option<String>) -> Result<String> {
    if let Some(id) = requested {
        ensure!(valid_name(&id), "invalid observation ID");
        return Ok(id);
    }
    let base = format!("{}-{}", case.to_ascii_lowercase(), now()?);
    for suffix in 0..1_000 {
        let id = if suffix == 0 {
            base.clone()
        } else {
            format!("{base}-{suffix}")
        };
        if !local_path(root, &format!("{}/{id}.json", evidence::EVIDENCE_DIR))?.exists() {
            return Ok(id);
        }
    }
    bail!("cannot allocate an observation ID for {case}")
}

/// The outcome of one explicit case execution, already recorded.
pub(super) struct RunResult {
    pub(super) observation: String,
    pub(super) outcome: Outcome,
    pub(super) summary: String,
}

pub(super) fn run_case(
    root: &Path,
    id: &str,
    environment: &str,
    requested_id: Option<String>,
) -> Result<RunResult> {
    checkpoint::read_session(root)?;
    ensure!(
        !environment.trim().is_empty(),
        "environment must not be blank"
    );
    let located = find_checked(root, id)?;
    let case = &located.case;
    let binding = case.bind.as_ref().with_context(|| {
        format!(
            "{id} has no binding; record a manual or agent-assisted run with `chough cases record`"
        )
    })?;
    let digest = located.digest()?;
    let argv = argv_for(binding);
    let timeout = Duration::from_secs(binding.timeout_seconds.unwrap_or(600));
    let observation = observation_id(root, id, requested_id)?;
    let env: Vec<(String, String)> = binding
        .env
        .iter()
        .map(|(name, value)| (name.clone(), value.clone()))
        .collect();
    let started = Instant::now();
    let execution = execute(root, &argv, &env, timeout)?;
    let duration_seconds = started.elapsed().as_secs();
    let (output, text, escapes_removed) = artifact(&execution);
    let artifact = format!("{ARTIFACTS_DIR}/{observation}.log");
    ensure!(
        write_new_bytes(root, &artifact, &output)?,
        "artifact {artifact} already exists"
    );
    let mut limitations = vec![];
    let missing: Vec<&String> = binding
        .expect_output
        .iter()
        .filter(|fragment| !text.contains(fragment.as_str()))
        .collect();
    let mut tests_run = None;
    let mut outcome = if execution.timed_out {
        limitations.push(if execution.descendants_stopped {
            format!(
                "timed out after {} s; the command and its descendants were stopped",
                timeout.as_secs()
            )
        } else {
            format!(
                "timed out after {} s; the command was stopped, but stopping its descendants failed",
                timeout.as_secs()
            )
        });
        Outcome::Failed
    } else if binding.adapter == "cargo-test" {
        let (passed, failed) = matched_counts(&String::from_utf8_lossy(&output), &text);
        tests_run = Some(passed + failed);
        if passed + failed == 0 && execution.exit_code != Some(0) {
            limitations.push(format!(
                "cargo exited {} before reporting any test result: a build or environment failure, not an assertion about the behavior",
                execution
                    .exit_code
                    .map_or("without a code".to_string(), |code| code.to_string())
            ));
            Outcome::NotRun
        } else if passed + failed == 0 {
            limitations.push("no test matched the binding; nothing was asserted".into());
            Outcome::NotRun
        } else if passed + failed != 1 {
            limitations.push(format!(
                "the binding should select exactly one test; {} ran",
                passed + failed
            ));
            Outcome::NotRun
        } else if failed > 0 || execution.exit_code != Some(0) || !missing.is_empty() {
            Outcome::Failed
        } else {
            Outcome::Passed
        }
    } else if execution.exit_code != Some(0) || !missing.is_empty() {
        Outcome::Failed
    } else {
        Outcome::Passed
    };
    if execution.held_open {
        limitations.push(if execution.descendants_stopped {
            "descendants still held output after exit and were stopped".into()
        } else {
            "descendants still held output after exit and could not be stopped".into()
        });
    }
    if execution.capture_incomplete {
        limitations.push(
            "output capture incomplete: output was still open when reading stopped, so this run cannot pass"
                .into(),
        );
        if outcome == Outcome::Passed {
            outcome = Outcome::Failed;
        }
    }
    if !missing.is_empty() {
        limitations.push(format!(
            "expected output missing: {}",
            missing
                .iter()
                .map(|fragment| format!("{fragment:?}"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    if binding.adapter == "fledge-task" || binding.adapter == "command" {
        limitations.push(
            "outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected"
                .into(),
        );
    }
    if escapes_removed {
        limitations.push(ESCAPES_REMOVED.into());
    }
    limitations.push(INHERITED_ENVIRONMENT.into());
    if !binding.env.is_empty() {
        // Names only: values are bound through the case digest and the committed file.
        limitations.push(format!(
            "declared environment set: {}",
            binding.env.keys().cloned().collect::<Vec<_>>().join(", ")
        ));
    }
    let outcome_word = match outcome {
        Outcome::Passed => "passed",
        Outcome::Failed => "failed",
        Outcome::NotRun => "not-run",
    };
    let summary = format!(
        "Ran {} for {id}: {outcome_word} (exit {}{}).",
        argv.join(" "),
        execution
            .exit_code
            .map_or("none".to_string(), |code| code.to_string()),
        tests_run.map_or(String::new(), |count| format!(", tests run {count}"))
    );
    let record = Evidence {
        schema_version: 2,
        id: observation.clone(),
        criteria: case.criteria.clone(),
        requirements: case.requirements.clone(),
        contracts: vec![located.contract()],
        kind: Kind::ProductTest,
        outcome: outcome.clone(),
        environment: environment.into(),
        summary: summary.clone(),
        artifacts: vec![artifact.clone()],
        artifact_fingerprints: evidence::artifact_fingerprints(root, &[artifact])?,
        observed_at_unix_seconds: now()?,
        snapshot: evidence::snapshot(root)?,
        provenance: PROVENANCE.into(),
        cases: vec![CaseRef {
            id: id.into(),
            digest,
        }],
        method: Some(case.method.clone()),
        track: Some(case.track.clone()),
        argv,
        exit_code: execution.exit_code,
        tests_run,
        limitations,
        duration_seconds: Some(duration_seconds),
    };
    evidence::record(root, &record)?;
    Ok(RunResult {
        observation,
        outcome,
        summary,
    })
}

fn record_manual(root: &Path, args: RecordArgs) -> Result<Value> {
    let RecordArgs {
        id,
        environment,
        method,
        verdicts,
        summary,
        artifacts,
        limitations,
        observation_id: requested_id,
    } = args;
    let id = id.as_str();
    checkpoint::read_session(root)?;
    let located = find_checked(root, id)?;
    let case = &located.case;
    ensure!(
        case.method != "automated",
        "{id} is automated; run it with `chough cases run`"
    );
    ensure!(
        !environment.trim().is_empty() && !summary.trim().is_empty(),
        "environment and summary must not be blank"
    );
    let mut results = BTreeMap::new();
    for verdict in &verdicts {
        let (index, value) = verdict
            .split_once('=')
            .context("verdicts look like 1=pass or 2=fail")?;
        let index: usize = index.parse().context("verdict index must be a number")?;
        ensure!(
            (1..=case.expected.len()).contains(&index),
            "verdict index {index} is outside 1..={}",
            case.expected.len()
        );
        ensure!(
            matches!(value, "pass" | "fail"),
            "verdict must be pass or fail"
        );
        ensure!(
            results.insert(index, value).is_none(),
            "duplicate verdict for expected outcome {index}"
        );
    }
    ensure!(
        results.len() == case.expected.len(),
        "give a verdict for every expected outcome (1..={})",
        case.expected.len()
    );
    let outcome = if results.values().all(|value| *value == "pass") {
        Outcome::Passed
    } else {
        Outcome::Failed
    };
    let verdict_text = results
        .iter()
        .map(|(index, value)| format!("{index}={value}"))
        .collect::<Vec<_>>()
        .join(", ");
    let observation = observation_id(root, id, requested_id)?;
    let record = Evidence {
        schema_version: 2,
        id: observation.clone(),
        criteria: case.criteria.clone(),
        requirements: case.requirements.clone(),
        contracts: vec![located.contract()],
        kind: if method == "manual" {
            Kind::HumanObservation
        } else {
            Kind::ProductTest
        },
        outcome: outcome.clone(),
        environment,
        summary: format!("Verdicts {verdict_text}. {summary}"),
        artifacts: artifacts.clone(),
        artifact_fingerprints: evidence::artifact_fingerprints(root, &artifacts)?,
        observed_at_unix_seconds: now()?,
        snapshot: evidence::snapshot(root)?,
        provenance: PROVENANCE.into(),
        cases: vec![CaseRef {
            id: id.into(),
            digest: located.digest()?,
        }],
        method: Some(method),
        track: Some(case.track.clone()),
        argv: vec![],
        exit_code: None,
        tests_run: None,
        limitations,
        duration_seconds: None,
    };
    let name = evidence::record(root, &record)?;
    Ok(json!({"schema_version": 1, "recorded": name, "outcome": outcome, "provenance": PROVENANCE}))
}

/// Exit code for a recorded run whose outcome is not passed.
pub(super) const NOT_PASSED_EXIT: i32 = 3;

pub(super) fn run(root: &Path, command: CasesCommand) -> Result<Option<i32>> {
    match command {
        CasesCommand::Check | CasesCommand::List => {
            let cases = if matches!(command, CasesCommand::Check) {
                check(root)?
            } else {
                load(root)?
            };
            let summaries = cases.iter().map(summary).collect::<Result<Vec<_>>>()?;
            print_json(
                &json!({"schema_version": 1, "profile": PROFILE, "valid": true, "cases": summaries}),
            )?;
            Ok(None)
        }
        CasesCommand::Select {
            criteria,
            requirements,
            paths,
            tracks,
        } => {
            print_json(&select(root, &criteria, &requirements, &paths, &tracks)?)?;
            Ok(None)
        }
        CasesCommand::Run {
            id,
            environment,
            observation_id,
        } => {
            let result = run_case(root, &id, &environment, observation_id)?;
            print_json(&json!({
                "schema_version": 1,
                "case": id,
                "observation": result.observation,
                "outcome": result.outcome,
                "summary": result.summary,
                "provenance": PROVENANCE,
                "retention": evidence::auto_prune(root),
            }))?;
            Ok((result.outcome != Outcome::Passed).then_some(NOT_PASSED_EXIT))
        }
        CasesCommand::Record(args) => {
            let value = record_manual(root, args)?;
            let passed = value["outcome"] == "passed";
            print_json(&value)?;
            Ok((!passed).then_some(NOT_PASSED_EXIT))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{cargo_counts, matched_counts, strip_escapes};

    fn clean(input: &[u8]) -> (String, bool) {
        let (bytes, removed) = strip_escapes(input);
        (String::from_utf8(bytes).unwrap(), removed)
    }

    #[test]
    fn complete_sequences_are_removed() {
        for (input, expected) in [
            (&b"\x1b[1;32mok\x1b[0m"[..], "ok"),
            (b"\x1b[?25l\x1b[2K\x1b[1G done", " done"),
            (b"a\x1b[ qb", "ab"),
            (b"\x1b]0;title\x07text", "text"),
            (b"\x1b]8;;file:///x\x1b\\link\x1b]8;;\x1b\\", "link"),
            (b"a\x1bMb\x1b\\c\x1b_d", "abcd"),
        ] {
            assert_eq!(clean(input), (expected.to_string(), true), "{input:?}");
        }
    }

    #[test]
    fn incomplete_or_unknown_sequences_are_kept() {
        for input in [
            &b"plain text\n"[..],
            b"two-byte limit \x1b7 and \x1b` stay",
            b"esc before a break \x1b\nnext",
            b"csi cut by a break \x1b[31\nnext",
            b"csi with a bad byte \x1b[31\x01m",
            b"osc cut by a break \x1b]0;title\nnext\x07",
            b"osc cut by a return \x1b]0;title\rnext\x07",
            b"osc cut by the end \x1b]0;title",
            b"csi cut by the end \x1b[",
            b"lone trailing escape \x1b",
        ] {
            assert_eq!(
                clean(input),
                (String::from_utf8(input.to_vec()).unwrap(), false),
                "{input:?}"
            );
        }
    }

    /// A small deterministic generator, so the property runs the same everywhere.
    struct Generator(u64);

    impl Generator {
        fn next(&mut self) -> u64 {
            self.0 = self
                .0
                .wrapping_mul(6_364_136_223_846_793_005)
                .wrapping_add(1_442_695_040_888_963_407);
            self.0 >> 33
        }

        fn byte(&mut self) -> u8 {
            // Bias towards the bytes that start, continue or end sequences.
            const INTERESTING: &[u8] = b"\x1b\x1b\x1b[]\\\x07\n\r;0m ?@~_";
            if self.next().is_multiple_of(2) {
                INTERESTING[(self.next() % INTERESTING.len() as u64) as usize]
            } else {
                self.next() as u8
            }
        }
    }

    fn line_breaks(bytes: &[u8]) -> usize {
        bytes
            .iter()
            .filter(|byte| matches!(byte, b'\n' | b'\r'))
            .count()
    }

    fn result_lines(bytes: &[u8]) -> usize {
        String::from_utf8_lossy(bytes)
            .lines()
            .filter(|line| line.starts_with("test result:"))
            .count()
    }

    #[test]
    fn cleaning_keeps_every_line_break_and_never_hides_a_test_result_line() {
        let mut generator = Generator(0x0275_6dc2);
        let result = b"test result: ok. 1 passed; 0 failed; 0 ignored\n";
        let mut inputs: Vec<Vec<u8>> = (0..=255_u8).map(|byte| vec![0x1b, byte]).collect();
        for prefix in [&b"\x1b]0;open"[..], b"\x1b[31", b"\x1b", b"\x1b]8;;x\x1b"] {
            let mut input = prefix.to_vec();
            input.push(b'\n');
            input.extend(result);
            input.extend(b"\x07");
            input.extend(result);
            inputs.push(input);
        }
        inputs.push(
            b"test result: ok. 1 passed; 0 failed\ntest result: ok. 0 passed;\x1b[0;1 passed\n"
                .to_vec(),
        );
        for _ in 0..2_000 {
            let mut input = vec![];
            for _ in 0..(generator.next() % 12) {
                for _ in 0..(generator.next() % 24) {
                    input.push(generator.byte());
                }
                if generator.next().is_multiple_of(3) {
                    input.push(b'\n');
                    input.extend(result);
                }
            }
            inputs.push(input);
        }
        for input in inputs {
            let (cleaned, _) = strip_escapes(&input);
            assert_eq!(line_breaks(&cleaned), line_breaks(&input), "{input:?}");
            assert!(
                result_lines(&cleaned) >= result_lines(&input),
                "{input:?} -> {cleaned:?}"
            );
            let raw = String::from_utf8_lossy(&input);
            let before = cargo_counts(&raw);
            let after = matched_counts(&raw, &String::from_utf8_lossy(&cleaned));
            assert!(after.0 >= before.0 && after.1 >= before.1, "{input:?}");
        }
    }

    #[test]
    fn a_sequence_that_swallows_a_count_never_lowers_the_matched_counts() {
        // CSI parameter bytes include digits and `;` and intermediate bytes include a
        // space, so `ESC[0;1 p` swallows the `1` that the raw text counts as a result.
        let raw =
            b"test result: ok. 1 passed; 0 failed\ntest result: ok. 0 passed;\x1b[0;1 passed\n";
        let (cleaned, removed) = strip_escapes(raw);
        assert!(removed);
        let raw = String::from_utf8_lossy(raw);
        let cleaned = String::from_utf8_lossy(&cleaned);
        assert_eq!(cargo_counts(&raw), (2, 0));
        assert_eq!(cargo_counts(&cleaned), (1, 0));
        // Two tests ran by the raw count, so the run cannot read as a one-test pass.
        assert_eq!(matched_counts(&raw, &cleaned), (2, 0));
        // A count that only cleaning reveals, such as coloured output, still counts.
        let coloured = "test result: ok. \x1b[32m1\x1b[0m passed; \x1b[31m0\x1b[0m failed";
        let (visible, _) = strip_escapes(coloured.as_bytes());
        assert_eq!(cargo_counts(coloured), (0, 0));
        assert_eq!(
            matched_counts(coloured, &String::from_utf8_lossy(&visible)),
            (1, 0)
        );
    }

    #[test]
    fn cleaning_a_long_unterminated_line_is_linear() {
        let input = b"\x1b]".repeat(500_000);
        let started = std::time::Instant::now();
        let (cleaned, removed) = strip_escapes(&input);
        assert!(!removed);
        assert_eq!(cleaned, input);
        assert!(started.elapsed().as_secs() < 5, "{:?}", started.elapsed());
    }
}
