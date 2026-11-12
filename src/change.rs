//! SpecSync change steps through Chough (REQ-cli-057, DRIVE-20): `chough change new`,
//! `answer`, `check` and `status` run the matching SpecSync step in the project root
//! and pass its standard output, standard error and exit code through unchanged.
//! Nothing under `.chough/` is read or written. `chough change ship` (REQ-cli-059)
//! checks the session, its stage, its open gates and the review relay first, ships,
//! and then closes the review gate and moves the session to done with Chough's own
//! commands.

use crate::records::{self, Limits};
use anyhow::{Result, bail};
use clap::Subcommand;
use std::ffi::OsStr;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::time::Duration;

/// The SpecSync program, the one Chough's native reads run.
const SPECSYNC: &str = "specsync";
/// The variable and default of the native read bound, as for every native read.
const NATIVE_TIMEOUT_VARIABLE: &str = "CHOUGH_NATIVE_TIMEOUT_SECONDS";
const NATIVE_DEADLINE: Duration = Duration::from_secs(120);
const INSTALL: &str = "cargo install --locked specsync@6.3.0";

#[derive(Subcommand)]
pub(super) enum ChangeCommand {
    /// Run `specsync change new <description> … --json`.
    New {
        description: String,
        #[arg(long)]
        kind: Option<String>,
        #[arg(long = "spec")]
        specs: Vec<String>,
        #[arg(long = "path")]
        paths: Vec<String>,
        /// Declare that canonical specs do not change; needs --rationale.
        #[arg(long, requires = "rationale")]
        no_spec_change: bool,
        /// Why no canonical spec changes.
        #[arg(long, requires = "no_spec_change", allow_hyphen_values = true)]
        rationale: Option<String>,
    },
    /// Run `specsync change answer <id> <question> <answer> --json`.
    Answer {
        id: String,
        question: String,
        answer: String,
    },
    /// Run `specsync change check <id> --commit`, or without `--commit` under
    /// `--no-commit`.
    Check {
        id: String,
        /// Run the check without `--commit`, as the accept step does first to
        /// materialize requirement text (REQ-cli-057).
        #[arg(long)]
        no_commit: bool,
    },
    /// Run `specsync change status [<id>] --json` within the native read bound.
    Status { id: Option<String> },
    /// Check first, run `specsync change ship <id>`, then close the review gate with its
    /// relay and move the session to done (REQ-cli-059).
    Ship(Box<ShipArgs>),
}

/// `chough change ship`: the change, and the relay of the human decision that closes
/// its open review gate after the ship.
#[derive(clap::Args)]
pub(super) struct ShipArgs {
    id: String,
    /// The open gate the relay closes after the ship, by its exact text.
    #[arg(long, conflicts_with = "list")]
    close_gate: Option<String>,
    /// A `chough gate list` list holding the gate, in place of --close-gate
    /// (REQ-cli-061); the relay then keeps that gate's summary digest.
    #[arg(long, requires = "gate")]
    list: Option<String>,
    /// The gate's number on --list.
    #[arg(long, requires = "list")]
    gate: Option<usize>,
    /// Who decided the gate.
    #[arg(long)]
    decided_by: Option<String>,
    /// Their exact words.
    #[arg(long)]
    words: Option<String>,
    /// Where and when they said them, and who relayed them.
    #[arg(long = "where")]
    place: Option<String>,
    /// The agent recording the decision.
    #[arg(long)]
    recorded_by: Option<String>,
}

