//! Sessions per objective or native change (REQ-cli-037, REQ-history-006, DRIVE-3).

mod common;

use common::Fixture;
use serde_json::Value;
use std::fs;

fn json(fixture: &Fixture, name: &str) -> Value {
    serde_json::from_str(&fixture.read(name)).unwrap()
}

fn sessions(fixture: &Fixture) {
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["session", "new", "beta", "--objective", "Ship beta"]);
}

fn cycle(fixture: &Fixture, key: &str) -> String {
    json(fixture, &format!(".chough/sessions/{key}.json"))["history_cursor"]["cycle"]
        .as_str()
        .unwrap()
        .to_string()
}

fn events(fixture: &Fixture, cycle: &str) -> usize {
    let dir = fixture
        .root()
        .join(format!(".chough/history/cycles/{cycle}"));
    match fs::read_dir(dir) {
        Ok(entries) => entries.count(),
        Err(_) => 0,
    }
}

#[test]
fn each_session_has_its_own_cycle() {
    let fixture = Fixture::new();
    sessions(&fixture);
    assert!(!fixture.exists(".chough/session.json"));
    assert_eq!(fixture.read(".chough/local/active-session").trim(), "beta");
    assert_eq!(fixture.read(".chough/local/.gitignore"), "*\n");
    let (alpha, beta) = (cycle(&fixture, "alpha"), cycle(&fixture, "beta"));
    assert_ne!(alpha, beta);
    let before = fixture.read(".chough/sessions/alpha.json");
    fixture.ok(&["stage", "advance"]);
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), before);
    let beta_session = json(&fixture, ".chough/sessions/beta.json");
    assert_eq!(beta_session["objective"], "Ship beta");
    assert_ne!(beta_session["stage"]["current"], "intent");
    assert_eq!(events(&fixture, &alpha), 0);
    assert!(events(&fixture, &beta) > 0);
    let refused = fixture.fails(&["session", "new", "beta"]);
    assert!(refused.contains("already exists"), "{refused}");
    assert!(
        fixture
            .fails(&["init", "--objective", "x"])
            .contains("sessions mode")
    );
    let list = fixture.ok(&["session", "list"]);
    let keys: Vec<_> = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|session| {
            (
                session["key"].as_str().unwrap().to_string(),
                session["active"].as_bool().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        keys,
        [("alpha".to_string(), false), ("beta".to_string(), true)]
    );
}

#[test]
fn commands_use_the_active_session_and_refuse_when_unclear() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha"]);
    fs::remove_file(fixture.root().join(".chough/local/active-session")).unwrap();
    // The only open session is active without a pointer.
    fixture.ok(&["checkpoint", "--decision", "alpha decision"]);
    assert_eq!(
        json(&fixture, ".chough/sessions/alpha.json")["decisions"][0],
        "alpha decision"
    );
    fixture.ok(&["session", "new", "beta"]);
    fs::remove_file(fixture.root().join(".chough/local/active-session")).unwrap();
    let before = (
        fixture.read(".chough/sessions/alpha.json"),
        fixture.read(".chough/sessions/beta.json"),
    );
    let refused = fixture.fails(&["checkpoint", "--decision", "lost"]);
    assert!(refused.contains("chough session use"), "{refused}");
    fixture.write(".chough/local/active-session", "gamma\n");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("gamma"), "{refused}");
    assert_eq!(
        before,
        (
            fixture.read(".chough/sessions/alpha.json"),
            fixture.read(".chough/sessions/beta.json"),
        )
    );
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["checkpoint", "--decision", "second"]);
    assert_eq!(
        json(&fixture, ".chough/sessions/alpha.json")["decisions"][1],
        "second"
    );
    assert!(
        fixture
            .fails(&["session", "use", "gamma"])
            .contains("gamma")
    );
}

#[test]
fn adopt_moves_the_checkpoint_byte_for_byte() {
    let fixture = Fixture::new();
    fixture.ok(&["init", "--objective", "Legacy objective"]);
    fixture.ok(&["checkpoint", "--decision", "kept"]);
    let refused = fixture.fails(&["session", "new", "other"]);
    assert!(refused.contains("session adopt"), "{refused}");
    let bytes = fixture.read(".chough/session.json");
    fixture.ok(&["session", "adopt", "legacy"]);
    assert!(!fixture.exists(".chough/session.json"));
    assert_eq!(fixture.read(".chough/sessions/legacy.json"), bytes);
    assert_eq!(
        fixture.read(".chough/local/active-session").trim(),
        "legacy"
    );
    fixture.ok(&["stage", "advance"]);
    assert_ne!(fixture.read(".chough/sessions/legacy.json"), bytes);

    // A legacy checkpoint without a history cursor must adopt history first.
    let old = Fixture::new();
    old.init();
    let bytes = old.read(".chough/session.json");
    let refused = old.fails(&["session", "adopt", "legacy"]);
    assert!(refused.contains("history adopt"), "{refused}");
    assert_eq!(old.read(".chough/session.json"), bytes);
    assert!(!old.exists(".chough/sessions"));
}

