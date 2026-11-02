//! Optional repeated validation in the foreground. It runs only with a run consent or a
//! standing preference, checks for a stop request and a paused checkpoint before every
//! attempt, and never claims soak testing, live health or authority to repair anything.

use crate::bugs::{self, Finding};
use crate::cases;
use crate::checkpoint::read_checkpoint;
use crate::evidence::{self, Outcome};
use crate::prefs;
use crate::records::{
    local_path, now, print_json, read_record, valid_name, write_new, write_replace,
};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

const RUNS_DIR: &str = ".chough/validation";
pub(super) const STOP_REQUEST: &str = ".chough/validation.stop";
/// The longest wait between checks for a stop request or pause during an interval.
const WAIT_SLICE: Duration = Duration::from_millis(200);

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Run {
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
    let session = read_checkpoint(root)?;
    if let Some(origin) = prefs::authorized(root, &session, "repeat-validation")? {
        return Ok(origin);
    }
    let (value, origin) = prefs::effective(root, "repeat-validation")?;
    bail!(
        "repeated validation is optional and not authorized (repeat-validation = {value}, {origin}). Ask the human; record their answer with `chough consent grant repeat-validation --source <their words>` for this objective, or `chough pref set repeat-validation always --source <...>` only if they asked for it to always apply"
    )
}

fn run_path(id: &str) -> String {
    format!("{RUNS_DIR}/{id}.json")
}

fn save(root: &Path, run: &Run, new: bool) -> Result<()> {
    let content = serde_json::to_string_pretty(run)? + "\n";
    if new {
        ensure!(
            write_new(root, &run_path(&run.id), &content)
                .with_context(|| format!("creating validation run {}", run.id))?,
            "validation run {} already exists",
            run.id
        );
        Ok(())
    } else {
        write_replace(root, &run_path(&run.id), &content)
    }
}

fn load(root: &Path, id: &str) -> Result<Run> {
    ensure!(valid_name(id), "invalid run ID");
    let run: Run = serde_json::from_str(
        &read_record(&local_path(root, &run_path(id))?)
            .with_context(|| format!("unknown validation run: {id}"))?,
    )
    .context("invalid validation run record")?;
    ensure!(run.schema_version == 1, "unsupported validation run schema");
    Ok(run)
}

