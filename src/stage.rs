//! Stage routing. Native SpecSync status decides gate eligibility; this module only
//! records transitions and refuses those that native state does not allow.

use crate::checkpoint::{Caller, Checkpoint, read_checkpoint, save_checkpoint};
use crate::policy::{Policy, sdd_policy};
use crate::records::{
    NATIVE_RECORD_LIMIT, NativeJson, ensure_text, head_revision, is_digest, local_path,
    native_json, now, print_json, read_native_record,
};
use crate::{bugs, evidence, prefs};
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
        /// Move through each main stage up to this one on one native read; every step is
        /// checked before any transition is recorded (REQ-cli-052).
        #[arg(long)]
        to: Option<String>,
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
    /// Record a move to the next main stage that already happened, at its original
    /// time with the commit and quote it rests on. Every advance refusal applies and
    /// native state is read now; no human approval or review is recorded.
    Backfill {
        /// When the move happened: `YYYY-MM-DDTHH:MM:SSZ` or Unix seconds.
        #[arg(long)]
        at: String,
        /// The commit the move rests on; it must be an ancestor of HEAD.
        #[arg(long)]
        commit: String,
        /// The human's or the record's words, kept verbatim as evidence only.
        #[arg(long)]
        quote: String,
        #[arg(long)]
        note: Option<String>,
    },
}

/// Evidence for a transition recorded after it happened (REQ-history-005). The
/// transition's `at_unix_seconds` is the original time; this is evidence only.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Backfill {
    pub(super) recorded_at_unix_seconds: u64,
    pub(super) commit: String,
    pub(super) quote: String,
}

/// A human decision an agent recorded on someone's behalf (REQ-history-010): who is
/// said to have decided, their words verbatim, where and when they said it, and the
/// agent that recorded it. A relayed claim, not authenticated.
#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RelayedBy {
    pub(super) decided_by: String,
    pub(super) words: String,
    #[serde(rename = "where")]
    pub(super) place: String,
    pub(super) recorded_by: String,
    /// The digest of the one-screen summary the decision was given on, with the list
    /// it came from (REQ-history-014); both or neither.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) summary_digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) list: Option<String>,
}

/// Whether `value` is exactly `length` lowercase hex digits.
pub(super) fn lowercase_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

/// The bound of each relay value, the checkpoint text bound.
pub(super) const RELAY_TEXT_LIMIT: usize = 4_096;

impl RelayedBy {
    /// The four values with their flag names, in order.
    pub(super) fn fields(&self) -> [(&'static str, &str); 4] {
        [
            ("decided_by", &self.decided_by),
            ("words", &self.words),
            ("where", &self.place),
            ("recorded_by", &self.recorded_by),
        ]
    }
}

/// Refuse a transition whose relay is not a valid gate-closed relay (REQ-history-010).
pub(super) fn validate_relay(transition: &Transition) -> Result<()> {
    let Some(relay) = &transition.relayed_by else {
        return Ok(());
    };
    ensure!(
        transition.kind == GATE_CLOSED,
        "invalid transition payload: relayed_by is only allowed on gate-closed, not {}",
        transition.kind
    );
    for (key, value) in relay.fields() {
        ensure!(
            !value.trim().is_empty() && value.len() <= RELAY_TEXT_LIMIT,
            "invalid transition payload: relayed_by.{key} must be non-blank and at most {RELAY_TEXT_LIMIT} bytes"
        );
    }
    match (&relay.summary_digest, &relay.list) {
        (None, None) => {}
        (Some(digest), Some(list)) => ensure!(
            lowercase_hex(digest, 64) && lowercase_hex(list, 12),
            "invalid transition payload: relayed_by.summary_digest must be 64 and relayed_by.list 12 lowercase hex digits"
        ),
        _ => anyhow::bail!(
            "invalid transition payload: relayed_by.summary_digest and relayed_by.list go together"
        ),
    }
    Ok(())
}

/// A lifecycle record of a native change that changed outside Chough (REQ-history-013):
/// the change, each watched file added, changed or removed, and the change's state
/// before and after.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct DirectWrite {
    pub(super) change: String,
    pub(super) files: Vec<WrittenFile>,
    pub(super) before_state: Option<String>,
    pub(super) after_state: Option<String>,
}

/// One watched file of a direct write and how it changed.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub(super) struct WrittenFile {
    pub(super) path: String,
    pub(super) change: String,
}

/// The kind of a `direct-native-write` event (REQ-history-013).
pub(super) const DIRECT_WRITE: &str = "direct-native-write";

/// Refuse a transition whose direct write is invalid or on the wrong kind
/// (REQ-history-013).
pub(super) fn validate_direct_write(transition: &Transition) -> Result<()> {
    let Some(write) = &transition.direct_write else {
        ensure!(
            transition.kind != DIRECT_WRITE,
            "invalid transition payload: {DIRECT_WRITE} needs direct_write"
        );
        return Ok(());
    };
    ensure!(
        transition.kind == DIRECT_WRITE,
        "invalid transition payload: direct_write is only allowed on {DIRECT_WRITE}, not {}",
        transition.kind
    );
    ensure!(
        transition.from == transition.to,
        "invalid transition payload: a {DIRECT_WRITE} event stays in its stage"
    );
    ensure!(
        !write.change.trim().is_empty()
            && !write.files.is_empty()
            && write.files.iter().all(|file| {
                crate::direct_writes::WATCHED.contains(&file.path.as_str())
                    && matches!(file.change.as_str(), "added" | "changed" | "removed")
            }),
        "invalid transition payload: direct_write needs its change and each watched file with added, changed or removed"
    );
    Ok(())
}

/// Every check of a transition's optional records.
pub(super) fn validate_records(transition: &Transition) -> Result<()> {
    validate_relay(transition)?;
    validate_direct_write(transition)?;
    validate_gate_and_correction(transition)
}