#[test]
fn switching_archives_done_sessions() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&["session", "use", "alpha"]);
    // Release is requested before done, so the session stays open for it (#155).
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release alpha",
    ]);
    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    fixture.ok(&["stage", "advance"]);
    left_open(&fixture, "alpha");
    assert_eq!(
        json(&fixture, ".chough/sessions/alpha.json")["stage"]["current"],
        "done"
    );
    let bytes = fixture.read(".chough/sessions/alpha.json");
    let switched = fixture.ok(&["session", "use", "beta"]);
    assert_eq!(switched["archived"], serde_json::json!(["alpha"]));
    assert!(!fixture.exists(".chough/sessions/alpha.json"));
    assert_eq!(fixture.read(".chough/sessions/archive/alpha.json"), bytes);
    let list = fixture.ok(&["session", "list"]);
    let alpha = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|session| session["key"] == "alpha")
        .unwrap()
        .clone();
    assert_eq!(alpha["archived"], true);
    assert_eq!(alpha["stage"], "done");
    let refused = fixture.fails(&["session", "use", "alpha"]);
    assert!(refused.contains("archived"), "{refused}");
    // A key stays taken once archived.
    assert!(
        fixture
            .fails(&["session", "new", "alpha"])
            .contains("already exists")
    );
}

#[test]
fn inbox_entries_fold_once_without_touching_the_session() {
    let fixture = Fixture::new();
    sessions(&fixture);
    let before = fixture.read(".chough/sessions/alpha.json");
    for (kind, text) in [
        ("observation", "OBS-parser-slow"),
        ("decision", "keep the cache"),
        ("completed", "wrote the parser test"),
        ("question", "is the cache shared?"),
        ("observation", "OBS-parser-slow"),
    ] {
        fixture.ok(&[
            "inbox",
            "add",
            "--session",
            "alpha",
            "--from",
            "subagent-1",
            "--kind",
            kind,
            "--text",
            text,
        ]);
    }
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), before);
    let inbox = fixture.root().join(".chough/sessions/alpha.inbox");
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
    let session = json(&fixture, ".chough/sessions/alpha.json");
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
    assert_eq!(session["awaiting_human"], serde_json::json!([]));
    assert_eq!(session["run_consents"], serde_json::json!([]));
    assert_eq!(fs::read_dir(inbox.join("folded")).unwrap().count(), 5);
    let after = fixture.read(".chough/sessions/alpha.json");
    let again = fixture.ok(&["inbox", "fold"]);
    assert_eq!(again["folded"], serde_json::json!([]));
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), after);

    // An entry that tries to carry an approval is refused and nothing is folded.
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        "alpha",
        "--from",
        "x",
        "--kind",
        "observation",
        "--text",
        "ok",
    ]);
    let entry = fs::read_dir(&inbox)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .find(|path| path.is_file())
        .unwrap();
    let mut record: Value = serde_json::from_str(&fs::read_to_string(&entry).unwrap()).unwrap();
    record["approval"] = Value::from("approved by Leif");
    fs::write(&entry, record.to_string()).unwrap();
    assert!(fixture.fails(&["inbox", "fold"]).contains("inbox entry"));
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), after);
}