/// The state a repeated run takes when it must not start another attempt now: a stop
/// request stops it, and a paused checkpoint pauses it.
fn halt(root: &Path) -> Result<Option<&'static str>> {
    if local_path(root, STOP_REQUEST)?.exists() {
        return Ok(Some("stopped"));
    }
    if read_checkpoint(root)?.stage.status == "paused" {
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

fn continue_run(root: &Path, mut run: Run) -> Result<()> {
    let started = Instant::now();
    let previous = run.elapsed_seconds;
    run.state = "running".into();
    save(root, &run, false)?;
    while run.completed < run.times {
        if run.completed > 0 && run.interval_seconds > 0 {
            wait_interval(root, run.interval_seconds)?;
        }
        // Checked immediately before every attempt, including after the interval.
        if let Some(state) = halt(root)? {
            run.state = state.into();
            break;
        }
        let result = cases::run_case(root, &run.case, &run.environment, None)?;
        run.completed += 1;
        run.observations.push(result.observation.clone());
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
        save(root, &run, false)?;
    }
    if run.state == "running" {
        run.state = "complete".into();
    }
    run.elapsed_seconds = previous + started.elapsed().as_secs();
    save(root, &run, false)?;
    let (initiation, _) = prefs::effective(root, "monitor-initiation")?;
    let proposal = (run.failures > 0 && initiation == "propose-change").then(|| {
        format!(
            "Proposed only, not run: specsync change new \"Investigate {} failures in validation run {}\" --kind bug-fix",
            run.case, run.id
        )
    });
    let retention = evidence::auto_prune(root);
    print_json(&json!({
        "schema_version": 1,
        "run": run,
        "retention": retention,
        "label": format!(
            "repeated run: {} of {} attempts, {} failed, {} not run, {} s elapsed; this is not a soak test, live-health check or repair authorization",
            run.completed, run.times, run.failures, run.not_run, run.elapsed_seconds
        ),
        "captured_locally": run.bugs,
        "proposal": proposal,
    }))
}

/// Each stored run's ID with the observations it recorded, in attempt order.
pub(super) fn run_references(root: &Path) -> Result<Vec<(String, Vec<String>)>> {
    let mut runs = vec![];
    for name in crate::records::record_names(root, RUNS_DIR)? {
        let run = load(root, name.trim_end_matches(".json"))?;
        runs.push((run.id, run.observations));
    }
    Ok(runs)
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
    let mut runs = vec![];
    for name in crate::records::record_names(root, RUNS_DIR)? {
        let run = load(root, name.trim_end_matches(".json"))?;
        runs.push(RunTimes {
            attempts: run.observations.len(),
            id: run.id,
            case: run.case,
            environment: run.environment,
            state: run.state,
            started_at_unix_seconds: run.started_at_unix_seconds,
            completed: run.completed,
            failures: run.failures,
            not_run: run.not_run,
            elapsed_seconds: run.elapsed_seconds,
        });
    }
    runs.sort_by(|left, right| {
        left.started_at_unix_seconds
            .cmp(&right.started_at_unix_seconds)
            .then(left.id.cmp(&right.id))
    });
    Ok(runs)
}

fn create_run(
    root: &Path,
    case: String,
    times: u32,
    interval_seconds: u64,
    environment: String,
    authority: String,
    started_at_unix_seconds: u64,
) -> Result<Run> {
    let id = crate::identity::validation(&case)?;
    ensure!(
        crate::identity::valid_validation_id(&id),
        "invalid allocated validation run ID"
    );
    let run = Run {
        schema_version: 1,
        id,
        case,
        times,
        interval_seconds,
        environment,
        authority,
        state: "running".into(),
        completed: 0,
        failures: 0,
        not_run: 0,
        observations: vec![],
        bugs: vec![],
        elapsed_seconds: 0,
        started_at_unix_seconds,
    };
    save(root, &run, true)?;
    Ok(run)
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
            let run = create_run(root, case, times, interval, environment, authority, now()?)?;
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
            let labels = evidence::Labels::new(root);
            let mut runs = vec![];
            for name in crate::records::record_names(root, RUNS_DIR)? {
                let run = load(root, name.trim_end_matches(".json"))?;
                let mut observation_labels = vec![];
                for id in &run.observations {
                    observation_labels.push(json!({
                        "id": id,
                        "label": labels.label(root, id, Some(&run.environment))?,
                    }));
                }
                // The stored counts are kept as recorded; labels never recount them.
                let mut value = serde_json::to_value(&run)?;
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

    fn create(root: &Path, times: u32) -> Result<Run> {
        create_run(
            root,
            "TC-cli-0001".into(),
            times,
            0,
            "local".into(),
            "test".into(),
            10,
        )
    }

    #[test]
    fn validation_collision_preserves_the_original_without_fallback() {
        let root = tempfile::tempdir().unwrap();
        let first = crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            create(root.path(), 1).unwrap()
        });
        let path = root.path().join(run_path(&first.id));
        let original = fs::read(&path).unwrap();

        let error = crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            create(root.path(), 2).err().unwrap()
        });
        assert!(error.to_string().contains(&first.id), "{error:#}");
        assert_eq!(crate::identity::random_draws(), 1);
        assert_eq!(fs::read(&path).unwrap(), original);
        assert_eq!(
            crate::records::record_names(root.path(), RUNS_DIR)
                .unwrap()
                .len(),
            1
        );
    }

    #[test]
    fn timing_uses_timestamps_instead_of_random_id_order() {
        let root = tempfile::tempdir().unwrap();
        let mut later = crate::identity::with_random_results(vec![Ok([0; 12])], || {
            create(root.path(), 1).unwrap()
        });
        later.started_at_unix_seconds = 20;
        save(root.path(), &later, false).unwrap();
        let earlier = crate::identity::with_random_results(vec![Ok([0xff; 12])], || {
            create(root.path(), 1).unwrap()
        });
        assert!(later.id < earlier.id);

        let times = run_times(root.path()).unwrap();
        assert_eq!(times[0].id, earlier.id);
        assert_eq!(times[1].id, later.id);
    }

    #[test]
    fn validation_random_failure_preserves_existing_records() {
        let root = tempfile::tempdir().unwrap();
        let first = crate::identity::with_random_results(vec![Ok([0xab; 12])], || {
            create(root.path(), 1).unwrap()
        });
        let path = root.path().join(run_path(&first.id));
        let original = fs::read(&path).unwrap();

        let error =
            crate::identity::with_random_results(vec![Err("injected random failure")], || {
                create(root.path(), 2).err().unwrap()
            });
        assert!(error.to_string().contains("injected random failure"));
        assert_eq!(crate::identity::random_draws(), 1);
        assert_eq!(fs::read(&path).unwrap(), original);
        assert_eq!(
            crate::records::record_names(root.path(), RUNS_DIR)
                .unwrap()
                .len(),
            1
        );
    }
}
