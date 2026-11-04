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
    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: release alpha",
    ]);
    fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    fixture.ok(&["stage", "advance"]);
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
    assert!(!fixture.exists(".chough/local"));
    assert!(fixture.fails(&["inbox", "fold"]).contains("sessions"));
}
