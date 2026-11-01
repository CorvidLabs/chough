//! Stage routing. Native SpecSync status decides gate eligibility; this module only
//! records transitions and refuses those that native state does not allow.

use crate::checkpoint::{Caller, Checkpoint, read_checkpoint, save_checkpoint};
use crate::records::{
    NATIVE_RECORD_LIMIT, NativeJson, append_line, ensure_text, head_revision, is_digest,
    local_path, native_json, now, print_json, read_log_tail, read_native_record, read_record,
};
use crate::{bugs, prefs};
use anyhow::{Result, bail, ensure};
use clap::Subcommand;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::path::Path;

pub(super) const TRANSITIONS: &str = ".chough/transitions.jsonl";

/// Every stage with its kind. `references/stages.md` documents the same names.
pub(super) const STAGES: &[(&str, &str)] = &[
    ("intent", "main"),
    ("design", "conditional"),
    ("plan", "main"),
    ("build", "main"),
    ("test", "main"),
    ("accept", "main"),
    ("done", "terminal"),
    ("release", "optional"),
    ("bugs", "side"),
    ("validate", "side"),
    ("librarian", "side"),
];
const ORDER: [&str; 7] = [
    "intent", "design", "plan", "build", "test", "accept", "done",
];
const BUILD_STATES: [&str; 3] = ["approved", "implementing", "verifying"];

pub(super) fn is_stage(name: &str) -> bool {
    STAGES.iter().any(|(stage, _)| *stage == name)
}

pub(super) fn is_main(name: &str) -> bool {
    ORDER.contains(&name)
}

pub(super) fn is_side(name: &str) -> bool {
    STAGES
        .iter()
        .any(|(stage, kind)| *stage == name && *kind == "side")
}

fn position(name: &str) -> Option<usize> {
    ORDER.iter().position(|stage| *stage == name)
}

#[derive(Subcommand)]
pub(super) enum StageCommand {
    /// Show the current stage, recent transitions and the valid transitions.
    Show,
    /// Move to the next main stage, subject to native gates.
    Advance {
        #[arg(long)]
        note: Option<String>,
    },
    /// Record another iteration within the current stage.
    Iterate {
        #[arg(long)]
        note: String,
    },
    /// Skip the current stage with a recorded reason and move to the next one.
    Skip {
        #[arg(long)]
        reason: String,
    },
    /// Return to an earlier main stage; history is kept and refresh targets are named.
    Back {
        #[arg(long)]
        to: String,
        #[arg(long)]
        reason: String,
    },
    /// Start again from a main stage (intent by default); history is kept.
    Reset {
        #[arg(long, default_value = "intent")]
        to: String,
        #[arg(long)]
        reason: String,
    },
    /// Pause work in the current stage.
    Pause {
        #[arg(long)]
        reason: String,
    },
    /// Resume the paused stage with the same scope, preferences and gates.
    Resume,
    /// Enter a side stage (bugs, validate, librarian) or, when authorized, release.
    Enter {
        stage: String,
        #[arg(long)]
        reason: String,
    },
    /// Leave a side stage and return to its caller.
    Return {
        #[arg(long)]
        note: Option<String>,
    },
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Transition {
    schema_version: u32,
    at_unix_seconds: u64,
    kind: String,
    from: String,
    to: String,
    #[serde(default)]
    reason: Option<String>,
    #[serde(default)]
    gate: Option<String>,
    #[serde(default)]
    revision: Option<String>,
    #[serde(default)]
    refresh: Vec<String>,
}

/// What native lifecycle state allows, without inventing any of it.
pub(super) struct Gate {
    pub(super) allowed: bool,
    pub(super) label: String,
    pub(super) detail: String,
}

/// Native lifecycle state for the checkpoint's change.
#[derive(Clone)]
pub(super) enum NativeState {
    NotConfigured,
    Disabled,
    PolicyInvalid(String),
    NoChange,
    State(String, Value),
    Unknown(String),
}

const POLICY: &str = ".specsync/sdd.json";

/// SpecSync 6.1's `SddPolicy` shape, deserialized only to apply SpecSync's own rules:
/// missing fields take these defaults (SDD enabled), unknown fields are accepted, and a
/// wrong type or a duplicate key makes the policy invalid.
#[derive(Deserialize)]
#[serde(default)]
#[allow(dead_code)]
struct PolicyFile {
    version: u32,
    enabled: bool,
    require_change_for_meaningful_files: bool,
    meaningful_paths: Vec<String>,
    ignored_paths: Vec<String>,
    verification_commands: Vec<String>,
    custom_artifacts: std::collections::BTreeMap<String, String>,
    principles_file: Option<String>,
}

impl Default for PolicyFile {
    fn default() -> Self {
        Self {
            version: 2,
            enabled: true,
            require_change_for_meaningful_files: true,
            meaningful_paths: vec![],
            ignored_paths: vec![],
            verification_commands: vec![],
            custom_artifacts: Default::default(),
            principles_file: None,
        }
    }
}

enum Policy {
    /// No policy file: SDD is not adopted.
    Absent,
    Enabled(bool),
    Invalid(String),
}

fn sdd_policy(root: &Path) -> Result<Policy> {
    let path = local_path(root, POLICY)?;
    if !path.exists() {
        return Ok(Policy::Absent);
    }
    let text = match read_record(&path) {
        Ok(text) => text,
        Err(error) => {
            return Ok(Policy::Invalid(format!(
                "{POLICY} cannot be read: {error:#}"
            )));
        }
    };
    Ok(match serde_json::from_str::<PolicyFile>(&text) {
        Ok(policy) => Policy::Enabled(policy.enabled),
        Err(error) => Policy::Invalid(format!("{POLICY} is not a valid SpecSync policy: {error}")),
    })
}

/// Whether the project has a native SpecSync configuration.
pub(super) fn native_configured(root: &Path) -> Result<bool> {
    Ok(local_path(root, ".specsync/config.toml")?.is_file()
        || local_path(root, ".specsync/config.json")?.is_file())
}

/// Native state decided without a status read, or the change whose status decides it.
fn native_prelude<'a>(
    root: &Path,
    change: Option<&'a str>,
) -> Result<std::result::Result<&'a str, NativeState>> {
    if !native_configured(root)? {
        return Ok(Err(NativeState::NotConfigured));
    }
    match sdd_policy(root)? {
        Policy::Absent | Policy::Enabled(false) => return Ok(Err(NativeState::Disabled)),
        Policy::Invalid(reason) => return Ok(Err(NativeState::PolicyInvalid(reason))),
        Policy::Enabled(true) => {}
    }
    Ok(change.ok_or(NativeState::NoChange))
}

