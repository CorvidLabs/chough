//! Optional repeated validation in the foreground. It runs only with a run consent or a
//! standing preference, checks for a stop request and a paused checkpoint before every
//! attempt, and never claims soak testing, live health or authority to repair anything.
//!
//! A repeated run is one evidence entry of the active session (REQ-cli-023), named from
//! its case ID and `run`, that holds each attempt; each attempt is also its own case-run
//! entry, which bugs link.

use crate::bugs::{self, Finding};
use crate::cases;
use crate::checkpoint::read_checkpoint_unlocked;
use crate::evidence::Outcome;
use crate::markdown::{self, Record, Value as Front};
use crate::prefs;
use crate::records::{local_path, now, print_json, write_new, write_replace_bounded};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::Deserialize;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

/// A machine-local stop request: never committed (REQ-records-002).
pub(super) const STOP_REQUEST: &str = ".chough/local/validation.stop";
/// The longest wait between checks for a stop request or pause during an interval.
const WAIT_SLICE: Duration = Duration::from_millis(200);

/// A 0.5.x run file as 0.5.0 wrote it; read only by [`legacy_run`].
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)] // bugs is read only to accept the file
struct LegacyJson {
    schema_version: u32,
    id: String,
    case: String,
    times: u32,
    interval_seconds: u64,
    environment: String,
    authority: String,
    state: String,
    completed: u32,
    failures: u32,
    not_run: u32,
    observations: Vec<String>,
    bugs: Vec<String>,
    elapsed_seconds: u64,
    started_at_unix_seconds: u64,
}

/// One attempt of a repeated run: when it started, when its observation was recorded,
/// its result and run time, and the case-run entry it recorded.
#[derive(Clone, Debug, PartialEq)]
struct Attempt {
    started: u64,
    observed: u64,
    result: String,
    duration_seconds: Option<u64>,
    observation: String,
}

/// A repeated validation run, as its evidence entry holds it.
#[derive(Clone, Debug)]
struct Run {
    /// The entry's name, which is its ID (REQ-records-003).
    id: String,
    /// The entry's project-relative path.
    path: String,
    aliases: Vec<String>,
    case: String,
    times: u32,
    interval_seconds: u64,
    environment: String,
    authority: String,
    state: String,
    failures: u32,
    not_run: u32,
    attempts: Vec<Attempt>,
    bugs: Vec<String>,
    elapsed_seconds: u64,
    started_at_unix_seconds: u64,
    completed_at_unix_seconds: Option<u64>,
}

impl Run {
    /// The run as commands print it: its counts, its attempts' observations and its entry.
    fn view(&self) -> Value {
        json!({
            "id": self.id,
            "entry": self.path,
            "aliases": self.aliases,
            "case": self.case,
            "times": self.times,
            "interval_seconds": self.interval_seconds,
            "environment": self.environment,
            "authority": self.authority,
            "state": self.state,
            "completed": self.attempts.len(),
            "failures": self.failures,
            "not_run": self.not_run,
            "observations": self.attempts.iter().map(|attempt| attempt.observation.as_str()).collect::<Vec<_>>(),
            "bugs": self.bugs,
            "elapsed_seconds": self.elapsed_seconds,
            "started_at_unix_seconds": self.started_at_unix_seconds,
        })
    }

