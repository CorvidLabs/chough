//! The session record, `.chough/sessions/<id>/session.md` (REQ-records-004), and its
//! append-only Timeline (REQ-history-002).
//!
//! The front matter is the checkpoint. The body is the story: `# <objective>`, Goal,
//! Where it stands, Decisions and why, What was verified, Open threads and Timeline.
//! Decisions, completed work and open questions live only in their sections; a Timeline
//! entry is one list item, `- <time> · <kind> · <from> → <to> · <text> ^t-xxxxxx`, with
//! its structured fields as indented `- key: value` lines. Chough appends entries and
//! items and never edits or removes an entry; it marks a withdrawn completed item and a
//! resolved question on their own lines, the one in-place edit REQ-records-004 allows.

use crate::checkpoint::{Caller, Checkpoint, Consent, References, StageState};
use crate::markdown::{Kind, Record, Value};
use crate::stage::Transition;
use anyhow::{Context, Result, ensure};

pub(super) const GOAL: &str = "Goal";
pub(super) const STANDS: &str = "Where it stands";
pub(super) const DECISIONS: &str = "Decisions and why";
pub(super) const VERIFIED: &str = "What was verified";
pub(super) const THREADS: &str = "Open threads";
pub(super) const TIMELINE: &str = "Timeline";
/// At most this many entries in one session's Timeline (REQ-history-003).
pub(super) const MAX_ENTRIES: usize = 4_096;
/// At most this many bytes in one entry with its fields (REQ-history-003).
pub(super) const ENTRY_BYTES: usize = 65_536;
const WITHDRAWN: &str = " (withdrawn)";
const RESOLVED: &str = " (resolved)";

/// What `session.md` holds besides the checkpoint fields Chough already modelled.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct Meta {
    pub(super) id: String,
    pub(super) aliases: Vec<String>,
    pub(super) pr: String,
    pub(super) gates: Vec<GateMeta>,
}

/// An open human gate with the stage it holds and when it opened (REQ-cli-030).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(super) struct GateMeta {
    pub(super) gate: String,
    pub(super) holds: String,
    pub(super) opened: String,
}

/// One parsed Timeline entry.
#[derive(Clone)]
pub(super) struct Entry {
    pub(super) id: String,
    pub(super) transition: Transition,
    pub(super) native_change: Option<String>,
}

/// The body of a new session: the story's parts in order, the Goal from the objective.
pub(super) fn new_body(objective: &str) -> String {
    let title = one_line(objective);
    format!(
        "# {title}\n\n## {GOAL}\n\n{title}\n\n## {STANDS}\n\n## {DECISIONS}\n\n## {VERIFIED}\n\n## {THREADS}\n\n## {TIMELINE}\n\n"
    )
}

/// Read a session record into its checkpoint, refusing anything that is not one.
pub(super) fn read(relative: &str, text: &str) -> Result<Checkpoint> {
    let record = crate::markdown::parse(relative, text)?;
    ensure!(
        record.scalar("kind") == Some("session"),
        "{relative}: not a session record (kind: session)"
    );
    let field = |key: &str| record.scalar(key).unwrap_or_default().to_string();
    let callers = record
        .maps("callers")
        .into_iter()
        .map(|map| Caller {
            stage: pick(&map, "stage"),
            reason: pick(&map, "reason"),
        })
        .collect();
    let gates: Vec<GateMeta> = record
        .maps("waiting_on_human")
        .into_iter()
        .map(|map| GateMeta {
            gate: pick(&map, "gate"),
            holds: pick(&map, "holds"),
            opened: pick(&map, "opened"),
        })
        .collect();
    let run_consents = record
        .maps("run_consents")
        .into_iter()
        .map(|map| -> Result<Consent> {
            Ok(Consent {
                key: pick(&map, "key"),
                source: pick(&map, "source"),
                objective: pick(&map, "objective"),
                granted_at_unix_seconds: parse_time(&pick(&map, "granted"))
                    .with_context(|| format!("{relative}: run consent time"))?,
            })
        })
        .collect::<Result<Vec<_>>>()?;
    let iteration = field("iteration");
    let mut session = crate::checkpoint::new_checkpoint(
        field("objective"),
        field("intent_scope"),
        record.list("modules"),
    );
    session.stage = StageState {
        current: field("stage"),
        status: field("stage_status"),
        callers,
        iteration: if iteration.is_empty() {
            1
        } else {
            iteration
                .parse()
                .with_context(|| format!("{relative}: iteration is a whole number"))?
        },
    };
    session.native_change = Some(field("native_change")).filter(|change| !change.is_empty());
    session.references = References {
        criteria: record.list("criteria"),
        requirements: record.list("requirements"),
        cases: record.list("cases"),
        design: record.list("design"),
        files: record.list("files"),
    };
    session.observations = record.list("observations");
    session.bugs = record.list("bugs");
    session.blockers = record.list("blockers");
    session.awaiting_human = gates.iter().map(|gate| gate.gate.clone()).collect();
    session.run_consents = run_consents;
    session.next_action = field("next");
    session.decisions = open_items(&record, DECISIONS, &[]);
    session.completed = open_items(&record, STANDS, &[WITHDRAWN]);
    session.open_questions = open_items(&record, THREADS, &[RESOLVED]);
    session.meta = Meta {
        id: field("id"),
        aliases: record.list("aliases"),
        pr: field("pr"),
        gates,
    };
    session.source = Some(record);
    Ok(session)
}

