//! Sessions in their own folders (REQ-history-006, REQ-records-004, REQ-history-002,
//! REQ-history-007, REQ-history-009).

mod common;

use common::{Fixture, parse_session, timeline};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;

fn sessions(fixture: &Fixture) {
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["session", "new", "beta", "--objective", "Ship beta"]);
}

fn add_entry(fixture: &Fixture, key: &str, kind: &str, text: &str) -> String {
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        key,
        "--from",
        "subagent-1",
        "--kind",
        kind,
        "--text",
        text,
    ])["id"]
        .as_str()
        .unwrap()
        .to_string()
}

/// Every file under `.chough/sessions/` and `.chough/archive/`, with its bytes.
fn snapshot(fixture: &Fixture) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &std::path::Path, relative: &str, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries {
            let entry = entry.unwrap();
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if entry.file_type().unwrap().is_dir() {
                walk(root, &name, out);
            } else {
                out.insert(name, fs::read(entry.path()).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(fixture.root(), ".chough/sessions", &mut out);
    walk(fixture.root(), ".chough/archive", &mut out);
    out
}

/// Advance the active session from where it is to `stage`.
fn advance_to(fixture: &Fixture, stage: &str) {
    for _ in 0..8 {
        if fixture.checkpoint()["stage"]["current"] == stage {
            return;
        }
        fixture.ok(&["stage", "advance", "--json"]);
    }
    panic!("never reached {stage}");
}

/// Give `key` the native change `change` and archive that change on HEAD, as a move
/// into done that timed out after SpecSync finalized it leaves things.
fn finalized_change(fixture: &Fixture, key: &str, change: &str) {
    fixture.ok(&["session", "use", key]);
    fixture.set_front("native_change", change);
    fixture.write(
        &format!(".specsync/archive/changes/2026-11-06-{change}/finalization.json"),
        &format!("{{\"schema_version\": 2, \"change_id\": \"{change}\", \"timestamp\": 700}}\n"),
    );
}

#[cfg(unix)]
fn no_native_reads(fixture: &Fixture) {
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\necho \"$*\" >>.native-calls\nexit 2\n",
    );
    fixture.executable("bin/specsync");
}

#[test]
fn each_session_has_its_own_folder_and_timeline() {
    let fixture = Fixture::new();
    sessions(&fixture);
    let (alpha, beta) = (fixture.id_of("alpha"), fixture.id_of("beta"));
    assert!(
        alpha.ends_with("-alpha") && alpha.len() == "2026-11-11-alpha".len(),
        "{alpha}"
    );
    assert!(!fixture.exists(".chough/session.json"));
    assert_eq!(fixture.read(".chough/local/active-session").trim(), beta);
    assert_eq!(fixture.read(".chough/local/.gitignore"), "*\n");
    let alpha_file = fixture.session_file_of("alpha");
    let before = fixture.read(&alpha_file);
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        fixture.read(&alpha_file),
        before,
        "another session is untouched"
    );
    let beta_text = fixture.read(&fixture.session_file_of("beta"));
    let beta_session = parse_session(&beta_text);
    assert_eq!(beta_session["objective"], "Ship beta");
    assert_eq!(beta_session["aliases"], json!(["beta"]));
    assert_eq!(beta_session["stage"]["current"], "design");
    // Each timeline starts with its own session's start entry (REQ-history-002).
    let alpha_entries = timeline(&before);
    assert_eq!(alpha_entries.len(), 1, "{before}");
    assert!(alpha_entries[0].contains(" · start · intent → intent · Ship alpha ^t-"));
    let entries = timeline(&beta_text);
    assert_eq!(entries.len(), 2, "{beta_text}");
    assert!(entries[0].contains(" · start · intent → intent · Ship beta ^t-"));
    assert!(
        entries[1].contains(" · advance · intent → design · "),
        "{entries:?}"
    );
    assert!(!fixture.exists(".chough/history"));
    assert!(!fixture.exists(".chough/transitions.jsonl"));
    let refused = fixture.fails(&["session", "new", "beta"]);
    assert!(refused.contains("already used"), "{refused}");
    assert!(
        fixture
            .fails(&["init", "--objective", "x"])
            .contains("already has an open session")
    );
    let list = fixture.ok(&["session", "list"]);
    let ids: Vec<_> = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|session| {
            (
                session["id"].as_str().unwrap().to_string(),
                session["active"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(ids, [(alpha, false), (beta, true)]);
}

#[test]
fn commands_use_the_active_session_and_refuse_when_unclear() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha"]);
    fs::remove_file(fixture.root().join(".chough/local/active-session")).unwrap();
    // The only open session is active without a pointer.
    fixture.ok(&["checkpoint", "--decision", "alpha decision"]);
    assert_eq!(
        fixture.session_json("alpha")["decisions"][0],
        "alpha decision"
    );
    fixture.ok(&["session", "new", "beta"]);
    fs::remove_file(fixture.root().join(".chough/local/active-session")).unwrap();
    let before = snapshot(&fixture);
    let refused = fixture.fails(&["checkpoint", "--decision", "lost"]);
    assert!(refused.contains("chough session use"), "{refused}");
    fixture.write(".chough/local/active-session", "2026-01-01-gamma\n");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("gamma"), "{refused}");
    assert_eq!(before, snapshot(&fixture));
    // An alias and the full ID both name a session.
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["checkpoint", "--decision", "second"]);
    assert_eq!(fixture.session_json("alpha")["decisions"][1], "second");
    let beta = fixture.id_of("beta");
    fixture.ok(&["session", "use", &beta]);
    assert_eq!(fixture.read(".chough/local/active-session").trim(), beta);
    assert!(
        fixture
            .fails(&["session", "use", "gamma"])
            .contains("gamma")
    );
}

/// Q3 (#207, and the spec-sync repro of 2026-11-11): starting or switching work never
/// archives, closes or changes another session, even a finished one, so an unrelated
/// change carries no other session's files. A finished open session is listed with its
/// fix instead.
#[test]
fn starting_or_switching_never_touches_another_session() {
    let fixture = Fixture::new();
    sessions(&fixture);
    // alpha reaches done with release authorized, so it stays open (#155).
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release alpha",
    ]);
    advance_to(&fixture, "done");
    assert_eq!(fixture.session_json("alpha")["stage"]["current"], "done");
    // beta is finished because its native change is archived; it still waits on a gate.
    finalized_change(&fixture, "beta", "ch-beta");
    fixture.ok(&["checkpoint", "--awaiting-human", "Publish beta"]);
    fixture.commit("finished sessions");
    let before = snapshot(&fixture);
    let created = fixture.ok(&["session", "new", "gamma", "--objective", "Ship gamma"]);
    assert!(created.get("archived").is_none(), "{created}");
    let gamma = fixture.id_of("gamma");
    let mut after = snapshot(&fixture);
    after.retain(|path, _| !path.contains(&gamma));
    assert_eq!(before, after, "session new touched another session's files");
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["session", "use", &gamma]);
    let mut after = snapshot(&fixture);
    after.retain(|path, _| !path.contains(&gamma));
    assert_eq!(before, after, "session use touched another session's files");
    let status = std::process::Command::new("git")
        .args([
            "status",
            "--porcelain",
            "--",
            ".chough/sessions",
            ".chough/archive",
        ])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    let changed = String::from_utf8_lossy(&status.stdout);
    assert!(
        changed.lines().all(|line| line.contains(&gamma)),
        "only the new session's folder is new: {changed}"
    );
    let list = fixture.ok(&["session", "list"]);
    for key in ["alpha", "beta"] {
        let id = fixture.id_of(key);
        let entry = list["sessions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|session| session["id"] == id)
            .unwrap()
            .clone();
        assert_eq!(entry["archived"], false, "{entry}");
        assert_eq!(entry["finished"], true, "{entry}");
        assert_eq!(entry["fix"], "chough doctor --fix", "{entry}");
    }
}

/// DRIVE-16 (chough#163): a session whose native change is archived on HEAD is finished
/// even short of done, so it is not one of several open sessions, without a native read;
/// it stays open until `chough doctor --fix` archives it (Q3).
#[cfg(unix)]
#[test]
fn a_session_whose_native_change_is_archived_is_finished_without_a_native_read() {
    let fixture = Fixture::new();
    sessions(&fixture);
    finalized_change(&fixture, "alpha", "ch-alpha");
    no_native_reads(&fixture);
    fs::remove_file(fixture.root().join(".chough/local/active-session")).unwrap();
    let alpha = fixture.read(&fixture.session_file_of("alpha"));
    // Only beta is still open work, so it is active without a pointer.
    fixture.ok(&["checkpoint", "--decision", "beta decision"]);
    assert_eq!(
        fixture.session_json("beta")["decisions"][0],
        "beta decision"
    );
    assert_eq!(fixture.read(&fixture.session_file_of("alpha")), alpha);
    fixture.ok(&["session", "new", "gamma", "--objective", "Ship gamma"]);
    assert_eq!(fixture.read(&fixture.session_file_of("alpha")), alpha);
    assert!(!fixture.is_archived("alpha"));
    assert!(
        !fixture.exists(".native-calls"),
        "{}",
        fixture.read(".native-calls")
    );
}

/// TC-history-sessions-in-folders: the move into done of the active session writes one
/// story with its full timeline and a closing line, removes each committed file of the
/// session folder, keeps and reports one that is not committed, clears the pointer, and
/// `next` then names `chough session new <key>`.
#[test]
fn sessions_live_in_folders_and_finish_into_one_story() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "Keep it small",
        "--completed",
        "Planned",
    ]);
    advance_to(&fixture, "accept");
    let id = fixture.id_of("alpha");
    fixture.write(
        &format!(".chough/sessions/{id}/agent/notes.md"),
        "scratch\n",
    );
    fixture.commit("alpha at accept");
    let session_text = fixture.read(&fixture.session_file_of("alpha"));
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    let archive = &moved["applied"]["session_archive"];
    assert_eq!(archive["status"], "archived", "{moved:#}");
    let story_path = format!(".chough/archive/sessions/{id}.md");
    assert_eq!(archive["story"], story_path);
    let story = fixture.read(&story_path);
    let parsed = parse_session(&story);
    assert_eq!(parsed["stage"]["current"], "done");
    assert_eq!(parsed["outcome"], "done");
    assert_eq!(parsed["decisions"], json!(["Keep it small"]));
    assert!(story.contains("\nreceipts: []\n"), "{story}");
    let entries = timeline(&story);
    assert!(entries.len() > timeline(&session_text).len(), "{story}");
    assert!(
        entries
            .last()
            .unwrap()
            .contains(" · advance · accept → done · ")
    );
    // The closing line ends the story, before its `## Receipts` (REQ-history-006).
    let (told, _) = story
        .split_once("\n## Receipts\n")
        .unwrap_or_else(|| panic!("no Receipts section: {story}"));
    assert!(told.trim_end().ends_with(": done."), "{story}");
    // session.md changed with the move, so it stays until it is committed; the
    // committed agent note goes.
    assert_eq!(
        archive["kept"],
        json!([format!(".chough/sessions/{id}/session.md")])
    );
    assert!(!fixture.exists(&format!(".chough/sessions/{id}/agent/notes.md")));
    assert!(!fixture.exists(".chough/local/active-session"));
    // REQ-cli-021 (#371): while the folder still holds session.md, `next` names
    // finishing the archive before a new session.
    let next = fixture.ok_text(&["next"]);
    assert!(
        next.contains(&format!("Finish the archive of session {id}")),
        "{next}"
    );
    // With no session open, `history show` reads the story it finished into.
    let shown = fixture.ok(&["history", "show"]);
    assert_eq!(shown["session"], id.as_str(), "{shown:#}");
    assert_eq!(shown["archived"], true, "{shown:#}");
    let last = shown["entries"].as_array().unwrap().last().unwrap().clone();
    assert_eq!(
        (&last["from"], &last["to"]),
        (&json!("accept"), &json!("done"))
    );
    let list = fixture.ok(&["session", "list"]);
    assert_eq!(list["sessions"][0]["archived"], true, "{list:#}");
    assert_eq!(list["sessions"].as_array().unwrap().len(), 1, "{list:#}");
}