    /// The entry record (REQ-cli-023, plan delta correction 4): the run's fields in front
    /// matter, one flat map per attempt, and a structural claim. The run entry is a
    /// summary, never an executed result: its attempts are the observations, so it
    /// names its case as `run_case`, not `case`, and readers of case runs skip it.
    fn record(&self, id: &str, revision: &str) -> Result<Record> {
        let mut record = Record::new(&format!(
            "**A repeated validation run of {}.**\n\n{} of {} attempts, {} failed, {} not run, {} seconds; this is not a soak test, live-health check or repair authorization.\n",
            self.case,
            self.attempts.len(),
            self.times,
            self.failures,
            self.not_run,
            self.elapsed_seconds
        ));
        let iso = crate::session_md::iso_time;
        record.set_scalar("kind", "evidence")?;
        record.set_scalar("id", id)?;
        record.set("aliases", Front::List(self.aliases.clone()))?;
        record.set_scalar("validation_run", "true")?;
        record.set_scalar(
            "claim",
            &format!("Repeated validation run of {}", self.case),
        )?;
        record.set_scalar("result", "not-run")?;
        record.set_scalar("evidence_kind", "structural")?;
        record.set_scalar("run_case", &self.case)?;
        record.set_scalar("environment", &self.environment)?;
        record.set_scalar("authority", &self.authority)?;
        record.set_scalar("times", &self.times.to_string())?;
        record.set_scalar("interval_seconds", &self.interval_seconds.to_string())?;
        record.set_scalar("state", &self.state)?;
        record.set_scalar("started", &iso(self.started_at_unix_seconds))?;
        if let Some(completed) = self.completed_at_unix_seconds {
            record.set_scalar("completed", &iso(completed))?;
        }
        record.set_scalar("failures", &self.failures.to_string())?;
        record.set_scalar("not_run", &self.not_run.to_string())?;
        record.set_scalar("elapsed_seconds", &self.elapsed_seconds.to_string())?;
        record.set("bugs", Front::List(self.bugs.clone()))?;
        record.set(
            "attempts",
            Front::Maps(
                self.attempts
                    .iter()
                    .map(|attempt| {
                        let mut map = vec![
                            ("started".to_string(), iso(attempt.started)),
                            ("observed".to_string(), iso(attempt.observed)),
                            ("result".to_string(), attempt.result.clone()),
                        ];
                        if let Some(seconds) = attempt.duration_seconds {
                            map.push(("duration_seconds".to_string(), seconds.to_string()));
                        }
                        map.push(("observation".to_string(), attempt.observation.clone()));
                        map
                    })
                    .collect(),
            ),
        )?;
        let observed = self
            .attempts
            .last()
            .map_or(self.started_at_unix_seconds, |attempt| attempt.observed);
        record.set_scalar("observed", &iso(observed))?;
        record.set_scalar("revision", revision)?;
        Ok(record)
    }

    /// Read a run back from its entry, or None when the entry is not a run.
    fn from_record(path: &str, record: &Record) -> Result<Option<Run>> {
        if record.scalar("validation_run") != Some("true") {
            return Ok(None);
        }
        let need = |key: &str| {
            record
                .scalar(key)
                .map(str::to_string)
                .with_context(|| format!("{path}: the validation run lacks {key}"))
        };
        let number = |key: &str| -> Result<u64> {
            need(key)?
                .parse::<u64>()
                .with_context(|| format!("{path}: the validation run's {key} is not a number"))
        };
        let time = |text: &str| {
            crate::session_md::parse_time(text)
                .with_context(|| format!("{path}: a validation run time"))
        };
        let mut attempts = vec![];
        for map in record.maps("attempts") {
            let get = |key: &str| {
                map.iter()
                    .find(|(name, _)| name == key)
                    .map(|(_, value)| value.clone())
                    .with_context(|| format!("{path}: an attempt lacks {key}"))
            };
            attempts.push(Attempt {
                started: time(&get("started")?)?,
                observed: time(&get("observed")?)?,
                result: get("result")?,
                duration_seconds: get("duration_seconds")
                    .ok()
                    .and_then(|text| text.parse().ok()),
                observation: get("observation")?,
            });
        }
        Ok(Some(Run {
            id: need("id")?,
            path: path.to_string(),
            aliases: record.list("aliases"),
            case: need("run_case")?,
            times: u32::try_from(number("times")?).context("times is too large")?,
            interval_seconds: number("interval_seconds")?,
            environment: need("environment")?,
            authority: need("authority")?,
            state: need("state")?,
            failures: u32::try_from(number("failures")?).context("failures is too large")?,
            not_run: u32::try_from(number("not_run")?).context("not_run is too large")?,
            attempts,
            bugs: record.list("bugs"),
            elapsed_seconds: number("elapsed_seconds")?,
            started_at_unix_seconds: time(&need("started")?)?,
            completed_at_unix_seconds: record.scalar("completed").map(time).transpose()?,
        }))
    }
}