/// The record to write for `session`: its previous record with the front matter set,
/// new section items appended, withdrawn and resolved items marked on their own lines,
/// and its pending transitions appended to the Timeline. `now` stamps a new gate.
pub(super) fn render(session: &Checkpoint, now: u64) -> Result<Record> {
    let mut record = match &session.source {
        Some(record) => record.clone(),
        None => Record::new(&new_body(&session.objective)),
    };
    let meta = &session.meta;
    let list = |items: &[String]| Value::List(items.to_vec());
    record.set_scalar("kind", "session")?;
    record.set_scalar("id", &meta.id)?;
    record.set("aliases", list(&meta.aliases))?;
    record.set_scalar("objective", &session.objective)?;
    record.set_scalar("intent_scope", &session.intent_scope)?;
    record.set("modules", list(&session.modules))?;
    record.set_scalar("stage", &session.stage.current)?;
    record.set_scalar("stage_status", &session.stage.status)?;
    record.set_scalar("iteration", &session.stage.iteration.to_string())?;
    record.set(
        "callers",
        Value::Maps(
            session
                .stage
                .callers
                .iter()
                .map(|caller| {
                    vec![
                        ("stage".into(), caller.stage.clone()),
                        ("reason".into(), caller.reason.clone()),
                    ]
                })
                .collect(),
        ),
    )?;
    record.set_scalar(
        "native_change",
        session.native_change.as_deref().unwrap_or(""),
    )?;
    record.set_scalar("pr", &meta.pr)?;
    record.set("criteria", list(&session.references.criteria))?;
    record.set("requirements", list(&session.references.requirements))?;
    record.set("cases", list(&session.references.cases))?;
    record.set("design", list(&session.references.design))?;
    record.set("files", list(&session.references.files))?;
    record.set("observations", list(&session.observations))?;
    record.set("bugs", list(&session.bugs))?;
    record.set("blockers", list(&session.blockers))?;
    let gates: Vec<Vec<(String, String)>> = session
        .awaiting_human
        .iter()
        .map(|gate| {
            let known = meta.gates.iter().find(|meta| &meta.gate == gate);
            vec![
                ("gate".into(), gate.clone()),
                (
                    "holds".into(),
                    known.map_or_else(|| session.stage.current.clone(), |meta| meta.holds.clone()),
                ),
                (
                    "opened".into(),
                    known.map_or_else(|| iso_time(now), |meta| meta.opened.clone()),
                ),
            ]
        })
        .collect();
    record.set("waiting_on_human", Value::Maps(gates))?;
    record.set(
        "run_consents",
        Value::Maps(
            session
                .run_consents
                .iter()
                .map(|consent| {
                    vec![
                        ("key".into(), consent.key.clone()),
                        ("source".into(), consent.source.clone()),
                        ("objective".into(), consent.objective.clone()),
                        ("granted".into(), iso_time(consent.granted_at_unix_seconds)),
                    ]
                })
                .collect(),
        ),
    )?;
    record.set_scalar("next", &session.next_action)?;
    sync_items(&mut record, DECISIONS, &session.decisions, "")?;
    sync_items(&mut record, STANDS, &session.completed, WITHDRAWN)?;
    sync_items(&mut record, THREADS, &session.open_questions, RESOLVED)?;
    let verified = items(&record, VERIFIED);
    for observation in &session.observations {
        if !verified.iter().any(|line| line == observation) {
            record.append_to_section(VERIFIED, &format!("- {observation}\n"))?;
        }
    }
    let existing = timeline(&record)?;
    ensure!(
        existing.len() + session.pending.len() <= MAX_ENTRIES,
        "refused: the timeline of session {} would pass its bound of {MAX_ENTRIES} entries; run `chough session close {} --reason <text>` and then `chough session new <key>`; the session file is unchanged",
        meta.id,
        meta.id
    );
    let mut ids: Vec<String> = existing.into_iter().map(|entry| entry.id).collect();
    for transition in &session.pending {
        let id = loop {
            let candidate = entry_id()?;
            if !ids.contains(&candidate) {
                break candidate;
            }
        };
        let text = render_entry(transition, session.native_change.as_deref(), &id);
        ensure!(
            text.len() <= ENTRY_BYTES,
            "refused: a timeline entry would be {} bytes, over its bound of {ENTRY_BYTES} bytes; run `chough session close {} --reason <text>` and then `chough session new <key>`; the session file is unchanged",
            text.len(),
            meta.id
        );
        record.append_to_section(TIMELINE, &text)?;
        ids.push(id);
    }
    record.render_within(Kind::Session)?;
    Ok(record)
}

