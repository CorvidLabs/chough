//! The knowledge directory is built with the librarian on, which is the default: at
//! `chough init`, creating only what is missing (REQ-cli-043, KNOW-1, KNOW-5).

mod common;

use common::Fixture;
use serde_json::{Value, json};

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

/// An archived change with a title of `title_len` extra characters.
fn archive(fixture: &Fixture, index: usize, id: &str, title_len: usize) {
    let dir = format!(".specsync/archive/changes/2026-0{}-01-{id}", index + 1);
    fixture.write(
        &format!("{dir}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {}, \"implementation_commit\": \"{}\"}}\n",
            (index + 1) * 100,
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{dir}/verification.json"),
        &serde_json::to_string_pretty(&json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": []},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/change.md"),
        &format!(
            "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id} {}\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- {id} works\n",
            "3".repeat(40),
            "x".repeat(title_len)
        ),
    );
}

#[test]
fn init_builds_the_knowledge_directory_with_the_librarian_on() {
    let fixture = Fixture::new();
    archive(&fixture, 0, "a1", 0);
    let out = init(&fixture);
    assert_eq!(out["library"]["status"], "created", "{out:#}");
    for file in [
        "knowledge/README.md",
        "knowledge/INDEX.md",
        "knowledge/archives/a1.md",
    ] {
        assert!(fixture.exists(file), "{file} missing: {out:#}");
    }
    assert!(fixture.root().join("knowledge/briefs").is_dir());
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["knowledge"]["index"], "knowledge/INDEX.md", "{next:#}");
}

#[test]
fn init_preserves_an_existing_knowledge_directory() {
    let fixture = Fixture::new();
    fixture.write("knowledge/README.md", "Our own notes.\n");
    fixture.write("knowledge/INDEX.md", "A hand-written index.\n");
    let out = init(&fixture);
    assert_eq!(out["library"]["status"], "preserved", "{out:#}");
    assert_eq!(fixture.read("knowledge/README.md"), "Our own notes.\n");
    assert_eq!(
        fixture.read("knowledge/INDEX.md"),
        "A hand-written index.\n"
    );
}

#[test]
fn init_still_succeeds_and_writes_no_knowledge_when_the_index_cannot_be_bounded() {
    let fixture = Fixture::new();
    for (index, id) in ["a1", "a2", "a3", "a4", "a5"].iter().enumerate() {
        archive(&fixture, index, id, 20_000);
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