/// A 0.5.x validation run, `.chough/validation/<VAL id>.json`, as 0.5.0 wrote it: what
/// the migration's evidence mapper reads to carry or remove it (REQ-records-009).
#[derive(Clone, Debug)]
#[cfg_attr(not(test), allow(dead_code))]
pub(super) struct LegacyRun {
    pub(super) id: String,
    pub(super) case: String,
    pub(super) times: u32,
    pub(super) interval_seconds: u64,
    pub(super) environment: String,
    /// Who authorized the run, as 0.5.x recorded it: the human's consent or a standing
    /// preference, kept when the run is carried.
    pub(super) authority: String,
    pub(super) state: String,
    pub(super) completed: u32,
    pub(super) failures: u32,
    pub(super) not_run: u32,
    pub(super) observations: Vec<String>,
    pub(super) elapsed_seconds: u64,
    pub(super) started_at_unix_seconds: u64,
}

/// Read a 0.5.x validation run from its text, refusing one that is not a schema 1 run
/// and naming `path`.
// The evidence mapper (t38b) calls it; until then only tests do.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn legacy_run(path: &str, text: &str) -> Result<LegacyRun> {
    let run: LegacyJson = serde_json::from_str(text)
        .with_context(|| format!("{path} is not a 0.5.x validation run"))?;
    ensure!(
        run.schema_version == 1,
        "{path} is not a schema 1 validation run"
    );
    Ok(LegacyRun {
        id: run.id,
        case: run.case,
        times: run.times,
        interval_seconds: run.interval_seconds,
        environment: run.environment,
        authority: run.authority,
        state: run.state,
        completed: run.completed,
        failures: run.failures,
        not_run: run.not_run,
        observations: run.observations,
        elapsed_seconds: run.elapsed_seconds,
        started_at_unix_seconds: run.started_at_unix_seconds,
    })
}

#[derive(Subcommand)]
pub(super) enum ValidateCommand {
    /// Repeat one case a fixed number of times and record every attempt.
    Run {
        case: String,
        #[arg(long)]
        times: u32,
        #[arg(long, default_value_t = 0)]
        interval: u64,
        #[arg(long)]
        environment: String,
    },
    /// Ask a running or future repeated run to stop before its next attempt.
    Stop,
    /// Continue a stopped or paused run with its stored limits.
    Resume { id: String },
    /// Show stored runs.
    Status,
}

fn authority(root: &Path) -> Result<String> {
    let session = read_checkpoint_unlocked(root)?;
    if let Some(origin) = prefs::authorized(root, &session, "repeat-validation")? {
        return Ok(origin);
    }
    let (value, origin) = prefs::effective(root, "repeat-validation")?;
    bail!(
        "repeated validation is optional and not authorized (repeat-validation = {value}, {origin}). Ask the human; record their answer with `chough consent grant repeat-validation --source <their words>` for this objective, or `chough pref set repeat-validation always --source <...>` only if they asked for it to always apply"
    )
}

/// One attempt of a 0.5.x run the migration carries: its observation's new ID, its
/// start and observed times, result and run time when known.
#[cfg_attr(not(test), allow(dead_code))] // the evidence mapper (t38b) builds them
pub(super) struct MigratedAttempt {
    pub(super) observation: String,
    pub(super) started: u64,
    pub(super) observed: u64,
    pub(super) result: String,
    pub(super) duration_seconds: Option<u64>,
}