/// Every Timeline entry, in order.
pub(super) fn timeline(record: &Record) -> Result<Vec<Entry>> {
    let Some(section) = record.section(TIMELINE) else {
        return Ok(vec![]);
    };
    let mut entries: Vec<Entry> = vec![];
    for line in section.lines() {
        if let Some(head) = line.strip_prefix("- ") {
            entries.push(parse_head(head)?);
        } else if let Some(field) = line.strip_prefix("  - ") {
            let entry = entries
                .last_mut()
                .context("a timeline field line comes after its entry")?;
            let (key, value) = field
                .split_once(": ")
                .map(|(key, value)| (key, value.to_string()))
                .or_else(|| field.strip_suffix(':').map(|key| (key, String::new())))
                .with_context(|| format!("timeline entry {}: a field is `key: value`", entry.id))?;
            apply_field(entry, key, value)?;
        }
        // Any other line is prose a person or the closing line wrote; it is not an entry.
    }
    for entry in &mut entries {
        // An entry whose text is its gate has no separate reason.
        if entry.transition.reason.as_deref() == entry.transition.gate.as_deref() {
            entry.transition.reason = None;
        }
        let transition = &entry.transition;
        ensure!(
            crate::stage::is_stage(&transition.from)
                && crate::stage::is_stage(&transition.to)
                && (crate::stage::STAGE_KINDS.contains(&transition.kind.as_str())
                    || matches!(
                        transition.kind.as_str(),
                        crate::stage::GATE_OPENED
                            | crate::stage::GATE_CLOSED
                            | crate::stage::DIRECT_WRITE
                    )),
            "invalid transition at timeline entry ^{}: unknown stage or kind",
            entry.id
        );
        crate::stage::validate_records(transition)
            .with_context(|| format!("invalid transition at timeline entry ^{}", entry.id))?;
    }
    Ok(entries)
}

fn parse_head(head: &str) -> Result<Entry> {
    parse_head_with(head, false).map(|(entry, _)| entry)
}

