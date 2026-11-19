//! Readiness before approval (REQ-cli-067, DRIVE-29).
//!
//! `chough change ready <id>` reads one change and prints every piece still missing.
//! It does not approve, check, or rewrite a tracked file. SpecSync's gitignored hash
//! cache is restored when an export check writes it.

use anyhow::{Context, Result, anyhow};
use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Write};
use std::ops::Range;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const SPECSYNC: &str = "specsync";
const INSTALL: &str = "cargo install --locked specsync@6.3.0";
const DELTA_DOMAIN: &[u8] = b"specsync.approved-delta.v1";
const BASE_DOMAIN: &[u8] = b"specsync.approved-delta-base.v1";
const HISTORY_LIMIT: usize = 200;
const LINE_LIMIT: usize = 1_000;
const PLAN_SECTIONS: [&str; 5] = [
    "approach",
    "out of scope",
    "steps",
    "risks",
    "constraints consulted",
];

struct Finding {
    text: String,
    fix: String,
}

struct Change {
    id: String,
    state: String,
    base_commit: String,
    affected_specs: Vec<String>,
    affected_paths: Vec<String>,
    no_spec_change: bool,
    selected_artifacts: Vec<String>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Operation {
    Added,
    Modified,
    Removed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Target {
    Requirement,
    SpecSection,
}

#[derive(Clone)]
struct Item {
    operation: Operation,
    target: Target,
    key: String,
    content: String,
}

#[derive(Deserialize)]
struct Ledger {
    #[serde(default)]
    approvals: Vec<Approval>,
}

#[derive(Deserialize)]
struct Approval {
    gate: String,
    #[serde(default)]
    approved_delta_digests: Option<BTreeMap<String, String>>,
    #[serde(default)]
    approved_delta_bases: Option<BTreeMap<String, String>>,
}

/// Print the readiness report and return 0 when it is empty, 1 when it is not.
pub(super) fn run(root: &Path, id: &str) -> Result<i32> {
    if !valid_id(id) || !change_dir(root, id).join("state.json").is_file() {
        let mut err = io::stderr().lock();
        // An invalid id is not shown as typed: it can hold anything (REQ-records-006).
        if valid_id(id) {
            writeln!(err, "no change `{id}` to read")?;
        } else {
            writeln!(err, "no change to read: that is not a valid change id")?;
        }
        writeln!(err, "Fix: `chough change status`")?;
        err.flush()?;
        return Ok(1);
    }
    let change = load(root, id)?;
    let findings = collect(root, &change);
    let mut out = io::stdout().lock();
    writeln!(out, "chough change ready {}", change.id)?;
    writeln!(out)?;
    if findings.is_empty() {
        writeln!(out, "Nothing is missing.")?;
    } else {
        for finding in &findings {
            writeln!(out, "- {}", finding.text)?;
            writeln!(out, "  Fix: `{}`", finding.fix)?;
        }
        writeln!(out)?;
        let noun = if findings.len() == 1 {
            "finding"
        } else {
            "findings"
        };
        writeln!(out, "{} {noun}.", findings.len())?;
    }
    out.flush()?;
    Ok(if findings.is_empty() { 0 } else { 1 })
}

fn valid_id(id: &str) -> bool {
    !id.is_empty()
        && !id.contains('/')
        && !id.contains('\\')
        && !id.contains("..")
        && id.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
}

fn change_dir(root: &Path, id: &str) -> PathBuf {
    root.join(".specsync/changes").join(id)
}

fn load(root: &Path, id: &str) -> Result<Change> {
    let path = change_dir(root, id).join("state.json");
    let text = fs::read_to_string(&path).with_context(|| {
        format!(
            "could not read {}; run `chough change status`",
            path.display()
        )
    })?;
    let value: Value = serde_json::from_str(&text)
        .with_context(|| format!("could not read {id} as a change; run `chough change status`"))?;
    let strings = |key: &str| {
        value
            .get(key)
            .and_then(Value::as_array)
            .map(|items| {
                items
                    .iter()
                    .filter_map(Value::as_str)
                    .map(str::to_string)
                    .collect()
            })
            .unwrap_or_default()
    };
    Ok(Change {
        id: id.to_string(),
        state: value
            .get("state")
            .and_then(Value::as_str)
            .unwrap_or("draft")
            .to_string(),
        base_commit: value
            .get("base_commit")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string(),
        affected_specs: strings("affected_specs"),
        affected_paths: strings("affected_paths"),
        no_spec_change: value
            .get("no_spec_change")
            .and_then(Value::as_bool)
            .unwrap_or(false),
        selected_artifacts: strings("selected_artifacts"),
    })
}

fn collect(root: &Path, change: &Change) -> Vec<Finding> {
    let mut findings = Vec::new();
    findings.extend(delta_findings(root, change));
    findings.extend(stale_findings(root, change));
    findings.extend(specsync_findings(root, change));
    findings.extend(case_findings(root, change));
    findings.extend(path_findings(root, change));
    findings.extend(artifact_findings(root, change));
    findings
}

fn delta_findings(root: &Path, change: &Change) -> Vec<Finding> {
    let dir = change_dir(root, &change.id).join("deltas");
    let mut files = BTreeSet::new();
    if let Ok(entries) = fs::read_dir(&dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            files.insert(name);
        }
    }
    let mut findings = Vec::new();
    if change.no_spec_change {
        for name in files {
            let path = format!(".specsync/changes/{}/deltas/{name}", change.id);
            findings.push(Finding {
                text: format!("delta {name} is present on a change that declares no spec change"),
                fix: format!("remove {path}"),
            });
        }
        return findings;
    }
    let expected: BTreeSet<String> = change
        .affected_specs
        .iter()
        .map(|module| format!("{module}.md"))
        .collect();
    for module in &change.affected_specs {
        if !files.contains(&format!("{module}.md")) {
            let path = format!(".specsync/changes/{}/deltas/{module}.md", change.id);
            findings.push(Finding {
                text: format!("missing delta for {module}"),
                fix: format!("write {path}"),
            });
        }
    }
    for name in files {
        if !expected.contains(&name) {
            let path = format!(".specsync/changes/{}/deltas/{name}", change.id);
            findings.push(Finding {
                text: format!("extra delta {name}"),
                fix: format!("remove {path}"),
            });
        }
    }
    findings
}

fn stale_findings(root: &Path, change: &Change) -> Vec<Finding> {
    if change.no_spec_change {
        return Vec::new();
    }
    let Some((bases, bodies)) = recorded_bases(root, change) else {
        return Vec::new();
    };
    let mut findings = Vec::new();
    for module in &change.affected_specs {
        let path = change_dir(root, &change.id)
            .join("deltas")
            .join(format!("{module}.md"));
        let Ok(delta) = fs::read_to_string(&path) else {
            continue;
        };
        let Some(previous) = bodies.get(module) else {
            continue;
        };
        if delta_digest(module, &delta) != *previous {
            continue;
        }
        let Ok(items) = parse_delta(&delta) else {
            findings.push(Finding {
                text: format!("delta {module}.md cannot be read"),
                fix: format!("rewrite .specsync/changes/{}/deltas/{module}.md", change.id),
            });
            continue;
        };
        let (spec, requirements) = canonical_text(root, module);
        for item in items {
            if item.operation == Operation::Added || is_change_log(&item) {
                continue;
            }
            let key = base_key(module, &item);
            let Some(recorded) = bases.get(&key) else {
                continue;
            };
            if applied(&spec, &requirements, &item) {
                continue;
            }
            let current = block_digest(module, &spec, &requirements, &item);
            if current == *recorded {
                continue;
            }
            if carried(root, module, &item, recorded, &spec, &requirements) {
                continue;
            }
            let label = match item.target {
                Target::Requirement => item.key.clone(),
                Target::SpecSection => format!("section {}", item.key),
            };
            findings.push(Finding {
                text: format!("stale canonical text for {module} {label}"),
                fix: format!(
                    "rewrite .specsync/changes/{}/deltas/{module}.md and then chough change ready {}",
                    change.id, change.id
                ),
            });
        }
    }
    findings
}

fn recorded_bases(
    root: &Path,
    change: &Change,
) -> Option<(BTreeMap<String, String>, BTreeMap<String, String>)> {
    let text = fs::read_to_string(change_dir(root, &change.id).join("approvals.json")).ok()?;
    let ledger: Ledger = serde_json::from_str(&text).ok()?;
    let approval = ledger
        .approvals
        .into_iter()
        .rev()
        .find(|approval| approval.gate == "definition")?;
    Some((
        approval.approved_delta_bases?,
        approval.approved_delta_digests?,
    ))
}

fn carried(
    root: &Path,
    module: &str,
    item: &Item,
    recorded: &str,
    spec: &str,
    requirements: &str,
) -> bool {
    let relative = canonical_relative(root, module, item.target);
    let Some(old) = recover_base(root, &relative, module, item, recorded) else {
        return false;
    };
    let source = match item.target {
        Target::Requirement => requirements,
        Target::SpecSection => spec,
    };
    let current = block_body(source, item).unwrap_or_default();
    let approved = item
        .content
        .replace("\r\n", "\n")
        .trim_matches(['\n', ' ', '\t'])
        .to_string();
    three_way_keeps_body(&old, &current, &approved)
}

fn recover_base(
    root: &Path,
    relative: &str,
    module: &str,
    item: &Item,
    recorded: &str,
) -> Option<String> {
    let prefix = repo_prefix(root)?;
    let limit = HISTORY_LIMIT.to_string();
    let log = git(
        root,
        &["log", "--format=%H", "-n", limit.as_str(), "--", relative],
    )
    .ok()?;
    if !log.status.success() {
        return None;
    }
    let commits = String::from_utf8_lossy(&log.stdout);
    for commit in commits
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let object = format!("{commit}:{prefix}{relative}");
        let show = git(root, &["show", object.as_str()]).ok()?;
        if !show.status.success() {
            continue;
        }
        let text = String::from_utf8_lossy(&show.stdout).into_owned();
        let (spec, requirements) = match item.target {
            Target::Requirement => ("", text.as_str()),
            Target::SpecSection => (text.as_str(), ""),
        };
        if block_digest(module, spec, requirements, item) == recorded {
            let source = if spec.is_empty() { requirements } else { spec };
            return Some(block_body(source, item).unwrap_or_default());
        }
    }
    None
}