/// #165 (KNOW-34, REQ-history-006): the move into done never removes or moves another
/// session's archived story. With seven earlier stories and `knowledge/` not committed,
/// accept to done leaves every earlier story byte-identical in place, Git shows no
/// deleted file, and the only new record is the new story.
#[test]
fn done_leaves_earlier_archived_stories_in_place() {
    let fixture = Fixture::new();
    for index in 0..7 {
        let key = format!("earlier-{index}");
        fixture.ok(&[
            "session",
            "new",
            &key,
            "--objective",
            &format!("Earlier objective {index}"),
        ]);
        fixture.ok(&["session", "close", &key, "--reason", "earlier work"]);
    }
    fixture.ok(&["session", "new", "current", "--objective", "Ship current"]);
    advance_to(&fixture, "accept");
    fixture.commit("seven earlier stories");
    fixture.git(&[
        "rm",
        "-r",
        "-q",
        "--cached",
        "--ignore-unmatch",
        "knowledge",
    ]);
    fixture.commit("knowledge/ not committed");
    let earlier: Vec<(String, String)> = (0..7)
        .map(|index| {
            let path = fixture.story_of(&format!("earlier-{index}"));
            let text = fixture.read(&path);
            (path, text)
        })
        .collect();
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        moved["applied"]["session_archive"]["status"], "archived",
        "{moved:#}"
    );
    for (path, text) in &earlier {
        assert!(fixture.exists(path), "{path} left its place: {moved:#}");
        assert_eq!(&fixture.read(path), text, "{path} changed");
    }
    let status = std::process::Command::new("git")
        .args(["status", "--porcelain", "--", ".chough"])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    let changed = String::from_utf8_lossy(&status.stdout).to_string();
    assert!(
        !changed
            .lines()
            .any(|line| line.starts_with(" D") || line.starts_with("D ")),
        "{changed}"
    );
    let current = fixture.id_of("current");
    let new_files: Vec<&str> = changed
        .lines()
        .filter(|line| line.starts_with("??"))
        .collect();
    assert_eq!(
        new_files,
        [format!("?? .chough/archive/sessions/{current}.md")],
        "{changed}"
    );
}

#[test]
fn session_list_reports_an_archived_session_as_archived() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&[
        "session",
        "close",
        "alpha",
        "--reason",
        "superseded by beta",
    ]);
    let list = fixture.ok(&["session", "list"]);
    let alpha = fixture.id_of("alpha");
    let entry = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|session| session["id"] == alpha)
        .unwrap()
        .clone();
    assert_eq!(entry["archived"], true, "{entry}");
    assert_eq!(entry["status"], "archived", "{entry}");
    let refused = fixture.fails(&["session", "use", "alpha"]);
    assert!(refused.contains("archived"), "{refused}");
}

/// Fresh-eyes finding F12c (REQ-history-006): an archived story is finished, whether
/// it closed at done or without done, so `session list` reports it `finished: true`
/// with no fix to run; an open session short of done stays unfinished.
#[test]
fn session_list_reports_an_archived_story_as_finished() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&[
        "session",
        "close",
        "alpha",
        "--reason",
        "superseded by beta",
    ]);
    fixture.ok(&["session", "use", "beta"]);
    advance_to(&fixture, "accept");
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        moved["applied"]["session_archive"]["status"], "archived",
        "{moved:#}"
    );
    fixture.ok(&["session", "new", "gamma", "--objective", "Ship gamma"]);
    let list = fixture.ok(&["session", "list"]);
    let entry = |key: &str| {
        let id = fixture.id_of(key);
        list["sessions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|session| session["id"] == id)
            .unwrap_or_else(|| panic!("{key} is not listed: {list:#}"))
            .clone()
    };
    for key in ["alpha", "beta"] {
        let story = entry(key);
        assert_eq!(story["archived"], true, "{key}: {story}");
        assert_eq!(story["finished"], true, "{key}: {story}");
        assert_eq!(story["fix"], Value::Null, "{key}: {story}");
    }
    let open = entry("gamma");
    assert_eq!(open["archived"], false, "{open}");
    assert_eq!(open["finished"], false, "{open}");
    assert_eq!(open["fix"], Value::Null, "{open}");
}

/// One command case, `TC-cli-0001`, that prints ok.
const ONE_CASE: &str = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"It prints ok\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/input.txt\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"it prints ok\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo ok\"]\nexpect_output = [\"ok\"]\n";

/// The item lines of a record's What was verified section.
fn verified_lines(text: &str) -> Vec<String> {
    let start = text
        .find("\n## What was verified\n")
        .unwrap_or_else(|| panic!("no What was verified: {text}"));
    let section = &text[start + 1..];
    let section = &section[..section[3..]
        .find("\n## ")
        .map_or(section.len(), |at| at + 3)];
    section
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(String::from)
        .collect()
}

/// The front matter lines of a record.
fn front_lines(text: &str) -> Vec<String> {
    text.strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .unwrap_or_else(|| panic!("no front matter: {text}"))
        .0
        .lines()
        .map(String::from)
        .collect()
}

/// Fresh-eyes finding F8 (REQ-history-006): a story's What was verified gives the
/// latest run of each case and every entry bound to no case, one line each, a command
/// once, and one line counting the case's earlier runs, which stay in `receipts`.
#[test]
fn a_story_lists_each_cases_latest_run_once_and_counts_the_earlier_ones() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".chough/cases/cli.cases.toml", ONE_CASE);
    fixture.init();
    fixture.commit("start");
    let mut runs = vec![];
    for _ in 0..3 {
        if !runs.is_empty() {
            // Each run is observed in a later second than the one before it.
            std::thread::sleep(std::time::Duration::from_millis(1_100));
        }
        let ran = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
        runs.push(
            ran["observation"]
                .as_str()
                .unwrap_or_else(|| panic!("{ran:#}"))
                .to_string(),
        );
    }
    let added = fixture.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "SYNTHETIC manual check",
    ]);
    let manual = added["observation"]
        .as_str()
        .unwrap_or_else(|| panic!("{added:#}"))
        .to_string();
    let id = fixture.active_id();
    fixture.commit("the runs");
    fixture.ok(&["session", "close", &id, "--reason", "SYNTHETIC finished"]);
    let story = fixture.read(&format!(".chough/archive/sessions/{id}.md"));
    let lines = verified_lines(&story);
    // By the `^<id>` a line ends with: one run's ID can be the start of another's.
    let naming = |entry: &str| -> Vec<String> {
        lines
            .iter()
            .filter(|line| line.ends_with(&format!("^{entry}")))
            .cloned()
            .collect()
    };
    let latest = naming(&runs[2]);
    assert_eq!(latest.len(), 1, "{lines:#?}");
    assert_eq!(
        latest[0].matches("sh -c echo ok").count(),
        1,
        "the command is repeated: {}",
        latest[0]
    );
    for earlier in &runs[..2] {
        assert!(naming(earlier).is_empty(), "{earlier} listed: {lines:#?}");
    }
    assert_eq!(naming(&manual).len(), 1, "{lines:#?}");
    assert_eq!(
        lines
            .iter()
            .filter(|line| line.contains("2 earlier runs"))
            .count(),
        1,
        "{lines:#?}"
    );
    assert_eq!(lines.len(), 3, "{lines:#?}");
    for entry in runs.iter().chain([&manual]) {
        assert!(
            story.contains(&format!("\n  - id: {entry}\n")),
            "no receipt for {entry}: {story}"
        );
    }
}

/// Fresh-eyes finding Kimi 6 (REQ-history-006): an archived story's front matter gives
/// `stage_status: archived` and no `next`, whether it closed at done or without done,
/// and every reader still reads it.
#[test]
fn an_archived_story_says_archived_and_keeps_no_next() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&[
        "session",
        "close",
        "alpha",
        "--reason",
        "superseded by beta",
    ]);
    fixture.ok(&["session", "use", "beta"]);
    advance_to(&fixture, "accept");
    fixture.ok(&[
        "checkpoint",
        "--next-action",
        "SYNTHETIC: a next action that goes stale at done",
    ]);
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        moved["applied"]["session_archive"]["status"], "archived",
        "{moved:#}"
    );
    for key in ["alpha", "beta"] {
        let story = fixture.read(&fixture.story_of(key));
        let front = front_lines(&story);
        assert!(
            front.iter().any(|line| line == "stage_status: archived"),
            "{key}: {front:#?}"
        );
        assert!(
            !front.iter().any(|line| line.starts_with("next:")),
            "{key}: {front:#?}"
        );
    }
    // Every reader still reads both stories.
    let list = fixture.ok(&["session", "list"]);
    let listed = list["sessions"].as_array().unwrap();
    assert_eq!(listed.len(), 2, "{list:#}");
    for entry in listed {
        assert_eq!(entry["status"], "archived", "{entry}");
        assert!(entry.get("readable").is_none(), "{entry}");
    }
    let beta = fixture.id_of("beta");
    let shown = fixture.ok(&["history", "show"]);
    assert_eq!(shown["session"], beta.as_str(), "{shown:#}");
    assert_eq!(shown["archived"], true, "{shown:#}");
    fixture.ok(&["timing", "--json"]);
    // The kept, uncommitted session files are findings here; no story is unreadable.
    let doctor = fixture.run(&["doctor"]);
    let report = String::from_utf8_lossy(&doctor.stdout);
    assert!(
        report.contains("\"findings\"") && !report.contains("unreadable"),
        "{report}{}",
        String::from_utf8_lossy(&doctor.stderr)
    );
}

/// DRIVE-16: `chough session close <key> --reason` archives abandoned work with the
/// reason in its decisions and outcome `closed-without-done`; a blank reason, an
/// archived or missing session and one with unfolded inbox entries refuse unchanged.
#[test]
fn session_close_archives_abandoned_work_with_its_reason() {
    let fixture = Fixture::new();
    sessions(&fixture);
    add_entry(&fixture, "beta", "decision", "pending");
    let before = snapshot(&fixture);
    assert!(
        fixture
            .fails(&["session", "close", "alpha", "--reason", "  "])
            .contains("blank")
    );
    assert!(
        fixture
            .fails(&["session", "close", "nope", "--reason", "x"])
            .contains("no session nope")
    );
    assert!(
        fixture
            .fails(&["session", "close", "beta", "--reason", "x"])
            .contains("unfolded")
    );
    assert_eq!(before, snapshot(&fixture));
    fixture.commit("two sessions");
    let closed = fixture.ok(&[
        "session",
        "close",
        "alpha",
        "--reason",
        "Abandoned for beta",
    ]);
    let alpha = fixture.id_of("alpha");
    assert_eq!(closed["closed"], alpha);
    let story = parse_session(&fixture.read(&fixture.story_of("alpha")));
    assert_eq!(story["outcome"], "closed-without-done");
    assert_eq!(
        story["decisions"],
        json!(["Closed without done: Abandoned for beta"])
    );
    // Its session.md was committed, so the folder goes.
    assert!(
        !fixture.exists(&format!(".chough/sessions/{alpha}")),
        "{closed}"
    );
    assert!(
        fixture
            .fails(&["session", "close", "alpha", "--reason", "x"])
            .contains("archived")
    );
}