/// The entry of a head line; with `lenient`, a time that cannot be read is unknown
/// (`false` beside the entry) instead of an error.
fn parse_head_with(head: &str, lenient: bool) -> Result<(Entry, bool)> {
    let (body, id) = head
        .rsplit_once(" ^")
        .filter(|(_, id)| valid_entry_id(id))
        .with_context(|| format!("timeline entry has no `^t-xxxxxx` ID: {head}"))?;
    let mut parts = body.splitn(4, " · ");
    let time = parts.next().unwrap_or_default();
    let kind = parts.next().context("timeline entry has no kind")?;
    let stages = parts.next().context("timeline entry has no stages")?;
    let text = parts.next().unwrap_or_default();
    let (from, to) = stages
        .split_once(" → ")
        .context("timeline entry stages are `<from> → <to>`")?;
    let at = match parse_time(time) {
        Ok(at) => Some(at),
        Err(_) if lenient => None,
        Err(error) => return Err(error),
    };
    Ok((
        Entry {
            id: id.to_string(),
            transition: Transition {
                schema_version: 1,
                at_unix_seconds: at.unwrap_or_default(),
                kind: kind.to_string(),
                from: from.to_string(),
                to: to.to_string(),
                reason: (!text.is_empty()).then(|| text.to_string()),
                gate: None,
                revision: None,
                refresh: vec![],
                backfill: None,
                relayed_by: None,
                direct_write: None,
            },
            native_change: None,
        },
        at.is_some(),
    ))
}

/// Every Timeline entry as a transition value, for the read-only timing report
/// (REQ-reporting-001): an entry whose time cannot be read has no `at_unix_seconds`, so
/// its time is unknown and never zero, and an entry that cannot be read at all is its
/// raw line, which timing counts as unreadable and skips. Every other reader refuses
/// such an entry.
pub(super) fn timeline_for_timing(record: &Record) -> Vec<serde_json::Value> {
    let Some(section) = record.section(TIMELINE) else {
        return vec![];
    };
    let mut groups: Vec<(String, Vec<String>)> = vec![];
    for line in section.lines() {
        if let Some(head) = line.strip_prefix("- ") {
            groups.push((head.to_string(), vec![]));
        } else if let Some(field) = line.strip_prefix("  - ")
            && let Some((_, fields)) = groups.last_mut()
        {
            fields.push(field.to_string());
        }
    }
    groups
        .into_iter()
        .map(|(head, fields)| {
            let read = || -> Result<serde_json::Value> {
                let (mut entry, known) = parse_head_with(&head, true)?;
                for field in &fields {
                    let (key, value) = field
                        .split_once(": ")
                        .map(|(key, value)| (key, value.to_string()))
                        .or_else(|| field.strip_suffix(':').map(|key| (key, String::new())))
                        .context("a field is `key: value`")?;
                    apply_field(&mut entry, key, value)?;
                }
                if entry.transition.reason.as_deref() == entry.transition.gate.as_deref() {
                    entry.transition.reason = None;
                }
                let mut value = serde_json::to_value(&entry.transition)?;
                if !known && let Some(object) = value.as_object_mut() {
                    object.remove("at_unix_seconds");
                }
                Ok(value)
            };
            read().unwrap_or(serde_json::Value::String(head))
        })
        .collect()
}

fn apply_field(entry: &mut Entry, key: &str, value: String) -> Result<()> {
    let transition = &mut entry.transition;
    match key {
        "native_change" => entry.native_change = Some(value),
        "gate" => transition.gate = Some(value),
        "revision" => transition.revision = Some(value),
        "refresh" => {
            transition.refresh = value
                .split(", ")
                .filter(|item| !item.is_empty())
                .map(str::to_string)
                .collect()
        }
        "recorded_at" | "commit" | "quote" => {
            let backfill = transition
                .backfill
                .get_or_insert_with(|| crate::stage::Backfill {
                    recorded_at_unix_seconds: 0,
                    commit: String::new(),
                    quote: String::new(),
                });
            match key {
                "recorded_at" => backfill.recorded_at_unix_seconds = parse_time(&value)?,
                "commit" => backfill.commit = value,
                _ => backfill.quote = value,
            }
        }
        "relayed_by" => transition.relayed_by = Some(serde_json::from_str(&value)?),
        "direct_write" => transition.direct_write = Some(serde_json::from_str(&value)?),
        // A field a later Chough writes (`by`, corrections and so on) reads as unknown.
        _ => {}
    }
    Ok(())
}

