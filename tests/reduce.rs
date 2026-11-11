//! The move into done (REQ-history-006, REQ-cli-014): the session archives itself, and
//! delivery receipts keep their kinds. Nothing is reduced, trimmed or moved into the vault
//! at done any more (REQ-records-012, REQ-cli-044): proof stays in its session and story.
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::json;

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// One archived native change, a5, a stub `specsync` reporting it archived and
/// authenticated, and the checkpoint at accept on a5, so `stage advance` moves into done.
fn project() -> Fixture {
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
            "---\nid: a5\nstate: archived\nbase_commit: {}\n---\n\n# Title of a5\n\n## Acceptance Criteria\n\n- a5 works\n",
            "3".repeat(40)
        ),
    );
    fixture.init();
    fixture.commit("fixture");
    fixture.set_front("stage", "accept");
    fixture.set_front("native_change", "a5");
    fixture
}

/// Every file under `relative` whose text names `id`, with its path and text.
fn naming(fixture: &Fixture, relative: &str, id: &str) -> Vec<(String, String)> {
    let mut found = vec![];
    let mut stack = vec![fixture.root().join(relative)];
    while let Some(folder) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&folder) else {
            continue;
        };
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else if let Ok(text) = std::fs::read_to_string(&path)
                && text.contains(id)
            {
                let name = path.strip_prefix(fixture.root()).unwrap();
                found.push((name.to_string_lossy().into_owned(), text));
            }
        }
    }
    found
}

/// REQ-cli-014 (#152, audit MA-02): a dry run never stands in for a delivery kind. Each
/// delivery observation is kept through the move into done with its own kind as a receipt,
/// and a dry run's receipt still reads as a dry run. Nothing is trimmed and no receipt is
/// moved or copied into the vault, as the trim at done once did.
#[test]
fn delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not() {
    let fixture = project();
    let kinds = [
        "publication-observation",
        "installation-observation",
        "deployment-observation",
        "live-health-observation",
        "dry-run",
    ];
    let ids: Vec<String> = kinds
        .iter()
        .enumerate()
        .map(|(index, kind)| format!("OBS-{kind}-{:024x}", index + 1))
        .collect();
    for (kind, id) in kinds.iter().zip(&ids) {
        fixture.ok(&[
            "evidence",
            "add",
            id,
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            kind,
            "--outcome",
            "passed",
            "--environment",
            "production",
            "--summary",
            &format!("A SYNTHETIC {kind} of the 1.2.0 release"),
        ]);
    }
    fixture.commit("receipts");
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    assert!(
        moved["applied"].get("librarian_trim").is_none(),
        "{moved:#}"
    );
    let delivery = &kinds[..4];
    for (kind, id) in kinds.iter().zip(&ids) {
        let receipts = naming(&fixture, ".chough", id);
        assert!(
            receipts.iter().any(|(_, text)| text.contains(kind)),
            "{kind}: no receipt keeps {id} with its kind: {receipts:?}"
        );
        assert!(
            naming(&fixture, "knowledge", id).is_empty(),
            "{kind}: the receipt was copied into the vault"
        );
        if *kind == "dry-run" {
            // A receipt of its own reads as a dry run, never as a delivery kind.
            for (path, text) in receipts
                .iter()
                .filter(|(path, _)| path.contains(id.as_str()))
            {
                assert!(
                    !delivery.iter().any(|other| text.contains(other)),
                    "the dry run reads as a delivery kind in {path}: {text}"
                );
            }
        }
    }
    assert!(!fixture.exists("knowledge/attachments"));
}

/// #132: a script a test installs must run even while an old write handle to its path
/// is still open, as a child forked by another test thread holds one until it execs
/// (ETXTBSY, "Text file busy"). Both the `stand_in` helper and `Fixture::executable`
/// must give the script a fresh file.
#[cfg(unix)]
#[test]
fn a_script_installed_while_an_old_write_handle_is_open_still_runs() {
    let fixture = Fixture::new();
    let bin = fixture.root().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    for (name, install) in [
        (
            "via-stand-in",
            Box::new(|path: &std::path::Path| {
                common::stand_in(path.parent().unwrap(), "via-stand-in", "ran");
            }) as Box<dyn Fn(&std::path::Path)>,
        ),
        (
            "via-executable",
            Box::new(|_: &std::path::Path| {
                fixture.write("bin/via-executable", "#!/bin/sh\nprintf 'ran\\n'\n");
                fixture.executable("bin/via-executable");
            }),
        ),
    ] {
        let path = bin.join(name);
        // The leaked handle: open for writing, never closed while the script runs.
        let leaked = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)
            .unwrap();
        install(&path);
        let output = std::process::Command::new(&path)
            .output()
            .unwrap_or_else(|error| panic!("{name}: {error}"));
        assert_eq!(String::from_utf8_lossy(&output.stdout), "ran\n", "{name}");
        drop(leaked);
    }
}