/// F2 of the third fresh-eyes round (REQ-history-006): `chough session close` on a
/// session with an open human gate withdraws it as `--clear-awaiting-human` does, one
/// `gate-closed` entry with no relayed decision, names it under `withdrawn_gates`, and
/// the story lists no gate as awaiting, so no gate is dropped silently.
#[test]
fn session_close_records_the_gates_it_withdraws() {
    const GATE: &str = "Approve the alpha intent";
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["checkpoint", "--awaiting-human", GATE]);
    fixture.ok(&["session", "use", "beta"]);
    fixture.commit("two sessions, alpha with an open gate");
    let closed = fixture.ok(&[
        "session",
        "close",
        "alpha",
        "--reason",
        "Abandoned for beta",
    ]);
    assert_eq!(closed["withdrawn_gates"], json!([GATE]), "{closed}");
    let text = fixture.read(&fixture.story_of("alpha"));
    let story = parse_session(&text);
    assert_eq!(story["awaiting_human"], json!([]), "{text}");
    let closed_entries: Vec<&str> = text
        .lines()
        .filter(|line| line.contains(" · gate-closed · ") && line.contains(GATE))
        .collect();
    assert_eq!(closed_entries.len(), 1, "{text}");
    assert!(
        !text.contains("relayed_by"),
        "a decision was recorded: {text}"
    );
    assert_eq!(
        story["decisions"],
        json!(["Closed without done: Abandoned for beta"])
    );
}

#[test]
fn inbox_entries_fold_once_without_touching_the_session() {
    let fixture = Fixture::new();
    sessions(&fixture);
    let alpha_file = fixture.session_file_of("alpha");
    let before = fixture.read(&alpha_file);
    for (kind, text) in [
        ("observation", "OBS-parser-slow"),
        ("decision", "keep the cache"),
        ("completed", "wrote the parser test"),
        ("question", "is the cache shared?"),
        ("observation", "OBS-parser-slow"),
    ] {
        add_entry(&fixture, "alpha", kind, text);
    }
    assert_eq!(fixture.read(&alpha_file), before);
    let inbox = fixture.root().join(format!(
        ".chough/sessions/{}/agent/inbox",
        fixture.id_of("alpha")
    ));
    assert_eq!(fs::read_dir(&inbox).unwrap().count(), 5);
    assert!(
        fixture
            .fails(&[
                "inbox",
                "add",
                "--session",
                "alpha",
                "--from",
                "x",
                "--kind",
                "approval",
                "--text",
                "approved"
            ])
            .contains("kind")
    );
    assert!(
        fixture
            .fails(&[
                "inbox",
                "add",
                "--session",
                "nope",
                "--from",
                "x",
                "--kind",
                "observation",
                "--text",
                "t"
            ])
            .contains("nope")
    );
    fixture.ok(&["session", "use", "alpha"]);
    let folded = fixture.ok(&["inbox", "fold"]);
    assert_eq!(folded["folded"].as_array().unwrap().len(), 5);
    let session = fixture.session_json("alpha");
    let observations: Vec<_> = session["observations"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|o| o.as_str().unwrap().contains("OBS-parser-slow"))
        .collect();
    assert_eq!(observations.len(), 1, "{session}");
    assert!(session["decisions"].to_string().contains("keep the cache"));
    assert!(
        session["completed"]
            .to_string()
            .contains("wrote the parser test")
    );
    assert!(
        session["open_questions"]
            .to_string()
            .contains("is the cache shared?")
    );
    assert_eq!(session["awaiting_human"], json!([]));
    assert_eq!(fs::read_dir(inbox.join("folded")).unwrap().count(), 5);
    let after = fixture.read(&alpha_file);
    let again = fixture.ok(&["inbox", "fold"]);
    assert_eq!(again["folded"], json!([]));
    assert_eq!(fixture.read(&alpha_file), after);
    // An entry that tries to carry an approval is refused and nothing is folded.
    add_entry(&fixture, "alpha", "observation", "ok");
    let entry = fs::read_dir(&inbox)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.is_file())
        .unwrap();
    let text = fs::read_to_string(&entry).unwrap();
    assert!(text.starts_with("---\nkind: observation\n"), "{text}");
    fs::write(
        &entry,
        text.replacen("---\n", "---\napproval: approved by Leif\n", 1),
    )
    .unwrap();
    assert!(fixture.fails(&["inbox", "fold"]).contains("inbox entry"));
    assert_eq!(fixture.read(&alpha_file), after);
}

/// At the move into done, a session with unfolded inbox entries stays open with the
/// count, and keeps its pointer.
#[test]
fn a_done_session_with_unfolded_entries_stays_open() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    advance_to(&fixture, "accept");
    add_entry(&fixture, "alpha", "decision", "late");
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    let archive = &moved["applied"]["session_archive"];
    assert_eq!(archive["status"], "kept_open", "{moved:#}");
    assert_eq!(archive["inbox_unfolded"], 1, "{moved:#}");
    assert!(!fixture.is_archived("alpha"));
    assert_eq!(
        fixture.read(".chough/local/active-session").trim(),
        fixture.id_of("alpha")
    );
}

/// #98: an add holds the writer lock from before it checks the session; an archive
/// under that lock leaves the add refusing with nothing written.
#[test]
fn inbox_add_racing_an_archive_never_strands_its_entry() {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};
    let fixture = Fixture::new();
    sessions(&fixture);
    let alpha = fixture.id_of("alpha");
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(common::checkpoint_lock_path(fixture.root()).unwrap())
        .unwrap();
    <fs::File as fs4::FileExt>::lock(&lock).unwrap();
    let mut add = Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args([
            "inbox",
            "add",
            "--session",
            "alpha",
            "--from",
            "subagent-1",
            "--kind",
            "decision",
            "--text",
            "Late decision",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(2) && add.try_wait().unwrap().is_none() {
        std::thread::sleep(Duration::from_millis(25));
    }
    // Archive alpha while the add waits: its story exists from now on.
    let story = fixture.read(&fixture.session_file_of("alpha"));
    fixture.write(&format!(".chough/archive/sessions/{alpha}.md"), &story);
    drop(lock);
    let output = add.wait_with_output().unwrap();
    let stranded: Vec<_> = fs::read_dir(
        fixture
            .root()
            .join(format!(".chough/sessions/{alpha}/agent/inbox")),
    )
    .map(|entries| {
        entries
            .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".md"))
            .collect()
    })
    .unwrap_or_default();
    assert!(
        stranded.is_empty(),
        "entry {stranded:?} stranded beside archived alpha"
    );
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no open session alpha"), "{stderr}");
}

/// #99 review (REQ-history-009): an entry is published whole, so an `inbox add` killed
/// while writing leaves no partial entry; the temp it left is ignored by Git and the
/// next save removes it.
#[cfg(unix)]
#[test]
fn an_inbox_add_killed_while_writing_leaves_no_partial_entry() {
    use std::os::unix::process::ExitStatusExt;
    let fixture = Fixture::new();
    sessions(&fixture);
    let kept = add_entry(&fixture, "beta", "decision", "Before the killed add");
    // One block of file size lets the writer lock record its process ID (REQ-cli-040),
    // and the entry, longer than that, is stopped while it is written.
    let killed = std::process::Command::new("sh")
        .args(["-c", "ulimit -f 1; exec \"$@\"", "sh"])
        .arg(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args([
            "inbox",
            "add",
            "--session",
            "beta",
            "--from",
            "subagent-1",
            "--kind",
            "decision",
            "--text",
            &format!("Killed while writing {}", "x".repeat(3_000)),
        ])
        .output()
        .unwrap();
    assert!(
        killed.status.signal().is_some(),
        "not stopped while writing: {killed:?}"
    );
    let entries = fixture.ok(&["inbox", "list"])["entries"].clone();
    let ids: Vec<&str> = entries
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, [kept.as_str()]);
    fixture.ok(&["status"]);
    fixture.ok(&["next", "--json"]);
    // A record's temporary sits in the Git-ignored .chough/local/, never beside the
    // record (REQ-records-002), and nothing is left in the inbox.
    let beta = fixture.id_of("beta");
    let in_inbox = fs::read_dir(
        fixture
            .root()
            .join(format!(".chough/sessions/{beta}/agent/inbox")),
    )
    .unwrap()
    .filter(|entry| {
        entry
            .as_ref()
            .unwrap()
            .file_name()
            .to_string_lossy()
            .ends_with(".tmp")
    })
    .count();
    assert_eq!(in_inbox, 0);
    let inbox = fixture.root().join(".chough/local");
    let temps: Vec<String> = fs::read_dir(&inbox)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert_eq!(temps.len(), 1, "{temps:?}");
    fixture.commit("after the killed add");
    let temp = format!(".chough/local/{}", temps[0]);
    let ignored = std::process::Command::new("git")
        .args(["check-ignore", "-q", "--no-index", &temp])
        .current_dir(fixture.root())
        .status()
        .unwrap();
    assert!(ignored.success(), "{temp} is not ignored by Git");
    fixture.ok(&["checkpoint", "--decision", "after the killed add"]);
    assert!(
        !inbox.join(&temps[0]).exists(),
        "{} was left behind",
        temps[0]
    );
    assert_eq!(fixture.ok(&["inbox", "list"])["entries"], entries);
}

#[test]
fn an_unrecorded_inbox_observation_is_added_with_a_warning() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    // A recorded observation: a 0.6.0 evidence entry of the session (REQ-records-005).
    let recorded_id = "2026-01-01-recorded";
    fixture.write(
        &format!(
            ".chough/sessions/{}/evidence/{recorded_id}.md",
            fixture.id_of("alpha")
        ),
        &format!(
            "---\nkind: evidence\nid: {recorded_id}\naliases: []\nclaim: a recorded observation\nresult: passed\nevidence_kind: product-test\ncase: []\ncase_digest: []\ncriteria: []\nrequirements: []\ncontracts: []\nenvironment: local\nrevision: {}\nobserved: 2026-01-01T00:00:00Z\n---\n**a recorded observation.**\n",
            "0".repeat(40)
        ),
    );
    let output = fixture.run(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "subagent-1",
        "--kind",
        "observation",
        "--text",
        "OBS-bogus",
    ]);
    assert!(output.status.success(), "the entry is still added");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("OBS-bogus") && stderr.contains("no record"),
        "{stderr}"
    );
    let output = fixture.run(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "subagent-1",
        "--kind",
        "observation",
        "--text",
        recorded_id,
    ]);
    assert!(output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("no record"));
    let list = fixture.ok(&["inbox", "list"]);
    let recorded = |text: &str| {
        list["entries"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["text"] == text)
            .map(|entry| entry["recorded"].clone())
            .unwrap_or_else(|| panic!("{text} missing: {list:#}"))
    };
    assert_eq!(recorded("OBS-bogus"), false, "{list:#}");
    assert_eq!(recorded(recorded_id), true, "{list:#}");
}