#[test]
fn branches_with_different_sessions_merge_without_conflict() {
    let fixture = Fixture::new();
    fixture.write(
        ".gitattributes",
        ".chough/sessions/*.json merge=chough-session\n",
    );
    sessions(&fixture);
    fixture.commit("base");
    fixture.git(&["checkout", "-b", "work-alpha"]);
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        "beta",
        "--from",
        "sub",
        "--kind",
        "observation",
        "--text",
        "OBS-alpha-branch",
    ]);
    fixture.commit("alpha work");
    fixture.git(&["checkout", "main"]);
    fixture.git(&["checkout", "-b", "work-beta"]);
    fixture.ok(&["session", "use", "beta"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.commit("beta work");
    fixture.git(&["-c", "merge.ff=false", "merge", "--no-edit", "work-alpha"]);
    let alpha = json(&fixture, ".chough/sessions/alpha.json");
    let beta = json(&fixture, ".chough/sessions/beta.json");
    assert_eq!(events(&fixture, &cycle(&fixture, "alpha")), 1);
    assert_eq!(events(&fixture, &cycle(&fixture, "beta")), 2);
    assert_ne!(alpha["stage"]["current"], "intent");
    assert_ne!(beta["stage"]["current"], alpha["stage"]["current"]);
    assert_eq!(fixture.ok(&["history", "show"])["complete"], true);
    fixture.ok(&["inbox", "fold"]);
    assert!(
        json(&fixture, ".chough/sessions/beta.json")["observations"]
            .to_string()
            .contains("OBS-alpha-branch")
    );
}

#[test]
fn a_project_without_sessions_is_unchanged() {
    let fixture = Fixture::new();
    fixture.ok(&["init", "--objective", "Plain"]);
    fixture.ok(&["stage", "advance"]);
    assert!(!fixture.exists(".chough/sessions"));
    // The checkpoint writer lock lives in `.chough/local/` in every project (#83,
    // REQ-history-007): it holds only the lock and its `.gitignore`, both ignored by
    // Git, so no tracked or session file changes.
    let mut local: Vec<String> = fs::read_dir(fixture.root().join(".chough/local"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    local.sort();
    assert_eq!(local, [".gitignore", "checkpoint.lock"]);
    assert_eq!(fixture.read(".chough/local/.gitignore"), "*\n");
    assert!(fixture.fails(&["inbox", "fold"]).contains("sessions"));
}

fn finish(fixture: &Fixture, key: &str) {
    fixture.ok(&["session", "use", key]);
    // Release is requested before done, so the session stays open for it (#155).
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release it",
    ]);
    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    fixture.ok(&["stage", "advance"]);
    left_open(fixture, key);
}

/// Since #155 a session archives itself at its move into done. A done session still
/// open is one an older Chough left, or one merged in from another branch; this puts
/// the session back where such a one sits, so the switch rules can be tested.
fn left_open(fixture: &Fixture, key: &str) {
    std::fs::rename(
        fixture
            .root()
            .join(format!(".chough/sessions/archive/{key}.json")),
        fixture.root().join(format!(".chough/sessions/{key}.json")),
    )
    .unwrap();
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

/// A done session with unfolded inbox entries is not archived on a switch, so its
/// entries are not stranded; `session list` shows each session's unfolded count
/// (review of #77).
#[test]
fn a_done_session_with_unfolded_entries_stays_open_on_switch() {
    let fixture = Fixture::new();
    sessions(&fixture);
    finish(&fixture, "alpha");
    add_entry(&fixture, "alpha", "decision", "keep the cache");
    let switched = fixture.ok(&["session", "use", "beta"]);
    assert_eq!(switched["archived"], serde_json::json!([]), "{switched}");
    assert_eq!(
        switched["kept_open"],
        serde_json::json!([{"key": "alpha", "inbox_unfolded": 1}]),
        "{switched}"
    );
    assert!(fixture.exists(".chough/sessions/alpha.json"));
    assert!(!fixture.exists(".chough/sessions/archive/alpha.json"));
    let list = fixture.ok(&["session", "list"]);
    let counts: Vec<_> = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|session| {
            (
                session["key"].as_str().unwrap().to_string(),
                session["inbox_unfolded"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(counts, [("alpha".to_string(), 1), ("beta".to_string(), 0)]);
    // Folding first lets the next switch archive it.
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["inbox", "fold"]);
    let switched = fixture.ok(&["session", "use", "beta"]);
    assert_eq!(
        switched["archived"],
        serde_json::json!(["alpha"]),
        "{switched}"
    );
}

/// `inbox fold` checks every `folded/` target before it saves the session, so an
/// existing target refuses with nothing changed and a rerun is not repeated work
/// (review of #77).
#[test]
fn fold_checks_its_targets_before_saving_the_session() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha"]);
    let first = add_entry(&fixture, "alpha", "decision", "keep the cache");
    let second = add_entry(&fixture, "alpha", "decision", "drop the index");
    fixture.write(
        &format!(".chough/sessions/alpha.inbox/folded/{second}.json"),
        "{}\n",
    );
    let before = fixture.read(".chough/sessions/alpha.json");
    let refused = fixture.fails(&["inbox", "fold"]);
    assert!(refused.contains("already folded"), "{refused}");
    assert_eq!(fixture.read(".chough/sessions/alpha.json"), before);
    for id in [&first, &second] {
        assert!(fixture.exists(&format!(".chough/sessions/alpha.inbox/{id}.json")));
    }
    assert!(!fixture.exists(&format!(".chough/sessions/alpha.inbox/folded/{first}.json")));
}

/// `inbox fold` validates each entry the way `inbox add` does, so an edited entry with
/// blank text or an invalid `from` is refused with nothing folded (review of #77).
#[test]
fn fold_validates_entries_like_add() {
    for (field, value, expected) in [
        ("text", "   ", "blank"),
        ("from", "not valid!", "--from"),
        ("text", "not an id", "evidence ID"),
    ] {
        let fixture = Fixture::new();
        fixture.ok(&["session", "new", "alpha"]);
        let kind = if expected == "evidence ID" {
            "observation"
        } else {
            "decision"
        };
        let text = if kind == "observation" {
            "OBS-ok"
        } else {
            "keep the cache"
        };
        let id = add_entry(&fixture, "alpha", kind, text);
        let file = format!(".chough/sessions/alpha.inbox/{id}.json");
        let mut record: Value = serde_json::from_str(&fixture.read(&file)).unwrap();
        record[field] = Value::from(value);
        fixture.write(&file, &record.to_string());
        let before = fixture.read(".chough/sessions/alpha.json");
        let refused = fixture.fails(&["inbox", "fold"]);
        assert!(refused.contains(expected), "{field}: {refused}");
        assert_eq!(fixture.read(".chough/sessions/alpha.json"), before);
        assert!(fixture.exists(&file));
    }
}

/// A command resolves its session once: a `session use` in another process partway
/// through a command does not redirect its save into the newly active session.
#[cfg(unix)]
#[test]
fn a_switch_partway_through_a_command_does_not_redirect_its_save() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    // The SpecSync stand-in switches the active session the first time it runs after
    // .switch-to appears, as another agent's `chough session use` would.
    let stub = fixture.read("bin/specsync").replacen(
        "#!/bin/sh\n",
        "#!/bin/sh\nif [ -f .switch-to ]; then cp .switch-to .chough/local/active-session; rm .switch-to; fi\n",
        1,
    );
    fixture.write("bin/specsync", &stub);
    fixture.executable("bin/specsync");
    sessions(&fixture);
    fixture.ok(&["session", "use", "alpha"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    fixture.no_spec_approval("add-thing");
    let alpha_stage = json(&fixture, ".chough/sessions/alpha.json")["stage"]["current"].clone();
    let beta = fixture.read(".chough/sessions/beta.json");
    fixture.write(".switch-to", "beta\n");
    fixture.ok(&["stage", "advance"]);
    assert_eq!(fixture.read(".chough/local/active-session"), "beta\n");
    assert_eq!(fixture.read(".chough/sessions/beta.json"), beta);
    let alpha = json(&fixture, ".chough/sessions/alpha.json");
    assert_ne!(alpha["stage"]["current"], alpha_stage);
    assert_eq!(alpha["objective"], "Ship alpha");
}

/// REQ-cli-007: the checkpoint merge driver covers session files too.
#[test]
fn the_repository_declares_the_merge_driver_for_every_session_file() {
    let attributes =
        fs::read_to_string(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".gitattributes"))
            .unwrap()
            .replace("\r\n", "\n");
    for pattern in [
        ".chough/session.json",
        ".chough/sessions/*.json",
        ".chough/sessions/archive/*.json",
    ] {
        assert!(
            attributes
                .lines()
                .any(|line| line == format!("{pattern} merge=chough-session")),
            "{pattern}"
        );
    }
}

/// Review of #77 (blocker 1): in sessions mode, with no `.chough/session.json`,
/// concurrent read-modify-writes each apply their change to the latest checkpoint, so
/// 20 concurrent `checkpoint --decision` calls keep all 20 decisions.
#[test]
fn concurrent_decisions_in_sessions_mode_keep_every_one() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    assert!(!fixture.exists(".chough/session.json"));
    let texts: Vec<String> = (0..20)
        .map(|n| format!("concurrent decision {n}"))
        .collect();
    std::thread::scope(|scope| {
        for text in &texts {
            let fixture = &fixture;
            scope.spawn(move || {
                let output = fixture.run(&["checkpoint", "--decision", text]);
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            });
        }
    });
    let decisions = json(&fixture, ".chough/sessions/alpha.json")["decisions"].clone();
    let kept: Vec<&str> = decisions
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect();
    let missing: Vec<&String> = texts
        .iter()
        .filter(|t| !kept.contains(&t.as_str()))
        .collect();
    assert!(
        missing.is_empty(),
        "lost {} of 20: {missing:?}",
        missing.len()
    );
}

/// Review of #77 (blocker 1): `inbox fold` racing one writer keeps both the folded
/// observation and the writer's decision, in every one of 10 trials.
#[test]
fn fold_racing_a_writer_keeps_both_updates() {
    let mut lost = vec![];
    for trial in 0..10 {
        let fixture = Fixture::new();
        fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
        let observation = format!("OBS-folded-{trial}");
        add_entry(&fixture, "alpha", "observation", &observation);
        let decision = format!("racing decision {trial}");
        std::thread::scope(|scope| {
            let fixture = &fixture;
            let decision = &decision;
            scope.spawn(move || {
                let output = fixture.run(&["inbox", "fold"]);
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            });
            scope.spawn(move || {
                let output = fixture.run(&["checkpoint", "--decision", decision]);
                assert!(
                    output.status.success(),
                    "{}",
                    String::from_utf8_lossy(&output.stderr)
                );
            });
        });
        let session = json(&fixture, ".chough/sessions/alpha.json");
        let has = |field: &str, text: &str| {
            session[field]
                .as_array()
                .unwrap()
                .iter()
                .any(|value| value == text)
        };
        if !has("observations", &observation) || !has("decisions", &decision) {
            lost.push(trial);
        }
    }
    assert!(lost.is_empty(), "lost an update in trials {lost:?} of 10");
}

/// Review of #77 (blocker 2, Leif's P1 thread 4462791456 and follow-up 4464264245):
/// legacy writers racing `session adopt alpha` end in exactly one mode. After a
/// successful adopt, `.chough/session.json` is gone and `.chough/sessions/` holds the
/// session; a refused adopt leaves the legacy checkpoint and no sessions directory.
#[test]
fn legacy_writers_racing_adopt_end_in_one_mode() {
    let mut both = vec![];
    for trial in 0..20 {
        let fixture = Fixture::new();
        fixture.ok(&["init", "--objective", "Legacy objective"]);
        std::thread::scope(|scope| {
            let fixture = &fixture;
            for writer in 0..4 {
                scope.spawn(move || {
                    fixture.run(&["checkpoint", "--decision", &format!("writer {writer}")]);
                });
            }
            scope.spawn(move || {
                fixture.run(&["session", "adopt", "alpha"]);
            });
        });
        let legacy = fixture.exists(".chough/session.json");
        let sessions = fixture.exists(".chough/sessions");
        if legacy == sessions {
            both.push(trial);
        }
    }
    assert!(
        both.is_empty(),
        "not exactly one mode in trials {both:?} of 20"
    );
}

/// Review of #77 (gap ii): a session-file temp left by a killed writer
/// (`.chough/sessions/.<key>.json.<pid>.tmp`) is removed by the next checkpoint write
/// and is ignored by Git, so `git add -A` never commits it.
#[test]
fn orphaned_session_file_temps_are_removed_and_ignored() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let temp = ".chough/sessions/.alpha.json.4242.tmp";
    fixture.write(temp, "{\"partial\":");
    let init = std::process::Command::new("git")
        .args(["init", "-q"])
        .current_dir(fixture.root())
        .status()
        .unwrap();
    assert!(init.success());
    let ignored = std::process::Command::new("git")
        .args(["check-ignore", "-q", "--no-index", temp])
        .current_dir(fixture.root())
        .status()
        .unwrap();
    assert!(ignored.success(), "{temp} is not ignored by Git");
    fixture.ok(&["checkpoint", "--decision", "after the killed writer"]);
    assert!(!fixture.exists(temp), "{temp} was left behind");
    assert!(fixture.exists(".chough/sessions/alpha.json"));
}

/// Issue #84: a subagent's question folded from the inbox is shown by `next` and
/// `status`, not parked silently in `open_questions` (LIFE-3).
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
}

