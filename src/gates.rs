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
/// At an intent gate, the lines its `hi/` files and criteria take, the bytes the whole
/// intent may take, so the tests and risks still fit, and the IDs a list line names.
const INTENT_LINES: usize = 16;
const INTENT_BYTES: usize = 3_072;
const LISTED_IDS: usize = 12;
/// What ends a sentence or a summary cut to fit.
const CUT: &str = "… (cut)";

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
    git_untrimmed(root, args).map(|text| text.trim().to_string())
}

/// [`git`]'s output as Git printed it, for a NUL-separated list whose first path may
/// start with a space.
fn git_untrimmed(root: &Path, args: &[&str]) -> Option<String> {
    let mut command = Command::new("git");
    command.args(args).current_dir(root);
    let output = records::run_owned(&mut command, None).ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
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
    // #389: an observation the session names that links no case, as `evidence add`
    // with criteria records, counts by its own entry.
    let mut uncased: Vec<(String, String)> = vec![];
    for view in &views {
        let named_here = std::iter::once(&view.id)
            .chain(&view.aliases)
            .any(|id| named.contains(id));
        match &view.case {
            Some(case) if named_here => {
                wanted.insert(case.clone());
            }
            None if named_here => uncased.push((view.id.clone(), view.result.clone())),
            _ => {}
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
    for (case, (_, outcome)) in latest.into_iter().chain(
        uncased
            .into_iter()
            .map(|(entry, outcome)| (entry, (0, outcome))),
    ) {
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

/// Whether the working tree holds a change since HEAD, tracked or untracked, that what
/// changed counts (`counts`, the same rule as that line's), so the diff link must show it
/// (#389). At a gate that drives approval the change's own definition counts, though the
/// rest of `.specsync/changes/` is left out (tester2's finding on #428).
fn uncommitted_work(root: &Path, counts: impl Fn(&str) -> bool) -> bool {
    let tracked = git(root, &["diff", "--name-only", "HEAD"]).unwrap_or_default();
    tracked
        .lines()
        .map(String::from)
        .chain(untracked(root).into_iter().map(|(_, _, path)| path))
        .any(|path| !path.is_empty() && counts(&path))
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

/// Each untracked file Git does not ignore, in the whole work tree, with its lines as
/// insertions and no deletions, its path from the work tree's top as `git diff` gives
/// it (F2).
fn untracked(root: &Path) -> Vec<(u64, u64, String)> {
    let Some(top) = git(root, &["rev-parse", "--show-toplevel"]) else {
        return vec![];
    };
    let top = Path::new(&top);
    let listed = git_untrimmed(
        top,
        &[
            "ls-files",
            "--others",
            "--exclude-standard",
            "--full-name",
            "-z",
        ],
    )
    .unwrap_or_default();
    let mut found: Vec<(u64, u64, String)> = listed
        .split('\0')
        .filter(|path| !path.is_empty())
        .map(|path| (added_lines(&top.join(path)), 0, path.to_string()))
        .collect();
    found.sort_by(|left, right| left.2.cmp(&right.2));
    found
}

/// The lines an added file inserts, as Git counts them: every newline, plus a last line
/// without one; none for a binary file (a NUL byte in its first 8,000 bytes, Git's test),
/// a file that cannot be read or anything that is not a regular file.
fn added_lines(path: &Path) -> u64 {
    use std::io::Read;
    if !fs::symlink_metadata(path).is_ok_and(|meta| meta.is_file()) {
        return 0;
    }
    let Ok(mut file) = fs::File::open(path) else {
        return 0;
    };
    let mut buffer = vec![0; 65_536];
    let (mut lines, mut read, mut last) = (0u64, 0usize, None);
    loop {
        let count = match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(count) => count,
            Err(_) => return 0,
        };
        let chunk = &buffer[..count];
        if read < 8_000 && chunk[..count.min(8_000 - read)].contains(&0) {
            return 0;
        }
        read += count;
        lines += chunk.iter().filter(|byte| **byte == b'\n').count() as u64;
        last = chunk.last().copied();
    }
    lines + u64::from(last.is_some_and(|byte| byte != b'\n'))
}

/// The requirement IDs the delta at `path` adds and those it modifies, each said as such
/// (REQ-cli-061), as " (adds <IDs>; modifies <IDs>)" with only the parts that name an ID,
/// or nothing when it names none or cannot be read.
fn delta_adds(root: &Path, path: &str) -> String {
    let (added, modified) = local_path(root, path)
        .ok()
        .and_then(|file| records::read_native_record(&file, records::NATIVE_RECORD_LIMIT).ok())
        .flatten()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .map(|text| crate::stage::added_and_modified(&text))
        .unwrap_or_default();
    let said: Vec<String> = [("adds", added), ("modifies", modified)]
        .into_iter()
        .filter(|(_, ids)| !ids.is_empty())
        .map(|(verb, ids)| format!("{verb} {}", ids.into_iter().collect::<Vec<_>>().join(", ")))
        .collect();
    if said.is_empty() {
        String::new()
    } else {
        format!(" ({})", said.join("; "))
    }
}

/// The sentence a `hi/` file's `text` gives criterion `id`: the rest of the list item
/// whose first word, its Markdown emphasis left out, is the ID, as `hi` writes
/// `- **<ID>**  <sentence>`. A file that only mentions the ID elsewhere gives none.
fn criterion_sentence(text: &str, id: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let item = line.trim_start();
        let item = item
            .strip_prefix("- ")
            .or_else(|| item.strip_prefix("* "))?
            .trim_start();
        let (word, rest) = item.split_at(item.find(char::is_whitespace).unwrap_or(item.len()));
        (word.trim_matches(|c| matches!(c, '*' | '_' | '`' | ':')) == id)
            .then(|| rest.trim().to_string())
    })
}

/// `ids` joined, at most `LISTED_IDS` of them, then how many more.
fn listed(ids: &[&String]) -> String {
    let shown = ids
        .iter()
        .take(LISTED_IDS)
        .map(|id| id.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    match ids.len().saturating_sub(LISTED_IDS) {
        0 => shown,
        more => format!("{shown} and {more} more"),
    }
}

/// The longest a sentence may be so that sentences of `lengths` fit in `available`
/// bytes: none is cut when they all fit; otherwise the longest are cut to one length.
fn sentence_cap(lengths: &[usize], available: usize) -> usize {
    let mut sorted = lengths.to_vec();
    sorted.sort_unstable();
    let mut left = available;
    for (index, length) in sorted.iter().enumerate() {
        let share = left / (sorted.len() - index);
        if *length > share {
            return share;
        }
        left -= length;
    }
    usize::MAX
}

/// `text` whole when it is at most `cap` bytes, otherwise cut on a character boundary
/// to end in `CUT` within `cap` bytes.
fn cut_to(text: &str, cap: usize) -> String {
    if text.len() <= cap {
        return text.to_string();
    }
    let mut end = cap.saturating_sub(CUT.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}{CUT}", text[..end].trim_end())
}

/// The intent a gate held at intent approves when the session records no native change
/// (REQ-cli-061): each criterion the session references, by ID with its sentence from the
/// `hi/` file that holds it, laid out as `hi ls` lays it out, the files in the order the
/// session first references one of their criteria. The files are read (the instruction
/// files left out) and no native tool runs. Files and criteria take at most
/// `INTENT_LINES` lines, the rest named on one line, and the sentences share what is left
/// of `INTENT_BYTES`, the longest cut to fit, so the tests and risks still show.
fn intent_lines(root: &Path, session: &Value) -> Vec<String> {
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
    // Each criterion's file and sentence, from the first file by name that holds it.
    let mut held: Vec<Option<(String, String)>> = vec![None; criteria.len()];
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
        for (id, found) in criteria.iter().zip(held.iter_mut()) {
            if found.is_none()
                && let Some(sentence) = criterion_sentence(&text, id)
            {
                *found = Some((relative.clone(), sentence));
            }
        }
    }
    let mut files: Vec<(&str, Vec<(&String, &str)>)> = vec![];
    for (id, found) in criteria.iter().zip(&held) {
        let Some((file, sentence)) = found else {
            continue;
        };
        match files.iter_mut().find(|(name, _)| name == file) {
            Some((_, items)) => items.push((id, sentence)),
            None => files.push((file, vec![(id, sentence)])),
        }
    }
    // Each file line with as many of its criteria as the lines left allow.
    let mut room = INTENT_LINES;
    let mut shown: Vec<(&str, &[(&String, &str)])> = vec![];
    let mut more: Vec<&String> = vec![];
    for (file, items) in &files {
        let take = if room >= 2 {
            items.len().min(room - 1)
        } else {
            0
        };
        more.extend(items[take..].iter().map(|(id, _)| *id));
        if take > 0 {
            room -= take + 1;
            shown.push((file, &items[..take]));
        }
    }
    more.sort_by_key(|id| criteria.iter().position(|criterion| criterion == *id));
    let missing: Vec<&String> = criteria
        .iter()
        .zip(&held)
        .filter(|(_, found)| found.is_none())
        .map(|(id, _)| id)
        .collect();
    let mut tail = vec![];
    if !more.is_empty() {
        tail.push(format!("  and {} more: {}", more.len(), listed(&more)));
    }
    if !missing.is_empty() {
        tail.push(format!("  in no hi/ file: {}", listed(&missing)));
    }
    let header = format!(
        "Intent: {} the session references, each with its sentence from hi/",
        match criteria.len() {
            1 => "1 criterion".to_string(),
            count => format!("{count} criteria"),
        }
    );
    // The sentences share what the other lines, newlines included, leave.
    let fixed = header.len()
        + 1
        + tail.iter().map(|line| line.len() + 1).sum::<usize>()
        + shown
            .iter()
            .map(|(file, items)| {
                file.len() + 3 + items.iter().map(|(id, _)| id.len() + 7).sum::<usize>()
            })
            .sum::<usize>();
    let lengths: Vec<usize> = shown
        .iter()
        .flat_map(|(_, items)| items.iter().map(|(_, sentence)| sentence.len()))
        .collect();
    let cap = sentence_cap(&lengths, INTENT_BYTES.saturating_sub(fixed));
    let mut lines = vec![header];
    for (file, items) in shown {
        lines.push(format!("  {file}"));
        for (id, sentence) in items {
            lines.push(
                format!("    {id}  {}", cut_to(sentence, cap))
                    .trim_end()
                    .to_string(),
            );
        }
    }
    lines.extend(tail);
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
                lines.push(format!("Change: {change} - {title} ({current})"));
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
    // #389: a session with no native change records no base, as one started before the
    // repository's first commit; count from that first commit when there is exactly
    // one, and say so. A native change with no usable base keeps the base unknown.
    let from_first_commit = base.is_none() && change.is_none();
    let base = base.or_else(|| {
        git(root, &["rev-list", "--max-parents=0", "HEAD"])
            .filter(|roots| from_first_commit && !roots.is_empty() && roots.lines().count() == 1)
    });
    let from_first_commit = from_first_commit && base.is_some();
    let left_out = left_out_of_summary(root);
    // REQ-cli-061 (t38j): at a gate that drives approval, the change's own definition is
    // what the human approves, so its Markdown files count and come first, its deltas
    // before its other files, each with the requirements it adds and those it modifies.
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
            // F2 (REQ-cli-061): counted in the working tree, so a change's own files count
            // before they are committed: tracked files against the base, and each untracked
            // file, not ignored, as all insertions.
            let numstat = git(root, &["diff", "--numstat", base]).unwrap_or_default();
            let mut files: Vec<(u64, u64, String)> = numstat
                .lines()
                .filter_map(|line| {
                    let mut parts = line.splitn(3, '\t');
                    let added = parts.next()?.parse().unwrap_or(0);
                    let deleted = parts.next()?.parse().unwrap_or(0);
                    Some((added, deleted, parts.next()?.to_string()))
                })
                .chain(untracked(root))
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
            if from_first_commit {
                lines.push(
                    "  no base commit was recorded: the session started before the first commit, so this counts from it"
                        .into(),
                );
            }
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
    // line says so rather than give a diff command that fails (REQ-cli-061). A gate held
    // at intent with no native change approves the intent, so it has no diff line.
    match base.as_deref() {
        _ if at_intent => {}
        // #389: work not committed yet is in no range to HEAD, so the link is the working
        // tree against the base, and says so.
        Some(base)
            if uncommitted_work(root, |path| {
                is_definition(path)
                    || !left_out
                        .iter()
                        .any(|prefix| path.starts_with(prefix.as_str()))
            }) =>
        {
            lines.push(format!("Diff: git diff {base} (work not committed yet)"))
        }
        Some(base) => lines.push(format!("Diff: {}", diff_link(root, base, &head))),
        None => lines.push("Diff: unknown, with no merge base and no recorded base commit".into()),
    }
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
                "refused:not-listed: --gates must be listed numbers such as 1,3; got {}, so nothing was written.\nRun `chough gate list` to see the numbers.",
                crate::leaks::shown("--gates", &args.gates)
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
        crate::checkpoint::check_relay(root, &open, Some(&listed.gate), values).map_err(|problem| {
            match problem {
                crate::checkpoint::RelayProblem::Identity(error) => error,
                _ => anyhow::anyhow!(
                    "refused:relay-incomplete: one reply needs who decided, their words, where they said them and the recording agent, each non-blank and within the checkpoint text bound, so nothing was written.\nRun `chough gate record {} --gates {} --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`.",
                    args.list,
                    args.gates
                ),
            }
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
    // fix-438: before any real close writes, dry-run every named gate's close, so a close
    // that would be refused over the session record bound refuses the whole reply with
    // nothing written instead of closing the gates before it. The dry runs close each
    // named gate in order on the state the earlier named gates' closes left, write
    // nothing and run no native step, and share the close's checks.
    let mut dry = crate::checkpoint::read_checkpoint(root)?;
    for listed in &checked {
        let stage = dry.stage.current.clone();
        let values = [
            ("--decided-by", args.decided_by.as_deref()),
            ("--words", args.words.as_deref()),
            ("--where", args.place.as_deref()),
            ("--recorded-by", args.recorded_by.as_deref()),
        ];
        let relayed =
            crate::checkpoint::check_relay(root, &dry.awaiting_human, Some(&listed.gate), values)
                .map_err(dry_relay_refusal)?;
        let (gate, relay, _decision) = relayed.expect("a close names its gate");
        // The spawned close only receives --verdict when its gate drives a native step
        // (close_command), so the dry run rehearses exactly that much of the reply.
        let verdict = if drives(listed) != crate::humans::Drives::None {
            args.verdict.as_deref()
        } else {
            None
        };
        let (native, native_text, native_human) = crate::checkpoint::native_step_for(
            root,
            &dry,
            &gate,
            &relay,
            verdict,
            args.no_native,
        )
        .map_err(|error| {
            let named: Vec<String> = checked
                .iter()
                .map(|listed| listed.number.to_string())
                .collect();
            anyhow::anyhow!(
                "closing gate {} (\"{}\") was refused: {}\nRun `chough gate list`, then `chough gate record` for gates {} again.\nNo gate was closed and nothing was written, for the reply naming gates {}.",
                listed.number,
                listed.gate,
                error,
                named.join(","),
                named.join(",")
            )
        })?;
        // REQ-history-010: the entry names the SpecSync command the close runs and the
        // actor it runs as, as the real close sets them before it renders.
        let mut relay = relay;
        if let Some(step) = &native {
            let verb = match step.step {
                crate::humans::Step::Approve { .. } => "approve",
                crate::humans::Step::Review { .. } => "review",
            };
            relay.native = Some(format!("specsync change {verb} {}", step.change));
            relay.actor = Some(step.human.specsync_actor.clone());
        }
        let decision = crate::checkpoint::decision_text(
            &gate,
            &relay,
            native_text.as_deref(),
            native_human
                .as_ref()
                .map(|human| human.name.as_str())
                .unwrap_or(&relay.decided_by),
        );
        if decision.len() > crate::stage::RELAY_TEXT_LIMIT {
            bail!(
                "refused:relay-too-long: the decision for gate \"{gate}\" would be {} bytes, over the {}-byte checkpoint text bound, so nothing ran or was saved.\nShorten --words or --where (keep the full text in a committed file and name it in --where), then run `chough checkpoint --close-gate \"{gate}\"` with the four values again.",
                decision.len(),
                crate::stage::RELAY_TEXT_LIMIT
            );
        }
        crate::stage::log_gate(
            root,
            &mut dry,
            crate::stage::GATE_CLOSED,
            &stage,
            &listed.gate,
            Some(relay.clone()),
        )?;
        dry.awaiting_human.retain(|open| open != &listed.gate);
        dry.decisions.push(decision.clone());
        let holds = crate::session_md::gate(&dry, &listed.gate).holds.clone();
        // The real close sets the next action before its save renders (the decision's
        // text is in the front matter), and the save refreshes the evidence links before
        // it renders, so the dry run mirrors that order exactly.
        crate::checkpoint::apply_gate_close_next_action(
            &mut dry,
            &listed.gate,
            &holds,
            false,
            args.verdict.as_deref(),
            args.no_native,
            false,
        );
        dry.evidence_links = crate::checkpoint::evidence_links(root, &dry);
        let (record, _redacted) = crate::checkpoint::render_counted(&dry)?;
        // The save a real close runs next settles the entry it queued, so the state the
        // next named gate's dry run checks is the one the earlier closes left.
        dry.source = Some(record);
        dry.pending.clear();
    }
    // The dry runs held the checkpoint writer lock for the rest of this process; release
    // it so the real closes below take it again before they read and save (REQ-cli-040).
    crate::checkpoint::release_checkpoint_locks();
    for (index, listed) in checked.iter().enumerate() {
        let mut command = close_command(
            root,
            &chough,
            listed,
            &drives(listed),
            args.decided_by.as_deref().unwrap_or_default(),
            args.words.as_deref().unwrap_or_default(),
            args.place.as_deref().unwrap_or_default(),
            args.recorded_by.as_deref().unwrap_or_default(),
            &listed.digest,
            &args.list,
            false,
            args.no_native,
            args.verdict.as_deref(),
        );
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
                quoted_refusal(&String::from_utf8_lossy(&output.stderr)),
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

/// A refused close's standard error as `gate record` quotes it: the close's own `Error: `
/// prefix is dropped from the line that carries it, after any line printed before it, so
/// the refusal says `Error:` once (#380).
fn quoted_refusal(stderr: &str) -> String {
    stderr
        .trim()
        .lines()
        .map(|line| line.strip_prefix("Error: ").unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// The refusal a dry run of one named gate's close hits, as the close words it
/// (fix-438): the relay values the close would have checked.
fn dry_relay_refusal(problem: crate::checkpoint::RelayProblem) -> anyhow::Error {
    use crate::checkpoint::RelayProblem;
    match problem {
        RelayProblem::WithoutGate(given) => anyhow::anyhow!(
            "refused:relay-without-gate: {} describe a human's decision at a gate, so they need --close-gate with that gate's exact text; nothing was saved.\nRun `chough next` to list the open gates, then `{}` with one of them.",
            given.join(", "),
            crate::stage::CLOSE_GATE_COMMAND
        ),
        RelayProblem::Incomplete(gate, missing) => anyhow::anyhow!(
            "refused:relay-incomplete: closing gate \"{}\" on a human's behalf needs who decided, their words, where they said them and the recording agent; {} {} missing or blank, so nothing was saved.\nRun `chough checkpoint --close-gate \"{}\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"`.",
            crate::leaks::shown("gate", &gate),
            missing.join(", "),
            if missing.len() == 1 { "is" } else { "are" },
            if crate::leaks::unshowable(&gate).is_none() {
                gate.clone()
            } else {
                "<gate>".to_string()
            }
        ),
        RelayProblem::NotOpen(gate, closest) => anyhow::anyhow!(
            "refused:gate-not-open: no open gate has the exact text \"{}\", so nothing was saved.{ }\nRun `chough next` to list the open gates, then close one by its exact text.",
            crate::leaks::shown("gate", &gate),
            closest
                .map(|closest| format!(" Did you mean \"{closest}\"?"))
                .unwrap_or_default()
        ),
        RelayProblem::TooLong(gate, length) => anyhow::anyhow!(
            "refused:relay-too-long: the decision for gate \"{}\" would be {length} bytes, over the {}-byte checkpoint text bound, so nothing was saved.\nShorten --words or --where (keep the full text in a committed file and name it in --where), then run `chough checkpoint --close-gate \"{}\"` with the four values again.",
            crate::leaks::shown("gate", &gate),
            crate::stage::RELAY_TEXT_LIMIT,
            if crate::leaks::unshowable(&gate).is_none() {
                gate.clone()
            } else {
                "<gate>".to_string()
            }
        ),
        RelayProblem::Identity(error) => error,
    }
}

/// The close `chough checkpoint --close-gate <gate>` runs for one listed gate's reply,
/// through Chough's own command, so the record and its checks are exactly that command's;
/// with `dry_run`, the close is a rehearsal that writes nothing and runs no native step.
#[allow(clippy::too_many_arguments)]
fn close_command(
    root: &Path,
    chough: &Path,
    listed: &Listed,
    drives: &crate::humans::Drives,
    decided_by: &str,
    words: &str,
    place: &str,
    recorded_by: &str,
    digest: &str,
    list: &str,
    dry_run: bool,
    no_native: bool,
    verdict: Option<&str>,
) -> Command {
    let mut command = Command::new(chough);
    command.arg("--root").arg(root);
    // REQ-records-006: a `gate record --redact` redacts in the close it runs too.
    if crate::leaks::redacting() {
        command.arg("--redact");
    }
    command
        .args(["checkpoint", "--close-gate", &listed.gate])
        .args(["--decided-by", decided_by])
        .args(["--words", words])
        .args(["--where", place])
        .args(["--recorded-by", recorded_by])
        .args(["--summary-digest", digest, "--list", list])
        .current_dir(root);
    if dry_run {
        // fix-438: a dry run of the close adds its own flag, so the close checks every gate
        // the close checks but writes nothing and runs no native step.
        command.arg("--dry-run");
    }
    if !dry_run && *drives != crate::humans::Drives::None {
        if no_native {
            command.arg("--no-native");
        } else if let Some(verdict) = verdict {
            command.args(["--verdict", verdict]);
        }
    }
    command
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

    /// #380: the close's `Error: ` prefix is dropped wherever its line sits, so a line
    /// printed before it, such as a `--redact` count, leaves one `Error:` in the refusal.
    #[test]
    fn a_quoted_refusal_says_error_once() {
        assert_eq!(
            super::quoted_refusal("Error: refused:leak: x\n"),
            "refused:leak: x"
        );
        assert_eq!(
            super::quoted_refusal("chough: redacted 1 value(s)\nError: refused:leak: x"),
            "chough: redacted 1 value(s)\nrefused:leak: x"
        );
        assert_eq!(super::quoted_refusal("no prefix"), "no prefix");
    }
}
