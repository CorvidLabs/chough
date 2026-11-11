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
        fixture.ok(&["stage", "advance"]);
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
    fixture.ok(&["stage", "advance"]);
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
    assert!(timeline(&before).is_empty());
    let entries = timeline(&beta_text);
    assert_eq!(entries.len(), 1, "{beta_text}");
    assert!(
        entries[0].contains(" · advance · intent → design · "),
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
    let moved = fixture.ok(&["stage", "advance"]);
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
    assert!(story.trim_end().ends_with(": done."), "{story}");
    // session.md changed with the move, so it stays until it is committed; the
    // committed agent note goes.
    assert_eq!(
        archive["kept"],
        json!([format!(".chough/sessions/{id}/session.md")])
    );
    assert!(!fixture.exists(&format!(".chough/sessions/{id}/agent/notes.md")));
    assert!(!fixture.exists(".chough/local/active-session"));
    let refused = fixture.fails(&["next"]);
    assert!(refused.contains("chough session new <key>"), "{refused}");
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
    let moved = fixture.ok(&["stage", "advance"]);
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
    let moved = fixture.ok(&["stage", "advance"]);
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
    let killed = std::process::Command::new("sh")
        .args(["-c", "ulimit -f 0; exec \"$@\"", "sh"])
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
            "Killed while writing",
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
    let beta = fixture.id_of("beta");
    let inbox = fixture
        .root()
        .join(format!(".chough/sessions/{beta}/agent/inbox"));
    let temps: Vec<String> = fs::read_dir(&inbox)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert_eq!(temps.len(), 1, "{temps:?}");
    fixture.commit("after the killed add");
    let temp = format!(".chough/sessions/{beta}/agent/inbox/{}", temps[0]);
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
    fixture.write(".chough/evidence/OBS-recorded.json", "{}\n");
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
        "OBS-recorded",
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
    assert_eq!(recorded("OBS-recorded"), true, "{list:#}");
}

/// DRIVE-14: an unreadable session is listed with `chough session shelve`, `next` on
/// another session and `session new` still work, and shelving moves the folder byte for
/// byte under `.chough/archive/shelved/`.
#[test]
fn an_unreadable_session_is_listed_and_shelving_moves_its_folder_byte_for_byte() {
    let fixture = Fixture::new();
    sessions(&fixture);
    let alpha = fixture.id_of("alpha");
    let file = format!(".chough/sessions/{alpha}/session.md");
    fixture.write(&file, "not a record\n");
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
    assert!(!fixture.exists(&format!(".chough/sessions/{alpha}")));
}

/// REQ-history-007: `.chough/local/` holds only the pointer and the writer lock here,
/// and Git ignores all of it.
#[test]
fn local_holds_only_the_pointer_and_the_lock() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&["stage", "advance"]);
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
    fixture.ok(&["stage", "advance"]);
    fixture.commit("alpha moves");
    fixture.git(&["checkout", "-q", "-"]);
    fixture.ok(&["session", "use", "beta"]);
    fixture.ok(&["stage", "advance"]);
    fixture.commit("beta moves");
    fixture.git(&["merge", "-q", "--no-edit", "work-alpha"]);
    assert_eq!(
        timeline(&fixture.read(&fixture.session_file_of("alpha"))).len(),
        1
    );
    assert_eq!(
        timeline(&fixture.read(&fixture.session_file_of("beta"))).len(),
        1
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
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
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
    let mut advance = spawn(&["stage", "advance"]);
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
    let moved = fixture.ok(&["stage", "advance"]);
    let kept = moved["applied"]["session_archive"]["kept"].to_string();
    assert!(kept.contains("agent/scratch.md"), "{moved:#}");
    assert!(fixture.exists(&format!(".chough/sessions/{id}/agent/scratch.md")));
    assert!(
        fixture
            .fails(&["next"])
            .contains("chough session new <key>")
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
}

/// REQ-records-006: a value written into the session record that holds a local path is
/// refused as `refused:leak`, naming the field and the kind and never the text, with the
/// file unchanged; with `--redact` it is stored as `<redacted:<kind>>`.
#[test]
fn a_leak_in_a_session_value_is_refused_or_redacted() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["stage", "advance"]);
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
        "token ghp_abcdefghijklmnopqrstuvwxyz0123456789",
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
