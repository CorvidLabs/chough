//! Append-only corrections (REQ-history-015): `chough correct` appends a `correction`
//! entry linked to the entry it corrects and leaves every earlier byte unchanged.

mod common;

use common::{Fixture, timeline};
use serde_json::Value;

/// The newest Timeline entry's ID in `text`.
fn newest_id(text: &str) -> String {
    let entry = timeline(text).last().unwrap().clone();
    let start = entry.rfind(" ^").unwrap() + 2;
    entry[start..]
        .split_whitespace()
        .next()
        .unwrap()
        .to_string()
}

/// The one span `after` adds to `before`, asserting every byte of `before` is still
/// there, in order, around it (REQ-history-015: the file is otherwise byte-identical).
fn inserted(before: &str, after: &str) -> String {
    let start = before
        .bytes()
        .zip(after.bytes())
        .take_while(|(old, new)| old == new)
        .count();
    let added = after.len() - before.len();
    assert_eq!(&after[..start], &before[..start], "an earlier byte changed");
    assert_eq!(
        &after[start + added..],
        &before[start..],
        "a byte after the correction changed"
    );
    after[start..start + added].to_string()
}

fn reset(fixture: &Fixture, reason: &str) -> String {
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "reset", "--to", "intent", "--reason", reason]);
    newest_id(&fixture.read(&fixture.session_file()))
}

#[test]
fn a_correction_appends_one_linked_entry_and_changes_no_earlier_byte() {
    let fixture = Fixture::new();
    fixture.init();
    let id = reset(&fixture, "Start the paging work");
    let file = fixture.session_file();
    let before = fixture.read(&file);
    let corrected = fixture.ok(&[
        "correct",
        &format!("^{id}"),
        "--reason",
        "Start the search work; paging was a slip",
    ]);
    assert_eq!(corrected["corrects"], format!("^{id}"));
    let after = fixture.read(&file);
    let added = &inserted(&before, &after);
    let lines: Vec<&str> = added.lines().filter(|line| !line.is_empty()).collect();
    assert_eq!(lines.len(), 2, "{added}");
    assert!(
        lines[0].contains(
            " · correction · intent → intent · Start the search work; paging was a slip ^t-"
        ),
        "{added}"
    );
    assert_eq!(lines[1], format!("  - corrects: ^{id}"));
    // next and status show the corrected text, marked, and list the correction.
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["corrections"]["total"], 1, "{next:#}");
    assert_eq!(
        next["corrections"]["items"][0]["corrects"],
        format!("^{id}")
    );
    let status = fixture.ok(&["status", "--json"]);
    let marked = "Start the search work; paging was a slip (corrected, 1 correction)";
    for shown in [
        status["session_next_action"].as_str().unwrap(),
        status["next_valid_action"].as_str().unwrap(),
        next["next_valid_action"]["action"].as_str().unwrap(),
    ] {
        assert!(shown.contains(marked), "{shown}");
        assert!(!shown.contains("Start the paging work"), "{shown}");
    }
    // The checkpoint itself still holds what was recorded.
    assert!(
        fixture.checkpoint()["next_action"]
            .as_str()
            .unwrap()
            .contains("Start the paging work")
    );
}

#[test]
fn bad_targets_and_statements_are_refused_with_nothing_written() {
    let fixture = Fixture::new();
    fixture.init();
    let id = reset(&fixture, "Start");
    fixture.ok(&["correct", &format!("^{id}"), "--reason", "Restart"]);
    let correction = newest_id(&fixture.read(&fixture.session_file()));
    let file = fixture.session_file();
    let before = fixture.read(&file);
    let long = "x".repeat(4_097);
    for (target, reason, why) in [
        ("^t-000000", "Fine", "no Timeline entry ^t-000000"),
        (
            "nosuch#^t-000000",
            "Fine",
            "no open session or archived story nosuch",
        ),
        (&format!("^{id}") as &str, "  ", "not blank"),
        (&format!("^{id}"), long.as_str(), "4096-byte bound"),
        (
            &format!("^{correction}"),
            "Again",
            "a correction is not corrected",
        ),
        ("t-000000", "Fine", "^<entry id>"),
    ] {
        let refused = fixture.fails(&["correct", target, "--reason", reason]);
        assert!(refused.contains(why), "{target}: {refused}");
        assert_eq!(fixture.read(&file), before, "{target} wrote something");
    }
    // A 4,096-byte statement is accepted.
    fixture.ok(&["correct", &format!("^{id}"), "--reason", &"y".repeat(4_096)]);
}

