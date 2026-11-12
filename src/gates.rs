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
    /// The review verdict, `pass` or `block`, for the named gate that drives review.
    #[arg(long, value_parser = ["pass", "block"], conflicts_with = "no_native")]
    verdict: Option<String>,
    /// Close a gate that drives a native step without running it: the decision is
    /// neither an approval nor a review.
    #[arg(long)]
    no_native: bool,
}

/// One listed gate.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Listed {
    pub(super) number: usize,
    pub(super) session: String,
    pub(super) change: Option<String>,
    pub(super) gate: String,
    /// The stage the gate holds and the native step it drives (REQ-cli-030).
    #[serde(default)]
    pub(super) holds: String,
    #[serde(default)]
    pub(super) drives: String,
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

/// Every open session: its ID, its file and its checkpoint, by ID.
fn open_sessions(root: &Path) -> Result<Vec<(String, String, Value)>> {
    let mut sessions = vec![];
    for id in crate::sessions::open_ids(root)? {
        let file = crate::sessions::session_file(&id);
        let session = crate::sessions::read_at(root, &file)?;
        let mut value = serde_json::to_value(&session)?;
        // What each open gate holds and drives, which the checkpoint JSON leaves out.
        value["gate_meta"] = session
            .awaiting_human
            .iter()
            .map(|gate| {
                let meta = crate::session_md::gate(&session, gate);
                (
                    gate.clone(),
                    json!({"holds": meta.holds, "drives": meta.drives}),
                )
            })
            .collect::<serde_json::Map<_, _>>()
            .into();
        sessions.push((id, file, value));
    }
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

/// The requirements the change's deltas add or modify: each `### REQUIREMENT <id>`
/// heading in its workspace's or archive's `deltas/*.md`.
fn delta_requirements(root: &Path, change: &str) -> std::collections::BTreeSet<String> {
    let mut dirs = vec![root.join(".specsync/changes").join(change).join("deltas")];
    if let Ok(entries) = fs::read_dir(root.join(".specsync/archive/changes")) {
        dirs.extend(
            entries
                .filter_map(|entry| entry.ok().map(|entry| entry.path()))
                .filter(|path| {
                    path.file_name()
                        .and_then(|name| name.to_str())
                        .is_some_and(|name| name.ends_with(&format!("-{change}")))
                })
                .map(|path| path.join("deltas")),
        );
    }
    let mut found = std::collections::BTreeSet::new();
    for dir in dirs {
        let Ok(entries) = fs::read_dir(dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(text) = fs::read_to_string(entry.path()) else {
                continue;
            };
            for line in text.lines() {
                if let Some(id) = line.trim().strip_prefix("### REQUIREMENT ") {
                    found.insert(id.trim().to_string());
                }
            }
        }
    }
    found
}

/// The case IDs in `.chough/cases/*.cases.toml` that link one of `requirements`.
fn linked_cases(
    root: &Path,
    requirements: &std::collections::BTreeSet<String>,
) -> std::collections::BTreeSet<String> {
    let mut found = std::collections::BTreeSet::new();
    if requirements.is_empty() {
        return found;
    }
    let Ok(entries) = fs::read_dir(root.join(".chough/cases")) else {
        return found;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.to_string_lossy().ends_with(".cases.toml") {
            continue;
        }
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let Ok(value) = text.parse::<toml::Table>() else {
            continue;
        };
        for case in value
            .get("case")
            .and_then(|cases| cases.as_array())
            .into_iter()
            .flatten()
        {
            let Some(id) = case.get("id").and_then(|id| id.as_str()) else {
                continue;
            };
            let links = case
                .get("requirements")
                .and_then(|list| list.as_array())
                .is_some_and(|list| {
                    list.iter()
                        .filter_map(|requirement| requirement.as_str())
                        .any(|requirement| requirements.contains(requirement))
                });
            if links {
                found.insert(id.to_string());
            }
        }
    }
    found
}

/// The latest outcome of each case that links a requirement the change's deltas add
/// or modify, or that the session's observations name: passed, failed, not run, and
/// the failing case IDs.
fn tests(root: &Path, session: &Value) -> (usize, usize, usize, Vec<String>) {
    let mut wanted = session["native_change"]
        .as_str()
        .map(|change| linked_cases(root, &delta_requirements(root, change)))
        .unwrap_or_default();
    let named: std::collections::BTreeSet<String> = session["observations"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|id| id.as_str().map(String::from))
        .collect();
    let mut latest: std::collections::BTreeMap<String, (u64, String)> = Default::default();
    // Open sessions' entries, then archived stories' and bugs' receipts (REQ-records-005,
    // REQ-history-006).
    let Ok(views) = crate::evidence::every_observation(root) else {
        return (0, 0, 0, vec![]);
    };
    for view in &views {
        let named_here = std::iter::once(&view.id)
            .chain(&view.aliases)
            .any(|id| named.contains(id));
        if named_here && let Some(case) = &view.case {
            wanted.insert(case.clone());
        }
    }
    for view in views {
        let Some(case) = view.case else {
            continue;
        };
        if !wanted.contains(&case) {
            continue;
        }
        let at = view.observed.unwrap_or(0);
        if latest.get(&case).is_none_or(|(seen, _)| at >= *seen) {
            latest.insert(case, (at, view.result));
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

/// What a gate summary's change count leaves out (#190, REQ-cli-061): Chough's
/// records, the knowledge directory and SpecSync's lifecycle bookkeeping.
fn left_out_of_summary(root: &Path) -> Vec<String> {
    let knowledge = crate::prefs::effective(root, "library-dir")
        .map(|(dir, _)| dir)
        .unwrap_or_else(|_| "knowledge".into());
    vec![
        ".chough/".into(),
        format!("{}/", knowledge.trim_end_matches('/')),
        ".specsync/changes/".into(),
        ".specsync/archive/".into(),
        ".specsync/hashes.json".into(),
    ]
}

/// The requirement IDs the delta at `path` adds or modifies, as " (adds or modifies
/// <IDs>)", or nothing when it adds or modifies none or cannot be read (F5).
fn delta_adds(root: &Path, path: &str) -> String {
    let ids = local_path(root, path)
        .ok()
        .and_then(|file| records::read_native_record(&file, records::NATIVE_RECORD_LIMIT).ok())
        .flatten()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .map(|text| crate::stage::added_or_modified(&text))
        .unwrap_or_default();
    if ids.is_empty() {
        String::new()
    } else {
        format!(
            " (adds or modifies {})",
            ids.into_iter().collect::<Vec<_>>().join(", ")
        )
    }
}

/// The intent a gate held at intent approves when the session records no native change
/// (F4, REQ-cli-061): the criterion IDs the session references, and each `hi/` file that
/// holds one as a whole token, read as files (the instruction files left out), with no
/// native tool run.
fn intent_lines(root: &Path, session: &Value) -> Vec<String> {
    const SHOWN: usize = 12;
    let criteria: Vec<String> = session["references"]["criteria"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|id| id.as_str().map(String::from))
        .collect();
    if criteria.is_empty() {
        return vec![
            "Intent: no criteria referenced yet; reference them with `chough checkpoint --criterion <id>`".into(),
        ];
    }
    let more = criteria.len().saturating_sub(SHOWN);
    let mut lines = vec![format!(
        "Intent: {}{} (the criteria the session references)",
        criteria
            .iter()
            .take(SHOWN)
            .cloned()
            .collect::<Vec<_>>()
            .join(", "),
        if more > 0 {
            format!(" and {more} more")
        } else {
            String::new()
        }
    )];
    let mut names: Vec<String> = local_path(root, "hi")
        .ok()
        .and_then(|dir| fs::read_dir(dir).ok())
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md") && name != "AGENTS.md" && name != "CLAUDE.md")
        .collect();
    names.sort();
    let mut holding: Vec<(String, Vec<&String>)> = vec![];
    for name in names {
        let relative = format!("hi/{name}");
        let Some(text) = local_path(root, &relative)
            .ok()
            .and_then(|file| records::read_native_record(&file, records::NATIVE_RECORD_LIMIT).ok())
            .flatten()
            .and_then(|bytes| String::from_utf8(bytes).ok())
        else {
            continue;
        };
        let held: Vec<&String> = criteria
            .iter()
            .filter(|id| crate::evidence::holds_token(&text, id))
            .collect();
        if !held.is_empty() {
            holding.push((relative, held));
        }
    }
    let ids = |ids: &[&String]| {
        ids.iter()
            .map(|id| id.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    for (file, held) in holding.iter().take(TOP_FILES) {
        lines.push(format!("  {file}: {}", ids(held)));
    }
    if holding.len() > TOP_FILES {
        lines.push(format!(
            "  and {} more hi/ files",
            holding.len() - TOP_FILES
        ));
    }
    let missing: Vec<&String> = criteria
        .iter()
        .filter(|id| !holding.iter().any(|(_, held)| held.contains(id)))
        .collect();
    if !missing.is_empty() {
        lines.push(format!("  in no hi/ file: {}", ids(&missing)));
    }
    lines
}

/// The one-screen summary of `gate` in `session`: what changed, the tests, the risks
/// and a diff link, within 24 lines and 4,096 bytes.
pub(super) fn summary(root: &Path, session: &Value, gate: &str) -> String {
    let change = session["native_change"].as_str();
    let head = git(root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".into());
    let mut lines = vec![format!("Gate: {gate}")];
    let state = change.and_then(|change| change_state(root, change));
    let meta = &session["gate_meta"][gate];
    // F4 (REQ-cli-061): a gate held at intent with no native change approves the intent,
    // which the summary gives in place of what changed.
    let at_intent = change.is_none() && meta["holds"] == "intent";
    if at_intent {
        lines.extend(intent_lines(root, session));
    } else {
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
    }
    // REQ-cli-061: count from the change's own base commit, so an earlier change on
    // the same branch is not counted again (D2). Count from the merge base with the
    // default branch instead when default-branch work came in after that base, so the
    // merge base is not one of its ancestors (#190), or when the change records no base.
    // Work on the default branch itself has HEAD as its merge base, which is ignored.
    let base_commit = state
        .as_ref()
        .and_then(|state| state["base_commit"].as_str())
        .map(String::from);
    let merge_base = ["origin/HEAD", "main"]
        .iter()
        .find_map(|branch| git(root, &["merge-base", "HEAD", branch]))
        .filter(|merge_base| *merge_base != head);
    let base = match (base_commit, merge_base) {
        (Some(own), Some(merge_base))
            if git(root, &["merge-base", "--is-ancestor", &merge_base, &own]).is_none() =>
        {
            Some(merge_base)
        }
        (Some(own), _) => Some(own),
        (None, merge_base) => merge_base,
    };
    let left_out = left_out_of_summary(root);
    // REQ-cli-061 (t38j): at a gate that drives approval, the change's own definition is
    // what the human approves, so its Markdown files count and come first, its deltas
    // before its other files, each with the requirements it adds or modifies (F5).
    let drives = crate::humans::Drives::parse(meta["drives"].as_str().unwrap_or_default())
        .unwrap_or_else(|| {
            crate::humans::Drives::default_for(meta["holds"].as_str().unwrap_or_default())
        });
    let definition = change
        .filter(|_| drives == crate::humans::Drives::Approve)
        .map(|change| format!(".specsync/changes/{change}/"));
    let is_definition = |path: &str| {
        definition
            .as_ref()
            .is_some_and(|own| path.starts_with(own.as_str()) && path.ends_with(".md"))
    };
    let is_delta = |path: &str| {
        is_definition(path)
            && definition
                .as_ref()
                .is_some_and(|own| path[own.len()..].starts_with("deltas/"))
    };
    match &base {
        _ if at_intent => {}
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
                .filter(|(_, _, path)| {
                    is_definition(path)
                        || !left_out
                            .iter()
                            .any(|prefix| path.starts_with(prefix.as_str()))
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
                is_delta(&right.2)
                    .cmp(&is_delta(&left.2))
                    .then(is_definition(&right.2).cmp(&is_definition(&left.2)))
                    .then((right.0 + right.1).cmp(&(left.0 + left.1)))
                    .then(left.2.cmp(&right.2))
            });
            // Every delta is named, however many; other files fill the rest of the cap.
            let deltas = files.iter().filter(|file| is_delta(&file.2)).count();
            for (added, deleted, path) in files.iter().take(TOP_FILES.max(deltas)) {
                let adds = if is_delta(path) {
                    delta_adds(root, path)
                } else {
                    String::new()
                };
                lines.push(format!("  {path} +{added} −{deleted}{adds}"));
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
    // With no merge base and no recorded base commit there is no range to show, so the
    // line says so rather than give a diff command that fails (REQ-cli-061).
    lines.push(match base.as_deref() {
        Some(base) => format!("Diff: {}", diff_link(root, base, &head)),
        None => "Diff: unknown, with no merge base and no recorded base commit".into(),
    });
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
            let meta = &session["gate_meta"][gate];
            gates.push(Listed {
                number: gates.len() + 1,
                session: key.clone(),
                change: session["native_change"].as_str().map(String::from),
                gate: gate.to_string(),
                holds: meta["holds"].as_str().unwrap_or_default().to_string(),
                drives: meta["drives"].as_str().unwrap_or_default().to_string(),
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
            "#{} ({}; holds {}; drives {})\n{}\nDigest: {}\n",
            gate.number, gate.session, gate.holds, gate.drives, gate.summary, gate.digest
        );
    }
    // F6 (REQ-cli-061): the list ends with how to reply, `gate record` for the gates
    // whose change is not verifying and the ship for each verifying one, which
    // `gate record` refuses.
    let (ships, records): (Vec<&Listed>, Vec<&Listed>) = saved
        .gates
        .iter()
        .partition(|gate| verifying(root, gate).is_some());
    let mut footer = format!("List {id}.");
    if !records.is_empty() {
        let numbers: Vec<String> = records.iter().map(|gate| gate.number.to_string()).collect();
        footer.push_str(&format!(
            " Record one reply for {} {} with `chough gate record {id} --gates <n>[,<n>…] {RELAY}`.",
            if numbers.len() == 1 { "gate" } else { "gates" },
            spoken_list(&numbers)
        ));
    }
    for gate in ships {
        let change = verifying(root, gate).unwrap_or_default();
        footer.push_str(&format!(
            "\nGate {}'s change {change} is verifying, so it closes through the ship, which runs the review as the human and closes the gate after the native ship: `chough change ship {change} --list {id} --gate {} {RELAY}`.",
            gate.number, gate.number
        ));
    }
    println!("{footer}");
    Ok(())
}

/// The relay placeholders a reply names.
const RELAY: &str = "--decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"";

/// The change of listed `gate` when its `state.json` says it is verifying.
fn verifying<'a>(root: &Path, gate: &'a Listed) -> Option<&'a str> {
    let change = gate.change.as_deref()?;
    change_state(root, change)
        .is_some_and(|state| state["state"] == "verifying")
        .then_some(change)
}

/// `items` read aloud: "1", "1 and 2", "1, 2 and 3".
fn spoken_list(items: &[String]) -> String {
    match items {
        [] => String::new(),
        [only] => only.clone(),
        [head @ .., last] => format!("{} and {last}", head.join(", ")),
    }
}

/// The active session's key, when one is clear.
fn active_key(root: &Path) -> Option<String> {
    let relative = crate::sessions::checkpoint_path(root).ok()?;
    crate::sessions::id_of(&relative)
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
    let mut checked: Vec<Listed> = vec![];
    for number in &numbers {
        let listed = check_listed(root, &args.list, *number)?;
        // F6: the reason is how such a gate closes, never that Chough's records may not
        // be written, which verification_ignored_paths allows.
        if let Some(change) = verifying(root, &listed) {
            bail!(
                "refused:verifying: gate {number} (\"{}\") belongs to {change}, which is verifying, so it closes through the ship, which runs the review as the human and closes the gate after the native ship; nothing was written.\nRun `chough change ship {change} --list {} --gate {number} {RELAY}`.",
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
    // REQ-records-008: every reply names one configured human, whatever its gates
    // drive, before anything runs or is written.
    crate::humans::resolve(root, args.decided_by.as_deref().unwrap_or_default())?;
    // REQ-cli-061: at most one named gate may drive a native step, and it runs first,
    // as the decided human, before any gate is closed.
    let drives = |listed: &Listed| {
        crate::humans::Drives::parse(&listed.drives)
            .unwrap_or_else(|| crate::humans::Drives::default_for(&listed.holds))
    };
    let driving: Vec<&Listed> = checked
        .iter()
        .filter(|listed| drives(listed) != crate::humans::Drives::None)
        .collect();
    if driving.len() > 1 && !args.no_native {
        bail!(
            "refused:several-native-steps: gates {} each drive a native step ({}), and one reply runs at most one, so nothing ran or was written.\nRecord them one at a time: `chough gate record {} --gates <n>` with the relay for each.",
            driving
                .iter()
                .map(|listed| listed.number.to_string())
                .collect::<Vec<_>>()
                .join(" and "),
            driving
                .iter()
                .map(|listed| drives(listed).name())
                .collect::<Vec<_>>()
                .join(", "),
            args.list
        );
    }
    if args.verdict.is_some()
        && !driving
            .iter()
            .any(|listed| drives(listed) == crate::humans::Drives::Review)
    {
        bail!(
            "refused:verdict-not-applicable: no named gate drives a review, so --verdict does not apply; nothing ran or was written.\nRun `chough gate record {} --gates {}` without --verdict.",
            args.list,
            args.gates
        );
    }
    checked.sort_by_key(|listed| drives(listed) == crate::humans::Drives::None);
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
        if drives(listed) != crate::humans::Drives::None {
            if args.no_native {
                command.arg("--no-native");
            } else if let Some(verdict) = &args.verdict {
                command.args(["--verdict", verdict]);
            }
        }
        let output = records::run_owned(&mut command, None)?;
        if !output.status.success() {
            // A refused native step's own output stays visible (REQ-records-008).
            use std::io::Write;
            let _ = std::io::stdout().write_all(&output.stdout);
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
        // The close's own report names the native step it ran, or why none did
        // (REQ-records-008); the reply passes it on for each gate.
        let native_step = serde_json::from_slice::<Value>(&output.stdout)
            .ok()
            .and_then(|report| report.pointer("/relayed_decision/native_step").cloned())
            .unwrap_or(Value::Null);
        closed.push(json!({
            "number": listed.number,
            "gate": listed.gate,
            "summary_digest": listed.digest,
            "native_step": native_step,
        }));
    }
    print_json(&json!({"schema_version": 1, "list": args.list, "closed": closed}))
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_twelve_lowercase_hex_digits_name_a_list() {
        assert!(crate::stage::lowercase_hex("0123456789ab", 12));
        for id in ["", "0123456789AB", "0123456789a", "../etc/passw"] {
            assert!(!crate::stage::lowercase_hex(id, 12), "{id}");
        }
    }
}
