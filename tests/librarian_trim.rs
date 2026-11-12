//! The move into done refreshes the knowledge directory with the librarian on and builds
//! nothing with it off (REQ-cli-043, KNOW-4). Nothing is trimmed at done any more: proof
//! stays in its session and story (REQ-records-012).
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// One archived native change, a5, a stub `specsync` reporting every change archived and
/// authenticated, and the checkpoint at accept on a5, so `stage advance` moves into done.
fn at_accept() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"acceptance_entries\":[]}\\n' \"$3\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    let dir = ".specsync/archive/changes/2026-05-01-a5";
    fixture.write(
        &format!("{dir}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"a5\", \"timestamp\": 500, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{dir}/verification.json"),
        &serde_json::to_string_pretty(&json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": [
                {"path": "specs/cli/cli.spec.md", "kind": "file", "payload_digest": ZERO}
            ]},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/change.md"),
        &format!(
            "---\nid: a5\nstate: archived\nbase_commit: {}\n---\n\n# Title of a5\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- a5 works\n",
            "3".repeat(40)
        ),
    );
    fixture.init();
    fixture.commit("fixture");
    fixture.set_front("stage", "accept");
    fixture.set_front("native_change", "a5");
    fixture
}

fn advance_to_done(fixture: &Fixture) -> Value {
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    moved
}

/// REQ-cli-043: in a project that had no knowledge directory, the move into done writes
/// its journal post and refreshes the index, and `library index --check` passes right
/// after it with no command in between; nothing is trimmed.
#[test]
fn done_refreshes_the_knowledge_index_when_nothing_is_trimmed() {
    let fixture = at_accept();
    let _ = std::fs::remove_dir_all(fixture.root().join("knowledge"));
    let moved = advance_to_done(&fixture);
    let refresh = &moved["applied"]["librarian_index"];
    assert_eq!(refresh["status"], "refreshed", "{moved:#}");
    assert_eq!(refresh["index"], "knowledge/index.md", "{moved:#}");
    assert!(
        moved["applied"].get("librarian_trim").is_none(),
        "{moved:#}"
    );
    assert_eq!(
        moved["applied"]["journal_post"]["status"], "written",
        "{moved:#}"
    );
    let post = moved["applied"]["journal_post"]["post"]
        .as_str()
        .unwrap_or_default();
    assert!(fixture.exists(post), "{moved:#}");
    assert!(fixture.exists("knowledge/journal/feed.xml"));
    assert!(fixture.exists("knowledge/lessons"));
    fixture.ok(&["library", "index", "--check"]);
}

/// REQ-cli-043: with the librarian off the move into done writes no post, reports the
/// index skipped, builds no knowledge directory, and done is still recorded.
#[test]
fn a_librarian_off_project_gets_no_knowledge_directory_at_done() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: SYNTHETIC no librarian here",
    ]);
    let _ = std::fs::remove_dir_all(fixture.root().join("knowledge"));
    let moved = advance_to_done(&fixture);
    assert_eq!(
        moved["applied"]["librarian_index"]["status"], "skipped",
        "{moved:#}"
    );
    assert_eq!(
        moved["applied"]["journal_post"]["status"], "skipped",
        "{moved:#}"
    );
    assert!(!fixture.exists("knowledge"));
}