fn repo_prefix(root: &Path) -> Option<String> {
    let output = git(root, &["rev-parse", "--show-prefix"]).ok()?;
    if !output.status.success() {
        return None;
    }
    Some(
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .replace('\\', "/"),
    )
}

fn specsync_findings(root: &Path, change: &Change) -> Vec<Finding> {
    let mut findings = match specsync(root, &["change", "show", &change.id, "--json"]) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return vec![Finding {
                text: "specsync cannot be started, so interview questions and undocumented exports were not read".into(),
                fix: INSTALL.into(),
            }];
        }
        Err(_) => vec![Finding {
            text: format!("interview for {} could not be read", change.id),
            fix: "chough change status".into(),
        }],
        Ok(output) => questions_from(&change.id, &output),
    };
    findings.extend(export_findings(root, change));
    findings
}

fn questions_from(id: &str, output: &Output) -> Vec<Finding> {
    let text = String::from_utf8_lossy(&output.stdout);
    let Ok(value) = serde_json::from_str::<Value>(&text) else {
        return vec![Finding {
            text: format!("interview for {id} could not be read"),
            fix: "chough change status".into(),
        }];
    };
    let mut findings = Vec::new();
    for question in value
        .get("questions")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let Some(question_id) = question.get("id").and_then(Value::as_str) else {
            continue;
        };
        findings.push(Finding {
            text: format!("unanswered question {question_id}"),
            fix: format!("chough change answer {id} {question_id} \"<answer>\""),
        });
    }
    findings
}

