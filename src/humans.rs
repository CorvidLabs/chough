//! A human's gate decision drives the native approval or review as that human
//! (REQ-records-008, #236). Identities come from configuration, never from source:
//! the user file `chough/identity.toml` under `XDG_CONFIG_HOME` (by default
//! `~/.config`), replaced by `.chough/local/identity.toml` when that exists. Who
//! decided stays a relayed claim (FLOW-16): Chough maps the claim to the configured
//! SpecSync actor, it does not authenticate it.

use crate::records::{local_path, run_owned};
use anyhow::{Context, Result, bail};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The project's own identity file; it replaces the user file when present.
pub(crate) const LOCAL_FILE: &str = ".chough/local/identity.toml";
/// The user identity file, under `XDG_CONFIG_HOME` or `~/.config`.
pub(crate) const USER_FILE: &str = "chough/identity.toml";
/// How people find the user file, without naming this machine's home folder.
const USER_FILE_SHOWN: &str =
    "$XDG_CONFIG_HOME/chough/identity.toml (by default ~/.config/chough/identity.toml)";
const SPECSYNC: &str = "specsync";

/// The native step an open gate drives when a human closes it (REQ-cli-030).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Drives {
    Approve,
    Review,
    None,
}

impl Drives {
    /// The step a gate drives by default: approval for one held at plan, review for one
    /// held at accept, nothing otherwise.
    pub(crate) fn default_for(holds: &str) -> Drives {
        match holds {
            "plan" => Drives::Approve,
            "accept" => Drives::Review,
            _ => Drives::None,
        }
    }

    pub(crate) fn parse(text: &str) -> Option<Drives> {
        match text {
            "approve" => Some(Drives::Approve),
            "review" => Some(Drives::Review),
            "none" => Some(Drives::None),
            _ => None,
        }
    }

    pub(crate) fn name(self) -> &'static str {
        match self {
            Drives::Approve => "approve",
            Drives::Review => "review",
            Drives::None => "none",
        }
    }
}

/// One configured human.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub(crate) struct Human {
    /// The display name records show.
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) aliases: Vec<String>,
    /// The actor SpecSync records for this human.
    pub(crate) specsync_actor: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct File {
    #[serde(default)]
    human: Vec<Human>,
}

/// The user identity file's path, when a configuration folder is known.
fn user_file() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .or_else(|| {
            std::env::var_os("HOME")
                .filter(|value| !value.is_empty())
                .map(|home| PathBuf::from(home).join(".config"))
        })?;
    Some(base.join(USER_FILE))
}

/// The configured humans: the project file's when it exists, else the user file's,
/// else none. A file that cannot be read or parsed is refused, naming which one.
fn configured(root: &Path) -> Result<Vec<Human>> {
    let local = local_path(root, LOCAL_FILE)?;
    let (path, shown) = if local.is_file() {
        (local, LOCAL_FILE.to_string())
    } else {
        match user_file() {
            Some(path) if path.is_file() => (path, USER_FILE_SHOWN.to_string()),
            _ => return Ok(vec![]),
        }
    };
    let text = std::fs::read_to_string(&path)
        .with_context(|| format!("the identity file {shown} cannot be read"))?;
    let file: File = toml::from_str(&text).map_err(|error| {
        anyhow::anyhow!(
            "refused:identity-file-invalid: the identity file {shown} is not valid: {}; nothing ran or was written.\nGive each person as [[human]] with name, aliases (optional) and specsync_actor.",
            error.message()
        )
    })?;
    for human in &file.human {
        if human.name.trim().is_empty() || human.specsync_actor.trim().is_empty() {
            bail!(
                "refused:identity-file-invalid: a [[human]] in {shown} has a blank name or specsync_actor; nothing ran or was written."
            );
        }
    }
    Ok(file.human)
}

