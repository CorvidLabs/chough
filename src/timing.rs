//! The read-only timing report (REQ-reporting-001): how long stages, tests, bug fixes and
//! human gates took, derived from records that already exist. It writes no file, runs
//! no case and makes no network request; its only external commands are bounded
//! `specsync change status` reads. Every duration is elapsed wall-clock time, never
//! effort, and a time no record holds stays unknown, never zero.
//!
//! Native reads (REQ-reporting-002) are local to one report: each change ID is read at most
//! once, archived history is read only when the report shows it, and total native
//! wait is bounded by one budget. None of this applies to gates, `next` or `status`.

use crate::checkpoint::{Session, read_session};
use crate::evidence::{self, Outcome};
use crate::markdown::{self, Kind, Record as Front};
use crate::records::{
    NativeJson, TIMING_BUDGET_VARIABLE, local_path, native_json_within, now, print_json,
    read_bounded_record, read_native_text, trace_phase,
};
use crate::session_md::parse_time;
use crate::{bugs, stage, validate};
use anyhow::{Result, bail};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};

/// Transitions are read in full up to this bound; earlier bytes are skipped.
const TRANSITIONS_LIMIT: u64 = 16 * 1_048_576;
/// Archived changes read for history, most recent first; older ones are skipped.
const ARCHIVE_HISTORY: usize = 10;
/// Default whole-report native budget, in seconds (REQ-reporting-002).
const NATIVE_BUDGET: u64 = 120;
const HISTORY_RULE: &str =
    "Archived native history is read only for --cycle all with no --stage, --case or --bug filter.";
/// Fewer samples than this are too few to estimate.
const MIN_SAMPLES: usize = 3;
const ARCHIVE_DIR: &str = ".specsync/archive/changes";
/// Archived session stories (REQ-history-006), whose timelines and receipts timing reads.
const STORIES_DIR: &str = ".chough/archive/sessions";
/// A correction (REQ-history-015): a known Timeline kind that starts and ends no stage
/// segment. Plan step 7 adds it; this points at its constant once it lands.
const CORRECTION: &str = stage::CORRECTION;
const CHANGES_DIR: &str = ".specsync/changes";

const NO_FIRST: &str = "no transition before the first record";
const GATE_UNKNOWN_OPEN: &str = "opened before gate records or record missing";
const GATE_UNKNOWN_CLOSE: &str = "cleared without a gate record";
const ELAPSED: &str = "elapsed wall-clock";
const TOGETHER: &str = "recorded together";
const ONGOING: &str = "ongoing";
const RUN_WALL_CLOCK: &str = "total run wall-clock, including intervals and resumes";

#[derive(clap::Args)]
pub(super) struct TimingArgs {
    #[arg(long)]
    json: bool,
    /// Which cycles to show: the current one, or all of them.
    #[arg(long, value_parser = ["current", "all"], default_value = "all")]
    cycle: String,
    /// Show only this stage's segments, totals and estimates.
    #[arg(long)]
    stage: Option<String>,
    /// Show only this case's observations and validation runs.
    #[arg(long)]
    case: Option<String>,
    /// Show only this bug.
    #[arg(long)]
    bug: Option<String>,
}

// ---------------------------------------------------------------------------
// Times and durations.

/// A point in time, or why it is not known.
#[derive(Clone)]
enum Time {
    Known(u64),
    Unknown(String),
}

impl Time {
    fn of(value: Option<u64>, reason: &str) -> Self {
        value.map_or_else(|| Self::Unknown(reason.into()), Self::Known)
    }

    fn known(&self) -> Option<u64> {
        match self {
            Self::Known(at) => Some(*at),
            Self::Unknown(_) => None,
        }
    }

    fn to_json(&self) -> Value {
        match self {
            Self::Known(at) => json!({"unix_seconds": at, "utc": utc(*at), "unknown": null}),
            Self::Unknown(reason) => json!({"unix_seconds": null, "utc": null, "unknown": reason}),
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Known(at) => utc(*at),
            Self::Unknown(reason) => format!("unknown ({reason})"),
        }
    }
}

/// An elapsed wall-clock duration with its label, or why it is not known.
#[derive(Clone)]
enum Span {
    Known(u64, &'static str),
    Unknown(String),
}

impl Span {
    /// From start to end; an end before its start is unknown, never negative.
    fn between(start: &Time, end: &Time, ongoing: bool) -> Self {
        match (start, end) {
            (Time::Unknown(reason), _) | (_, Time::Unknown(reason)) => {
                Self::Unknown(reason.clone())
            }
            (Time::Known(start), Time::Known(end)) if end < start => {
                Self::Unknown("end before start".into())
            }
            (Time::Known(start), Time::Known(end)) => Self::Known(
                end - start,
                if ongoing {
                    ONGOING
                } else if end == start {
                    TOGETHER
                } else {
                    ELAPSED
                },
            ),
        }
    }

    /// A total: unknown when any part is unknown, never a partial sum.
    fn sum<'a>(parts: impl IntoIterator<Item = &'a Span>) -> Self {
        let mut total = 0;
        for part in parts {
            match part {
                Self::Known(seconds, _) => total += seconds,
                Self::Unknown(reason) => return Self::Unknown(reason.clone()),
            }
        }
        Self::Known(total, ELAPSED)
    }

    fn seconds(&self) -> Option<u64> {
        match self {
            Self::Known(seconds, _) => Some(*seconds),
            Self::Unknown(_) => None,
        }
    }

    fn to_json(&self) -> Value {
        match self {
            Self::Known(seconds, label) => {
                json!({"seconds": seconds, "label": label, "reason": null})
            }
            Self::Unknown(reason) => json!({"seconds": null, "label": "unknown", "reason": reason}),
        }
    }

    fn text(&self) -> String {
        match self {
            Self::Known(seconds, label) => format!("{} ({label})", clock(*seconds)),
            Self::Unknown(reason) => format!("unknown ({reason})"),
        }
    }
}

/// UTC ISO 8601 from Unix seconds (proleptic Gregorian calendar).
pub(super) fn utc(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64;
    let rest = seconds % 86_400;
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        rest / 3_600,
        rest % 3_600 / 60,
        rest % 60
    )
}

fn clock(seconds: u64) -> String {
    let (hours, minutes, rest) = (seconds / 3_600, seconds % 3_600 / 60, seconds % 60);
    match (hours, minutes) {
        (0, 0) => format!("{rest}s"),
        (0, _) => format!("{minutes}m {rest}s"),
        _ => format!("{hours}h {minutes}m {rest}s"),
    }
}

// ---------------------------------------------------------------------------
// Transitions: stage segments, cycles and gates.

struct Record {
    at: Time,
    kind: String,
    from: String,
    to: String,
    reason: Option<String>,
    /// A backfilled move's evidence (REQ-history-005): when it was recorded, the
    /// commit and the quote. Its `at` is the original time.
    backfill: Option<Value>,
}

struct Transitions {
    records: Vec<Record>,
    skipped_bytes: u64,
    unreadable: usize,
    unrecognised: usize,
    history_partial: Vec<String>,
    lineages: Vec<Value>,
}

/// Read transitions leniently: a line that is not a transition is counted and skipped,
/// and a missing or non-numeric time is unknown.
fn read_transitions(_root: &Path, session: &Session, all: bool) -> Result<Transitions> {
    let _ = all;
    let (lines, skipped_bytes, history_partial, lineages) = {
        let Session::V2(checkpoint) = session;
        let read = crate::history::timing(checkpoint);
        (
            read.records
                .iter()
                .map(Value::to_string)
                .collect::<Vec<_>>(),
            0_u64,
            read.partial,
            read.lineages,
        )
    };
    let mut records = vec![];
    let mut unreadable = 0;
    let mut unrecognised = 0;
    for line in &lines {
        let Ok(value) = serde_json::from_str::<Value>(line) else {
            unreadable += 1;
            continue;
        };
        let text = |key: &str| value.get(key).and_then(Value::as_str).map(str::to_string);
        let (Some(kind), Some(from), Some(to)) = (text("kind"), text("from"), text("to")) else {
            unreadable += 1;
            continue;
        };
        // A direct lifecycle write (REQ-history-013, REQ-reporting-001) and a correction
        // (REQ-history-015) are known kinds that start or end no segment: skipped, never
        // counted as unrecognised.
        if kind == stage::DIRECT_WRITE || kind == CORRECTION {
            continue;
        }
        if !stage::STAGE_KINDS.contains(&kind.as_str())
            && kind != stage::GATE_OPENED
            && kind != stage::GATE_CLOSED
        {
            unrecognised += 1;
            continue;
        }
        records.push(Record {
            at: Time::of(
                value.get("at_unix_seconds").and_then(Value::as_u64),
                "missing timestamp",
            ),
            kind,
            from,
            to,
            reason: text("reason"),
            backfill: value
                .get("backfill")
                .filter(|backfill| backfill.is_object())
                .cloned(),
        });
    }
    Ok(Transitions {
        records,
        skipped_bytes,
        unreadable,
        unrecognised,
        history_partial,
        lineages,
    })
}

struct Segment {
    stage: String,
    /// active, paused, or idle (time in done, between cycles).
    state: &'static str,
    side: bool,
    iteration: u32,
    visit: u32,
    entered_by: Option<String>,
    start: Time,
    end: Time,
    ongoing: bool,
    elapsed: Span,
    waiting: Option<Span>,
    active: Option<Span>,
    /// A backfilled move starts or ends this segment, so one bound is a time
    /// given after the fact.
    backfilled: bool,
}