fn export_findings(root: &Path, change: &Change) -> Vec<Finding> {
    let cache = root.join(".specsync/hashes.json");
    let mut before = Cache::observe(&cache);
    // With no cache yet, make the empty file SpecSync will fill, so the one removed
    // afterwards is provably this report's own: seeing a file after SpecSync ran does
    // not show who made it. If something appears first, it is not ours; look again.
    if before == Cache::Absent {
        before = match make_cache(&cache) {
            Some((device, inode)) => Cache::Made(device, inode),
            None => Cache::observe(&cache),
        };
    }
    let mut findings = Vec::new();
    for module in &change.affected_specs {
        let output = match specsync(root, &["check", "--spec", module, "--format", "json"]) {
            Ok(output) => output,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                findings.push(Finding {
                    text: "specsync cannot be started, so interview questions and undocumented exports were not read"
                        .into(),
                    fix: INSTALL.into(),
                });
                break;
            }
            Err(error) => {
                findings.push(Finding {
                    text: format!("specsync check --spec {module} could not be started ({error})"),
                    fix: format!("specsync check --spec {module} --format json"),
                });
                continue;
            }
        };
        let text = String::from_utf8_lossy(&output.stdout);
        let value: Value = match serde_json::from_str(&text) {
            Ok(value) => value,
            Err(_) => {
                findings.push(Finding {
                    text: format!("specsync check --spec {module} did not return JSON"),
                    fix: format!("specsync check --spec {module} --format json"),
                });
                continue;
            }
        };
        let warnings = value
            .get("warnings")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for warning in warnings {
            let Some(warning) = warning.as_str() else {
                continue;
            };
            let Some((symbol, file)) = export_warning(warning) else {
                continue;
            };
            if !change
                .affected_paths
                .iter()
                .any(|scope| covers(file, scope))
            {
                continue;
            }
            findings.push(Finding {
                text: format!("undocumented export '{symbol}' from {file}"),
                fix: format!("document {symbol} in specs/{module}/{module}.spec.md"),
            });
        }
    }
    // Not a finding: REQ-cli-067 lists what the report names, and nothing else.
    if !restore_cache(&cache, &before) {
        eprintln!(
            ".specsync/hashes.json was replaced while this report ran, so it was left as it is now"
        );
    }
    findings
}

fn export_warning(warning: &str) -> Option<(&str, &str)> {
    let rest = warning.strip_prefix("Undocumented export '")?;
    let (symbol, rest) = rest.split_once("' from ")?;
    let file = rest.trim();
    (!symbol.is_empty() && !file.is_empty()).then_some((symbol, file))
}

/// SpecSync's hash cache as one look found it, without following a symlink.
#[derive(PartialEq)]
enum Cache {
    Absent,
    /// A regular file: its device and inode, and its bytes.
    Regular(u64, u64, Vec<u8>),
    /// A symlink or anything else that is not a regular file. Never read or written.
    Other,
    /// No cache was there, so this report made an empty one: its device and inode.
    Made(u64, u64),
}

impl Cache {
    fn observe(path: &Path) -> Self {
        match fs::symlink_metadata(path) {
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Self::Absent,
            Ok(found) if found.file_type().is_file() => {}
            _ => return Self::Other,
        }
        let Ok((file, device, inode)) = open_cache(path, false) else {
            return Self::Other;
        };
        let mut bytes = Vec::new();
        let mut file = file;
        match io::Read::read_to_end(&mut file, &mut bytes) {
            Ok(_) => Self::Regular(device, inode, bytes),
            Err(_) => Self::Other,
        }
    }
}

/// Open the cache itself, never what a symlink at its path points to, and say which
/// file was opened.
fn open_cache(path: &Path, write: bool) -> io::Result<(fs::File, u64, u64)> {
    let mut options = fs::OpenOptions::new();
    options.read(!write).write(write);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt as _;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    let file = options.open(path)?;
    let found = file.metadata()?;
    if !found.is_file() {
        return Err(io::Error::other("not a regular file"));
    }
    #[cfg(unix)]
    let identity = {
        use std::os::unix::fs::MetadataExt as _;
        (found.dev(), found.ino())
    };
    #[cfg(not(unix))]
    let identity = (0, 0);
    Ok((file, identity.0, identity.1))
}

/// Make the empty cache file, only if nothing is at its path, and say which file it is.
fn make_cache(path: &Path) -> Option<(u64, u64)> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .ok()?;
    open_cache(path, false)
        .ok()
        .map(|(_, device, inode)| (device, inode))
}