/// `holds` and `drives` belong to gate-opened entries, and `corrects`, which a
/// correction needs, to corrections only.
fn validate_gate_and_correction(transition: &Transition) -> Result<()> {
    let gate_opened = transition.kind == GATE_OPENED;
    ensure!(
        gate_opened || (transition.holds.is_none() && transition.drives.is_none()),
        "invalid transition payload: holds and drives are only allowed on gate-opened, not {}",
        transition.kind
    );
    if let Some(holds) = &transition.holds {
        ensure!(
            is_stage(holds),
            "invalid transition payload: holds names no stage"
        );
    }
    if let Some(drives) = &transition.drives {
        ensure!(
            crate::humans::Drives::parse(drives).is_some(),
            "invalid transition payload: drives is approve, review or none"
        );
    }
    let correction = transition.kind == CORRECTION;
    match &transition.corrects {
        Some(target) => ensure!(
            correction
                && target
                    .strip_prefix('^')
                    .is_some_and(crate::session_md::valid_entry_id),
            "invalid transition payload: corrects is `^t-xxxxxx` and only allowed on a correction"
        ),
        None => ensure!(
            !correction,
            "invalid transition payload: a correction names the entry it corrects"
        ),
    }
    Ok(())
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Transition {
    pub(super) schema_version: u32,
    pub(super) at_unix_seconds: u64,
    pub(super) kind: String,
    pub(super) from: String,
    pub(super) to: String,
    #[serde(default)]
    pub(super) reason: Option<String>,
    #[serde(default)]
    pub(super) gate: Option<String>,
    #[serde(default)]
    pub(super) revision: Option<String>,
    #[serde(default)]
    pub(super) refresh: Vec<String>,
    /// Present only on a backfilled move, so other records keep their bytes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) backfill: Option<Backfill>,
    /// Present only on a gate-closed record of a relayed decision (REQ-history-010).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) relayed_by: Option<RelayedBy>,
    /// Present only on a `direct-native-write` event (REQ-history-013).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) direct_write: Option<DirectWrite>,
    /// The acting agent, when known (REQ-records-007): the entry's `by` field, an
    /// unauthenticated claim.
    #[serde(default, rename = "by", skip_serializing_if = "Option::is_none")]
    pub(super) actor: Option<crate::actor::Actor>,
    /// On a `gate-opened` entry, the stage the gate holds and the step it drives
    /// (REQ-cli-030).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) holds: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) drives: Option<String>,
    /// On a `correction` entry, the entry it corrects, as `^t-xxxxxx` (REQ-history-015).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) corrects: Option<String>,
}

/// An append-only correction of an earlier entry (REQ-history-015); it starts and ends
/// no stage segment.
pub(super) const CORRECTION: &str = "correction";

/// Whether `kind` is a Timeline entry kind (REQ-history-002).
pub(super) fn is_kind(kind: &str) -> bool {
    STAGE_KINDS.contains(&kind)
        || [GATE_OPENED, GATE_CLOSED, DIRECT_WRITE, CORRECTION].contains(&kind)
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
            detail: "Native SDD is disabled or not adopted; this transition is not gated by a native lifecycle. Run `specsync change adopt` to gate stages.".into(),
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

/// The native next action for a refusal, naming the status command once: when the
/// native next action already names it, no separate `Run` is added.
fn native_hint(next: &str, status: &str) -> String {
    if next.contains(status) {
        format!("Native next action: {next}.")
    } else {
        format!("Native next action: {next}. Run `{status}`.")
    }
}

/// A gate detail for the one-line next valid action: the part before the native next
/// action, which the views show on their own lines with the full gate detail.
fn action_detail(detail: &str) -> &str {
    detail
        .split_once(" Native next action")
        .map_or(detail, |(head, _)| head)
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
                    "{purpose} needs native state archived; it is {state}. {}",
                    native_hint(next, status)
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
                    "{purpose} needs an archive whose terminal evidence authenticates; summary.terminal_evidence.validity is {}. {}",
                    other.unwrap_or("not reported"),
                    native_hint(next, status)
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
                "{purpose} needs native state {wanted}; it is {state}. {}",
                native_hint(next, status)
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
                "{purpose} needs a valid definition approval; native status reports approval_valid {}. {}",
                approval.map_or("missing".to_string(), |valid| valid.to_string()),
                native_hint(next, status)
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
                "{purpose} needs valid native corrections; native status reports correction_valid false. {}",
                native_hint(next, status)
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
                    "{purpose} needs the approved semantic deltas; the delta for {} changed after the definition approval, while native status still reports approval_valid true (SpecSync 6.1.1, BUG-0002). Restore the approved delta, or have the changed scope approved: {} Digests use SpecSync 6.1.1's {DELTA_DOMAIN} framing.",
                    modules.join(", "),
                    reapproval(change)
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
                    "{purpose} needs the approved semantic deltas, which cannot be compared: {reason}. An approval records the digests, and a status read cannot clear this: {}",
                    reapproval(change)
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

/// The `approved_delta_digests` of the latest definition approval of `change`: `None`
/// when that approval records none, an error message when the ledger cannot be used.
fn approved_digests(
    root: &Path,
    change: &str,
) -> std::result::Result<Option<std::collections::BTreeMap<String, String>>, String> {
    let ledger_path = format!(".specsync/changes/{change}/approvals.json");
    let ledger = match native_file(root, &ledger_path)? {
        Some(bytes) => bytes,
        None => return Err(format!("the approval ledger {ledger_path} is missing")),
    };
    let malformed = || format!("the approval ledger {ledger_path} is malformed");
    let ledger = serde_json::from_slice::<Value>(&ledger).map_err(|_| malformed())?;
    let approvals = ledger
        .get("approvals")
        .and_then(Value::as_array)
        .ok_or_else(malformed)?;
    let Some(approval) = approvals
        .iter()
        .rev()
        .find(|approval| approval.get("gate").and_then(Value::as_str) == Some("definition"))
    else {
        return Err(format!("{ledger_path} has no definition approval"));
    };
    match approval.get("approved_delta_digests") {
        None | Some(Value::Null) => Ok(None),
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
                    _ => return Err(malformed()),
                }
            }
            Ok(Some(digests))
        }
        Some(_) => Err(malformed()),
    }
}