/// The entry text of a 0.5.x run the migration carries (REQ-records-009, REQ-cli-023):
/// the format `validate run` writes, named `id`, its old ID an alias, so `validate
/// resume <old ID>` continues it. Not leak-checked here: the migration redacts every
/// file it writes.
#[cfg_attr(not(test), allow(dead_code))] // the evidence mapper (t38b) calls it
pub(super) fn migrated_run_text(
    legacy: &LegacyRun,
    id: &str,
    authority: &str,
    interval_seconds: u64,
    attempts: Vec<MigratedAttempt>,
    revision: &str,
) -> Result<String> {
    let run = Run {
        id: id.to_string(),
        path: String::new(),
        aliases: vec![legacy.id.clone()],
        case: legacy.case.clone(),
        times: legacy.times,
        interval_seconds,
        environment: legacy.environment.clone(),
        authority: authority.to_string(),
        state: legacy.state.clone(),
        failures: legacy.failures,
        not_run: legacy.not_run,
        attempts: attempts
            .into_iter()
            .map(|attempt| Attempt {
                started: attempt.started,
                observed: attempt.observed,
                result: attempt.result,
                duration_seconds: attempt.duration_seconds,
                observation: attempt.observation,
            })
            .collect(),
        bugs: vec![],
        elapsed_seconds: legacy.elapsed_seconds,
        started_at_unix_seconds: legacy.started_at_unix_seconds,
        completed_at_unix_seconds: None,
    };
    run.record(id, revision)?
        .render_within(markdown::Kind::Evidence)
}

/// The active session's evidence folder, where a new run's entry goes.
fn active_evidence_dir(root: &Path) -> Result<String> {
    let relative = crate::sessions::checkpoint_path(root)?;
    let id = crate::sessions::id_of(&relative).with_context(|| {
        format!("{relative} is not an open session; run `chough session new <key>` first")
    })?;
    Ok(format!("{}/{id}/evidence", crate::sessions::DIR))
}

/// Render a run's entry under `id`, refusing a leak, or redacting it under --redact
/// (REQ-records-006).
fn render(root: &Path, run: &Run, id: &str) -> Result<String> {
    let revision = crate::records::head_revision(root).unwrap_or_default();
    let mut record = run.record(id, &revision)?;
    let local = crate::leaks::Local::current();
    if crate::leaks::redacting() {
        let redacted = record.redact_leaks(local);
        if redacted > 0 {
            eprintln!("chough: redacted {redacted} value(s) in validation run {id} (--redact)");
        }
    } else {
        record.check_leaks(local)?;
    }
    record.render_within(markdown::Kind::Evidence)
}

/// Create a new run's entry by create-new, named from its case ID and `run`, with one
/// clash suffix on a same-day clash and a refusal on a second (REQ-records-003,
/// REQ-cli-023). Returns the run with its name and path.
fn create(root: &Path, mut run: Run) -> Result<Run> {
    let dir = active_evidence_dir(root)?;
    let base = crate::identity::dated_name(
        &crate::identity::utc_date(run.started_at_unix_seconds),
        &format!("{} run", run.case),
        "run",
    );
    let name = crate::identity::create_named(
        root,
        &dir,
        &base,
        ".md",
        markdown::Kind::Evidence.bound(),
        |name| render(root, &run, name),
    )
    .with_context(|| format!("creating the validation run entry in {dir}"))?;
    run.path = format!("{dir}/{name}.md");
    run.id = name;
    Ok(run)
}

/// Replace a run's entry with its current state.
fn save(root: &Path, run: &Run) -> Result<()> {
    let text = render(root, run, &run.id)?;
    write_replace_bounded(root, &run.path, &text, markdown::Kind::Evidence.bound())
}

/// Every repeated run in the open sessions' evidence folders.
fn all_runs(root: &Path) -> Result<Vec<Run>> {
    let mut runs = vec![];
    for session in crate::sessions::open_ids(root)? {
        let dir = format!("{}/{session}/evidence", crate::sessions::DIR);
        let Ok(entries) = fs::read_dir(local_path(root, &dir)?) else {
            continue;
        };
        let mut names: Vec<String> = entries
            .filter_map(|entry| entry.ok())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| name.ends_with(".md") && !name.starts_with('.'))
            .collect();
        names.sort();
        for name in names {
            let path = format!("{dir}/{name}");
            let text = crate::records::read_bounded_record(
                &local_path(root, &path)?,
                markdown::Kind::Evidence.bound(),
            )
            .with_context(|| format!("reading {path}"))?;
            if let Some(run) = Run::from_record(&path, &markdown::parse(&path, &text)?)? {
                runs.push(run);
            }
        }
    }
    Ok(runs)
}