fn state_from_response(response: &NativeJson) -> NativeState {
    let Some(value) = &response.value else {
        let section = &response.section;
        return NativeState::Unknown(format!(
            "{} ({}): {}",
            section.source, section.state, section.content
        ));
    };
    match value.pointer("/summary/state").and_then(Value::as_str) {
        Some(state) => NativeState::State(state.to_string(), value.clone()),
        None => NativeState::Unknown(format!(
            "{}: no summary.state in the response",
            response.section.source
        )),
    }
}

pub(super) fn native_state(root: &Path, change: Option<&str>) -> Result<NativeState> {
    Ok(match native_prelude(root, change)? {
        Err(state) => state,
        Ok(change) => state_from_response(&native_json(
            root,
            "specsync",
            &["change", "status", change, "--json"],
        )),
    })
}

/// Native state from a status response already read, so one read serves a whole report.
/// `response` is the `specsync change status` read made whenever SpecSync is configured.
pub(super) fn native_state_from(
    root: &Path,
    change: Option<&str>,
    response: Option<&NativeJson>,
) -> Result<NativeState> {
    Ok(match (native_prelude(root, change)?, response) {
        (Err(state), _) => state,
        (Ok(_), Some(response)) => state_from_response(response),
        (Ok(_), None) => NativeState::Unknown("native status was not read".into()),
    })
}

fn status_command(change: Option<&str>) -> String {
    match change {
        Some(change) => format!("specsync change status {change}"),
        None => "specsync change status".into(),
    }
}

/// What a destination stage needs from the native lifecycle.
#[derive(Clone, Copy, PartialEq)]
enum Prerequisite {
    /// A valid definition approval: build.
    Approved,
    /// A valid approval, or an archived change: test and accept.
    ApprovedOrArchived,
    /// An archived change with authenticated terminal evidence: done.
    Archived,
}

fn prerequisite(destination: &str) -> Option<Prerequisite> {
    match destination {
        "build" => Some(Prerequisite::Approved),
        "test" | "accept" => Some(Prerequisite::ApprovedOrArchived),
        "done" => Some(Prerequisite::Archived),
        _ => None,
    }
}

/// Terminal evidence SpecSync reports for an archive that authenticates.
const ARCHIVE_VALIDITY: [&str; 3] = ["authenticated-history", "exact", "successor-covered"];

fn refused(label: String, detail: String) -> Gate {
    Gate {
        allowed: false,
        label,
        detail,
    }
}

/// The gate for reaching `destination`, or None when that stage has no prerequisite.
/// Every command that changes the main stage uses it, so each route meets the same rule.
pub(super) fn destination_gate(
    root: &Path,
    session: &Checkpoint,
    destination: &str,
) -> Result<Option<Gate>> {
    if prerequisite(destination).is_none() {
        return Ok(None);
    }
    let state = native_state(root, session.native_change.as_deref())?;
    Ok(gate_for_state(root, session, destination, state))
}

/// The gate for reaching `destination` given native state already read.
pub(super) fn gate_for_state(
    root: &Path,
    session: &Checkpoint,
    destination: &str,
    state: NativeState,
) -> Option<Gate> {
    let need = prerequisite(destination)?;
    let change = session.native_change.as_deref();
    let purpose = format!("Reaching {destination}");
    let status = status_command(change);
    Some(match state {
        NativeState::NotConfigured => Gate {
            allowed: true,
            label: "ungated:not-configured".into(),
            detail: "No native SpecSync configuration; this transition is not gated by a native lifecycle.".into(),
        },
        NativeState::Disabled => Gate {
            allowed: true,
            label: "ungated:sdd-disabled".into(),
            detail: "Native SDD is disabled or not adopted; this transition is not gated by a native lifecycle.".into(),
        },
        NativeState::PolicyInvalid(reason) => refused(
            "refused:sdd-policy-invalid".into(),
            format!(
                "{purpose} cannot be checked, so it is refused: {reason}. SpecSync rejects this policy too (see `specsync change audit`); fix the file, or set \"enabled\": false in a valid policy to disable native gates deliberately."
            ),
        ),
        NativeState::NoChange => refused(
            "refused:no-native-change".into(),
            format!(
                "{purpose} needs the native change: record it with `chough checkpoint --native-change <id>`."
            ),
        ),
        NativeState::Unknown(reason) => refused(
            "refused:native-unknown".into(),
            format!("{purpose} cannot be checked, so it is refused. {reason}. Run `{status}`."),
        ),
        NativeState::State(state, value) => native_gate(
            root,
            change.unwrap_or_default(),
            &purpose,
            need,
            &state,
            &value,
            &status,
        ),
    })
}