/// Issue #84 item a: an archived session is listed with status `archived`, not the
/// `active` stage status it was archived with.
#[test]
fn session_list_reports_an_archived_session_as_archived() {
    let fixture = Fixture::new();
    sessions(&fixture);
    finish(&fixture, "alpha");
    fixture.ok(&["session", "use", "beta"]);
    assert!(fixture.exists(".chough/sessions/archive/alpha.json"));
    let list = fixture.ok(&["session", "list"]);
    let entry = |key: &str, archived: bool| {
        list["sessions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["key"] == key && entry["archived"] == archived)
            .cloned()
            .unwrap_or_else(|| panic!("{key} missing: {list:#}"))
    };
    assert_eq!(entry("alpha", true)["status"], "archived", "{list:#}");
    assert_eq!(entry("alpha", true)["stage"], "done");
    assert_eq!(entry("beta", false)["status"], "active", "{list:#}");
}

/// Issue #84 item b, as Leif chose on 2026-11-04: an observation entry whose ID has no
/// evidence record is still added, with a warning naming it, and `inbox list` marks it.
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
    assert!(
        !String::from_utf8_lossy(&output.stderr).contains("no record"),
        "a recorded observation gets no warning"
    );
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

#[test]
fn session_new_accepts_the_module_ids_init_accepts() {
    // Review thread 4465229406: `session new --module chat.service` was refused while
    // `init --module chat.service` worked.
    let fixture = Fixture::new();
    fixture.ok(&[
        "session",
        "new",
        "alpha",
        "--objective",
        "Ship alpha",
        "--module",
        "chat.service",
        "--module",
        "identity-wipe.service",
    ]);
    let session = json(&fixture, ".chough/sessions/alpha.json");
    assert_eq!(
        session["modules"],
        serde_json::json!(["chat.service", "identity-wipe.service"])
    );
    for bad in ["a/b", "..", "trailing.", " lead"] {
        let refused = fixture.fails(&[
            "session",
            "new",
            "beta",
            "--objective",
            "Ship beta",
            "--module",
            bad,
        ]);
        assert!(refused.contains("invalid module name"), "{bad}: {refused}");
        assert!(!fixture.exists(".chough/sessions/beta.json"), "{bad}");
    }
}

/// Issue #98 (REQ-history-009): an `inbox add` racing a switch that archives its done session never leaves its entry
/// beside the archived session: either the switch sees the entry and keeps the session
/// open, or the add sees the session archived and refuses. The test holds the
/// checkpoint lock as `session use` does while it archives, then archives alpha itself.
#[cfg(unix)]
#[test]
fn inbox_add_racing_an_archive_never_strands_its_entry() {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};
    let fixture = Fixture::new();
    sessions(&fixture);
    finish(&fixture, "alpha");
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
    // An add that does not wait for the lock finishes well within this.
    let started = Instant::now();
    while started.elapsed() < Duration::from_secs(2) && add.try_wait().unwrap().is_none() {
        std::thread::sleep(Duration::from_millis(25));
    }
    fs::create_dir_all(fixture.root().join(".chough/sessions/archive")).unwrap();
    fs::rename(
        fixture.root().join(".chough/sessions/alpha.json"),
        fixture.root().join(".chough/sessions/archive/alpha.json"),
    )
    .unwrap();
    drop(lock);
    let output = add.wait_with_output().unwrap();
    let stranded: Vec<_> = fs::read_dir(fixture.root().join(".chough/sessions/alpha.inbox"))
        .map(|entries| {
            entries
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .filter(|name| name.ends_with(".json"))
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
/// while writing its entry (here by the file-size limit, which stops it at its first
/// write) leaves no partial entry. `inbox list`, `status`, `next` and `evidence prune`
/// still work, the temp it left is ignored by Git, and the next checkpoint save removes it.
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
        "the add was not stopped while writing: {killed:?}"
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
    fixture.ok(&["evidence", "prune"]);
    let inbox = fixture.root().join(".chough/sessions/beta.inbox");
    let temps: Vec<String> = fs::read_dir(&inbox)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".tmp"))
        .collect();
    assert_eq!(temps.len(), 1, "{temps:?}");
    fixture.commit("after the killed add");
    let temp = format!(".chough/sessions/beta.inbox/{}", temps[0]);
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

/// Session alpha at plan with an approved native change, whose stub `specsync` runs
/// `before` first on each call; beta is a second open session. Returns alpha's cycle.
#[cfg(unix)]
fn alpha_at_plan(fixture: &Fixture, before: &str) -> String {
    fixture.stubs();
    fixture.native("approved");
    sessions(fixture);
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
        &format!("#!/bin/sh\n{before}\nexec ./bin/specsync-stub \"$@\"\n"),
    );
    fixture.executable("bin/specsync");
    cycle(fixture, "alpha")
}

