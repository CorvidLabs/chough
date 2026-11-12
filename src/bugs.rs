//! Local-first bug backlog (REQ-cli-012). Each bug is one Markdown record,
//! `.chough/bugs/<date>-<slug>.md` (REQ-records-001, REQ-records-003): front matter for
//! the fields agents parse, then the write-up a person reads (`# <title>`, the What
//! happened, Steps, Expected, Why it matters and Fix paragraphs) and `## Log`, which is
//! append-only. Nothing here publishes to an external service.

use crate::actor::Actor;
use crate::identity;
use crate::leaks::{self, Local};
use crate::markdown::{self, Kind, Record, Value as Front};
use crate::records::{
    Section, ensure_text, head_revision, local_path, now, print_json, read_bounded_record,
    valid_criterion_id, valid_reference_id,
};
use crate::session_md::{iso_time, parse_time};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::Deserialize;
use serde_json::{Value, json};
use std::cell::OnceCell;
use std::collections::BTreeSet;
use std::path::Path;

pub(super) const BUGS_DIR: &str = ".chough/bugs";
/// Archived session stories, whose `receipts` a bug's evidence links resolve to once
/// the session holding them is archived (REQ-history-006).
const STORIES_DIR: &str = ".chough/archive/sessions";
const STATUSES: [&str; 8] = [
    "open",
    "investigating",
    "fix-pending-verification",
    "verified",
    "deferred",
    "duplicate",
    "wont-fix",
    "fixed-upstream",
];
/// Closed statuses. `fixed-upstream` closes a bug in another tool's behaviour; it is
/// never read as verified by Chough (REQ-cli-012).
const CLOSED: [&str; 4] = ["verified", "duplicate", "wont-fix", "fixed-upstream"];
const SCOPES: [&str; 3] = ["blocking", "in-scope", "out-of-scope"];
const CLASSES: [&str; 6] = [
    "unclassified",
    "product-defect",
    "regression",
    "environment",
    "requirement-gap",
    "evidence-gap",
];
const DUPLICATE_THRESHOLD: f64 = 0.5;

/// The body's paragraphs, each starting `**<label>:**`, in the order a bug is written.
const WHAT_HAPPENED: &str = "What happened";
const STEPS: &str = "Steps";
const EXPECTED: &str = "Expected";
const WHY: &str = "Why it matters";
const FIX: &str = "Fix";
/// The second line of What happened, when the actual behaviour differs from the symptom.
const ACTUAL: &str = "Actual: ";
/// The append-only section: one dated entry per report, note, evidence link and status
/// change.
const LOG: &str = "Log";
/// Front matter lists whose entries both sides of a merge may add to.
const LIST_KEYS: [&str; 7] = [
    "aliases",
    "criteria",
    "requirements",
    "cases",
    "evidence",
    "receipts",
    "related",
];

/// One bug record as read from `.chough/bugs/`.
#[derive(Clone)]
struct Stored {
    path: String,
    record: Record,
}

impl Stored {
    fn id(&self) -> &str {
        self.record.scalar("id").unwrap_or_default()
    }

    /// A scalar; missing or empty is `""`.
    fn text(&self, key: &str) -> &str {
        self.record.scalar(key).unwrap_or_default()
    }

    /// A scalar that is set and not empty.
    fn optional(&self, key: &str) -> Option<String> {
        Some(self.text(key))
            .filter(|value| !value.is_empty())
            .map(str::to_string)
    }

    fn list(&self, key: &str) -> Vec<String> {
        self.record.list(key)
    }

    fn status(&self) -> &str {
        self.text("status")
    }

    fn is_open(&self) -> bool {
        !CLOSED.contains(&self.status())
    }

    /// Whether `id` is this bug's name or one of its aliases (REQ-records-003).
    fn holds(&self, id: &str) -> bool {
        self.id() == id || self.list("aliases").iter().any(|alias| alias == id)
    }

    fn paragraph(&self, label: &str) -> String {
        paragraph(self.record.body(), label).unwrap_or_default()
    }