fn native_gate(
    root: &Path,
    change: &str,
    purpose: &str,
    need: Prerequisite,
    state: &str,
    value: &Value,
    status: &str,
) -> Gate {
    let next = value
        .pointer("/summary/next_action")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let archived = state == "archived";
    if need == Prerequisite::Archived || (need == Prerequisite::ApprovedOrArchived && archived) {
        if !archived {
            return refused(
                format!("refused:native-{state}"),
                format!(
                    "{purpose} needs native state archived; it is {state}. Native next action: {next}. Run `{status}`."
                ),
            );
        }
        let validity = value
            .pointer("/summary/terminal_evidence/validity")
            .and_then(Value::as_str);
        return match validity {
            Some(validity) if ARCHIVE_VALIDITY.contains(&validity) => Gate {
                allowed: true,
                label: "native:archived".into(),
                detail: format!("Native state is archived; its terminal evidence is {validity}."),
            },
            other => refused(
                format!("refused:native-archive-{}", other.unwrap_or("unreported")),
                format!(
                    "{purpose} needs an archive whose terminal evidence authenticates; summary.terminal_evidence.validity is {}. Native next action: {next}. Run `{status}`.",
                    other.unwrap_or("not reported")
                ),
            ),
        };
    }
    if !BUILD_STATES.contains(&state) {
        let wanted = if need == Prerequisite::ApprovedOrArchived {
            "approved, implementing, verifying or archived"
        } else {
            "approved, implementing or verifying"
        };
        return refused(
            format!("refused:native-{state}"),
            format!(
                "{purpose} needs native state {wanted}; it is {state}. Native next action: {next}. Run `{status}`."
            ),
        );
    }
    let approval = value
        .pointer("/summary/approval_valid")
        .and_then(Value::as_bool);
    if approval != Some(true) {
        return refused(
            "refused:native-approval-invalid".into(),
            format!(
                "{purpose} needs a valid definition approval; native status reports approval_valid {}. Native next action: {next}. Run `{status}`.",
                approval.map_or("missing".to_string(), |valid| valid.to_string())
            ),
        );
    }
    if value
        .pointer("/summary/correction_valid")
        .and_then(Value::as_bool)
        == Some(false)
    {
        return refused(
            "refused:native-correction-invalid".into(),
            format!(
                "{purpose} needs valid native corrections; native status reports correction_valid false. Native next action: {next}. Run `{status}`."
            ),
        );
    }
    let deltas = match approved_deltas(root, change, value) {
        Deltas::Unchanged(modules) => {
            format!("approved deltas unchanged ({})", modules.join(", "))
        }
        Deltas::NoSpec => "no semantic deltas (no_spec_change)".into(),
        Deltas::Changed(modules) => {
            return refused(
                "refused:native-delta-changed".into(),
                format!(
                    "{purpose} needs the approved semantic deltas; the delta for {} changed after the definition approval, while native status still reports approval_valid true (SpecSync 6.1.1, BUG-0002). Restore the approved delta, or ask the human for a new scope approval (specsync change approve {change} --actor \"<their name>\") and then run specsync change check {change}. Digests use SpecSync 6.1.1's {DELTA_DOMAIN} framing.",
                    modules.join(", ")
                ),
            );
        }
        Deltas::Unverifiable(reason) => {
            return refused(
                "refused:native-delta-unverifiable".into(),
                format!(
                    "{purpose} needs the approved semantic deltas, which cannot be compared: {reason}. Native next action: run {status}; Chough cannot confirm that the approved deltas are unchanged."
                ),
            );
        }
        Deltas::Unrecorded(reason) => {
            return refused(
                "refused:native-delta-unverifiable".into(),
                format!(
                    "{purpose} needs the approved semantic deltas, which cannot be compared: {reason}. Native next action: ask the human for a new scope approval (specsync change approve {change} --actor \"<their name>\"), which records the digests, and then run specsync change check {change}; a status read cannot clear this."
                ),
            );
        }
    };
    Gate {
        allowed: true,
        label: format!("native:{state}"),
        detail: format!("Native state is {state} with a valid definition approval; {deltas}."),
    }
}

/// SpecSync 6.1.1's domain for approved-delta digests.
const DELTA_DOMAIN: &str = "specsync.approved-delta.v1";

/// Whether the semantic deltas are the ones the latest definition approval recorded.
pub(super) enum Deltas {
    Unchanged(Vec<String>),
    NoSpec,
    /// Each module with what changed.
    Changed(Vec<String>),
    Unverifiable(String),
    /// The latest definition approval records no digests; only a new approval can.
    Unrecorded(String),
}

/// SpecSync 6.1.1's `FramedDigest` of one delta: SHA-256 over the length-framed domain,
/// module name and body, the body with CRLF normalized to LF.
fn delta_digest(module: &str, body: &[u8]) -> String {
    let mut text = Vec::with_capacity(body.len());
    let mut bytes = body.iter().peekable();
    while let Some(&byte) = bytes.next() {
        if !(byte == b'\r' && bytes.peek() == Some(&&b'\n')) {
            text.push(byte);
        }
    }
    let mut hash = Sha256::new();
    for (tag, value) in [
        ("domain", DELTA_DOMAIN.as_bytes()),
        ("module", module.as_bytes()),
        ("body", text.as_slice()),
    ] {
        hash.update((tag.len() as u64).to_be_bytes());
        hash.update(tag.as_bytes());
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
    }
    format!("{:x}", hash.finalize())
}