/// Run one step and return the exit code to end with.
pub(super) fn run(root: &Path, command: ChangeCommand) -> Result<i32> {
    let (step, arguments, read) = match command {
        ChangeCommand::New {
            description,
            kind,
            specs,
            paths,
            no_spec_change,
            rationale,
        } => {
            let mut arguments = vec!["change".to_string(), "new".into(), description];
            if let Some(kind) = kind {
                arguments.extend(["--kind".into(), kind]);
            }
            for spec in specs {
                arguments.extend(["--spec".into(), spec]);
            }
            for path in paths {
                arguments.extend(["--path".into(), path]);
            }
            if no_spec_change {
                arguments.push("--no-spec-change".into());
            }
            if let Some(rationale) = rationale {
                arguments.extend(["--rationale".into(), rationale]);
            }
            arguments.push("--json".into());
            ("new", arguments, false)
        }
        ChangeCommand::Answer {
            id,
            question,
            answer,
        } => (
            "answer",
            vec![
                "change".into(),
                "answer".into(),
                id,
                question,
                answer,
                "--json".into(),
            ],
            false,
        ),
        ChangeCommand::Check { id, no_commit } => {
            let mut arguments = vec!["change".to_string(), "check".into(), id];
            if !no_commit {
                arguments.push("--commit".into());
            }
            ("check", arguments, false)
        }
        ChangeCommand::Ship(args) => return ship(root, *args),
        ChangeCommand::Status { id } => {
            let mut arguments = vec!["change".to_string(), "status".into()];
            arguments.extend(id);
            arguments.push("--json".into());
            ("status", arguments, true)
        }
    };
    let rerun = rerun(step, &arguments);
    if read {
        status(root, &arguments, &rerun)
    } else {
        write(root, step, &arguments)
    }
}

/// The change a write step names: the ID argument of `answer` and `check`.
fn named_change<'a>(step: &str, arguments: &'a [String]) -> Option<&'a str> {
    matches!(step, "answer" | "check")
        .then(|| arguments.get(2).map(String::as_str))
        .flatten()
}

/// The `chough change status` command a retry runs, as typed.
fn rerun(step: &str, arguments: &[String]) -> String {
    let mut command = format!("chough change {step}");
    if step == "status"
        && let Some(id) = arguments.get(2).filter(|argument| *argument != "--json")
    {
        command.push(' ');
        command.push_str(id);
    }
    command
}

/// The refusal for a SpecSync that cannot be started.
fn not_started(step: &str, error: &io::Error) -> anyhow::Error {
    anyhow::anyhow!(
        "specsync could not be started ({error}), so nothing ran. Install it with `{INSTALL}` (README, Install the toolchain), then run `chough change {step}` again with the same arguments."
    )
}

/// A write step: never stopped by the native read bound. A stop signal to Chough still
/// stops SpecSync and its descendants (FLOW-14).
fn write(root: &Path, step: &str, arguments: &[String]) -> Result<i32> {
    // REQ-cli-060: what the records were before this step, against the baseline.
    let before = named_change(step, arguments)
        .and_then(|change| crate::direct_writes::before_step(root, change));
    let mut command = Command::new(SPECSYNC);
    command.args(arguments).current_dir(root);
    let output =
        records::run_owned(&mut command, None).map_err(|error| not_started(step, &error))?;
    // The step was Chough's, whatever its exit: the baseline follows it. A failed
    // baseline write is not reported here, so SpecSync's bytes stay unchanged.
    if let Some(before) = before {
        let _ = crate::direct_writes::after_step(root, before);
    } else if step == "new"
        && let Some(change) = serde_json::from_slice::<serde_json::Value>(&output.stdout)
            .ok()
            .and_then(|value| {
                value
                    .pointer("/change/id")
                    .and_then(|id| id.as_str())
                    .map(String::from)
            })
    {
        let _ = crate::direct_writes::created(root, &change);
    }
    pass_through(&output.stdout, &output.stderr)?;
    Ok(exit_code(output.status.code(), signal(&output.status)))
}

/// The status read, within the native read bound.
fn status(root: &Path, arguments: &[String], rerun: &str) -> Result<i32> {
    let deadline = match native_deadline(std::env::var_os(NATIVE_TIMEOUT_VARIABLE).as_deref()) {
        Ok(deadline) => deadline,
        Err(got) => bail!(
            "{NATIVE_TIMEOUT_VARIABLE} must be a whole number of seconds from 1 to 3600; got {got:?}, so specsync was not run. Run `{NATIVE_TIMEOUT_VARIABLE}=120 {rerun}`."
        ),
    };
    let mut argv = vec![SPECSYNC.to_string()];
    argv.extend(arguments.iter().cloned());
    let limits = Limits {
        stdout_cap: usize::MAX,
        stderr_cap: usize::MAX,
        deadline,
    };
    let output = records::run_bounded(root, &argv, &[], &limits)
        .map_err(|error| not_started("status", &error))?;
    if output.timed_out {
        let longer = (deadline.as_secs() * 4).min(3_600);
        bail!(
            "specsync change status gave no response within {} s ({NATIVE_TIMEOUT_VARIABLE}) and was stopped; nothing was recorded. Run `{NATIVE_TIMEOUT_VARIABLE}={longer} {rerun}` to wait longer.",
            deadline.as_secs()
        );
    }
    pass_through(&output.stdout, &output.stderr)?;
    Ok(exit_code(output.exit_code, output.signal))
}