/// DRIVE-14: an unreadable session is listed with `chough session shelve`, `next` on
/// another session and `session new` still work, and shelving moves the folder byte for
/// byte under `.chough/archive/shelved/` with a `shelved.md` giving its size, each file's
/// SHA-256, the time and the reason (REQ-history-006).
#[test]
fn an_unreadable_session_is_listed_and_shelving_moves_its_folder_byte_for_byte() {
    let fixture = Fixture::new();
    sessions(&fixture);
    let alpha = fixture.id_of("alpha");
    let file = format!(".chough/sessions/{alpha}/session.md");
    fixture.write(&file, "not a record\n");
    let kept = format!(".chough/sessions/{alpha}/agent/SYNTHETIC-kept.txt");
    fixture.write(&kept, "SYNTHETIC bytes kept as they are\n");
    let list = fixture.ok(&["session", "list"]);
    let entry = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|session| session["id"] == alpha)
        .unwrap()
        .clone();
    assert_eq!(entry["readable"], false, "{entry}");
    assert_eq!(entry["recovery"], format!("chough session shelve {alpha}"));
    fixture.ok(&["next", "--json"]);
    fixture.ok(&["session", "new", "gamma", "--objective", "Ship gamma"]);
    let shelved = fixture.ok(&["session", "shelve", &alpha]);
    assert_eq!(
        shelved["shelved"],
        format!(".chough/archive/shelved/{alpha}")
    );
    assert_eq!(
        fixture.read(&format!(".chough/archive/shelved/{alpha}/session.md")),
        "not a record\n"
    );
    assert_eq!(
        fixture.read(&format!(
            ".chough/archive/shelved/{alpha}/agent/SYNTHETIC-kept.txt"
        )),
        "SYNTHETIC bytes kept as they are\n"
    );
    assert!(!fixture.exists(&format!(".chough/sessions/{alpha}")));
    // The note is inside the shelved folder; nothing else is written beside it.
    let note = format!(".chough/archive/shelved/{alpha}/shelved.md");
    assert_eq!(shelved["record"], note.as_str(), "{shelved:#}");
    let names: Vec<String> = fs::read_dir(fixture.root().join(".chough/archive/shelved"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names, std::slice::from_ref(&alpha));
    let text = fixture.read(&note);
    let front = |key: &str| -> String {
        let (front, _) = text
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .unwrap_or_else(|| panic!("no front matter in {text}"));
        front
            .lines()
            .find_map(|line| line.strip_prefix(&format!("{key}: ")))
            .unwrap_or_default()
            .trim_matches('"')
            .to_string()
    };
    assert_eq!(front("kind"), "shelved", "{text}");
    assert_eq!(front("id"), alpha, "{text}");
    assert_eq!(front("from"), format!(".chough/sessions/{alpha}"), "{text}");
    assert_eq!(front("reason"), "not a readable session record", "{text}");
    assert!(front("shelved").ends_with('Z'), "{text}");
    let total = "not a record\n".len() + "SYNTHETIC bytes kept as they are\n".len();
    assert_eq!(front("bytes"), total.to_string(), "{text}");
    for (path, bytes) in [
        ("session.md", "not a record\n"),
        (
            "agent/SYNTHETIC-kept.txt",
            "SYNTHETIC bytes kept as they are\n",
        ),
    ] {
        let line = format!(
            "- `{path}` {} bytes {:x}",
            bytes.len(),
            <sha2::Sha256 as sha2::Digest>::digest(bytes.as_bytes())
        );
        assert!(text.contains(&line), "no {line:?} in {text}");
    }
}

/// REQ-history-007: `.chough/local/` holds only the pointer and the writer lock here,
/// and Git ignores all of it.
#[test]
fn local_holds_only_the_pointer_and_the_lock() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&["stage", "advance", "--json"]);
    let mut names: Vec<String> = fs::read_dir(fixture.root().join(".chough/local"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(names, [".gitignore", "active-session", "checkpoint.lock"]);
    fixture.commit("sessions");
    let tracked = std::process::Command::new("git")
        .args(["ls-files", ".chough/local"])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    assert!(
        tracked.stdout.is_empty(),
        "{}",
        String::from_utf8_lossy(&tracked.stdout)
    );
}

/// Two branches that each advance a different session merge with no conflict and keep
/// both timelines (REQ-history-002).
#[test]
fn branches_with_different_sessions_merge_without_conflict() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.commit("base");
    fixture.git(&["checkout", "-q", "-b", "work-alpha"]);
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.commit("alpha moves");
    fixture.git(&["checkout", "-q", "-"]);
    fixture.ok(&["session", "use", "beta"]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.commit("beta moves");
    fixture.git(&["merge", "-q", "--no-edit", "work-alpha"]);
    // Each keeps its start entry and its own move.
    assert_eq!(
        timeline(&fixture.read(&fixture.session_file_of("alpha"))).len(),
        2
    );
    assert_eq!(
        timeline(&fixture.read(&fixture.session_file_of("beta"))).len(),
        2
    );
}

#[test]
fn a_folded_inbox_question_appears_in_next() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let question = "Should SAVE20 stack with SAVE10?";
    add_entry(&fixture, "alpha", "question", question);
    fixture.ok(&["inbox", "fold"]);
    let output = fixture.run(&["next"]);
    assert!(output.status.success());
    let next = String::from_utf8(output.stdout).unwrap();
    assert!(next.contains(question), "{next}");
    let view = fixture.ok(&["next", "--json"]);
    assert_eq!(view["open_questions"]["items"][0], question, "{view:#}");
    let status = fixture.ok(&["status"]);
    assert_eq!(status["checkpoint"]["open_questions"][0], question);
    assert_eq!(fixture.session_json("alpha")["open_questions"][0], question);
}

/// CH-19 (REQ-history-009): an inbox entry file of exactly 16,384 bytes is read; one byte
/// more is refused naming the bound.
#[test]
fn an_inbox_entry_of_exactly_16_kib_is_read_and_one_byte_more_is_refused() {
    for (size, readable) in [(16_384usize, true), (16_385, false)] {
        let fixture = Fixture::new();
        sessions(&fixture);
        let id = add_entry(&fixture, "alpha", "decision", "a note from a subagent");
        let path = format!(
            ".chough/sessions/{}/agent/inbox/{id}.md",
            fixture.id_of("alpha")
        );
        let text = fixture.read(&path);
        assert!(text.len() < size);
        fixture.write(&path, &format!("{text}{}", " ".repeat(size - text.len())));
        fixture.ok(&["session", "use", "alpha"]);
        let output = fixture.run(&["inbox", "list"]);
        assert_eq!(
            output.status.success(),
            readable,
            "{size}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        if !readable {
            assert!(
                String::from_utf8_lossy(&output.stderr).contains("16384"),
                "{size}"
            );
        }
    }
}

/// CH-19 (REQ-history-009): an inbox holds 200 unfolded entries; the 201st is refused,
/// naming the bound and the fold that recovers.
#[test]
fn an_inbox_holds_200_unfolded_entries_and_refuses_the_201st() {
    let fixture = Fixture::new();
    sessions(&fixture);
    for index in 0..200 {
        add_entry(&fixture, "alpha", "decision", &format!("note {index}"));
    }
    let refused = fixture.fails(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "subagent-1",
        "--kind",
        "decision",
        "--text",
        "one more",
    ]);
    assert!(refused.contains("200 unfolded entries"), "{refused}");
    assert!(refused.contains("fold"), "{refused}");
}

/// Issue #82, re-linked to REQ-cli-040: the writer lock makes `session use` wait for a
/// running `stage advance`, so a switch never happens under a command and the advance's
/// entry lands in the session it read.
#[cfg(unix)]
#[test]
fn session_use_waits_for_a_running_stage_advance() {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    sessions(&fixture);
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["checkpoint", "--native-change", "alpha-change"]);
    fixture.no_spec_approval("alpha-change");
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "skip", "--json", "--reason", "No visuals"]);
    fs::rename(
        fixture.root().join("bin/specsync"),
        fixture.root().join("bin/specsync-stub"),
    )
    .unwrap();
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\n[ -e .slow ] && touch running && sleep 2\nexec ./bin/specsync-stub \"$@\"\n",
    );
    fixture.executable("bin/specsync");
    let before = timeline(&fixture.read(&fixture.session_file_of("alpha"))).len();
    fixture.write(".slow", "");
    let spawn = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_chough"))
            .arg("--root")
            .arg(fixture.root())
            .args(args)
            .env(
                "PATH",
                std::env::join_paths(std::iter::once(fixture.root().join("bin")).chain(
                    std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default()),
                ))
                .unwrap(),
            )
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };
    let mut advance = spawn(&["stage", "advance", "--json"]);
    let started = Instant::now();
    while !fixture.exists("running") {
        assert!(
            started.elapsed() < Duration::from_secs(30),
            "the native read never started"
        );
        std::thread::sleep(Duration::from_millis(20));
    }
    let switch = spawn(&["session", "use", "beta"])
        .wait_with_output()
        .unwrap();
    assert!(
        advance.try_wait().unwrap().is_some(),
        "session use finished while stage advance still held the checkpoint"
    );
    assert!(
        switch.status.success(),
        "{}",
        String::from_utf8_lossy(&switch.stderr)
    );
    let advanced = advance.wait_with_output().unwrap();
    assert!(
        advanced.status.success(),
        "{}",
        String::from_utf8_lossy(&advanced.stderr)
    );
    let alpha = fixture.read(&fixture.session_file_of("alpha"));
    assert_eq!(parse_session(&alpha)["stage"]["current"], "build");
    assert_eq!(
        timeline(&alpha).len(),
        before + 1,
        "exactly the advance's entry"
    );
}

/// REQ-history-006, REQ-records-012: an uncommitted file in `agent/` keeps the session
/// folder after the move into done, which reports it; `next` does not count the folder
/// as open; `chough doctor` names the commit to make, and after it `doctor --fix`
/// removes the folder.
#[test]
fn an_uncommitted_file_keeps_the_folder_until_a_commit_and_doctor_fix() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    advance_to(&fixture, "accept");
    let id = fixture.id_of("alpha");
    fixture.commit("alpha at accept");
    fixture.write(
        &format!(".chough/sessions/{id}/agent/scratch.md"),
        "draft\n",
    );
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    let kept = moved["applied"]["session_archive"]["kept"].to_string();
    assert!(kept.contains("agent/scratch.md"), "{moved:#}");
    assert!(fixture.exists(&format!(".chough/sessions/{id}/agent/scratch.md")));
    // REQ-cli-021 (#371): `next` names finishing the archive while the folder holds files.
    let next = fixture.ok_text(&["next"]);
    assert!(
        next.contains(&format!("Finish the archive of session {id}")),
        "{next}"
    );
    let blocked = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&blocked.stdout).unwrap();
    assert_eq!(
        report["findings"][0]["kind"], "unfinished_archive",
        "{report:#}"
    );
    assert_eq!(report["findings"][0]["status"], "skipped", "{report:#}");
    assert!(fixture.exists(&format!(".chough/sessions/{id}")));
    fixture.commit("the story and the leftovers");
    let fixed = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&fixed.stdout).unwrap();
    assert_eq!(report["findings"][0]["status"], "fixed", "{report:#}");
    assert!(!fixture.exists(&format!(".chough/sessions/{id}")));
    assert!(fixture.is_archived("alpha"));
    let next = fixture.ok_text(&["next"]);
    assert!(next.contains("chough session new <key>"), "{next}");
}