/// Issue #82 (REQ-history-008): #77's writer lock makes `session use` wait for a
/// running `stage advance`, so Chough itself never archives or switches a session
/// under a command and no orphan event is left. Guards the lock; passes at 75a732d.
#[cfg(unix)]
#[test]
fn session_use_waits_for_a_running_stage_advance() {
    use std::process::{Command, Stdio};
    use std::time::{Duration, Instant};
    let fixture = Fixture::new();
    let cycle = alpha_at_plan(&fixture, "[ -e .slow ] && touch running && sleep 2");
    let before = events(&fixture, &cycle);
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
    assert_eq!(
        json(&fixture, ".chough/sessions/alpha.json")["stage"]["current"],
        "build"
    );
    assert_eq!(
        events(&fixture, &cycle),
        before + 1,
        "exactly the advance's event"
    );
}

/// Issue #82 (REQ-history-008, Leif's 2026-11-04 choice): when something outside Chough
/// moves the session file away during `stage advance`, the refused save leaves no
/// orphan event: the event that command published is removed.
#[cfg(unix)]
#[test]
fn a_save_refused_because_the_session_vanished_leaves_no_event() {
    let fixture = Fixture::new();
    let cycle = alpha_at_plan(
        &fixture,
        "if [ -e .vanish ]; then rm .vanish; mkdir -p .chough/sessions/archive; mv .chough/sessions/alpha.json .chough/sessions/archive/alpha.json; fi",
    );
    let before = events(&fixture, &cycle);
    fixture.write(".vanish", "");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("nothing was saved"), "{refused}");
    assert_eq!(events(&fixture, &cycle), before, "an orphan event was left");
    assert!(
        !fixture.exists(".chough/sessions/alpha.json"),
        "alpha.json was recreated"
    );
}