/// Put the cache back as the report found it (REQ-cli-067), but only while it is still
/// the file this report can show is its own. A cache another process made or replaced
/// meanwhile, by a regular file or a symlink, is theirs: it is left alone and the
/// caller says so. False when it was left changed.
///
/// What is ours: with a cache before, the same file (device and inode), which SpecSync
/// rewrites in place; with none before, the empty file this report made. Another
/// writer that rewrites that same file in place cannot be told from SpecSync and is
/// not detected.
fn restore_cache(path: &Path, before: &Cache) -> bool {
    let now = Cache::observe(path);
    match (before, &now) {
        // Never ours to touch: it was not a regular file when the report started.
        (Cache::Other, _) => true,
        // Something appeared after the look and before this report could make its own.
        (Cache::Absent, _) => matches!(now, Cache::Absent),
        (Cache::Made(..), Cache::Absent) => true,
        (Cache::Made(device, inode), Cache::Regular(now_device, now_inode, _)) => {
            (device, inode) == (now_device, now_inode) && fs::remove_file(path).is_ok()
        }
        (Cache::Made(..), _) => false,
        (Cache::Regular(device, inode, bytes), Cache::Regular(now_device, now_inode, held)) => {
            if (device, inode) != (now_device, now_inode) {
                return false;
            }
            if held == bytes {
                return true;
            }
            let Ok((mut file, opened_device, opened_inode)) = open_cache(path, true) else {
                return false;
            };
            (opened_device, opened_inode) == (*device, *inode)
                && file.set_len(0).is_ok()
                && io::Write::write_all(&mut file, bytes).is_ok()
        }
        (Cache::Regular(..), _) => false,
    }
}

fn case_findings(root: &Path, change: &Change) -> Vec<Finding> {
    let changed = changed_files(root, &change.base_commit).unwrap_or_default();
    let mut names = BTreeSet::new();
    if let Ok(entries) = fs::read_dir(root.join(".chough/cases")) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !name.ends_with(".cases.toml") {
                continue;
            }
            let relative = format!(".chough/cases/{name}");
            let declared = change
                .affected_paths
                .iter()
                .any(|scope| covers(&relative, scope));
            if declared || changed.contains(&relative) {
                names.insert(relative);
            }
        }
    }
    let mut findings = Vec::new();
    for relative in names {
        let Ok(text) = fs::read_to_string(root.join(&relative)) else {
            continue;
        };
        let Ok(value) = toml::from_str::<toml::Value>(&text) else {
            findings.push(Finding {
                text: format!("{relative} cannot be read as cases"),
                fix: format!("rewrite {relative}"),
            });
            continue;
        };
        let module = value
            .get("module")
            .and_then(toml::Value::as_str)
            .unwrap_or("");
        let bound = module.replace('_', "-");
        let topic = case_topic(relative.rsplit('/').next().unwrap_or(&relative), module);
        let Some(cases) = value.get("case").and_then(toml::Value::as_array) else {
            continue;
        };
        for case in cases {
            let id = case
                .get("id")
                .and_then(toml::Value::as_str)
                .unwrap_or("case");
            let Some(requirements) = case.get("requirements").and_then(toml::Value::as_array)
            else {
                continue;
            };
            for requirement in requirements {
                let Some(requirement) = requirement.as_str() else {
                    continue;
                };
                let Some(owner) = requirement_owner(requirement) else {
                    continue;
                };
                if owner == bound {
                    continue;
                }
                findings.push(Finding {
                    text: format!("{id} links {requirement}, owned by {owner}"),
                    fix: format!("move {id} to .chough/cases/{owner}.{topic}.cases.toml"),
                });
            }
        }
    }
    findings
}

fn case_topic(file_name: &str, module: &str) -> String {
    let stem = file_name.trim_end_matches(".cases.toml");
    for candidate in [module, module.replace('_', "-").as_str()] {
        if let Some(topic) = stem.strip_prefix(&format!("{candidate}."))
            && !topic.is_empty()
        {
            return topic.to_string();
        }
    }
    "moved".into()
}

fn requirement_owner(id: &str) -> Option<String> {
    let rest = id.strip_prefix("REQ-")?;
    let (module, number) = rest.rsplit_once('-')?;
    (number.chars().all(|character| character.is_ascii_digit())
        && !number.is_empty()
        && !module.is_empty())
    .then(|| module.to_string())
}

fn path_findings(root: &Path, change: &Change) -> Vec<Finding> {
    let changed = match changed_files(root, &change.base_commit) {
        Ok(changed) => changed,
        Err(error) => {
            return vec![Finding {
                text: format!(
                    "changed files since {} could not be listed ({error})",
                    change.base_commit
                ),
                fix: "chough change status".into(),
            }];
        }
    };
    let ignored = ignored_prefixes(root);
    let workspace = format!(".specsync/changes/{}", change.id);
    let outside: Vec<String> = changed
        .into_iter()
        .filter(|path| {
            !change
                .affected_paths
                .iter()
                .any(|scope| covers(path, scope))
        })
        .filter(|path| !ignored.iter().any(|prefix| covers(path, prefix)))
        .filter(|path| !covers(path, &workspace))
        .collect();
    if outside.is_empty() {
        return Vec::new();
    }
    let mut declared = change.affected_paths.clone();
    for path in &outside {
        if !declared.iter().any(|scope| covers(path, scope)) {
            declared.push(path.clone());
        }
    }
    let list = declared.join(",");
    let fix = if change.state.eq_ignore_ascii_case("draft") {
        format!(
            "chough change answer {} affected_paths \"{list}\"",
            change.id
        )
    } else {
        format!(
            "specsync change send-back {} --actor <name> --reason \"{}\"",
            change.id,
            outside.join(",")
        )
    };
    outside
        .into_iter()
        .map(|path| Finding {
            text: format!("changed file {path} is outside the declared paths"),
            fix: fix.clone(),
        })
        .collect()
}