/// REQ-records-006: a value written into the session record that holds a local path is
/// refused as `refused:leak`, naming the field and the kind and never the text, with the
/// file unchanged; with `--redact` it is stored as `<redacted:<kind>>`.
#[test]
fn a_leak_in_a_session_value_is_refused_or_redacted() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["stage", "advance", "--json"]);
    let file = fixture.session_file();
    let before = fixture.read(&file);
    let refused = fixture.fails(&["checkpoint", "--decision", "Built in /Users/someone/app"]);
    assert!(
        refused.contains("refused:leak: decision holds home-path"),
        "{refused}"
    );
    assert!(!refused.contains("/Users/someone"), "{refused}");
    assert_eq!(fixture.read(&file), before);
    let refused = fixture.fails(&[
        "stage",
        "back",
        "--to",
        "intent",
        "--reason",
        concat!("token ", "ghp_", "abcdefghijklmnopqrstuvwxyz0123456789"),
    ]);
    assert!(refused.contains("refused:leak"), "{refused}");
    assert_eq!(fixture.read(&file), before);
    let output = fixture.run(&[
        "checkpoint",
        "--redact",
        "--decision",
        "Built in /Users/someone/app",
    ]);
    assert!(output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("redacted 1 value"));
    let text = fixture.read(&file);
    assert!(text.contains("- Built in <redacted:home-path>"), "{text}");
    assert!(!text.contains("/Users/someone"), "{text}");
}

/// The front matter keys of a Markdown record, in order.
fn front_keys(text: &str) -> Vec<String> {
    let front = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map(|(front, _)| front)
        .expect("front matter");
    front
        .lines()
        .filter(|line| !line.starts_with(' ') && !line.starts_with('-'))
        .filter_map(|line| line.split_once(':').map(|(key, _)| key.to_string()))
        .collect()
}

/// TC-history-inbox-in-agent-folder (REQ-history-009). An entry is one Markdown record
/// under its session's `agent/inbox/`, giving `kind`, `value`, `from` and, for an
/// observation, `recorded`, with no approval, review, consent, gate or stage field, and
/// adding it changes no session file. Then each acceptance criterion, by the test that
/// proves it alone:
/// - folding twice adds each entry once and never changes a session file;
/// - an add racing an archive under the writer lock refuses with nothing written;
/// - an add killed while writing leaves no partial entry, and the next save removes its
///   temporary file;
/// - an unrecorded observation is added with a warning and listed `recorded: false`.
#[cfg(unix)]
#[test]
fn inbox_entries_live_in_the_agent_folder() {
    let fixture = Fixture::new();
    sessions(&fixture);
    let alpha = fixture.id_of("alpha");
    let before = snapshot(&fixture);
    let decision = add_entry(&fixture, "alpha", "decision", "keep the cache");
    let observation = fixture.run(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "subagent-1",
        "--kind",
        "observation",
        "--text",
        "OBS-never-recorded",
    ]);
    assert!(observation.status.success());
    let observation: Value = serde_json::from_slice(&observation.stdout).unwrap();
    let observation = observation["id"].as_str().unwrap().to_string();
    let inbox = format!(".chough/sessions/{alpha}/agent/inbox");
    for (id, kind, value, recorded) in [
        (&decision, "decision", "keep the cache", None),
        (
            &observation,
            "observation",
            "OBS-never-recorded",
            Some("false"),
        ),
    ] {
        let text = fixture.read(&format!("{inbox}/{id}.md"));
        let keys = front_keys(&text);
        for key in ["kind", "value", "from"] {
            assert!(keys.iter().any(|known| known == key), "{key}: {text}");
        }
        assert!(text.contains(&format!("\nkind: {kind}\n")), "{text}");
        assert!(text.contains(&format!("\nvalue: {value}\n")), "{text}");
        assert!(text.contains("\nfrom: subagent-1\n"), "{text}");
        match recorded {
            Some(recorded) => assert!(text.contains(&format!("\nrecorded: {recorded}\n"))),
            None => assert!(!keys.iter().any(|key| key == "recorded"), "{text}"),
        }
        for forbidden in ["approval", "review", "consent", "gate", "stage"] {
            assert!(
                !keys.iter().any(|key| key.contains(forbidden)),
                "{forbidden}: {text}"
            );
        }
    }
    // Adding changed no session file: only the two entries are new.
    let after = snapshot(&fixture);
    let added: Vec<&String> = after
        .keys()
        .filter(|path| !before.contains_key(*path))
        .collect();
    assert_eq!(added.len(), 2, "{added:?}");
    assert!(
        added
            .iter()
            .all(|path| path.starts_with(&format!("{inbox}/"))),
        "{added:?}"
    );
    for (path, bytes) in &before {
        assert_eq!(after.get(path), Some(bytes), "{path} changed");
    }

    inbox_entries_fold_once_without_touching_the_session();
    inbox_add_racing_an_archive_never_strands_its_entry();
    an_inbox_add_killed_while_writing_leaves_no_partial_entry();
    an_unrecorded_inbox_observation_is_added_with_a_warning();
}

/// TC-history-local-only-ignored (REQ-history-007). `git status` after `chough init`
/// shows no file under `.chough/local/`; after a cycle with a case run, an external
/// artifact and a gate list, every file under `.chough/local/` is ignored by Git, and
/// no committed record names one of them except by a neutral ID.
#[cfg(unix)]
#[test]
fn local_state_is_ignored_by_git() {
    use std::process::Command;
    let git = |fixture: &Fixture, args: &[&str]| -> String {
        let output = Command::new("git")
            .args(args)
            .current_dir(fixture.root())
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap()
    };
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".gitignore", ".stub-*\n");
    fixture.git(&["init"]);
    fixture.init();
    let status = git(
        &fixture,
        &["status", "--porcelain", "--untracked-files=all"],
    );
    assert!(!status.is_empty(), "init wrote nothing Git sees");
    assert!(!status.contains(".chough/local"), "{status}");

    // A case run (its raw log), an external artifact and a gate list.
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-local-state\"\ntitle = \"Local state\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"printed\"]\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo local-state-marker\"]\nexpect_output = [\"local-state-marker\"]\n",
    );
    fixture.commit("base");
    let run = fixture.run(&[
        "cases",
        "run",
        "TC-cli-local-state",
        "--environment",
        "local",
    ]);
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let run: Value = serde_json::from_slice(&run.stdout).unwrap();
    let case_entry = run["observation"].as_str().unwrap().to_string();
    let outside = tempfile::tempdir().unwrap();
    let external = outside.path().join("local-state-report.txt");
    fs::write(&external, "an external report\n").unwrap();
    let external = external.to_str().unwrap().to_string();
    let added = fixture.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "Read the external report",
        "--artifact",
        &external,
    ]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Copy review"]);
    fixture.ok(&["gate", "list", "--json"]);

    let mut local = vec![];
    fn walk(root: &std::path::Path, relative: &str, out: &mut Vec<String>) {
        for entry in fs::read_dir(root.join(relative)).unwrap() {
            let entry = entry.unwrap();
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else {
                out.push(name);
            }
        }
    }
    walk(fixture.root(), ".chough/local", &mut local);
    local.sort();
    for expected in [
        format!(".chough/local/logs/{case_entry}.log"),
        ".chough/local/external-artifacts.toml".to_string(),
    ] {
        assert!(local.contains(&expected), "{expected} missing: {local:?}");
    }
    assert!(
        local
            .iter()
            .any(|path| path.starts_with(".chough/local/gate-lists/")),
        "no gate list: {local:?}"
    );
    for path in &local {
        let ignored = Command::new("git")
            .args(["check-ignore", "-q", path])
            .current_dir(fixture.root())
            .status()
            .unwrap();
        assert!(ignored.success(), "{path} is not ignored by Git");
    }

    // Commit everything Git sees; no committed record names a local file, and the
    // external artifact is named only by its neutral ID.
    fixture.commit("cycle");
    let tracked = git(&fixture, &["ls-files", "-z"]);
    let tracked: Vec<&str> = tracked
        .split('\0')
        .filter(|path| !path.is_empty())
        .collect();
    assert!(
        !tracked
            .iter()
            .any(|path| path.starts_with(".chough/local/")),
        "{tracked:?}"
    );
    let mut names: Vec<String> = vec![external.clone(), "local-state-report.txt".to_string()];
    for path in &local {
        let inside = path.trim_start_matches(".chough/local/");
        if inside.contains('/') {
            names.push(path.clone());
            names.push(inside.to_string());
        }
    }
    for path in tracked.iter().filter(|path| path.starts_with(".chough/")) {
        let text = fs::read_to_string(fixture.root().join(path)).unwrap_or_default();
        for name in &names {
            assert!(!text.contains(name.as_str()), "{path} names {name}");
        }
    }
    let entry = fixture.read(added["recorded"].as_str().unwrap());
    let neutral = entry
        .lines()
        .find_map(|line| line.strip_prefix("external: "))
        .unwrap_or_else(|| panic!("no external ID: {entry}"));
    assert!(
        neutral.len() == 10
            && neutral.starts_with("ext-")
            && neutral[4..]
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()),
        "{neutral}"
    );
}

/// REQ-history-006, REQ-records-004, TC-history-a-story-lists-each-entry-once (R6, S7):
/// a closed session's story lists each evidence entry once in What was verified, as
/// its receipt line; an entry with no command still separates its claim from the
/// fingerprint digits with ` - `.
#[test]
fn a_story_lists_each_entry_once_with_its_receipt() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let outside = tempfile::tempdir().unwrap();
    let artifact = outside.path().join("SYNTHETIC-proof.bin");
    std::fs::write(&artifact, b"SYNTHETIC proof bytes\n").unwrap();
    let added = fixture.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "SYNTHETIC external proof",
        "--artifact",
        artifact.to_str().unwrap(),
    ]);
    let entry = added["observation"]
        .as_str()
        .unwrap_or_else(|| panic!("{added:#}"))
        .to_string();
    let key = fixture.active_id();
    fixture.commit("the entry");
    fixture.ok(&[
        "session",
        "close",
        &key,
        "--reason",
        "SYNTHETIC done testing",
    ]);
    let story = fixture.read(&format!(".chough/archive/sessions/{key}.md"));
    let start = story.find("\n## What was verified\n").unwrap();
    let section = &story[start + 1..];
    let section = &section[..section[3..]
        .find("\n## ")
        .map_or(section.len(), |at| at + 3)];
    let naming: Vec<&str> = section
        .lines()
        .filter(|line| line.starts_with("- ") && line.contains(&entry))
        .collect();
    assert_eq!(naming.len(), 1, "{section}");
    let line = naming[0];
    let after = line
        .strip_prefix("- SYNTHETIC external proof - ")
        .unwrap_or_else(|| panic!("no separator: {line}"));
    assert!(
        after.len() > 12 && after[..12].bytes().all(|byte| byte.is_ascii_hexdigit()),
        "{line}"
    );
}