    /// The symptom: What happened without its `Actual:` line.
    fn symptom(&self) -> String {
        self.paragraph(WHAT_HAPPENED)
            .lines()
            .filter(|line| !line.starts_with(ACTUAL))
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The actual behaviour: What happened's `Actual:` line, or the symptom.
    fn actual(&self) -> String {
        self.paragraph(WHAT_HAPPENED)
            .lines()
            .find_map(|line| line.strip_prefix(ACTUAL))
            .map_or_else(|| self.symptom(), str::to_string)
    }

    /// The trigger: Steps without its numbered lines.
    fn trigger(&self) -> String {
        self.paragraph(STEPS)
            .lines()
            .filter(|line| numbered(line).is_none())
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// The numbered reproduction steps under Steps.
    fn steps(&self) -> Vec<String> {
        self.paragraph(STEPS)
            .lines()
            .filter_map(numbered)
            .map(str::to_string)
            .collect()
    }

    /// The `## Log` entries, oldest first.
    fn log(&self) -> Vec<String> {
        self.record
            .section(LOG)
            .unwrap_or_default()
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(str::to_string)
            .collect()
    }

    /// Every evidence ID this bug links: its evidence, then its regression baseline.
    fn linked(&self) -> Vec<String> {
        let mut ids = self.list("evidence");
        if let Some(baseline) = self.optional("regression_baseline")
            && !ids.contains(&baseline)
        {
            ids.push(baseline);
        }
        ids
    }
}

/// `step` from `<number>. step`.
fn numbered(line: &str) -> Option<&str> {
    let (number, rest) = line.split_once(". ")?;
    (!number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())).then_some(rest)
}

/// The text of the paragraph that starts `**<label>:**`, without the label, up to the
/// next blank line or heading; only the write-up above `## Log` is searched.
fn paragraph(body: &str, label: &str) -> Option<String> {
    let marker = format!("**{label}:**");
    let mut lines = vec![];
    let mut collecting = false;
    for line in body.lines() {
        if line.starts_with("## ") || (collecting && line.starts_with('#')) {
            if collecting {
                break;
            }
            if line.trim_end() == format!("## {LOG}") {
                return None;
            }
            continue;
        }
        if collecting {
            if line.trim().is_empty() {
                break;
            }
            lines.push(line);
        } else if let Some(rest) = line.strip_prefix(&marker) {
            collecting = true;
            lines.push(rest.trim_start());
        }
    }
    collecting.then(|| lines.join("\n"))
}

/// Line breaks inside a value become spaces: a paragraph, a Log entry and a front
/// matter value each keep one value on its lines.
fn one_line(text: &str) -> String {
    text.replace("\r\n", " ").replace(['\n', '\r'], " ")
}

/// A hint for a value that belongs to the other of `--scope` and `--classification`.
fn other_flag(value: &str, other: &[&str], flag: &str) -> String {
    if other.contains(&value) {
        format!("; `{value}` is a {flag}: use --{flag} {value}")
    } else {
        String::new()
    }
}

fn check_status(status: &str) -> Result<()> {
    ensure!(
        STATUSES.contains(&status),
        "status must be one of {}",
        STATUSES.join(", ")
    );
    Ok(())
}

fn check_scope(scope: &str) -> Result<()> {
    ensure!(
        SCOPES.contains(&scope),
        "scope must be one of {}{}",
        SCOPES.join(", "),
        other_flag(scope, &CLASSES, "classification")
    );
    Ok(())
}

fn check_classification(classification: &str) -> Result<()> {
    ensure!(
        CLASSES.contains(&classification),
        "classification must be one of {}{}",
        CLASSES.join(", "),
        other_flag(classification, &SCOPES, "scope")
    );
    Ok(())
}

/// The values a bug command writes, checked before anything is written.
fn validate(stored: &Stored) -> Result<()> {
    ensure!(
        stored.record.scalar("kind") == Some("bug"),
        "{}: a bug record has `kind: bug`",
        stored.path
    );
    ensure!(
        !stored.id().is_empty(),
        "{}: a bug record has an `id`",
        stored.path
    );
    ensure_text("title", stored.text("title"))?;
    check_status(stored.status())?;
    check_scope(stored.text("scope"))?;
    check_classification(stored.text("classification"))?;
    for id in stored.list("criteria") {
        ensure!(valid_criterion_id(&id), "invalid linked ID {id}");
    }
    for id in stored
        .list("requirements")
        .iter()
        .chain(&stored.list("cases"))
    {
        ensure!(valid_reference_id(id), "invalid linked ID {id}");
    }
    ensure!(
        stored.optional("duplicate_of").is_none() || stored.status() == "duplicate",
        "duplicate_of is only set for duplicate bugs"
    );
    Ok(())
}

/// Every bug record, by name. A `.json` bug is the 0.5.x layout, which the one guard in
/// main refuses before any bug command runs (REQ-records-010), so it is not read here; a
/// file that does not read as a bug record is refused naming its file, line and rule
/// (REQ-records-001).
fn read_all(root: &Path) -> Result<Vec<Stored>> {
    let folder = local_path(root, BUGS_DIR)?;
    let entries = match std::fs::read_dir(&folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(error) => return Err(error).with_context(|| format!("reading {BUGS_DIR}")),
    };
    let mut names = vec![];
    for entry in entries {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let Ok(name) = entry.file_name().into_string() else {
            continue;
        };
        if name.ends_with(".md") && !name.starts_with('.') {
            names.push(name);
        }
    }
    names.sort();
    let mut bugs = vec![];
    for name in names {
        let path = format!("{BUGS_DIR}/{name}");
        let text = read_bounded_record(&folder.join(&name), Kind::Bug.bound())?;
        let record = markdown::parse(&path, &text)?;
        let stored = Stored { path, record };
        ensure!(
            stored.record.scalar("kind") == Some("bug"),
            "{}: a bug record has `kind: bug`",
            stored.path
        );
        ensure!(
            name == format!("{}.md", stored.id()),
            "bug file name must match its id: {name}"
        );
        bugs.push(stored);
    }
    Ok(bugs)
}

/// The ID of the bug whose name or alias is `id`, refusing an unknown ID or an alias
/// two bugs hold (REQ-records-003), for a command that links a bug.
pub(super) fn resolve_id(root: &Path, id: &str) -> Result<String> {
    let bugs = read_all(root)?;
    Ok(find(&bugs, id)?.id().to_string())
}

/// The bug whose name or alias is `id`; an alias two bugs hold is refused.
fn find<'a>(bugs: &'a [Stored], id: &str) -> Result<&'a Stored> {
    let found: Vec<&Stored> = bugs.iter().filter(|bug| bug.holds(id)).collect();
    match found.as_slice() {
        [] => bail!("unknown bug: {id}; run `chough bug list --all` to see the bug IDs"),
        [bug] => Ok(bug),
        _ => bail!(
            "refused: {id} is the id or alias of more than one bug ({}); `chough doctor` reports it",
            found
                .iter()
                .map(|bug| bug.path.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn load(root: &Path, id: &str) -> Result<Stored> {
    find(&read_all(root)?, id).cloned()
}

/// A linked evidence entry or receipt as a bug reads it.
#[derive(Clone)]
struct Linked {
    result: Option<String>,
    observed: Option<u64>,
    revision: Option<String>,
    /// Where it was found: `<session>/<evidence>` for an entry or a story's receipt,
    /// `<bug>` for the bug's own receipt.
    place: String,
    label: &'static str,
}

impl Linked {
    fn from_fields(fields: &[(String, String)], place: String, label: &'static str) -> Self {
        let get = |key: &str| {
            fields
                .iter()
                .find(|(name, _)| name == key)
                .map(|(_, value)| value.clone())
                .filter(|value| !value.is_empty())
        };
        Self {
            result: get("result"),
            observed: get("observed").and_then(|text| parse_time(&text).ok()),
            revision: get("revision"),
            place,
            label,
        }
    }

    fn is(&self, result: &str) -> bool {
        self.result.as_deref() == Some(result)
    }
}

/// The IDs a receipt answers to: its `id` and its `aliases`, one scalar separated by
/// commas and spaces (a lenient reader also strips `[` and `]`).
pub(super) fn receipt_names(map: &[(String, String)]) -> Vec<String> {
    let mut names = vec![];
    for (key, value) in map {
        match key.as_str() {
            "id" => names.push(value.clone()),
            "aliases" => names.extend(
                value
                    .split(|char: char| char == ',' || char.is_whitespace())
                    .map(|alias| alias.trim_matches(['[', ']']))
                    .filter(|alias| !alias.is_empty())
                    .map(str::to_string),
            ),
            _ => {}
        }
    }
    names
}

/// The front matter of a record as `(key, scalar)` pairs.
fn scalar_fields(record: &Record) -> Vec<(String, String)> {
    record
        .keys()
        .into_iter()
        .filter_map(|key| Some((key.to_string(), record.scalar(key)?.to_string())))
        .collect()
}

/// Evidence a bug can link: the entries of every open session and the receipts of every
/// archived story, read once per command when a bug first needs them.
struct Evidence<'a> {
    root: &'a Path,
    index: OnceCell<Vec<(Vec<String>, Linked)>>,
}

impl<'a> Evidence<'a> {
    fn new(root: &'a Path) -> Self {
        Self {
            root,
            index: OnceCell::new(),
        }
    }

    /// Files that do not read as records are skipped here; `chough doctor` reports them.
    fn index(&self) -> &[(Vec<String>, Linked)] {
        self.index.get_or_init(|| {
            let mut index = vec![];
            for session in crate::sessions::open_ids(self.root).unwrap_or_default() {
                let dir = format!("{}/{session}/evidence", crate::sessions::DIR);
                for (_, record) in records_in(self.root, &dir, Kind::Evidence.bound()) {
                    let Some(id) = record.scalar("id").filter(|id| !id.is_empty()) else {
                        continue;
                    };
                    let mut names = vec![id.to_string()];
                    names.extend(record.list("aliases"));
                    let place = format!("{session}/{id}");
                    index.push((
                        names,
                        Linked::from_fields(&scalar_fields(&record), place, "entry"),
                    ));
                }
            }
            for (name, record) in records_in(self.root, STORIES_DIR, Kind::Story.bound()) {
                let story = name.trim_end_matches(".md");
                for map in record.maps("receipts") {
                    let names = receipt_names(&map);
                    let Some(id) = names.first() else {
                        continue;
                    };
                    let place = format!("{story}/{id}");
                    index.push((names, Linked::from_fields(&map, place, "story-receipt")));
                }
            }
            index
        })
    }

    /// Resolve one linked ID through open sessions' entries, then archived stories'
    /// receipts, then the bug's own receipts. An ID found in two places is refused as
    /// ambiguous, naming the qualified `<session>/<evidence>` form, which is accepted.
    fn find(&self, bug: &Stored, id: &str) -> Result<Option<Linked>> {
        let hits: Vec<&Linked> = match id.split_once('/') {
            Some((session, evidence)) => self
                .index()
                .iter()
                .filter(|(names, linked)| {
                    linked.place.split_once('/').map(|(place, _)| place) == Some(session)
                        && names.iter().any(|name| name == evidence)
                })
                .map(|(_, linked)| linked)
                .collect(),
            None => self
                .index()
                .iter()
                .filter(|(names, _)| names.iter().any(|name| name == id))
                .map(|(_, linked)| linked)
                .collect(),
        };
        match hits.as_slice() {
            [linked] => return Ok(Some((*linked).clone())),
            [] => {}
            _ => bail!(
                "refused: evidence {id} is ambiguous: it is {}; link one as <session-id>/<evidence-id>",
                hits.iter()
                    .map(|linked| linked.place.as_str())
                    .collect::<Vec<_>>()
                    .join(" and ")
            ),
        }
        if let Some(receipt) = bug
            .record
            .maps("receipts")
            .iter()
            .filter(|map| removed_by(map).is_none())
            .find_map(|map| {
                receipt_names(map)
                    .iter()
                    .any(|name| name == id)
                    .then(|| Linked::from_fields(map, bug.id().to_string(), "bug-receipt"))
            })
        {
            return Ok(Some(receipt));
        }
        Ok(legacy_observation(self.root, id))
    }

    /// The linked evidence that is present: an entry or a receipt. Missing evidence never
    /// counts as reproduced, passed or verifying.
    fn present(&self, bug: &Stored) -> Result<Vec<(String, Linked)>> {
        let mut present = vec![];
        for id in bug.list("evidence") {
            // A link a merge made ambiguous never counts; `labels` shows it as
            // ambiguous and the report still reads (REQ-cli-023). Writes refuse it.
            if let Ok(Some(linked)) = self.find(bug, &id) {
                present.push((id, linked));
            }
        }
        Ok(present)
    }

    /// The latest present linked failure, and whether a present linked pass was observed
    /// strictly after it. A failure or pass without an observed time is present but
    /// unordered: it never verifies.
    fn order(&self, bug: &Stored) -> Result<Option<(String, Option<u64>, bool)>> {
        let present = self.present(bug)?;
        let failures: Vec<&(String, Linked)> = present
            .iter()
            .filter(|(_, linked)| linked.is("failed"))
            .collect();
        if failures.is_empty() {
            return Ok(None);
        }
        if let Some((id, _)) = failures
            .iter()
            .find(|(_, linked)| linked.observed.is_none())
        {
            return Ok(Some((id.clone(), None, false)));
        }
        let Some((id, latest)) = failures
            .iter()
            .filter_map(|(id, linked)| Some((id, linked.observed?)))
            .max_by_key(|(_, observed)| *observed)
        else {
            return Ok(None);
        };
        let passed_after = present.iter().any(|(_, linked)| {
            linked.is("passed") && linked.observed.is_some_and(|observed| observed > latest)
        });
        Ok(Some((id.clone(), Some(latest), passed_after)))
    }

    /// Unverified until a failure is linked; reproduced and later passed only when a
    /// pass was observed strictly after the latest failure (REQ-cli-012).
    fn verification(&self, bug: &Stored) -> Result<&'static str> {
        Ok(match self.order(bug)? {
            None => "unverified",
            Some((_, _, false)) => "reproduced",
            Some((_, _, true)) => "reproduced-and-later-passed",
        })
    }

    /// Each linked ID labeled by where it resolves, or `missing`.
    fn labels(&self, bug: &Stored) -> Result<Vec<Value>> {
        let mut values = vec![];
        let mut seen = BTreeSet::new();
        let roles = bug
            .list("evidence")
            .into_iter()
            .map(|id| (id, "evidence"))
            .chain(
                bug.optional("regression_baseline")
                    .map(|id| (id, "regression-baseline")),
            );
        for (id, role) in roles {
            if !seen.insert(id.clone()) {
                continue;
            }
            let value = match self.find(bug, &id) {
                Ok(Some(linked)) => json!({
                    "id": id, "role": role, "label": linked.label, "place": linked.place,
                    "result": linked.result,
                    "observed": linked.observed.map(iso_time),
                }),
                Ok(None) => match removed_receipt(bug, &id) {
                    Some(receipt) => {
                        json!({"id": id, "role": role, "label": "removed", "receipt": receipt})
                    }
                    None => json!({"id": id, "role": role, "label": "missing"}),
                },
                Err(error) => {
                    json!({"id": id, "role": role, "label": "ambiguous", "reason": format!("{error:#}")})
                }
            };
            values.push(value);
        }
        Ok(values)
    }
}

/// Transitional, until plan step 4's evidence reader replaces [`Evidence::index`] at
/// integration (t382): a 0.5.x observation that `cases run`, `validate run` and
/// `evidence add` still write in this build, read only when no entry or receipt has
/// the ID. It is removed with those writers.
fn legacy_observation(root: &Path, id: &str) -> Option<Linked> {
    if !crate::evidence::is_present(root, id).unwrap_or(false) {
        return None;
    }
    let record = crate::evidence::load(root, id).ok()?;
    let result = match record.outcome {
        crate::evidence::Outcome::Passed => "passed",
        crate::evidence::Outcome::Failed => "failed",
        crate::evidence::Outcome::NotRun => "not-run",
    };
    Some(Linked {
        result: Some(result.to_string()),
        observed: Some(record.observed_at_unix_seconds),
        revision: Some(record.snapshot.revision),
        place: id.to_string(),
        label: "observation",
    })
}

/// The migration receipt a tombstone names: a receipt map `{id, removed: <receipt>}`
/// for an observation the 0.5.x migration removed without carrying (REQ-records-009).
/// It is never present, so it never counts as reproduced, passed or verifying.
fn removed_by(map: &[(String, String)]) -> Option<&str> {
    map.iter()
        .find(|(key, _)| key == "removed")
        .map(|(_, receipt)| receipt.as_str())
}

/// The migration receipt that removed `id`, when one of the bug's tombstones names it.
fn removed_receipt(bug: &Stored, id: &str) -> Option<String> {
    bug.record.maps("receipts").iter().find_map(|map| {
        let receipt = removed_by(map)?;
        receipt_names(map)
            .iter()
            .any(|name| name == id)
            .then(|| receipt.to_string())
    })
}

/// The records in `dir` that read, by file name; others are skipped.
pub(super) fn records_in(root: &Path, dir: &str, bound: u64) -> Vec<(String, Record)> {
    let Ok(folder) = local_path(root, dir) else {
        return vec![];
    };
    let Ok(entries) = std::fs::read_dir(&folder) else {
        return vec![];
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md") && !name.starts_with('.'))
        .collect();
    names.sort();
    names
        .into_iter()
        .filter_map(|name| {
            let text = read_bounded_record(&folder.join(&name), bound).ok()?;
            let record = markdown::parse(&format!("{dir}/{name}"), &text).ok()?;
            Some((name, record))
        })
        .collect()
}

/// Linked evidence for a new link must be present.
/// Check each linked ID resolves, and return each as the bug stores it: a link that
/// resolves to an open session's entry is stored qualified as `<session>/<id>`, so a
/// later merge that brings a same-named entry in another session cannot make it
/// ambiguous (REQ-cli-023); any other link is stored as given. An unknown or
/// ambiguous link refuses with nothing written.
fn check_evidence(evidence: &Evidence<'_>, bug: &Stored, ids: &[String]) -> Result<Vec<String>> {
    let mut stored = vec![];
    for id in ids {
        let linked = evidence.find(bug, id)?.with_context(|| {
            format!(
                "unknown evidence {id}: no open session's entry, archived story's receipt or receipt of this bug has that id or alias"
            )
        })?;
        stored.push(stored_form(id, &linked));
    }
    Ok(stored)
}

/// How a bug stores a link that resolved to `linked`: an open session's entry named by
/// a bare ID as `<session>/<id>`, so a later merge that brings a same-named entry in
/// another session cannot make it ambiguous (REQ-cli-023); anything else as given.
fn stored_form(id: &str, linked: &Linked) -> String {
    let session = (linked.label == "entry" && !id.contains('/'))
        .then(|| linked.place.split_once('/').map(|(session, _)| session))
        .flatten();
    match session {
        Some(session) => format!("{session}/{id}"),
        None => id.to_string(),
    }
}

fn check_regression(evidence: &Evidence<'_>, root: &Path, bug: &Stored) -> Result<()> {
    if bug.text("classification") != "regression" {
        return Ok(());
    }
    let baseline_id = bug.optional("regression_baseline").context(
        "a regression needs --regression-baseline: a passed observation at an earlier, ancestor revision",
    )?;
    let baseline = evidence.find(bug, &baseline_id)?.with_context(|| {
        format!("unknown evidence {baseline_id}: the regression baseline is not present")
    })?;
    ensure!(
        baseline.is("passed"),
        "the regression baseline {baseline_id} did not pass"
    );
    let failing: Vec<Linked> = evidence
        .present(bug)?
        .into_iter()
        .map(|(_, linked)| linked)
        .filter(|linked| linked.is("failed"))
        .collect();
    ensure!(
        !failing.is_empty(),
        "a regression needs a linked failed observation now"
    );
    let earlier = baseline.revision.as_deref().is_some_and(|revision| {
        crate::evidence::is_ancestor_of_head(root, revision)
            && failing
                .iter()
                .all(|linked| linked.revision.as_deref() != Some(revision))
    });
    ensure!(
        earlier,
        "the regression baseline must be at an earlier revision that is an ancestor of HEAD"
    );
    Ok(())
}

fn tokens(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| word.len() >= 3)
        .map(str::to_lowercase)
        .collect()
}

/// The word overlap of `text` with a bug's title, symptom and trigger, plus 0.25 for a
/// shared criterion, requirement or case (REQ-cli-054).
fn similarity(
    bug: &Stored,
    text: &str,
    criteria: &[String],
    requirements: &[String],
    cases: &[String],
) -> f64 {
    let query = tokens(text);
    let known = tokens(&format!(
        "{} {} {}",
        bug.text("title"),
        bug.symptom(),
        bug.trigger()
    ));
    let union = query.union(&known).count();
    let overlap = if union == 0 {
        0.0
    } else {
        query.intersection(&known).count() as f64 / union as f64
    };
    let shared = bug.list("criteria").iter().any(|id| criteria.contains(id))
        || bug
            .list("requirements")
            .iter()
            .any(|id| requirements.contains(id))
        || bug.list("cases").iter().any(|id| cases.contains(id));
    overlap + if shared { 0.25 } else { 0.0 }
}

fn search(
    bugs: &[Stored],
    text: &str,
    criteria: &[String],
    requirements: &[String],
    cases: &[String],
) -> Vec<Value> {
    let mut matches: Vec<(f64, Value)> = bugs
        .iter()
        .map(|bug| {
            let score = similarity(bug, text, criteria, requirements, cases);
            (
                score,
                json!({"id": bug.id(), "title": bug.text("title"), "status": bug.status(), "score": (score * 100.0).round() / 100.0}),
            )
        })
        .filter(|(score, _)| *score >= 0.2)
        .collect();
    matches.sort_by(|left, right| right.0.total_cmp(&left.0));
    matches.into_iter().map(|(_, value)| value).collect()
}

/// What a new bug says, before it has a name.
struct Draft {
    title: String,
    symptom: String,
    trigger: String,
    expected: String,
    actual: String,
    steps: Vec<String>,
    impact: String,
    classification: String,
    scope: String,
    criteria: Vec<String>,
    requirements: Vec<String>,
    cases: Vec<String>,
    evidence: Vec<String>,
    regression_baseline: Option<String>,
    fixture: Option<String>,
    environment: String,
    source: String,
    /// Log entries after the report, such as the reason a similar report is distinct.
    notes: Vec<String>,
}

/// The write-up: the title, the five paragraphs, with `fix` as the Fix paragraph, and an
/// empty Log.
fn write_up(draft: &Draft, fix: &str) -> String {
    let mut what = format!("**{WHAT_HAPPENED}:** {}", one_line(&draft.symptom));
    if draft.actual.trim() != draft.symptom.trim() {
        what.push_str(&format!("\n{ACTUAL}{}", one_line(&draft.actual)));
    }
    let mut steps = format!("**{STEPS}:** {}", one_line(&draft.trigger));
    for (index, step) in draft.steps.iter().enumerate() {
        steps.push_str(&format!("\n{}. {}", index + 1, one_line(step)));
    }
    format!(
        "# {title}\n\n{what}\n\n{steps}\n\n**{EXPECTED}:** {expected}\n\n**{WHY}:** {why}\n\n**{FIX}:** {fix}\n\n## {LOG}\n",
        title = one_line(&draft.title),
        expected = one_line(&draft.expected),
        why = one_line(&draft.impact),
        fix = one_line(fix),
    )
}

/// One Log line: `- <ISO UTC> · <text>`, and ` · by <actor>` when the actor is known.
fn log_line(at: u64, text: &str, actor: Option<&Actor>) -> String {
    let mut line = format!("- {} · {}", iso_time(at), one_line(text));
    if let Some(actor) = actor {
        line.push_str(&format!(" · by {}", actor.display()));
    }
    line
}

/// Check every value a record would write (REQ-records-006), or redact it under
/// `--redact`; returns how many values were redacted.
fn guard(record: &mut Record, local: &Local) -> Result<usize> {
    if leaks::redacting() {
        return Ok(record.redact_leaks(local));
    }
    record.check_leaks(local)?;
    Ok(0)
}

/// The Fix paragraph of a bug no one has fixed yet.
const NOT_FIXED: &str = "Not fixed yet.";

/// The record a draft makes, reported at `at` by `actor` at `revision`, before it has a
/// name: what `bug add`, failure capture and the 0.5.x migration all write
/// (REQ-cli-012). Its `id` is empty until the caller names it.
fn draft_record(
    draft: &Draft,
    at: u64,
    actor: Option<&Actor>,
    revision: Option<String>,
    fix: &str,
) -> Result<Record> {
    for id in &draft.criteria {
        ensure!(valid_criterion_id(id), "invalid linked ID {id}");
    }
    for id in draft.requirements.iter().chain(&draft.cases) {
        ensure!(valid_reference_id(id), "invalid linked ID {id}");
    }
    check_scope(&draft.scope)?;
    check_classification(&draft.classification)?;
    let mut record = Record::new(&write_up(draft, fix));
    let found_by = match actor {
        Some(actor) => format!("{} ({})", actor.display(), one_line(&draft.source)),
        None => one_line(&draft.source),
    };
    let scalars = |record: &mut Record, pairs: &[(&str, &str)]| -> Result<()> {
        for (key, value) in pairs {
            record.set_scalar(key, value)?;
        }
        Ok(())
    };
    let list = |items: &[String]| Front::List(items.iter().map(|item| one_line(item)).collect());
    scalars(&mut record, &[("kind", "bug"), ("id", "")])?;
    record.set("aliases", Front::List(vec![]))?;
    scalars(
        &mut record,
        &[
            ("title", &one_line(&draft.title)),
            ("status", "open"),
            ("classification", &draft.classification),
            ("scope", &draft.scope),
            ("impact", &one_line(&draft.impact)),
        ],
    )?;
    record.set("criteria", list(&draft.criteria))?;
    record.set("requirements", list(&draft.requirements))?;
    record.set("cases", list(&draft.cases))?;
    record.set("evidence", list(&draft.evidence))?;
    record.set("receipts", Front::Maps(vec![]))?;
    if let Some(baseline) = &draft.regression_baseline {
        record.set_scalar("regression_baseline", baseline)?;
    }
    if let Some(fixture) = &draft.fixture {
        record.set_scalar("fixture", fixture)?;
    }
    record.set_scalar("environment", &one_line(&draft.environment))?;
    if let Some(revision) = revision {
        record.set_scalar("revision", &revision)?;
    }
    record.set_scalar("found_by", &found_by)?;
    if let Some(actor) = actor {
        record.set("actor", actor.front_value())?;
    }
    record.set("related", Front::List(vec![]))?;
    record.set_scalar("created", &iso_time(at))?;
    record.set_scalar("updated", &iso_time(at))?;
    let mut report = format!("Reported ({})", one_line(&draft.source));
    if !draft.evidence.is_empty() {
        report.push_str(&format!("; evidence: {}", draft.evidence.join(", ")));
    }
    let mut entries = vec![log_line(at, &report, actor)];
    entries.extend(draft.notes.iter().map(|note| log_line(at, note, actor)));
    record.append_to_section(LOG, &(entries.join("\n") + "\n"))?;
    Ok(record)
}

/// Write a new bug named `<date>-<title slug>` by create-new (REQ-records-003); `redact`
/// redacts leaks instead of refusing, as automatic failure capture does. Returns the
/// stored record and how many values were redacted.
fn create(root: &Path, draft: Draft, redact: bool) -> Result<(Stored, usize)> {
    let at = now()?;
    let actor = Actor::current();
    let mut record = draft_record(&draft, at, actor.as_ref(), head_revision(root), NOT_FIXED)?;

    // The name comes from the checked title, so a redacted title never names the file.
    let local = Local::current();
    let redacted = if redact {
        record.redact_leaks(local)
    } else {
        guard(&mut record, local)?
    };
    let title = record.scalar("title").unwrap_or_default().to_string();
    let name = identity::dated_name(&identity::utc_date(at), &title, "bug");
    record.set_scalar("id", &name)?;
    validate(&Stored {
        path: format!("{BUGS_DIR}/{name}.md"),
        record: record.clone(),
    })?;
    let written =
        identity::create_named(root, BUGS_DIR, &name, ".md", Kind::Bug.bound(), |name| {
            let mut record = record.clone();
            record.set_scalar("id", name)?;
            record.check_leaks(local)?;
            record.render_within(Kind::Bug)
        })?;
    record.set_scalar("id", &written)?;
    Ok((
        Stored {
            path: format!("{BUGS_DIR}/{written}.md"),
            record,
        },
        redacted,
    ))
}

#[derive(clap::Args)]
pub(super) struct AddArgs {
    /// Short title; also names the bug `<date>-<title slug>`. Values may start with `--`.
    #[arg(long, allow_hyphen_values = true)]
    title: String,
    /// What was seen to go wrong.
    #[arg(long, allow_hyphen_values = true)]
    symptom: String,
    /// The input or action that triggers the failure.
    #[arg(long, allow_hyphen_values = true)]
    trigger: String,
    /// The behavior that should have happened.
    #[arg(long, allow_hyphen_values = true)]
    expected: String,
    /// The behavior that actually happened.
    #[arg(long, allow_hyphen_values = true)]
    actual: String,
    /// The environment where the failure was seen, for example local-linux-x86_64.
    #[arg(long, allow_hyphen_values = true)]
    environment: String,
    /// Who or what is affected and how badly.
    #[arg(long, default_value = "Not yet assessed.", allow_hyphen_values = true)]
    impact: String,
    /// A HI criterion ID the failure breaks (repeatable).
    #[arg(long = "criterion")]
    criteria: Vec<String>,
    /// A SpecSync requirement ID the failure breaks (repeatable).
    #[arg(long = "requirement")]
    requirements: Vec<String>,
    /// A test case ID that covers the failure (repeatable).
    #[arg(long = "case")]
    cases: Vec<String>,
    /// One reproduction step, in order (repeatable).
    #[arg(long = "step", allow_hyphen_values = true)]
    steps: Vec<String>,
    /// A fixture path or description that reproduces the failure.
    #[arg(long, allow_hyphen_values = true)]
    fixture: Option<String>,
    /// An evidence entry that shows the failure, by ID or alias (repeatable).
    #[arg(long = "observation", alias = "evidence")]
    observations: Vec<String>,
    /// One of unclassified, product-defect, regression, environment, requirement-gap or
    /// evidence-gap. Scope values go in --scope.
    #[arg(long, default_value = "unclassified")]
    classification: String,
    /// One of blocking, in-scope or out-of-scope. Classification values go in
    /// --classification.
    #[arg(long, default_value = "in-scope")]
    scope: String,
    /// A passed observation at an earlier, ancestor revision (required for regression).
    #[arg(long)]
    regression_baseline: Option<String>,
    /// Where the report came from.
    #[arg(long, default_value = "manual report", allow_hyphen_values = true)]
    source: String,
    /// Add even though similar bugs exist, stating why this one is distinct.
    #[arg(long, allow_hyphen_values = true)]
    distinct: Option<String>,
}

#[derive(clap::Args)]
pub(super) struct UpdateArgs {
    /// The bug's name or any of its aliases, such as a 0.5.x `BUG-0012`.
    id: String,
    /// One of open, investigating, fix-pending-verification, verified, deferred,
    /// duplicate, wont-fix or fixed-upstream (which needs --note naming the upstream fix
    /// and who checked it where).
    #[arg(long)]
    status: Option<String>,
    /// One of unclassified, product-defect, regression, environment, requirement-gap or
    /// evidence-gap. Scope values go in --scope.
    #[arg(long)]
    classification: Option<String>,
    /// One of blocking, in-scope or out-of-scope. Classification values go in
    /// --classification.
    #[arg(long)]
    scope: Option<String>,
    /// A note appended to the bug's Log.
    #[arg(long, allow_hyphen_values = true)]
    note: Option<String>,
    /// An evidence entry to link, by ID or alias (repeatable).
    #[arg(long = "observation", alias = "evidence")]
    observations: Vec<String>,
    /// A passed observation at an earlier, ancestor revision.
    #[arg(long)]
    regression_baseline: Option<String>,
    /// The original bug this one duplicates; sets the status to duplicate.
    #[arg(long)]
    duplicate_of: Option<String>,
    /// A related bug (repeatable).
    #[arg(long = "related")]
    related: Vec<String>,
}

#[derive(Subcommand)]
pub(super) enum BugCommand {
    /// Record a new local bug after searching for duplicates.
    Add(Box<AddArgs>),
    /// Append a Log entry and change status, scope, classification or links.
    Update(UpdateArgs),
    /// List the open backlog, or every bug with --all.
    List {
        #[arg(long)]
        all: bool,
    },
    /// Find similar bugs by symptom, trigger and shared IDs.
    Search {
        text: String,
        #[arg(long = "criterion")]
        criteria: Vec<String>,
        #[arg(long = "requirement")]
        requirements: Vec<String>,
        #[arg(long = "case")]
        cases: Vec<String>,
    },
    /// Show one bug with its derived verification state.
    Show { id: String },
    /// Print a Markdown report candidate. It is never published by Chough.
    Report { id: String },
    /// Git merge driver for `.chough/bugs/*.md` (`chough bug merge %O %A %B`): merge two
    /// edits of one bug into `ours`, or exit nonzero leaving `ours` for a human.
    Merge {
        base: std::path::PathBuf,
        ours: std::path::PathBuf,
        theirs: std::path::PathBuf,
    },
}

/// Values needed to capture a failure found by an automated run.
pub(super) struct Finding<'a> {
    pub(super) case: &'a str,
    pub(super) criteria: Vec<String>,
    pub(super) requirements: Vec<String>,
    pub(super) environment: &'a str,
    pub(super) observation: &'a str,
    pub(super) summary: &'a str,
    pub(super) source: String,
}