/// The one configured human `decided_by` names, by name or alias, ignoring case and
/// surrounding space. None, or two, is refused before anything runs; there is no
/// guess and no fallback to an agent account.
pub(crate) fn resolve(root: &Path, decided_by: &str) -> Result<Human> {
    let wanted = decided_by.trim();
    let humans = configured(root)?;
    let matches: Vec<&Human> = humans
        .iter()
        .filter(|human| {
            std::iter::once(&human.name)
                .chain(&human.aliases)
                .any(|name| name.trim().eq_ignore_ascii_case(wanted))
        })
        .collect();
    match matches.as_slice() {
        [human] => Ok((*human).clone()),
        [] => bail!(
            "refused:human-identity-unknown: --decided-by \"{wanted}\" names no configured human, so no native step ran and nothing was written.\nAdd them to {LOCAL_FILE} or to {USER_FILE_SHOWN} as:\n[[human]]\nname = \"<display name>\"\naliases = [\"{wanted}\"]\nspecsync_actor = \"<their SpecSync actor>\"\nThen close the gate again. A decision that is neither an approval nor a review closes with --no-native."
        ),
        several => bail!(
            "refused:human-identity-unknown: --decided-by \"{wanted}\" names {} configured humans ({}), so no native step ran and nothing was written.\nMake the name or alias unique in {LOCAL_FILE} or {USER_FILE_SHOWN}, then close the gate again.",
            several.len(),
            several
                .iter()
                .map(|human| human.name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// A native step to run as a human.
#[derive(Clone, Debug)]
pub(crate) enum Step {
    /// `specsync change approve <id> --actor <actor> --note <note>`.
    Approve { note: String },
    /// `specsync change review <id> --reviewer <actor> --verdict <verdict>`.
    Review { verdict: String },
}

impl Step {
    /// The arguments after `specsync`.
    pub(crate) fn arguments(&self, change: &str, human: &Human) -> Vec<String> {
        match self {
            Step::Approve { note } => vec![
                "change".into(),
                "approve".into(),
                change.into(),
                "--actor".into(),
                human.specsync_actor.clone(),
                "--note".into(),
                note.clone(),
            ],
            Step::Review { verdict } => vec![
                "change".into(),
                "review".into(),
                change.into(),
                "--reviewer".into(),
                human.specsync_actor.clone(),
                "--verdict".into(),
                verdict.clone(),
            ],
        }
    }

    /// The step as one line for the decision and the output, without the note's text.
    pub(crate) fn summary(&self, change: &str, human: &Human) -> String {
        match self {
            Step::Approve { .. } => format!(
                "ran specsync change approve {change} as {}",
                human.specsync_actor
            ),
            Step::Review { verdict } => format!(
                "ran specsync change review {change} as {} with verdict {verdict}",
                human.specsync_actor
            ),
        }
    }
}

/// The note an approval records: the human's words, where and when, and the agent
/// that relayed them (REQ-records-008).
pub(crate) fn approval_note(
    human: &Human,
    gate: &str,
    relay: &crate::stage::RelayedBy,
    at: &str,
) -> String {
    format!(
        "Approved by {} at the Chough gate \"{gate}\": \"{}\" ({}). Relayed and recorded by {} at {at} through chough checkpoint --close-gate; a relayed claim, not authenticated.",
        human.name, relay.words, relay.place, relay.recorded_by
    )
}

/// What a native step left: SpecSync's output and exit code.
pub(crate) struct Ran {
    pub(crate) code: i32,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

/// Run `step` for `change` as `human` in the project root, as Chough's own native
/// write: the direct-write baseline follows it (REQ-cli-060), and a stop signal to
/// Chough stops SpecSync too.
pub(crate) fn run(root: &Path, change: &str, human: &Human, step: &Step) -> Result<Ran> {
    let before = crate::direct_writes::before_step(root, change);
    let mut command = Command::new(SPECSYNC);
    command
        .args(step.arguments(change, human))
        .current_dir(root);
    let output = run_owned(&mut command, None).map_err(|error| {
        anyhow::anyhow!(
            "specsync could not be started ({error}), so no native step ran and nothing was written."
        )
    })?;
    if let Some(before) = before {
        let _ = crate::direct_writes::after_step(root, before);
    }
    #[cfg(unix)]
    let signal = std::os::unix::process::ExitStatusExt::signal(&output.status);
    #[cfg(not(unix))]
    let signal: Option<i32> = None;
    Ok(Ran {
        code: output
            .status
            .code()
            .unwrap_or_else(|| 128 + signal.unwrap_or(0)),
        stdout: output.stdout,
        stderr: output.stderr,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_step_follows_the_stage_a_gate_holds() {
        assert_eq!(Drives::default_for("plan"), Drives::Approve);
        assert_eq!(Drives::default_for("accept"), Drives::Review);
        for stage in ["intent", "design", "build", "test", "release", "bugs"] {
            assert_eq!(Drives::default_for(stage), Drives::None, "{stage}");
        }
        for drives in [Drives::Approve, Drives::Review, Drives::None] {
            assert_eq!(Drives::parse(drives.name()), Some(drives));
        }
        assert_eq!(Drives::parse("Approve"), None);
    }

    #[test]
    fn the_project_file_resolves_by_name_or_alias_and_refuses_none_or_two() {
        let root = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(root.path().join(".chough/local")).unwrap();
        std::fs::write(
            root.path().join(LOCAL_FILE),
            "[[human]]\nname = \"SYNTHETIC Person\"\naliases = [\"synthetic\"]\nspecsync_actor = \"SYNTHETIC-actor\"\n\n[[human]]\nname = \"Other\"\naliases = [\"Shared\"]\nspecsync_actor = \"other\"\n\n[[human]]\nname = \"Third\"\naliases = [\"shared\"]\nspecsync_actor = \"third\"\n",
        )
        .unwrap();
        let human = resolve(root.path(), " Synthetic ").unwrap();
        assert_eq!(human.specsync_actor, "SYNTHETIC-actor");
        assert_eq!(resolve(root.path(), "SYNTHETIC Person").unwrap(), human);
        let unknown = resolve(root.path(), "nobody").unwrap_err().to_string();
        assert!(
            unknown.starts_with("refused:human-identity-unknown"),
            "{unknown}"
        );
        assert!(unknown.contains(LOCAL_FILE) && unknown.contains("XDG_CONFIG_HOME"));
        assert!(unknown.contains("specsync_actor"), "{unknown}");
        let two = resolve(root.path(), "shared").unwrap_err().to_string();
        assert!(
            two.contains("names 2 configured humans (Other, Third)"),
            "{two}"
        );
        std::fs::write(root.path().join(LOCAL_FILE), "[[human]]\nname = \"x\"\n").unwrap();
        let invalid = resolve(root.path(), "x").unwrap_err().to_string();
        assert!(
            invalid.starts_with("refused:identity-file-invalid"),
            "{invalid}"
        );
    }
}