/// Issue #83 (REQ-history-007, #94 review): in sessions mode `.chough/local/` holds only
/// `.gitignore`, `checkpoint.lock` and `active-session`, and Git ignores all three.
#[test]
fn in_sessions_mode_local_holds_only_the_pointer_and_the_lock() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&["checkpoint", "--decision", "Take the lock"]);
    let mut local: Vec<String> = fs::read_dir(fixture.root().join(".chough/local"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    local.sort();
    assert_eq!(local, [".gitignore", "active-session", "checkpoint.lock"]);
    fixture.commit("fixture");
    let tracked = std::process::Command::new("git")
        .current_dir(fixture.root())
        .args(["ls-files", ".chough/local"])
        .output()
        .unwrap();
    assert!(
        tracked.stdout.is_empty(),
        "{}",
        String::from_utf8_lossy(&tracked.stdout)
    );
}

/// Adopting sessions on this repository: after a legacy project adopts its transition
/// log and its checkpoint, a session started with `session new` writes events to its own
/// cycle. The frozen legacy log stays guarded by the anchor the adopting session
/// recorded, and is not part of the new session's history (REQ-history-006).
#[test]
fn a_session_started_after_legacy_adoption_writes_events_and_keeps_the_log_guarded() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    let log = fixture.read(".chough/transitions.jsonl");
    assert!(!log.trim().is_empty(), "the legacy project has a log");
    fixture.ok(&["history", "adopt", "--apply"]);
    fixture.ok(&["session", "adopt", "legacy"]);
    fixture.ok(&["session", "new", "other", "--objective", "Other work"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(events(&fixture, &cycle(&fixture, "other")), 1);
    assert_eq!(fixture.read(".chough/transitions.jsonl"), log);
    // Its history is its own: history walks read the legacy log only through a cursor
    // that carries the anchor, and the new session's does not.
    assert!(json(&fixture, ".chough/sessions/other.json")["history_cursor"]["legacy"].is_null());

    // A changed legacy log still refuses new writes, in either session.
    fixture.write(".chough/transitions.jsonl", &format!("{log}\n"));
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("legacy history anchor changed"),
        "{refused}"
    );
    fixture.ok(&["session", "use", "legacy"]);
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("legacy history anchor changed"),
        "{refused}"
    );
}

