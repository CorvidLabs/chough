//! The session Timeline and its bounds (REQ-history-002, REQ-history-003).
//!
//! The 0.5.x event store, cursors, receipts, adoption and pruning are removed with
//! REQ-history-001, 004, 008 and 011; their tests went with them.

mod common;

use common::{Fixture, timeline};
use serde_json::{Value, json};

/// A Timeline of `count` advance entries, each with a unique ID.
fn entries(count: usize) -> String {
    (0..count)
        .map(|index| {
            format!("- 2026-11-11T00:00:00Z · iterate · intent → intent · round {index} ^t-{index:06x}\n")
        })
        .collect()
}

/// Replace the active session's Timeline with `count` entries.
fn fill_timeline(fixture: &Fixture, count: usize) -> String {
    let file = fixture.session_file();
    let text = fixture.read(&file);
    let start = text.find("## Timeline\n").expect("a Timeline section");
    let filled = format!("{}## Timeline\n\n{}", &text[..start], entries(count));
    fixture.write(&file, &filled);
    filled
}

/// REQ-history-003: a timeline holds 4,096 entries; the 4,097th is refused before the
/// save, naming the bound and the recovery, with the session file byte-identical.
#[test]
fn a_timeline_holds_4096_entries_and_refuses_the_4097th() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fill_timeline(&fixture, 4_095);
    fixture.ok(&["stage", "iterate", "--note", "the 4096th"]);
    let full = fixture.read(&fixture.session_file());
    assert_eq!(timeline(&full).len(), 4_096);
    let refused = fixture.fails(&["stage", "iterate", "--note", "the 4097th"]);
    assert!(refused.contains("bound of 4096 entries"), "{refused}");
    assert!(refused.contains("chough session close"), "{refused}");
    assert!(refused.contains("chough session new <key>"), "{refused}");
    assert_eq!(fixture.read(&fixture.session_file()), full);
}

/// REQ-history-003: recent guidance reads at most the newest 128 entries of the active
/// timeline, so `next` on a full timeline still answers.
#[test]
fn next_reads_only_the_newest_128_entries() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fill_timeline(&fixture, 4_096);
    fixture.ok(&["next", "--json"]);
    let status = fixture.ok(&["status"]);
    let read = &status["transition_history"];
    assert_eq!(read["events_read"], 128, "{read:#}");
    assert!(
        !read["partial"].as_array().unwrap().is_empty(),
        "older entries are left unread and said so: {read:#}"
    );
}

/// REQ-records-001, REQ-cli-032: a session record of exactly 1,048,576 bytes is read; a
/// save that would pass it is refused naming the bound, with the file unchanged; one
/// byte more is unreadable and names `chough session shelve`.
#[test]
fn a_session_record_of_exactly_1_mib_is_read_and_one_byte_more_is_refused() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let file = fixture.session_file();
    let text = fixture.read(&file);
    let goal = "## Goal\n\n";
    let at = text.find(goal).unwrap() + goal.len();
    let padding = "x".repeat(1_048_576 - text.len() - 1) + "\n";
    let exact = format!("{}{padding}{}", &text[..at], &text[at..]);
    assert_eq!(exact.len(), 1_048_576);
    fixture.write(&file, &exact);
    assert_eq!(fixture.checkpoint()["objective"], "Ship alpha");
    let refused = fixture.fails(&["checkpoint", "--decision", "one more decision"]);
    assert!(refused.contains("1048576"), "{refused}");
    assert_eq!(fixture.read(&file), exact);
    fixture.write(&file, &format!("{exact}y"));
    let refused = fixture.fails(&["checkpoint", "--decision", "x"]);
    assert!(refused.contains("chough session shelve"), "{refused}");
}

/// REQ-history-002: each session's timeline holds only its own entries, entries are
/// only appended, and an earlier entry line is never edited by a later save.
#[test]
fn entries_are_appended_in_order_and_never_rewritten() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Approve the design"]);
    let before = timeline(&fixture.read(&fixture.session_file()));
    assert_eq!(before.len(), 2, "{before:?}");
    assert!(before[1].contains(" · gate-opened · design → design · Approve the design ^t-"));
    fixture.ok(&[
        "stage",
        "back",
        "--to",
        "intent",
        "--reason",
        "Redo the intent",
    ]);
    let after = timeline(&fixture.read(&fixture.session_file()));
    assert_eq!(&after[..2], &before[..]);
    assert!(
        after[2].contains(" · back · design → intent · Redo the intent ^t-"),
        "{after:?}"
    );
    let mut ids: Vec<&str> = after
        .iter()
        .map(|line| line.rsplit(" ^").next().unwrap())
        .collect();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), after.len());
    let shown = fixture.ok(&["history", "show"]);
    assert_eq!(shown["entries"].as_array().unwrap().len(), 3, "{shown:#}");
    assert_eq!(shown["entries"][2]["kind"], "back", "{shown:#}");
}