fn render_entry(transition: &Transition, native_change: Option<&str>, id: &str) -> String {
    let text = transition
        .reason
        .as_deref()
        .or(transition.gate.as_deref())
        .map(one_line)
        .unwrap_or_default();
    let mut out = format!(
        "- {} · {} · {} → {} · {text} ^{id}\n",
        iso_time(transition.at_unix_seconds),
        transition.kind,
        transition.from,
        transition.to
    );
    let mut field =
        |key: &str, value: &str| out.push_str(&format!("  - {key}: {}\n", one_line(value)));
    if let Some(change) = native_change {
        field("native_change", change);
    }
    if let Some(gate) = &transition.gate {
        field("gate", gate);
    }
    if let Some(revision) = &transition.revision {
        field("revision", revision);
    }
    if !transition.refresh.is_empty() {
        field("refresh", &transition.refresh.join(", "));
    }
    let json = |value: &dyn erased_json::ToJson| value.to_json();
    if let Some(backfill) = &transition.backfill {
        // REQ-history-005: the original record time, the commit and the quoted evidence.
        field("recorded_at", &iso_time(backfill.recorded_at_unix_seconds));
        field("commit", &backfill.commit);
        field("quote", &backfill.quote);
    }
    if let Some(relayed) = &transition.relayed_by {
        field("relayed_by", &json(relayed));
    }
    if let Some(write) = &transition.direct_write {
        field("direct_write", &json(write));
    }
    out
}

mod erased_json {
    /// One-line JSON for a structured timeline field.
    pub(super) trait ToJson {
        fn to_json(&self) -> String;
    }

    impl<Item: serde::Serialize> ToJson for Item {
        fn to_json(&self) -> String {
            serde_json::to_string(self).unwrap_or_default()
        }
    }
}

/// The text of each top-level list item of a section, first line only.
fn items(record: &Record, heading: &str) -> Vec<String> {
    record
        .section(heading)
        .unwrap_or_default()
        .lines()
        .filter_map(|line| line.strip_prefix("- "))
        .map(str::to_string)
        .collect()
}

/// Items not marked with any of `marks`.
fn open_items(record: &Record, heading: &str, marks: &[&str]) -> Vec<String> {
    items(record, heading)
        .into_iter()
        .filter(|item| !marks.iter().any(|mark| item.ends_with(mark)))
        .collect()
}

/// Append the wanted items a section lacks, and mark the open items no longer wanted.
fn sync_items(record: &mut Record, heading: &str, wanted: &[String], mark: &str) -> Result<()> {
    let present = items(record, heading);
    let open: Vec<&String> = present
        .iter()
        .filter(|item| mark.is_empty() || !item.ends_with(mark))
        .collect();
    if !mark.is_empty() {
        for item in &open {
            if !wanted.iter().any(|want| one_line(want) == **item) {
                mark_item(record, heading, item, mark)?;
            }
        }
    }
    for want in wanted {
        let line = one_line(want);
        if !open.iter().any(|item| **item == line) {
            record.append_to_section(heading, &format!("- {line}\n"))?;
        }
    }
    Ok(())
}

/// Mark the first open item `- <item>` of a section by appending `mark` to its line.
fn mark_item(record: &mut Record, heading: &str, item: &str, mark: &str) -> Result<()> {
    record.mark_item(heading, item, mark)
}

fn pick(map: &[(String, String)], key: &str) -> String {
    map.iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.clone())
        .unwrap_or_default()
}

/// Text on one line, so a list item or field never spills into the next.
fn one_line(text: &str) -> String {
    text.replace("\r\n", " ").replace(['\n', '\r'], " ")
}

fn entry_id() -> Result<String> {
    let mut bytes = [0_u8; 3];
    getrandom::fill(&mut bytes)?;
    Ok(format!(
        "t-{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2]
    ))
}