/// #155 (REQ-history-006, DRIVE-13, DRIVE-3): the move into done archives the session
/// itself into one story, in its own change, and clears the active pointer, so no later
/// `session new` in another worktree moves it.
#[test]
fn the_move_into_done_archives_the_session_and_clears_the_pointer() {
    let fixture = project();
    fixture.commit("session at accept");
    let id = fixture.active_id();
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["applied"]["to"], "done", "{moved:#}");
    let archive = &moved["applied"]["session_archive"];
    assert_eq!(archive["status"], "archived", "{moved:#}");
    assert_eq!(archive["id"], id, "{moved:#}");
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    let story = common::parse_session(&fixture.read(&format!(".chough/archive/sessions/{id}.md")));
    assert_eq!(story["stage"]["current"], "done", "{story:#}");
    assert_eq!(story["outcome"], "done", "{story:#}");
    assert!(!fixture.exists(".chough/local/active-session"));
    let next = fixture.fails(&["next"]);
    assert!(next.contains("no open session"), "{next}");
    assert!(next.contains("chough session new <key>"), "{next}");
    let started = fixture.ok(&["session", "new", "beta", "--objective", "Next work"]);
    assert!(started.get("archived").is_none(), "{started:#}");
    let list = fixture.ok(&["session", "list"]);
    assert!(list.to_string().contains("\"archived\":true"), "{list:#}");
}

/// #155: a session with unfolded inbox entries stays open at done.
#[test]
fn the_move_into_done_keeps_a_session_with_unfolded_inbox_entries_open() {
    let fixture = project();
    let id = fixture.active_id();
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &id,
        "--from",
        "worker",
        "--kind",
        "decision",
        "--text",
        "Fold me first",
    ]);
    let moved = fixture.ok(&["stage", "advance"]);
    let archive = &moved["applied"]["session_archive"];
    assert_eq!(archive["status"], "kept_open", "{moved:#}");
    assert_eq!(archive["inbox_unfolded"], 1, "{moved:#}");
    assert!(fixture.exists(&format!(".chough/sessions/{id}/session.md")));
    assert!(!fixture.exists(&format!(".chough/archive/sessions/{id}.md")));
    assert_eq!(
        fixture.read(".chough/local/active-session"),
        format!("{id}\n")
    );
    fixture.ok(&["next", "--json"]);
}

/// #155: when release is authorized at the move into done, the session stays open for
/// it, and archives itself at the move from release into done.
#[test]
fn a_session_with_release_authorized_archives_itself_after_release() {
    let fixture = project();
    let id = fixture.active_id();
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release it",
    ]);
    let done = fixture.ok(&["stage", "advance"]);
    let archive = &done["applied"]["session_archive"];
    assert_eq!(archive["status"], "kept_open", "{done:#}");
    assert!(fixture.exists(&format!(".chough/sessions/{id}/session.md")));
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    let released = fixture.ok(&["stage", "advance"]);
    assert_eq!(released["applied"]["to"], "done", "{released:#}");
    assert_eq!(
        released["applied"]["session_archive"]["status"], "archived",
        "{released:#}"
    );
    assert!(fixture.exists(&format!(".chough/archive/sessions/{id}.md")));
    assert!(!fixture.exists(".chough/local/active-session"));
}

/// REQ-cli-021, as delta correction 1 has it: at done, `chough next` names a new
/// session, unless unfolded inbox entries or an authorized release keep the session
/// open; then it names what closes it, never a new session while it is still open.
#[test]
fn next_at_done_names_what_keeps_the_session_open() {
    // Unfolded inbox entries: fold them, then `chough doctor --fix` archives it.
    let fixture = project();
    let id = fixture.active_id();
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &id,
        "--from",
        "worker",
        "--kind",
        "decision",
        "--text",
        "Fold me first",
    ]);
    fixture.ok(&["stage", "advance"]);
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    assert_eq!(action["command"], "chough inbox fold", "{next:#}");
    let text = action["action"].as_str().unwrap();
    assert!(
        text.contains("1 unfolded inbox entry") && text.contains("chough doctor --fix"),
        "{text}"
    );
    assert!(!text.contains("session new"), "{text}");

    // An authorized release: enter it; the session archives itself after release.
    let fixture = project();
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release it",
    ]);
    fixture.ok(&["stage", "advance"]);
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    assert_eq!(
        action["command"], "chough stage enter release --reason \"<request>\"",
        "{next:#}"
    );
    assert!(
        !action["action"].as_str().unwrap().contains("session new"),
        "{next:#}"
    );

    // Neither: the session archived itself, and next names a new session.
    let fixture = project();
    fixture.ok(&["stage", "advance"]);
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        next["next_valid_action"]["command"]
            .as_str()
            .is_some_and(|command| command.starts_with("chough session new <key>")),
        "{next:#}"
    );
}
