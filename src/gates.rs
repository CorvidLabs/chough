//! Gate lists (REQ-cli-061, REQ-history-014, FLOW-19, FLOW-20): `chough gate list`
//! numbers every open human gate with a one-screen summary of what is being decided
//! and the SHA-256 digest of that summary, and saves the list untracked;
//! `chough gate record` closes only listed gates with one relayed reply, each relay
//! keeping its own summary digest and the list. No native read is made.

use crate::records::{self, local_path, print_json};
use anyhow::{Context, Result, bail};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;
use std::process::Command;

/// The bounds of one summary: a screen.
const SUMMARY_LINES: usize = 24;
const SUMMARY_BYTES: usize = 4_096;
const RISK_LINES: usize = 8;
const TOP_FILES: usize = 5;

#[derive(Subcommand)]
pub(super) enum GateCommand {
    /// Number every open human gate with a one-screen summary and its digest.
    List {
        /// Print the list as JSON.
        #[arg(long)]
        json: bool,
    },
    /// Close listed gates with one relayed reply; each relay keeps its summary digest.
    Record(Box<RecordArgs>),
}

#[derive(clap::Args)]
pub(super) struct RecordArgs {
    /// The list `chough gate list` printed.
    list: String,
    /// The listed numbers the reply decides, comma-separated.
    #[arg(long)]
    gates: String,
    /// Who decided.
    #[arg(long, allow_hyphen_values = true)]
    decided_by: Option<String>,
    /// Their exact words.
    #[arg(long, allow_hyphen_values = true)]
    words: Option<String>,
    /// Where and when they said them, and who relayed them.
    #[arg(long = "where", allow_hyphen_values = true)]
    place: Option<String>,
    /// The agent recording the decision.
    #[arg(long, allow_hyphen_values = true)]
    recorded_by: Option<String>,
}

/// One listed gate.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Listed {
    pub(super) number: usize,
    pub(super) session: String,
    pub(super) change: Option<String>,
    pub(super) gate: String,
    pub(super) digest: String,
    pub(super) summary: String,
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct List {
    schema_version: u32,
    head: String,
    gates: Vec<Listed>,
}

pub(super) fn run(root: &Path, command: GateCommand) -> Result<()> {
    match command {
        GateCommand::List { json: as_json } => list(root, as_json),
        GateCommand::Record(args) => record(root, *args),
    }
}

fn git(root: &Path, args: &[&str]) -> Option<String> {
    let mut command = Command::new("git");
    command.args(args).current_dir(root);
    let output = records::run_owned(&mut command, None).ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn sha256(text: &str) -> String {
    format!("{:x}", Sha256::digest(text.as_bytes()))
}

/// Every open session: its key and checkpoint, by key. A project without sessions has
/// its legacy checkpoint under the key `session`.
fn open_sessions(root: &Path) -> Result<Vec<(String, String, Value)>> {
    let dir = local_path(root, ".chough/sessions")?;
    let mut sessions = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(&dir)? {
            let path = entry?.path();
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            let Some(key) = name.strip_suffix(".json") else {
                continue;
            };
            if !path.is_file() || name.starts_with('.') {
                continue;
            }
            let value: Value = serde_json::from_slice(&fs::read(&path)?)
                .with_context(|| format!("{} is not a checkpoint", path.display()))?;
            sessions.push((key.to_string(), format!(".chough/sessions/{name}"), value));
        }
    } else if local_path(root, crate::checkpoint::SESSION)?.is_file() {
        let value: Value =
            serde_json::from_slice(&fs::read(local_path(root, crate::checkpoint::SESSION)?)?)?;
        sessions.push((
            "session".into(),
            crate::checkpoint::SESSION.to_string(),
            value,
        ));
    }
    sessions.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(sessions)
}

/// The change's state record, from its workspace or, failing that, its archive.
fn change_state(root: &Path, change: &str) -> Option<Value> {
    let workspace = root
        .join(".specsync/changes")
        .join(change)
        .join("state.json");
    if let Ok(bytes) = fs::read(workspace) {
        return serde_json::from_slice(&bytes).ok();
    }
    let archive = root.join(".specsync/archive/changes");
    let entries = fs::read_dir(archive).ok()?;
    entries
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.ends_with(&format!("-{change}")))
        })
        .find_map(|path| serde_json::from_slice(&fs::read(path.join("state.json")).ok()?).ok())
}

/// The "Risks" section of the change's `plan.md`: its first lines, or None.
fn risks(root: &Path, change: &str) -> Option<Vec<String>> {
    let text =
        fs::read_to_string(root.join(".specsync/changes").join(change).join("plan.md")).ok()?;
    let mut lines = text.lines().skip_while(|line| line.trim() != "## Risks");
    lines.next()?;
    let found: Vec<String> = lines
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| !line.trim().is_empty())
        .take(RISK_LINES)
        .map(String::from)
        .collect();
    (!found.is_empty()).then_some(found)
}