fn ignored_prefixes(root: &Path) -> Vec<String> {
    let Ok(text) = fs::read_to_string(root.join(".specsync/sdd.json")) else {
        return Vec::new();
    };
    let Ok(value) = serde_json::from_str::<Value>(&text) else {
        return Vec::new();
    };
    value
        .get("verification_ignored_paths")
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn changed_files(root: &Path, base: &str) -> Result<BTreeSet<String>> {
    if base.is_empty() {
        return Err(anyhow!("the change records no base commit"));
    }
    let run = |args: &[&str]| -> Result<BTreeSet<String>> {
        let output = git(root, args)?;
        if !output.status.success() {
            let detail = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!(detail.trim().to_string()));
        }
        Ok(names(&output.stdout))
    };
    // What differs from the base now, committed or not.
    let differs = run(&["diff", "--name-only", "-z", "--no-renames", base])?;
    // #363: only the change's own work counts. A file that a merge from another branch
    // brought in differs from the base too, but no commit of this branch's own line
    // touched it, so it is left out; a file changed and not committed yet is the
    // change's own.
    let range = format!("{base}..HEAD");
    let own = run(&[
        "log",
        "--first-parent",
        // For a merge commit, the combined diff: only the files its result changes
        // against every parent, such as a resolution's own edit, and none that came
        // from the other branch unchanged.
        "-c",
        "--name-only",
        "--no-renames",
        "-z",
        "--format=",
        &range,
    ])?;
    let uncommitted = run(&["diff", "--name-only", "-z", "--no-renames", "HEAD"])?;
    let untracked = run(&["ls-files", "-z", "--others", "--exclude-standard"])?;
    let mut files: BTreeSet<String> = differs
        .into_iter()
        .filter(|path| own.contains(path) || uncommitted.contains(path))
        .collect();
    files.extend(untracked);
    Ok(files)
}

fn names(bytes: &[u8]) -> BTreeSet<String> {
    bytes
        .split(|byte| *byte == 0)
        .filter(|name| !name.is_empty())
        .map(|name| String::from_utf8_lossy(name).replace('\\', "/"))
        .collect()
}

fn covers(path: &str, scope: &str) -> bool {
    let scope = scope.replace('\\', "/");
    let scope = scope.trim_end_matches('/');
    if scope.is_empty() || scope == "." {
        return scope == ".";
    }
    path == scope
        || path
            .strip_prefix(scope)
            .is_some_and(|rest| rest.starts_with('/'))
}

fn artifact_findings(root: &Path, change: &Change) -> Vec<Finding> {
    let mut findings = Vec::new();
    for artifact in &change.selected_artifacts {
        let relative = format!(".specsync/changes/{}/{artifact}.md", change.id);
        let text = fs::read_to_string(root.join(&relative)).unwrap_or_default();
        if incomplete(&text) {
            findings.push(Finding {
                text: format!("{relative} is still a TODO placeholder"),
                fix: format!("complete {relative}"),
            });
        }
        if artifact == "plan" {
            for section in missing_plan_sections(&text) {
                findings.push(Finding {
                    text: format!("{relative} is missing {section}"),
                    fix: format!("complete {relative}"),
                });
            }
        }
    }
    findings
}

fn incomplete(content: &str) -> bool {
    if content.contains("<!-- TODO") {
        return true;
    }
    let body = strip_frontmatter(content);
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if !placeholder(line) {
            return false;
        }
    }
    true
}

fn placeholder(line: &str) -> bool {
    let mut text = line.trim();
    while let Some(stripped) = text.strip_prefix('#') {
        text = stripped.trim_start();
    }
    let lower = text.to_ascii_lowercase();
    lower == "todo"
        || lower.starts_with("todo:")
        || lower.starts_with("todo ")
        || lower == "[ ] todo"
        || lower.starts_with("- [ ] todo")
}

fn missing_plan_sections(content: &str) -> Vec<&'static str> {
    let body = strip_comments(&strip_frontmatter(content));
    let mut sections: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut current: Option<String> = None;
    let mut fence: Option<(char, usize)> = None;
    for line in body.lines() {
        if let Some((marker, length)) = fence {
            if let Some(mark) = fence_mark(line)
                && mark.0 == marker
                && mark.1 >= length
                && mark.2
            {
                fence = None;
            }
            continue;
        } else if let Some(mark) = fence_mark(line) {
            fence = Some((mark.0, mark.1));
            continue;
        }
        if let Some(title) = line.trim().strip_prefix("## ") {
            let key = title.trim().to_ascii_lowercase();
            if PLAN_SECTIONS.contains(&key.as_str()) {
                current = Some(key.clone());
                sections.entry(key).or_default();
            } else {
                current = None;
            }
            continue;
        }
        if let Some(key) = &current {
            sections
                .entry(key.clone())
                .or_default()
                .push(line.to_string());
        }
    }
    PLAN_SECTIONS
        .into_iter()
        .filter(|title| {
            sections.get(*title).is_none_or(|lines| {
                !lines.iter().any(|line| {
                    let line = line.trim();
                    !line.is_empty()
                        && !line.starts_with("<!--")
                        && !line.starts_with('#')
                        && !placeholder(line)
                })
            })
        })
        .collect()
}