/// A module name SpecSync 6.1.1 accepts (`validate_module_name`): one plain path
/// segment with no separator, Windows-invalid or control character, no surrounding
/// space or trailing dot, no reserved device basename, and room for `.spec.md` in a
/// 255-byte component. Only such a name is read as `deltas/<module>.md`.
fn module_name(name: &str) -> bool {
    let mut components = Path::new(name).components();
    let segment = matches!(components.next(), Some(std::path::Component::Normal(_)))
        && components.next().is_none();
    let basename = name
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    let device = matches!(basename.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || (basename.len() == 4
            && (basename.starts_with("COM") || basename.starts_with("LPT"))
            && matches!(basename.as_bytes()[3], b'1'..=b'9'));
    segment
        && !name.chars().any(|character| {
            character.is_control()
                || matches!(
                    character,
                    '/' | '\\' | '<' | '>' | ':' | '"' | '|' | '?' | '*'
                )
        })
        && name == name.trim()
        && !name.ends_with('.')
        && name.len() + ".spec.md".len() <= 255
        && !device
}

/// Read a native change file read-only within SpecSync's bound: `Ok(None)` when it is
/// missing, an error message when it cannot be read.
fn native_file(root: &Path, relative: &str) -> std::result::Result<Option<Vec<u8>>, String> {
    let path = local_path(root, relative).map_err(|error| format!("{relative}: {error:#}"))?;
    match read_native_record(&path, NATIVE_RECORD_LIMIT) {
        Ok(Some(bytes)) => Ok(Some(bytes)),
        Ok(None) => Err(format!(
            "{relative} is over the {NATIVE_RECORD_LIMIT}-byte bound"
        )),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(format!("{relative} cannot be read: {error}")),
    }
}

/// Compare the deltas of `change` with the digests its latest definition approval
/// recorded, as SpecSync 6.1.1 does before verification (REQ-cli-022). It reads the
/// native approval ledger and deltas only, starts no process and copies nothing.
pub(super) fn approved_deltas(root: &Path, change: &str, value: &Value) -> Deltas {
    let directory = format!(".specsync/changes/{change}");
    let ledger_path = format!("{directory}/approvals.json");
    let ledger = match native_file(root, &ledger_path) {
        Ok(Some(bytes)) => bytes,
        Ok(None) => {
            return Deltas::Unverifiable(format!("the approval ledger {ledger_path} is missing"));
        }
        Err(reason) => return Deltas::Unverifiable(reason),
    };
    let malformed =
        || Deltas::Unverifiable(format!("the approval ledger {ledger_path} is malformed"));
    let Ok(ledger) = serde_json::from_slice::<Value>(&ledger) else {
        return malformed();
    };
    let Some(approvals) = ledger.get("approvals").and_then(Value::as_array) else {
        return malformed();
    };
    let Some(approval) = approvals
        .iter()
        .rev()
        .find(|approval| approval.get("gate").and_then(Value::as_str) == Some("definition"))
    else {
        return Deltas::Unverifiable(format!("{ledger_path} has no definition approval"));
    };
    let approved = match approval.get("approved_delta_digests") {
        None | Some(Value::Null) => None,
        Some(Value::Object(map)) => {
            let mut digests = std::collections::BTreeMap::new();
            for (module, digest) in map {
                match digest.as_str() {
                    Some(digest)
                        if module_name(module)
                            && is_digest(digest)
                            && !digest.bytes().any(|byte| byte.is_ascii_uppercase()) =>
                    {
                        digests.insert(module.clone(), digest.to_string());
                    }
                    _ => return malformed(),
                }
            }
            Some(digests)
        }
        Some(_) => return malformed(),
    };
    let Some(affected) = value
        .pointer("/change/affected_specs")
        .and_then(Value::as_array)
    else {
        return Deltas::Unverifiable("native status reports no change.affected_specs".into());
    };
    let mut modules = BTreeSet::new();
    for module in affected {
        match module.as_str().filter(|module| module_name(module)) {
            Some(module) => modules.insert(module.to_string()),
            None => {
                return Deltas::Unverifiable(format!(
                    "native status reports an invalid module name {module} in change.affected_specs"
                ));
            }
        };
    }
    let no_spec = value
        .pointer("/change/no_spec_change")
        .and_then(Value::as_bool)
        == Some(true);
    if let Some(approved) = &approved {
        modules.extend(approved.keys().cloned());
    }
    let mut bodies = vec![];
    for module in &modules {
        match native_file(root, &format!("{directory}/deltas/{module}.md")) {
            Ok(body) => bodies.push((module.clone(), body)),
            Err(reason) => return Deltas::Unverifiable(reason),
        }
    }
    let no_deltas = bodies.iter().all(|(_, body)| body.is_none());
    let Some(approved) = approved else {
        return if no_spec && no_deltas {
            Deltas::NoSpec
        } else {
            Deltas::Unrecorded(format!(
                "the definition approval in {ledger_path} records no approved_delta_digests"
            ))
        };
    };
    if no_spec && no_deltas && approved.is_empty() {
        return Deltas::NoSpec;
    }
    let mut changed = vec![];
    let mut unchanged = vec![];
    for (module, body) in bodies {
        match (approved.get(&module), body) {
            (Some(digest), Some(body)) if *digest == delta_digest(&module, &body) => {
                unchanged.push(module)
            }
            (Some(_), Some(_)) => changed.push(format!("{module} (edited)")),
            (Some(_), None) => changed.push(format!("{module} (deleted)")),
            (None, Some(_)) => changed.push(format!("{module} (not in the approval)")),
            (None, None) if no_spec => {}
            (None, None) => changed.push(format!("{module} (no approved delta)")),
        }
    }
    if changed.is_empty() {
        if unchanged.is_empty() {
            unchanged.push("no modules".into());
        }
        Deltas::Unchanged(unchanged)
    } else {
        Deltas::Changed(changed)
    }
}

/// The approved-delta result for a report, or why it was not checked.
pub(super) fn delta_summary(root: &Path, change: Option<&str>, state: &NativeState) -> String {
    let (Some(change), NativeState::State(state, value)) = (change, state) else {
        return match state {
            NativeState::State(..) | NativeState::NoChange => {
                "not checked: no native change recorded".into()
            }
            NativeState::Unknown(_) => "not checked: native state unknown".into(),
            NativeState::NotConfigured | NativeState::Disabled => {
                "not checked: no native lifecycle".into()
            }
            NativeState::PolicyInvalid(_) => "not checked: SDD policy invalid".into(),
        };
    };
    if !BUILD_STATES.contains(&state.as_str()) {
        return format!("not checked: native state {state}");
    }
    if value
        .pointer("/summary/approval_valid")
        .and_then(Value::as_bool)
        != Some(true)
    {
        return "not checked: approval_valid is not true".into();
    }
    match approved_deltas(root, change, value) {
        Deltas::Unchanged(modules) => format!("unchanged: {}", modules.join(", ")),
        Deltas::NoSpec => "no semantic deltas (no_spec_change)".into(),
        Deltas::Changed(modules) => format!("changed: {}", modules.join(", ")),
        Deltas::Unverifiable(reason) | Deltas::Unrecorded(reason) => {
            format!("unverifiable: {reason}")
        }
    }
}

/// The main stage a forward move from `current` reaches, if any.
pub(super) fn next_main(current: &str) -> Option<&'static str> {
    if current == "release" {
        return Some("done");
    }
    ORDER.get(position(current)? + 1).copied()
}

/// The next valid action with its reason and, when one applies, the command to run.
pub(super) struct Action {
    pub(super) action: String,
    pub(super) reason: String,
    pub(super) command: Option<String>,
}

const NEW_CYCLE_RESET: &str = "chough stage reset --to intent --reason \"<why>\"";
/// The new-cycle steps, shared by the done stage and an archive left behind at an
/// earlier main stage, so the wording cannot drift.
const NEW_CYCLE_STEPS: &str = "Start a new cycle: chough stage reset --to intent --reason \"<why>\" (or --to plan when the approved intent already covers the new work)";
const NEW_CYCLE_CHANGE: &str = "specsync change new \"<outcome>\" --spec <module> --path <path>";
const NEW_CYCLE_RECORD: &str =
    "chough checkpoint --native-change <id> --next-action \"<next step>\"";

/// The full new cycle after a native change: reset, a new native change, record it.
fn new_cycle() -> String {
    format!("{NEW_CYCLE_STEPS}, then {NEW_CYCLE_CHANGE}, then {NEW_CYCLE_RECORD}.")
}