/// REQ-records-009, REQ-history-006, TC-history-session-list-reads-stories-alone (U4):
/// with every session finished and no `.chough/sessions/` left, as a migrated 0.5.x
/// project whose sessions were all archived is, `session list` lists the stories.
#[test]
fn session_list_lists_archived_stories_when_no_session_folder_is_left() {
    let fixture = Fixture::new();
    fixture.init();
    let id = fixture.active_id();
    fixture.ok(&["session", "close", &id, "--reason", "SYNTHETIC finished"]);
    fixture.commit("closed");
    let folder = fixture.root().join(".chough/sessions");
    if folder.exists() {
        std::fs::remove_dir_all(&folder).unwrap();
    }
    let listed = fixture.ok(&["session", "list"]);
    assert!(listed.to_string().contains(&id), "{listed:#}");
}

/// A minimal valid PNG.
fn tiny_png() -> Vec<u8> {
    fn chunk(kind: &[u8], data: &[u8]) -> Vec<u8> {
        let mut out = (data.len() as u32).to_be_bytes().to_vec();
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        out.extend_from_slice(&[0, 0, 0, 0]);
        out
    }
    let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
    bytes.extend(chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0]));
    bytes.extend(chunk(b"IDAT", &[7u8; 16]));
    bytes.extend(chunk(b"IEND", &[]));
    bytes
}

/// REQ-records-012, TC-history-close-lists-each-kept-file-once (R7): closing a session
/// whose key screenshot is not committed yet keeps it in place and lists each kept file
/// once.
#[test]
fn closing_a_session_lists_each_kept_file_once() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let outside = tempfile::tempdir().unwrap();
    let shot = outside.path().join("SYNTHETIC-screen.png");
    std::fs::write(&shot, tiny_png()).unwrap();
    fixture.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "SYNTHETIC page looks right",
        "--screenshot",
        shot.to_str().unwrap(),
    ]);
    let id = fixture.active_id();
    let closed = fixture.ok(&[
        "session",
        "close",
        &id,
        "--reason",
        "SYNTHETIC done testing",
    ]);
    let kept: Vec<&str> = closed["kept"]
        .as_array()
        .unwrap_or_else(|| panic!("{closed:#}"))
        .iter()
        .filter_map(serde_json::Value::as_str)
        .collect();
    assert!(
        kept.iter().any(|file| file.ends_with(".png")),
        "the uncommitted screenshot was not kept: {closed:#}"
    );
    let mut once = kept.clone();
    once.sort_unstable();
    once.dedup();
    assert_eq!(once.len(), kept.len(), "{kept:#?}");
}

/// Fresh-eyes findings F10a and F10b (REQ-history-006): an archived story's front
/// matter carries no receipts, so its body starts within the first 20 lines; the story
/// names each bug of its checkpoint under `## Bugs` with its title and status, linked to
/// its record; it ends with the Timeline and its closing line, then `## Receipts`
/// holding every receipt as one fenced YAML block; and the bug those receipts verify and
/// the timing report still read them.
#[test]
fn a_story_puts_its_receipts_last_and_names_its_bugs() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".chough/cases/cli.cases.toml", ONE_CASE);
    fixture.init();
    fixture.commit("start");
    let observe = |summary: &str, outcome: &str| -> String {
        let added = fixture.ok(&[
            "evidence",
            "add",
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            "human-observation",
            "--outcome",
            outcome,
            "--environment",
            "local",
            "--summary",
            summary,
        ]);
        added["observation"]
            .as_str()
            .unwrap_or_else(|| panic!("{added:#}"))
            .to_string()
    };
    let failed = observe("SYNTHETIC the total is off by one", "failed");
    // The pass is observed in a later second than the failure.
    std::thread::sleep(std::time::Duration::from_millis(1_100));
    let passed = observe("SYNTHETIC the total is right", "passed");
    let ran = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "local"])["observation"]
        .as_str()
        .unwrap()
        .to_string();
    let added = fixture.ok(&[
        "bug",
        "add",
        "--title",
        "SYNTHETIC total is off by one",
        "--symptom",
        "The total is one short",
        "--trigger",
        "Counting a list",
        "--expected",
        "The full count",
        "--actual",
        "One short",
        "--environment",
        "local",
    ]);
    let bug = added["id"].as_str().unwrap().to_string();
    fixture.ok(&["bug", "update", &bug, "--observation", &failed]);
    fixture.ok(&[
        "bug",
        "update",
        &bug,
        "--observation",
        &passed,
        "--status",
        "verified",
    ]);
    let id = fixture.active_id();
    fixture.commit("the work");
    fixture.ok(&["session", "close", &id, "--reason", "SYNTHETIC finished"]);
    let story = fixture.read(&format!(".chough/archive/sessions/{id}.md"));

    // F10a: the story first, the receipts last.
    let front = front_lines(&story);
    assert!(
        !front.iter().any(|line| line.starts_with("receipts:")),
        "the front matter holds receipts: {front:#?}"
    );
    let heading = story
        .lines()
        .position(|line| line.starts_with("# "))
        .unwrap_or_else(|| panic!("no title: {story}"));
    assert!(
        heading < 20,
        "the body starts at line {}: {story}",
        heading + 1
    );
    let timeline = story.find("\n## Timeline\n").expect("a Timeline");
    let closed = story.find("\nClosed ").expect("a closing line");
    let receipts = story
        .find("\n## Receipts\n")
        .unwrap_or_else(|| panic!("no Receipts section: {story}"));
    assert!(timeline < closed && closed < receipts, "{story}");
    let tail = &story[receipts + "\n## Receipts\n".len()..];
    assert!(
        tail.starts_with("\n```yaml\n") && tail.trim_end().ends_with("```"),
        "{tail}"
    );
    assert!(!tail.contains("\n## "), "Receipts is not last: {tail}");
    for entry in [&failed, &passed, &ran] {
        assert!(
            tail.contains(&format!("\n  - id: {entry}\n")),
            "no receipt for {entry}: {tail}"
        );
    }

    // F10b: the bug the session found, linked, with its title and status.
    let start = story
        .find("\n## Bugs\n")
        .unwrap_or_else(|| panic!("no Bugs section: {story}"))
        + "\n## Bugs\n".len();
    let rest = &story[start..];
    let section = &rest[..rest.find("\n## ").unwrap_or(rest.len())];
    let link = format!("[SYNTHETIC total is off by one](../../bugs/{bug}.md)");
    assert!(
        section.lines().any(|line| line.starts_with("- ")
            && line.contains(&link)
            && line.contains("verified")),
        "{section}"
    );
    assert!(story.find("\n## Bugs\n").unwrap() < timeline, "{story}");

    // The readers follow: the bug resolves its evidence to the story's receipts and
    // stays verified, and timing reads the case run from the story.
    let shown = fixture.ok(&["bug", "show", &bug]);
    assert_eq!(
        shown["verification"], "reproduced-and-later-passed",
        "{shown:#}"
    );
    assert_eq!(
        shown["observation_labels"][0]["label"], "story-receipt",
        "{shown:#}"
    );
    let report = fixture.ok(&["timing", "--json"]);
    assert!(
        report.to_string().contains(&format!("receipt in {id}")),
        "{report:#}"
    );
}

/// 0.6.1 (t38n's fourth fresh-eyes round, F10; REQ-history-006): after the move into done
/// archived the session, the normal state until the next session opens, `chough next`
/// printed "Error: no open session" and exited 1. It exits 0, says there is no open
/// session and names `chough session new <key>`, and writes nothing, while `chough
/// status` refuses with the same message; the named command opens the next session.
#[test]
fn after_the_move_into_done_next_exits_0_and_status_refuses() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    advance_to(&fixture, "accept");
    fixture.commit("alpha at accept");
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        moved["applied"]["session_archive"]["status"], "archived",
        "{moved:#}"
    );
    let before = snapshot(&fixture);
    // REQ-cli-021 (#371): while the archived folder still holds files, `next` names
    // finishing the archive first, and writes nothing.
    let next = fixture.ok_text(&["next"]);
    assert!(next.contains("Finish the archive of session"), "{next}");
    assert_eq!(snapshot(&fixture), before, "next wrote a record");
    let refused = fixture.fails(&["status"]);
    assert!(
        refused.contains("no open session; run `chough session new <key>`"),
        "{refused}"
    );
    fixture.commit("the archive");
    let _ = fixture.run(&["doctor", "--fix"]);
    let before = snapshot(&fixture);
    let next = fixture.ok_text(&["next"]);
    assert!(next.contains("no open session"), "{next}");
    assert!(next.contains("Run: chough session new <key>"), "{next}");
    assert_eq!(snapshot(&fixture), before, "next wrote a record");
    fixture.ok(&["session", "new", "beta", "--objective", "Ship beta"]);
    let next = fixture.ok_text(&["next"]);
    assert!(next.starts_with("Objective: Ship beta\n"), "{next}");
}

/// An `evidence add` in the active session with `summary`, for the stubbed specs; its ID.
#[cfg(unix)]
fn evidence_entry(fixture: &Fixture, summary: &str) -> String {
    fixture.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        summary,
    ])["observation"]
        .as_str()
        .unwrap()
        .to_string()
}

/// Git in the fixture, with a committer, whatever its exit status.
#[cfg(unix)]
fn git(fixture: &Fixture, args: &[&str]) -> std::process::Output {
    std::process::Command::new("git")
        .current_dir(fixture.root())
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "-c",
            "commit.gpgsign=false",
        ])
        .args(args)
        .output()
        .unwrap()
}