/// Append a failed run to the open bug for its case, or open a new local bug named by
/// its title like a manual one. Failing output is redacted, never refused
/// (REQ-records-005), so a leak in it never stops the capture.
pub(super) fn capture_failure(root: &Path, finding: Finding<'_>) -> Result<String> {
    crate::sessions::lock_project(root)?;
    let bugs = read_all(root)?;
    // The failure is linked as bug add links it (REQ-cli-023); one that does not
    // resolve here is linked as given, so a capture never fails on it.
    let index = Evidence::new(root);
    let link_for = |bug: &Stored| match index.find(bug, finding.observation) {
        Ok(Some(linked)) => stored_form(finding.observation, &linked),
        _ => finding.observation.to_string(),
    };
    if let Some(existing) = bugs
        .iter()
        .find(|bug| bug.is_open() && bug.list("cases").iter().any(|case| case == finding.case))
    {
        let mut stored = existing.clone();
        let at = now()?;
        let link = link_for(&stored);
        let mut evidence = stored.list("evidence");
        if !evidence
            .iter()
            .any(|id| *id == link || id == finding.observation)
        {
            evidence.push(link.clone());
        }
        stored.record.set("evidence", Front::List(evidence))?;
        stored.record.set_scalar("updated", &iso_time(at))?;
        let text = format!(
            "Additional failure from {}; evidence: {link}",
            finding.source
        );
        stored.record.append_to_section(
            LOG,
            &(log_line(at, &text, Actor::current().as_ref()) + "\n"),
        )?;
        stored.record.redact_leaks(Local::current());
        markdown::replace(root, &stored.path, &stored.record, Kind::Bug)?;
        return Ok(stored.id().to_string());
    }
    let draft = Draft {
        title: format!("{} failed during {}", finding.case, finding.source),
        symptom: finding.summary.into(),
        trigger: format!("Running case {}", finding.case),
        expected: format!("Case {} passes its declared assertion", finding.case),
        actual: finding.summary.into(),
        steps: vec![format!(
            "chough cases run {} --environment {}",
            finding.case, finding.environment
        )],
        impact: "Not yet assessed.".into(),
        classification: "unclassified".into(),
        scope: "in-scope".into(),
        criteria: finding.criteria,
        requirements: finding.requirements,
        cases: vec![finding.case.into()],
        evidence: vec![link_for(&Stored {
            path: String::new(),
            record: Record::new(""),
        })],
        regression_baseline: None,
        fixture: None,
        environment: finding.environment.into(),
        source: finding.source,
        notes: vec![],
    };
    let (stored, _) = create(root, draft, true)?;
    Ok(stored.id().to_string())
}