/// The terminal evidence validity a native status response reports.
fn terminal_validity(state: &NativeState) -> &str {
    match state {
        NativeState::State(_, value) => value
            .pointer("/summary/terminal_evidence/validity")
            .and_then(Value::as_str)
            .unwrap_or("not reported"),
        _ => "not reported",
    }
}

/// Whether the recorded native change is archived with terminal evidence that the done
/// gate accepts, the same test `done_action` uses.
fn archived_and_authenticated(root: &Path, session: &Checkpoint, state: &NativeState) -> bool {
    matches!(state, NativeState::State(name, _) if name == "archived")
        && gate_for_state(root, session, "done", state.clone()).is_some_and(|gate| gate.allowed)
}

/// The gate for the next main stage and the next valid action, both evaluated on one
/// native state, so `next` and `status` report the same decision (REQ-cli-021).
pub(super) fn assess(
    root: &Path,
    session: &Checkpoint,
    state: &NativeState,
    blocking: &[String],
) -> (Option<(String, Gate)>, Action) {
    let current = session.stage.current.as_str();
    let gate = next_main(current).and_then(|to| {
        gate_for_state(root, session, to, state.clone()).map(|gate| (to.to_string(), gate))
    });
    let native_next = match state {
        NativeState::State(_, value) => value
            .pointer("/summary/next_action")
            .and_then(Value::as_str)
            .filter(|text| !text.trim().is_empty()),
        _ => None,
    };
    let action = if session.stage.status == "paused" {
        Action {
            action: format!(
                "The {current} stage is paused. Check native state and freshness, then `chough stage resume`."
            ),
            reason: "A paused stage keeps its scope, preferences and gates; native state may have changed meanwhile.".into(),
            command: Some("chough stage resume".into()),
        }
    } else if !session.awaiting_human.is_empty() {
        Action {
            action: format!(
                "Waiting for a human decision: {}. Do not record it on their behalf; continue only independent work within scope.",
                session.awaiting_human.join("; ")
            ),
            reason: "Only a person can close these gates.".into(),
            command: None,
        }
    } else if !blocking.is_empty() && matches!(current, "test" | "accept") {
        Action {
            action: format!(
                "Acceptance-blocking bugs are open ({}); go back to build and retest the affected cases.",
                blocking.join(", ")
            ),
            reason: "Acceptance waits for every blocking bug to be verified or rescoped.".into(),
            command: Some("chough stage back --to build --reason \"<why>\"".into()),
        }
    } else if current == "done" {
        done_action(root, session, state, native_next)
    } else if is_main(current) && archived_and_authenticated(root, session, state) {
        archived_action(session, state, native_next)
    } else if let Some((to, gate)) = gate.as_ref().filter(|(_, gate)| !gate.allowed) {
        Action {
            action: format!("Gate not open ({}): {}", gate.label, gate.detail),
            reason: format!("Native state does not allow reaching {to} yet."),
            command: Some(status_command(session.native_change.as_deref())),
        }
    } else {
        Action {
            action: format!("Continue {current}: {}", session.next_action),
            reason: "No gate, human decision or blocking bug stops the current stage; this is the checkpoint's next action.".into(),
            command: None,
        }
    };
    (gate, action)
}

/// In stage done: a closed cycle recommends a new one; an archive that does not
/// authenticate keeps its gate refusal.
fn done_action(
    root: &Path,
    session: &Checkpoint,
    state: &NativeState,
    native_next: Option<&str>,
) -> Action {
    let check_first = native_next
        .map(|text| format!(" Native next action, check it first: {text}."))
        .unwrap_or_default();
    let closed = |action: String, reason: String| Action {
        action,
        reason: format!("{reason}{check_first}"),
        command: Some(NEW_CYCLE_RESET.into()),
    };
    let done = gate_for_state(root, session, "done", state.clone());
    match (state, done) {
        (NativeState::NotConfigured | NativeState::Disabled, _) => closed(
            format!("{NEW_CYCLE_STEPS}, then chough checkpoint --next-action \"<next step>\"."),
            "The cycle is closed; no native lifecycle gates it.".into(),
        ),
        (NativeState::NoChange, _) => closed(
            new_cycle(),
            "No native change is recorded for this cycle, so it is closed.".into(),
        ),
        (NativeState::State(..), Some(gate)) if gate.allowed => closed(
            new_cycle(),
            format!(
                "The cycle for {} is closed: native state archived, terminal evidence {}.",
                session
                    .native_change
                    .as_deref()
                    .unwrap_or("the native change"),
                terminal_validity(state)
            ),
        ),
        (_, Some(gate)) => Action {
            action: format!("Gate not open ({}): {}", gate.label, gate.detail),
            reason: "Stage done needs an archived change whose terminal evidence authenticates."
                .into(),
            command: Some(status_command(session.native_change.as_deref())),
        },
        (_, None) => Action {
            action: format!("Continue done: {}", session.next_action),
            reason: "Stage done has no native prerequisite here.".into(),
            command: None,
        },
    }
}

/// At intent to accept, a recorded native change that is archived with authenticated
/// terminal evidence closes this cycle's work: name the archive and the new cycle, after
/// recording done first at accept (accept.md's documented close after finalize).
fn archived_action(session: &Checkpoint, state: &NativeState, native_next: Option<&str>) -> Action {
    let current = session.stage.current.as_str();
    let change = session
        .native_change
        .as_deref()
        .unwrap_or("the native change");
    let at_accept = current == "accept";
    let record_done = if at_accept {
        "Record done first: chough stage advance (accept to done is allowed). Then: "
    } else {
        ""
    };
    let check_first = native_next
        .map(|text| format!(" Native next action, check it first: {text}."))
        .unwrap_or_default();
    Action {
        action: format!(
            "Native change {change} is archived while the checkpoint is at {current}, so this cycle's work is closed. {record_done}{}",
            new_cycle()
        ),
        reason: format!(
            "The archive's terminal evidence is {}; continuing {current} would work on a closed change.{check_first}",
            terminal_validity(state)
        ),
        command: Some(if at_accept {
            "chough stage advance".into()
        } else {
            NEW_CYCLE_RESET.into()
        }),
    }
}

