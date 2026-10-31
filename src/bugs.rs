//! Local-first bug backlog. Nothing here publishes to an external service.

use crate::evidence::{self, Outcome};
use crate::records::{
    Section, ensure_text, head_revision, local_path, now, print_json, read_record, record_names,
    valid_name, write_new, write_replace,
};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeSet;
use std::path::Path;

pub(super) const BUGS_DIR: &str = ".chough/bugs";
const STATUSES: [&str; 7] = [
    "open",
    "investigating",
    "fix-pending-verification",
    "verified",
    "deferred",
    "duplicate",
    "wont-fix",
];
const CLOSED: [&str; 3] = ["verified", "duplicate", "wont-fix"];
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

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Note {
    at_unix_seconds: u64,
    text: String,
    #[serde(default)]
    observations: Vec<String>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Bug {
    schema_version: u32,
    pub(super) id: String,
    pub(super) title: String,
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
    pub(super) classification: String,
    pub(super) scope: String,
    pub(super) status: String,
    #[serde(default)]
    regression_baseline: Option<String>,
    #[serde(default)]
    duplicate_of: Option<String>,
    #[serde(default)]
    related: Vec<String>,
    source: String,
    #[serde(default)]
    notes: Vec<Note>,
    created_at_unix_seconds: u64,
    updated_at_unix_seconds: u64,
}

impl Bug {
    pub(super) fn is_open(&self) -> bool {
        !CLOSED.contains(&self.status.as_str())
    }
}

#[derive(clap::Args)]
pub(super) struct AddArgs {
    #[arg(long)]
    title: String,
    #[arg(long)]
    symptom: String,
    #[arg(long)]
    trigger: String,
    #[arg(long)]
    expected: String,
    #[arg(long)]
    actual: String,
    #[arg(long)]
    environment: String,
    #[arg(long, default_value = "Not yet assessed.")]
    impact: String,
    #[arg(long = "criterion")]
    criteria: Vec<String>,
    #[arg(long = "requirement")]
    requirements: Vec<String>,
    #[arg(long = "case")]
    cases: Vec<String>,
    #[arg(long = "step")]
    steps: Vec<String>,
    #[arg(long)]
    fixture: Option<String>,
    /// A recorded observation that shows the failure.
    #[arg(long = "observation")]
    observations: Vec<String>,
    #[arg(long, default_value = "unclassified")]
    classification: String,
    #[arg(long, default_value = "in-scope")]
    scope: String,
    /// A passed observation at an earlier, ancestor revision (required for regression).
    #[arg(long)]
    regression_baseline: Option<String>,
    #[arg(long, default_value = "manual report")]
    source: String,
    /// Add even though similar bugs exist, stating why this one is distinct.
    #[arg(long)]
    distinct: Option<String>,
}

#[derive(clap::Args)]
pub(super) struct UpdateArgs {
    id: String,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    classification: Option<String>,
    #[arg(long)]
    scope: Option<String>,
    #[arg(long)]
    note: Option<String>,
    #[arg(long = "observation")]
    observations: Vec<String>,
    #[arg(long)]
    regression_baseline: Option<String>,
    #[arg(long)]
    duplicate_of: Option<String>,
    #[arg(long = "related")]
    related: Vec<String>,
}

#[derive(Subcommand)]
pub(super) enum BugCommand {
    /// Record a new local bug after searching for duplicates.
    Add(Box<AddArgs>),
    /// Append notes or evidence and change status, scope or classification.
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
}

pub(super) fn load_all(root: &Path) -> Result<Vec<Bug>> {
    let mut bugs = vec![];
    for name in record_names(root, BUGS_DIR)? {
        let path = local_path(root, &format!("{BUGS_DIR}/{name}"))?;
        let bug: Bug = serde_json::from_str(&read_record(&path)?)
            .with_context(|| format!("invalid bug record: {name}"))?;
        ensure!(
            name == format!("{}.json", bug.id),
            "bug file name must match its ID: {name}"
        );
        validate(&bug)?;
        bugs.push(bug);
    }
    Ok(bugs)
}

fn load(root: &Path, id: &str) -> Result<Bug> {
    ensure!(valid_bug_id(id), "bug IDs look like BUG-0001");
    load_all(root)?
        .into_iter()
        .find(|bug| bug.id == id)
        .with_context(|| format!("unknown bug: {id}"))
}

fn valid_bug_id(id: &str) -> bool {
    id.strip_prefix("BUG-")
        .is_some_and(|number| number.len() == 4 && number.bytes().all(|byte| byte.is_ascii_digit()))
}

fn validate(bug: &Bug) -> Result<()> {
    ensure!(bug.schema_version == 1, "unsupported bug schema_version");
    ensure!(valid_bug_id(&bug.id), "invalid bug ID {}", bug.id);
    for (label, value) in [
        ("title", &bug.title),
        ("symptom", &bug.symptom),
        ("trigger", &bug.trigger),
        ("expected", &bug.expected),
        ("actual", &bug.actual),
        ("environment", &bug.environment),
        ("impact", &bug.impact),
        ("source", &bug.source),
    ] {
        ensure_text(label, value)?;
    }
    ensure!(
        STATUSES.contains(&bug.status.as_str()),
        "status must be one of {}",
        STATUSES.join(", ")
    );
    ensure!(
        SCOPES.contains(&bug.scope.as_str()),
        "scope must be one of {}",
        SCOPES.join(", ")
    );
    ensure!(
        CLASSES.contains(&bug.classification.as_str()),
        "classification must be one of {}",
        CLASSES.join(", ")
    );
    for id in bug
        .criteria
        .iter()
        .chain(&bug.requirements)
        .chain(&bug.cases)
        .chain(&bug.observations)
    {
        ensure!(valid_name(id), "invalid linked ID {id}");
    }
    ensure!(
        bug.duplicate_of.is_none() || bug.status == "duplicate",
        "duplicate_of is only set for duplicate bugs"
    );
    Ok(())
}

/// Reproduced when a linked observation failed; otherwise the report is unverified.
fn verification(root: &Path, bug: &Bug) -> Result<&'static str> {
    let mut failed = false;
    let mut passed_after = false;
    for id in &bug.observations {
        let observation = evidence::load(root, id)?;
        match observation.outcome {
            Outcome::Failed => failed = true,
            Outcome::Passed => passed_after = true,
            Outcome::NotRun => {}
        }
    }
    Ok(match (failed, passed_after) {
        (false, _) => "unverified",
        (true, false) => "reproduced",
        (true, true) => "reproduced-and-later-passed",
    })
}