fn add(root: &Path, args: AddArgs) -> Result<Value> {
    crate::sessions::lock_project(root)?;
    let bugs = read_all(root)?;
    let query = format!("{} {} {}", args.title, args.symptom, args.trigger);
    let candidates = search(
        &bugs,
        &query,
        &args.criteria,
        &args.requirements,
        &args.cases,
    );
    let likely: Vec<&Value> = candidates
        .iter()
        .filter(|candidate| candidate["score"].as_f64().unwrap_or(0.0) >= DUPLICATE_THRESHOLD)
        .collect();
    if !likely.is_empty() && args.distinct.is_none() {
        bail!(
            "similar bugs exist: {}. Append evidence with `chough bug update <id> --observation <id> --note <text>`, or add with --distinct <why this is a different defect>",
            serde_json::to_string(&likely)?
        );
    }
    for (label, value) in [
        ("title", &args.title),
        ("symptom", &args.symptom),
        ("trigger", &args.trigger),
        ("expected", &args.expected),
        ("actual", &args.actual),
        ("environment", &args.environment),
        ("impact", &args.impact),
        ("source", &args.source),
    ] {
        ensure_text(label, value)?;
    }
    check_scope(&args.scope)?;
    check_classification(&args.classification)?;
    let mut notes = vec![];
    if let Some(reason) = &args.distinct {
        ensure_text("distinct reason", reason)?;
        notes.push(format!("Added despite similar reports: {reason}"));
    }
    if let Some(fixture) = &args.fixture {
        local_path(root, fixture)?;
    }
    // Links and the regression rule are checked against the bug before it is written.
    let evidence = Evidence::new(root);
    let mut probe = Record::new("");
    probe.set("evidence", Front::List(args.observations.clone()))?;
    if let Some(baseline) = &args.regression_baseline {
        probe.set_scalar("regression_baseline", baseline)?;
    }
    probe.set_scalar("classification", &args.classification)?;
    let probe = Stored {
        path: String::new(),
        record: probe,
    };
    let links = check_evidence(&evidence, &probe, &args.observations)?;
    check_regression(&evidence, root, &probe)?;
    let draft = Draft {
        title: args.title,
        symptom: args.symptom,
        trigger: args.trigger,
        expected: args.expected,
        actual: args.actual,
        steps: args.steps,
        impact: args.impact,
        classification: args.classification,
        scope: args.scope,
        criteria: args.criteria,
        requirements: args.requirements,
        cases: args.cases,
        evidence: links,
        regression_baseline: args.regression_baseline,
        fixture: args.fixture,
        environment: args.environment,
        source: args.source,
        notes,
    };
    let (stored, redacted) = create(root, draft, false)?;
    let mut output = json!({
        "schema_version": 1,
        "recorded": stored.path,
        "id": stored.id(),
        "verification": evidence.verification(&stored)?,
        "similar": candidates,
        "published": false,
    });
    if leaks::redacting() {
        output["redacted"] = json!(redacted);
    }
    Ok(output)
}

fn update(root: &Path, args: UpdateArgs) -> Result<Value> {
    crate::sessions::lock_project(root)?;
    let bugs = read_all(root)?;
    let mut stored = find(&bugs, &args.id)?.clone();
    let id = stored.id().to_string();
    let at = now()?;
    let mut changes = vec![];
    if let Some(status) = &args.status {
        check_status(status)?;
        ensure!(
            status != "fixed-upstream"
                || args
                    .note
                    .as_deref()
                    .is_some_and(|note| !note.trim().is_empty()),
            "fixed-upstream needs --note naming the upstream fix and who checked it where; Chough never reads it as verified"
        );
    }
    if let Some(scope) = &args.scope {
        check_scope(scope)?;
    }
    if let Some(classification) = &args.classification {
        check_classification(classification)?;
    }
    if let Some(note) = &args.note {
        ensure_text("note", note)?;
    }
    let evidence = Evidence::new(root);
    let links = check_evidence(&evidence, &stored, &args.observations)?;
    for (field, value) in [
        ("status", &args.status),
        ("classification", &args.classification),
        ("scope", &args.scope),
    ] {
        if let Some(value) = value {
            let current = stored.text(field).to_string();
            if current != *value {
                changes.push(format!("{field}: {current} -> {value}"));
                stored.record.set_scalar(field, value)?;
            }
        }
    }
    if let Some(baseline) = &args.regression_baseline {
        stored.record.set_scalar("regression_baseline", baseline)?;
        changes.push(format!("regression baseline: {baseline}"));
    }
    if let Some(original) = &args.duplicate_of {
        let target = find(&bugs, original)?;
        ensure!(
            target.id() != id,
            "a bug cannot duplicate itself; name the original with --duplicate-of <id> (run `chough bug list --all` to see the bug IDs)"
        );
        stored.record.set_scalar("duplicate_of", target.id())?;
        changes.push(format!("duplicate of {}", target.id()));
        if stored.status() != "duplicate" {
            changes.push(format!("status: {} -> duplicate", stored.status()));
            stored.record.set_scalar("status", "duplicate")?;
        }
    }
    let mut related = stored.list("related");
    for link in &args.related {
        let target = find(&bugs, link)?;
        ensure!(
            target.id() != id,
            "a bug cannot relate to itself; name another bug with --related <id> (run `chough bug list --all` to see the bug IDs)"
        );
        // A link already held, by name or by alias, is not added twice.
        let held = related
            .iter()
            .any(|existing| existing == target.id() || target.holds(existing));
        if !held {
            related.push(target.id().to_string());
            changes.push(format!("related: {}", target.id()));
        }
    }
    stored.record.set("related", Front::List(related))?;
    // Reopening a bug marked duplicate by mistake clears its link to the original.
    if stored.status() != "duplicate"
        && let Some(previous) = stored.optional("duplicate_of")
    {
        stored.record.remove("duplicate_of");
        changes.push(format!("no longer a duplicate of {previous}"));
    }
    let mut linked = stored.list("evidence");
    let mut added = vec![];
    for (given, observation) in args.observations.iter().zip(&links) {
        if !linked.contains(observation) && !linked.contains(given) {
            linked.push(observation.clone());
            added.push(observation.clone());
        }
    }
    stored.record.set("evidence", Front::List(linked))?;
    if !added.is_empty() {
        changes.push(format!("evidence: {}", added.join(", ")));
    }
    if stored.status() == "verified" {
        match evidence.order(&stored)? {
            None => bail!(
                "verified needs a linked failed observation that reproduces the bug, then a linked passed observation after it; no failed observation is linked: link one with `chough bug update {id} --observation <id>`"
            ),
            Some((failure, None, _)) => bail!(
                "verified needs a linked passed observation after the latest linked failure {failure}, which has no observed time, so no pass can be ordered after it: link a failure with an observed time with `chough bug update {id} --observation <id>`"
            ),
            Some((failure, Some(observed), false)) => bail!(
                "verified needs a linked passed observation after the fix, observed after the latest linked failure {failure} (observed {}): link a later pass with `chough bug update {id} --observation <id>`",
                iso_time(observed)
            ),
            Some((_, _, true)) => {}
        }
    }
    check_regression(&evidence, root, &stored)?;
    let mut text = changes.join("; ");
    if let Some(note) = &args.note {
        if !text.is_empty() {
            text.push_str(". ");
        }
        text.push_str(note);
    }
    if text.is_empty() {
        text = "Updated with no change.".into();
    }
    stored.record.set_scalar("updated", &iso_time(at))?;
    stored.record.append_to_section(
        LOG,
        &(log_line(at, &text, Actor::current().as_ref()) + "\n"),
    )?;
    validate(&stored)?;
    let redacted = guard(&mut stored.record, Local::current())?;
    markdown::replace(root, &stored.path, &stored.record, Kind::Bug)?;
    let mut output = json!({
        "schema_version": 1,
        "recorded": stored.path,
        "bug": bug_json(&stored),
        "verification": evidence.verification(&stored)?,
    });
    if leaks::redacting() {
        output["redacted"] = json!(redacted);
    }
    Ok(output)
}

/// A front matter value as JSON.
fn front_json(value: &Front) -> Value {
    match value {
        Front::Scalar(text) => json!(text),
        Front::List(items) => json!(items),
        Front::Maps(maps) => Value::Array(
            maps.iter()
                .map(|map| {
                    Value::Object(
                        map.iter()
                            .map(|(key, value)| (key.clone(), json!(value)))
                            .collect(),
                    )
                })
                .collect(),
        ),
    }
}

/// The bug as JSON: every front matter key, its paragraphs and its Log.
fn bug_json(stored: &Stored) -> Value {
    let mut object = serde_json::Map::new();
    for key in stored.record.keys() {
        if let Some(value) = stored.record.get(key) {
            object.insert(key.to_string(), front_json(value));
        }
    }
    object.insert("symptom".into(), json!(stored.symptom()));
    object.insert("actual".into(), json!(stored.actual()));
    object.insert("trigger".into(), json!(stored.trigger()));
    object.insert("steps".into(), json!(stored.steps()));
    object.insert("expected".into(), json!(stored.paragraph(EXPECTED)));
    object.insert("why_it_matters".into(), json!(stored.paragraph(WHY)));
    object.insert("fix".into(), json!(stored.paragraph(FIX)));
    object.insert("log".into(), json!(stored.log()));
    Value::Object(object)
}

fn report(evidence: &Evidence<'_>, bug: &Stored) -> Result<String> {
    let list = |items: &[String]| {
        if items.is_empty() {
            "none recorded".to_string()
        } else {
            items.join(", ")
        }
    };
    let or_none = |text: &str| {
        if text.is_empty() {
            "none recorded".to_string()
        } else {
            text.to_string()
        }
    };
    let steps = bug.steps();
    let steps = if steps.is_empty() {
        "Not recorded.".to_string()
    } else {
        steps
            .iter()
            .enumerate()
            .map(|(index, step)| format!("{}. {step}", index + 1))
            .collect::<Vec<_>>()
            .join("\n")
    };
    Ok(format!(
        "<!-- Local report candidate from {path}. Not published; posting needs an explicit human request. -->\n\
# {title}\n\n\
Classification and confidence: {class}; verification: {verification}\n\
Impact: {impact}\n\
Traceability: HI {criteria}; requirements {requirements}; cases {cases}\n\
Scope and status: {scope}, {status}\n\
Also known as: {aliases}\n\
Found by: {found_by}\n\n\
## Reproduction\n\
Revision: {revision}\n\
Environment: {environment}\n\
Fixture: {fixture}\n\
What happened: {symptom}\n\
Trigger: {trigger}\n\
Steps:\n{steps}\n\
Expected: {expected}\n\
Observed: {actual}\n\
Why it matters: {why}\n\
Fix: {fix}; fixed in: {fixed_in}\n\n\
## Evidence and deduplication\n\
Evidence: {evidence}\n\
Related: {related}; duplicate of: {duplicate}\n\
Limits: a reproduction without a linked failed observation is unverified.\n",
        path = bug.path,
        title = bug.text("title"),
        class = bug.text("classification"),
        verification = evidence.verification(bug)?,
        impact = bug.text("impact"),
        criteria = list(&bug.list("criteria")),
        requirements = list(&bug.list("requirements")),
        cases = list(&bug.list("cases")),
        scope = bug.text("scope"),
        status = bug.status(),
        aliases = list(&bug.list("aliases")),
        found_by = or_none(bug.text("found_by")),
        revision = bug.optional("revision").unwrap_or_else(|| "unknown".into()),
        environment = bug.text("environment"),
        fixture = or_none(bug.text("fixture")),
        symptom = bug.symptom(),
        trigger = bug.trigger(),
        expected = bug.paragraph(EXPECTED),
        actual = bug.actual(),
        why = bug.paragraph(WHY),
        fix = bug.paragraph(FIX),
        fixed_in = or_none(bug.text("fixed_in")),
        evidence = list(&bug.list("evidence")),
        related = list(&bug.list("related")),
        duplicate = bug
            .optional("duplicate_of")
            .unwrap_or_else(|| "none".into()),
    ))
}