fn refresh_note(stage: &str) -> &'static str {
    match stage {
        "intent" => "intent: HI criteria and FRD; append new criteria, never rewrite approved ones",
        "design" => {
            "design: canonical prototype and companion; their human approval covers the old version"
        }
        "plan" => "plan: native definition; scope or contract changes need native reapproval",
        "build" => "build: implementation against the current definition",
        "test" => "test: rerun affected cases; earlier observations may be stale",
        "accept" => "accept: native verification and the human implementation review",
        _ => "done: closeout",
    }
}

fn log_transition(
    root: &Path,
    kind: &str,
    from: &str,
    to: &str,
    reason: Option<&str>,
    gate: Option<&str>,
    refresh: Vec<String>,
) -> Result<()> {
    let transition = Transition {
        schema_version: 1,
        at_unix_seconds: now()?,
        kind: kind.into(),
        from: from.into(),
        to: to.into(),
        reason: reason.map(str::to_string),
        gate: gate.map(str::to_string),
        revision: head_revision(root),
        refresh,
    };
    append_line(root, TRANSITIONS, &serde_json::to_string(&transition)?)
}

/// Transition kinds that end the open stage segment and start the next one.
pub(super) const STAGE_KINDS: [&str; 9] = [
    "advance", "iterate", "back", "reset", "skip", "pause", "resume", "enter", "return",
];
/// A human gate was added to `awaiting_human`; it starts or ends no stage segment.
pub(super) const GATE_OPENED: &str = "gate-opened";
/// A human gate was cleared from `awaiting_human`.
pub(super) const GATE_CLOSED: &str = "gate-closed";

/// Append a gate record: when a gate opened or was cleared, in the current stage, with
/// the gate text as its reason. It never records who decided or what was decided.
pub(super) fn log_gate(root: &Path, kind: &str, stage: &str, gate: &str) -> Result<()> {
    ensure!(
        kind == GATE_OPENED || kind == GATE_CLOSED,
        "not a gate record kind: {kind}"
    );
    log_transition(root, kind, stage, stage, Some(gate), None, vec![])
}

pub(super) fn recent_transitions(root: &Path, count: usize) -> Result<(Vec<Value>, bool)> {
    let (lines, partial) = read_log_tail(&local_path(root, TRANSITIONS)?, count)?;
    let mut values = vec![];
    for line in lines {
        let transition: Transition = serde_json::from_str(&line)
            .map_err(|error| anyhow::anyhow!("invalid transition record: {error}"))?;
        values.push(serde_json::to_value(transition)?);
    }
    Ok((values, partial))
}

/// The transitions a fresh agent may use now, as commands.
pub(super) fn valid_transitions(session: &Checkpoint) -> Vec<String> {
    let current = session.stage.current.as_str();
    if session.stage.status == "paused" {
        return vec!["chough stage resume".into()];
    }
    let mut commands = vec![];
    if is_side(current) {
        commands.push(format!(
            "chough stage return  (back to {})",
            session
                .stage
                .callers
                .last()
                .map_or("unknown", |caller| caller.stage.as_str())
        ));
    } else if current == "release" {
        commands.push(
            "chough stage advance  (release -> done)  [native gate: archived with authenticated terminal evidence]"
                .into(),
        );
    } else if let Some(index) = position(current) {
        if let Some(next) = ORDER.get(index + 1) {
            let gate = match (current, *next) {
                (_, "build") => "  [native gate: valid definition approval]",
                (_, "test" | "accept") => {
                    "  [native gate: valid approval or authenticated archive]"
                }
                ("accept", _) => "  [native gate: archived with authenticated terminal evidence]",
                _ => "",
            };
            commands.push(format!("chough stage advance  ({current} -> {next}){gate}"));
            commands.push("chough stage skip --reason <why not applicable>".into());
        } else {
            commands.push(
                "chough stage enter release --reason <request>  [needs release consent or standing preference]"
                    .into(),
            );
        }
        commands.push("chough stage iterate --note <what changed>".into());
        if index > 0 {
            commands.push(format!(
                "chough stage back --to <{}> --reason <why>",
                ORDER[..index].join("|")
            ));
        }
    }
    commands.push("chough stage reset [--to <stage>] --reason <why>".into());
    commands.push("chough stage pause --reason <why>".into());
    if session.stage.callers.len() < 4 {
        commands.push(
            "chough stage enter <bugs|validate|librarian> --reason <why>  [validate needs consent or preference]"
                .into(),
        );
    }
    commands
}

fn report(root: &Path, session: &Checkpoint, applied: Option<Value>) -> Result<()> {
    let (transitions, partial) = recent_transitions(root, 10)?;
    print_json(&json!({
        "schema_version": 1,
        "applied": applied,
        "stage": session.stage,
        "native_change": session.native_change,
        "awaiting_human": session.awaiting_human,
        "valid_transitions": valid_transitions(session),
        "stages": STAGES.iter().map(|(name, kind)| json!({"name": name, "kind": kind})).collect::<Vec<_>>(),
        "recent_transitions": transitions,
        "transitions_partial": partial,
        "authority": "Native SpecSync status decides lifecycle gates; Chough never records human approval or review.",
    }))
}

fn ensure_active(session: &Checkpoint) -> Result<()> {
    ensure!(
        session.stage.status == "active",
        "the stage is paused; run `chough stage resume` first"
    );
    Ok(())
}

fn ensure_main(session: &Checkpoint, verb: &str) -> Result<()> {
    ensure!(
        !is_side(&session.stage.current),
        "cannot {verb} from side stage {}; run `chough stage return` first",
        session.stage.current
    );
    Ok(())
}

/// Apply a move to the next stage, checking the gates that apply to it.
fn move_forward(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    reason: Option<&str>,
) -> Result<Value> {
    ensure_active(session)?;
    ensure_main(session, kind)?;
    let from = session.stage.current.clone();
    let to = if from == "release" {
        "done".to_string()
    } else {
        let index = position(&from).ok_or_else(|| anyhow::anyhow!("unknown stage {from}"))?;
        match ORDER.get(index + 1) {
            Some(next) => (*next).to_string(),
            None => bail!(
                "done is terminal; release is entered only with `chough stage enter release` when authorized"
            ),
        }
    };
    let mut gate_label = None;
    if from == "test" && to == "accept" {
        let blocking = bugs::open_blocking(root)?;
        ensure!(
            blocking.is_empty(),
            "{kind} refused (refused:blocking-bugs): open acceptance-blocking bugs {}; go back to build, fix and retest",
            blocking.join(", ")
        );
    }
    if let Some(gate) = destination_gate(root, session, &to)? {
        ensure!(
            gate.allowed,
            "{} refused ({}): {}",
            kind,
            gate.label,
            gate.detail
        );
        gate_label = Some(gate.label);
    }
    log_transition(
        root,
        kind,
        &from,
        &to,
        reason,
        gate_label.as_deref(),
        vec![],
    )?;
    session.stage.current = to.clone();
    session.stage.iteration = 1;
    Ok(json!({"kind": kind, "from": from, "to": to, "gate": gate_label}))
}

