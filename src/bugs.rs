//! Local-first bug backlog. Nothing here publishes to an external service.

use crate::evidence::{self, Outcome};
use crate::identity;
use crate::records::{
    Section, ensure_text, head_revision, local_path, now, print_json, read_record, record_names,
    valid_name, write_new, write_replace,
};
use anyhow::{Context, Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
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
const BUG_ID_DOMAIN: &[u8] = b"chough/bug-id/v1";
const BUG_SLUG_LIMIT: usize = 71;

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

#[derive(Serialize)]
struct CreationPayload<'a> {
    schema_version: u32,
    title: &'a str,
    symptom: &'a str,
    trigger: &'a str,
    expected: &'a str,
    actual: &'a str,
    criteria: &'a [String],
    requirements: &'a [String],
    cases: &'a [String],
    environment: &'a str,
    revision: &'a Option<String>,
    steps: &'a [String],
    fixture: &'a Option<String>,
    observations: &'a [String],
    impact: &'a str,
    classification: &'a str,
    scope: &'a str,
    status: &'a str,
    regression_baseline: &'a Option<String>,
    duplicate_of: &'a Option<String>,
    related: &'a [String],
    source: &'a str,
    notes: &'a [Note],
    created_at_unix_seconds: u64,
}

impl<'a> From<&'a Bug> for CreationPayload<'a> {
    fn from(bug: &'a Bug) -> Self {
        Self {
            schema_version: bug.schema_version,
            title: &bug.title,
            symptom: &bug.symptom,
            trigger: &bug.trigger,
            expected: &bug.expected,
            actual: &bug.actual,
            criteria: &bug.criteria,
            requirements: &bug.requirements,
            cases: &bug.cases,
            environment: &bug.environment,
            revision: &bug.revision,
            steps: &bug.steps,
            fixture: &bug.fixture,
            observations: &bug.observations,
            impact: &bug.impact,
            classification: &bug.classification,
            scope: &bug.scope,
            status: &bug.status,
            regression_baseline: &bug.regression_baseline,
            duplicate_of: &bug.duplicate_of,
            related: &bug.related,
            source: &bug.source,
            notes: &bug.notes,
            created_at_unix_seconds: bug.created_at_unix_seconds,
        }
    }
}

fn canonical_creation_payload(bug: &Bug) -> Result<Vec<u8>> {
    Ok(serde_json::to_vec(&CreationPayload::from(bug))?)
}

fn title_slug(title: &str) -> String {
    identity::slug(title, "bug", BUG_SLUG_LIMIT)
}

fn generate_bug_id(bug: &Bug) -> Result<String> {
    let payload = canonical_creation_payload(bug)?;
    let mut hasher = Sha256::new();
    hasher.update(BUG_ID_DOMAIN);
    hasher.update([0]);
    hasher.update(payload);
    let digest = hasher.finalize();
    let suffix = digest[..12]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(format!("BUG-{}-{suffix}", title_slug(&bug.title)))
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
    /// One of unclassified, product-defect, regression, environment, requirement-gap or
    /// evidence-gap. Scope values go in --scope.
    #[arg(long)]
    classification: Option<String>,
    /// One of blocking, in-scope or out-of-scope. Classification values go in
    /// --classification.
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
    ensure!(
        valid_bug_id(id),
        "bug IDs look like BUG-0001 or BUG-title-0123456789abcdef01234567"
    );
    load_all(root)?
        .into_iter()
        .find(|bug| bug.id == id)
        .with_context(|| format!("unknown bug: {id}"))
}