#[test]
fn an_archived_story_gets_the_correction_appended_and_nothing_else() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    // The move into done archives the session into its story.
    while !fixture.is_archived("alpha") {
        fixture.ok(&["stage", "advance"]);
    }
    let id = fixture.id_of("alpha");
    let story = format!(".chough/archive/sessions/{id}.md");
    let before = fixture.read(&story);
    let entry = newest_id(&before);
    fixture.ok(&[
        "correct",
        &format!("alpha#^{entry}"),
        "--reason",
        "Done was reached on review, not on tests",
    ]);
    let after = fixture.read(&story);
    assert!(inserted(&before, &after).contains(&format!("  - corrects: ^{entry}")));
    // The story's Timeline still reads, the correction last.
    let shown = fixture.ok(&["history", "show"]);
    let last: &Value = shown["entries"].as_array().unwrap().last().unwrap();
    assert_eq!(last["kind"], "correction");
    assert_eq!(last["corrects"], format!("^{entry}"));
}

#[test]
fn next_lists_the_ten_newest_corrections_and_the_total() {
    let fixture = Fixture::new();
    fixture.init();
    let id = reset(&fixture, "Start");
    for index in 0..12 {
        fixture.ok(&[
            "correct",
            &format!("^{id}"),
            "--reason",
            &format!("Fix {index:02}"),
        ]);
    }
    let next = fixture.ok(&["next", "--json"]);
    let corrections = &next["corrections"];
    assert_eq!(corrections["total"], 12);
    let items = corrections["items"].as_array().unwrap();
    assert_eq!(items.len(), 10);
    // Newest first by recorded time, then entry ID.
    let keys: Vec<(String, String)> = items
        .iter()
        .map(|item| {
            (
                item["at"].as_str().unwrap().to_string(),
                item["entry"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    let mut sorted = keys.clone();
    sorted.sort_by(|left, right| right.cmp(left));
    assert_eq!(keys, sorted);
}

/// REQ-history-015, REQ-history-010, TC-history-a-correction-keeps-the-timeline-one-list
/// (S4): the correction joins the Timeline right after its last entry, as ordinary
/// saves do, so no blank line ever splits the list, and the next save still appends
/// after it.
#[test]
fn a_correction_joins_the_timeline_list_with_no_blank_line() {
    let fixture = Fixture::new();
    fixture.init();
    let id = reset(&fixture, "Start the paging work");
    fixture.ok(&[
        "correct",
        &format!("^{id}"),
        "--reason",
        "Start the search work; paging was a slip",
    ]);
    fixture.ok(&["stage", "advance"]);
    let text = fixture.read(&fixture.session_file());
    let start = text.find("\n## Timeline\n").unwrap() + "\n## Timeline\n".len();
    let section = &text[start..];
    let section = &section[..section.find("\n## ").unwrap_or(section.len())];
    let list = section.trim_matches('\n');
    assert!(
        !list.contains("\n\n"),
        "a blank line splits the Timeline:\n{list}"
    );
    let entries = timeline(&text);
    assert!(
        entries[entries.len() - 2].contains(" · correction · "),
        "{entries:#?}"
    );
    assert!(
        entries.last().unwrap().contains(" · advance · "),
        "{entries:#?}"
    );
}
