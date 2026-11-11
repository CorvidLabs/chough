//! The session Timeline and its bounds (REQ-history-002, REQ-history-003).
//!
//! The 0.5.x event store, cursors, receipts, adoption and pruning are removed with
//! REQ-history-001, 004, 008 and 011; their tests went with them.

mod common;

use common::{Fixture, timeline};

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
    fixture.ok(&["stage", "back", "--to", "intent", "--reason", "Redo the intent"]);
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