impl Segment {
    fn to_json(&self) -> Value {
        json!({
            "stage": self.stage,
            "state": self.state,
            "side": self.side,
            "iteration": self.iteration,
            "visit": self.visit,
            "entered_by": self.entered_by,
            "start": self.start.to_json(),
            "end": self.end.to_json(),
            "ongoing": self.ongoing,
            "elapsed": self.elapsed.to_json(),
            "waiting": self.waiting.as_ref().map(Span::to_json),
            "active": self.active.as_ref().map(Span::to_json),
            "backfilled": self.backfilled,
        })
    }

    fn text(&self) -> String {
        let what = match self.state {
            "idle" => format!("{} (idle between cycles)", self.stage),
            "paused" => format!("{} paused", self.stage),
            _ if self.side => format!("{} (side stage)", self.stage),
            _ => format!("{}, iteration {}", self.stage, self.iteration),
        };
        let mut line = format!(
            "- {what}: {} -> {}{}, {}",
            self.start.text(),
            if self.ongoing {
                "now".to_string()
            } else {
                self.end.text()
            },
            self.entered_by
                .as_ref()
                .map_or(String::new(), |kind| format!(" (entered by {kind})")),
            self.elapsed.text()
        );
        if let (Some(waiting), Some(active)) = (&self.waiting, &self.active) {
            line.push_str(&format!(
                "; waiting on a human {}; active {}",
                waiting.text(),
                active.text()
            ));
        }
        line
    }
}

struct Cycle {
    number: usize,
    start: Time,
    segments: Vec<Segment>,
    completed: bool,
}

/// One gate's open and close times; an interval for overlap with segments.
struct Gate {
    text: String,
    opened: Time,
    closed: Option<Time>,
    open_now: bool,
}

impl Gate {
    fn waiting(&self, at: u64) -> Span {
        match (&self.closed, self.open_now) {
            (_, true) => Span::between(&self.opened, &Time::Known(at), true),
            (Some(closed), false) => Span::between(&self.opened, closed, false),
            (None, false) => Span::Unknown(GATE_UNKNOWN_CLOSE.into()),
        }
    }

    /// The interval as known start and end; None where the time is not known.
    fn interval(&self, at: u64) -> (Option<u64>, Option<u64>) {
        let end = if self.open_now {
            Some(at)
        } else {
            self.closed.as_ref().and_then(Time::known)
        };
        (self.opened.known(), end)
    }

    fn to_json(&self, at: u64) -> Value {
        let closed = match (&self.closed, self.open_now) {
            (_, true) => Value::Null,
            (Some(closed), false) => closed.to_json(),
            (None, false) => Time::Unknown(GATE_UNKNOWN_CLOSE.into()).to_json(),
        };
        json!({
            "gate": self.text,
            "state": if self.open_now { "open" } else { "closed" },
            "opened": self.opened.to_json(),
            "closed": closed,
            "waiting": self.waiting(at).to_json(),
        })
    }
}

/// Pair gate records by text and reconcile them with the current awaiting-human list.
fn gates(records: &[Record], awaiting: &[String]) -> Vec<Gate> {
    let mut gates: Vec<Gate> = vec![];
    let mut open: BTreeMap<String, usize> = BTreeMap::new();
    for record in records {
        let Some(text) = record.reason.clone() else {
            continue;
        };
        if record.kind == stage::GATE_OPENED {
            // The checkpoint writes gate-opened only for a gate not already listed, so a
            // second one for an open gate means it was cleared in between without a
            // record: the earlier opening keeps an unknown close (`closed` None) and a
            // new gate starts here.
            open.insert(text.clone(), gates.len());
            gates.push(Gate {
                text,
                opened: record.at.clone(),
                closed: None,
                open_now: false,
            });
        } else if record.kind == stage::GATE_CLOSED {
            match open.remove(&text) {
                Some(index) => gates[index].closed = Some(record.at.clone()),
                None => gates.push(Gate {
                    text,
                    opened: Time::Unknown(GATE_UNKNOWN_OPEN.into()),
                    closed: Some(record.at.clone()),
                    open_now: false,
                }),
            }
        }
    }
    // Opened and still listed is open; opened, never closed and no longer listed was
    // cleared at an unknown time (the gate's `closed` stays None).
    for (text, index) in &open {
        gates[*index].open_now = awaiting.contains(text);
    }
    for text in awaiting {
        if !open.contains_key(text) {
            gates.push(Gate {
                text: text.clone(),
                opened: Time::Unknown(GATE_UNKNOWN_OPEN.into()),
                closed: None,
                open_now: true,
            });
        }
    }
    gates
}

/// Waiting-on-human time within a segment: the merged overlap of known gate intervals.
/// A gate that may overlap but has an unknown open or close time makes it unknown.
fn waiting_in(start: u64, end: u64, gates: &[Gate], at: u64) -> Span {
    let mut overlaps = vec![];
    for gate in gates {
        match gate.interval(at) {
            // An end before its start means one of the two times is wrong. If the open
            // time is wrong the gate lies before `closed`; if the close time is, after
            // `opened`. A segment that could overlap under either reading is unknown;
            // an open gate opened after now may have opened at any earlier time. A
            // segment strictly between the two times fits neither reading and gets no
            // overlap from the arm below.
            (Some(opened), Some(closed))
                if closed < opened && (gate.open_now || start < closed || end > opened) =>
            {
                return Span::Unknown(format!(
                    "gate \"{}\" may overlap and its end is before its start",
                    gate.text
                ));
            }
            (Some(opened), Some(closed)) => {
                let (from, to) = (opened.max(start), closed.min(end));
                if from < to {
                    overlaps.push((from, to));
                }
            }
            (None, Some(closed)) if start < closed => {
                return Span::Unknown(format!(
                    "gate \"{}\" may overlap and its open time is unknown",
                    gate.text
                ));
            }
            (Some(opened), None) if end > opened => {
                return Span::Unknown(format!(
                    "gate \"{}\" may overlap and its close time is unknown",
                    gate.text
                ));
            }
            (None, None) => {
                return Span::Unknown(format!(
                    "gate \"{}\" has unknown open and close times",
                    gate.text
                ));
            }
            _ => {}
        }
    }
    overlaps.sort_unstable();
    let mut total = 0;
    let mut reach = start;
    for (from, to) in overlaps {
        let from = from.max(reach);
        if to > from {
            total += to - from;
            reach = to;
        }
    }
    Span::Known(total, ELAPSED)
}

struct Derived {
    before_first: Option<Segment>,
    cycles: Vec<Cycle>,
    gates: Vec<Gate>,
}

fn state_for(stage: &str, paused: bool) -> &'static str {
    if stage == "done" {
        "idle"
    } else if paused {
        "paused"
    } else {
        "active"
    }
}

fn close(mut segment: Segment, end: Time, ongoing: bool, gates: &[Gate], at: u64) -> Segment {
    segment.elapsed = Span::between(&segment.start, &end, ongoing);
    segment.end = end;
    segment.ongoing = ongoing;
    if segment.state == "active" {
        let waiting = match (segment.start.known(), segment.end.known(), &segment.elapsed) {
            (Some(start), Some(end), Span::Known(..)) => waiting_in(start, end, gates, at),
            (_, _, Span::Unknown(reason)) => Span::Unknown(reason.clone()),
            _ => Span::Unknown("unknown segment time".into()),
        };
        segment.active = Some(match (&segment.elapsed, &waiting) {
            (Span::Known(elapsed, label), Span::Known(waiting, _)) => {
                Span::Known(elapsed.saturating_sub(*waiting), label)
            }
            (Span::Unknown(reason), _) | (_, Span::Unknown(reason)) => {
                Span::Unknown(reason.clone())
            }
        });
        segment.waiting = Some(waiting);
    }
    segment
}