/// The run whose ID or alias is `id`; a 0.5.x `VAL-` ID resolves through the alias the
/// migration keeps (REQ-cli-023).
fn load(root: &Path, id: &str) -> Result<Run> {
    let mut found: Vec<Run> = all_runs(root)?
        .into_iter()
        .filter(|run| run.id == id || run.aliases.iter().any(|alias| alias == id))
        .collect();
    match found.len() {
        1 => Ok(found.remove(0)),
        0 => bail!("unknown validation run: {id}"),
        _ => bail!(
            "refused: validation run {id} is ambiguous: it is {}",
            found
                .iter()
                .map(|run| run.path.as_str())
                .collect::<Vec<_>>()
                .join(" and ")
        ),
    }
}

/// The state a repeated run takes when it must not start another attempt now: a stop
/// request stops it, and a paused checkpoint pauses it.
fn halt(root: &Path) -> Result<Option<&'static str>> {
    if local_path(root, STOP_REQUEST)?.exists() {
        return Ok(Some("stopped"));
    }
    if read_checkpoint_unlocked(root)?.stage.status == "paused" {
        return Ok(Some("paused"));
    }
    Ok(None)
}

/// Wait out an interval in short slices, returning early once the run must halt.
fn wait_interval(root: &Path, seconds: u64) -> Result<()> {
    let until = Instant::now() + Duration::from_secs(seconds);
    loop {
        let left = until.saturating_duration_since(Instant::now());
        if left.is_zero() || halt(root)?.is_some() {
            return Ok(());
        }
        thread::sleep(left.min(WAIT_SLICE));
    }
}

fn outcome_word(outcome: &Outcome) -> &'static str {
    match outcome {
        Outcome::Passed => "passed",
        Outcome::Failed => "failed",
        Outcome::NotRun => "not-run",
    }
}

fn continue_run(root: &Path, mut run: Run) -> Result<()> {
    let started = Instant::now();
    let previous = run.elapsed_seconds;
    run.state = "running".into();
    save(root, &run)?;
    while (run.attempts.len() as u32) < run.times {
        if !run.attempts.is_empty() && run.interval_seconds > 0 {
            // Another command may write while this run waits (REQ-cli-040).
            crate::checkpoint::release_checkpoint_locks();
            wait_interval(root, run.interval_seconds)?;
        }
        // Checked immediately before every attempt, including after the interval.
        if let Some(state) = halt(root)? {
            run.state = state.into();
            break;
        }
        let attempt_started = now()?;
        let attempt_clock = Instant::now();
        let result = cases::run_case(root, &run.case, &run.environment, None)?;
        let (observed, duration) = match crate::evidence::resolve_entry(root, &result.observation) {
            Ok(Some((_, entry))) => (entry.observed_at_unix_seconds, entry.duration_seconds),
            _ => (now()?, Some(attempt_clock.elapsed().as_secs())),
        };
        run.attempts.push(Attempt {
            started: attempt_started,
            observed,
            result: outcome_word(&result.outcome).into(),
            duration_seconds: duration,
            observation: result.observation.clone(),
        });
        match result.outcome {
            Outcome::Passed => {}
            Outcome::NotRun => run.not_run += 1,
            Outcome::Failed => {
                run.failures += 1;
                let located = cases::find(root, &run.case)?;
                let bug = bugs::capture_failure(
                    root,
                    Finding {
                        case: &run.case,
                        criteria: located.case.criteria.clone(),
                        requirements: located.case.requirements.clone(),
                        environment: &run.environment,
                        observation: &result.observation,
                        summary: &result.summary,
                        source: format!("validation run {}", run.id),
                    },
                )?;
                if !run.bugs.contains(&bug) {
                    run.bugs.push(bug);
                }
            }
        }
        run.elapsed_seconds = previous + started.elapsed().as_secs();
        save(root, &run)?;
    }
    if run.state == "running" {
        run.state = "complete".into();
        run.completed_at_unix_seconds = Some(now()?);
    }
    run.elapsed_seconds = previous + started.elapsed().as_secs();
    save(root, &run)?;
    let (initiation, _) = prefs::effective(root, "monitor-initiation")?;
    let proposal = (run.failures > 0 && initiation == "propose-change").then(|| {
        format!(
            "Proposed only, not run: specsync change new \"Investigate {} failures in validation run {}\" --kind bug-fix",
            run.case, run.id
        )
    });
    print_json(&json!({
        "schema_version": 1,
        "run": run.view(),
        "label": format!(
            "repeated run: {} of {} attempts, {} failed, {} not run, {} s elapsed; this is not a soak test, live-health check or repair authorization",
            run.attempts.len(), run.times, run.failures, run.not_run, run.elapsed_seconds
        ),
        "captured_locally": run.bugs,
        "proposal": proposal,
    }))
}