/// REQ-history-012, REQ-history-006 (#381), TC-history-the-archive-finish-keeps-what-the-story-does-not-hold:
/// a branch adds an evidence entry and an inbox entry to a session that main closes and
/// archives; the merge keeps main's deletion of `session.md`. `doctor --fix` then removes
/// the folder's other files but keeps both entries, byte for byte, and its
/// `unfinished_archive` finding is `skipped`, naming each and the way out by hand.
#[cfg(unix)]
#[test]
fn the_archive_finish_keeps_what_the_story_does_not_hold() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("start");
    let id = fixture.active_id();
    let held = evidence_entry(&fixture, "SYNTHETIC check the story holds");
    fixture.commit("a session with one entry");
    assert!(
        git(&fixture, &["checkout", "-q", "-b", "late"])
            .status
            .success()
    );
    let late = evidence_entry(&fixture, "SYNTHETIC late check on the branch");
    let note = add_entry(&fixture, &id, "decision", "SYNTHETIC late decision");
    fixture.commit("late work on the branch");
    assert!(git(&fixture, &["checkout", "-q", "main"]).status.success());
    fixture.ok(&[
        "session",
        "close",
        &id,
        "--reason",
        "SYNTHETIC done elsewhere",
    ]);
    fixture.commit("the session closed");
    let _ = fixture.run(&["doctor", "--fix"]);
    fixture.commit("its archive finished");
    assert!(!fixture.exists(&format!(".chough/sessions/{id}")));
    let merged = git(&fixture, &["merge", "--no-edit", "late"]);
    assert!(!merged.status.success(), "the merge had no conflict");
    let session = format!(".chough/sessions/{id}/session.md");
    assert!(git(&fixture, &["rm", "-q", &session]).status.success());
    assert!(
        git(&fixture, &["commit", "-q", "--no-edit"])
            .status
            .success()
    );
    let entry = format!(".chough/sessions/{id}/evidence/{late}.md");
    let inbox = format!(".chough/sessions/{id}/agent/inbox/{note}.md");
    let entry_bytes = fixture.read(&entry);
    let inbox_bytes = fixture.read(&inbox);

    let output = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let finding = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["kind"] == "unfinished_archive")
        .unwrap_or_else(|| panic!("{report:#}"))
        .clone();
    assert_eq!(finding["status"], "skipped", "{report:#}");
    let said = finding.to_string();
    assert!(said.contains(&late), "{said}");
    assert!(said.contains(&note), "{said}");
    // With no session open, the way out says to open one, and dropping is printed whole.
    assert!(said.contains("chough session new <key>"), "{said}");
    assert!(said.contains("git rm -q -- "), "{said}");
    assert!(!said.contains(&held), "the held entry was named: {said}");
    assert_eq!(fixture.read(&entry), entry_bytes);
    assert_eq!(fixture.read(&inbox), inbox_bytes);
    // REQ-cli-021: while only kept entries hold the folder, `next` names the way out
    // by hand after `doctor --fix`, not `doctor --fix` alone.
    let next = fixture.ok_text(&["next"]);
    assert!(
        next.contains("move or drop by hand each entry it keeps"),
        "{next}"
    );

    // The way out by hand, run exactly as printed once a session is open: move the
    // evidence entry into it, add the inbox entry's text again there and drop the kept
    // copy. The archive then finishes and the entry resolves again.
    fixture.ok(&[
        "session",
        "new",
        "next",
        "--objective",
        "Pick up the late work",
    ]);
    let (finding, report, _) = archive_finding(&fixture, &["doctor"]);
    let fix = finding["fix"].as_str().unwrap_or_default().to_string();
    let commands = printed_commands(&fix);
    let keep = commands
        .iter()
        .find(|command| command.starts_with("mkdir -p "))
        .unwrap_or_else(|| panic!("no keep command: {report:#}"));
    let add = commands
        .iter()
        .find(|command| command.starts_with("chough inbox add "))
        .unwrap_or_else(|| panic!("no inbox command: {report:#}"));
    for command in [keep, add] {
        let output = run_printed(&fixture, command);
        assert!(
            output.status.success(),
            "{command}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let output = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|finding| finding["kind"] != "unfinished_archive" || finding["status"] == "fixed"),
        "{report:#}"
    );
    assert!(!fixture.exists(&format!(".chough/sessions/{id}")));
    fixture.ok(&["checkpoint", "--observation", &late]);
}

/// REQ-history-012 (#381): an evidence entry whose ID a receipt of the story holds, but
/// whose receipt it no longer matches, is kept too; an exact copy of what the story holds
/// is removed.
#[cfg(unix)]
#[test]
fn an_entry_that_differs_from_its_receipt_is_kept() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("start");
    let id = fixture.active_id();
    let first = evidence_entry(&fixture, "SYNTHETIC first check");
    let second = evidence_entry(&fixture, "SYNTHETIC second check");
    fixture.commit("two entries");
    let path = |entry: &str| format!(".chough/sessions/{id}/evidence/{entry}.md");
    let (first_bytes, second_bytes) = (fixture.read(&path(&first)), fixture.read(&path(&second)));
    fixture.ok(&["session", "close", &id, "--reason", "SYNTHETIC done"]);
    fixture.commit("closed");
    let _ = fixture.run(&["doctor", "--fix"]);
    fixture.commit("archive finished");
    assert!(!fixture.exists(&format!(".chough/sessions/{id}")));
    // A later commit brings both back: one as the story holds it, one changed.
    fixture.write(&path(&first), &first_bytes);
    fixture.write(
        &path(&second),
        &second_bytes.replace("SYNTHETIC second check", "SYNTHETIC second check, changed"),
    );
    fixture.commit("the entries came back");
    let output = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let said = report.to_string();
    assert!(!fixture.exists(&path(&first)), "{said}");
    assert!(fixture.exists(&path(&second)), "{said}");
    assert!(said.contains(&second), "{said}");
}

/// REQ-history-012 (#377), TC-history-macos-metadata-never-blocks-the-archive-finish: a
/// `.DS_Store` that Finder writes and a `._session.md` that an exFAT volume leaves in an
/// archived session's folder are not committed, yet they do not block `doctor --fix`,
/// which removes them with the folder.
#[test]
fn macos_metadata_never_blocks_the_archive_finish() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    advance_to(&fixture, "accept");
    let id = fixture.id_of("alpha");
    fixture.commit("alpha at accept");
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.commit("the story");
    fixture.write(&format!(".chough/sessions/{id}/.DS_Store"), "finder\n");
    fixture.write(
        &format!(".chough/sessions/{id}/._session.md"),
        "appledouble\n",
    );
    let fixed = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&fixed.stdout).unwrap();
    let archive = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["kind"] == "unfinished_archive")
        .unwrap_or_else(|| panic!("{report:#}"))
        .clone();
    assert_eq!(archive["status"], "fixed", "{report:#}");
    assert!(
        !fixture.exists(&format!(".chough/sessions/{id}")),
        "{report:#}"
    );
}

/// An archived session whose evidence entry was not committed at the move into done, so
/// the folder keeps it, with everything committed after; returns the session's ID and the
/// entry's file.
#[cfg(unix)]
fn archived_with_a_kept_entry(fixture: &Fixture) -> (String, String) {
    fixture.stubs();
    fixture.init();
    fixture.commit("start");
    let id = fixture.active_id();
    advance_to(fixture, "accept");
    fixture.commit("at accept");
    let entry = evidence_entry(fixture, "SYNTHETIC check at accept");
    fixture.ok(&["stage", "advance", "--json"]);
    let file = format!(".chough/sessions/{id}/evidence/{entry}.md");
    assert!(fixture.exists(&file), "the move into done removed {file}");
    (id, file)
}

/// The `unfinished_archive` finding a `doctor` run reports, with its exit status, stdout
/// and stderr.
#[cfg(unix)]
fn archive_finding(fixture: &Fixture, args: &[&str]) -> (Value, Value, String) {
    let output = fixture.run(args);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout)
        .unwrap_or_else(|error| panic!("{args:?} gave no report ({error}): {said}"));
    let finding = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["kind"] == "unfinished_archive")
        .unwrap_or_else(|| panic!("{report:#}"))
        .clone();
    (finding, report, said)
}

/// REQ-history-012 (#381, fable-lead's review of D1), TC-history-a-story-an-earlier-release-wrote-still-holds-its-entries:
/// a story that 0.6.0 or 0.6.1 wrote has receipts without the `criteria`, `requirements`
/// and `limitations` fields #342 added. An entry it holds is still held, compared on the
/// fields its receipt holds, so `doctor --fix` finishes the archive.
#[cfg(unix)]
#[test]
fn a_story_an_earlier_release_wrote_still_holds_its_entries() {
    let fixture = Fixture::new();
    let (id, file) = archived_with_a_kept_entry(&fixture);
    let story = format!(".chough/archive/sessions/{id}.md");
    let text = fixture.read(&story);
    let (told, receipts) = text.split_once("\n## Receipts\n").unwrap();
    let older: String = receipts
        .lines()
        .filter(|line| {
            let field = line.trim_start();
            !["criteria: ", "requirements: ", "limitations: "]
                .iter()
                .any(|name| field.starts_with(name))
        })
        .map(|line| format!("{line}\n"))
        .collect();
    assert_ne!(
        older, receipts,
        "the receipt held no criteria or requirements"
    );
    fixture.write(&story, &format!("{told}\n## Receipts\n{older}"));
    fixture.commit("archived by an earlier release");
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "fixed", "{report:#}");
    assert!(!fixture.exists(&file));
    assert!(!fixture.exists(&format!(".chough/sessions/{id}")));
}

/// REQ-history-012 (#381, fable-lead's review of D1, tester2), TC-history-an-unreadable-file-never-stops-doctor:
/// a 0-byte evidence entry, a cut-off one and a story over its bound in an archived
/// session's folder each leave `doctor` and `doctor --fix` reporting, as on main, with the
/// file's own finding there and no absolute path in any output: an entry that cannot be
/// read is kept and named, with dropping it as the way out, and a story that cannot be read
/// leaves `unfinished_archive` removing nothing and saying why.
#[cfg(unix)]
#[test]
fn an_unreadable_file_in_an_archived_folder_never_stops_doctor() {
    for case in ["empty entry", "cut-off entry", "oversize story"] {
        let fixture = Fixture::new();
        let (id, file) = archived_with_a_kept_entry(&fixture);
        let story = format!(".chough/archive/sessions/{id}.md");
        let damaged = match case {
            "empty entry" => {
                fixture.write(&file, "");
                file.clone()
            }
            "cut-off entry" => {
                let text = fixture.read(&file);
                fixture.write(&file, &text[..text.len() / 2]);
                file.clone()
            }
            _ => {
                let text = fixture.read(&story);
                fixture.write(&story, &format!("{text}\n{}\n", "x".repeat(1_100_000)));
                story.clone()
            }
        };
        fixture.commit(case);
        let root = fixture.root().to_string_lossy().into_owned();
        let canonical = fixture
            .root()
            .canonicalize()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        for args in [&["doctor"][..], &["doctor", "--fix"][..]] {
            let (finding, report, said) = archive_finding(&fixture, args);
            assert!(
                !said.contains(&root) && !said.contains(&canonical),
                "{case}, {args:?}: an absolute path: {said}"
            );
            let named =
                report["findings"].as_array().unwrap().iter().any(|other| {
                    other["kind"] != "unfinished_archive" && other["target"] == damaged
                });
            assert!(
                named,
                "{case}, {args:?}: no finding names {damaged}: {report:#}"
            );
            if args.len() > 1 {
                assert_eq!(finding["status"], "skipped", "{case}: {report:#}");
                let said = finding.to_string();
                if case == "oversize story" {
                    assert!(
                        said.contains("removes nothing until it reads"),
                        "{case}: {finding:#}"
                    );
                } else {
                    // An entry that cannot be read is kept and named, and only dropping it
                    // is advised, since moving it would make an open session unreadable.
                    assert!(
                        finding["kept"].to_string().contains(&file),
                        "{case}: {finding:#}"
                    );
                    assert!(
                        said.contains("cannot be read") && said.contains("git rm"),
                        "{case}: {finding:#}"
                    );
                    assert!(!said.contains("git mv"), "{case}: {finding:#}");
                }
            }
            assert!(fixture.exists(&file), "{case}, {args:?}: the entry went");
        }
    }
}

