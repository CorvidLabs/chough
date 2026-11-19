//! REQ-cli-037 (#271): a shell pins its session with `--session <id>` or the
//! `CHOUGH_SESSION` variable, so each agent in one worktree writes to its own session
//! whatever the shared active-session pointer says, and the pointer never moves.
#![cfg(unix)]

mod common;

use common::Fixture;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// Every file under `root` but `.git` and `.chough/local`, with its bytes.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            let name = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if name == ".git" || name.starts_with(".chough/local") {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn said(output: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// Two open sessions with the pointer on the first: (fixture, first, other).
fn two_sessions() -> (Fixture, String, String) {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.configure_human();
    fixture.init();
    let first = fixture.active_id();
    fixture.ok(&[
        "session",
        "new",
        "SYNTHETIC-other",
        "--objective",
        "Other work",
    ]);
    let other = fixture.active_id();
    fixture.ok(&["session", "use", &first]);
    assert_eq!(fixture.active_id(), first);
    fixture.commit("two sessions");
    (fixture, first, other)
}

fn evidence_add(summary: &str) -> Vec<String> {
    [
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
    ]
    .iter()
    .map(|part| part.to_string())
    .collect()
}

/// The observation references in `session`'s record.
fn observations(fixture: &Fixture, session: &str) -> usize {
    let text = fixture.read(&format!(".chough/sessions/{session}/session.md"));
    let line = text
        .lines()
        .find(|line| line.starts_with("observations:"))
        .expect("an observations line")
        .to_string();
    line.matches("2026-").count()
}

/// REQ-cli-037, TC-cli-a-pinned-shell-writes-its-own-session: with the pointer on the
/// first session, `--session <other>` and `CHOUGH_SESSION=<other>` each record into the
/// other session, the pointer stays, and `next --session <other>` shows that session.
#[test]
fn a_pinned_shell_writes_its_own_session_whatever_the_pointer_says() {
    let (fixture, first, other) = two_sessions();
    let mut args = evidence_add("SYNTHETIC entry by flag");
    args.extend(["--session".to_string(), other.clone()]);
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let recorded = fixture.ok(&args);
    assert!(
        recorded["recorded"]
            .as_str()
            .unwrap_or_default()
            .contains(&other),
        "{recorded:#}"
    );
    let args = evidence_add("SYNTHETIC entry by variable");
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let output = fixture.run_with_env(&args, &[("CHOUGH_SESSION", &other)]);
    assert!(output.status.success(), "{}", said(&output));
    assert_eq!(fixture.active_id(), first, "the pointer moved");
    assert_eq!(observations(&fixture, &other), 2);
    assert_eq!(observations(&fixture, &first), 0);

    let next = fixture.run(&["next", "--session", &other]);
    assert!(
        said(&next).contains("Objective: Other work"),
        "{}",
        said(&next)
    );
    let next = fixture.run(&["next"]);
    assert!(
        !said(&next).contains("Objective: Other work"),
        "{}",
        said(&next)
    );
}

/// REQ-cli-037: a `--session` or `CHOUGH_SESSION` naming an unknown or archived session
/// refuses with every file unchanged, naming `chough session list`.
#[test]
fn a_pin_to_a_session_that_is_not_open_refuses_with_nothing_written() {
    let (fixture, _first, other) = two_sessions();
    fixture.ok(&[
        "session",
        "close",
        &other,
        "--reason",
        "SYNTHETIC done elsewhere",
    ]);
    fixture.commit("the other session closed");
    let decision = ["checkpoint", "--decision", "decided: SYNTHETIC pinned"];
    for (flag, env) in [
        (Some("SYNTHETIC-nowhere"), None),
        (Some(other.as_str()), None),
        (None, Some("SYNTHETIC-nowhere")),
        (None, Some(other.as_str())),
    ] {
        let before = tree(fixture.root());
        let mut args: Vec<&str> = decision.to_vec();
        if let Some(flag) = flag {
            args.extend(["--session", flag]);
        }
        let output = match env {
            Some(value) => fixture.run_with_env(&args, &[("CHOUGH_SESSION", value)]),
            None => fixture.run(&args),
        };
        let text = said(&output);
        assert!(
            !output.status.success(),
            "{args:?} {env:?} was written: {text}"
        );
        assert!(text.contains("chough session list"), "{text}");
        assert_eq!(
            tree(fixture.root()),
            before,
            "{args:?} {env:?} changed a file"
        );
    }
}

/// REQ-cli-037: `init --session` still names the first session's key, `inbox add
/// --session` still names its session, and `session use` ignores `CHOUGH_SESSION`.
#[test]
fn init_and_inbox_add_keep_their_session_and_session_use_ignores_the_variable() {
    let fixture = Fixture::new();
    fixture.configure_human();
    let init = fixture.ok(&[
        "init",
        "--objective",
        "Pinned work",
        "--session",
        "SYNTHETIC-key",
    ]);
    assert!(init.to_string().contains("synthetic-key"), "{init:#}");
    let first = fixture.active_id();
    fixture.ok(&[
        "session",
        "new",
        "SYNTHETIC-second",
        "--objective",
        "Second",
    ]);
    let second = fixture.active_id();
    let output = fixture.run_with_env(&["session", "use", &first], &[("CHOUGH_SESSION", &second)]);
    assert!(output.status.success(), "{}", said(&output));
    assert_eq!(fixture.active_id(), first);
    let output = fixture.run(&[
        "inbox",
        "add",
        "--session",
        &second,
        "--from",
        "SYNTHETIC-helper",
        "--kind",
        "question",
        "--text",
        "SYNTHETIC question for the second session",
    ]);
    assert!(output.status.success(), "{}", said(&output));
}

