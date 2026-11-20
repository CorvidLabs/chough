//! SpecSync change steps through Chough (REQ-cli-057, DRIVE-20): `chough change new`,
//! `answer`, `check` and `status` run the matching SpecSync step in the project root
//! and pass its standard output, standard error and exit code through unchanged.
//! Nothing under `.chough/` is read or written. `chough change ready` (REQ-cli-067)
//! is not one of those steps: it prints its own report and writes nothing.
//! `chough change ship` (REQ-cli-059)
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
    /// List everything still missing before approval, and name each fix (REQ-cli-067).
    Ready { id: String },
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
    /// Print each step's full report after SpecSync's output, in place of the short
    /// text summary (REQ-cli-059).
    #[arg(long)]
    json: bool,
}

/// The most lines the short summary after SpecSync's output takes (REQ-cli-059).
const SUMMARY_LINES: usize = 12;

/// Run one step and return the exit code to end with.
/// REQ-cli-057 (G1, #367): an argument SpecSync would write or echo back is checked
/// before SpecSync runs. One holding a credential, a personal value or a control
/// character is refused, naming the argument and what it holds and never the value;
/// `--redact` does not change this, since SpecSync, not Chough, would write it.
fn refuse_unshowable(arguments: &[(&str, &str)]) -> Result<()> {
    for (name, value) in arguments {
        if let Some(held) = crate::leaks::unshowable(value) {
            bail!(
                "refused:unshowable-argument: the {name} argument holds {held}, so SpecSync was not run and nothing was written.\nGive the {name} without it and run the command again."
            );
        }
    }
    Ok(())
}