/// Append one hand-written entry line, with its field lines, to the active session's
/// Timeline, the way a person or another build could; `history show` then reads it.
fn with_entry(fixture: &Fixture, original: &str, entry: &str) {
    fixture.write(&fixture.session_file(), &format!("{original}{entry}"));
}

/// The time of the Timeline's last entry, so a hand-written entry after it keeps order.
fn last_time(text: &str) -> String {
    let section = &text[text.find("\n## Timeline\n").expect("a Timeline section")..];
    let start = section.rfind("\n- ").expect("an entry") + 3;
    section[start..start + 20].to_string()
}

/// TC-history-timeline-bounds (REQ-history-003). The timeline's bounds that a command
/// can reach:
/// - the 4,097th entry is refused before the save, naming the bound and the recovery,
///   with the session file byte-identical, and `next` on 4,096 entries reads only the
///   newest 128 (the two tests above, run here as part of the case);
/// - `chough timing` over more than 16 MiB of timelines reports what it did not read as
///   partial.
///
/// No command can write an entry over 65,536 bytes: every text it takes is at most
/// 4,096 bytes, and so is a relayed decision as a whole. The entry bound is proved by
/// the unit test `session_md::tests::a_timeline_over_its_bound_is_refused`.
#[test]
fn the_timeline_is_bounded() {
    a_timeline_holds_4096_entries_and_refuses_the_4097th();
    next_reads_only_the_newest_128_entries();

    // Timing's bound: 17 archived stories of 1,000,000 bytes each pass 16 MiB, so the
    // oldest is left unread and reported partial, never counted as zero.
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "story", "--objective", "Ship a story"]);
    let id = fixture.id_of("story");
    fixture.ok(&[
        "session",
        "close",
        &id,
        "--reason",
        "Closed for the timing bound",
    ]);
    let story = fixture.read(&format!(".chough/archive/sessions/{id}.md"));
    for index in 1..=17 {
        let copy = format!("2026-10-{index:02}-story");
        let text = story.replacen(&format!("id: {id}\n"), &format!("id: {copy}\n"), 1);
        assert_ne!(text, story, "the story's id line");
        let goal = "## Goal\n\n";
        let at = text.find(goal).expect("a Goal section") + goal.len();
        let padding = "p".repeat(1_000_000 - text.len() - 1) + "\n";
        let padded = format!("{}{padding}{}", &text[..at], &text[at..]);
        assert_eq!(padded.len(), 1_000_000);
        fixture.write(&format!(".chough/archive/sessions/{copy}.md"), &padded);
    }
    let report = fixture.ok(&["timing", "--json"]);
    let partial = report["partial"].to_string();
    assert!(
        partial.contains("older stories skipped beyond the 16777216-byte bound"),
        "{partial}"
    );
}

/// The relay values, or a native field, of a gate-closed entry that `history show`
/// reads back: on the entry, or inside its `relayed_by`.
fn relay_field<'a>(entry: &'a Value, key: &str) -> &'a Value {
    match &entry[key] {
        Value::Null => &entry["relayed_by"][key],
        value => value,
    }
}

const HUMANS: &str = "[[human]]\nname = \"SYNTHETIC Person\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-actor\"\n";