fn brief(evidence: &Evidence<'_>, bug: &Stored) -> Result<Value> {
    Ok(json!({
        "id": bug.id(),
        "title": bug.text("title"),
        "status": bug.status(),
        "scope": bug.text("scope"),
        "classification": bug.text("classification"),
        "verification": evidence.verification(bug)?,
    }))
}

/// Open bugs for status, next and context.
pub(super) fn open_backlog(root: &Path) -> Result<Vec<Value>> {
    let evidence = Evidence::new(root);
    read_all(root)?
        .iter()
        .filter(|bug| bug.is_open())
        .map(|bug| brief(&evidence, bug))
        .collect()
}

/// Every evidence ID each bug links, whatever its status, labeled `<bug-id> <status>`:
/// its evidence and its regression baseline, a session-qualified link also by its bare
/// ID (see [`with_bare_ids`]).
pub(super) fn observation_references(root: &Path) -> Result<Vec<(String, Vec<String>)>> {
    Ok(read_all(root)?
        .iter()
        .map(|bug| {
            (
                format!("{} {}", bug.id(), bug.status()),
                with_bare_ids(bug.linked()),
            )
        })
        .collect())
}

/// Each linked ID as stored and, for a session-qualified link (`<session>/<id>`, as
/// `bug add` and `bug update` store a link to a session entry, REQ-cli-023), its bare
/// evidence ID too, the part after the last `/`. The test gate and next's last failure
/// match bug links against bare observation IDs, so both forms count as a link.
fn with_bare_ids(links: Vec<String>) -> Vec<String> {
    let mut ids = vec![];
    for link in links {
        if let Some((_, bare)) = link.rsplit_once('/')
            && !bare.is_empty()
            && !ids.iter().any(|id| id == bare)
        {
            ids.push(bare.to_string());
        }
        if !ids.contains(&link) {
            ids.push(link);
        }
    }
    ids
}

/// Whether a bug status is closed: verified, duplicate, wont-fix or fixed-upstream.
pub(super) fn is_closed_status(status: &str) -> bool {
    CLOSED.contains(&status)
}

pub(super) fn open_blocking(root: &Path) -> Result<Vec<String>> {
    Ok(read_all(root)?
        .iter()
        .filter(|bug| bug.is_open() && bug.text("scope") == "blocking")
        .map(|bug| bug.id().to_string())
        .collect())
}

pub(super) fn open_backlog_section(root: &Path) -> Result<Section> {
    let open = open_backlog(root)?;
    Ok(Section {
        source: format!("{BUGS_DIR} (open backlog)"),
        state: if open.is_empty() {
            "empty".into()
        } else {
            "present".into()
        },
        content: serde_json::to_string_pretty(&open)?,
    })
}

/// One bug's times for the timing report (REQ-reporting-001): its creation, its
/// earliest present linked failure and the first Log entry that records a move to
/// verified. No retention ledger is read.
pub(super) struct FixTimes {
    pub(super) id: String,
    /// Its name and its aliases, which `timing --bug` accepts (REQ-cli-023).
    pub(super) names: Vec<String>,
    pub(super) status: String,
    /// `None` when `created` does not read.
    pub(super) created: Option<u64>,
    /// The earliest linked failure with an observed time, and where it was read.
    pub(super) reproduced: Option<(u64, String)>,
    /// A linked ID that is missing or ambiguous, or a linked failure without an observed
    /// time: either could be the earlier failure, so the reproduction time is unknown.
    pub(super) unavailable: bool,
    /// The first Log entry that records `status: <old> -> verified`.
    pub(super) verified: Option<u64>,
}

/// One Log entry's time and text, from `- <ISO UTC> · <text>`, without the ` · by
/// <actor>` that ends an entry whose actor is known.
fn log_entry(line: &str) -> Option<(u64, String)> {
    let (time, text) = line.strip_prefix("- ")?.split_once(" · ")?;
    let text = text.rsplit_once(" · by ").map_or(text, |(text, _)| text);
    Some((parse_time(time).ok()?, text.to_string()))
}

/// Whether a Log entry's changes (the `; `-joined `field: old -> new` entries before
/// the first `. `, as `bug update` writes them) record a move to verified.
fn records_verified(text: &str) -> bool {
    let prefix = text.split(". ").next().unwrap_or_default();
    prefix.split("; ").any(|change| {
        change
            .strip_prefix("status: ")
            .and_then(|rest| rest.split_once(" -> "))
            .is_some_and(|(old, new)| {
                !old.is_empty() && !old.contains(' ') && new.trim_end_matches('.') == "verified"
            })
    })
}

/// Every bug's fix times.
pub(super) fn fix_times(root: &Path) -> Result<Vec<FixTimes>> {
    let evidence = Evidence::new(root);
    Ok(read_all(root)?
        .iter()
        .map(|bug| {
            let mut unavailable = false;
            let mut earliest: Option<(u64, String)> = None;
            for id in bug.list("evidence") {
                match evidence.find(bug, &id) {
                    Ok(Some(linked)) if linked.is("failed") => match linked.observed {
                        Some(at) if earliest.as_ref().is_none_or(|(best, _)| at < *best) => {
                            earliest = Some((at, format!("evidence {id} ({})", linked.label)));
                        }
                        Some(_) => {}
                        None => unavailable = true,
                    },
                    Ok(Some(_)) => {}
                    Ok(None) | Err(_) => unavailable = true,
                }
            }
            let mut names = vec![bug.id().to_string()];
            names.extend(bug.list("aliases"));
            FixTimes {
                id: bug.id().to_string(),
                names,
                status: bug.status().to_string(),
                created: parse_time(bug.text("created")).ok(),
                reproduced: earliest,
                unavailable,
                verified: bug
                    .log()
                    .iter()
                    .filter_map(|line| log_entry(line))
                    .find(|(_, text)| records_verified(text))
                    .map(|(at, _)| at),
            }
        })
        .collect())
}

/// One receipt: the flat map of its front matter fields (REQ-history-006).
pub(super) type Receipt = Vec<(String, String)>;

/// Each bug's receipts (REQ-cli-012), with the bug's ID, for the timing report's case
/// runs.
pub(super) fn receipts(root: &Path) -> Result<Vec<(String, Receipt)>> {
    Ok(read_all(root)?
        .iter()
        .flat_map(|bug| {
            bug.record
                .maps("receipts")
                .into_iter()
                .filter(|map| removed_by(map).is_none())
                .map(|map| (bug.id().to_string(), map))
        })
        .collect())
}

/// A 0.5.x bug record, `.chough/bugs/<BUG-ID>.json` (schema 1), as every release from
/// 0.1.0 to 0.5.0 wrote it. Unknown keys are ignored.
#[derive(Deserialize)]
struct Legacy {
    id: String,
    title: String,
    symptom: String,
    trigger: String,
    expected: String,
    actual: String,
    #[serde(default)]
    criteria: Vec<String>,
    #[serde(default)]
    requirements: Vec<String>,
    #[serde(default)]
    cases: Vec<String>,
    environment: String,
    #[serde(default)]
    revision: Option<String>,
    #[serde(default)]
    steps: Vec<String>,
    #[serde(default)]
    fixture: Option<String>,
    #[serde(default)]
    observations: Vec<String>,
    impact: String,
    classification: String,
    scope: String,
    status: String,
    #[serde(default)]
    regression_baseline: Option<String>,
    #[serde(default)]
    duplicate_of: Option<String>,
    #[serde(default)]
    related: Vec<String>,
    source: String,
    #[serde(default)]
    notes: Vec<LegacyNote>,
    created_at_unix_seconds: u64,
    updated_at_unix_seconds: u64,
}

#[derive(Deserialize)]
struct LegacyNote {
    at_unix_seconds: u64,
    text: String,
    #[serde(default)]
    observations: Vec<String>,
}

impl Legacy {
    /// Every observation it links, in link order: its observations, its notes'
    /// observations and its regression baseline.
    fn linked(&self) -> Vec<String> {
        let mut ids: Vec<String> = vec![];
        for id in self
            .observations
            .iter()
            .chain(self.notes.iter().flat_map(|note| &note.observations))
            .chain(&self.regression_baseline)
        {
            if !ids.contains(id) {
                ids.push(id.clone());
            }
        }
        ids
    }

    fn is_open(&self) -> bool {
        !CLOSED.contains(&self.status.as_str())
    }
}

/// The 0.5.x bound of one record file.
const LEGACY_BOUND: u64 = 1_048_576;

/// Every 0.5.x bug, `BUG-NNNN` and `BUG-<slug>-<hex>` alike, with its project-relative
/// path, sorted by path. A file that does not read as a 0.5.x bug refuses the plan,
/// naming it.
fn legacy_bugs(root: &Path) -> Result<Vec<(String, Legacy)>> {
    let folder = local_path(root, BUGS_DIR)?;
    let entries = match std::fs::read_dir(&folder) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(error) => return Err(error).with_context(|| format!("reading {BUGS_DIR}")),
    };
    let mut names: Vec<String> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".json"))
        .collect();
    names.sort();
    let mut bugs = vec![];
    for name in names {
        let path = format!("{BUGS_DIR}/{name}");
        let text = read_bounded_record(&folder.join(&name), LEGACY_BOUND)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        let legacy: Legacy = serde_json::from_str(&text)
            .with_context(|| format!("{path} is not a 0.5.x bug record, so no plan was made"))?;
        bugs.push((path, legacy));
    }
    Ok(bugs)
}

/// Migration hook 2 (REQ-records-009): each 0.5.x bug's ID with the observation IDs it
/// links, for the evidence mapper to place before the bug mapper runs.
pub(super) fn links_0_5(root: &Path) -> Result<std::collections::BTreeMap<String, Vec<String>>> {
    Ok(legacy_bugs(root)?
        .into_iter()
        .map(|(_, legacy)| (legacy.id.clone(), legacy.linked()))
        .collect())
}

/// Each 0.5.x bug's ID with the path the bug mapper writes it to (REQ-records-009),
/// for the evidence mapper, which runs first, to alias an observation only a bug holds
/// as a receipt. The names do not depend on where observations go.
pub(super) fn paths_0_5(root: &Path) -> Result<std::collections::BTreeMap<String, String>> {
    let ids: BTreeSet<String> = legacy_bugs(root)?
        .into_iter()
        .map(|(_, legacy)| legacy.id)
        .collect();
    let carried = migrate_0_5(root, &crate::migrate::ObservationHomes::default())?;
    Ok(carried
        .aliases
        .into_iter()
        .filter(|(id, _)| ids.contains(id))
        .collect())
}

/// The files under `fixture` when it names a file or folder in the project; nothing
/// for a description that is not a path.
fn fixture_files(root: &Path, fixture: &str) -> Vec<String> {
    fn walk(root: &Path, relative: &str, files: &mut Vec<String>) {
        let Ok(path) = local_path(root, relative) else {
            return;
        };
        if path.is_file() {
            files.push(relative.to_string());
        } else if let Ok(entries) = std::fs::read_dir(&path) {
            let mut names: Vec<String> = entries
                .filter_map(|entry| entry.ok())
                .filter_map(|entry| entry.file_name().into_string().ok())
                .collect();
            names.sort();
            for name in names {
                walk(
                    root,
                    &format!("{}/{name}", relative.trim_end_matches('/')),
                    files,
                );
            }
        }
    }
    let mut files = vec![];
    walk(root, fixture.trim_end_matches('/'), &mut files);
    files
}

// The evidence mapper (t37d) calls it to keep these files; until then only tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// The fixture files of every open 0.5.x bug, folders expanded: kept byte for byte by
/// the migration, so the evidence mapper never lists them as removed (REQ-records-009).
pub(super) fn open_fixtures_0_5(root: &Path) -> Result<BTreeSet<String>> {
    Ok(open_fixture_owners_0_5(root)?.into_keys().collect())
}

/// Each fixture file of an open 0.5.x bug, folders expanded, with why the migration
/// keeps it byte for byte, naming the bug (REQ-records-009).
pub(super) fn open_fixture_owners_0_5(
    root: &Path,
) -> Result<std::collections::BTreeMap<String, String>> {
    let mut owners = std::collections::BTreeMap::new();
    for (_, legacy) in legacy_bugs(root)? {
        if !legacy.is_open() {
            continue;
        }
        let Some(fixture) = legacy.fixture.as_deref() else {
            continue;
        };
        for file in fixture_files(root, fixture) {
            owners.entry(file).or_insert_with(|| {
                format!("the fixture of open bug {}, kept byte for byte", legacy.id)
            });
        }
    }
    Ok(owners)
}

/// The migration receipt this plan writes. Plan step 10's `migrate::receipt_path` gives
/// it at integration; it is the same path.
fn migration_receipt() -> Result<String> {
    crate::migrate::receipt_path()
}

/// A deterministic clash suffix for a migrated bug: the first 6 hexadecimal digits of
/// the SHA-256 of its 0.5.x ID, so a preview and its apply name every bug alike.
fn migration_suffix(old: &str) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(old.as_bytes()))[..6].to_string()
}