/// The archived session `alpha` with its folder's files committed by name, and its story
/// and journal post left as the move into done wrote them; returns the session's ID, its
/// story and its post.
#[cfg(unix)]
fn archived_with_the_folder_committed(fixture: &Fixture) -> (String, String, String) {
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    advance_to(fixture, "accept");
    let id = fixture.id_of("alpha");
    fixture.commit("alpha at accept");
    fixture.ok(&["stage", "advance", "--json"]);
    let folder = format!(".chough/sessions/{id}");
    assert!(
        fixture.exists(&folder),
        "the move into done removed the folder"
    );
    assert!(git(fixture, &["add", "--", &folder]).status.success());
    assert!(
        git(fixture, &["commit", "-q", "-m", "only the folder"])
            .status
            .success()
    );
    let story = format!(".chough/archive/sessions/{id}.md");
    let post = std::fs::read_dir(fixture.root().join("knowledge/journal"))
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .find(|name| name.ends_with(".md") && name != "index.md")
        .map(|name| format!("knowledge/journal/{name}"))
        .expect("the move into done wrote no journal post");
    (id, story, post)
}

/// Whether Git tracks `path` in the fixture.
#[cfg(unix)]
fn tracked(fixture: &Fixture, path: &str) -> bool {
    git(fixture, &["ls-files", "--error-unmatch", "--", path])
        .status
        .success()
}

/// REQ-history-012 (#389, tester2's run of D1 on real repositories),
/// TC-history-the-finish-waits-for-the-story-to-be-committed: with the folder's files
/// committed but the story and its journal post untracked, `doctor --fix` removes nothing
/// and its `unfinished_archive` finding names the story and the post to commit; once they
/// are committed, it removes the folder, and the story stays tracked.
#[cfg(unix)]
#[test]
fn the_finish_waits_until_the_story_and_post_are_committed() {
    let fixture = Fixture::new();
    let (id, story, post) = archived_with_the_folder_committed(&fixture);
    assert!(!tracked(&fixture, &story) && !tracked(&fixture, &post));
    let folder = format!(".chough/sessions/{id}");
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "skipped", "{report:#}");
    let said = finding.to_string();
    assert!(said.contains(&story) && said.contains(&post), "{finding:#}");
    assert!(
        fixture.exists(&folder),
        "the folder went before the story was committed"
    );
    assert!(fixture.exists(&story));

    assert!(
        git(&fixture, &["add", "--", &story, &post])
            .status
            .success()
    );
    assert!(
        git(&fixture, &["commit", "-q", "-m", "the story and its post"])
            .status
            .success()
    );
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "fixed", "{report:#}");
    assert!(!fixture.exists(&folder));
    assert!(tracked(&fixture, &story));
}

/// REQ-history-012 (#389): an archived story edited after its commit, as `chough correct`
/// appends to its Timeline, holds the finish back until that edit is committed, and the
/// finding names the story.
#[cfg(unix)]
#[test]
fn a_story_edited_after_its_commit_holds_the_finish() {
    let fixture = Fixture::new();
    let (id, story, post) = archived_with_the_folder_committed(&fixture);
    assert!(
        git(&fixture, &["add", "--", &story, &post])
            .status
            .success()
    );
    assert!(
        git(&fixture, &["commit", "-q", "-m", "the story and its post"])
            .status
            .success()
    );
    let text = fixture.read(&story);
    let entry = text
        .lines()
        .filter_map(|line| line.rsplit_once(" ^").map(|(_, id)| id.trim().to_string()))
        .next_back()
        .expect("the story has no Timeline entry");
    fixture.ok(&[
        "correct",
        &format!("{id}#^{entry}"),
        "--reason",
        "SYNTHETIC correction after the commit",
    ]);
    assert_ne!(fixture.read(&story), text);
    let folder = format!(".chough/sessions/{id}");
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "skipped", "{report:#}");
    assert!(finding.to_string().contains(&story), "{finding:#}");
    assert!(fixture.exists(&folder));

    assert!(git(&fixture, &["add", "--", &story]).status.success());
    assert!(
        git(&fixture, &["commit", "-q", "-m", "the correction"])
            .status
            .success()
    );
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "fixed", "{report:#}");
    assert!(!fixture.exists(&folder));
}

/// REQ-history-012 (#389, fable-lead's read of D1): a story Git ignores is never listed
/// by `git status`, so the finish asks Git whether it is committed. With the story
/// ignored, `doctor --fix` removes nothing, and its finding names a way out that works,
/// `git add -f`; once it is committed, the folder goes.
#[cfg(unix)]
#[test]
fn an_ignored_story_holds_the_finish_and_names_git_add_f() {
    let fixture = Fixture::new();
    let (id, story, post) = archived_with_the_folder_committed(&fixture);
    fixture.write(".gitignore", &format!("{story}\n"));
    assert!(
        git(&fixture, &["add", "--", ".gitignore", &post])
            .status
            .success()
    );
    assert!(
        git(&fixture, &["commit", "-q", "-m", "ignore the story"])
            .status
            .success()
    );
    assert!(!tracked(&fixture, &story));
    let folder = format!(".chough/sessions/{id}");
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "skipped", "{report:#}");
    let fix = finding["fix"].as_str().unwrap_or_default().to_string();
    assert!(
        fix.contains(&format!("git add -f -- '{story}'")),
        "{finding:#}"
    );
    assert!(
        fixture.exists(&folder),
        "the folder went while the story was ignored"
    );

    // The way out, run exactly as printed.
    let command = printed_commands(&fix)
        .into_iter()
        .find(|command| command.starts_with("git add "))
        .unwrap_or_else(|| panic!("{finding:#}"));
    let output = run_printed(&fixture, &command);
    assert!(
        output.status.success(),
        "{command}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "fixed", "{report:#}");
    assert!(!fixture.exists(&folder));
}

/// REQ-history-012 (#389): the session's journal post is found by its `record` field,
/// not only among the paths `git status` lists, so an ignored post holds the finish too.
#[cfg(unix)]
#[test]
fn an_ignored_journal_post_holds_the_finish() {
    let fixture = Fixture::new();
    let (id, story, post) = archived_with_the_folder_committed(&fixture);
    fixture.write(".gitignore", "knowledge/journal/\n");
    assert!(
        git(&fixture, &["add", "--", ".gitignore", &story])
            .status
            .success()
    );
    assert!(
        git(&fixture, &["commit", "-q", "-m", "ignore the journal"])
            .status
            .success()
    );
    assert!(!tracked(&fixture, &post));
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "skipped", "{report:#}");
    let fix = finding["fix"].as_str().unwrap_or_default();
    assert!(
        fix.contains(&format!("git add -f -- '{post}'")),
        "{finding:#}"
    );
    assert!(fixture.exists(&format!(".chough/sessions/{id}")));
}

/// REQ-history-012 (#389): when Git cannot read HEAD, the finish removes nothing and its
/// finding names that, not files that only seem uncommitted.
#[cfg(unix)]
#[test]
fn a_failed_git_read_holds_the_finish_and_says_so() {
    let fixture = Fixture::new();
    let (id, story, post) = archived_with_the_folder_committed(&fixture);
    assert!(
        git(&fixture, &["add", "--", &story, &post])
            .status
            .success()
    );
    assert!(
        git(&fixture, &["commit", "-q", "-m", "the story and its post"])
            .status
            .success()
    );
    let output = fixture.run_with_env(&["doctor", "--fix"], &[("GIT_DIR", "/nonexistent-git-dir")]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "no report ({error}): {}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    let finding = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|finding| finding["kind"] == "unfinished_archive")
        .unwrap_or_else(|| panic!("{report:#}"))
        .clone();
    assert_eq!(finding["status"], "skipped", "{report:#}");
    let reason = finding["reason"].as_str().unwrap_or_default();
    assert!(reason.contains("Git cannot read HEAD"), "{finding:#}");
    assert!(fixture.exists(&format!(".chough/sessions/{id}")));
}

/// The whole commands a way out prints, each between backticks.
fn printed_commands(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| {
            ["git ", "mkdir ", "chough inbox add "]
                .iter()
                .any(|start| span.starts_with(start))
        })
        .map(str::to_string)
        .collect()
}

/// Run `command` exactly as a person pastes it, in the fixture's root, with this build's
/// `chough` on PATH, the fixture's user configuration and a committer.
#[cfg(unix)]
fn run_printed(fixture: &Fixture, command: &str) -> std::process::Output {
    let mut paths = vec![
        std::path::Path::new(env!("CARGO_BIN_EXE_chough"))
            .parent()
            .unwrap()
            .to_path_buf(),
    ];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let mut shell = std::process::Command::new("sh");
    shell
        .current_dir(fixture.root())
        .env("PATH", std::env::join_paths(paths).unwrap())
        .env("XDG_CONFIG_HOME", fixture.root().join(common::USER_CONFIG))
        .env("GIT_AUTHOR_NAME", "Test")
        .env("GIT_AUTHOR_EMAIL", "test@example.invalid")
        .env("GIT_COMMITTER_NAME", "Test")
        .env("GIT_COMMITTER_EMAIL", "test@example.invalid")
        .env("GIT_CONFIG_COUNT", "1")
        .env("GIT_CONFIG_KEY_0", "commit.gpgsign")
        .env("GIT_CONFIG_VALUE_0", "false");
    for name in common::ACTOR_VARIABLES {
        shell.env_remove(name);
    }
    shell.args(["-c", command]).output().unwrap()
}

/// REQ-history-012 (#381, tester2's planted cases, rule 3): with an evidence entry that
/// cannot be read, the first `doctor --fix` removes the folder's committed files and keeps
/// the entry. `next` then names committing the removal, and its command, run as printed,
/// commits it; the finding's drop command, run as printed, drops the entry; the next fix
/// finishes the archive.
#[cfg(unix)]
#[test]
fn an_unreadable_entry_is_dropped_by_the_printed_commands() {
    let fixture = Fixture::new();
    let (id, file) = archived_with_a_kept_entry(&fixture);
    fixture.write(&file, "");
    fixture.commit("an entry a killed run left empty");
    let (finding, report, _) = archive_finding(&fixture, &["doctor", "--fix"]);
    assert_eq!(finding["status"], "skipped", "{report:#}");
    assert!(fixture.exists(&file));
    assert!(!fixture.exists(&format!(".chough/sessions/{id}/session.md")));

    // What `next` prints now: commit the removal, then run the fix again.
    let view = fixture.ok(&["next", "--json"]);
    let command = view["next_valid_action"]["command"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    assert!(
        command.starts_with("git add -- ") && command.contains("session.md"),
        "{view:#}"
    );
    let output = run_printed(&fixture, &command);
    let said = String::from_utf8_lossy(&output.stdout);
    assert!(said.contains("unfinished_archive"), "{command}: {said}");

    // The finding's drop command, as printed.
    let (finding, report, _) = archive_finding(&fixture, &["doctor"]);
    let fix = finding["fix"].as_str().unwrap_or_default().to_string();
    let drop = printed_commands(&fix)
        .into_iter()
        .find(|command| command.starts_with("git rm "))
        .unwrap_or_else(|| panic!("no drop command: {report:#}"));
    assert!(!fix.contains("git mv"), "{fix}");
    let output = run_printed(&fixture, &drop);
    assert!(
        output.status.success(),
        "{drop}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = fixture.run(&["doctor", "--fix"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .all(|finding| finding["kind"] != "unfinished_archive" || finding["status"] == "fixed"),
        "{report:#}"
    );
    assert!(!fixture.exists(&format!(".chough/sessions/{id}")));
}