/// TC-history-relay-fields-on-gate-entries (REQ-history-010).
/// - A relayed `gate-closed` entry reads back with its four values byte for byte, and
///   one that drove an approval also reads back `native`, the SpecSync command run,
///   and `actor`, the SpecSync actor it ran as.
/// - An entry with three of the four, a blank or over-long value, or relay fields on a
///   `gate-opened` or stage entry, is refused as invalid.
#[cfg(unix)]
#[test]
fn relay_fields_live_on_gate_closed_entries() {
    // The refusals, on hand-written entries.
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "iterate", "--note", "A first entry"]);
    let good = json!({"decided_by": "SYNTHETIC-human", "words": "Approved", "where": "chat", "recorded_by": "SYNTHETIC-agent"});
    let mut invalid = vec![];
    for key in ["decided_by", "words", "where", "recorded_by"] {
        let mut three = good.clone();
        three.as_object_mut().unwrap().remove(key);
        invalid.push(("gate-closed", three));
        let mut blank = good.clone();
        blank[key] = json!("  ");
        invalid.push(("gate-closed", blank));
        let mut long = good.clone();
        long[key] = json!("x".repeat(4_097));
        invalid.push(("gate-closed", long));
    }
    invalid.push(("gate-opened", good.clone()));
    invalid.push(("advance", good.clone()));
    invalid.push(("iterate", good.clone()));
    let original = fixture.read(&fixture.session_file());
    let time = last_time(&original);
    let entry = |kind: &str, relay: &Value| {
        let to = if kind == "advance" {
            "design"
        } else {
            "intent"
        };
        format!(
            "- {time} · {kind} · intent → {to} · Gate ^t-aaaaaa\n  - relayed_by: {}\n",
            serde_json::to_string(relay).unwrap()
        )
    };
    with_entry(&fixture, &original, &entry("gate-closed", &good));
    let shown = fixture.ok(&["history", "show"]);
    assert_eq!(
        shown["entries"].as_array().unwrap().last().unwrap()["relayed_by"],
        good
    );
    for (kind, relay) in &invalid {
        with_entry(&fixture, &original, &entry(kind, relay));
        let error = fixture.fails(&["history", "show"]);
        assert!(
            error.contains("invalid transition"),
            "{kind} {relay}: {error}"
        );
    }

    // Through the CLI: a gate that drives nothing keeps the four values byte for byte,
    // whatever characters they hold, with the decision they make within its 4,096 bytes.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "relay-change"]);
    fixture.write(".chough/local/identity.toml", HUMANS);
    let words = "yes — ship it \"now\" · intent → done ^t-000000 # not a heading \\ end";
    let place = format!("SYNTHETIC chat {}", "w".repeat(2_000));
    fixture.ok(&["checkpoint", "--awaiting-human", "Copy review"]);
    fixture.ok(&[
        "checkpoint",
        "--close-gate",
        "Copy review",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        words,
        "--where",
        &place,
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    let closed = |fixture: &Fixture| -> Value {
        fixture.ok(&["history", "show"])["entries"]
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .find(|entry| entry["kind"] == "gate-closed")
            .expect("a gate-closed entry")
            .clone()
    };
    let entry = closed(&fixture);
    for (key, value) in [
        ("decided_by", "SYNTHETIC-human"),
        ("words", words),
        ("where", place.as_str()),
        ("recorded_by", "SYNTHETIC-agent"),
    ] {
        assert_eq!(relay_field(&entry, key), value, "{key}: {entry:#}");
    }
    assert_eq!(relay_field(&entry, "native"), &Value::Null, "{entry:#}");

    // A gate held at plan drives the approval, which runs as the configured human's
    // SpecSync actor; its entry also names the command and the actor.
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Scope approval",
        "--holds",
        "plan",
    ]);
    let output = fixture.ok(&[
        "checkpoint",
        "--close-gate",
        "Scope approval",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "yes, go",
        "--where",
        "SYNTHETIC chat",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    assert_eq!(
        output["relayed_decision"]["native_step"]["specsync_actor"], "SYNTHETIC-actor",
        "{output:#}"
    );
    let entry = closed(&fixture);
    assert_eq!(relay_field(&entry, "words"), "yes, go", "{entry:#}");
    let native = relay_field(&entry, "native").as_str().unwrap_or_default();
    assert!(
        native.contains("change approve relay-change"),
        "the gate-closed entry names no native command: {entry:#}"
    );
    assert_eq!(relay_field(&entry, "actor"), "SYNTHETIC-actor", "{entry:#}");
}

/// TC-history-direct-write-entries (REQ-history-013).
/// - A `direct-native-write` entry reads back with its change, its files (each added,
///   changed or removed) and its states, and the same fields on any other kind are
///   refused as invalid.
/// - `chough timing` reads a timeline holding one as complete: no unrecognised kind
///   and no partial mark.
#[test]
fn direct_write_entries_are_known_to_timing() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "iterate", "--note", "A first entry"]);
    let write = json!({
        "change": "some-change",
        "files": [
            {"path": "approvals.json", "change": "changed"},
            {"path": "state.json", "change": "added"},
            {"path": "review.json", "change": "removed"},
        ],
        "before_state": "none",
        "after_state": "approved",
    });
    let original = fixture.read(&fixture.session_file());
    let time = last_time(&original);
    let entry = |kind: &str| {
        format!(
            "- {time} · {kind} · intent → intent ·  ^t-aaaaaa\n  - direct_write: {}\n",
            serde_json::to_string(&write).unwrap()
        )
    };
    for kind in ["gate-closed", "gate-opened", "iterate", "reset"] {
        with_entry(&fixture, &original, &entry(kind));
        let error = fixture.fails(&["history", "show"]);
        assert!(error.contains("invalid transition"), "{kind}: {error}");
    }
    with_entry(&fixture, &original, &entry("direct-native-write"));
    let shown = fixture.ok(&["history", "show"]);
    let last = shown["entries"].as_array().unwrap().last().unwrap().clone();
    assert_eq!(last["kind"], "direct-native-write", "{last:#}");
    assert_eq!(last["from"], "intent", "{last:#}");
    assert_eq!(last["to"], "intent", "{last:#}");
    assert_eq!(last["direct_write"], write, "{last:#}");

    let report = fixture.ok(&["timing", "--json"]);
    assert_eq!(report["partial"], json!([]), "{report:#}");
    fn counts<'a>(value: &'a Value, found: &mut Vec<&'a Value>) {
        match value {
            Value::Object(map) => {
                for (name, inner) in map {
                    if name == "unrecognised_kinds" {
                        found.push(inner);
                    }
                    counts(inner, found);
                }
            }
            Value::Array(items) => items.iter().for_each(|item| counts(item, found)),
            _ => {}
        }
    }
    let mut found = vec![];
    counts(&report, &mut found);
    assert!(!found.is_empty(), "no unrecognised_kinds count: {report:#}");
    assert!(found.iter().all(|count| **count == json!(0)), "{found:?}");
}