/// Compare the deltas of `change` with the digests its latest definition approval
/// recorded, as SpecSync 6.1.1 does before verification (REQ-cli-022). It reads the
/// native approval ledger and deltas only, starts no process and copies nothing.
pub(super) fn approved_deltas(root: &Path, change: &str, value: &Value) -> Deltas {
    let directory = format!(".specsync/changes/{change}");
    let ledger_path = format!("{directory}/approvals.json");
    let approved = match approved_digests(root, change) {
        Ok(approved) => approved,
        Err(reason) => return Deltas::Unverifiable(reason),
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

/// The requirement IDs that the delta of `change` for `module` adds or modifies, taken
/// from the same bytes whose digest matches the latest definition approval. A module
/// outside the approval's `approved_delta_digests` resolves nothing, so an extra delta
/// file cannot satisfy a link (REQ-cli-009).
pub(super) fn delta_requirements(
    root: &Path,
    change: &str,
    module: &str,
) -> std::result::Result<BTreeSet<String>, String> {
    let not_approved =
        || format!("its {module} delta is not among the change's verified approved deltas");
    let digest = approved_digests(root, change)?
        .and_then(|approved| approved.get(module).cloned())
        .ok_or_else(not_approved)?;
    let relative = format!(".specsync/changes/{change}/deltas/{module}.md");
    let Some(bytes) = native_file(root, &relative)? else {
        return Err(format!(
            "refused:native-delta-changed: {relative} (deleted)"
        ));
    };
    if delta_digest(module, &bytes) != digest {
        return Err(format!("refused:native-delta-changed: {relative} (edited)"));
    }
    let text = String::from_utf8(bytes).map_err(|_| format!("{relative} is not UTF-8"))?;
    let mut section = "";
    let mut ids = BTreeSet::new();
    for line in text.lines().map(str::trim_end) {
        if let Some(heading) = line.strip_prefix("## ") {
            section = heading.trim();
        } else if let Some(rest) = line.strip_prefix("### REQUIREMENT ")
            && matches!(section, "ADDED" | "MODIFIED")
            && let Some(id) = rest.split_whitespace().next()
        {
            ids.insert(id.to_string());
        }
    }
    Ok(ids)
}

/// The build gate's native checks for `change`, which a case link to a requirement only
/// its approved delta holds must pass (REQ-cli-009, REQ-cli-022).
pub(super) fn approved_change_gate(root: &Path, change: &str, state: &str, value: &Value) -> Gate {
    native_gate(
        root,
        change,
        "Linking a requirement from the approved delta",
        Prerequisite::Approved,
        state,
        value,
        &status_command(Some(change)),
    )
}

/// The approved-delta result for a report, or why it was not checked.
pub(super) fn delta_summary(root: &Path, change: Option<&str>, state: &NativeState) -> String {
    let (Some(change), NativeState::State(state, value)) = (change, state) else {
        return match state {
            NativeState::State(..) | NativeState::NoChange => {
                "not checked: no native change recorded".into()
            }
            NativeState::Unknown(_) => "not checked: native state unknown".into(),
            NativeState::NotConfigured => "not checked: no native lifecycle".into(),
            // `specsync init` leaves SDD off; say how to turn the lifecycle on (issue #40).
            NativeState::Disabled => {
                "not checked: native SDD is disabled; run `specsync change adopt` to gate stages"
                    .into()
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

/// After done the session is archived into its story (REQ-history-006), so the next
/// piece of work starts a new session, not a reset (REQ-cli-021, corrected in the
/// plan's Delta corrections).
const NEXT_SESSION: &str = "chough session new <key> --objective \"<outcome>\"";

/// The next piece of work after done: a new session, then, with a native lifecycle, a
/// new native change recorded in it.
fn next_work(native: bool) -> String {
    if native {
        format!(
            "Start the next piece of work in a new session: {NEXT_SESSION}, then {NEW_CYCLE_CHANGE}, then {NEW_CYCLE_RECORD}."
        )
    } else {
        format!(
            "Start the next piece of work in a new session: {NEXT_SESSION}, then chough checkpoint --next-action \"<next step>\"."
        )
    }
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

/// Whether the latest stage transition is a reset, which starts the new cycle: the
/// checkpoint still names the archived change until a new one is recorded, and the
/// reset's own next action applies. Gate records start no stage segment and are
/// skipped; a missing log reads as no reset, and an unreadable legacy log or one too
/// large to read whole is unknown (`Err`).
fn reached_by_reset(
    root: &Path,
    history: Option<&crate::history::HistoryRead>,
) -> Result<bool, String> {
    if let Some(read) = history {
        return Ok(read
            .records
            .iter()
            .rev()
            .filter_map(|value| value["kind"].as_str())
            .find(|kind| STAGE_KINDS.contains(kind))
            .is_some_and(|kind| kind == "reset"));
    }
    let _ = root;
    Ok(false)
}

/// When the newest reset was recorded, if the transition log names one; an unreadable
/// legacy log or one too large to read whole is unknown (`Err`).
fn latest_reset_at(
    root: &Path,
    history: Option<&crate::history::HistoryRead>,
) -> Result<Option<u64>, String> {
    // The newest reset by recorded time, not the last appended: merged logs can hold
    // resets out of order. When any reset's time cannot be read, none is trusted and
    // only the reached-by-reset test applies.
    fn newest<'a>(records: impl Iterator<Item = &'a Value>) -> Option<u64> {
        let mut newest = None;
        for record in records {
            if record.get("kind").and_then(Value::as_str) != Some("reset") {
                continue;
            }
            let at = record.get("at_unix_seconds").and_then(Value::as_u64)?;
            newest = newest.max(Some(at));
        }
        newest
    }
    let _ = root;
    Ok(history.and_then(|read| newest(read.records.iter())))
}

/// Whether the recorded archived change belongs to the cycle before the latest reset:
/// the latest stage transition is that reset, or the change's recorded finalization is
/// no later than it (REQ-cli-021). Without both times only the first test applies.
///
/// With event history the current cycle is read whole, by its own walk bounded by the
/// cycle's capacity (4096 events, 16 MiB), not by the recent window `next` shows: a
/// reset starts a new cycle as its root event, so a reset followed by more than 128
/// events is still found. When that walk is partial the answer is unknown (`Err`, with
/// why), never inferred from an omitted event.
fn previous_cycle(
    root: &Path,
    session: &Checkpoint,
    history: Option<&crate::history::HistoryRead>,
) -> Result<bool, String> {
    let finalized = session
        .native_change
        .as_deref()
        .and_then(|change| crate::library::finalized_at(root, change));
    let (reached, reset_at) = match history {
        Some(_) => {
            let cycle = crate::history::selected(
                session,
                crate::history::CYCLE_EVENTS,
                crate::history::CYCLE_BYTES,
                true,
            );
            if !cycle.partial.is_empty() {
                return Err(cycle.partial.join("; "));
            }
            (
                reached_by_reset(root, Some(&cycle))?,
                latest_reset_at(root, Some(&cycle))?,
            )
        }
        None => (reached_by_reset(root, None)?, latest_reset_at(root, None)?),
    };
    Ok(reached
        || finalized
            .zip(reset_at)
            .is_some_and(|(finalized, reset)| reset >= finalized))
}

/// The gate for the next main stage and the next valid action, both evaluated on one
/// native state, so `next` and `status` report the same decision (REQ-cli-021).
pub(super) fn assess(
    root: &Path,
    session: &Checkpoint,
    state: &NativeState,
    blocking: &[String],
    history: Option<&crate::history::HistoryRead>,
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
    // Read once, and only when an archived change at a main stage needs it.
    let archived_cycle = std::cell::OnceCell::new();
    let archived_cycle = || {
        archived_cycle
            .get_or_init(|| {
                (is_main(current) && archived_and_authenticated(root, session, state))
                    .then(|| previous_cycle(root, session, history))
            })
            .clone()
    };
    let action = if session.stage.status == "paused" {
        Action {
            action: format!(
                "The {current} stage is paused. Check native state and freshness, then `chough stage resume`."
            ),
            reason: "A paused stage keeps its scope, preferences and gates; native state may have changed meanwhile.".into(),
            command: Some("chough stage resume".into()),
        }
    } else if let holding @ [_, ..] = session
        .awaiting_human
        .iter()
        .filter(|gate| crate::session_md::gate(session, gate).holds == current)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        // REQ-cli-030: only a gate holding the current stage waits here; a gate holding
        // a later stage is listed and waits there (#237).
        Action {
            action: format!(
                "Waiting for a human decision: {}. Do not decide it for them; continue only independent work within scope. When they decide, record their words with `{CLOSE_GATE_COMMAND}`. `chough stage advance` and `chough stage skip` are refused until it is closed.",
                holding.join("; ")
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
    } else if let Some(refusal) = (current == "test")
        .then(|| unlinked_failure(root, session).ok().flatten())
        .flatten()
    {
        Action {
            action: format!("Leaving test is refused (refused:unlinked-failure): {refusal}"),
            reason: "A failing case stays failed until it passes or a bug records the failure (FLOW-7, LIFE-5).".into(),
            command: Some("chough bug add ... --observation <id>".into()),
        }
    } else if current == "done" {
        done_action(root, session, state, native_next)
    } else if archived_cycle() == Some(Ok(false)) {
        archived_action(session, state, native_next)
    } else if let Some(Err(why)) = archived_cycle() {
        // Neither cycle can be established: say so rather than guess either way.
        Action {
            action: format!(
                "Unknown whether native change {} belongs to this cycle: the current cycle's history cannot be read whole ({why}). Check `chough history show`, then reset for a new cycle or record this cycle's native change.",
                session.native_change.as_deref().unwrap_or("the native change")
            ),
            reason: "The reset that began this cycle is not in readable history, so the archived change is neither assumed closed with this cycle nor assumed to be the previous one's.".into(),
            command: Some("chough history show".into()),
        }
    } else if let Some((to, gate)) = gate.as_ref().filter(|(_, gate)| !gate.allowed) {
        if archived_cycle() == Some(Ok(true)) {
            // The archive is the previous cycle's: keep the reset's guidance and name
            // the new native change, not the archived refusal or another reset.
            Action {
                action: format!("Continue {current}: {}", session.next_action),
                reason: format!(
                    "Native change {} is archived from before the latest reset, so it belongs to the previous cycle; reaching {to} needs this cycle's new native change ({}).",
                    session
                        .native_change
                        .as_deref()
                        .unwrap_or("the native change"),
                    gate.label
                ),
                command: Some(NEW_CYCLE_CHANGE.into()),
            }
        } else {
            Action {
                action: format!(
                    "Gate not open ({}): {}",
                    gate.label,
                    action_detail(&gate.detail)
                ),
                reason: format!("Native state does not allow reaching {to} yet."),
                command: Some(if gate.label == "refused:no-native-change" {
                    NEW_CYCLE_CHANGE.into()
                } else {
                    status_command(session.native_change.as_deref())
                }),
            }
        }
    } else if current == "test" {
        // REQ-cli-038 (FLOW-15): native verification alone is not test evidence.
        Action {
            action: format!("Continue test: {}", session.next_action),
            reason: "Recorded test evidence is needed before accept: run the affected cases (`chough cases list` shows them) or record a manual result with `chough evidence add`; native verification is not product proof.".into(),
            command: Some("chough cases run <case-id> --environment <env>".into()),
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
    // A session still open at done is kept for release or for inbox entries to fold;
    // its next piece of work still starts in a new session.
    let closed = |action: String, reason: String| Action {
        action,
        reason: format!("{reason}{check_first}"),
        command: Some(NEXT_SESSION.into()),
    };
    let done = gate_for_state(root, session, "done", state.clone());
    match (state, done) {
        (NativeState::NotConfigured | NativeState::Disabled, _) => closed(
            next_work(false),
            "The cycle is closed; no native lifecycle gates it.".into(),
        ),
        (NativeState::NoChange, _) => closed(
            next_work(true),
            "No native change is recorded for this cycle, so it is closed.".into(),
        ),
        (NativeState::State(..), Some(gate)) if gate.allowed => closed(
            next_work(true),
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
            action: format!(
                "Gate not open ({}): {}",
                gate.label,
                action_detail(&gate.detail)
            ),
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
        "Record done first: chough stage advance (accept to done is allowed), which archives this session. Then: "
    } else {
        ""
    };
    let check_first = native_next
        .map(|text| format!(" Native next action, check it first: {text}."))
        .unwrap_or_default();
    // The action has fixed text, so it stays within the 512-byte cut of `chough next`
    // whatever the change ID; the ID goes in the reason.
    Action {
        action: format!(
            "The recorded native change is archived while the checkpoint is at {current}, so this cycle's work is closed. {record_done}{}",
            if at_accept {
                next_work(true)
            } else {
                new_cycle()
            }
        ),
        reason: format!(
            "Native change {change} is archived while the checkpoint is at {current}, with terminal evidence {}; continuing {current} would work on a closed change.{check_first}",
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
    session: &mut Checkpoint,
    kind: &str,
    route: (&str, &str),
    reason: Option<&str>,
    gate: Option<&str>,
    refresh: Vec<String>,
) -> Result<()> {
    log_transition_at(
        root, session, kind, route, reason, gate, refresh, None, None,
    )
}

#[allow(clippy::too_many_arguments)]
fn log_transition_at(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    route: (&str, &str),
    reason: Option<&str>,
    gate: Option<&str>,
    refresh: Vec<String>,
    backfill: Option<(u64, Backfill)>,
    relayed_by: Option<RelayedBy>,
) -> Result<()> {
    let (from, to) = route;
    let (at, backfill) = match backfill {
        Some((at, backfill)) => (at, Some(backfill)),
        None => (now()?, None),
    };
    let transition = Transition {
        schema_version: 1,
        at_unix_seconds: at,
        kind: kind.into(),
        from: from.into(),
        to: to.into(),
        reason: reason.map(str::to_string),
        gate: gate.map(str::to_string),
        revision: head_revision(root),
        refresh,
        backfill,
        relayed_by,
        direct_write: None,
        actor: None,
        holds: None,
        drives: None,
        corrects: None,
    };
    validate_records(&transition)?;
    // REQ-cli-060 (DRIVE-21): a stage move first records the direct lifecycle writes
    // found since the baseline, in the stage it leaves.
    if STAGE_KINDS.contains(&kind) {
        for write in crate::direct_writes::to_record(root, session.native_change.as_deref()) {
            let event = Transition {
                schema_version: 1,
                at_unix_seconds: transition.at_unix_seconds,
                kind: DIRECT_WRITE.into(),
                from: from.into(),
                to: from.into(),
                reason: None,
                gate: None,
                revision: transition.revision.clone(),
                refresh: vec![],
                backfill: None,
                relayed_by: None,
                direct_write: Some(write),
                actor: None,
                holds: None,
                drives: None,
                corrects: None,
            };
            validate_records(&event)?;
            append_transition(root, session, event)?;
        }
    }
    append_transition(root, session, transition)
}

/// Append one transition to the session's history, or to the legacy log.
fn append_transition(root: &Path, session: &mut Checkpoint, transition: Transition) -> Result<()> {
    crate::history::append(root, session, transition)
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
pub(super) fn log_gate(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    stage: &str,
    gate: &str,
    relayed_by: Option<RelayedBy>,
) -> Result<()> {
    ensure!(
        kind == GATE_OPENED || kind == GATE_CLOSED,
        "not a gate record kind: {kind}"
    );
    log_transition_at(
        root,
        session,
        kind,
        (stage, stage),
        Some(gate),
        None,
        vec![],
        None,
        relayed_by,
    )?;
    // A gate-opened entry records what the gate holds and drives (REQ-cli-030).
    if kind == GATE_OPENED {
        let meta = crate::session_md::gate(session, gate);
        if let Some(entry) = session.pending.last_mut() {
            entry.holds = Some(meta.holds);
            entry.drives = Some(meta.drives);
        }
    }
    Ok(())
}

pub(super) fn recent_transitions(root: &Path, count: usize) -> Result<(Vec<Value>, bool)> {
    let session = read_checkpoint(root)?;
    let read = crate::history::selected(
        &session,
        count.min(crate::history::RECENT_EVENTS),
        crate::history::RECENT_BYTES,
        true,
    );
    // An entry that is not a valid transition is refused, never shown as history.
    if let Some(invalid) = read
        .partial
        .iter()
        .find(|why| why.contains("invalid transition"))
    {
        bail!("{invalid}");
    }
    Ok((read.records, !read.partial.is_empty()))
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
        "authority": "Native SpecSync status decides lifecycle gates; Chough never writes native approval or review, and a relayed decision it keeps is a claim, not authenticated.",
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

/// How to record a human's decision on a gate on their behalf (REQ-cli-050).
pub(super) const CLOSE_GATE_COMMAND: &str = "chough checkpoint --close-gate \"<gate>\" --decided-by \"<who>\" --words \"<their words>\" --where \"<where>\" --recorded-by \"<agent>\"";

/// How the scope of `change` is approved again (REQ-cli-022): the human's decision at
/// a scope gate that drives the approval (REQ-cli-030, REQ-records-008), or, on
/// authority they gave in advance, the acting agent's approval quoting it
/// (REQ-cli-029); never a command for the human to run.
fn reapproval(change: &str) -> String {
    format!(
        "open the scope gate with `chough checkpoint --awaiting-human \"Approve the changed scope of {change}\" --drives approve`, and when the human decides, record it with `{CLOSE_GATE_COMMAND}`, which runs the approval as them; on authority they gave in advance, record specsync change approve {change} --actor \"<agent>-<terminal>\" --note \"Approved under <the human's words> (<source>); covered scope: <scope>; limits: <limits>. Recorded by <agent> (<model>, hc <terminal>).\" instead. Then run specsync change check {change}."
    )
}

/// An open human gate holds advance and skip out of the stage it holds (REQ-cli-030);
/// no reason bypasses it. A gate holding a later stage does not stop a move out of an
/// earlier one (#237). `leaving` is every stage the move leaves.
fn ensure_no_open_human_gate(session: &Checkpoint, kind: &str, leaving: &[&str]) -> Result<()> {
    let gates: Vec<&String> = session
        .awaiting_human
        .iter()
        .filter(|gate| leaving.contains(&crate::session_md::gate(session, gate).holds.as_str()))
        .collect();
    if gates.is_empty() {
        return Ok(());
    }
    let more = match gates.len().saturating_sub(3) {
        0 => String::new(),
        rest => format!(" (and {rest} more)"),
    };
    bail!(
        "{kind} refused (refused:awaiting-human): waiting for a human decision: {}{more}; record the human's decision on a gate with `{CLOSE_GATE_COMMAND}`, or withdraw gates no human decision closes with `chough checkpoint --clear-awaiting-human` (re-pass gates still waiting with --awaiting-human)",
        gates
            .iter()
            .take(3)
            .map(|gate| gate.as_str())
            .collect::<Vec<_>>()
            .join("; ")
    )
}

/// Leaving test needs every case referenced in the checkpoint to have a latest
/// observation, by recorded time and then ID, that did not fail, or one that a local
/// bug of any status links (issue #39). Describes the first such failure by case ID.
fn unlinked_failure(root: &Path, session: &Checkpoint) -> Result<Option<String>> {
    if session.references.cases.is_empty() {
        return Ok(None);
    }
    let observations = evidence::load_all(root)?;
    let linked: BTreeSet<String> = bugs::observation_references(root)?
        .into_iter()
        .flat_map(|(_, ids)| ids)
        .collect();
    let referenced: BTreeSet<&String> = session.references.cases.iter().collect();
    let failures: Vec<(&String, &String)> = referenced
        .into_iter()
        .filter_map(|case| {
            observations
                .iter()
                .filter(|record| record.cases.iter().any(|link| &link.id == case))
                // A not-run attempt executed nothing; the latest executed result decides.
                .filter(|record| record.outcome != evidence::Outcome::NotRun)
                .max_by(|a, b| {
                    (a.observed_at_unix_seconds, &a.id).cmp(&(b.observed_at_unix_seconds, &b.id))
                })
                .filter(|latest| {
                    latest.outcome == evidence::Outcome::Failed && !linked.contains(&latest.id)
                })
                .map(|latest| (case, &latest.id))
        })
        .collect();
    let Some((case, observation)) = failures.first() else {
        return Ok(None);
    };
    let others = match failures.len() - 1 {
        0 => String::new(),
        1 => " (and 1 more such case)".into(),
        more => format!(" (and {more} more such cases)"),
    };
    Ok(Some(format!(
        "case {case} latest observation {observation} failed and no bug links it{others}; rerun the case until it passes, or link the failure with `chough bug add ... --observation {observation}` or `chough bug update <bug> --observation {observation}`"
    )))
}

/// Apply a move to the next stage, checking the gates that apply to it.
/// What a backfill's output says about its quote.
const BACKFILL_EVIDENCE: &str = "The quote and commit are evidence only; no human approval or review is recorded, and native state decided the gate now.";

/// Parse `--at`: Unix seconds or `YYYY-MM-DDTHH:MM:SSZ`.
fn parse_at(text: &str) -> Result<u64> {
    let text = text.trim();
    if !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit()) {
        return text
            .parse()
            .map_err(|_| anyhow::anyhow!("--at {text:?} is out of range"));
    }
    let bad = || anyhow::anyhow!("--at {text:?} is not Unix seconds or YYYY-MM-DDTHH:MM:SSZ");
    let bytes = text.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(bad());
    }
    let number = |range: std::ops::Range<usize>| -> Result<i64> {
        let part = &text[range];
        ensure!(part.bytes().all(|byte| byte.is_ascii_digit()), bad());
        Ok(part.parse()?)
    };
    let (year, month, day) = (number(0..4)?, number(5..7)?, number(8..10)?);
    let (hour, minute, second) = (number(11..13)?, number(14..16)?, number(17..19)?);
    ensure!(
        (1970..=9999).contains(&year)
            && (1..=12).contains(&month)
            && (1..=31).contains(&day)
            && hour < 24
            && minute < 60
            && second < 60,
        bad()
    );
    // Howard Hinnant's days_from_civil.
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (month + 9) % 12;
    let doy = (153 * mp + 2) / 5 + day - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    let seconds = days * 86_400 + hour * 3_600 + minute * 60 + second;
    ensure!(
        crate::timing::utc(seconds as u64) == text,
        "--at {text:?} is not a calendar date"
    );
    Ok(seconds as u64)
}

/// Git under the stop watch (FLOW-14): a stop stops it and ends Chough by the signal.
fn git_output(root: &Path, args: &[&str]) -> Option<std::process::Output> {
    crate::records::run_owned(
        std::process::Command::new("git")
            .args(args)
            .current_dir(root),
        None,
    )
    .ok()
}

/// Check a backfill's time and evidence before any native read or record.
fn backfill_evidence(root: &Path, at: &str, commit: &str, quote: &str) -> Result<(u64, Backfill)> {
    ensure_text("quote", quote)?;
    let at = parse_at(at)?;
    let recorded = now()?;
    ensure!(
        at <= recorded,
        "backfill refused: --at {} is in the future",
        crate::timing::utc(at)
    );
    let (latest, _) = recent_transitions(root, 1).map_err(|error| {
        anyhow::anyhow!("backfill refused: the latest transition is unreadable: {error:#}")
    })?;
    if let Some(record) = latest.last() {
        let Some(previous) = record.get("at_unix_seconds").and_then(Value::as_u64) else {
            bail!("backfill refused: the latest transition has no readable time");
        };
        ensure!(
            at >= previous,
            "backfill refused: --at {} is before the latest recorded transition at {}; history stays in time order",
            crate::timing::utc(at),
            crate::timing::utc(previous)
        );
    }
    ensure!(
        !commit.trim().is_empty() && !commit.starts_with('-'),
        "backfill refused: --commit {commit:?} does not resolve to a commit"
    );
    let resolved = git_output(
        root,
        &[
            "rev-parse",
            "--verify",
            "--quiet",
            &format!("{commit}^{{commit}}"),
        ],
    )
    .filter(|output| output.status.success())
    .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
    .filter(|sha| !sha.is_empty())
    .ok_or_else(|| {
        anyhow::anyhow!("backfill refused: --commit {commit:?} does not resolve to a commit")
    })?;
    let ancestor = git_output(root, &["merge-base", "--is-ancestor", &resolved, "HEAD"])
        .is_some_and(|output| output.status.success());
    ensure!(
        ancestor,
        "backfill refused: commit {resolved} is not an ancestor of HEAD"
    );
    Ok((
        at,
        Backfill {
            recorded_at_unix_seconds: recorded,
            commit: resolved,
            quote: quote.to_string(),
        },
    ))
}

/// Check one forward step from `route.0` to `route.1` as `stage advance` checks it: on
/// leaving test, open blocking bugs and an unlinked failure, then the destination's
/// native gate. `read` holds the native state once read, so a batch of steps shares one
/// read (REQ-cli-052). Returns the gate label to record, if the destination has a gate.
fn forward_step(
    root: &Path,
    session: &Checkpoint,
    kind: &str,
    route: (&str, &str),
    read: &mut Option<NativeState>,
) -> Result<Option<String>> {
    let (from, to) = route;
    if from == "test" && to == "accept" {
        let blocking = bugs::open_blocking(root)?;
        ensure!(
            blocking.is_empty(),
            "{kind} refused (refused:blocking-bugs): open acceptance-blocking bugs {}; go back to build, fix and retest",
            blocking.join(", ")
        );
        if let Some(refusal) = unlinked_failure(root, session)? {
            bail!("{kind} refused (refused:unlinked-failure): {refusal}");
        }
    }
    if prerequisite(to).is_none() {
        return Ok(None);
    }
    let state = match read {
        Some(state) => state.clone(),
        None => read
            .insert(native_state(root, session.native_change.as_deref())?)
            .clone(),
    };
    let Some(gate) = gate_for_state(root, session, to, state) else {
        return Ok(None);
    };
    ensure!(
        gate.allowed,
        "{} refused ({}): {}",
        kind,
        gate.label,
        gate.detail
    );
    Ok(Some(gate.label))
}

/// `stage advance --to <target>`: every main stage up to `target` on one native read.
/// Every step is checked before the first transition is recorded, so a refused step
/// records nothing (REQ-cli-052). Returns the applied move and the native read it made.
fn advance_to(
    root: &Path,
    session: &mut Checkpoint,
    target: &str,
    note: Option<&str>,
) -> Result<(Value, Option<NativeState>)> {
    ensure_active(session)?;
    ensure_main(session, "advance")?;
    let from = session.stage.current.clone();
    let start = position(&from).ok_or_else(|| {
        anyhow::anyhow!(
            "{from} is not on the advance path; `stage advance --to` moves through main stages only, so use `chough stage advance`"
        )
    })?;
    let end = position(target).ok_or_else(|| {
        anyhow::anyhow!(
            "{target} is not a main stage on the advance path; `--to` takes design, plan, build, test, accept or done"
        )
    })?;
    ensure!(
        end > start,
        "{target} is not ahead of {from}; `stage advance --to` only moves forward (use `chough stage back` or `chough stage reset` to go back)"
    );
    // Every stage the move leaves counts (REQ-cli-030).
    ensure_no_open_human_gate(session, "advance", &ORDER[start..end])?;
    let mut read = None;
    let mut steps = Vec::new();
    for index in start..end {
        let route = (ORDER[index], ORDER[index + 1]);
        let kind = format!("advance {} -> {}", route.0, route.1);
        let label = forward_step(root, session, &kind, route, &mut read)
            .map_err(|error| anyhow::anyhow!("{error:#}; nothing was recorded"))?;
        steps.push((route, label));
    }
    let mut applied_steps = Vec::new();
    for ((step_from, step_to), label) in &steps {
        if let Err(error) = log_transition(
            root,
            session,
            "advance",
            (step_from, step_to),
            note,
            label.as_deref(),
            vec![],
        ) {
            // Entries are written only with the save, so nothing of the batch is left.
            bail!(
                "advance {step_from} -> {step_to} could not be recorded: {error:#}; nothing was saved"
            );
        }
        applied_steps.push(json!({"from": step_from, "to": step_to, "gate": label}));
    }
    session.stage.current = target.to_string();
    session.stage.iteration = 1;
    let gate = steps.last().and_then(|(_, label)| label.clone());
    Ok((
        json!({"kind": "advance", "from": from, "to": target, "gate": gate, "steps": applied_steps}),
        read,
    ))
}

fn move_forward(
    root: &Path,
    session: &mut Checkpoint,
    kind: &str,
    reason: Option<&str>,
    backfill: Option<(u64, Backfill)>,
) -> Result<(Value, Option<NativeState>)> {
    ensure_active(session)?;
    ensure_main(session, kind)?;
    let from = session.stage.current.clone();
    ensure_no_open_human_gate(session, kind, &[from.as_str()])?;
    ensure!(
        backfill.is_none() || from != "release",
        "backfill refused: release to done cannot be backfilled; advance it when it happens"
    );
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
    let mut read = None;
    let gate_label = forward_step(root, session, kind, (&from, &to), &mut read)?;
    let evidence = backfill.as_ref().map(|(at, backfill)| {
        json!({
            "at_unix_seconds": at,
            "recorded_at_unix_seconds": backfill.recorded_at_unix_seconds,
            "commit": backfill.commit,
            "quote": backfill.quote,
            "evidence": BACKFILL_EVIDENCE,
        })
    });
    log_transition_at(
        root,
        session,
        kind,
        (&from, &to),
        reason,
        gate_label.as_deref(),
        vec![],
        backfill,
        None,
    )?;
    session.stage.current = to.clone();
    session.stage.iteration = 1;
    let mut applied = json!({"kind": kind, "from": from, "to": to, "gate": gate_label});
    if let Some(evidence) = evidence {
        applied["backfill"] = evidence;
    }
    Ok((applied, read))
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
        session,
        kind,
        (&from, target),
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
    let mut applied = match command {
        StageCommand::Show => None,
        StageCommand::Advance { note, to } => {
            if let Some(note) = &note {
                ensure_text("note", note)?;
            }
            let (applied, _) = match to {
                Some(target) => advance_to(root, &mut session, &target, note.as_deref())?,
                None => move_forward(root, &mut session, "advance", note.as_deref(), None)?,
            };
            Some(applied)
        }
        StageCommand::Skip { reason } => {
            ensure_text("reason", &reason)?;
            Some(move_forward(root, &mut session, "skip", Some(&reason), None)?.0)
        }
        StageCommand::Iterate { note } => {
            ensure_text("note", &note)?;
            ensure_active(&session)?;
            let stage = session.stage.current.clone();
            log_transition(
                root,
                &mut session,
                "iterate",
                (&stage, &stage),
                Some(&note),
                None,
                vec![],
            )?;
            session.stage.iteration += 1;
            Some(json!({"kind": "iterate", "stage": stage, "iteration": session.stage.iteration}))
        }
        StageCommand::Backfill {
            at,
            commit,
            quote,
            note,
        } => {
            if let Some(note) = &note {
                ensure_text("note", note)?;
            }
            let backfill = backfill_evidence(root, &at, &commit, &quote)?;
            let (applied, _) = move_forward(
                root,
                &mut session,
                "advance",
                note.as_deref(),
                Some(backfill),
            )?;
            Some(applied)
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
            log_transition(
                root,
                &mut session,
                "pause",
                (&stage, &stage),
                Some(&reason),
                None,
                vec![],
            )?;
            session.stage.status = "paused".into();
            Some(json!({"kind": "pause", "stage": stage}))
        }
        StageCommand::Resume => {
            ensure!(session.stage.status == "paused", "the stage is not paused");
            let stage = session.stage.current.clone();
            log_transition(
                root,
                &mut session,
                "resume",
                (&stage, &stage),
                None,
                None,
                vec![],
            )?;
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
                    &mut session,
                    "enter",
                    (&from, "release"),
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
                    &mut session,
                    "enter",
                    (&from, &stage),
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
                &mut session,
                "return",
                (&from, &caller.stage),
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
        save_checkpoint(root, &mut session)?;
        // REQ-cli-060: the findings are recorded, so the baseline is the records now; a
        // failed baseline write never undoes the move.
        if let Err(error) = crate::direct_writes::settle(root, session.native_change.as_deref())
            && let Some(applied) = applied.as_mut()
        {
            applied["direct_writes"] = json!({"unknown": error.to_string()});
        }
    }
    // REQ-cli-041 (KNOW-13): after the move into done is saved, trim old proof; the
    // result is reported and never undoes the move.
    if let Some(applied) = applied
        .as_mut()
        .filter(|applied| applied["kind"] == "advance" && applied["to"] == "done")
    {
        // REQ-history-006 (DRIVE-13, #155): the session archives itself in its own change
        // and the active pointer is cleared; never undoes the move.
        let from = applied["from"].as_str().unwrap_or_default().to_string();
        applied["session_archive"] = crate::sessions::archive_at_done(root, &session, &from);
        // REQ-records-011: then its journal post, and REQ-cli-043: then index.md and
        // feed.xml, regenerated once; neither undoes the move.
        if applied["session_archive"]["status"] == "archived"
            && let Some(id) = applied["session_archive"]["id"]
                .as_str()
                .map(str::to_string)
        {
            applied["journal_post"] = crate::journal::post_for_done(root, &id);
        }
        applied["librarian_index"] = crate::library::refresh_at_done(root);
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