/// The recorded times and counts of one stored run, for the timing report.
pub(super) struct RunTimes {
    pub(super) id: String,
    pub(super) case: String,
    pub(super) environment: String,
    pub(super) state: String,
    pub(super) started_at_unix_seconds: u64,
    pub(super) completed: u32,
    pub(super) failures: u32,
    pub(super) not_run: u32,
    pub(super) attempts: usize,
    /// Accumulated across interval waits and resumes; never a per-attempt time.
    pub(super) elapsed_seconds: u64,
}

/// Every stored run's times and counts, in timestamp order.
pub(super) fn run_times(root: &Path) -> Result<Vec<RunTimes>> {
    let mut runs: Vec<RunTimes> = all_runs(root)?
        .into_iter()
        .map(|run| RunTimes {
            attempts: run.attempts.len(),
            completed: run.attempts.len() as u32,
            id: run.id,
            case: run.case,
            environment: run.environment,
            state: run.state,
            started_at_unix_seconds: run.started_at_unix_seconds,
            failures: run.failures,
            not_run: run.not_run,
            elapsed_seconds: run.elapsed_seconds,
        })
        .collect();
    runs.sort_by(|left, right| {
        left.started_at_unix_seconds
            .cmp(&right.started_at_unix_seconds)
            .then(left.id.cmp(&right.id))
    });
    Ok(runs)
}

fn new_run(
    case: String,
    times: u32,
    interval_seconds: u64,
    environment: String,
    authority: String,
    started_at_unix_seconds: u64,
) -> Run {
    Run {
        id: String::new(),
        path: String::new(),
        aliases: vec![],
        case,
        times,
        interval_seconds,
        environment,
        authority,
        state: "running".into(),
        failures: 0,
        not_run: 0,
        attempts: vec![],
        bugs: vec![],
        elapsed_seconds: 0,
        started_at_unix_seconds,
        completed_at_unix_seconds: None,
    }
}