fn derive(transitions: &Transitions, session: &Session, at: u64) -> Derived {
    let (awaiting, current, paused) = match session {
        Session::V2(checkpoint) => (
            checkpoint.awaiting_human.clone(),
            checkpoint.stage.current.clone(),
            checkpoint.stage.status == "paused",
        ),
    };
    let gates = gates(&transitions.records, &awaiting);
    let moves: Vec<&Record> = transitions
        .records
        .iter()
        .filter(|record| stage::STAGE_KINDS.contains(&record.kind.as_str()))
        .collect();
    let blank = |stage: &str, paused: bool, start: Time| Segment {
        stage: stage.into(),
        state: state_for(stage, paused),
        side: stage::is_side(stage),
        iteration: 1,
        visit: 1,
        entered_by: None,
        start,
        end: Time::Unknown(String::new()),
        ongoing: false,
        elapsed: Span::Unknown(String::new()),
        waiting: None,
        active: None,
        backfilled: false,
    };
    let Some(first) = moves.first() else {
        // No stage record yet: the current stage since an unrecorded start.
        let open = blank(&current, paused, Time::Unknown(NO_FIRST.into()));
        return Derived {
            before_first: Some(close(open, Time::Known(at), true, &gates, at)),
            cycles: vec![],
            gates,
        };
    };
    let mut open = blank(
        &first.from,
        first.kind == "resume",
        Time::Unknown(NO_FIRST.into()),
    );
    let mut before_first = None;
    let mut cycles: Vec<Cycle> = vec![];
    let mut visits: BTreeMap<String, u32> = BTreeMap::new();
    for (index, record) in moves.iter().enumerate() {
        let mut ended = close(open, record.at.clone(), false, &gates, at);
        ended.backfilled |= record.backfill.is_some();
        if index == 0 {
            before_first = Some(ended);
        } else {
            cycles
                .last_mut()
                .expect("a cycle starts at the first record")
                .segments
                .push(ended);
        }
        if index == 0 || record.kind == "reset" {
            cycles.push(Cycle {
                number: cycles.len() + 1,
                start: record.at.clone(),
                segments: vec![],
                completed: false,
            });
            visits.clear();
        }
        let previous = cycles
            .last()
            .and_then(|cycle| cycle.segments.last())
            .or(before_first.as_ref());
        let previous_iteration = previous.map_or(1, |segment| segment.iteration);
        let previous_visit = previous.map_or(1, |segment| segment.visit);
        let continuing = matches!(record.kind.as_str(), "pause" | "resume" | "iterate")
            && previous.is_some_and(|segment| segment.stage == record.to);
        let (iteration, visit) = if continuing {
            let iteration = previous_iteration + u32::from(record.kind == "iterate");
            // A stage continued from before this cycle's first record is its first visit.
            let visit = *visits.entry(record.to.clone()).or_insert(previous_visit);
            (iteration, visit)
        } else {
            let visit = visits.entry(record.to.clone()).or_insert(0);
            *visit += 1;
            (1, *visit)
        };
        open = blank(&record.to, record.kind == "pause", record.at.clone());
        open.iteration = iteration;
        open.visit = visit;
        open.entered_by = Some(record.kind.clone());
        open.backfilled = record.backfill.is_some();
    }
    let last = close(open, Time::Known(at), true, &gates, at);
    cycles
        .last_mut()
        .expect("a cycle starts at the first record")
        .segments
        .push(last);
    let count = cycles.len();
    for cycle in &mut cycles {
        cycle.completed = cycle.number < count
            || cycle
                .segments
                .last()
                .is_some_and(|segment| segment.state == "idle");
    }
    Derived {
        before_first,
        cycles,
        gates,
    }
}

/// Per-stage totals in one cycle, in order of first visit.
struct StageTotal {
    stage: String,
    side: bool,
    visits: u32,
    iterations: usize,
    stage_time: Span,
    active: Span,
    waiting: Span,
    paused: Span,
    /// Stage time in segments a backfilled move bounds.
    backfilled: Span,
}

