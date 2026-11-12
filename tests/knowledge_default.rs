//! The knowledge directory is built with the librarian on, which is the default: at
//! `chough init`, creating only what is missing (REQ-cli-043, KNOW-1, KNOW-5).

mod common;

use common::Fixture;
use serde_json::Value;

fn init(fixture: &Fixture) -> Value {
    fixture.ok(&[
        "init",
        "--objective",
        "Ship a focused change",
        "--intent-scope",
        "LIFE",
        "--module",
        "cli",
    ])
}

/// The exact names in `relative`, sorted; on a case-insensitive file system `exists`
/// cannot tell `INDEX.md` from `index.md`.
fn names(fixture: &Fixture, relative: &str) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(fixture.root().join(relative))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn init_builds_the_knowledge_directory_with_the_librarian_on() {
    let fixture = Fixture::new();
    let out = init(&fixture);
    assert_eq!(out["library"]["status"], "created", "{out:#}");
    assert_eq!(out["library"]["index"], "knowledge/index.md", "{out:#}");
    // index.md, journal/ with its feed, and lessons/; no copied or generated notes.
    assert_eq!(
        names(&fixture, "knowledge"),
        ["index.md", "journal", "lessons"]
    );
    assert_eq!(names(&fixture, "knowledge/journal"), ["feed.xml"]);
    assert!(names(&fixture, "knowledge/lessons").is_empty());
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["knowledge"]["index"], "knowledge/index.md", "{next:#}");
    fixture.ok(&["library", "index", "--check"]);
}

#[test]
fn init_preserves_an_existing_knowledge_directory() {
    let fixture = Fixture::new();
    fixture.write("knowledge/README.md", "Our own notes.\n");
    fixture.write("knowledge/index.md", "A hand-written index.\n");
    let out = init(&fixture);
    assert_eq!(out["library"]["status"], "preserved", "{out:#}");
    assert_eq!(fixture.read("knowledge/README.md"), "Our own notes.\n");
    assert_eq!(
        fixture.read("knowledge/index.md"),
        "A hand-written index.\n"
    );
    assert_eq!(names(&fixture, "knowledge"), ["README.md", "index.md"]);
}

#[test]
fn init_still_succeeds_and_writes_no_knowledge_when_the_index_cannot_be_bounded() {
    let fixture = Fixture::new();
    // An earlier `library/` folder is linked from the index file by file (KNOW-6); this
    // many files cannot be linked within the 65,536-byte bound.
    for index in 0..800 {
        fixture.write(
            &format!("library/briefs/brief-number-{index:04}-with-a-long-name.md"),
            "kept\n",
        );
    }
    let out = init(&fixture);
    assert_eq!(out["library"]["status"], "refused", "{out:#}");
    assert!(
        out["library"]["reason"]
            .as_str()
            .unwrap_or_default()
            .contains("65536"),
        "{out:#}"
    );
    assert!(
        !fixture.exists("knowledge"),
        "a partial knowledge folder was left"
    );
    assert!(
        fixture
            .open_ids()
            .iter()
            .any(|id| fixture.exists(&format!(".chough/sessions/{id}/session.md"))),
        "no session record was written"
    );
}