pub(super) fn run(root: &Path, command: ValidateCommand) -> Result<()> {
    match command {
        ValidateCommand::Run {
            case,
            times,
            interval,
            environment,
        } => {
            ensure!((1..=1_000).contains(&times), "times must be 1 to 1000");
            ensure!(interval <= 3_600, "interval is at most 3600 seconds");
            ensure!(
                !environment.trim().is_empty(),
                "environment must not be blank"
            );
            let authority = authority(root)?;
            let located = cases::find(root, &case)?;
            ensure!(
                located.case.bind.is_some(),
                "{case} has no binding to repeat"
            );
            let run = create(
                root,
                new_run(case, times, interval, environment, authority, now()?),
            )?;
            continue_run(root, run)
        }
        ValidateCommand::Stop => {
            write_new(
                root,
                STOP_REQUEST,
                "Stop requested; repeated runs halt before their next attempt.\n",
            )?;
            print_json(&json!({"schema_version": 1, "stop_requested": STOP_REQUEST}))
        }
        ValidateCommand::Resume { id } => {
            let run = load(root, &id)?;
            ensure!(run.state != "complete", "run {id} is already complete");
            let authority = authority(root)?;
            let stop = local_path(root, STOP_REQUEST)?;
            if stop.exists() {
                fs::remove_file(stop)?;
            }
            let mut run = run;
            run.authority = authority;
            continue_run(root, run)
        }
        ValidateCommand::Status => {
            let labels = crate::evidence::Labels::new(root);
            let mut runs = vec![];
            for run in all_runs(root)? {
                let mut observation_labels = vec![];
                for attempt in &run.attempts {
                    observation_labels.push(json!({
                        "id": attempt.observation,
                        "label": labels.label(root, &attempt.observation, Some(&run.environment))?,
                    }));
                }
                // The stored counts are kept as recorded; labels never recount them.
                let mut value = run.view();
                value["observation_labels"] = json!(observation_labels);
                runs.push(value);
            }
            print_json(&json!({
                "schema_version": 1,
                "stop_requested": local_path(root, STOP_REQUEST)?.exists(),
                "runs": runs,
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A project with one open session and its pointer, as `session new` leaves it.
    fn project() -> tempfile::TempDir {
        let root = tempfile::tempdir().unwrap();
        let id = "2026-01-01-alpha";
        let dir = root.path().join(format!(".chough/sessions/{id}"));
        fs::create_dir_all(dir.join("evidence")).unwrap();
        fs::write(
            dir.join("session.md"),
            "---\nkind: session\nid: 2026-01-01-alpha\n---\n# Alpha\n",
        )
        .unwrap();
        fs::create_dir_all(root.path().join(".chough/local")).unwrap();
        fs::write(
            root.path().join(".chough/local/active-session"),
            format!("{id}\n"),
        )
        .unwrap();
        root
    }

    fn started(root: &Path, at: u64) -> Result<Run> {
        create(
            root,
            new_run(
                "TC-cli-0001".into(),
                2,
                0,
                "local".into(),
                "test".into(),
                at,
            ),
        )
    }

    /// A run is one entry named from its case ID and `run`; a same-day second run gets
    /// one clash suffix, and a third that draws the same suffix is refused with nothing
    /// written and no other draw (REQ-records-003, REQ-cli-023).
    #[test]
    fn runs_are_named_from_their_case_and_a_clash_draws_once() {
        let root = project();
        let first = started(root.path(), 86_400 * 20_454).unwrap();
        assert_eq!(first.id, "2026-01-01-tc-cli-0001-run");
        assert!(
            first
                .path
                .ends_with("/evidence/2026-01-01-tc-cli-0001-run.md"),
            "{}",
            first.path
        );
        let original = fs::read(root.path().join(&first.path)).unwrap();
        let second = crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            started(root.path(), 86_400 * 20_454 + 5).unwrap()
        });
        assert_ne!(second.id, first.id);
        assert!(
            second.id.starts_with("2026-01-01-tc-cli-0001-run-"),
            "{}",
            second.id
        );
        let error = crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            started(root.path(), 86_400 * 20_454 + 9).err().unwrap()
        });
        assert!(format!("{error:#}").contains("also exists"), "{error:#}");
        assert_eq!(crate::identity::random_draws(), 1);
        assert_eq!(fs::read(root.path().join(&first.path)).unwrap(), original);
    }

    /// A run's entry reads back as it was saved, attempts included, and resolves by its
    /// ID and by an alias such as a migrated 0.5.x `VAL-` ID.
    #[test]
    fn a_run_entry_round_trips_and_resolves_by_id_or_alias() {
        let root = project();
        let mut run = started(root.path(), 86_400 * 20_454).unwrap();
        run.aliases = vec!["VAL-tc-cli-0001-000000000000000000000001".into()];
        run.attempts.push(Attempt {
            started: 86_400 * 20_454 + 1,
            observed: 86_400 * 20_454 + 3,
            result: "failed".into(),
            duration_seconds: Some(2),
            observation: "2026-01-01-tc-cli-0001".into(),
        });
        run.failures = 1;
        run.state = "stopped".into();
        save(root.path(), &run).unwrap();
        for id in [run.id.as_str(), "VAL-tc-cli-0001-000000000000000000000001"] {
            let back = load(root.path(), id).unwrap();
            assert_eq!(back.attempts, run.attempts);
            assert_eq!((back.failures, back.state.as_str()), (1, "stopped"));
        }
        assert!(load(root.path(), "VAL-unknown").is_err());
        let times = run_times(root.path()).unwrap();
        assert_eq!((times.len(), times[0].attempts), (1, 1));
    }

    /// A migrated 0.5.x run reads back as a run, and resolves by its old ID.
    #[test]
    fn a_migrated_run_reads_back_and_resolves_by_its_old_id() {
        let root = project();
        let legacy = legacy_run(
            "x.json",
            r#"{"schema_version":1,"id":"VAL-tc-cli-0001-000000000000000000000001","case":"TC-cli-0001","times":2,"interval_seconds":0,"environment":"local","authority":"standing","state":"stopped","completed":1,"failures":0,"not_run":0,"observations":["OBS-a"],"bugs":[],"elapsed_seconds":4,"started_at_unix_seconds":1767225600}"#,
        )
        .unwrap();
        let text = migrated_run_text(
            &legacy,
            "2026-01-01-tc-cli-0001-run",
            "standing",
            0,
            vec![MigratedAttempt {
                observation: "2026-01-01-tc-cli-0001".into(),
                started: 1_767_225_601,
                observed: 1_767_225_603,
                result: "passed".into(),
                duration_seconds: Some(2),
            }],
            "",
        )
        .unwrap();
        fs::write(
            root.path()
                .join(".chough/sessions/2026-01-01-alpha/evidence/2026-01-01-tc-cli-0001-run.md"),
            text,
        )
        .unwrap();
        let run = load(root.path(), "VAL-tc-cli-0001-000000000000000000000001").unwrap();
        assert_eq!(run.id, "2026-01-01-tc-cli-0001-run");
        assert_eq!(
            (run.times, run.attempts.len(), run.state.as_str()),
            (2, 1, "stopped")
        );
        assert_eq!(run.started_at_unix_seconds, 1_767_225_600);
    }

    /// Timing orders runs by their start time, never by name.
    #[test]
    fn timing_uses_timestamps_instead_of_name_order() {
        let root = project();
        let later = started(root.path(), 86_400 * 20_454 + 100).unwrap();
        let mut other = new_run(
            "TC-cli-0000".into(),
            1,
            0,
            "local".into(),
            "test".into(),
            86_400 * 20_454 + 10,
        );
        other = create(root.path(), other).unwrap();
        assert!(other.id < later.id);
        let times = run_times(root.path()).unwrap();
        assert_eq!(times[0].id, other.id);
        assert_eq!(times[1].id, later.id);
    }

    /// The migration reads a 0.5.x run as 0.5.0 wrote it, and refuses anything else
    /// naming its path.
    #[test]
    fn a_legacy_run_reads_as_written_and_refuses_naming_its_path() {
        let path = ".chough/validation/VAL-x-000000000000000000000001.json";
        let text = r#"{"schema_version":1,"id":"VAL-x-000000000000000000000001","case":"TC-cli-x","times":3,"interval_seconds":0,"environment":"local","authority":"standing","state":"completed","completed":3,"failures":1,"not_run":0,"observations":["OBS-a","OBS-b","OBS-c"],"bugs":[],"elapsed_seconds":12,"started_at_unix_seconds":1793527200}"#;
        let run = legacy_run(path, text).unwrap();
        assert_eq!(run.id, "VAL-x-000000000000000000000001");
        assert_eq!(run.case, "TC-cli-x");
        assert_eq!(
            (run.times, run.completed, run.failures, run.not_run),
            (3, 3, 1, 0)
        );
        assert_eq!(run.observations, ["OBS-a", "OBS-b", "OBS-c"]);
        assert_eq!(
            (run.elapsed_seconds, run.started_at_unix_seconds),
            (12, 1793527200)
        );
        assert_eq!(
            (run.environment.as_str(), run.state.as_str()),
            ("local", "completed")
        );
        for bad in [
            "{}",
            "not json",
            &text.replace("\"schema_version\":1", "\"schema_version\":2"),
        ] {
            let refused = format!("{:#}", legacy_run(path, bad).unwrap_err());
            assert!(refused.contains(path), "{refused}");
        }
    }
}
