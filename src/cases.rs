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
                "cargo-test uses only test_target, test, expect_output and timeout_seconds"
            );
        }
        "fledge-task" => {
            ensure!(
                binding.task.as_deref().is_some_and(valid_name),
                "fledge-task needs task"
            );
            ensure!(
                binding.test_target.is_none() && binding.test.is_none() && binding.argv.is_empty(),
                "fledge-task uses only task, expect_output and timeout_seconds"
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
                "command uses only argv, expect_output and timeout_seconds"
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

/// Run a binding without a shell through the bounded runner, each stream capped at
/// the artifact bound while reading.
fn execute(root: &Path, argv: &[String], timeout: Duration) -> Result<Captured> {
    let limits = Limits {
        stdout_cap: OUTPUT_LIMIT,
        stderr_cap: OUTPUT_LIMIT,
        deadline: timeout,
    };
    run_bounded(root, argv, &limits).with_context(|| format!("cannot start {}", argv[0]))
}

/// The artifact: both streams within the 1 MiB total, then a note for each stream
/// whose bytes are not all present. Also returns the text the assertions read, which
/// excludes those notes.
fn artifact(execution: &Captured) -> (Vec<u8>, String) {
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
    let text = String::from_utf8_lossy(&output).into_owned();
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
    (output, text)
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
    let started = Instant::now();
    let execution = execute(root, &argv, timeout)?;
    let duration_seconds = started.elapsed().as_secs();
    let (output, text) = artifact(&execution);
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
        let (passed, failed) = cargo_counts(&text);
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