/// A CommonMark fence: marker, length, and whether only whitespace follows it.
fn fence_mark(line: &str) -> Option<(char, usize, bool)> {
    let indent = line
        .chars()
        .take_while(|character| *character == ' ')
        .count();
    if indent > 3 {
        return None;
    }
    let rest = &line[indent..];
    let marker = rest.chars().next()?;
    if !matches!(marker, '`' | '~') {
        return None;
    }
    let length = rest
        .chars()
        .take_while(|character| *character == marker)
        .count();
    if length < 3 {
        return None;
    }
    Some((marker, length, rest[length..].trim().is_empty()))
}

fn strip_frontmatter(content: &str) -> String {
    let Some(rest) = content
        .strip_prefix("---\n")
        .or_else(|| content.strip_prefix("---\r\n"))
    else {
        return content.to_string();
    };
    if let Some(end) = rest.find("\n---\n").or_else(|| rest.find("\n---\r\n")) {
        rest[end + 1..]
            .trim_start_matches('-')
            .trim_start_matches('\r')
            .trim_start_matches('\n')
            .to_string()
    } else {
        content.to_string()
    }
}

fn strip_comments(content: &str) -> String {
    let mut kept = String::new();
    let mut rest = content;
    while let Some(start) = rest.find("<!--") {
        kept.push_str(&rest[..start]);
        rest = &rest[start + 4..];
        if let Some(end) = rest.find("-->") {
            rest = &rest[end + 3..];
        } else {
            rest = "";
        }
    }
    kept.push_str(rest);
    kept
}

fn specsync(root: &Path, args: &[&str]) -> io::Result<Output> {
    let mut command = Command::new(SPECSYNC);
    command.args(args).current_dir(root);
    crate::records::run_owned(&mut command, None)
}

fn git(root: &Path, args: &[&str]) -> io::Result<Output> {
    let mut command = Command::new("git");
    command.arg("-C").arg(root).args(args);
    crate::records::run_owned(&mut command, None)
}

fn canonical_text(root: &Path, module: &str) -> (String, String) {
    let dir = root.join(specs_dir(root)).join(module);
    let read = |path: PathBuf| fs::read_to_string(path).unwrap_or_default();
    (
        read(dir.join(format!("{module}.spec.md"))),
        read(dir.join("requirements.md")),
    )
}

fn canonical_relative(root: &Path, module: &str, target: Target) -> String {
    let dir = format!("{}/{module}", specs_dir(root));
    match target {
        Target::Requirement => format!("{dir}/requirements.md"),
        Target::SpecSection => format!("{dir}/{module}.spec.md"),
    }
}

fn specs_dir(root: &Path) -> String {
    let Ok(text) = fs::read_to_string(root.join(".specsync/config.toml")) else {
        return "specs".into();
    };
    for line in text.lines() {
        let line = line.trim();
        let Some(value) = line.strip_prefix("specs_dir") else {
            continue;
        };
        let value = value
            .trim()
            .trim_start_matches('=')
            .trim()
            .trim_matches(['"', '\'']);
        if !value.is_empty() {
            return value.to_string();
        }
    }
    "specs".into()
}

fn applied(spec: &str, requirements: &str, item: &Item) -> bool {
    let (source, prefix) = match item.target {
        Target::Requirement => (requirements, "### "),
        Target::SpecSection => (spec, "## "),
    };
    let heading = format!("{prefix}{}", item.key);
    match block_range(source, &heading, prefix_level(prefix)) {
        None => item.operation == Operation::Removed,
        Some(range) => {
            item.operation != Operation::Removed
                && block_matches(&source[range], &heading, &item.content)
        }
    }
}

fn is_change_log(item: &Item) -> bool {
    item.target == Target::SpecSection && item.key == "Change Log"
}

fn block_matches(existing: &str, heading: &str, content: &str) -> bool {
    let existing = normalize(existing);
    let (header, body) = existing.split_once('\n').unwrap_or((&existing, ""));
    heading_matches(header, &normalize(heading)) && normalize(body) == normalize(content)
}

fn normalize(value: &str) -> String {
    value
        .replace("\r\n", "\n")
        .trim_matches(['\n', ' ', '\t'])
        .to_string()
}

fn block_body(source: &str, item: &Item) -> Option<String> {
    let prefix = match item.target {
        Target::Requirement => "### ",
        Target::SpecSection => "## ",
    };
    let heading = format!("{prefix}{}", item.key);
    let range = block_range(source, &heading, prefix_level(prefix))?;
    let body = source[range].split_once('\n').map_or("", |(_, rest)| rest);
    Some(body.trim_matches(['\n', ' ', '\t']).to_string())
}

fn heading_matches(line: &str, heading: &str) -> bool {
    if line.trim_end() == heading {
        return true;
    }
    let Some(key) = heading.strip_prefix("### ") else {
        return false;
    };
    requirement_id(line) == Some(key)
}

fn requirement_id(line: &str) -> Option<&str> {
    let indent = line
        .chars()
        .take_while(|character| *character == ' ')
        .count();
    if indent > 3 {
        return None;
    }
    let rest = line[indent..].strip_prefix("### ")?.trim();
    let token = rest.split_whitespace().next().unwrap_or(rest);
    let token = token.trim_matches(|character: char| {
        matches!(character, ':' | ',' | '.' | ';' | ')' | '(' | ']' | '[')
    });
    let mut parts = token.split('-');
    if parts.next() != Some("REQ") {
        return None;
    }
    let module = parts.next()?;
    if module.is_empty()
        || !module
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
    {
        return None;
    }
    let rest: Vec<&str> = parts.collect();
    let last = *rest.last()?;
    (rest.iter().all(|part| {
        !part.is_empty()
            && part
                .chars()
                .all(|character| character.is_ascii_alphanumeric())
    }) && last.chars().all(|character| character.is_ascii_digit()))
    .then_some(token)
}