/// Fills a session file past the 1 MiB checkpoint cap (DRIVE-14).
fn oversize(fixture: &Fixture, relative: &str) -> String {
    let mut session = json(fixture, relative);
    for list in ["decisions", "completed", "open_questions"] {
        session[list] = serde_json::json!(
            (0..100)
                .map(|index| format!("{list} {index} {}", "x".repeat(4_000)))
                .collect::<Vec<_>>()
        );
    }
    let text = serde_json::to_string(&session).unwrap();
    assert!(text.len() > common::CHECKPOINT_CAP, "{}", text.len());
    fixture.write(relative, &text);
    text
}

/// DRIVE-14: one oversized or unreadable session does not break the others. `session
/// list` reports it per session and names `chough session shelve <key>`; `next` on the
/// active session and `session new` still work.
#[test]
fn one_oversized_or_unreadable_session_does_not_break_the_others() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&["session", "use", "beta"]);
    oversize(&fixture, ".chough/sessions/alpha.json");
    let list = fixture.ok(&["session", "list"]);
    let alpha = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|session| session["key"] == "alpha")
        .cloned()
        .unwrap_or_else(|| panic!("{list:#}"));
    assert!(
        alpha.to_string().contains("chough session shelve alpha"),
        "{alpha:#}"
    );
    fixture.ok(&["next", "--json"]);
    fixture.ok(&["session", "new", "gamma", "--objective", "Ship gamma"]);
    fixture.write(".chough/sessions/beta.json", "{ not json\n");
    let list = fixture.ok(&["session", "list"]);
    assert!(
        list.to_string().contains("chough session shelve beta"),
        "{list:#}"
    );
    fixture.ok(&["next", "--json"]);
}