/// Migration hook 4 (REQ-records-009): each 0.5.x bug as the Markdown bug `bug add`
/// writes, named by its creation date and title, its 0.5.x ID an alias. Every field,
/// note and status change carries over with its original time. Each linked observation
/// is a receipt when the evidence mapper made it one, an evidence link when it is an
/// open session's entry, and otherwise a tombstone naming the migration receipt, which
/// never counts. An open bug's fixture files are kept byte for byte, each that holds a
/// leak listed; a closed bug's fixture files under `.chough/artifacts/` are removed.
/// Nothing is written and nothing redacted here: the framework redacts every written
/// file.
pub(super) fn migrate_0_5(
    root: &Path,
    homes: &crate::migrate::ObservationHomes,
) -> Result<crate::migrate::Carried> {
    let mut carried = crate::migrate::Carried::default();
    let receipt = migration_receipt()?;
    // Names already taken: 0.6.0 bugs in a mixed tree, then the ones this plan writes.
    // A 0.6.0 bug already in the tree, with the 0.5.x IDs it answers to. One that holds
    // a migrating bug's old ID is that bug's own write from an interrupted apply, so its
    // name is not taken from it and the plan, and its digest, stay the same.
    let existing: Vec<(String, Vec<String>)> = records_in(root, BUGS_DIR, Kind::Bug.bound())
        .into_iter()
        .map(|(name, record)| {
            (
                name.trim_end_matches(".md").to_string(),
                record.list("aliases"),
            )
        })
        .collect();
    let mut planned: BTreeSet<String> = BTreeSet::new();
    let taken = |name: &str, old: &str, planned: &BTreeSet<String>| {
        planned.contains(name)
            || existing
                .iter()
                .any(|(other, aliases)| other == name && !aliases.iter().any(|alias| alias == old))
    };
    let local = Local::current();
    for (source, legacy) in legacy_bugs(root)? {
        carried.sources.insert(source.clone());
        let linked = legacy.linked();
        // The observations the report itself linked; later ones arrive in notes.
        let noted: BTreeSet<&String> = legacy
            .notes
            .iter()
            .flat_map(|note| &note.observations)
            .collect();
        let reported: Vec<String> = legacy
            .observations
            .iter()
            .filter(|id| !noted.contains(id))
            .cloned()
            .collect();
        let draft = Draft {
            title: legacy.title.clone(),
            symptom: legacy.symptom.clone(),
            trigger: legacy.trigger.clone(),
            expected: legacy.expected.clone(),
            actual: legacy.actual.clone(),
            steps: legacy.steps.clone(),
            impact: legacy.impact.clone(),
            classification: legacy.classification.clone(),
            scope: legacy.scope.clone(),
            criteria: legacy.criteria.clone(),
            requirements: legacy.requirements.clone(),
            cases: legacy.cases.clone(),
            evidence: reported,
            regression_baseline: legacy.regression_baseline.clone(),
            fixture: legacy.fixture.clone(),
            environment: legacy.environment.clone(),
            source: legacy.source.clone(),
            notes: vec![],
        };
        let fix = if legacy.is_open() {
            NOT_FIXED
        } else {
            "Recorded before 0.6.0; the Log keeps its history."
        };
        let mut record = draft_record(
            &draft,
            legacy.created_at_unix_seconds,
            None,
            legacy.revision.clone(),
            fix,
        )
        .with_context(|| format!("{source} does not carry into a bug record"))?;
        record.set_scalar("status", &legacy.status)?;
        record.set("aliases", Front::List(vec![legacy.id.clone()]))?;
        // The evidence it links: its observations, then its notes'; the regression
        // baseline keeps its own field.
        let mut evidence: Vec<String> = vec![];
        for id in legacy
            .observations
            .iter()
            .chain(legacy.notes.iter().flat_map(|note| &note.observations))
        {
            if !evidence.contains(id) {
                evidence.push(id.clone());
            }
        }
        record.set("evidence", Front::List(evidence))?;
        let mut receipts: Vec<Vec<(String, String)>> = vec![];
        for id in &linked {
            if let Some(map) = homes.receipts.get(id) {
                if !receipts.contains(map) {
                    receipts.push(map.clone());
                }
            } else if !homes.entries.contains_key(id) {
                receipts.push(vec![
                    ("id".to_string(), id.clone()),
                    ("removed".to_string(), receipt.clone()),
                ]);
                carried.aliases.insert(id.clone(), "removed".to_string());
            }
        }
        record.set("receipts", Front::Maps(receipts))?;
        record.set("related", Front::List(legacy.related.clone()))?;
        if let Some(original) = &legacy.duplicate_of {
            record.set_scalar("duplicate_of", original)?;
        }
        record.set_scalar("updated", &iso_time(legacy.updated_at_unix_seconds))?;
        let entries: Vec<String> = legacy
            .notes
            .iter()
            .map(|note| {
                let mut text = note.text.clone();
                if !note.observations.is_empty() {
                    text.push_str(&format!("; evidence: {}", note.observations.join(", ")));
                }
                log_line(note.at_unix_seconds, &text, None)
            })
            .collect();
        if !entries.is_empty() {
            record.append_to_section(LOG, &(entries.join("\n") + "\n"))?;
        }
        let title = record.scalar("title").unwrap_or_default().to_string();
        let mut name = identity::dated_name(
            &identity::utc_date(legacy.created_at_unix_seconds),
            &title,
            "bug",
        );
        if taken(&name, &legacy.id, &planned) {
            name = format!("{name}-{}", migration_suffix(&legacy.id));
            ensure!(
                !taken(&name, &legacy.id, &planned),
                "refused: {source} and another bug both migrate to {BUGS_DIR}/{name}.md, so no plan was made"
            );
        }
        planned.insert(name.clone());
        record.set_scalar("id", &name)?;
        let path = format!("{BUGS_DIR}/{name}.md");
        let stored = Stored {
            path: path.clone(),
            record,
        };
        validate(&stored).with_context(|| format!("{source} does not carry into a bug record"))?;
        carried.written.push(crate::migrate::Written {
            path: path.clone(),
            bytes: stored.record.render_within(Kind::Bug)?,
        });
        carried.aliases.insert(legacy.id.clone(), path);
        if let Some(fixture) = &legacy.fixture {
            for file in fixture_files(root, fixture) {
                if legacy.is_open() {
                    let bytes = std::fs::read(local_path(root, &file)?)?;
                    let mut kinds: Vec<String> =
                        leaks::find(local, &String::from_utf8_lossy(&bytes))
                            .iter()
                            .map(|found| found.kind.name().to_string())
                            .collect();
                    kinds.sort();
                    kinds.dedup();
                    if !kinds.is_empty() {
                        carried.kept_with_leaks.insert(file, kinds);
                    }
                } else if file.starts_with(".chough/artifacts/") {
                    carried.removed.insert(
                        file,
                        format!(
                            "the fixture of closed bug {}, which no open bug names",
                            legacy.id
                        ),
                    );
                }
            }
        }
    }
    Ok(carried)
}

pub(super) fn run(root: &Path, command: BugCommand) -> Result<()> {
    match command {
        BugCommand::Add(args) => print_json(&add(root, *args)?),
        BugCommand::Update(args) => print_json(&update(root, args)?),
        BugCommand::List { all } => {
            let evidence = Evidence::new(root);
            let mut listed = vec![];
            for bug in read_all(root)?.iter().filter(|bug| all || bug.is_open()) {
                let mut value = brief(&evidence, bug)?;
                if !bug.is_open() {
                    value["closed_as"] = json!(bug.status().replace('-', " "));
                }
                listed.push(value);
            }
            print_json(&json!({"schema_version": 1, "open_only": !all, "bugs": listed}))
        }
        BugCommand::Search {
            text,
            criteria,
            requirements,
            cases,
        } => print_json(&json!({
            "schema_version": 1,
            "matches": search(&read_all(root)?, &text, &criteria, &requirements, &cases),
            "note": "Scores compare words in title, symptom and trigger plus shared IDs; decide duplicates from the evidence.",
        })),
        BugCommand::Show { id } => {
            let bug = load(root, &id)?;
            let evidence = Evidence::new(root);
            print_json(&json!({
                "schema_version": 1,
                "path": bug.path,
                "bug": bug_json(&bug),
                "verification": evidence.verification(&bug)?,
                "observation_labels": evidence.labels(&bug)?,
                "label_note": "missing evidence never counts as reproduced, passed or verifying",
            }))
        }
        BugCommand::Merge { base, ours, theirs } => merge::files(&base, &ours, &theirs),
        BugCommand::Report { id } => {
            let bug = load(root, &id)?;
            print!("{}", report(&Evidence::new(root), &bug)?);
            Ok(())
        }
    }
}

/// `chough bug merge <base> <ours> <theirs>`, Git's merge driver for `.chough/bugs/*.md`
/// (REQ-cli-047, DRIVE-12).
mod merge {
    use super::{LIST_KEYS, LOG, Stored, validate};
    use crate::markdown::{self, Kind, Record, Value as Front};
    use anyhow::{Context, Result, bail};
    use std::path::Path;

    /// Merge the three files into `ours` the way bug commands write a bug, or fail with
    /// `ours` unchanged, naming each field or paragraph both sides changed differently.
    pub(super) fn files(base: &Path, ours: &Path, theirs: &Path) -> Result<()> {
        let read = |path: &Path| -> Result<(String, Record)> {
            let text = crate::records::read_bounded_record(path, Kind::Bug.bound())?;
            let record = markdown::parse(&path.display().to_string(), &text)
                .with_context(|| format!("{} is not a bug record", path.display()))?;
            Ok((text, record))
        };
        let (_, base) = read(base)?;
        let (_, mine) = read(ours)?;
        let (_, other) = read(theirs)?;
        let content = merge(&base, &mine, &other)?;
        let staged = ours.with_extension("chough-merge.tmp");
        std::fs::write(&staged, content)?;
        std::fs::rename(&staged, ours)?;
        Ok(())
    }

    /// The merged file text.
    pub(super) fn merge(base: &Record, ours: &Record, theirs: &Record) -> Result<String> {
        let mut conflicts = vec![];
        let front = merge_front(base, ours, theirs, &mut conflicts)?;
        let body = merge_body(base.body(), ours.body(), theirs.body(), &mut conflicts);
        if !conflicts.is_empty() {
            bail!(
                "both sides changed {} differently; resolve this bug by hand",
                conflicts.join(", ")
            );
        }
        // The merged front matter rendered as bug commands render it, then the body.
        let rendered = front.render();
        let front_text = rendered
            .strip_suffix(ours.body())
            .context("the merged front matter did not render")?;
        let text = format!("{front_text}{body}");
        let record = markdown::parse("the merged bug", &text)?;
        let stored = Stored {
            path: "the merged bug".into(),
            record,
        };
        validate(&stored).context("the merged bug is not a valid record")?;
        stored.record.render_within(Kind::Bug)?;
        Ok(text)
    }

    /// A field only one side changed takes that change; list fields keep base's entries
    /// and then each side's additions without duplicates; `updated` is the later one;
    /// any other field both sides changed differently is a conflict.
    fn merge_front(
        base: &Record,
        ours: &Record,
        theirs: &Record,
        conflicts: &mut Vec<String>,
    ) -> Result<Record> {
        let mut merged = ours.clone();
        let mut keys: Vec<String> = vec![];
        for key in ours
            .keys()
            .into_iter()
            .chain(theirs.keys())
            .chain(base.keys())
        {
            if !keys.iter().any(|known| known == key) {
                keys.push(key.to_string());
            }
        }
        for key in keys {
            let (b, o, t) = (base.get(&key), ours.get(&key), theirs.get(&key));
            if o == t || b == t {
                continue;
            }
            if b == o {
                match t {
                    Some(value) => merged.set(&key, value.clone())?,
                    None => merged.remove(&key),
                }
            } else if key == "updated" {
                let later = match (o, t) {
                    (Some(Front::Scalar(left)), Some(Front::Scalar(right))) => {
                        left.clone().max(right.clone())
                    }
                    (Some(Front::Scalar(one)), _) | (_, Some(Front::Scalar(one))) => one.clone(),
                    _ => {
                        conflicts.push(key.clone());
                        continue;
                    }
                };
                merged.set_scalar(&key, &later)?;
            } else if LIST_KEYS.contains(&key.as_str()) {
                match union(b, o, t) {
                    Some(value) => merged.set(&key, value)?,
                    None => conflicts.push(key.clone()),
                }
            } else {
                conflicts.push(key.clone());
            }
        }
        Ok(merged)
    }