fn block_range(source: &str, heading: &str, level: usize) -> Option<Range<usize>> {
    let mut lines = Vec::new();
    let mut offset = 0;
    for raw in source.split_inclusive('\n') {
        let line = raw.strip_suffix('\n').unwrap_or(raw);
        let line = line.strip_suffix('\r').unwrap_or(line);
        lines.push((offset, line.to_string()));
        offset += raw.len();
    }
    let start = lines
        .iter()
        .position(|(_, line)| heading_matches(line, heading))?;
    let end = lines
        .iter()
        .skip(start + 1)
        .find(|(_, line)| heading_level(line).is_some_and(|found| found <= level))
        .map_or(source.len(), |(next, _)| *next);
    Some(lines[start].0..end)
}

fn prefix_level(prefix: &str) -> usize {
    prefix
        .chars()
        .take_while(|character| *character == '#')
        .count()
}

fn heading_level(line: &str) -> Option<usize> {
    let indent = line
        .chars()
        .take_while(|character| *character == ' ')
        .count();
    if indent > 3 {
        return None;
    }
    let line = &line[indent..];
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if level == 0 || line.as_bytes().get(level) != Some(&b' ') {
        return None;
    }
    Some(level)
}

fn parse_delta(content: &str) -> Result<Vec<Item>, String> {
    let mut operation = None;
    let mut target = None;
    let mut key = String::new();
    let mut body = Vec::new();
    let mut items = Vec::new();
    let flush = |items: &mut Vec<Item>,
                 operation: Option<Operation>,
                 target: Option<Target>,
                 key: &str,
                 body: &mut Vec<String>| {
        if let (Some(operation), Some(target)) = (operation, target) {
            items.push(Item {
                operation,
                target,
                key: key.trim().to_string(),
                content: body.join("\n").trim().to_string(),
            });
        }
        body.clear();
    };
    for line in content.lines() {
        if let Some(header) = line.strip_prefix("## ") {
            flush(&mut items, operation, target, &key, &mut body);
            target = None;
            key.clear();
            operation = match header.trim().to_ascii_uppercase().as_str() {
                "ADDED" => Some(Operation::Added),
                "MODIFIED" => Some(Operation::Modified),
                "REMOVED" => Some(Operation::Removed),
                _ => {
                    return Err(format!("invalid delta operation heading `## {header}`"));
                }
            };
            continue;
        }
        if let Some(header) = line.strip_prefix("### ") {
            let parsed = strip_prefix_ignore_ascii(header, "REQUIREMENT ")
                .map(|value| (Target::Requirement, value))
                .or_else(|| {
                    strip_prefix_ignore_ascii(header, "SPEC SECTION ")
                        .map(|value| (Target::SpecSection, value))
                });
            if let Some((next_target, next_key)) = parsed {
                flush(&mut items, operation, target, &key, &mut body);
                target = Some(next_target);
                key = next_key.trim().to_string();
            } else if target.is_some() {
                body.push(line.to_string());
            } else {
                return Err(format!("invalid delta item heading `### {header}`"));
            }
            continue;
        }
        if target.is_some() {
            body.push(line.to_string());
        }
    }
    flush(&mut items, operation, target, &key, &mut body);
    if items.iter().any(|item| item.key.is_empty()) {
        return Err("semantic delta contains an item with an empty key".into());
    }
    Ok(items)
}

fn strip_prefix_ignore_ascii<'a>(value: &'a str, prefix: &str) -> Option<&'a str> {
    // `get` gives nothing for a value shorter than the prefix or cut inside a
    // character, where a plain slice would panic (the review of #311).
    let (head, rest) = (value.get(..prefix.len())?, value.get(prefix.len()..)?);
    head.eq_ignore_ascii_case(prefix).then_some(rest)
}

fn delta_digest(module: &str, body: &str) -> String {
    let mut digest = Framed::new(DELTA_DOMAIN);
    digest.frame(b"module", module.as_bytes());
    digest.frame(b"body", fold_crlf(body).as_bytes());
    digest.finish()
}

fn block_digest(module: &str, spec: &str, requirements: &str, item: &Item) -> String {
    let (source, prefix) = match item.target {
        Target::Requirement => (requirements, "### "),
        Target::SpecSection => (spec, "## "),
    };
    let heading = format!("{prefix}{}", item.key);
    let mut digest = Framed::new(BASE_DOMAIN);
    digest.frame(b"key", base_key(module, item).as_bytes());
    match block_range(source, &heading, prefix_level(prefix)) {
        None => digest.frame(b"absent", b""),
        Some(range) => {
            let block = fold_crlf(&source[range]);
            digest.frame(b"block", block.trim_matches(['\n', ' ', '\t']).as_bytes());
        }
    }
    digest.finish()
}

fn base_key(module: &str, item: &Item) -> String {
    let target = match item.target {
        Target::Requirement => "REQUIREMENT",
        Target::SpecSection => "SPEC SECTION",
    };
    format!("{module}: {target} {}", item.key)
}

fn fold_crlf(body: &str) -> std::borrow::Cow<'_, str> {
    if body.contains('\r') {
        std::borrow::Cow::Owned(body.replace("\r\n", "\n"))
    } else {
        std::borrow::Cow::Borrowed(body)
    }
}

struct Framed {
    hasher: Sha256,
}

impl Framed {
    fn new(domain: &[u8]) -> Self {
        let mut digest = Self {
            hasher: Sha256::new(),
        };
        digest.frame(b"domain", domain);
        digest
    }