fn valid_entry_id(id: &str) -> bool {
    id.len() == 8
        && id.starts_with("t-")
        && id[2..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// `YYYY-MM-DDTHH:MM:SSZ` for a Unix time.
pub(super) fn iso_time(seconds: u64) -> String {
    let day = seconds % 86_400;
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        crate::identity::utc_date(seconds),
        day / 3_600,
        day % 3_600 / 60,
        day % 60
    )
}

/// The Unix time of `YYYY-MM-DDTHH:MM:SSZ`.
pub(super) fn parse_time(text: &str) -> Result<u64> {
    let invalid = || anyhow::anyhow!("invalid time {text:?}; expected YYYY-MM-DDTHH:MM:SSZ");
    let bytes = text.as_bytes();
    if bytes.len() != 20
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || bytes[16] != b':'
        || bytes[19] != b'Z'
    {
        return Err(invalid());
    }
    let number = |range: std::ops::Range<usize>| -> Result<i64> {
        text[range].parse::<i64>().map_err(|_| invalid())
    };
    let (year, month, day) = (number(0..4)?, number(5..7)?, number(8..10)?);
    let (hour, minute, second) = (number(11..13)?, number(14..16)?, number(17..19)?);
    if !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 59
    {
        return Err(invalid());
    }
    // Days from civil (H. Hinnant).
    let shifted_year = if month <= 2 { year - 1 } else { year };
    let era = shifted_year.div_euclid(400);
    let year_of_era = shifted_year - era * 400;
    let shifted_month = if month > 2 { month - 3 } else { month + 9 };
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    let seconds = days * 86_400 + hour * 3_600 + minute * 60 + second;
    let seconds = u64::try_from(seconds).map_err(|_| invalid())?;
    ensure!(
        iso_time(seconds) == text,
        "invalid time {text:?}: no such date"
    );
    Ok(seconds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn transition(kind: &str, from: &str, to: &str, reason: Option<&str>) -> Transition {
        Transition {
            schema_version: 1,
            at_unix_seconds: 1_794_380_000,
            kind: kind.into(),
            from: from.into(),
            to: to.into(),
            reason: reason.map(str::to_string),
            gate: None,
            revision: None,
            refresh: vec![],
            backfill: None,
            relayed_by: None,
            direct_write: None,
        }
    }

    fn session() -> Checkpoint {
        let mut session = crate::checkpoint::new_checkpoint(
            "Ship alpha".into(),
            "DRIVE".into(),
            vec!["cli".into()],
        );
        session.meta.id = "2026-11-11-alpha".into();
        session.meta.aliases = vec!["alpha".into()];
        session
    }

    #[test]
    fn times_round_trip() {
        for seconds in [0, 951_782_400, 1_794_380_000, 1_735_689_599] {
            assert_eq!(parse_time(&iso_time(seconds)).unwrap(), seconds);
        }
        assert!(parse_time("2026-02-30T00:00:00Z").is_err());
        assert!(parse_time("2026-11-11 00:00:00").is_err());
    }

    #[test]
    fn a_new_session_has_every_key_and_the_story_in_order() {
        let record = render(&session(), 1_794_380_000).unwrap();
        for key in [
            "kind",
            "id",
            "aliases",
            "objective",
            "intent_scope",
            "modules",
            "stage",
            "stage_status",
            "iteration",
            "callers",
            "native_change",
            "pr",
            "criteria",
            "requirements",
            "cases",
            "design",
            "files",
            "observations",
            "bugs",
            "blockers",
            "waiting_on_human",
            "run_consents",
            "next",
        ] {
            assert!(record.get(key).is_some(), "{key}");
        }
        let body = record.body();
        let order = [
            "# Ship alpha",
            "## Goal",
            "## Where it stands",
            "## Decisions and why",
            "## What was verified",
            "## Open threads",
            "## Timeline",
        ];
        let positions: Vec<usize> = order.iter().map(|part| body.find(part).unwrap()).collect();
        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{body}");
        assert!(body.contains("## Goal\n\nShip alpha\n"), "{body}");
    }

    #[test]
    fn a_checkpoint_round_trips_through_its_record() {
        let mut first = session();
        first.decisions = vec!["Use plan A".into()];
        first.completed = vec!["Wrote the plan".into()];
        first.open_questions = vec!["Which option?".into()];
        first.observations = vec!["OBS-x-0123".into()];
        first.awaiting_human = vec!["Scope approval".into()];
        first.native_change = Some("change-a".into());
        first.stage.current = "plan".into();
        first.pending = vec![transition("advance", "intent", "plan", Some("Leif: go"))];
        let text = render(&first, 1_794_380_000).unwrap().render();
        let read = super::read("s.md", &text).unwrap();
        assert_eq!(read.decisions, first.decisions);
        assert_eq!(read.completed, first.completed);
        assert_eq!(read.open_questions, first.open_questions);
        assert_eq!(read.observations, first.observations);
        assert_eq!(read.awaiting_human, first.awaiting_human);
        assert_eq!(read.native_change.as_deref(), Some("change-a"));
        assert_eq!(read.stage.current, "plan");
        assert_eq!(read.meta.gates[0].holds, "plan");
        assert_eq!(read.meta.gates[0].opened, "2026-11-11T06:53:20Z");
        let entries = timeline(read.source.as_ref().unwrap()).unwrap();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].transition.reason.as_deref(), Some("Leif: go"));
        assert_eq!(entries[0].native_change.as_deref(), Some("change-a"));
        assert!(
            text.contains("- 2026-11-11T06:53:20Z · advance · intent → plan · Leif: go ^t-"),
            "{text}"
        );
        // Saving the read checkpoint unchanged writes the same bytes.
        assert_eq!(render(&read, 1_800_000_000).unwrap().render(), text);
    }

    #[test]
    fn removed_items_are_marked_and_new_ones_appended_without_touching_other_text() {
        let mut first = session();
        first.completed = vec!["one".into(), "two".into()];
        first.open_questions = vec!["why?".into()];
        let mut record = render(&first, 0).unwrap();
        record
            .append_to_section(GOAL, "A person's added line.\n")
            .unwrap();
        let mut second = super::read("s.md", &record.render()).unwrap();
        second.completed = vec!["two".into(), "three".into()];
        second.open_questions = vec![];
        let text = render(&second, 0).unwrap().render();
        assert!(
            text.contains("- one (withdrawn)\n- two\n- three\n"),
            "{text}"
        );
        assert!(text.contains("- why? (resolved)\n"), "{text}");
        assert!(text.contains("A person's added line.\n"), "{text}");
        let third = super::read("s.md", &text).unwrap();
        assert_eq!(third.completed, vec!["two", "three"]);
        assert!(third.open_questions.is_empty());
    }

    #[test]
    fn entries_are_only_appended_with_unique_ids() {
        let mut first = session();
        first.pending = vec![
            transition("advance", "intent", "design", None),
            transition("gate-opened", "design", "design", None),
        ];
        first.pending[1].gate = Some("Approve the design".into());
        let text = render(&first, 0).unwrap().render();
        let mut second = super::read("s.md", &text).unwrap();
        second.pending = vec![transition("back", "design", "intent", Some("redo"))];
        let next = render(&second, 0).unwrap().render();
        assert!(
            next.starts_with(text.trim_end()),
            "earlier bytes stay a prefix"
        );
        let entries = timeline(&crate::markdown::parse("s.md", &next).unwrap()).unwrap();
        assert_eq!(entries.len(), 3);
        assert_eq!(
            entries[1].transition.gate.as_deref(),
            Some("Approve the design")
        );
        assert_eq!(entries[1].transition.reason, None);
        let mut ids: Vec<&str> = entries.iter().map(|entry| entry.id.as_str()).collect();
        ids.dedup();
        assert_eq!(ids.len(), 3);
    }

    #[test]
    fn a_timeline_over_its_bound_is_refused() {
        let mut long = session();
        long.pending = vec![transition(
            "advance",
            "intent",
            "design",
            Some(&"x".repeat(ENTRY_BYTES)),
        )];
        let error = render(&long, 0).unwrap_err().to_string();
        assert!(error.contains("over its bound of 65536 bytes"), "{error}");
        assert!(
            error.contains("chough session close 2026-11-11-alpha --reason"),
            "{error}"
        );
    }
}