/// The latest outcome of each case the session's observations name: passed, failed,
/// not run, and the failing case IDs.
fn tests(root: &Path, session: &Value) -> (usize, usize, usize, Vec<String>) {
    let mut latest: std::collections::BTreeMap<String, (u64, String)> = Default::default();
    for id in session["observations"].as_array().into_iter().flatten() {
        let Some(id) = id.as_str() else { continue };
        let Ok(bytes) = fs::read(root.join(".chough/evidence").join(format!("{id}.json"))) else {
            continue;
        };
        let Ok(record) = serde_json::from_slice::<Value>(&bytes) else {
            continue;
        };
        let at = record["observed_at_unix_seconds"].as_u64().unwrap_or(0);
        let outcome = record["outcome"].as_str().unwrap_or_default().to_string();
        for case in record["cases"].as_array().into_iter().flatten() {
            let Some(case) = case["id"].as_str() else {
                continue;
            };
            let newer = latest.get(case).is_none_or(|(seen, _)| at >= *seen);
            if newer {
                latest.insert(case.to_string(), (at, outcome.clone()));
            }
        }
    }
    let mut counts = (0, 0, 0, vec![]);
    for (case, (_, outcome)) in latest {
        match outcome.as_str() {
            "passed" => counts.0 += 1,
            "failed" => {
                counts.1 += 1;
                counts.3.push(case);
            }
            _ => counts.2 += 1,
        }
    }
    counts
}

/// The diff link: a GitHub compare URL when `origin` is on GitHub, else the command.
fn diff_link(root: &Path, base: &str, head: &str) -> String {
    let origin = git(root, &["remote", "get-url", "origin"]).unwrap_or_default();
    let repository = origin
        .strip_prefix("https://github.com/")
        .or_else(|| origin.strip_prefix("git@github.com:"))
        .map(|rest| rest.trim_end_matches(".git").to_string());
    match repository {
        Some(repository) if !repository.is_empty() => {
            format!("https://github.com/{repository}/compare/{base}...{head}")
        }
        _ => format!("git diff {base}..{head}"),
    }
}