/// REQ-cli-037 (#334), TC-cli-a-pinned-session-before-any-session-names-session-new: with
/// `CHOUGH_SESSION` set in a project where no session is open yet, a command refuses
/// naming the variable and `chough session new <key>`, the way on, not `chough session
/// list`, which would list nothing; `--session` is named the same way, as an option to
/// drop rather than a variable to unset. With every session archived, the refusal still
/// says the named one is archived. Nothing is written.
#[test]
fn a_pinned_session_before_any_session_names_session_new() {
    let fixture = Fixture::new();
    // `chough init` alone creates no session (REQ-cli-037).
    fixture.ok(&["init"]);
    let before = tree(fixture.root());
    let output = fixture.run_with_env(&["next"], &[("CHOUGH_SESSION", "alpha")]);
    let text = said(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("refused:session-not-open"), "{text}");
    assert!(text.contains("CHOUGH_SESSION"), "{text}");
    assert!(text.contains("chough session new <key>"), "{text}");
    assert!(text.contains("unset CHOUGH_SESSION"), "{text}");
    assert!(!text.contains("chough session list"), "{text}");
    assert_eq!(tree(fixture.root()), before);

    let output = fixture.run(&["next", "--session", "alpha"]);
    let text = said(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains("--session"), "{text}");
    assert!(text.contains("chough session new <key>"), "{text}");
    assert!(text.contains("drop --session"), "{text}");
    assert!(!text.contains("unset --session"), "{text}");
    assert_eq!(tree(fixture.root()), before);

    // Every session archived: the way on is still a new session, and the refusal keeps
    // saying that the named session is archived.
    let (fixture, first, other) = two_sessions();
    for id in [&other, &first] {
        fixture.ok(&["session", "close", id, "--reason", "SYNTHETIC done"]);
    }
    fixture.commit("every session closed");
    let before = tree(fixture.root());
    let archived = format!("session {other} is archived");
    let output = fixture.run_with_env(&["next"], &[("CHOUGH_SESSION", other.as_str())]);
    let text = said(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains(&archived), "{text}");
    assert!(text.contains("chough session new <key>"), "{text}");
    assert!(text.contains("unset CHOUGH_SESSION"), "{text}");
    assert_eq!(tree(fixture.root()), before);

    let output = fixture.run(&["next", "--session", &other]);
    let text = said(&output);
    assert!(!output.status.success(), "{text}");
    assert!(text.contains(&archived), "{text}");
    assert!(text.contains("drop --session"), "{text}");
    assert_eq!(tree(fixture.root()), before);
}

/// REQ-history-009 (#393): `inbox list` honors the pin as `inbox fold` does. With the
/// pointer on one session and `--session` or `CHOUGH_SESSION` naming another, it lists
/// the named session's entries and names that session.
#[test]
fn inbox_list_lists_the_pinned_session() {
    let (fixture, first, other) = two_sessions();
    for (session, text) in [
        (&first, "SYNTHETIC for first"),
        (&other, "SYNTHETIC for other"),
    ] {
        fixture.ok(&[
            "inbox",
            "add",
            "--session",
            session,
            "--kind",
            "decision",
            "--text",
            text,
            "--from",
            "SYNTHETIC-agent",
        ]);
    }
    let by_option = fixture.ok(&["--session", &other, "inbox", "list"]);
    let output = fixture.run_with_env(&["inbox", "list"], &[("CHOUGH_SESSION", &other)]);
    assert!(output.status.success(), "{}", said(&output));
    let by_variable: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    for listed in [&by_option, &by_variable] {
        assert_eq!(listed["session"], other.as_str(), "{listed}");
        let texts: Vec<&str> = listed["entries"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|entry| entry["text"].as_str())
            .collect();
        assert_eq!(texts, ["SYNTHETIC for other"], "{listed}");
    }
}

/// REQ-cli-037 (#372): `session use` and `session new` still ignore the pin, but say
/// on standard error when `--session` or `CHOUGH_SESSION` still names another open
/// session; with neither set, or when it names the session made active, they say
/// nothing.
#[test]
fn session_use_and_new_say_when_a_pin_still_names_another_session() {
    let (fixture, first, other) = two_sessions();
    let stderr =
        |output: &std::process::Output| String::from_utf8_lossy(&output.stderr).into_owned();
    let used = fixture.run_with_env(&["session", "use", &other], &[("CHOUGH_SESSION", &first)]);
    assert!(used.status.success(), "{}", said(&used));
    assert_eq!(fixture.active_id(), other);
    let note = stderr(&used);
    assert!(
        note.contains("CHOUGH_SESSION") && note.contains(&first),
        "{note}"
    );
    let by_option = fixture.run(&["--session", &first, "session", "use", &other]);
    let note = stderr(&by_option);
    assert!(
        note.contains("--session") && note.contains(&first),
        "{note}"
    );
    let made = fixture.run_with_env(
        &["session", "new", "SYNTHETIC-third", "--objective", "Third"],
        &[("CHOUGH_SESSION", &first)],
    );
    assert!(made.status.success(), "{}", said(&made));
    assert!(stderr(&made).contains(&first), "{}", stderr(&made));
    // Unset, or naming the session made active: nothing said.
    let quiet = fixture.run(&["session", "use", &first]);
    assert_eq!(stderr(&quiet), "");
    let same = fixture.run_with_env(&["session", "use", &other], &[("CHOUGH_SESSION", &other)]);
    assert_eq!(stderr(&same), "");
}