/// `chough change ship` (REQ-cli-059): every check first, with nothing run or written;
/// then the ship; then the gate close and the move into done, each as Chough's own
/// command.
fn ship(root: &Path, args: ShipArgs) -> Result<i32> {
    let id = &args.id;
    let relay_command = |gate: &str| {
        format!(
            "chough change ship {id} --close-gate \"{gate}\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\""
        )
    };
    // REQ-cli-059 (modified): --list and --gate name a listed gate, whose summary must
    // still have its listed digest, before anything runs.
    let listed = match (&args.list, args.gate) {
        (Some(list), Some(number)) => Some((
            list.clone(),
            crate::gates::check_listed(root, list, number)?,
        )),
        _ => None,
    };
    let close_gate = listed
        .as_ref()
        .map(|(_, listed)| listed.gate.clone())
        .or_else(|| args.close_gate.clone());
    let session = crate::checkpoint::read_session(root)?.to_value()?;
    match session["native_change"].as_str() {
        None => bail!(
            "refused:no-native-change: the active session records no native change, so nothing ran.\nRun `chough checkpoint --native-change {id}`, then `chough change ship {id}` again."
        ),
        Some(other) if other != id => bail!(
            "refused:other-change: the active session's native change is {other}, not {id}, so nothing ran.\nRun `chough change ship {other}` to ship it, or `chough session use <key>` to make {id}'s session active."
        ),
        Some(_) => {}
    }
    let stage = session["stage"]["current"].as_str().unwrap_or_default();
    if stage != "accept" {
        bail!(
            "refused:not-at-accept: ship runs at accept, and the active session is at {stage}, so nothing ran.\nRun `chough stage advance --to accept`, then `chough change ship {id}` again."
        );
    }
    let open: Vec<String> = session["awaiting_human"]
        .as_array()
        .map(|gates| {
            gates
                .iter()
                .filter_map(|gate| gate.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();
    let named = close_gate.as_deref();
    if let Some(other) = open.iter().find(|gate| Some(gate.as_str()) != named) {
        if named.is_some_and(|named| open.iter().any(|gate| gate == named)) {
            bail!(
                "refused:other-gates-open: ship closes only the gate it names, and gate \"{other}\" is open too, so nothing ran.\nDecide it first with `chough checkpoint --close-gate \"{other}\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"{}`, then run `chough change ship {id}` with the review relay again.",
                other_gate_flags(root, other)
            );
        }
        bail!(
            "refused:gate-open: gate \"{other}\" is open, and ship closes it after shipping only with its relay, so nothing ran.\nRun `{}`.",
            relay_command(other)
        );
    }
    let values = [
        ("--decided-by", args.decided_by.as_deref()),
        ("--words", args.words.as_deref()),
        ("--where", args.place.as_deref()),
        ("--recorded-by", args.recorded_by.as_deref()),
    ];
    let mut relayed = crate::checkpoint::check_relay(&open, named, values).map_err(|problem| {
        use crate::checkpoint::RelayProblem;
        match problem {
            RelayProblem::WithoutGate(given) => anyhow::anyhow!(
                "refused:relay-without-gate: {} describe a human's decision at a gate, and no gate is open, so nothing ran.\nRun `chough change ship {id}` without them.",
                given.join(", ")
            ),
            RelayProblem::Incomplete(gate, missing) => anyhow::anyhow!(
                "refused:relay-incomplete: closing gate \"{gate}\" after the ship needs who decided, their words, where they said them and the recording agent; {} {} missing or blank, so nothing ran.\nRun `{}`.",
                missing.join(", "),
                if missing.len() == 1 { "is" } else { "are" },
                relay_command(&gate)
            ),
            RelayProblem::NotOpen(gate) => anyhow::anyhow!(
                "refused:gate-not-open: no gate is open, so there is no gate \"{gate}\" to close, and nothing ran.\nRun `chough change ship {id}` without --close-gate and the relay."
            ),
            RelayProblem::TooLong(gate, length) => anyhow::anyhow!(
                "refused:relay-too-long: the decision for gate \"{gate}\" would be {length} bytes, over the {}-byte checkpoint text bound, so nothing ran.\nShorten --words or --where (keep the full text in a committed file and name it in --where), then run `{}`.",
                crate::stage::RELAY_TEXT_LIMIT,
                relay_command(&gate)
            ),
        }
    })?;

    // REQ-cli-059, REQ-records-008: a gate that drives review ships with the decided
    // human's review, verdict pass since a ship runs only on acceptance, unless a
    // scoped review is already current. The identity is resolved before SpecSync runs,
    // and a refused review stops the ship.
    let mut native_ran = None;
    let mut reviewed: Option<(String, String)> = None;
    if let Some((gate, relay, _)) = relayed.as_mut() {
        crate::checkpoint::guard_relay(relay)?;
        let checkpoint = crate::checkpoint::read_checkpoint_unlocked(root)?;
        let drives = crate::session_md::gate(&checkpoint, gate).drives();
        // REQ-records-008: every close names one configured human, whatever the gate
        // drives, before SpecSync runs.
        let human = crate::humans::resolve(root, &relay.decided_by)?;
        if drives != crate::humans::Drives::None {
            native_ran = Some(if drives != crate::humans::Drives::Review {
                format!(
                    "no native {} ran: change ship records only a review",
                    drives.name()
                )
            } else if review_is_current(root, id) {
                format!("the scoped review of {id} was already current, so no review ran")
            } else {
                let step = crate::humans::Step::Review {
                    verdict: "pass".into(),
                };
                let ran = crate::humans::run(root, id, &human, &step)?;
                if ran.code != 0 {
                    pass_through(&ran.stdout, &ran.stderr)?;
                    return Ok(ran.code);
                }
                pass_through(&[], &ran.stdout)?;
                pass_through(&[], &ran.stderr)?;
                reviewed = Some((
                    format!("specsync change review {id}"),
                    human.specsync_actor.clone(),
                ));
                step.summary(id, &human)
            });
        }
    }

    // REQ-cli-060: a direct write before the ship stays pending for the move into done.
    let before = crate::direct_writes::before_step(root, id);
    let mut command = Command::new(SPECSYNC);
    command.args(["change", "ship", id]).current_dir(root);
    let output =
        records::run_owned(&mut command, None).map_err(|error| not_started("ship", &error))?;
    if let Some(before) = before {
        let _ = crate::direct_writes::after_step(root, before);
    }
    pass_through(&output.stdout, &output.stderr)?;
    let code = exit_code(output.status.code(), signal(&output.status));
    if code != 0 {
        return Ok(code);
    }

    // After a successful ship: Chough's own commands, so their checks and records are
    // exactly those of `chough checkpoint --close-gate` and `chough stage advance`.
    let mut steps: Vec<(String, Vec<String>)> = vec![];
    if let Some((gate, relay, _)) = relayed {
        steps.push((
            format!(
                "chough checkpoint --close-gate \"{gate}\" --decided-by \"{}\" --words \"{}\" --where \"{}\" --recorded-by \"{}\"",
                relay.decided_by, relay.words, relay.place, relay.recorded_by
            ),
            vec![
                "checkpoint".into(),
                "--close-gate".into(),
                gate,
                "--decided-by".into(),
                relay.decided_by,
                "--words".into(),
                relay.words,
                "--where".into(),
                relay.place,
                "--recorded-by".into(),
                relay.recorded_by,
            ]
            .into_iter()
            .chain(listed.iter().flat_map(|(list, listed)| {
                [
                    "--summary-digest".to_string(),
                    listed.digest.clone(),
                    "--list".to_string(),
                    list.clone(),
                ]
            }))
            // The review, if any, ran before the ship; the close names it and runs none.
            .chain(
                native_ran
                    .iter()
                    .flat_map(|ran| ["--native-ran".to_string(), ran.clone()]),
            )
            .chain(reviewed.iter().flat_map(|(command, actor)| {
                [
                    "--native-command".to_string(),
                    command.clone(),
                    "--native-actor".to_string(),
                    actor.clone(),
                ]
            }))
            .collect(),
        ));
    }
    steps.push((
        "chough stage advance --to done".into(),
        ["stage", "advance", "--to", "done"]
            .map(String::from)
            .to_vec(),
    ));
    let chough = std::env::current_exe()?;
    for (index, (shown, arguments)) in steps.iter().enumerate() {
        let mut command = Command::new(&chough);
        command
            .arg("--root")
            .arg(root)
            .args(arguments)
            .current_dir(root);
        let output = records::run_owned(&mut command, None)?;
        pass_through(&output.stdout, &output.stderr)?;
        if !output.status.success() {
            let remaining: Vec<&str> = steps[index..]
                .iter()
                .map(|(shown, _)| shown.as_str())
                .collect();
            bail!(
                "specsync change ship {id} succeeded, so the change is shipped; `{shown}` was refused. Run `{}`.",
                remaining.join("`, then `")
            );
        }
    }
    Ok(0)
}

/// The flags closing `gate` needs beyond the relay: a gate that drives review takes
/// the human's verdict, or `--no-native` for a decision that is not a review
/// (REQ-records-008).
fn other_gate_flags(root: &Path, gate: &str) -> &'static str {
    let drives = crate::checkpoint::read_checkpoint_unlocked(root)
        .map(|session| crate::session_md::gate(&session, gate).drives());
    match drives {
        Ok(crate::humans::Drives::Review) => {
            " --verdict <pass|block> (or --no-native for a decision that is not a review)"
        }
        _ => "",
    }
}

/// Whether SpecSync reports a current scoped review for `id`; an unreadable status
/// counts as not current, so the review runs and SpecSync decides.
fn review_is_current(root: &Path, id: &str) -> bool {
    records::native_json(root, "specsync", &["change", "status", id, "--json"])
        .value
        .is_some_and(|value| {
            value.pointer("/summary/scoped_review_current") == Some(&serde_json::Value::Bool(true))
        })
}

/// The native read bound: 120 s unless the variable names a whole number of seconds
/// from 1 to 3600, as for Chough's other native reads; anything else is refused with
/// the value it got.
fn native_deadline(value: Option<&OsStr>) -> Result<Duration, String> {
    let Some(value) = value else {
        return Ok(NATIVE_DEADLINE);
    };
    value
        .to_str()
        .and_then(|text| text.parse::<u64>().ok())
        .filter(|seconds| (1..=3_600).contains(seconds))
        .map(Duration::from_secs)
        .ok_or_else(|| value.to_string_lossy().chars().take(64).collect::<String>())
}

/// Copy SpecSync's bytes to Chough's own streams, unchanged.
fn pass_through(stdout: &[u8], stderr: &[u8]) -> Result<()> {
    let mut out = io::stdout().lock();
    out.write_all(stdout)?;
    out.flush()?;
    let mut err = io::stderr().lock();
    err.write_all(stderr)?;
    err.flush()?;
    Ok(())
}

/// SpecSync's exit code; a SpecSync ended by a signal ends Chough with 128 plus that
/// signal, as a shell reports it.
fn exit_code(code: Option<i32>, signal: Option<i32>) -> i32 {
    code.unwrap_or_else(|| 128 + signal.unwrap_or(0))
}

#[cfg(unix)]
fn signal(status: &std::process::ExitStatus) -> Option<i32> {
    use std::os::unix::process::ExitStatusExt;
    status.signal()
}

#[cfg(not(unix))]
fn signal(_: &std::process::ExitStatus) -> Option<i32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_native_bound_accepts_one_to_3600_seconds_and_defaults_to_120() {
        assert_eq!(native_deadline(None), Ok(NATIVE_DEADLINE));
        assert_eq!(
            native_deadline(Some(OsStr::new("1"))),
            Ok(Duration::from_secs(1))
        );
        assert_eq!(
            native_deadline(Some(OsStr::new("3600"))),
            Ok(Duration::from_secs(3_600))
        );
        for invalid in ["0", "3601", "-1", "1.5", "", "soon"] {
            assert!(
                native_deadline(Some(OsStr::new(invalid))).is_err(),
                "{invalid}"
            );
        }
    }

    #[test]
    fn a_retry_names_the_status_change_only_when_one_was_given() {
        let with = ["change", "status", "free-shipping", "--json"].map(String::from);
        let without = ["change", "status", "--json"].map(String::from);
        assert_eq!(rerun("status", &with), "chough change status free-shipping");
        assert_eq!(rerun("status", &without), "chough change status");
        assert_eq!(rerun("check", &with), "chough change check");
    }
}