/// The one-screen summary of `gate` in `session`: what changed, the tests, the risks
/// and a diff link, within 24 lines and 4,096 bytes.
pub(super) fn summary(root: &Path, session: &Value, gate: &str) -> String {
    let change = session["native_change"].as_str();
    let head = git(root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let mut lines = vec![format!("Gate: {gate}")];
    let state = change.and_then(|change| change_state(root, change));
    match change {
        Some(change) => {
            let title = state
                .as_ref()
                .and_then(|state| state["title"].as_str())
                .unwrap_or("untitled");
            let current = state
                .as_ref()
                .and_then(|state| state["state"].as_str())
                .unwrap_or("unknown");
            lines.push(format!("Change: {change} — {title} ({current})"));
        }
        None => lines.push("Change: none recorded".into()),
    }
    let base = state
        .as_ref()
        .and_then(|state| state["base_commit"].as_str())
        .map(String::from);
    match &base {
        Some(base) => {
            let numstat =
                git(root, &["diff", "--numstat", &format!("{base}..HEAD")]).unwrap_or_default();
            let mut files: Vec<(u64, u64, String)> = numstat
                .lines()
                .filter_map(|line| {
                    let mut parts = line.splitn(3, '\t');
                    let added = parts.next()?.parse().unwrap_or(0);
                    let deleted = parts.next()?.parse().unwrap_or(0);
                    Some((added, deleted, parts.next()?.to_string()))
                })
                .collect();
            let insertions: u64 = files.iter().map(|file| file.0).sum();
            let deletions: u64 = files.iter().map(|file| file.1).sum();
            lines.push(format!(
                "What changed: {} files, +{insertions} −{deletions} since {}",
                files.len(),
                &base[..base.len().min(8)]
            ));
            files.sort_by(|left, right| {
                (right.0 + right.1)
                    .cmp(&(left.0 + left.1))
                    .then(left.2.cmp(&right.2))
            });
            for (added, deleted, path) in files.iter().take(TOP_FILES) {
                lines.push(format!("  {path} +{added} −{deleted}"));
            }
        }
        None => lines.push("What changed: unknown (no base commit recorded)".into()),
    }
    let (passed, failed, not_run, failing) = tests(root, session);
    lines.push(format!(
        "Tests: {passed} passed, {failed} failed, {not_run} not run"
    ));
    if !failing.is_empty() {
        lines.push(format!("  Failing: {}", failing.join(", ")));
    }
    match change.and_then(|change| risks(root, change)) {
        Some(found) => {
            lines.push("Risks:".into());
            lines.extend(found.into_iter().map(|line| format!("  {}", line.trim())));
        }
        None => lines.push("Risks: none recorded".into()),
    }
    lines.push(format!(
        "Diff: {}",
        diff_link(root, base.as_deref().unwrap_or("unknown"), &head)
    ));
    if lines.len() > SUMMARY_LINES {
        lines.truncate(SUMMARY_LINES - 1);
        lines.push("… (cut)".into());
    }
    let mut text = lines.join("\n");
    if text.len() > SUMMARY_BYTES {
        let mut end = SUMMARY_BYTES - "\n… (cut)".len();
        while !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
        text.push_str("\n… (cut)");
    }
    text
}

fn list_relative(id: &str) -> String {
    format!(".chough/local/gate-lists/{id}.json")
}

/// The saved list `id`, or None when there is none.
pub(super) fn load(root: &Path, id: &str) -> Option<Vec<Listed>> {
    if !crate::stage::lowercase_hex(id, 12) {
        return None;
    }
    let bytes = fs::read(local_path(root, &list_relative(id)).ok()?).ok()?;
    let list: List = serde_json::from_slice(&bytes).ok()?;
    Some(list.gates)
}

fn list(root: &Path, as_json: bool) -> Result<()> {
    let mut gates = vec![];
    for (key, _, session) in open_sessions(root)? {
        for gate in session["awaiting_human"].as_array().into_iter().flatten() {
            let Some(gate) = gate.as_str() else { continue };
            let text = summary(root, &session, gate);
            gates.push(Listed {
                number: gates.len() + 1,
                session: key.clone(),
                change: session["native_change"].as_str().map(String::from),
                gate: gate.to_string(),
                digest: sha256(&text),
                summary: text,
            });
        }
    }
    let head = git(root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let saved = List {
        schema_version: 1,
        head,
        gates,
    };
    let bytes = serde_json::to_vec_pretty(&saved)?;
    let id = sha256(&String::from_utf8_lossy(&bytes))[..12].to_string();
    // Untracked: `.chough/local/` holds a `.gitignore` of `*`.
    let path = local_path(root, &list_relative(&id))?;
    let dir = path.parent().context("the list has no directory")?;
    fs::create_dir_all(dir)?;
    let ignore = local_path(root, ".chough/local/.gitignore")?;
    if !ignore.exists() {
        fs::write(&ignore, "*\n")?;
    }
    let temporary = dir.join(format!(".{id}.json.{}.tmp", std::process::id()));
    fs::write(&temporary, &bytes)?;
    fs::rename(&temporary, &path)?;
    if as_json {
        return print_json(&json!({
            "schema_version": 1,
            "list": id,
            "head": saved.head,
            "gates": saved.gates,
        }));
    }
    if saved.gates.is_empty() {
        println!("No open gates.");
        return Ok(());
    }
    for gate in &saved.gates {
        println!(
            "#{} ({})\n{}\nDigest: {}\n",
            gate.number, gate.session, gate.summary, gate.digest
        );
    }
    println!(
        "List {id}. Record one reply for listed gates with `chough gate record {id} --gates <n>[,<n>…] --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`."
    );
    Ok(())
}

/// The active session's key, when one is clear.
fn active_key(root: &Path) -> Option<String> {
    let relative = crate::sessions::checkpoint_path(root).ok()?;
    relative
        .strip_prefix(".chough/sessions/")
        .and_then(|name| name.strip_suffix(".json"))
        .map(String::from)
        .or_else(|| (relative == crate::checkpoint::SESSION).then(|| "session".into()))
}

/// Check that listed gate `number` of list `id` may be closed now: on the list, in the
/// active session, still open, its summary unchanged, and its change not verifying.
/// Returns the listed gate. `ship` is the command that closes a verifying change's
/// gate, given the relay placeholders.
pub(super) fn check_listed(root: &Path, id: &str, number: usize) -> Result<Listed> {
    let Some(gates) = load(root, id) else {
        bail!(
            "refused:unknown-list: there is no gate list {id}, so nothing was written.\nRun `chough gate list` to number the open gates, then reply with its list."
        );
    };
    let Some(listed) = gates.iter().find(|gate| gate.number == number).cloned() else {
        let numbers: Vec<String> = gates.iter().map(|gate| gate.number.to_string()).collect();
        bail!(
            "refused:not-listed: list {id} has no gate {number} (it lists {}), so nothing was written.\nRun `chough gate list` to see the open gates and their numbers.",
            if numbers.is_empty() {
                "none".to_string()
            } else {
                numbers.join(", ")
            }
        );
    };
    if active_key(root).as_deref() != Some(listed.session.as_str()) {
        bail!(
            "refused:other-session: gate {number} of list {id} is in session {}, which is not active, so nothing was written.\nRun `chough session use {}`, then `chough gate record {id} --gates {number}` with the relay again.",
            listed.session,
            listed.session
        );
    }
    let session = open_sessions(root)?
        .into_iter()
        .find(|(key, _, _)| *key == listed.session)
        .map(|(_, _, value)| value)
        .unwrap_or(Value::Null);
    let open = session["awaiting_human"]
        .as_array()
        .is_some_and(|gates| gates.iter().any(|gate| gate.as_str() == Some(&listed.gate)));
    if !open {
        bail!(
            "refused:gate-not-open: gate {number} of list {id} (\"{}\") is no longer open, so nothing was written.\nRun `chough gate list` to number the gates that are open now.",
            listed.gate
        );
    }
    if sha256(&summary(root, &session, &listed.gate)) != listed.digest {
        bail!(
            "refused:summary-changed: the summary of gate {number} (\"{}\") has changed since list {id} showed it, so the reply was not given on it and nothing was written.\nRun `chough gate list` to show the current summaries, and record the reply against that list.",
            listed.gate
        );
    }
    Ok(listed)
}

fn record(root: &Path, args: RecordArgs) -> Result<()> {
    let numbers: Vec<usize> = args
        .gates
        .split(',')
        .map(|number| number.trim().parse::<usize>())
        .collect::<std::result::Result<_, _>>()
        .map_err(|_| {
            anyhow::anyhow!(
                "refused:not-listed: --gates must be listed numbers such as 1,3; got {:?}, so nothing was written.\nRun `chough gate list` to see the numbers.",
                args.gates
            )
        })?;
    let mut checked = vec![];
    for number in &numbers {
        let listed = check_listed(root, &args.list, *number)?;
        let verifying = listed
            .change
            .as_deref()
            .and_then(|change| change_state(root, change))
            .is_some_and(|state| state["state"] == "verifying");
        if verifying {
            let change = listed.change.as_deref().unwrap_or_default();
            bail!(
                "refused:verifying: gate {number} (\"{}\") belongs to {change}, which is verifying, and nothing may write .chough/ before it ships, so nothing was written.\nRun `chough change ship {change} --list {} --gate {number} --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`.",
                listed.gate,
                args.list
            );
        }
        checked.push(listed);
    }
    let values = [
        ("--decided-by", args.decided_by.as_deref()),
        ("--words", args.words.as_deref()),
        ("--where", args.place.as_deref()),
        ("--recorded-by", args.recorded_by.as_deref()),
    ];
    let open: Vec<String> = checked.iter().map(|listed| listed.gate.clone()).collect();
    for listed in &checked {
        crate::checkpoint::check_relay(&open, Some(&listed.gate), values).map_err(|_| {
            anyhow::anyhow!(
                "refused:relay-incomplete: one reply needs who decided, their words, where they said them and the recording agent, each non-blank and within the checkpoint text bound, so nothing was written.\nRun `chough gate record {} --gates {} --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`.",
                args.list,
                args.gates
            )
        })?;
    }
    // Close each gate as `chough checkpoint --close-gate` does, through Chough's own
    // command, so the record and its checks are exactly that command's.
    let chough = std::env::current_exe()?;
    let mut closed = vec![];
    for (index, listed) in checked.iter().enumerate() {
        let mut command = Command::new(&chough);
        command
            .arg("--root")
            .arg(root)
            .args(["checkpoint", "--close-gate", &listed.gate])
            .args([
                "--decided-by",
                args.decided_by.as_deref().unwrap_or_default(),
            ])
            .args(["--words", args.words.as_deref().unwrap_or_default()])
            .args(["--where", args.place.as_deref().unwrap_or_default()])
            .args([
                "--recorded-by",
                args.recorded_by.as_deref().unwrap_or_default(),
            ])
            .args(["--summary-digest", &listed.digest, "--list", &args.list])
            .current_dir(root);
        let output = records::run_owned(&mut command, None)?;
        if !output.status.success() {
            let remaining: Vec<String> = checked[index..]
                .iter()
                .map(|listed| listed.number.to_string())
                .collect();
            bail!(
                "closing gate {} (\"{}\") was refused: {}\nRun `chough gate list`, then `chough gate record` for gates {} again.",
                listed.number,
                listed.gate,
                String::from_utf8_lossy(&output.stderr).trim(),
                remaining.join(",")
            );
        }
        closed.push(
            json!({"number": listed.number, "gate": listed.gate, "summary_digest": listed.digest}),
        );
    }
    print_json(&json!({"schema_version": 1, "list": args.list, "closed": closed}))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_twelve_lowercase_hex_digits_name_a_list() {
        assert!(crate::stage::lowercase_hex("0123456789ab", 12));
        for id in ["", "0123456789AB", "0123456789a", "../etc/passw"] {
            assert!(!crate::stage::lowercase_hex(id, 12), "{id}");
        }
    }
}