fn stage_totals(cycle: &Cycle) -> Vec<StageTotal> {
    let mut order: Vec<String> = vec![];
    for segment in &cycle.segments {
        if segment.state != "idle" && !order.contains(&segment.stage) {
            order.push(segment.stage.clone());
        }
    }
    order
        .into_iter()
        .map(|name| {
            let of: Vec<&Segment> = cycle
                .segments
                .iter()
                .filter(|segment| segment.stage == name && segment.state != "idle")
                .collect();
            let working: Vec<&Segment> = of
                .iter()
                .copied()
                .filter(|segment| segment.state == "active")
                .collect();
            let none = Span::Unknown("not applicable".into());
            StageTotal {
                side: stage::is_side(&name),
                visits: of.iter().map(|segment| segment.visit).max().unwrap_or(0),
                iterations: of
                    .iter()
                    .filter(|segment| segment.entered_by.as_deref() == Some("iterate"))
                    .count(),
                stage_time: Span::sum(working.iter().map(|segment| &segment.elapsed)),
                active: Span::sum(
                    working
                        .iter()
                        .map(|segment| segment.active.as_ref().unwrap_or(&none)),
                ),
                waiting: Span::sum(
                    working
                        .iter()
                        .map(|segment| segment.waiting.as_ref().unwrap_or(&none)),
                ),
                paused: Span::sum(
                    of.iter()
                        .filter(|segment| segment.state == "paused")
                        .map(|segment| &segment.elapsed),
                ),
                backfilled: Span::sum(
                    working
                        .iter()
                        .filter(|segment| segment.backfilled)
                        .map(|segment| &segment.elapsed),
                ),
                stage: name,
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Estimates from history.

fn estimate(samples: &mut [u64], items: &str) -> Value {
    let count = samples.len();
    let noun = if count == 1 {
        items.strip_suffix('s').unwrap_or(items)
    } else {
        items
    };
    if count < MIN_SAMPLES {
        return json!({
            "samples": count,
            "min": null,
            "median": null,
            "max": null,
            "label": format!("too few to estimate ({count} recorded {noun}; at least {MIN_SAMPLES} needed)"),
        });
    }
    samples.sort_unstable();
    let median = if count % 2 == 1 {
        samples[count / 2]
    } else {
        (samples[count / 2 - 1] + samples[count / 2]) / 2
    };
    json!({
        "samples": count,
        "min": samples[0],
        "median": median,
        "max": samples[count - 1],
        "label": format!("estimate from {count} recorded {items}; not a target"),
    })
}

fn estimate_text(value: &Value) -> String {
    match (
        value["min"].as_u64(),
        value["median"].as_u64(),
        value["max"].as_u64(),
    ) {
        (Some(min), Some(median), Some(max)) => format!(
            "{}: min {}, median {}, max {} (elapsed wall-clock)",
            value["label"].as_str().unwrap_or_default(),
            clock(min),
            clock(median),
            clock(max)
        ),
        _ => value["label"].as_str().unwrap_or_default().to_string(),
    }
}

// ---------------------------------------------------------------------------
// Tests, validation runs and bugs.

fn outcome_word(outcome: &Outcome) -> &'static str {
    match outcome {
        Outcome::Passed => "passed",
        Outcome::Failed => "failed",
        Outcome::NotRun => "not-run",
    }
}

/// One case run: from an open session's evidence entry, an archived story's or a bug's
/// receipt (REQ-reporting-001, REQ-cli-017), or, until plan step 4's writers land, a
/// 0.5.x observation.
struct Run {
    id: String,
    cases: Vec<String>,
    environment: String,
    result: String,
    observed: Option<u64>,
    duration: Option<u64>,
    found_in: String,
}

/// A front matter record's scalars as `(key, value)` pairs.
fn scalars(record: &Front) -> Vec<(String, String)> {
    record
        .keys()
        .into_iter()
        .filter_map(|key| Some((key.to_string(), record.scalar(key)?.to_string())))
        .collect()
}

/// A run from an entry's or a receipt's fields; None without an `id`.
fn run_from(fields: &[(String, String)], found_in: String) -> Option<Run> {
    let get = |key: &str| {
        fields
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.clone())
            .filter(|value| !value.is_empty())
    };
    Some(Run {
        id: get("id")?,
        cases: get("case").into_iter().collect(),
        environment: get("environment").unwrap_or_default(),
        result: get("result").unwrap_or_else(|| "unknown".into()),
        observed: get("observed").and_then(|text| parse_time(&text).ok()),
        duration: get("duration_seconds").and_then(|text| text.parse().ok()),
        found_in,
    })
}

/// Every case run: open sessions' entries, then archived stories' and bugs' receipts.
/// The same run read twice, such as one receipt kept by a story and a bug, counts once.
fn case_runs(root: &Path, stories: &[Story], partial: &mut Vec<Value>) -> Vec<Run> {
    let mut runs = vec![];
    for session in crate::sessions::open_ids(root).unwrap_or_default() {
        let dir = format!("{}/{session}/evidence", crate::sessions::DIR);
        for (_, record) in bugs::records_in(root, &dir, Kind::Evidence.bound()) {
            runs.extend(run_from(&scalars(&record), format!("entry in {session}")));
        }
    }
    for story in stories {
        for map in story.record.maps("receipts") {
            runs.extend(run_from(&map, format!("receipt in {}", story.id)));
        }
    }
    match bugs::receipts(root) {
        Ok(receipts) => {
            for (bug, map) in receipts {
                runs.extend(run_from(&map, format!("receipt in bug {bug}")));
            }
        }
        Err(error) => partial.push(json!({"source": bugs::BUGS_DIR, "reason": format!("bugs unreadable, so their receipts are not shown: {error:#}")})),
    }
    // Transitional: `cases run` and `evidence add` still write 0.5.x observations in this
    // build. Plan step 4's evidence reader replaces this at integration.
    match evidence::load_all(root) {
        Ok(records) => runs.extend(records.into_iter().map(|record| Run {
            cases: record.cases.iter().map(|case| case.id.clone()).collect(),
            environment: record.environment,
            result: outcome_word(&record.outcome).to_string(),
            observed: Some(record.observed_at_unix_seconds),
            duration: record.duration_seconds,
            found_in: "observation".into(),
            id: record.id,
        })),
        Err(error) => {
            partial.push(json!({"source": evidence::EVIDENCE_DIR, "reason": format!("observations unreadable, so none are shown: {error:#}")}));
        }
    }
    let mut seen = BTreeSet::new();
    runs.retain(|run| seen.insert((run.id.clone(), run.observed, run.cases.clone())));
    runs
}

fn tests_section(runs: &[Run], case: Option<&str>) -> Vec<Value> {
    let mut groups: BTreeMap<(Option<String>, String), Vec<&Run>> = BTreeMap::new();
    for run in runs {
        let mut keys: Vec<Option<String>> = run.cases.iter().cloned().map(Some).collect();
        if keys.is_empty() {
            keys.push(None);
        }
        for key in keys {
            groups
                .entry((key, run.environment.clone()))
                .or_default()
                .push(run);
        }
    }
    groups
        .into_iter()
        .filter(|((id, _), _)| case.is_none() || id.as_deref() == case)
        .map(|((id, environment), mut runs)| {
            // Known times in order; a run without an observed time last, never at zero.
            runs.sort_by(|a, b| {
                (a.observed.is_none(), a.observed, &a.id)
                    .cmp(&(b.observed.is_none(), b.observed, &b.id))
            });
            let observations: Vec<Value> = runs
                .iter()
                .map(|run| {
                    let run_time = run.duration.map_or_else(
                        || Span::Unknown("no run time recorded".into()),
                        // Whole seconds: a run under one second is recorded together.
                        |seconds| Span::Known(seconds, if seconds == 0 { TOGETHER } else { ELAPSED }),
                    );
                    json!({
                        "id": run.id,
                        "outcome": run.result,
                        "observed": Time::of(run.observed, "no observed time").to_json(),
                        "run_time": run_time.to_json(),
                        "found_in": run.found_in,
                    })
                })
                .collect();
            json!({"case": id, "environment": environment, "count": observations.len(), "observations": observations})
        })
        .collect()
}

fn validation_section(root: &Path, case: Option<&str>, partial: &mut Vec<Value>) -> Vec<Value> {
    let runs = match validate::run_times(root) {
        Ok(runs) => runs,
        Err(error) => {
            partial.push(json!({"source": ".chough/validation", "reason": format!("validation runs unreadable, so none are shown: {error:#}")}));
            return vec![];
        }
    };
    runs.into_iter()
        .filter(|run| case.is_none_or(|case| run.case == case))
        .map(|run| {
            json!({
                "id": run.id,
                "case": run.case,
                "environment": run.environment,
                "state": run.state,
                "started": Time::Known(run.started_at_unix_seconds).to_json(),
                "completed": run.completed,
                "failures": run.failures,
                "not_run": run.not_run,
                "attempts": run.attempts,
                "run_wall_clock": {"seconds": run.elapsed_seconds, "label": RUN_WALL_CLOCK, "reason": null},
            })
        })
        .collect()
}

struct BugRow {
    value: Value,
    fix_time: Option<u64>,
}

/// Each bug's creation, reproduction and verification, from its linked entries and
/// receipts and its Log (REQ-reporting-001); no retention ledger is read.
fn bug_rows(root: &Path, only: Option<&str>, partial: &mut Vec<Value>) -> Vec<BugRow> {
    let bugs = match bugs::fix_times(root) {
        Ok(bugs) => bugs,
        Err(error) => {
            partial.push(json!({"source": bugs::BUGS_DIR, "reason": format!("bugs unreadable, so none are shown: {error:#}")}));
            return vec![];
        }
    };
    bugs.into_iter()
        .filter(|bug| only.is_none_or(|only| bug.names.iter().any(|name| name == only)))
        .map(|bug| {
            let (reproduced, reproduced_source) = match (bug.unavailable, bug.reproduced) {
                (true, _) => (Time::Unknown("unavailable".into()), None),
                (false, Some((at, source))) => (Time::Known(at), Some(source)),
                (false, None) => (Time::Unknown("no linked failed observation".into()), None),
            };
            let verified = Time::of(bug.verified, "no Log entry records status -> verified");
            let fix_time = Span::between(&reproduced, &verified, false);
            BugRow {
                fix_time: fix_time.seconds(),
                value: json!({
                    "id": bug.id,
                    "status": bug.status,
                    "created": Time::of(bug.created, "no readable created time").to_json(),
                    "reproduced": reproduced.to_json(),
                    "reproduced_source": reproduced_source,
                    "verified": verified.to_json(),
                    "verified_source": verified.known().map(|_| "from the bug Log"),
                    "fix_time": fix_time.to_json(),
                }),
            }
        })
        .collect()
}

// ---------------------------------------------------------------------------
// Native lifecycle times, read only.

/// Read one native record file within SpecSync's own 4 MiB artifact bound, never
/// Chough's 64 KiB record bound (LIFE-15); its parsed JSON, or why not.
fn native_record(root: &Path, relative: &str) -> Result<Value, String> {
    let path = local_path(root, relative).map_err(|_| "record unreadable".to_string())?;
    if !path.is_file() {
        return Err("no native record".into());
    }
    let text = read_native_text(&path).map_err(|error| {
        if error.to_string().contains("exceeds") {
            "record over its bound".to_string()
        } else {
            "record unreadable".to_string()
        }
    })?;
    serde_json::from_str(&text).map_err(|_| "unrecognised record shape".into())
}

fn record_time(root: &Path, directory: &str, name: &str) -> Time {
    match native_record(root, &format!("{directory}/{name}")) {
        Ok(value) => Time::of(
            value.get("timestamp").and_then(Value::as_u64),
            "unrecognised record shape",
        ),
        Err(reason) => Time::Unknown(reason),
    }
}

/// One change's native times: created and updated from status; approvals (gate and
/// timestamp only), reopening timestamps, verification, review and finalization from
/// its record files. Actor, note, reviewer and every other field are never read.
/// The whole-report native budget: CHOUGH_TIMING_NATIVE_BUDGET_SECONDS, a whole
/// number of seconds from 1 to 3600, 120 when unset. An invalid value is refused.
fn native_budget() -> Result<Duration> {
    let Some(value) = std::env::var_os(TIMING_BUDGET_VARIABLE) else {
        return Ok(Duration::from_secs(NATIVE_BUDGET));
    };
    match value
        .to_str()
        .and_then(|text| text.parse::<u64>().ok())
        .filter(|seconds| (1..=3_600).contains(seconds))
    {
        Some(seconds) => Ok(Duration::from_secs(seconds)),
        None => bail!(
            "{TIMING_BUDGET_VARIABLE} must be a whole number of seconds from 1 to 3600; got {:?}, so no native command was run",
            value.to_string_lossy().chars().take(64).collect::<String>()
        ),
    }
}

/// The native reads of one report: each change ID is read at most once, and every
/// read shares one budget. Nothing outlives the report.
struct NativeReads {
    budget: Duration,
    started: Instant,
    responses: BTreeMap<String, NativeJson>,
}

/// How a change's status was obtained for the report.
enum Fetch {
    Read,
    Reused,
    BudgetExhausted,
}

impl NativeReads {
    fn new(budget: Duration) -> Self {
        Self {
            budget,
            started: Instant::now(),
            responses: BTreeMap::new(),
        }
    }

    fn fetch(&mut self, root: &Path, id: &str) -> Fetch {
        if self.responses.contains_key(id) {
            return Fetch::Reused;
        }
        let remaining = self.budget.saturating_sub(self.started.elapsed());
        if remaining.is_zero() {
            return Fetch::BudgetExhausted;
        }
        let response = native_json_within(
            root,
            "specsync",
            &["change", "status", id, "--json"],
            Some(remaining),
        );
        self.responses.insert(id.to_string(), response);
        Fetch::Read
    }
}

/// One change for the report; `response` is None when the budget left no time to read.
fn native_change(
    root: &Path,
    id: &str,
    directory: Option<&str>,
    response: Option<&NativeJson>,
) -> Value {
    let (status, unavailable, status_read, not_read_reason) = match response {
        Some(response) => match &response.value {
            Some(value) => (Some(value), None, response.section.state.clone(), None),
            None => (
                None,
                Some(format!("native read {}", response.section.state)),
                response.section.state.clone(),
                None,
            ),
        },
        None => (
            None,
            Some(format!(
                "native read not-read: the timing native budget ({TIMING_BUDGET_VARIABLE}) was exhausted"
            )),
            "not-read".to_string(),
            Some("budget-exhausted"),
        ),
    };
    let status_time = |key: &str| match (&status, &unavailable) {
        (Some(value), _) => Time::of(
            value
                .pointer(&format!("/change/{key}"))
                .and_then(Value::as_u64),
            "native status does not expose this time",
        ),
        (None, reason) => Time::Unknown(reason.clone().unwrap_or_default()),
    };
    let approval_valid = status
        .and_then(|value| value.pointer("/summary/approval_valid"))
        .and_then(Value::as_bool);
    // A change the budget left no time for is reported with every time unknown: its
    // local record files are not read either, so nothing implies a status read.
    if let Some(reason) = not_read_reason {
        let unknown = Time::Unknown(unavailable.clone().unwrap_or_default()).to_json();
        return json!({
            "id": id,
            "status_read": status_read,
            "not_read_reason": reason,
            "created": unknown,
            "updated": unknown,
            "approvals": [],
            "approvals_unknown": unavailable,
            "reopenings": [],
            "verification": unknown,
            "review": unknown,
            "finalization": unknown,
        });
    }
    let Some(directory) = directory else {
        let missing = Time::Unknown("no native record".into()).to_json();
        return json!({
            "id": id,
            "status_read": status_read,
            "not_read_reason": not_read_reason,
            "created": status_time("created_at").to_json(),
            "updated": status_time("updated_at").to_json(),
            "approvals": [],
            "approvals_unknown": "no native record",
            "reopenings": [],
            "verification": missing,
            "review": missing,
            "finalization": missing,
        });
    };
    let mut approvals_unknown: Option<String> = None;
    let mut approvals: Vec<(String, u64)> = vec![];
    let mut reopenings: Vec<u64> = vec![];
    match native_record(root, &format!("{directory}/approvals.json")) {
        Err(reason) => approvals_unknown = Some(reason),
        Ok(value) => {
            let entries = value.get("approvals").and_then(Value::as_array);
            let parsed: Option<Vec<(String, u64)>> = entries.and_then(|entries| {
                entries
                    .iter()
                    .map(|entry| {
                        Some((
                            entry.get("gate")?.as_str()?.to_string(),
                            entry.get("timestamp")?.as_u64()?,
                        ))
                    })
                    .collect()
            });
            let reopened: Option<Vec<u64>> = match value.get("reopenings") {
                None => Some(vec![]),
                Some(list) => list.as_array().and_then(|list| {
                    list.iter()
                        .map(|entry| entry.get("timestamp")?.as_u64())
                        .collect()
                }),
            };
            match (parsed, reopened) {
                (Some(parsed), Some(reopened)) => {
                    approvals = parsed;
                    reopenings = reopened;
                }
                _ => approvals_unknown = Some("unrecognised record shape".into()),
            }
        }
    }
    let standings: Vec<Value> = approvals
        .iter()
        .enumerate()
        .map(|(index, (gate, timestamp))| {
            let superseded = approvals[index + 1..]
                .iter()
                .any(|(later, at)| later == gate && at >= timestamp);
            // A native reopening supersedes the latest terminal approval (acceptance or
            // finalization); a definition approval keeps the validity status reports.
            let terminal = matches!(gate.as_str(), "acceptance" | "finalization");
            let (standing, reason) = if terminal && reopenings.iter().any(|at| at > timestamp) {
                ("reopened", "a later native reopening supersedes it".to_string())
            } else if superseded {
                ("not current", "a later approval of the same gate supersedes it".to_string())
            } else if let Some(reason) = &unavailable {
                ("unknown", format!("validity comes only from native status: {reason}"))
            } else if gate == "definition" {
                match approval_valid {
                    Some(true) => ("current", "native status reports approval_valid true".into()),
                    Some(false) => ("not current", "native status reports approval_valid false".into()),
                    None => ("not current", "native status does not report approval_valid".into()),
                }
            } else {
                ("not reported", "native status reports no validity for this gate".into())
            };
            json!({"gate": gate, "at": Time::Known(*timestamp).to_json(), "standing": standing, "reason": reason})
        })
        .collect();
    json!({
        "id": id,
        "status_read": status_read,
        "not_read_reason": not_read_reason,
        "created": status_time("created_at").to_json(),
        "updated": status_time("updated_at").to_json(),
        "approvals": standings,
        "approvals_unknown": approvals_unknown,
        "reopenings": reopenings.iter().map(|at| Time::Known(*at).to_json()).collect::<Vec<_>>(),
        "verification": record_time(root, directory, "verification.json").to_json(),
        "review": record_time(root, directory, "review.json").to_json(),
        "finalization": record_time(root, directory, "finalization.json").to_json(),
    })
}

fn plain_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 255
        && !id.starts_with('-')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_'))
}

/// Archive directory names, without links: most recent archive date first, and on the
/// same date most recent finalization first (an unknown finalization time after known
/// ones, then by name), so the history bound keeps the most recent changes.
fn archived(root: &Path) -> Result<Vec<String>, String> {
    let path = local_path(root, ARCHIVE_DIR).map_err(|error| error.to_string())?;
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut names = vec![];
    for entry in fs::read_dir(&path).map_err(|error| format!("{ARCHIVE_DIR}: {error}"))? {
        let entry = entry.map_err(|error| format!("{ARCHIVE_DIR}: {error}"))?;
        let kind = entry
            .file_type()
            .map_err(|error| format!("{ARCHIVE_DIR}: {error}"))?;
        if kind.is_dir()
            && !kind.is_symlink()
            && let Some(name) = entry.file_name().to_str()
        {
            names.push(name.to_string());
        }
    }
    // Descending: date, then finalization time (Some before None), then name.
    let mut keyed: Vec<(String, Option<u64>, String)> = names
        .into_iter()
        .map(|name| {
            let dated = archived_id(&name).len() < name.len();
            let date = if dated {
                name[..10].to_string()
            } else {
                String::new()
            };
            let finalized =
                record_time(root, &format!("{ARCHIVE_DIR}/{name}"), "finalization.json").known();
            (date, finalized, name)
        })
        .collect();
    keyed.sort_unstable_by(|a, b| b.cmp(a));
    Ok(keyed.into_iter().map(|(_, _, name)| name).collect())
}

/// The change ID of an archive directory named `<YYYY-MM-DD>-<id>`.
fn archived_id(directory: &str) -> &str {
    let bytes = directory.as_bytes();
    let dated = bytes.len() > 11
        && bytes[..10].iter().enumerate().all(|(index, byte)| {
            if matches!(index, 4 | 7) {
                *byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
        && bytes[10] == b'-';
    if dated { &directory[11..] } else { directory }
}

/// The native section. Archived history is read only when `excluded` (the filters that
/// hide it) is empty; the checkpoint change is always read, through `reads`.
fn native_section(
    root: &Path,
    session: &Session,
    partial: &mut Vec<Value>,
    excluded: &[String],
    reads: &mut NativeReads,
) -> Result<Value> {
    if !local_path(root, ".specsync/config.toml")?.is_file()
        && !local_path(root, ".specsync/config.json")?.is_file()
    {
        return Ok(json!({
            "state": "not-configured",
            "detail": "No native SpecSync configuration; native lifecycle times are unknown.",
            "current": null,
            "history": [],
            "history_selection": {"state": "not-configured", "filters": excluded},
            "skipped_archived": 0,
        }));
    }
    let current = match session {
        Session::V2(checkpoint) => checkpoint.native_change.as_deref().map(|id| {
            let active = format!("{CHANGES_DIR}/{id}");
            let directory = if local_path(root, &active).is_ok_and(|path| path.is_dir()) {
                Some(active)
            } else {
                archived(root).ok().and_then(|names| {
                    names
                        .into_iter()
                        .find(|name| archived_id(name) == id)
                        .map(|name| format!("{ARCHIVE_DIR}/{name}"))
                })
            };
            reads.fetch(root, id);
            native_change(root, id, directory.as_deref(), reads.responses.get(id))
        }),
    };
    if !excluded.is_empty() {
        partial.push(json!({"source": ARCHIVE_DIR, "reason": format!("archived native history not read: excluded by {}", excluded.join(", "))}));
        return Ok(json!({
            "state": "observed",
            "detail": "Times from `specsync change status --json` (created, updated) and native record files read read-only (gate and timestamp only); approval validity only from native status.",
            "current": current,
            "history": null,
            "history_selection": {"state": "skipped-by-filter", "filters": excluded, "reason": HISTORY_RULE},
            "skipped_archived": null,
        }));
    }
    let mut history = vec![];
    let mut skipped = 0;
    let (mut read, mut reused, mut not_read_budget) = (0, 0, 0);
    match archived(root) {
        Err(reason) => partial.push(json!({"source": ARCHIVE_DIR, "reason": format!("archived changes unreadable: {reason}")})),
        Ok(names) => {
            skipped = names.len().saturating_sub(ARCHIVE_HISTORY);
            for name in names.into_iter().take(ARCHIVE_HISTORY) {
                let id = archived_id(&name).to_string();
                if plain_id(&id) {
                    match reads.fetch(root, &id) {
                        Fetch::Read => read += 1,
                        Fetch::Reused => reused += 1,
                        Fetch::BudgetExhausted => not_read_budget += 1,
                    }
                    history.push(native_change(
                        root,
                        &id,
                        Some(&format!("{ARCHIVE_DIR}/{name}")),
                        reads.responses.get(&id),
                    ));
                } else {
                    let unknown = Time::Unknown("unexpected archive directory name".into()).to_json();
                    history.push(json!({"id": name, "status_read": "not-read", "not_read_reason": "unexpected-name", "created": unknown, "updated": unknown, "approvals": [], "approvals_unknown": "unexpected archive directory name", "reopenings": [], "verification": unknown, "review": unknown, "finalization": unknown}));
                }
            }
            if skipped > 0 {
                partial.push(json!({"source": ARCHIVE_DIR, "reason": format!("{skipped} older archived changes skipped; at most {ARCHIVE_HISTORY} are read")}));
            }
            if not_read_budget > 0 {
                partial.push(json!({"source": ARCHIVE_DIR, "reason": format!("{not_read_budget} archived changes not read: the {} s timing native budget ({TIMING_BUDGET_VARIABLE}) was exhausted", reads.budget.as_secs())}));
            }
        }
    }
    Ok(json!({
        "state": "observed",
        "detail": "Times from `specsync change status --json` (created, updated) and native record files read read-only (gate and timestamp only); approval validity only from native status.",
        "current": current,
        "history": history,
        "history_selection": {
            "state": "selected",
            "filters": excluded,
            "read": read,
            "reused": reused,
            "skipped_beyond_bound": skipped,
            "not_read_budget": not_read_budget,
            "budget_seconds": reads.budget.as_secs(),
        },
        "skipped_archived": skipped,
    }))
}

fn native_text(change: &Value) -> Vec<String> {
    let time = |value: &Value| match (value["utc"].as_str(), value["unknown"].as_str()) {
        (Some(utc), _) => utc.to_string(),
        (_, Some(reason)) => format!("unknown ({reason})"),
        _ => "unknown".into(),
    };
    let mut lines = vec![format!(
        "- {}: created {}, updated {}, verification {}, review {}, finalization {}",
        change["id"].as_str().unwrap_or_default(),
        time(&change["created"]),
        time(&change["updated"]),
        time(&change["verification"]),
        time(&change["review"]),
        time(&change["finalization"]),
    )];
    if let Some(reason) = change["approvals_unknown"].as_str() {
        lines.push(format!("  - approvals unknown ({reason})"));
    }
    for approval in change["approvals"].as_array().into_iter().flatten() {
        lines.push(format!(
            "  - {} approval at {}: {} ({})",
            approval["gate"].as_str().unwrap_or_default(),
            time(&approval["at"]),
            approval["standing"].as_str().unwrap_or_default(),
            approval["reason"].as_str().unwrap_or_default(),
        ));
    }
    for reopening in change["reopenings"].as_array().into_iter().flatten() {
        lines.push(format!("  - reopened at {}", time(reopening)));
    }
    lines
}

// ---------------------------------------------------------------------------
// Every session's timeline: open sessions and archived stories.

/// An archived session story: its record, for its receipts, and its checkpoint, for its
/// timeline, when that reads.
struct Story {
    id: String,
    record: Front,
    session: Option<Session>,
}

/// Archived stories, newest name first, within the timing byte bound
/// (REQ-reporting-001, REQ-history-003). A story that cannot be read is named in
/// `partial`, and so is the count of older stories beyond the bound.
fn read_stories(root: &Path, partial: &mut Vec<Value>) -> Vec<Story> {
    let Ok(folder) = local_path(root, STORIES_DIR) else {
        return vec![];
    };
    let mut names: Vec<String> = match fs::read_dir(&folder) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
            .filter_map(|entry| entry.file_name().into_string().ok())
            .filter(|name| name.ends_with(".md") && !name.starts_with('.'))
            .collect(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return vec![],
        Err(error) => {
            partial.push(json!({"source": STORIES_DIR, "reason": format!("archived stories unreadable, so none are shown: {error}")}));
            return vec![];
        }
    };
    // Dated names: the newest sort last, and are read first.
    names.sort_unstable_by(|left, right| right.cmp(left));
    let (mut stories, mut bytes, mut skipped) = (vec![], 0_u64, 0_usize);
    for name in names {
        let relative = format!("{STORIES_DIR}/{name}");
        let path = folder.join(&name);
        let size = fs::metadata(&path).map_or(0, |metadata| metadata.len());
        if bytes + size > TRANSITIONS_LIMIT {
            skipped += 1;
            continue;
        }
        bytes += size;
        let read = read_bounded_record(&path, Kind::Story.bound())
            .and_then(|text| Ok((markdown::parse(&relative, &text)?, text)));
        let (record, text) = match read {
            Ok(read) => read,
            Err(error) => {
                partial.push(json!({"source": relative, "reason": format!("story unreadable, so it is not shown: {error:#}")}));
                continue;
            }
        };
        let session = match crate::checkpoint::parse(&relative, &text) {
            Ok(session) => Some(session),
            Err(error) => {
                partial.push(json!({"source": relative, "reason": format!("story timeline unreadable, so its stages are not shown: {error:#}")}));
                None
            }
        };
        stories.push(Story {
            id: name.trim_end_matches(".md").to_string(),
            record,
            session,
        });
    }
    if skipped > 0 {
        partial.push(json!({"source": STORIES_DIR, "reason": format!("{skipped} older stories skipped beyond the {TRANSITIONS_LIMIT}-byte bound")}));
    }
    stories
}

/// One session's timeline, derived.
struct SessionTiming {
    id: String,
    state: &'static str,
    stage: String,
    transitions: Transitions,
    derived: Derived,
}

impl SessionTiming {
    fn of(
        root: &Path,
        id: String,
        state: &'static str,
        session: &Session,
        at: u64,
    ) -> Result<Self> {
        let transitions = read_transitions(root, session, true)?;
        let derived = derive(&transitions, session, at);
        let Session::V2(checkpoint) = session;
        Ok(Self {
            id,
            state,
            stage: checkpoint.stage.current.clone(),
            transitions,
            derived,
        })
    }
}

/// Every open session's and every read story's timeline (REQ-reporting-001). A session
/// that cannot be read is named in `partial`, as is anything its timeline read skipped.
fn session_timings(
    root: &Path,
    stories: &[Story],
    selected: &str,
    at: u64,
    partial: &mut Vec<Value>,
) -> Vec<SessionTiming> {
    let mut timings = vec![];
    let mut add = |id: String,
                   state: &'static str,
                   source: String,
                   session: &Session,
                   partial: &mut Vec<Value>| {
        match SessionTiming::of(root, id.clone(), state, session, at) {
            Ok(timing) => {
                // The selected session's skips are reported at the top already.
                if id != selected {
                    for reason in &timing.transitions.history_partial {
                        partial.push(json!({"source": source, "reason": reason}));
                    }
                    if timing.transitions.unreadable > 0 {
                        partial.push(json!({"source": source, "reason": format!("{} unreadable timeline entries skipped", timing.transitions.unreadable)}));
                    }
                }
                timings.push(timing);
            }
            Err(error) => partial.push(json!({"source": source, "reason": format!("timeline unreadable, so its stages are not shown: {error:#}")})),
        }
    };
    for id in crate::sessions::open_ids(root).unwrap_or_default() {
        let relative = crate::sessions::session_file(&id);
        match crate::sessions::read_at(root, &relative) {
            Ok(checkpoint) => add(id, "open", relative, &Session::V2(Box::new(checkpoint)), partial),
            Err(error) => partial.push(json!({"source": relative, "reason": format!("session unreadable, so its stages are not shown: {error:#}")})),
        }
    }
    for story in stories {
        if let Some(session) = &story.session {
            add(
                story.id.clone(),
                "archived",
                format!("{STORIES_DIR}/{}.md", story.id),
                session,
                partial,
            );
        }
    }
    timings
}

/// One cycle as JSON, its segments and stage totals filtered to `stage_filter`.
fn cycle_json(cycle: &Cycle, derived: &Derived, stage_filter: Option<&str>) -> Value {
    let shows = |stage: &str| stage_filter.is_none_or(|filter| stage == filter);
    let next_start = derived
        .cycles
        .iter()
        .find(|other| other.number == cycle.number + 1)
        .map(|other| other.start.to_json());
    json!({
        "number": cycle.number,
        "start": cycle.start.to_json(),
        "end": next_start,
        "completed": cycle.completed,
        "iterations": cycle.segments.iter().filter(|segment| segment.entered_by.as_deref() == Some("iterate")).count(),
        "side_visits": cycle.segments.iter().filter(|segment| segment.side && segment.entered_by.as_deref() == Some("enter")).count(),
        "idle": Span::sum(cycle.segments.iter().filter(|segment| segment.state == "idle").map(|segment| &segment.elapsed)).to_json(),
        "segments": cycle.segments.iter().filter(|segment| shows(&segment.stage)).map(Segment::to_json).collect::<Vec<_>>(),
        "stages": stage_totals(cycle).iter().filter(|total| shows(&total.stage)).map(|total| json!({
            "stage": total.stage,
            "side": total.side,
            "visits": total.visits,
            "iterations": total.iterations,
            "stage_time": total.stage_time.to_json(),
            "active": total.active.to_json(),
            "waiting": total.waiting.to_json(),
            "paused": total.paused.to_json(),
            "backfilled": total.backfilled.to_json(),
        })).collect::<Vec<_>>(),
    })
}

/// One session's stage segments and gate waits, from its own timeline.
fn session_json(
    timing: &SessionTiming,
    selected: &str,
    stage_filter: Option<&str>,
    at: u64,
) -> Value {
    json!({
        "id": timing.id,
        "state": timing.state,
        "selected": timing.id == selected,
        "stage": timing.stage,
        "timeline_entries": timing.transitions.records.len(),
        "unrecognised_kinds": timing.transitions.unrecognised,
        "before_first_record": timing.derived.before_first.as_ref().filter(|segment| stage_filter.is_none_or(|stage| segment.stage == stage)).map(Segment::to_json),
        "cycles": timing.derived.cycles.iter().map(|cycle| cycle_json(cycle, &timing.derived, stage_filter)).collect::<Vec<_>>(),
        "gates": timing.derived.gates.iter().map(|gate| gate.to_json(at)).collect::<Vec<_>>(),
    })
}

// ---------------------------------------------------------------------------
// The report.

pub(super) fn run(root: &Path, args: TimingArgs) -> Result<()> {
    // Refused before any record or native read when invalid (REQ-reporting-002).
    let budget = native_budget()?;
    let phase = Instant::now();
    // With no open session, the newest archived story is the cycle to report (plan step
    // 9 reads every timeline and receipt).
    let session = match read_session(root) {
        Ok(session) => session,
        Err(error) => crate::sessions::newest_archived(root)?.ok_or(error)?,
    };
    let at = now()?;
    let mut partial: Vec<Value> = vec![];
    let transitions = read_transitions(root, &session, args.cycle == "all")?;
    let history_source = "the session timeline";
    for reason in &transitions.history_partial {
        partial.push(json!({"source":history_source,"reason":reason}));
    }
    if transitions.skipped_bytes > 0 {
        partial.push(json!({"source": stage::TRANSITIONS, "reason": format!("earliest {} bytes skipped beyond the {TRANSITIONS_LIMIT}-byte bound", transitions.skipped_bytes)}));
    }
    if transitions.unreadable > 0 {
        partial.push(json!({"source": stage::TRANSITIONS, "reason": format!("{} unreadable lines skipped", transitions.unreadable)}));
    }
    if transitions.unrecognised > 0 {
        partial.push(json!({"source": stage::TRANSITIONS, "reason": format!("{} records of unrecognised kinds skipped", transitions.unrecognised)}));
    }
    let derived = derive(&transitions, &session, at);
    let selected = match &session {
        Session::V2(checkpoint) => checkpoint.meta.id.clone(),
    };
    let stories = read_stories(root, &mut partial);
    let timings = session_timings(root, &stories, &selected, at, &mut partial);
    trace_phase("transitions", phase);
    let stage_filter = args.stage.as_deref();
    let shows = |segment: &Segment| stage_filter.is_none_or(|stage| segment.stage == stage);

    // Estimates use every completed cycle of every session read, whatever the filters
    // show; the selected session counts once.
    let mut history: Vec<&Derived> = timings.iter().map(|timing| &timing.derived).collect();
    if !timings.iter().any(|timing| timing.id == selected) {
        history.push(&derived);
    }
    let mut stage_samples: BTreeMap<String, Vec<u64>> = BTreeMap::new();
    for cycle in history
        .iter()
        .flat_map(|derived| &derived.cycles)
        .filter(|cycle| cycle.completed)
    {
        for total in stage_totals(cycle) {
            // A stage a backfilled move bounds has a time given after the fact.
            if total.backfilled.seconds() != Some(0) {
                continue;
            }
            // Same-second records are written together, not samples.
            if let Some(seconds) = total.stage_time.seconds().filter(|seconds| *seconds > 0) {
                stage_samples.entry(total.stage).or_default().push(seconds);
            }
        }
    }
    let stage_estimates: Vec<Value> = stage::STAGES
        .iter()
        .filter(|(name, _)| stage_filter.is_none_or(|stage| stage == *name))
        .filter_map(|(name, _)| {
            stage_samples.get_mut(*name).map(|samples| {
                let mut value = estimate(samples, &format!("{name} stage times"));
                value["stage"] = json!(name);
                value
            })
        })
        .collect();
    let gate_rows: Vec<Value> = derived.gates.iter().map(|gate| gate.to_json(at)).collect();
    let mut gate_samples: Vec<u64> = history
        .iter()
        .flat_map(|derived| &derived.gates)
        .filter(|gate| !gate.open_now)
        .filter_map(|gate| gate.waiting(at).seconds())
        .filter(|seconds| *seconds > 0)
        .collect();
    let bug_rows = bug_rows(root, args.bug.as_deref(), &mut partial);
    let mut fix_samples: Vec<u64> = bug_rows
        .iter()
        .filter_map(|row| row.fix_time)
        .filter(|seconds| *seconds > 0)
        .collect();
    let phase = Instant::now();
    let runs_read = case_runs(root, &stories, &mut partial);
    let tests = tests_section(&runs_read, args.case.as_deref());
    trace_phase("tests", phase);
    let phase = Instant::now();
    let runs = validation_section(root, args.case.as_deref(), &mut partial);
    trace_phase("validation", phase);
    // Archived history is shown, and so read, only for an unfiltered report.
    let mut excluded = vec![];
    if args.cycle == "current" {
        excluded.push("--cycle current".to_string());
    }
    for (flag, value) in [
        ("--stage", &args.stage),
        ("--case", &args.case),
        ("--bug", &args.bug),
    ] {
        if let Some(value) = value {
            excluded.push(format!("{flag} {value}"));
        }
    }
    let phase = Instant::now();
    let mut reads = NativeReads::new(budget);
    let native = native_section(root, &session, &mut partial, &excluded, &mut reads)?;
    trace_phase("native", phase);

    let shown: Vec<&Cycle> = if args.cycle == "current" {
        derived.cycles.last().into_iter().collect()
    } else {
        derived.cycles.iter().collect()
    };
    let before_first = derived
        .before_first
        .as_ref()
        .filter(|segment| (args.cycle == "all" || derived.cycles.is_empty()) && shows(segment));
    // Every session's own segments and gate waits; `--cycle current` shows the selected
    // session only.
    let sessions: Vec<Value> = timings
        .iter()
        .filter(|timing| args.cycle == "all" || timing.id == selected)
        .map(|timing| session_json(timing, &selected, stage_filter, at))
        .collect();
    let report = json!({
        "schema_version": 1,
        "generated_at": Time::Known(at).to_json(),
        "labels": {
            "durations": "Every duration is elapsed wall-clock time, not effort.",
            "recorded_together": "Start and end share a second, usually records written together; not instant work.",
            "unknown": "A time no record holds is unknown (null in JSON), never zero.",
            "idle": "Time in done until the next reset is idle between cycles: not stage time and not an estimate sample.",
            "waiting": "Waiting on a human is the time a recorded awaiting-human gate was open; it never says who decided or what.",
            "estimates": "Estimates come from recorded history and are not targets; fewer than 3 samples are too few to estimate.",
            "validation": format!("Validation run time is the {RUN_WALL_CLOCK}, not per-test time."),
            "backfilled": BACKFILLED,
        },
        "filters": {"cycle": args.cycle, "stage": args.stage, "case": args.case, "bug": args.bug},
        "partial": partial,
        "transitions": {
            "records": transitions.records.len(),
            "skipped_bytes": transitions.skipped_bytes,
            "unreadable_lines": transitions.unreadable,
            "unrecognised_kinds": transitions.unrecognised,
            "bound_bytes": TRANSITIONS_LIMIT,
            "selected_session": match &session { Session::V2(checkpoint) => Value::String(checkpoint.meta.id.clone()) },
            "lineage_selection": "Only the checkpoint-selected parent chain is derived; unselected branch timelines remain separate and their durations are unknown. Inspect chough history show for other tips.",
        },
        "transition_lineages": transitions.lineages,
        "backfilled": backfilled_moves(&transitions.records),
        "before_first_record": before_first.map(Segment::to_json),
        "cycles": shown.iter().map(|cycle| cycle_json(cycle, &derived, stage_filter)).collect::<Vec<_>>(),
        "sessions": sessions,
        "gates": gate_rows,
        "tests": tests,
        "validation_runs": runs,
        "bugs": bug_rows.iter().map(|row| row.value.clone()).collect::<Vec<_>>(),
        "native": native,
        "estimates": {
            "stages": stage_estimates,
            "bug_fixes": estimate(&mut fix_samples, "bug fix times"),
            "gates": estimate(&mut gate_samples, "human gate waiting times"),
        },
    });
    let phase = Instant::now();
    if args.json {
        print_json(&report)?;
    } else {
        print!(
            "{}",
            render(&report, &derived, &shown, before_first, stage_filter)
        );
    }
    trace_phase("render", phase);
    Ok(())
}

const BACKFILLED: &str = "A backfilled move was recorded after it happened, at the original time it gives, with the commit and quote it rests on as evidence only; it records no human approval or review. Segments it bounds are marked and are not estimate samples.";

/// Every backfilled move, with its original and recording times and its evidence.
fn backfilled_moves(records: &[Record]) -> Vec<Value> {
    records
        .iter()
        .filter_map(|record| {
            let backfill = record.backfill.as_ref()?;
            Some(json!({
                "kind": record.kind,
                "from": record.from,
                "to": record.to,
                "at": record.at.to_json(),
                "recorded_at": Time::of(
                    backfill.get("recorded_at_unix_seconds").and_then(Value::as_u64),
                    "missing recording time",
                ).to_json(),
                "commit": backfill.get("commit"),
                "quote": backfill.get("quote"),
            }))
        })
        .collect()
}

fn time_text(value: &Value) -> String {
    match (value["utc"].as_str(), value["unknown"].as_str()) {
        (Some(utc), _) => utc.to_string(),
        (_, Some(reason)) => format!("unknown ({reason})"),
        _ => "unknown".into(),
    }
}

fn span_text(value: &Value) -> String {
    match (value["seconds"].as_u64(), value["label"].as_str()) {
        (Some(seconds), Some(label)) => format!("{} ({label})", clock(seconds)),
        _ => format!(
            "unknown ({})",
            value["reason"].as_str().unwrap_or("not recorded")
        ),
    }
}

fn render(
    report: &Value,
    derived: &Derived,
    shown: &[&Cycle],
    before_first: Option<&Segment>,
    stage_filter: Option<&str>,
) -> String {
    let mut out = String::from("# Chough timing\n\n");
    out.push_str("Every duration is elapsed wall-clock time, not effort. A time no record holds is unknown, never zero. Estimates come from recorded history and are not targets.\n");
    out.push_str(&format!(
        "Generated {}.\n",
        time_text(&report["generated_at"])
    ));
    for entry in report["partial"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "Partial: {}: {}\n",
            entry["source"].as_str().unwrap_or_default(),
            entry["reason"].as_str().unwrap_or_default()
        ));
    }
    out.push_str("\n## Stages\n\n");
    if let Some(segment) = before_first {
        out.push_str(&format!("Before the first record:\n{}\n", segment.text()));
    }
    if shown.is_empty() && before_first.is_none() {
        out.push_str("No stage transitions recorded.\n");
    }
    for cycle in shown {
        out.push_str(&format!(
            "\n### Cycle {} (started {}, {})\n\n",
            cycle.number,
            cycle.start.text(),
            if cycle.completed {
                "completed"
            } else {
                "current"
            }
        ));
        for segment in cycle
            .segments
            .iter()
            .filter(|segment| stage_filter.is_none_or(|stage| segment.stage == stage))
        {
            out.push_str(&segment.text());
            out.push('\n');
        }
        out.push_str("\nStage totals:\n");
        for total in stage_totals(cycle)
            .iter()
            .filter(|total| stage_filter.is_none_or(|stage| total.stage == stage))
        {
            out.push_str(&format!(
                "- {}{}: {} visit(s), {} iteration(s); stage time {}; active {}; waiting on a human {}; paused {}\n",
                total.stage,
                if total.side { " (side stage)" } else { "" },
                total.visits,
                total.iterations,
                total.stage_time.text(),
                total.active.text(),
                total.waiting.text(),
                total.paused.text()
            ));
        }
        let idle = Span::sum(
            cycle
                .segments
                .iter()
                .filter(|segment| segment.state == "idle")
                .map(|segment| &segment.elapsed),
        );
        out.push_str(&format!("Idle between cycles: {}\n", idle.text()));
    }
    let sessions = report["sessions"].as_array().cloned().unwrap_or_default();
    if sessions.len() > 1 {
        out.push_str(
            "\n## Sessions\n\nEach session's stages and gate waits, from its own timeline.\n\n",
        );
        for session in &sessions {
            let stage_time = |name: &str| -> u64 {
                session["cycles"]
                    .as_array()
                    .into_iter()
                    .flatten()
                    .flat_map(|cycle| cycle["stages"].as_array().cloned().unwrap_or_default())
                    .filter_map(|total| total[name]["seconds"].as_u64())
                    .sum()
            };
            out.push_str(&format!(
                "- {} ({}{}), at {}: {} timeline entries; known stage time {}, waiting on a human {}; {} gate(s)\n",
                session["id"].as_str().unwrap_or_default(),
                session["state"].as_str().unwrap_or_default(),
                if session["selected"] == true { ", shown above" } else { "" },
                session["stage"].as_str().unwrap_or_default(),
                session["timeline_entries"],
                clock(stage_time("stage_time")),
                clock(stage_time("waiting")),
                session["gates"].as_array().map_or(0, Vec::len),
            ));
        }
    }
    let backfilled = report["backfilled"].as_array().cloned().unwrap_or_default();
    if !backfilled.is_empty() {
        out.push_str(&format!("\n## Backfilled moves\n\n{BACKFILLED}\n\n"));
        for entry in &backfilled {
            out.push_str(&format!(
                "- {} -> {} at {}, recorded {}, commit {}: \"{}\"\n",
                entry["from"].as_str().unwrap_or_default(),
                entry["to"].as_str().unwrap_or_default(),
                time_text(&entry["at"]),
                time_text(&entry["recorded_at"]),
                entry["commit"].as_str().unwrap_or("unknown"),
                entry["quote"].as_str().unwrap_or_default()
            ));
        }
    }
    out.push_str("\n## Human gates\n\n");
    if derived.gates.is_empty() {
        out.push_str("No awaiting-human gates recorded.\n");
    }
    for gate in report["gates"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "- \"{}\" ({}): opened {}, closed {}, waiting {}\n",
            gate["gate"].as_str().unwrap_or_default(),
            gate["state"].as_str().unwrap_or_default(),
            time_text(&gate["opened"]),
            if gate["closed"].is_null() {
                "not yet".to_string()
            } else {
                time_text(&gate["closed"])
            },
            span_text(&gate["waiting"])
        ));
    }
    out.push_str("\n## Tests\n\n");
    if report["tests"].as_array().is_none_or(Vec::is_empty) {
        out.push_str("No observations recorded.\n");
    }
    for group in report["tests"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "- {} in {}: {} observation(s)\n",
            group["case"].as_str().unwrap_or("(no case)"),
            group["environment"].as_str().unwrap_or_default(),
            group["count"]
        ));
        for observation in group["observations"].as_array().into_iter().flatten() {
            out.push_str(&format!(
                "  - {} {} at {}; run time {}\n",
                observation["id"].as_str().unwrap_or_default(),
                observation["outcome"].as_str().unwrap_or_default(),
                time_text(&observation["observed"]),
                span_text(&observation["run_time"])
            ));
        }
    }
    out.push_str("\n## Validation runs\n\n");
    if report["validation_runs"]
        .as_array()
        .is_none_or(Vec::is_empty)
    {
        out.push_str("No validation runs recorded.\n");
    }
    for run in report["validation_runs"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "- {} ({} in {}, {}): started {}; {} completed, {} failed, {} not run, {} attempt(s) recorded; {}\n",
            run["id"].as_str().unwrap_or_default(),
            run["case"].as_str().unwrap_or_default(),
            run["environment"].as_str().unwrap_or_default(),
            run["state"].as_str().unwrap_or_default(),
            time_text(&run["started"]),
            run["completed"],
            run["failures"],
            run["not_run"],
            run["attempts"],
            span_text(&run["run_wall_clock"])
        ));
    }
    out.push_str("\n## Bugs\n\n");
    if report["bugs"].as_array().is_none_or(Vec::is_empty) {
        out.push_str("No bugs recorded.\n");
    }
    for bug in report["bugs"].as_array().into_iter().flatten() {
        out.push_str(&format!(
            "- {} ({}): created {}; reproduced {}{}; verified {}{}; fix time {}\n",
            bug["id"].as_str().unwrap_or_default(),
            bug["status"].as_str().unwrap_or_default(),
            time_text(&bug["created"]),
            time_text(&bug["reproduced"]),
            bug["reproduced_source"]
                .as_str()
                .map_or(String::new(), |source| format!(" ({source})")),
            time_text(&bug["verified"]),
            bug["verified_source"]
                .as_str()
                .map_or(String::new(), |source| format!(" ({source})")),
            span_text(&bug["fix_time"])
        ));
    }
    out.push_str("\n## Native lifecycle\n\n");
    let native = &report["native"];
    out.push_str(&format!(
        "{}\n",
        native["detail"].as_str().unwrap_or_default()
    ));
    if native["current"].is_object() {
        out.push_str("Current change:\n");
        for line in native_text(&native["current"]) {
            out.push_str(&format!("{line}\n"));
        }
    }
    let selection = &native["history_selection"];
    match selection["state"].as_str() {
        Some("skipped-by-filter") => out.push_str(&format!(
            "Archived native history was not read: excluded by {}. {HISTORY_RULE}\n",
            selection["filters"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .collect::<Vec<_>>()
                .join(", ")
        )),
        Some("selected") => out.push_str(&format!(
            "Archived native history: read {}, reused {}, skipped beyond the bound {}, not read for the budget {} (budget {} s).\n",
            selection["read"], selection["reused"], selection["skipped_beyond_bound"], selection["not_read_budget"], selection["budget_seconds"]
        )),
        _ => {}
    }
    if native["history"]
        .as_array()
        .is_some_and(|list| !list.is_empty())
    {
        out.push_str(
            "Archived changes, most recent first (archive date, then finalization time):\n",
        );
        for change in native["history"].as_array().into_iter().flatten() {
            for line in native_text(change) {
                out.push_str(&format!("{line}\n"));
            }
        }
    }
    out.push_str("\n## Estimates from recorded history (not targets)\n\n");
    for entry in report["estimates"]["stages"]
        .as_array()
        .into_iter()
        .flatten()
    {
        out.push_str(&format!(
            "- {}: {}\n",
            entry["stage"].as_str().unwrap_or_default(),
            estimate_text(entry)
        ));
    }
    out.push_str(&format!(
        "- bug fixes: {}\n- human gates: {}\n",
        estimate_text(&report["estimates"]["bug_fixes"]),
        estimate_text(&report["estimates"]["gates"])
    ));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utc_times_are_iso_8601() {
        assert_eq!(utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(utc(951_782_400), "2000-02-29T00:00:00Z");
        assert_eq!(utc(1_793_496_378), "2026-11-01T01:26:18Z");
    }

    #[test]
    fn spans_never_go_negative_or_hide_unknowns() {
        let start = Time::Known(100);
        assert_eq!(
            Span::between(&start, &Time::Known(160), false).seconds(),
            Some(60)
        );
        assert!(matches!(
            Span::between(&start, &Time::Known(100), false),
            Span::Known(0, TOGETHER)
        ));
        assert!(matches!(
            Span::between(&start, &Time::Known(99), false),
            Span::Unknown(reason) if reason == "end before start"
        ));
        let parts = [Span::Known(5, ELAPSED), Span::Unknown("missing".into())];
        assert!(matches!(Span::sum(&parts), Span::Unknown(_)));
    }

    #[test]
    fn overlapping_gates_are_counted_once() {
        let gate = |opened: u64, closed: u64| Gate {
            text: "g".into(),
            opened: Time::Known(opened),
            closed: Some(Time::Known(closed)),
            open_now: false,
        };
        let gates = [gate(10, 50), gate(30, 70), gate(200, 300)];
        assert_eq!(waiting_in(0, 100, &gates, 1_000).seconds(), Some(60));
        assert_eq!(waiting_in(60, 250, &gates, 1_000).seconds(), Some(60));
    }

    #[test]
    fn archive_directories_name_their_change() {
        assert_eq!(archived_id("2026-10-31-some-change"), "some-change");
        assert_eq!(archived_id("plain-change"), "plain-change");
    }
}