fn move_back(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    target: &str,
    reason: &str,
) -> Result<Value> {
    ensure_text("reason", reason)?;
    ensure!(is_main(target), "{target} is not a main stage");
    let from = session.stage.current.clone();
    let origin = session
        .stage
        .callers
        .first()
        .map_or(from.as_str(), |caller| caller.stage.as_str())
        .to_string();
    let origin_index = position(&origin).unwrap_or(ORDER.len() - 1);
    let target_index = position(target).unwrap_or(0);
    if kind == "back" {
        ensure_active(session)?;
        ensure_main(session, "go back")?;
        ensure!(
            target_index < origin_index,
            "back goes to an earlier stage than {from}"
        );
    }
    let mut gate_label = None;
    if let Some(gate) = destination_gate(root, session, target)? {
        ensure!(
            gate.allowed,
            "{kind} refused ({}): {}",
            gate.label,
            gate.detail
        );
        gate_label = Some(gate.label);
    }
    let mut refresh: Vec<String> = ORDER[target_index..=origin_index.max(target_index)]
        .iter()
        .filter(|stage| **stage != "done")
        .map(|stage| refresh_note(stage).to_string())
        .collect();
    if !session.references.cases.is_empty() || !session.observations.is_empty() {
        refresh.push(format!(
            "checkpoint references: {} case(s) and {} observation(s) to re-check with `chough status`",
            session.references.cases.len(),
            session.observations.len()
        ));
    }
    if target_index <= 2
        && let NativeState::State(state, _) = native_state(root, session.native_change.as_deref())?
        && state != "draft"
    {
        refresh.push(format!(
            "native: the definition is {state}; changing intent, contract, acceptance criteria or scope needs native reapproval. Follow `{}`; Chough does not change native state.",
            status_command(session.native_change.as_deref())
        ));
    }
    log_transition(
        root,
        kind,
        &from,
        target,
        Some(reason),
        gate_label.as_deref(),
        refresh.clone(),
    )?;
    if kind == "reset" {
        // A reset starts a new cycle, so the old next action no longer applies. The
        // reason is kept on one line and cut so the next action stays within 4096 bytes;
        // the transition keeps it whole.
        let head = format!("New cycle from {from} after reset: ");
        let tail = ". Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action.";
        let room = 4_096 - head.len() - tail.len();
        let mut line = String::new();
        for character in reason
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
        {
            if line.len() + character.len_utf8() > room {
                break;
            }
            line.push(character);
        }
        session.next_action = format!("{head}{line}{tail}");
    }
    session.stage.current = target.into();
    session.stage.callers.clear();
    session.stage.status = "active".into();
    session.stage.iteration = 1;
    Ok(json!({"kind": kind, "from": from, "to": target, "refresh": refresh}))
}