fn valid_bug_id(id: &str) -> bool {
    let Some(rest) = id.strip_prefix("BUG-") else {
        return false;
    };
    if rest.len() == 4 && rest.bytes().all(|byte| byte.is_ascii_digit()) {
        return true;
    }
    if id.len() > 100 {
        return false;
    }
    let Some((slug, digest)) = rest.rsplit_once('-') else {
        return false;
    };
    !slug.is_empty()
        && slug.len() <= BUG_SLUG_LIMIT
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && !slug.contains("--")
        && slug
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        && digest.len() == 24
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// A hint for a value that belongs to the other of `--scope` and `--classification`.
fn other_flag(value: &str, other: &[&str], flag: &str) -> String {
    if other.contains(&value) {
        format!("; `{value}` is a {flag}: use --{flag} {value}")
    } else {
        String::new()
    }
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
        "scope must be one of {}{}",
        SCOPES.join(", "),
        other_flag(&bug.scope, &CLASSES, "classification")
    );
    ensure!(
        CLASSES.contains(&bug.classification.as_str()),
        "classification must be one of {}{}",
        CLASSES.join(", "),
        other_flag(&bug.classification, &SCOPES, "scope")
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

/// The recorded times of one bug, for the timing report.
pub(super) struct BugTimes {
    pub(super) id: String,
    pub(super) status: String,
    pub(super) created_at_unix_seconds: u64,
    pub(super) observations: Vec<String>,
    /// Each note's time and text, in record order.
    pub(super) notes: Vec<(u64, String)>,
}

/// Every bug's recorded times, through the bounded, validated loader.
pub(super) fn bug_times(root: &Path) -> Result<Vec<BugTimes>> {
    Ok(load_all(root)?
        .into_iter()
        .map(|bug| BugTimes {
            id: bug.id,
            status: bug.status,
            created_at_unix_seconds: bug.created_at_unix_seconds,
            observations: bug.observations,
            notes: bug
                .notes
                .into_iter()
                .map(|note| (note.at_unix_seconds, note.text))
                .collect(),
        })
        .collect())
}

/// Linked observations that are present; pruned or unavailable ones never count.
fn present_observations(root: &Path, ids: &[String]) -> Result<Vec<evidence::Evidence>> {
    let mut present = vec![];
    for id in ids {
        if evidence::is_present(root, id)? {
            present.push(evidence::load(root, id)?);
        }
    }
    Ok(present)
}

/// The latest present linked failure, and whether a present linked pass was observed
/// strictly after it. Pruned or unavailable observations never count.
fn order(root: &Path, bug: &Bug) -> Result<Option<(evidence::Evidence, bool)>> {
    let present = present_observations(root, &bug.observations)?;
    let Some(latest_failure) = present
        .iter()
        .filter(|observation| observation.outcome == Outcome::Failed)
        .max_by_key(|observation| observation.observed_at_unix_seconds)
        .cloned()
    else {
        return Ok(None);
    };
    let passed_after = present.iter().any(|observation| {
        observation.outcome == Outcome::Passed
            && observation.observed_at_unix_seconds > latest_failure.observed_at_unix_seconds
    });
    Ok(Some((latest_failure, passed_after)))
}

/// Unverified until a failure is linked; reproduced and later passed only when a pass
/// was observed strictly after the latest failure (REQ-cli-012).
fn verification(root: &Path, bug: &Bug) -> Result<&'static str> {
    Ok(match order(root, bug)? {
        None => "unverified",
        Some((_, false)) => "reproduced",
        Some((_, true)) => "reproduced-and-later-passed",
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

fn create(root: &Path, mut bug: Bug) -> Result<Bug> {
    bug.id = generate_bug_id(&bug)?;
    save(root, &bug, true)?;
    Ok(bug)
}

#[cfg(test)]
fn create_with_id(root: &Path, mut bug: Bug, id: &str) -> Result<Bug> {
    bug.id = id.into();
    save(root, &bug, true)?;
    Ok(bug)
}

fn save(root: &Path, bug: &Bug, new: bool) -> Result<()> {
    validate(bug)?;
    let name = format!("{BUGS_DIR}/{}.json", bug.id);
    let content = serde_json::to_string_pretty(bug)? + "\n";
    if new {
        let created = match write_new(root, &name, &content) {
            Ok(created) => created,
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|error| error.kind() == std::io::ErrorKind::AlreadyExists) =>
            {
                false
            }
            Err(error) => return Err(error),
        };
        if created {
            return Ok(());
        }
        let existing = read_record(&local_path(root, &name)?)?;
        ensure!(existing != content, "bug {} already exists", bug.id);
        bail!(
            "bug ID collision: {} already exists with different content; refusing to overwrite or use numeric fallback",
            bug.id
        );
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
        id: String::new(),
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
    let bug = create(root, bug)?;
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
        id: String::new(),
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
    check_regression(root, &bug)?;
    let bug = create(root, bug)?;
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
        match order(root, &bug)? {
            None => bail!(
                "verified needs a linked failed observation that reproduces the bug, then a linked passed observation after it; no failed observation is linked"
            ),
            Some((failure, false)) => bail!(
                "verified needs a linked passed observation after the fix, observed after the latest linked failure {} (observed_at_unix_seconds {})",
                failure.id,
                failure.observed_at_unix_seconds
            ),
            Some((_, true)) => {}
        }
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

/// Every observation each bug links, whatever its status, labeled `<bug-id> <status>`:
/// its observations, note observations and regression baseline.
pub(super) fn observation_references(root: &Path) -> Result<Vec<(String, Vec<String>)>> {
    Ok(load_all(root)?
        .into_iter()
        .map(|bug| {
            let mut ids: Vec<String> = vec![];
            for id in bug
                .observations
                .iter()
                .chain(bug.notes.iter().flat_map(|note| &note.observations))
                .chain(&bug.regression_baseline)
            {
                if !ids.contains(id) {
                    ids.push(id.clone());
                }
            }
            (format!("{} {}", bug.id, bug.status), ids)
        })
        .collect())
}

/// Each bug's fixture path, whatever its status, labeled `<bug-id> <status>`.
pub(super) fn fixture_references(root: &Path) -> Result<Vec<(String, String)>> {
    Ok(load_all(root)?
        .into_iter()
        .filter_map(|bug| {
            let fixture = bug.fixture?;
            Some((format!("{} {}", bug.id, bug.status), fixture))
        })
        .collect())
}

/// Current, historical, pruned or unavailable for each observation a bug links: its
/// observations, regression baseline and note observations, each labeled once.
fn observation_labels(root: &Path, bug: &Bug) -> Result<Vec<Value>> {
    let labels = evidence::Labels::new(root);
    let linked = bug
        .observations
        .iter()
        .map(|id| (id, "observation"))
        .chain(
            bug.regression_baseline
                .iter()
                .map(|id| (id, "regression-baseline")),
        )
        .chain(
            bug.notes
                .iter()
                .flat_map(|note| &note.observations)
                .map(|id| (id, "note")),
        );
    let mut seen = BTreeSet::new();
    let mut values = vec![];
    for (id, role) in linked {
        if seen.insert(id) {
            values.push(json!({"id": id, "role": role, "label": labels.label(root, id, None)?}));
        }
    }
    Ok(values)
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
            print_json(&json!({
                "schema_version": 1,
                "bug": bug,
                "verification": verification(root, &bug)?,
                "observation_labels": observation_labels(root, &bug)?,
                "label_note": "pruned and unavailable observations never count as reproduced, passed or verifying",
            }))
        }
        BugCommand::Report { id } => {
            let bug = load(root, &id)?;
            print!("{}", report(root, &bug)?);
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Arc, Barrier};

    fn bug(title: &str) -> Bug {
        Bug {
            schema_version: 1,
            id: String::new(),
            title: title.into(),
            symptom: "symptom".into(),
            trigger: "trigger".into(),
            expected: "expected".into(),
            actual: "actual".into(),
            criteria: vec!["FLOW-8".into()],
            requirements: vec!["REQ-cli-012".into()],
            cases: vec!["TC-cli-0125".into()],
            environment: "test-local".into(),
            revision: Some("revision".into()),
            steps: vec!["step".into()],
            fixture: None,
            observations: vec![],
            impact: "impact".into(),
            classification: "product-defect".into(),
            scope: "in-scope".into(),
            status: "open".into(),
            regression_baseline: None,
            duplicate_of: None,
            related: vec![],
            source: "unit test".into(),
            notes: vec![],
            created_at_unix_seconds: 1_793_000_000,
            updated_at_unix_seconds: 1_793_000_001,
        }
    }

    fn known_answer_bug() -> Bug {
        let mut bug = bug("  Fix   LOGIN... now!  ");
        bug.symptom = "Quote \" and newline\n".into();
        bug.trigger = "Backslash \\ and tab\t".into();
        bug.expected = "Expected café".into();
        bug.actual = "Actual \u{1}".into();
        bug.criteria = vec!["FLOW-8".into(), "LIFE-5".into()];
        bug.environment = "local-macos-arm64".into();
        bug.revision = Some("abc123".into());
        bug.steps = vec!["first".into(), "second".into()];
        bug.fixture = Some("tests/fixtures/legacy.json".into());
        bug.observations = vec!["observation-1".into()];
        bug.impact = "High".into();
        bug.regression_baseline = Some("baseline-1".into());
        bug.related = vec!["BUG-0001".into()];
        bug.source = "known answer".into();
        bug.notes = vec![Note {
            at_unix_seconds: 42,
            text: "note \"quoted\"\n".into(),
            observations: vec!["note-observation".into()],
        }];
        bug
    }

    #[test]
    fn slug_normalization_is_ascii_bounded_and_nonempty() {
        for (title, expected) in [
            ("  Fix   LOGIN... now!  ", "fix-login-now"),
            ("already--hyphenated", "already-hyphenated"),
            ("ASCII_ separators!!!", "ascii-separators"),
            ("café", "caf"),
            ("猫", "bug"),
            ("!!!", "bug"),
            ("", "bug"),
        ] {
            assert_eq!(title_slug(title), expected, "title: {title:?}");
        }

        let seventy_one = "a".repeat(71);
        let id = generate_bug_id(&bug(&seventy_one)).unwrap();
        assert_eq!(id.len(), 100);
        assert_eq!(&id[4..75], seventy_one);
        assert_eq!(id.as_bytes()[75], b'-');
        assert_eq!(id.len() - 76, 24);
        assert!(valid_bug_id(&id));

        let longer = generate_bug_id(&bug(&"a".repeat(72))).unwrap();
        assert_eq!(&longer[4..75], seventy_one);
        assert_eq!(longer.len(), 100);

        let trimmed = title_slug(&format!("{}-b", "a".repeat(70)));
        assert_eq!(trimmed, "a".repeat(70));
        assert!(!trimmed.ends_with('-'));
    }

    #[test]
    fn canonical_payload_and_digest_match_fixed_known_answer() {
        const PAYLOAD: &str = r#"{"schema_version":1,"title":"  Fix   LOGIN... now!  ","symptom":"Quote \" and newline\n","trigger":"Backslash \\ and tab\t","expected":"Expected café","actual":"Actual \u0001","criteria":["FLOW-8","LIFE-5"],"requirements":["REQ-cli-012"],"cases":["TC-cli-0125"],"environment":"local-macos-arm64","revision":"abc123","steps":["first","second"],"fixture":"tests/fixtures/legacy.json","observations":["observation-1"],"impact":"High","classification":"product-defect","scope":"in-scope","status":"open","regression_baseline":"baseline-1","duplicate_of":null,"related":["BUG-0001"],"source":"known answer","notes":[{"at_unix_seconds":42,"text":"note \"quoted\"\n","observations":["note-observation"]}],"created_at_unix_seconds":1793000000}"#;
        const SUFFIX: &str = "051630f784909bd78369ee90";

        let bug = known_answer_bug();
        assert_eq!(
            canonical_creation_payload(&bug).unwrap(),
            PAYLOAD.as_bytes()
        );
        let mut input = BUG_ID_DOMAIN.to_vec();
        input.push(0);
        input.extend(PAYLOAD.as_bytes());
        let digest = Sha256::digest(input);
        let suffix = digest[..12]
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        assert_eq!(suffix, SUFFIX);
        assert_eq!(
            generate_bug_id(&bug).unwrap(),
            "BUG-fix-login-now-051630f784909bd78369ee90"
        );
    }

    #[test]
    fn canonical_payload_frames_adjacent_fields_and_creation_is_deterministic() {
        let left = bug("Same title");
        let mut right = left.clone();
        right.id = "excluded-from-the-payload".into();
        right.updated_at_unix_seconds += 100;
        assert_eq!(
            generate_bug_id(&left).unwrap(),
            generate_bug_id(&right).unwrap()
        );

        let mut left = bug("Same title");
        left.symptom = "ab".into();
        left.trigger = "c".into();
        let mut right = left.clone();
        right.symptom = "a".into();
        right.trigger = "bc".into();
        assert_ne!(
            canonical_creation_payload(&left).unwrap(),
            canonical_creation_payload(&right).unwrap()
        );
        assert_ne!(
            generate_bug_id(&left).unwrap(),
            generate_bug_id(&right).unwrap()
        );

        let original_slug = title_slug(&left.title);
        right = left.clone();
        right.actual = "different creation content".into();
        let left_id = generate_bug_id(&left).unwrap();
        let right_id = generate_bug_id(&right).unwrap();
        assert_ne!(left_id, right_id);
        assert!(left_id.starts_with(&format!("BUG-{original_slug}-")));
        assert!(right_id.starts_with(&format!("BUG-{original_slug}-")));
    }

    #[test]
    fn bug_id_validation_accepts_legacy_and_generated_forms_only() {
        for id in [
            "BUG-0000",
            "BUG-0019",
            "BUG-9999",
            "BUG-bug-0123456789abcdef01234567",
            "BUG-title-with-runs-0123456789abcdef01234567",
        ] {
            assert!(valid_bug_id(id), "accepted ID: {id}");
        }
        assert!(valid_bug_id(
            &generate_bug_id(&bug("A valid title")).unwrap()
        ));

        let overlong = format!("BUG-{}-{}", "a".repeat(72), "0".repeat(24));
        for id in [
            "BUG-000",
            "BUG-00000",
            "bug-0001",
            "BUG--0123456789abcdef01234567",
            "BUG-bug-0123456789abcdef0123456",
            "BUG-bug-0123456789abcdef012345678",
            "BUG-bug-0123456789abcdef0123456A",
            "BUG-Title-0123456789abcdef01234567",
            "BUG-title--run-0123456789abcdef01234567",
            "BUG--title-0123456789abcdef01234567",
            "BUG-title--0123456789abcdef01234567",
            &overlong,
        ] {
            assert!(!valid_bug_id(id), "refused ID: {id}");
        }
    }

    #[test]
    fn same_id_different_content_is_refused_without_overwrite_or_numeric_fallback() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let original = bug("Collision");
        let id = "BUG-collision-000000000000000000000000";
        create_with_id(root, original.clone(), id).unwrap();
        let path = root.join(format!("{BUGS_DIR}/{id}.json"));
        let before = fs::read(&path).unwrap();

        let mut different = original.clone();
        different.actual = "different content at the selected path".into();
        let error = create_with_id(root, different, id)
            .err()
            .unwrap()
            .to_string();
        assert!(error.contains("bug ID collision"), "{error}");
        assert!(error.contains("different content"), "{error}");
        assert!(error.contains("numeric fallback"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), before);
        assert_eq!(
            record_names(root, BUGS_DIR).unwrap(),
            vec![format!("{id}.json")]
        );

        let error = create_with_id(root, original, id)
            .err()
            .unwrap()
            .to_string();
        assert!(error.contains("already exists"), "{error}");
        assert!(!error.contains("bug ID collision:"), "{error}");
        assert_eq!(fs::read(&path).unwrap(), before);
    }

    #[test]
    fn concurrent_create_new_has_one_winner_and_never_replaces_it() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path().to_path_buf();
        let id = "BUG-race-000000000000000000000000";
        let first = bug("Race");
        let mut second = first.clone();
        second.actual = "a different racing report".into();
        let expected = [first.clone(), second.clone()].map(|mut bug| {
            bug.id = id.into();
            serde_json::to_string_pretty(&bug).unwrap() + "\n"
        });
        let barrier = Arc::new(Barrier::new(2));
        let handles = [first, second].map(|bug| {
            let root = root.clone();
            let barrier = Arc::clone(&barrier);
            std::thread::spawn(move || {
                barrier.wait();
                create_with_id(&root, bug, id)
            })
        });
        let results = handles.map(|handle| handle.join().unwrap());
        assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
        let error = results
            .iter()
            .find_map(|result| result.as_ref().err())
            .unwrap()
            .to_string();
        assert!(error.contains("bug ID collision"), "{error}");
        assert!(error.contains("different content"), "{error}");
        let stored = fs::read_to_string(root.join(format!("{BUGS_DIR}/{id}.json"))).unwrap();
        assert!(expected.contains(&stored));
        assert_eq!(
            record_names(&root, BUGS_DIR).unwrap(),
            vec![format!("{id}.json")]
        );
    }

    #[test]
    fn updates_keep_the_generated_id_and_filename_after_creation_content_changes() {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        let created = create(root, bug("Original title")).unwrap();
        let id = created.id.clone();
        let suffix = id.rsplit_once('-').unwrap().1.to_string();
        let mut changed = created;
        changed.title = "Changed title".into();
        changed.actual = "Changed actual behavior".into();
        changed.status = "investigating".into();
        changed.notes.push(Note {
            at_unix_seconds: 1_793_000_100,
            text: "Updated without identity recomputation".into(),
            observations: vec!["observation-2".into()],
        });
        save(root, &changed, false).unwrap();
        let loaded = load(root, &id).unwrap();
        assert_eq!(loaded.id, id);
        assert_eq!(loaded.title, "Changed title");
        assert_eq!(loaded.id.rsplit_once('-').unwrap().1, suffix);
        assert!(root.join(format!("{BUGS_DIR}/{id}.json")).is_file());
        assert!(
            !root
                .join(".chough/bugs/BUG-changed-title-000000000000000000000000.json")
                .exists()
        );
    }
}