    /// Base's entries, then ours' additions, then theirs', without duplicates.
    fn union(base: Option<&Front>, ours: Option<&Front>, theirs: Option<&Front>) -> Option<Front> {
        let maps = [base, ours, theirs]
            .iter()
            .any(|value| matches!(value, Some(Front::Maps(_))));
        if maps {
            let items = |value: Option<&Front>| -> Option<Vec<Vec<(String, String)>>> {
                match value {
                    None => Some(vec![]),
                    Some(Front::Maps(maps)) => Some(maps.clone()),
                    Some(Front::List(items)) if items.is_empty() => Some(vec![]),
                    _ => None,
                }
            };
            let mut merged: Vec<Vec<(String, String)>> = vec![];
            for item in items(base)?
                .into_iter()
                .chain(items(ours)?)
                .chain(items(theirs)?)
            {
                if !merged.contains(&item) {
                    merged.push(item);
                }
            }
            return Some(Front::Maps(merged));
        }
        let items = |value: Option<&Front>| -> Option<Vec<String>> {
            match value {
                None => Some(vec![]),
                Some(Front::List(items)) => Some(items.clone()),
                Some(Front::Scalar(text)) if text.is_empty() => Some(vec![]),
                Some(Front::Scalar(text)) => Some(vec![text.clone()]),
                Some(Front::Maps(_)) => None,
            }
        };
        let mut merged: Vec<String> = vec![];
        for item in items(base)?
            .into_iter()
            .chain(items(ours)?)
            .chain(items(theirs)?)
        {
            if !merged.contains(&item) {
                merged.push(item);
            }
        }
        Some(Front::List(merged))
    }