    fn frame(&mut self, tag: &[u8], value: &[u8]) {
        self.hasher.update((tag.len() as u64).to_be_bytes());
        self.hasher.update(tag);
        self.hasher.update((value.len() as u64).to_be_bytes());
        self.hasher.update(value);
    }

    fn finish(self) -> String {
        format!("{:x}", self.hasher.finalize())
    }
}

fn three_way_keeps_body(base: &str, current: &str, approved: &str) -> bool {
    let base: Vec<&str> = base.lines().collect();
    let current: Vec<&str> = current.lines().collect();
    let approved: Vec<&str> = approved.lines().collect();
    if [base.len(), current.len(), approved.len()]
        .iter()
        .any(|lines| *lines > LINE_LIMIT)
    {
        return false;
    }
    let to_current = lcs(&base, &current);
    let to_approved = lcs(&base, &approved);
    let (mut b, mut c, mut a) = (0usize, 0usize, 0usize);
    loop {
        while b < base.len() && to_current[b] == Some(c) && to_approved[b] == Some(a) {
            b += 1;
            c += 1;
            a += 1;
        }
        if b == base.len() && c == current.len() && a == approved.len() {
            return true;
        }
        let next = (b..base.len()).find(|&index| {
            to_current[index].is_some_and(|line| line >= c)
                && to_approved[index].is_some_and(|line| line >= a)
        });
        let (base_end, current_end, approved_end) = match next {
            Some(index) => (
                index,
                to_current[index].unwrap_or(current.len()),
                to_approved[index].unwrap_or(approved.len()),
            ),
            None => (base.len(), current.len(), approved.len()),
        };
        let base_chunk = &base[b..base_end];
        let current_chunk = &current[c..current_end];
        let approved_chunk = &approved[a..approved_end];
        if current_chunk != base_chunk && current_chunk != approved_chunk {
            return false;
        }
        b = base_end;
        c = current_end;
        a = approved_end;
        if next.is_none() {
            return true;
        }
    }
}

fn lcs(left: &[&str], right: &[&str]) -> Vec<Option<usize>> {
    let (rows, columns) = (left.len(), right.len());
    let mut lengths = vec![0u32; (rows + 1) * (columns + 1)];
    let at = |row: usize, column: usize| row * (columns + 1) + column;
    for row in (0..rows).rev() {
        for column in (0..columns).rev() {
            lengths[at(row, column)] = if left[row] == right[column] {
                lengths[at(row + 1, column + 1)] + 1
            } else {
                lengths[at(row + 1, column)].max(lengths[at(row, column + 1)])
            };
        }
    }
    let mut map = vec![None; rows];
    let (mut row, mut column) = (0usize, 0usize);
    while row < rows && column < columns {
        if left[row] == right[column] {
            map[row] = Some(column);
            row += 1;
            column += 1;
        } else if lengths[at(row + 1, column)] >= lengths[at(row, column + 1)] {
            row += 1;
        } else {
            column += 1;
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The review of #311: a heading shorter than the prefix, or one whose text is cut
    /// inside a character at the prefix's length, is no match and never a panic.
    #[test]
    fn a_short_or_multi_byte_heading_is_no_prefix_match() {
        assert_eq!(strip_prefix_ignore_ascii("Notes", "REQUIREMENT "), None);
        assert_eq!(strip_prefix_ignore_ascii("", "REQUIREMENT "), None);
        // 12 bytes in, this text is inside a two-byte character.
        assert_eq!(
            strip_prefix_ignore_ascii("Ünïcödé notes", "REQUIREMENT "),
            None
        );
        assert_eq!(
            strip_prefix_ignore_ascii("Übersicht", "SPEC SECTION "),
            None
        );
        assert_eq!(
            strip_prefix_ignore_ascii("requirement REQ-cli-001", "REQUIREMENT "),
            Some("REQ-cli-001")
        );
        assert_eq!(
            strip_prefix_ignore_ascii("REQUIREMENT Ünïcödé", "REQUIREMENT "),
            Some("Ünïcödé")
        );
    }

    #[test]
    fn the_framed_delta_digest_matches_specsyncs_construction() {
        let body = "## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nOld line.\n";
        assert_eq!(
            delta_digest("cli", body),
            "26e8d49573f595e176975eb425d1e937808a08a86943633e66461a0d56cc9b07"
        );
        let requirements = "### REQ-cli-001\n\nOld line.\n";
        let item = Item {
            operation: Operation::Modified,
            target: Target::Requirement,
            key: "REQ-cli-001".into(),
            content: "Old line.".into(),
        };
        assert_eq!(
            block_digest("cli", "", requirements, &item),
            "1d58afd5faaa83ca253b19d8f3031eeb35e76ba2aa45c824f15fd062300aaf34"
        );
    }

    #[test]
    fn a_body_that_already_holds_the_canonical_insertion_is_carried() {
        assert!(three_way_keeps_body(
            "alpha\ntail",
            "alpha\ngamma\ntail",
            "alpha\ngamma\ntail\nbeta"
        ));
        assert!(!three_way_keeps_body("Old line.", "New line.", "Old line."));
    }

    #[test]
    fn a_declared_prefix_covers_a_file_under_it_and_not_a_sibling() {
        assert!(covers("src/ready.rs", "src/"));
        assert!(covers("src/ready.rs", "src/ready.rs"));
        assert!(!covers("src-extra/ready.rs", "src"));
        assert!(!covers(".chough/sessions/a.md", ".chough/archive/"));
    }
}