pub(super) fn run(root: &Path, command: StageCommand) -> Result<()> {
    let mut session = read_checkpoint(root)?;
    let applied = match command {
        StageCommand::Show => None,
        StageCommand::Advance { note } => {
            if let Some(note) = &note {
                ensure_text("note", note)?;
            }
            Some(move_forward(
                root,
                &mut session,
                "advance",
                note.as_deref(),
            )?)
        }
        StageCommand::Skip { reason } => {
            ensure_text("reason", &reason)?;
            Some(move_forward(root, &mut session, "skip", Some(&reason))?)
        }
        StageCommand::Iterate { note } => {
            ensure_text("note", &note)?;
            ensure_active(&session)?;
            let stage = session.stage.current.clone();
            log_transition(root, "iterate", &stage, &stage, Some(&note), None, vec![])?;
            session.stage.iteration += 1;
            Some(json!({"kind": "iterate", "stage": stage, "iteration": session.stage.iteration}))
        }
        StageCommand::Back { to, reason } => {
            Some(move_back(root, &mut session, "back", &to, &reason)?)
        }
        StageCommand::Reset { to, reason } => {
            Some(move_back(root, &mut session, "reset", &to, &reason)?)
        }
        StageCommand::Pause { reason } => {
            ensure_text("reason", &reason)?;
            ensure_active(&session)?;
            let stage = session.stage.current.clone();
            log_transition(root, "pause", &stage, &stage, Some(&reason), None, vec![])?;
            session.stage.status = "paused".into();
            Some(json!({"kind": "pause", "stage": stage}))
        }
        StageCommand::Resume => {
            ensure!(session.stage.status == "paused", "the stage is not paused");
            let stage = session.stage.current.clone();
            log_transition(root, "resume", &stage, &stage, None, None, vec![])?;
            session.stage.status = "active".into();
            Some(json!({
                "kind": "resume",
                "stage": stage,
                "check": "Run `chough next --environment <env>` to re-check native state and evidence freshness before continuing."
            }))
        }
        StageCommand::Enter { stage, reason } => {
            ensure_text("reason", &reason)?;
            ensure_active(&session)?;
            let from = session.stage.current.clone();
            if stage == "release" {
                ensure!(
                    from == "done",
                    "release is entered from done, after native archive and merge"
                );
                let authority = prefs::authorized(root, &session, "release")?.ok_or_else(|| {
                    anyhow::anyhow!(
                        "release is not authorized: it needs a request from the human recorded with `chough consent grant release --source <their words>` or a standing `chough pref set release standing --source <...>`"
                    )
                })?;
                log_transition(
                    root,
                    "enter",
                    &from,
                    "release",
                    Some(&reason),
                    Some(&authority),
                    vec![],
                )?;
                session.stage.current = "release".into();
                session.stage.iteration = 1;
                Some(
                    json!({"kind": "enter", "from": from, "to": "release", "authority": authority}),
                )
            } else {
                ensure!(is_side(&stage), "{stage} is not a side stage");
                ensure!(from != stage, "already in {stage}");
                ensure!(
                    session.stage.callers.len() < 4,
                    "side stages nest at most 4 deep"
                );
                let mut authority = None;
                if stage == "validate" {
                    authority = prefs::authorized(root, &session, "repeat-validation")?
                        .or(prefs::authorized(root, &session, "monitoring")?);
                    ensure!(
                        authority.is_some(),
                        "validate is optional: it needs a run consent or a standing preference for repeat-validation or monitoring"
                    );
                }
                log_transition(
                    root,
                    "enter",
                    &from,
                    &stage,
                    Some(&reason),
                    authority.as_deref(),
                    vec![],
                )?;
                session.stage.callers.push(Caller {
                    stage: from.clone(),
                    reason,
                });
                session.stage.current = stage.clone();
                session.stage.iteration = 1;
                Some(json!({"kind": "enter", "from": from, "to": stage}))
            }
        }
        StageCommand::Return { note } => {
            if let Some(note) = &note {
                ensure_text("note", note)?;
            }
            ensure_active(&session)?;
            let caller = session.stage.callers.last().cloned().ok_or_else(|| {
                anyhow::anyhow!(
                    "{} is not a side stage with a caller",
                    session.stage.current
                )
            })?;
            let mut gate_label = None;
            if let Some(gate) = destination_gate(root, &session, &caller.stage)? {
                ensure!(
                    gate.allowed,
                    "return refused ({}): {}; `chough stage reset --to <stage> --reason \"<why>\"` moves elsewhere",
                    gate.label,
                    gate.detail
                );
                gate_label = Some(gate.label);
            }
            session.stage.callers.pop();
            let from = session.stage.current.clone();
            log_transition(
                root,
                "return",
                &from,
                &caller.stage,
                note.as_deref(),
                gate_label.as_deref(),
                vec![],
            )?;
            session.stage.current = caller.stage.clone();
            session.stage.iteration = 1;
            Some(json!({"kind": "return", "from": from, "to": caller.stage}))
        }
    };
    if applied.is_some() {
        save_checkpoint(root, &session)?;
    }
    report(root, &session, applied)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DELTA: &str = "## ADDED\n\n### REQUIREMENT REQ-cli-090\n\nThe helper SHALL do it.\n";

    /// The guard's own digest matches every digest SpecSync recorded in this
    /// repository's archive, on every platform.
    #[test]
    fn product_digests_match_every_archived_approval() {
        let archive = Path::new(env!("CARGO_MANIFEST_DIR")).join(".specsync/archive/changes");
        let mut compared = 0;
        for entry in std::fs::read_dir(&archive).unwrap() {
            let directory = entry.unwrap().path();
            let ledger: Value =
                serde_json::from_slice(&std::fs::read(directory.join("approvals.json")).unwrap())
                    .unwrap();
            let approval = ledger["approvals"]
                .as_array()
                .unwrap()
                .iter()
                .rev()
                .find(|approval| approval["gate"] == "definition")
                .unwrap();
            for (module, digest) in approval["approved_delta_digests"].as_object().unwrap() {
                let body = std::fs::read(directory.join(format!("deltas/{module}.md"))).unwrap();
                assert_eq!(delta_digest(module, &body), digest.as_str().unwrap());
                compared += 1;
            }
        }
        assert!(compared >= 7, "only {compared} archived digests compared");
    }

    fn deltas(root: &Path, affected: &[&str], approval: Value) -> Deltas {
        let directory = root.join(".specsync/changes/guarded");
        std::fs::create_dir_all(directory.join("deltas")).unwrap();
        std::fs::write(
            directory.join("approvals.json"),
            json!({"approvals": [approval]}).to_string(),
        )
        .unwrap();
        let status = json!({"change": {"affected_specs": affected, "no_spec_change": false}});
        approved_deltas(root, "guarded", &status)
    }

    /// The guard's comparison, on every platform: matching deltas, including after a
    /// line-ending-only edit and for names SpecSync accepts, pass; edits, deletions and
    /// approvals without digests do not; a path-like module name is never read.
    #[test]
    fn approved_deltas_compare_on_every_platform() {
        let root = tempfile::tempdir().unwrap();
        let root = root.path();
        let delta =
            |module: &str| root.join(format!(".specsync/changes/guarded/deltas/{module}.md"));
        let modules = ["cli", "a.b", "Módulo"];
        let mut digests = serde_json::Map::new();
        for module in modules {
            digests.insert(module.into(), json!(delta_digest(module, DELTA.as_bytes())));
        }
        let approval = json!({"gate": "definition", "approved_delta_digests": digests});
        std::fs::create_dir_all(root.join(".specsync/changes/guarded/deltas")).unwrap();
        for module in modules {
            std::fs::write(delta(module), DELTA).unwrap();
        }
        assert!(matches!(
            deltas(root, &modules, approval.clone()),
            Deltas::Unchanged(found) if found.len() == 3
        ));
        std::fs::write(delta("a.b"), DELTA.replace('\n', "\r\n")).unwrap();
        assert!(matches!(
            deltas(root, &modules, approval.clone()),
            Deltas::Unchanged(_)
        ));
        std::fs::write(delta("a.b"), format!("{DELTA}More.\n")).unwrap();
        assert!(matches!(
            deltas(root, &modules, approval.clone()),
            Deltas::Changed(found) if found == ["a.b (edited)"]
        ));
        std::fs::remove_file(delta("a.b")).unwrap();
        assert!(matches!(
            deltas(root, &modules, approval.clone()),
            Deltas::Changed(found) if found == ["a.b (deleted)"]
        ));
        assert!(matches!(
            deltas(root, &["cli"], json!({"gate": "definition"})),
            Deltas::Unrecorded(_)
        ));
        for name in [
            "../cli",
            "a/b",
            "a\\b",
            ".",
            "..",
            "trailing.",
            " cli",
            "CON",
            "com1.x",
            "a:b",
        ] {
            assert!(!module_name(name), "{name}");
            assert!(matches!(
                deltas(root, &[name], approval.clone()),
                Deltas::Unverifiable(_)
            ));
        }
        for name in ["cli", "a.b", "Módulo", "v2", "auth-service", "user_profile"] {
            assert!(module_name(name), "{name}");
        }
        assert!(module_name(&"m".repeat(247)) && !module_name(&"m".repeat(248)));
    }
}