pub(super) fn run(root: &Path, command: ChangeCommand) -> Result<i32> {
    match &command {
        ChangeCommand::New {
            kind, specs, paths, ..
        } => {
            let mut checked: Vec<(&str, &str)> = vec![];
            if let Some(kind) = kind {
                checked.push(("kind", kind));
            }
            checked.extend(specs.iter().map(|spec| ("spec", spec.as_str())));
            checked.extend(paths.iter().map(|path| ("path", path.as_str())));
            refuse_unshowable(&checked)?;
        }
        ChangeCommand::Answer { id, question, .. } => {
            refuse_unshowable(&[("id", id), ("question", question)])?;
        }
        ChangeCommand::Check { id, .. } => refuse_unshowable(&[("id", id)])?,
        ChangeCommand::Status { id: Some(id) } => refuse_unshowable(&[("id", id)])?,
        _ => {}
    }
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
        ChangeCommand::Ready { id } => return crate::ready::run(root, &id),
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
    // REQ-records-006 (#367): every message before the id is checked against the
    // session shows it only when it is clean.
    let id_shown = crate::leaks::shown("id", id);
    let relay_command = |gate: &str| {
        format!(
            "chough change ship {id_shown} --close-gate \"{gate}\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\""
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
            "refused:no-native-change: the active session records no native change, so nothing ran.\nRun `chough checkpoint --native-change {id_shown}`, then `chough change ship {id_shown}` again."
        ),
        Some(other) if other != id => bail!(
            "refused:other-change: the active session's native change is {other}, not {id_shown}, so nothing ran.\nRun `chough change ship {other}` to ship it, or `chough session use <key>` to make {id_shown}'s session active."
        ),
        Some(_) => {}
    }
    let stage = session["stage"]["current"].as_str().unwrap_or_default();
    if stage != "accept" {
        bail!(
            "refused:not-at-accept: ship runs at accept, and the active session is at {stage}, so nothing ran.\nRun `chough stage advance --to accept`, then `chough change ship {id_shown}` again."
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
                "refused:other-gates-open: ship closes only the gate it names, and gate \"{other}\" is open too, so nothing ran.\nDecide it first with `chough checkpoint --close-gate \"{other}\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"{}`, then run `chough change ship {id_shown}` with the review relay again.",
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
                "refused:relay-without-gate: {} describe a human's decision at a gate, and no gate is open, so nothing ran.\nRun `chough change ship {id_shown}` without them.",
                given.join(", ")
            ),
            RelayProblem::Incomplete(gate, missing) => anyhow::anyhow!(
                "refused:relay-incomplete: closing gate \"{gate}\" after the ship needs who decided, their words, where they said them and the recording agent; {} {} missing or blank, so nothing ran.\nRun `{}`.",
                missing.join(", "),
                if missing.len() == 1 { "is" } else { "are" },
                relay_command(&gate)
            ),
            RelayProblem::NotOpen(gate, _) => anyhow::anyhow!(
                "refused:gate-not-open: no gate is open, so there is no gate \"{gate}\" to close, and nothing ran.\nRun `chough change ship {id_shown}` without --close-gate and the relay."
            ),
            RelayProblem::TooLong(gate, length) => anyhow::anyhow!(
                "refused:relay-too-long: the decision for gate \"{gate}\" would be {length} bytes, over the {}-byte checkpoint text bound, so nothing ran.\nShorten --words or --where (keep the full text in a committed file and name it in --where), then run `{}`.",
                crate::stage::RELAY_TEXT_LIMIT,
                relay_command(&gate)
            ),
        }
    })?;

    // REQ-cli-059 (#298): refuse before anything runs when the change's history holds a
    // commit that SpecSync reads as corrupt, so nothing is left half done.
    refuse_corrupt_history(root, id)?;

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
                    relay: (&*relay).into(),
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

    // REQ-cli-059 (0.6.1): after SpecSync's output, a short text summary of the review,
    // the ship, the gate close and the move into done; each step's full report under
    // --json instead.
    let review = match (&relayed, &native_ran) {
        (None, _) => "none ran; no gate was named to close".to_string(),
        (Some((gate, _, _)), None) => format!("none ran; gate \"{gate}\" drives no native step"),
        (Some(_), Some(ran)) => ran.clone(),
    };
    let mut summary = vec![
        format!("Review: {review}"),
        format!("Ship: specsync change ship {id} succeeded; its output is above"),
    ];
    let gate_closed = match &relayed {
        Some((gate, relay, _)) => format!(
            "Gate: \"{gate}\" closed with {}'s decision, recorded by {} as a relayed claim",
            relay.decided_by, relay.recorded_by
        ),
        None => "Gate: none was named, so none closed".to_string(),
    };
    if relayed.is_none() {
        summary.push(gate_closed.clone());
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
    // The move prints its full report under --json, and its short text summary, which
    // ends this one, otherwise.
    let mut advance = ["stage", "advance", "--to", "done"]
        .map(String::from)
        .to_vec();
    if args.json {
        advance.push("--json".into());
    }
    steps.push(("chough stage advance --to done".into(), advance));
    let chough = std::env::current_exe()?;
    // REQ-cli-059 (#369): the gate close and the move into done act on the session this
    // command resolved, pinned or not, never on whatever the shared pointer says now.
    let session = crate::sessions::id_of(&crate::sessions::checkpoint_path(root)?);
    for (index, (shown, arguments)) in steps.iter().enumerate() {
        let mut command = Command::new(&chough);
        command.arg("--root").arg(root);
        if let Some(session) = &session {
            command.arg("--session").arg(session);
        }
        command.args(arguments).current_dir(root);
        let output = records::run_owned(&mut command, None)?;
        if !output.status.success() {
            if !args.json {
                print_summary(&summary)?;
            }
            pass_through(&output.stdout, &output.stderr)?;
            let remaining: Vec<&str> = steps[index..]
                .iter()
                .map(|(shown, _)| shown.as_str())
                .collect();
            bail!(
                "specsync change ship {id} succeeded, so the change is shipped; `{shown}` was refused. Run `{}`.",
                remaining.join("`, then `")
            );
        }
        if args.json {
            pass_through(&output.stdout, &output.stderr)?;
            continue;
        }
        pass_through(&[], &output.stderr)?;
        if index + 1 < steps.len() {
            summary.push(gate_closed.clone());
        } else {
            summary.extend(
                String::from_utf8_lossy(&output.stdout)
                    .lines()
                    .map(String::from),
            );
        }
    }
    if !args.json {
        print_summary(&summary)?;
    }
    Ok(0)
}

/// The short summary's text, at most [`SUMMARY_LINES`] lines (REQ-cli-059). When
/// the summary is longer, its last line names how many lines were dropped and
/// points at `--json`, which prints every step's full report, so an agent loses
/// nothing.
fn short_summary(lines: &[String]) -> String {
    let mut lines = lines.to_vec();
    if lines.len() > SUMMARY_LINES {
        // The cut's marker is the last line, so a cut keeps the first SUMMARY_LINES - 1
        // lines and names the full report under --json rather than dropping silently.
        let kept = SUMMARY_LINES - 1;
        let dropped = lines.len() - kept;
        lines.truncate(kept);
        lines.push(format!(
            "… {dropped} more lines; run with --json for the full report"
        ));
    }
    lines.join("\n")
}

/// Print the short summary (REQ-cli-059).
fn print_summary(lines: &[String]) -> Result<()> {
    let mut stdout = io::stdout().lock();
    for line in short_summary(lines).lines() {
        writeln!(stdout, "{line}")?;
    }
    stdout.flush()?;
    Ok(())
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

/// Refuse a ship whose change history SpecSync would read as corrupt (REQ-cli-059, #298):
/// a commit between the change's recorded base commit and HEAD that holds another change
/// both open, `.specsync/changes/<id>/state.json`, and archived,
/// `.specsync/archive/changes/<date>-<id>/state.json`, as a merge of main can leave; a
/// leftover file other than `state.json` is not a second state, as SpecSync reads it. SpecSync then
/// reports the archive's terminal evidence as `corrupt-history`, but only after it has
/// archived, so the ship would stop halfway.
///
/// Interim: this repeats one SpecSync rule in Chough until SpecSync offers a validity
/// preflight before the review (CorvidLabs/spec-sync#1082); then Chough calls that
/// instead. With no recorded base commit, or outside Git, there is nothing to check.
fn refuse_corrupt_history(root: &Path, id: &str) -> Result<()> {
    let state = root.join(".specsync/changes").join(id).join("state.json");
    let Some(base) = std::fs::read_to_string(state)
        .ok()
        .and_then(|text| serde_json::from_str::<serde_json::Value>(&text).ok())
        .and_then(|state| state["base_commit"].as_str().map(String::from))
        .filter(|base| !base.is_empty())
    else {
        return Ok(());
    };
    let git = |arguments: &[&str]| -> Option<String> {
        let output = records::run_owned(
            Command::new("git").arg("-C").arg(root).args(arguments),
            None,
        )
        .ok()?;
        output
            .status
            .success()
            .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
    };
    // Outside a Git repository there is nothing to check.
    if git(&["rev-parse", "--git-dir"]).is_none() {
        return Ok(());
    }
    // A base_commit that is not a Git object id (a hand edit, a bad merge of
    // state.json) names no range at all, as a base that is gone does: say so without
    // showing the value, and go on, as before this check existed. Only then is it
    // ASCII, so the short form below is a safe byte slice.
    if base.is_empty() || base.len() > 64 || !base.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        eprintln!(
            "note: the change's recorded base commit is not a Git object id, so its history from that base could not be checked; the ship goes on."
        );
        return Ok(());
    }
    let short_base = &base[..base.len().min(12)];
    // The remote to fetch from, by its real name so the command pastes as printed: the
    // one the current branch tracks, else the only remote there is (the tester's
    // recheck of #413).
    let tracked = git(&["symbolic-ref", "--quiet", "--short", "HEAD"])
        .and_then(|branch| git(&["config", &format!("branch.{}.remote", branch.trim())]))
        .map(|remote| remote.trim().to_string())
        .filter(|remote| !remote.is_empty() && remote != ".");
    let remotes: Vec<String> = git(&["remote"])
        .map(|names| {
            names
                .lines()
                .filter(|name| !name.is_empty())
                .map(String::from)
                .collect()
        })
        .unwrap_or_default();
    let remote = tracked.or_else(|| (remotes.len() == 1).then(|| remotes[0].clone()));
    let refetch = match &remote {
        // A name that cannot be shown is not printed, so there is nothing to paste.
        Some(remote) if crate::leaks::unshowable(remote).is_some() => format!(
            "Run `git fetch --refetch -- <remote>` with the name of {}, to fetch every object again, then the ship.",
            crate::leaks::shown("remote", remote)
        ),
        // One quoted word after `--`, so any name a person's config holds pastes as one
        // argument and is never read as an option or by the shell.
        Some(remote) => format!(
            "Run `git fetch --refetch -- {}` to fetch every object again, then the ship.",
            shell_quoted(remote)
        ),
        // Several remotes and none tracked: they exist, so name a few to choose from.
        None if !remotes.is_empty() => {
            let mut named: Vec<String> = remotes
                .iter()
                .take(5)
                .map(|remote| match crate::leaks::unshowable(remote) {
                    Some(_) => crate::leaks::shown("remote", remote),
                    None => shell_quoted(remote),
                })
                .collect();
            if remotes.len() > 5 {
                named.push(format!("{} more (`git remote`)", remotes.len() - 5));
            }
            format!(
                "Run `git fetch --refetch -- <remote>` with the remote that holds this history, one of {}, to fetch every object again, then the ship.",
                named.join(", ")
            )
        }
        None => "Add the remote that holds this history with `git remote add <name> <url>`, then run `git fetch --refetch -- <name>` to fetch every object again, then the ship.".to_string(),
    };
    let unchecked = |what: &str| {
        anyhow::anyhow!(
            "refused:history-unchecked: Git could not read {what}, so this change's history from its base commit {short_base} to HEAD could not be checked and nothing ran.\n{refetch}"
        )
    };
    // REQ-cli-059 (#400): a base commit that is not an object. In a shallow clone the
    // history is only missing here, so fetching it is the way on; in a complete clone,
    // as after a squash merge removed the base, there is no range to read at all, and
    // refusing would stop the ship for good, so it goes on and says so.
    if git(&["cat-file", "-e", &format!("{base}^{{commit}}")]).is_none() {
        if git(&["rev-parse", "--is-shallow-repository"])
            .as_deref()
            .map(str::trim)
            == Some("true")
        {
            bail!(
                "refused:history-unchecked: this shallow clone does not hold the change's base commit {short_base}, so its history to HEAD could not be checked and nothing ran.\nRun `git fetch --unshallow`, then the ship again."
            );
        }
        eprintln!(
            "note: the change's base commit {short_base} is not in this repository, as after a squash merge removed it, so its history from that base could not be checked; the ship goes on."
        );
        return Ok(());
    }
    // REQ-cli-059 (#400): the whole range, however many commits.
    let range = format!("{base}..HEAD");
    // When a commit object itself cannot be read, `git rev-list` says which on
    // standard error (`error: Could not read <id>`); name that commit, else the range.
    let listed = records::run_owned(
        Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["rev-list", &range]),
        None,
    )
    .map_err(|_| unchecked(&range))?;
    if !listed.status.success() {
        let said = String::from_utf8_lossy(&listed.stderr);
        let lost = said
            .split(|character: char| !character.is_ascii_hexdigit())
            .find(|word| word.len() == 40 || word.len() == 64);
        return Err(match lost {
            Some(lost) => unchecked(&format!("commit {lost}")),
            None => unchecked(&range),
        });
    }
    let commits = String::from_utf8_lossy(&listed.stdout).into_owned();
    // One streaming call gives each commit's two folder trees and its root tree, so
    // commits that leave both folders as they were cost no further Git call.
    let mut asked = String::new();
    for commit in commits.lines() {
        asked.push_str(&format!(
            "{commit}:.specsync/changes\n{commit}:.specsync/archive/changes\n{commit}^{{tree}}\n"
        ));
    }
    let trees = records::run_owned(
        Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["cat-file", "--batch-check"]),
        Some(asked.as_bytes()),
    )
    .ok()
    .filter(|output| output.status.success())
    .map(|output| String::from_utf8_lossy(&output.stdout).into_owned())
    .ok_or_else(|| unchecked("the trees of the commits in that range"))?;
    // A present folder answers `<tree id> tree <size>`; a missing one
    // `<commit>:<path> missing`, which is one value here, "missing", whatever the commit.
    let tree_ids: Vec<&str> = trees
        .lines()
        .map(|line| {
            if line.ends_with(" missing") {
                "missing"
            } else {
                line.split_whitespace().next().unwrap_or_default()
            }
        })
        .collect();
    // Fewer answers than questions would leave commits unread; never pass them.
    if tree_ids.len() != commits.lines().count() * 3 {
        return Err(unchecked("the trees of every commit in that range"));
    }
    let mut by_trees: std::collections::HashMap<(String, String), Vec<String>> =
        std::collections::HashMap::new();
    // `missing` is also Git's answer for a folder that exists but whose own tree cannot
    // be read (corvid-agent's review of #413). `git ls-tree -d` reads only the parent
    // trees, so it still lists such a folder; one listing per distinct root tree.
    let mut folders_by_root: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    // Newest first, as `git rev-list` gives them: the change kept both ways, and every
    // commit in the window that holds it, so the refusal can name the one that brought
    // it in, as SpecSync's own reason does, and say how many hold it.
    let mut found: Option<(String, Vec<String>)> = None;
    for (index, commit) in commits.lines().enumerate() {
        let key = (
            tree_ids[index * 3].to_string(),
            tree_ids[index * 3 + 1].to_string(),
        );
        if key.0 == "missing" || key.1 == "missing" {
            let root_tree = tree_ids[index * 3 + 2];
            if root_tree == "missing" {
                return Err(unchecked(&format!("commit {commit}")));
            }
            let folders = match folders_by_root.get(root_tree) {
                Some(folders) => folders.clone(),
                None => {
                    let folders = git(&[
                        "ls-tree",
                        "-d",
                        "--name-only",
                        commit,
                        "--",
                        ".specsync/changes",
                        ".specsync/archive/changes",
                    ])
                    .ok_or_else(|| unchecked(&format!("commit {commit}")))?;
                    folders_by_root.insert(root_tree.to_string(), folders.clone());
                    folders
                }
            };
            let exists = |path: &str| folders.lines().any(|line| line == path);
            if (key.0 == "missing" && exists(".specsync/changes"))
                || (key.1 == "missing" && exists(".specsync/archive/changes"))
            {
                return Err(unchecked(&format!("commit {commit}")));
            }
        }
        let both: Vec<String> = match by_trees.get(&key) {
            // With either folder absent no change can be both open and archived.
            _ if key.0 == "missing" || key.1 == "missing" => vec![],
            Some(both) => both.clone(),
            None => {
                let both = changes_both_ways(&git, commit, id)
                    .ok_or_else(|| unchecked(&format!("commit {commit}")))?;
                by_trees.insert(key, both.clone());
                both
            }
        };
        let both: Vec<&String> = both.iter().collect();
        match &mut found {
            None => {
                if let Some(first) = both.first() {
                    found = Some(((*first).clone(), vec![commit.to_string()]));
                }
            }
            Some((change, holding)) => {
                if both.iter().any(|both| *both == change) {
                    holding.push(commit.to_string());
                }
            }
        }
    }
    if let Some((both, holding)) = found {
        let short = |commit: &str| commit[..commit.len().min(12)].to_string();
        let first = short(holding.last().map(String::as_str).unwrap_or_default());
        let (held, those) = match holding.len() {
            1 => (
                "it is the only commit that holds it".to_string(),
                "that commit",
            ),
            count => (
                format!("{count} commits from it to {} hold it", short(&holding[0])),
                "those commits",
            ),
        };
        bail!(
            "refused:corrupt-history: commit {first} in this change's history brought in change `{both}` both open (.specsync/changes/{both}/state.json) and archived (under .specsync/archive/changes/), and {held}; SpecSync reads that as corrupt history, so the archive's terminal evidence would not authenticate and the ship would stop halfway; nothing ran. Rebuild the branch from the default branch without {those} (for example, start a new branch from it and bring over this change's own commits), then run `chough change check {id}` and the ship again. (Chough checks this until SpecSync's own preflight, CorvidLabs/spec-sync#1082, exists.)"
        );
    }
    Ok(())
}

/// `value` as one POSIX shell word: in single quotes, each `'` written as `'\''`.
fn shell_quoted(value: &str) -> String {
    format!("'{}'", value.replace('\'', "'\\''"))
}

/// The changes other than `id` that `commit` holds both open and archived, by their
/// `state.json` on each side, as SpecSync counts them (#298, #333); None when Git
/// cannot list that commit's tree.
fn changes_both_ways(
    git: &dyn Fn(&[&str]) -> Option<String>,
    commit: &str,
    id: &str,
) -> Option<Vec<String>> {
    let paths = git(&[
        "ls-tree",
        "-r",
        "--name-only",
        commit,
        "--",
        ".specsync/changes",
        ".specsync/archive/changes",
    ])?;
    // SpecSync's rule counts a change's `state.json` on both sides, not any file: a
    // leftover such as a `tasks.md` beside an archive is not a second state (#333).
    let mut open = std::collections::BTreeSet::new();
    let mut archived = std::collections::BTreeSet::new();
    for path in paths.lines() {
        let Some(folder) = path.strip_suffix("/state.json") else {
            continue;
        };
        if let Some(folder) = folder.strip_prefix(".specsync/archive/changes/") {
            // `<YYYY-MM-DD>-<id>`: the change ID follows the 11-byte date prefix.
            if !folder.contains('/') && folder.len() > 11 && folder.as_bytes()[10] == b'-' {
                archived.insert(folder[11..].to_string());
            }
        } else if let Some(folder) = folder.strip_prefix(".specsync/changes/")
            && !folder.contains('/')
        {
            open.insert(folder.to_string());
        }
    }
    // The change being shipped is left out: its own archive appears only when
    // SpecSync ships it, and a history is poisoned by another change kept both open
    // and archived, as a merge of main leaves (#298).
    Some(
        open.intersection(&archived)
            .filter(|both| both.as_str() != id)
            .cloned()
            .collect(),
    )
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

    /// REQ-cli-059 (#276): a summary longer than [`SUMMARY_LINES`] is cut, and its
    /// last line names the dropped lines and `--json` for the full report.
    #[test]
    fn the_summary_names_the_cut_and_json_when_it_is_longer_than_twelve_lines() {
        let lines: Vec<String> = (1..=20).map(|n| format!("line {n}")).collect();
        let summary = short_summary(&lines);
        let printed: Vec<&str> = summary.lines().collect();
        assert_eq!(printed.len(), SUMMARY_LINES, "{summary}");
        assert_eq!(
            printed[SUMMARY_LINES - 1],
            "… 9 more lines; run with --json for the full report"
        );
        assert!(printed[SUMMARY_LINES - 1].contains("--json"));
        assert_eq!(
            printed[..SUMMARY_LINES - 1],
            lines[..SUMMARY_LINES - 1]
                .iter()
                .map(|line| line.as_str())
                .collect::<Vec<_>>()
        );
        // At the bound and under it, every line is printed whole and in order.
        let short: Vec<String> = (1..=SUMMARY_LINES).map(|n| format!("line {n}")).collect();
        assert_eq!(short_summary(&short), short.join("\n"));
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