    /// A body split at its `## Log` section: the write-up before it, the section's
    /// heading line, its entries' text and whatever follows it.
    struct Parts<'a> {
        before: &'a str,
        heading: &'a str,
        log: &'a str,
        after: &'a str,
    }

    fn parts(body: &str) -> Parts<'_> {
        let mut offset = 0;
        let mut fenced = false;
        let mut heading = None;
        for line in body.split_inclusive('\n') {
            let content = line.trim_end();
            let fence = content.trim_start().starts_with("```");
            if !fenced && !fence {
                if let Some((start, end)) = heading {
                    if content.starts_with("# ") || content.starts_with("## ") {
                        return Parts {
                            before: &body[..start],
                            heading: &body[start..end],
                            log: &body[end..offset],
                            after: &body[offset..],
                        };
                    }
                } else if content == format!("## {LOG}") {
                    heading = Some((offset, offset + line.len()));
                }
            }
            if fence {
                fenced = !fenced;
            }
            offset += line.len();
        }
        match heading {
            Some((start, end)) => Parts {
                before: &body[..start],
                heading: &body[start..end],
                log: &body[end..],
                after: "",
            },
            None => Parts {
                before: body,
                heading: "",
                log: "",
                after: "",
            },
        }
    }

    fn merge_body(base: &str, ours: &str, theirs: &str, conflicts: &mut Vec<String>) -> String {
        if ours == theirs || base == theirs {
            return ours.to_string();
        }
        if base == ours {
            return theirs.to_string();
        }
        let (b, o, t) = (parts(base), parts(ours), parts(theirs));
        let heading = [o.heading, t.heading, b.heading]
            .into_iter()
            .find(|heading| !heading.is_empty())
            .unwrap_or_default();
        format!(
            "{}{heading}{}{}",
            merge_prose(b.before, o.before, t.before, conflicts),
            merge_log(b.log, o.log, t.log),
            merge_prose(b.after, o.after, t.after, conflicts),
        )
    }

    /// Base's entries, then each side's added entries in order, without duplicates. An
    /// entry is a `- ` line with the indented lines under it.
    fn merge_log(base: &str, ours: &str, theirs: &str) -> String {
        fn entries(text: &str) -> Vec<String> {
            let mut entries: Vec<String> = vec![];
            for line in text.split_inclusive('\n') {
                if line.trim().is_empty() {
                    continue;
                }
                let line = if line.ends_with('\n') {
                    line.to_string()
                } else {
                    format!("{line}\n")
                };
                match entries.last_mut() {
                    Some(last) if line.starts_with(' ') || line.starts_with('\t') => {
                        last.push_str(&line)
                    }
                    _ => entries.push(line),
                }
            }
            entries
        }
        let leading: String = ours
            .split_inclusive('\n')
            .take_while(|line| line.trim().is_empty())
            .collect();
        let trailing_blank = ours
            .split_inclusive('\n')
            .rev()
            .take_while(|line| line.trim().is_empty())
            .count();
        let base_entries = entries(base);
        let mut merged = base_entries.clone();
        for entry in entries(ours).into_iter().chain(entries(theirs)) {
            if !merged.contains(&entry) {
                merged.push(entry);
            }
        }
        if merged.is_empty() {
            return ours.to_string();
        }
        format!(
            "{leading}{}{}",
            merged.concat(),
            "\n".repeat(trailing_blank)
        )
    }

    /// A block of the write-up: a paragraph or heading with the blank lines after it.
    #[derive(Clone, PartialEq)]
    struct Block {
        key: String,
        text: String,
    }

    fn blocks(text: &str) -> Vec<Block> {
        let mut blocks: Vec<String> = vec![];
        let mut ended = true;
        for line in text.split_inclusive('\n') {
            let blank = line.trim().is_empty();
            match blocks.last_mut() {
                Some(last) if blank || !ended => last.push_str(line),
                _ => blocks.push(line.to_string()),
            }
            ended = blank;
        }
        blocks
            .into_iter()
            .map(|text| {
                let first = text.lines().next().unwrap_or_default();
                let key = if first.starts_with("# ") {
                    "#".to_string()
                } else if first.starts_with("## ") {
                    format!("heading:{}", first.trim_end())
                } else if let Some(label) = first
                    .strip_prefix("**")
                    .and_then(|rest| rest.split_once(":**"))
                    .map(|(label, _)| label)
                {
                    format!("label:{label}")
                } else {
                    format!("text:{}", text.trim_end())
                };
                Block { key, text }
            })
            .collect()
    }

    /// The longest common subsequence of two block-key sequences, as index pairs.
    fn matches(left: &[Block], right: &[Block]) -> Vec<(usize, usize)> {
        let (rows, columns) = (left.len(), right.len());
        let mut table = vec![vec![0usize; columns + 1]; rows + 1];
        for row in (0..rows).rev() {
            for column in (0..columns).rev() {
                table[row][column] = if left[row].key == right[column].key {
                    table[row + 1][column + 1] + 1
                } else {
                    table[row + 1][column].max(table[row][column + 1])
                };
            }
        }
        let (mut row, mut column, mut pairs) = (0, 0, vec![]);
        while row < rows && column < columns {
            if left[row].key == right[column].key {
                pairs.push((row, column));
                row += 1;
                column += 1;
            } else if table[row + 1][column] >= table[row][column + 1] {
                row += 1;
            } else {
                column += 1;
            }
        }
        pairs
    }

    /// A three-way merge of the write-up by blocks: blocks are matched by their label
    /// (the title, each `**Label:**` paragraph, each heading) or, for other paragraphs,
    /// by their text. A matched block only one side changed takes that change; between
    /// matched blocks, a run only one side changed takes that change; anything both sides
    /// changed differently is a conflict, named by its label.
    fn merge_prose(base: &str, ours: &str, theirs: &str, conflicts: &mut Vec<String>) -> String {
        if ours == theirs || base == theirs {
            return ours.to_string();
        }
        if base == ours {
            return theirs.to_string();
        }
        let (b, o, t) = (blocks(base), blocks(ours), blocks(theirs));
        let with_ours = matches(&b, &o);
        let with_theirs = matches(&b, &t);
        // Base blocks matched on both sides anchor the merge.
        let mut anchors = vec![];
        for (base_index, ours_index) in &with_ours {
            if let Some((_, theirs_index)) =
                with_theirs.iter().find(|(index, _)| index == base_index)
            {
                anchors.push((*base_index, *ours_index, *theirs_index));
            }
        }
        let mut out = String::new();
        let (mut base_at, mut ours_at, mut theirs_at) = (0, 0, 0);
        let name = |blocks: &[Block]| -> String {
            blocks
                .first()
                .map_or_else(|| "a paragraph".to_string(), |block| label(&block.key))
        };
        let mut chunk = |base: &[Block], ours: &[Block], theirs: &[Block], out: &mut String| {
            let text = |blocks: &[Block]| {
                blocks
                    .iter()
                    .map(|block| block.text.as_str())
                    .collect::<String>()
            };
            let (base_text, ours_text, theirs_text) = (text(base), text(ours), text(theirs));
            if ours_text == theirs_text || base_text == theirs_text {
                out.push_str(&ours_text);
            } else if base_text == ours_text {
                out.push_str(&theirs_text);
            } else {
                conflicts.push(name(if base.is_empty() { ours } else { base }));
                out.push_str(&ours_text);
            }
        };
        for (base_index, ours_index, theirs_index) in
            anchors
                .into_iter()
                .chain(std::iter::once((b.len(), o.len(), t.len())))
        {
            chunk(
                &b[base_at..base_index],
                &o[ours_at..ours_index],
                &t[theirs_at..theirs_index],
                &mut out,
            );
            if base_index < b.len() {
                chunk(
                    &b[base_index..=base_index],
                    &o[ours_index..=ours_index],
                    &t[theirs_index..=theirs_index],
                    &mut out,
                );
            }
            (base_at, ours_at, theirs_at) = (base_index + 1, ours_index + 1, theirs_index + 1);
        }
        out
    }

    /// The name of a block for a conflict message.
    fn label(key: &str) -> String {
        if key == "#" {
            "the title".into()
        } else if let Some(label) = key.strip_prefix("label:") {
            format!("the {label} paragraph")
        } else if let Some(heading) = key.strip_prefix("heading:") {
            format!("the {heading} section")
        } else {
            "a paragraph".into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bug(text: &str) -> Record {
        markdown::parse("test.md", text).unwrap()
    }

    const BASE: &str = "---\nkind: bug\nid: 2026-11-10-totals\naliases: [BUG-0012]\ntitle: Totals\nstatus: open\nclassification: product-defect\nscope: in-scope\nimpact: High\ncriteria: []\nrequirements: []\ncases: []\nevidence: []\nreceipts: []\nenvironment: test-local\nfound_by: manual report\nrelated: []\ncreated: 2026-11-10T10:00:00Z\nupdated: 2026-11-10T10:00:00Z\n---\n# Totals\n\n**What happened:** Off by one\n\n**Steps:** Any order\n\n**Expected:** Exact\n\n**Why it matters:** High\n\n**Fix:** Not fixed yet.\n\n## Log\n\n- 2026-11-10T10:00:00Z · Reported (manual report)\n";

    #[test]
    fn one_sided_edits_merge_and_logs_union() {
        let ours = BASE.replace("status: open", "status: verified").replace(
            "updated: 2026-11-10T10:00:00Z",
            "updated: 2026-11-10T11:00:00Z",
        ) + "- 2026-11-10T11:00:00Z · status: open -> verified. Fixed on A\n";
        let theirs = BASE
            .replace("evidence: []", "evidence: [2026-11-10-seen-again]")
            .replace("**Fix:** Not fixed yet.", "**Fix:** Round once.")
            .replace(
                "updated: 2026-11-10T10:00:00Z",
                "updated: 2026-11-10T12:00:00Z",
            )
            + "- 2026-11-10T12:00:00Z · evidence: 2026-11-10-seen-again. Seen on B\n";
        let merged = merge::merge(&bug(BASE), &bug(&ours), &bug(&theirs)).unwrap();
        let record = bug(&merged);
        assert_eq!(record.scalar("status"), Some("verified"));
        assert_eq!(record.scalar("updated"), Some("2026-11-10T12:00:00Z"));
        assert_eq!(record.list("evidence"), vec!["2026-11-10-seen-again"]);
        assert!(merged.contains("**Fix:** Round once.\n"), "{merged}");
        assert!(
            merged.ends_with("Reported (manual report)\n- 2026-11-10T11:00:00Z · status: open -> verified. Fixed on A\n- 2026-11-10T12:00:00Z · evidence: 2026-11-10-seen-again. Seen on B\n"),
            "{merged}"
        );
    }

    #[test]
    fn two_sided_edits_conflict_by_name() {
        let ours = BASE.replace("**Fix:** Not fixed yet.", "**Fix:** A.");
        let theirs = BASE.replace("**Fix:** Not fixed yet.", "**Fix:** B.");
        let error = merge::merge(&bug(BASE), &bug(&ours), &bug(&theirs))
            .unwrap_err()
            .to_string();
        assert!(error.contains("the Fix paragraph"), "{error}");
        let ours = BASE.replace("status: open", "status: deferred");
        let theirs = BASE.replace("status: open", "status: investigating");
        let error = merge::merge(&bug(BASE), &bug(&ours), &bug(&theirs))
            .unwrap_err()
            .to_string();
        assert!(error.contains("status"), "{error}");
    }

    #[test]
    fn adjacent_paragraphs_edited_on_each_side_both_merge() {
        let ours = BASE.replace("**Why it matters:** High", "**Why it matters:** Money.");
        let theirs = BASE.replace("**Fix:** Not fixed yet.", "**Fix:** Round once.");
        let merged = merge::merge(&bug(BASE), &bug(&ours), &bug(&theirs)).unwrap();
        assert!(
            merged.contains("**Why it matters:** Money.\n\n**Fix:** Round once.\n"),
            "{merged}"
        );
    }

    #[test]
    fn only_a_recorded_status_change_marks_verified() {
        assert!(records_verified("status: open -> verified"));
        assert!(records_verified(
            "scope: in-scope -> blocking; status: fix-pending-verification -> verified. done"
        ));
        assert!(!records_verified("free note mentioning verified"));
        assert!(!records_verified(
            "status: open -> fix-pending-verification. verified later"
        ));
        assert!(!records_verified("Additional evidence."));
    }

    /// 2026-11-01T10:00:00Z, a 0.5.x bug's creation time in the migration tests.
    const CREATED: u64 = 1_793_527_200;

    fn legacy_json(id: &str, title: &str, status: &str, extra: serde_json::Value) -> String {
        let mut value = json!({
            "schema_version": 1,
            "id": id,
            "title": title,
            "symptom": "Totals are off by one",
            "trigger": "Any order",
            "expected": "Exact totals",
            "actual": "Total plus one",
            "criteria": ["FLOW-8"],
            "requirements": ["REQ-cli-012"],
            "cases": ["TC-cli-0001"],
            "environment": "local",
            "revision": "0123456789abcdef0123456789abcdef01234567",
            "steps": ["Open checkout"],
            "fixture": null,
            "observations": [],
            "impact": "Every order",
            "classification": "product-defect",
            "scope": "in-scope",
            "status": status,
            "regression_baseline": null,
            "duplicate_of": null,
            "related": [],
            "source": "manual report",
            "notes": [],
            "created_at_unix_seconds": CREATED,
            "updated_at_unix_seconds": CREATED + 500,
        });
        for (key, extra) in extra.as_object().unwrap() {
            value[key] = extra.clone();
        }
        serde_json::to_string_pretty(&value).unwrap()
    }

    /// TC-records-migrate-from-0-5, the bug mapper (REQ-records-009, REQ-cli-012): each
    /// 0.5.x bug, of either ID form, becomes the Markdown bug `bug add` writes, named by
    /// its creation date and title, its 0.5.x ID an alias, with every field, note and
    /// status change at its original time. Linked observations become receipts, evidence
    /// links or tombstones naming the migration receipt; an open bug's fixture is kept
    /// and its leak listed; a closed bug's artifact fixture is removed; a same-day,
    /// same-title clash gets the suffix its old ID derives.
    #[test]
    fn migrated_bugs_carry_every_field_alias_link_and_fixture() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let write = |relative: &str, text: &str| {
            let path = root.join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        };
        write(
            ".chough/bugs/BUG-0007.json",
            &legacy_json(
                "BUG-0007",
                "Totals are off by one",
                "investigating",
                json!({
                    "scope": "blocking",
                    "fixture": ".chough/artifacts/repro/",
                    "observations": ["OBS-entry", "OBS-receipt", "OBS-gone", "OBS-later"],
                    "related": ["BUG-closed-receipts-0123456789abcdef01234567"],
                    "notes": [{"at_unix_seconds": CREATED + 100, "text": "status: open -> investigating. Seen again", "observations": ["OBS-later"]}],
                }),
            ),
        );
        write(
            ".chough/artifacts/repro/case.log",
            "failed under /Users/SYNTHETIC-user/project\n",
        );
        write(".chough/artifacts/repro/clean.log", "failed\n");
        write(
            ".chough/bugs/BUG-closed-receipts-0123456789abcdef01234567.json",
            &legacy_json(
                "BUG-closed-receipts-0123456789abcdef01234567",
                "Closed receipts",
                "verified",
                json!({
                    "fixture": ".chough/artifacts/closed/proof.log",
                    "observations": ["OBS-receipt"],
                    "notes": [{"at_unix_seconds": CREATED + 300, "text": "status: fix-pending-verification -> verified", "observations": []}],
                }),
            ),
        );
        write(".chough/artifacts/closed/proof.log", "passed\n");
        // The same title on the same day as BUG-0007.
        write(
            ".chough/bugs/BUG-0008.json",
            &legacy_json(
                "BUG-0008",
                "Totals are off by one",
                "open",
                json!({"fixture": "tests/cli.rs"}),
            ),
        );
        let mut homes = crate::migrate::ObservationHomes::default();
        homes.entries.insert(
            "OBS-entry".into(),
            ".chough/sessions/2026-11-01-work/evidence/2026-11-01-entry.md".into(),
        );
        let receipt_map = vec![
            ("id".to_string(), "2026-11-01-receipt".to_string()),
            ("aliases".to_string(), "OBS-receipt".to_string()),
            ("result".to_string(), "failed".to_string()),
            ("observed".to_string(), "2026-11-01T10:01:00Z".to_string()),
        ];
        homes
            .receipts
            .insert("OBS-receipt".into(), receipt_map.clone());

        assert_eq!(
            links_0_5(root).unwrap().get("BUG-0007"),
            Some(&vec![
                "OBS-entry".to_string(),
                "OBS-receipt".to_string(),
                "OBS-gone".to_string(),
                "OBS-later".to_string()
            ])
        );
        let carried = migrate_0_5(root, &homes).unwrap();
        assert_eq!(
            carried.sources.iter().cloned().collect::<Vec<_>>(),
            vec![
                ".chough/bugs/BUG-0007.json",
                ".chough/bugs/BUG-0008.json",
                ".chough/bugs/BUG-closed-receipts-0123456789abcdef01234567.json"
            ]
        );
        let open = ".chough/bugs/2026-11-01-totals-are-off-by-one.md";
        let clash = format!(
            ".chough/bugs/2026-11-01-totals-are-off-by-one-{}.md",
            migration_suffix("BUG-0008")
        );
        let closed = ".chough/bugs/2026-11-01-closed-receipts.md";
        assert_eq!(carried.aliases.get("BUG-0007"), Some(&open.to_string()));
        assert_eq!(carried.aliases.get("BUG-0008"), Some(&clash));
        assert_eq!(
            carried
                .aliases
                .get("BUG-closed-receipts-0123456789abcdef01234567"),
            Some(&closed.to_string())
        );
        for removed in ["OBS-gone", "OBS-later"] {
            assert_eq!(carried.aliases.get(removed), Some(&"removed".to_string()));
        }
        assert!(!carried.aliases.contains_key("OBS-entry"));
        assert!(!carried.aliases.contains_key("OBS-receipt"));
        let written = |path: &str| -> Record {
            let file = carried
                .written
                .iter()
                .find(|written| written.path == path)
                .unwrap_or_else(|| panic!("{path} not written: {:?}", carried.written));
            markdown::parse(path, &file.bytes).unwrap()
        };

        // The open bug: every field, its alias, its links and its Log at original times.
        let bug = written(open);
        let receipt = migration_receipt().unwrap();
        assert_eq!(
            bug.keys(),
            vec![
                "kind",
                "id",
                "aliases",
                "title",
                "status",
                "classification",
                "scope",
                "impact",
                "criteria",
                "requirements",
                "cases",
                "evidence",
                "receipts",
                "fixture",
                "environment",
                "revision",
                "found_by",
                "related",
                "created",
                "updated"
            ]
        );
        assert_eq!(bug.scalar("id"), Some("2026-11-01-totals-are-off-by-one"));
        assert_eq!(bug.list("aliases"), vec!["BUG-0007"]);
        assert_eq!(bug.scalar("status"), Some("investigating"));
        assert_eq!(bug.scalar("scope"), Some("blocking"));
        assert_eq!(
            bug.list("evidence"),
            vec!["OBS-entry", "OBS-receipt", "OBS-gone", "OBS-later"]
        );
        let tombstone = |id: &str| {
            vec![
                ("id".to_string(), id.to_string()),
                ("removed".to_string(), receipt.clone()),
            ]
        };
        assert_eq!(
            bug.maps("receipts"),
            vec![
                receipt_map.clone(),
                tombstone("OBS-gone"),
                tombstone("OBS-later")
            ]
        );
        assert_eq!(
            bug.list("related"),
            vec!["BUG-closed-receipts-0123456789abcdef01234567"]
        );
        assert_eq!(bug.scalar("created"), Some("2026-11-01T10:00:00Z"));
        assert_eq!(bug.scalar("updated"), Some("2026-11-01T10:08:20Z"));
        assert_eq!(bug.scalar("found_by"), Some("manual report"));
        assert_eq!(bug.get("actor"), None);
        let stored = Stored {
            path: open.to_string(),
            record: bug,
        };
        assert_eq!(
            stored.log(),
            vec![
                "- 2026-11-01T10:00:00Z · Reported (manual report); evidence: OBS-entry, OBS-receipt, OBS-gone",
                "- 2026-11-01T10:01:40Z · status: open -> investigating. Seen again; evidence: OBS-later",
            ]
        );
        assert_eq!(stored.paragraph(FIX), NOT_FIXED);
        assert_eq!(stored.symptom(), "Totals are off by one");
        assert_eq!(stored.actual(), "Total plus one");
        assert_eq!(stored.steps(), vec!["Open checkout"]);

        // Removed links never count: they read `removed`, naming the receipt.
        let evidence = Evidence::new(root);
        let labels = evidence.labels(&stored).unwrap();
        let label = |id: &str| {
            labels
                .iter()
                .find(|label| label["id"] == id)
                .cloned()
                .unwrap()
        };
        assert_eq!(label("OBS-gone")["label"], "removed");
        assert_eq!(label("OBS-gone")["receipt"], receipt.as_str());
        assert_eq!(label("OBS-receipt")["label"], "bug-receipt");
        assert_eq!(evidence.verification(&stored).unwrap(), "reproduced");
        let mut gone_only = stored.clone();
        gone_only
            .record
            .set("evidence", Front::List(vec!["OBS-gone".into()]))
            .unwrap();
        assert_eq!(evidence.verification(&gone_only).unwrap(), "unverified");

        // The closed bug: its status, its Log and its Fix paragraph.
        let closed_bug = Stored {
            path: closed.to_string(),
            record: written(closed),
        };
        assert_eq!(closed_bug.status(), "verified");
        assert_eq!(
            closed_bug.paragraph(FIX),
            "Recorded before 0.6.0; the Log keeps its history."
        );
        assert!(
            closed_bug
                .log()
                .last()
                .unwrap()
                .ends_with("status: fix-pending-verification -> verified")
        );
        assert_eq!(closed_bug.record.maps("receipts"), vec![receipt_map]);

        // Fixtures: the open bug's files are kept, the one holding a leak listed; the
        // closed bug's artifact is removed; a fixture outside the artifacts is untouched.
        assert_eq!(
            carried.kept_with_leaks,
            [(
                ".chough/artifacts/repro/case.log".to_string(),
                vec!["home-path".to_string()]
            )]
            .into()
        );
        assert_eq!(
            carried.removed.keys().cloned().collect::<Vec<_>>(),
            vec![".chough/artifacts/closed/proof.log"]
        );
        assert!(
            carried.removed[".chough/artifacts/closed/proof.log"]
                .contains("BUG-closed-receipts-0123456789abcdef01234567")
        );
        assert_eq!(
            open_fixtures_0_5(root)
                .unwrap()
                .into_iter()
                .collect::<Vec<_>>(),
            vec![
                ".chough/artifacts/repro/case.log",
                ".chough/artifacts/repro/clean.log"
            ]
        );
        // The mapper writes nothing.
        assert!(!root.join(open).exists());
        assert!(root.join(".chough/bugs/BUG-0007.json").exists());
    }

    /// A clash of the derived suffix too refuses the plan, naming the source.
    #[test]
    fn a_migrated_name_taken_twice_refuses_the_plan() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        std::fs::create_dir_all(root.join(".chough/bugs")).unwrap();
        std::fs::write(
            root.join(".chough/bugs/BUG-0001.json"),
            legacy_json("BUG-0001", "Same", "open", json!({})),
        )
        .unwrap();
        // A 0.6.0 bug already holds both names.
        for name in [
            "2026-11-01-same".to_string(),
            format!("2026-11-01-same-{}", migration_suffix("BUG-0001")),
        ] {
            std::fs::write(
                root.join(format!(".chough/bugs/{name}.md")),
                format!("---\nkind: bug\nid: {name}\n---\n# Same\n"),
            )
            .unwrap();
        }
        let error = migrate_0_5(root, &crate::migrate::ObservationHomes::default())
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("refused") && error.contains("BUG-0001.json"),
            "{error}"
        );
    }

    #[test]
    fn paragraphs_and_steps_read_back() {
        let draft = Draft {
            title: "T".into(),
            symptom: "It breaks".into(),
            trigger: "Run it".into(),
            expected: "Works".into(),
            actual: "Broke\nbadly".into(),
            steps: vec!["one".into(), "--two".into()],
            impact: "Some".into(),
            classification: "unclassified".into(),
            scope: "in-scope".into(),
            criteria: vec![],
            requirements: vec![],
            cases: vec![],
            evidence: vec![],
            regression_baseline: None,
            fixture: None,
            environment: "e".into(),
            source: "s".into(),
            notes: vec![],
        };
        let stored = Stored {
            path: String::new(),
            record: Record::new(&write_up(&draft, NOT_FIXED)),
        };
        assert_eq!(stored.symptom(), "It breaks");
        assert_eq!(stored.actual(), "Broke badly");
        assert_eq!(stored.trigger(), "Run it");
        assert_eq!(stored.steps(), vec!["one", "--two"]);
        assert_eq!(stored.paragraph(FIX), "Not fixed yet.");
        assert_eq!(
            log_entry("- 2026-11-10T10:00:00Z · Reported (x) · by a"),
            Some((1_794_304_800, "Reported (x)".to_string()))
        );
    }
}