fn check_observations(root: &Path, ids: &[String]) -> Result<()> {
    for id in ids {
        evidence::load(root, id)?;
    }
    Ok(())
}

fn check_regression(root: &Path, bug: &Bug) -> Result<()> {
    if bug.classification != "regression" {
        return Ok(());
    }
    let baseline_id = bug.regression_baseline.as_deref().context(
        "a regression needs --regression-baseline: a passed observation at an earlier, ancestor revision",
    )?;
    let baseline = evidence::load(root, baseline_id)?;
    ensure!(
        baseline.outcome == Outcome::Passed,
        "the regression baseline {baseline_id} did not pass"
    );
    let failing: Vec<_> = bug
        .observations
        .iter()
        .map(|id| evidence::load(root, id))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .filter(|observation| observation.outcome == Outcome::Failed)
        .collect();
    ensure!(
        !failing.is_empty(),
        "a regression needs a linked failed observation now"
    );
    ensure!(
        evidence::is_ancestor_of_head(root, &baseline.snapshot.revision)
            && failing
                .iter()
                .all(|observation| observation.snapshot.revision != baseline.snapshot.revision),
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

fn similarity(
    bug: &Bug,
    text: &str,
    criteria: &[String],
    requirements: &[String],
    cases: &[String],
) -> f64 {
    let query = tokens(text);
    let known = tokens(&format!("{} {} {}", bug.title, bug.symptom, bug.trigger));
    let union = query.union(&known).count();
    let overlap = if union == 0 {
        0.0
    } else {
        query.intersection(&known).count() as f64 / union as f64
    };
    let shared = bug.criteria.iter().any(|id| criteria.contains(id))
        || bug.requirements.iter().any(|id| requirements.contains(id))
        || bug.cases.iter().any(|id| cases.contains(id));
    overlap + if shared { 0.25 } else { 0.0 }
}

fn search(
    root: &Path,
    text: &str,
    criteria: &[String],
    requirements: &[String],
    cases: &[String],
) -> Result<Vec<Value>> {
    let mut matches: Vec<(f64, Value)> = load_all(root)?
        .iter()
        .map(|bug| {
            let score = similarity(bug, text, criteria, requirements, cases);
            (
                score,
                json!({"id": bug.id, "title": bug.title, "status": bug.status, "score": (score * 100.0).round() / 100.0}),
            )
        })
        .filter(|(score, _)| *score >= 0.2)
        .collect();
    matches.sort_by(|left, right| right.0.total_cmp(&left.0));
    Ok(matches.into_iter().map(|(_, value)| value).collect())
}

fn next_id(root: &Path) -> Result<String> {
    let highest = load_all(root)?
        .iter()
        .filter_map(|bug| bug.id[4..].parse::<u32>().ok())
        .max()
        .unwrap_or(0);
    ensure!(highest < 9_999, "bug ID space exhausted");
    Ok(format!("BUG-{:04}", highest + 1))
}

fn save(root: &Path, bug: &Bug, new: bool) -> Result<()> {
    validate(bug)?;
    let name = format!("{BUGS_DIR}/{}.json", bug.id);
    let content = serde_json::to_string_pretty(bug)? + "\n";
    if new {
        ensure!(
            write_new(root, &name, &content)?,
            "bug {} already exists",
            bug.id
        );
        Ok(())
    } else {
        write_replace(root, &name, &content)
    }
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

/// Append a failed run to the open bug for its case, or open a new local bug.
pub(super) fn capture_failure(root: &Path, finding: Finding<'_>) -> Result<String> {
    let at = now()?;
    if let Some(mut bug) = load_all(root)?
        .into_iter()
        .find(|bug| bug.is_open() && bug.cases.iter().any(|case| case == finding.case))
    {
        bug.observations.push(finding.observation.into());
        bug.notes.push(Note {
            at_unix_seconds: at,
            text: format!("Additional failure from {}.", finding.source),
            observations: vec![finding.observation.into()],
        });
        bug.updated_at_unix_seconds = at;
        save(root, &bug, false)?;
        return Ok(bug.id);
    }
    let bug = Bug {
        schema_version: 1,
        id: next_id(root)?,
        title: format!("{} failed during {}", finding.case, finding.source),
        symptom: finding.summary.into(),
        trigger: format!("Running case {}", finding.case),
        expected: format!("Case {} passes its declared assertion", finding.case),
        actual: finding.summary.into(),
        criteria: finding.criteria,
        requirements: finding.requirements,
        cases: vec![finding.case.into()],
        environment: finding.environment.into(),
        revision: head_revision(root),
        steps: vec![format!(
            "chough cases run {} --environment {}",
            finding.case, finding.environment
        )],
        fixture: None,
        observations: vec![finding.observation.into()],
        impact: "Not yet assessed.".into(),
        classification: "unclassified".into(),
        scope: "in-scope".into(),
        status: "open".into(),
        regression_baseline: None,
        duplicate_of: None,
        related: vec![],
        source: finding.source,
        notes: vec![],
        created_at_unix_seconds: at,
        updated_at_unix_seconds: at,
    };
    save(root, &bug, true)?;
    Ok(bug.id)
}

fn add(root: &Path, args: AddArgs) -> Result<Value> {
    let query = format!("{} {} {}", args.title, args.symptom, args.trigger);
    let candidates = search(
        root,
        &query,
        &args.criteria,
        &args.requirements,
        &args.cases,
    )?;
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
    check_observations(root, &args.observations)?;
    if let Some(fixture) = &args.fixture {
        local_path(root, fixture)?;
    }
    let at = now()?;
    let mut notes = vec![];
    if let Some(reason) = args.distinct {
        ensure_text("distinct reason", &reason)?;
        notes.push(Note {
            at_unix_seconds: at,
            text: format!("Added despite similar reports: {reason}"),
            observations: vec![],
        });
    }
    let bug = Bug {
        schema_version: 1,
        id: next_id(root)?,
        title: args.title,
        symptom: args.symptom,
        trigger: args.trigger,
        expected: args.expected,
        actual: args.actual,
        criteria: args.criteria,
        requirements: args.requirements,
        cases: args.cases,
        environment: args.environment,
        revision: head_revision(root),
        steps: args.steps,
        fixture: args.fixture,
        observations: args.observations,
        impact: args.impact,
        classification: args.classification,
        scope: args.scope,
        status: "open".into(),
        regression_baseline: args.regression_baseline,
        duplicate_of: None,
        related: vec![],
        source: args.source,
        notes,
        created_at_unix_seconds: at,
        updated_at_unix_seconds: at,
    };
    validate(&bug)?;
    check_regression(root, &bug)?;
    save(root, &bug, true)?;
    Ok(json!({
        "schema_version": 1,
        "recorded": format!("{BUGS_DIR}/{}.json", bug.id),
        "id": bug.id,
        "verification": verification(root, &bug)?,
        "similar": candidates,
        "published": false,
    }))
}

fn update(root: &Path, args: UpdateArgs) -> Result<Value> {
    let mut bug = load(root, &args.id)?;
    let at = now()?;
    let mut changes = vec![];
    check_observations(root, &args.observations)?;
    for (field, value) in [
        ("status", args.status),
        ("classification", args.classification),
        ("scope", args.scope),
    ] {
        if let Some(value) = value {
            let slot = match field {
                "status" => &mut bug.status,
                "classification" => &mut bug.classification,
                _ => &mut bug.scope,
            };
            if *slot != value {
                changes.push(format!("{field}: {slot} -> {value}"));
                *slot = value;
            }
        }
    }
    if let Some(baseline) = args.regression_baseline {
        bug.regression_baseline = Some(baseline);
    }
    if let Some(original) = args.duplicate_of {
        ensure!(original != bug.id, "a bug cannot duplicate itself");
        load(root, &original)?;
        bug.duplicate_of = Some(original.clone());
        if bug.status != "duplicate" {
            changes.push(format!("status: {} -> duplicate", bug.status));
            bug.status = "duplicate".into();
        }
    }
    for related in args.related {
        load(root, &related)?;
        if !bug.related.contains(&related) {
            bug.related.push(related);
        }
    }
    for observation in &args.observations {
        if !bug.observations.contains(observation) {
            bug.observations.push(observation.clone());
        }
    }
    if bug.status == "verified" {
        let passed = bug
            .observations
            .iter()
            .map(|id| evidence::load(root, id))
            .collect::<Result<Vec<_>>>()?
            .iter()
            .any(|observation| observation.outcome == Outcome::Passed);
        ensure!(
            passed,
            "verified needs a linked passed observation after the fix"
        );
    }
    check_regression(root, &bug)?;
    if let Some(note) = &args.note {
        ensure_text("note", note)?;
    }
    if args.note.is_some() || !changes.is_empty() || !args.observations.is_empty() {
        let mut text = changes.join("; ");
        if let Some(note) = args.note {
            if !text.is_empty() {
                text.push_str(". ");
            }
            text.push_str(&note);
        }
        if text.is_empty() {
            text = "Additional evidence.".into();
        }
        bug.notes.push(Note {
            at_unix_seconds: at,
            text,
            observations: args.observations,
        });
    }
    bug.updated_at_unix_seconds = at;
    save(root, &bug, false)?;
    Ok(json!({"schema_version": 1, "bug": bug, "verification": verification(root, &bug)?}))
}

fn report(root: &Path, bug: &Bug) -> Result<String> {
    let list = |items: &[String]| {
        if items.is_empty() {
            "none recorded".to_string()
        } else {
            items.join(", ")
        }
    };
    let steps = if bug.steps.is_empty() {
        "Not recorded.".to_string()
    } else {
        bug.steps
            .iter()
            .enumerate()
            .map(|(index, step)| format!("{}. {step}", index + 1))
            .collect::<Vec<_>>()
            .join("\n")
    };
    Ok(format!(
        "<!-- Local report candidate from {BUGS_DIR}/{id}.json. Not published; posting needs an explicit human request. -->\n\
# {title}\n\n\
Classification and confidence: {class}; verification: {verification}\n\
Impact: {impact}\n\
Traceability: HI {criteria}; requirements {requirements}; cases {cases}\n\
Scope and status: {scope}, {status}\n\n\
## Reproduction\n\
Revision: {revision}\n\
Environment: {environment}\n\
Fixture: {fixture}\n\
Steps:\n{steps}\n\
Expected: {expected}\n\
Observed: {actual}\n\n\
## Evidence and deduplication\n\
Observations: {observations}\n\
Related: {related}; duplicate of: {duplicate}\n\
Limits: a reproduction without a linked failed observation is unverified.\n",
        id = bug.id,
        title = bug.title,
        class = bug.classification,
        verification = verification(root, bug)?,
        impact = bug.impact,
        criteria = list(&bug.criteria),
        requirements = list(&bug.requirements),
        cases = list(&bug.cases),
        scope = bug.scope,
        status = bug.status,
        revision = bug.revision.as_deref().unwrap_or("unknown"),
        environment = bug.environment,
        fixture = bug.fixture.as_deref().unwrap_or("none recorded"),
        expected = bug.expected,
        actual = bug.actual,
        observations = list(&bug.observations),
        related = list(&bug.related),
        duplicate = bug.duplicate_of.as_deref().unwrap_or("none"),
    ))
}

fn brief(root: &Path, bug: &Bug) -> Result<Value> {
    Ok(json!({
        "id": bug.id,
        "title": bug.title,
        "status": bug.status,
        "scope": bug.scope,
        "classification": bug.classification,
        "verification": verification(root, bug)?,
    }))
}

/// Open bugs for status, next and context.
pub(super) fn open_backlog(root: &Path) -> Result<Vec<Value>> {
    load_all(root)?
        .iter()
        .filter(|bug| bug.is_open())
        .map(|bug| brief(root, bug))
        .collect()
}

pub(super) fn open_blocking(root: &Path) -> Result<Vec<String>> {
    Ok(load_all(root)?
        .into_iter()
        .filter(|bug| bug.is_open() && bug.scope == "blocking")
        .map(|bug| bug.id)
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

pub(super) fn run(root: &Path, command: BugCommand) -> Result<()> {
    match command {
        BugCommand::Add(args) => print_json(&add(root, *args)?),
        BugCommand::Update(args) => print_json(&update(root, args)?),
        BugCommand::List { all } => {
            let bugs = load_all(root)?;
            let listed = bugs
                .iter()
                .filter(|bug| all || bug.is_open())
                .map(|bug| brief(root, bug))
                .collect::<Result<Vec<_>>>()?;
            print_json(&json!({"schema_version": 1, "open_only": !all, "bugs": listed}))
        }
        BugCommand::Search {
            text,
            criteria,
            requirements,
            cases,
        } => print_json(&json!({
            "schema_version": 1,
            "matches": search(root, &text, &criteria, &requirements, &cases)?,
            "note": "Scores compare words in title, symptom and trigger plus shared IDs; decide duplicates from the evidence.",
        })),
        BugCommand::Show { id } => {
            let bug = load(root, &id)?;
            print_json(
                &json!({"schema_version": 1, "bug": bug, "verification": verification(root, &bug)?}),
            )
        }
        BugCommand::Report { id } => {
            let bug = load(root, &id)?;
            print!("{}", report(root, &bug)?);
            Ok(())
        }
    }
}
