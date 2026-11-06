//! SpecSync change steps through Chough (REQ-cli-057, DRIVE-20): `chough change new`,
//! `answer`, `check` and `status` run the matching SpecSync step in the project root
//! and pass its standard output, standard error and exit code through unchanged.
//! Nothing under `.chough/` is read or written.

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
const INSTALL: &str = "cargo install --locked specsync@6.2.0";

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
    },
    /// Run `specsync change answer <id> <question> <answer> --json`.
    Answer {
        id: String,
        question: String,
        answer: String,
    },
    /// Run `specsync change check <id> --commit`.
    Check { id: String },
    /// Run `specsync change status [<id>] --json` within the native read bound.
    Status { id: Option<String> },
}

/// Run one step and return the exit code to end with.
pub(super) fn run(root: &Path, command: ChangeCommand) -> Result<i32> {
    let (step, arguments, read) = match command {
        ChangeCommand::New {
            description,
            kind,
            specs,
            paths,
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
        ChangeCommand::Check { id } => (
            "check",
            vec!["change".into(), "check".into(), id, "--commit".into()],
            false,
        ),
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
    let mut command = Command::new(SPECSYNC);
    command.args(arguments).current_dir(root);
    let output =
        records::run_owned(&mut command, None).map_err(|error| not_started(step, &error))?;
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