/// DRIVE-14: an oversized session is shelved unparsed, with the reason recorded, and
/// `session new` works again; its bytes are unchanged in `.chough/sessions/shelved/`.
#[test]
fn shelving_an_oversized_session_lets_work_continue() {
    let fixture = Fixture::new();
    sessions(&fixture);
    fixture.ok(&["session", "use", "alpha"]);
    let bytes = oversize(&fixture, ".chough/sessions/alpha.json");
    let refused = fixture.fails(&["next"]);
    assert!(refused.contains("1048576"), "{refused}");
    assert!(refused.contains("chough session shelve alpha"), "{refused}");
    let shelved = fixture.ok(&["session", "shelve", "alpha"]);
    assert_eq!(
        fixture.read(".chough/sessions/shelved/alpha.json"),
        bytes,
        "{shelved:#}"
    );
    assert!(!fixture.exists(".chough/sessions/alpha.json"));
    assert!(shelved.to_string().contains("1048576"), "{shelved:#}");
    fixture.ok(&[
        "session",
        "new",
        "alpha-2",
        "--objective",
        "Ship alpha again",
    ]);
    fixture.ok(&["next", "--json"]);
    let list = fixture.ok(&["session", "list"]);
    assert!(!list.to_string().contains("\"alpha\""), "{list:#}");
}

/// DRIVE-14: an oversized legacy checkpoint is shelved with `--legacy`, and a session
/// can then start.
#[test]
fn shelving_an_oversized_legacy_checkpoint_lets_a_session_start() {
    let fixture = Fixture::new();
    fixture.ok(&["init", "--objective", "Legacy objective"]);
    let bytes = oversize(&fixture, ".chough/session.json");
    let refused = fixture.fails(&["next"]);
    assert!(
        refused.contains("chough session shelve --legacy"),
        "{refused}"
    );
    fixture.ok(&["session", "shelve", "--legacy"]);
    assert_eq!(fixture.read(".chough/sessions/shelved/legacy.json"), bytes);
    assert!(!fixture.exists(".chough/session.json"));
    fixture.ok(&["session", "new", "fresh", "--objective", "Next change"]);
    fixture.ok(&["next", "--json"]);
}

/// #141 (P1): the session that adopted the legacy history holds its anchor. Once KNOW-29
/// rolls it into the vault, or it is shelved, other sessions still find the anchor and
/// keep writing; a changed legacy log is still refused.
#[test]
fn sessions_still_write_after_the_anchor_holder_rolls_into_the_vault_or_is_shelved() {
    for destination in [
        "knowledge/attachments/legacy/sessions/legacy.json",
        ".chough/sessions/shelved/legacy.json",
    ] {
        let fixture = Fixture::new();
        fixture.init();
        fixture.ok(&["stage", "advance"]);
        let log = fixture.read(".chough/transitions.jsonl");
        fixture.ok(&["history", "adopt", "--apply"]);
        fixture.ok(&["session", "adopt", "legacy"]);
        fixture.ok(&["session", "new", "other", "--objective", "Other work"]);
        let holder = fixture.read(".chough/sessions/legacy.json");
        assert!(
            holder.contains("\"legacy\""),
            "the adopting session holds the anchor"
        );
        fs::remove_file(fixture.root().join(".chough/sessions/legacy.json")).unwrap();
        fixture.write(destination, &holder);
        fixture.ok(&["stage", "advance"]);
        assert_eq!(
            fixture.read(".chough/transitions.jsonl"),
            log,
            "{destination}"
        );
        fixture.write(".chough/transitions.jsonl", &format!("{log}\n"));
        let refused = fixture.fails(&["stage", "advance"]);
        assert!(
            refused.contains("legacy history anchor changed"),
            "{destination}: {refused}"
        );
    }
}
