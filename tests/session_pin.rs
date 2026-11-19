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

/// REQ-records-006, REQ-history-006 (#367), TC-records-errors-show-a-value-only-when-clean:
/// a session name from `CHOUGH_SESSION`, `--session` or `session use` that holds a token
/// or a control character is refused naming the variable or option and what it holds,
/// never the name: the output holds neither the token nor the escape. A clean unknown
/// name is still named. Each holds with no session open and with one open.
#[test]
fn a_session_name_is_shown_only_when_clean() {
    let token = concat!("x ghp_", "abcdefghij0123456789", "abcdefghij0123 y");
    let escape = "a\u{1b}[31mb";
    for open in [false, true] {
        let fixture = Fixture::new();
        if open {
            fixture.init();
        } else {
            fixture.ok(&["init"]);
        }
        let before = tree(fixture.root());
        for (value, held) in [(token, "token"), (escape, "control characters")] {
            for output in [
                fixture.run_with_env(&["next"], &[("CHOUGH_SESSION", value)]),
                fixture.run(&["next", "--session", value]),
                fixture.run(&["session", "use", value]),
            ] {
                let text = said(&output);
                assert!(!output.status.success(), "{text}");
                assert!(!text.contains("ghp_"), "the token was shown: {text}");
                assert!(!text.contains('\u{1b}'), "the escape was shown: {text:?}");
            }
            // With sessions to look in, `session use` says what the name holds; with
            // none, it names no value at all.
            if open {
                let text = said(&fixture.run(&["session", "use", value]));
                assert!(text.contains(&format!("holding {held}")), "{text}");
            }
            let text = said(&fixture.run_with_env(&["next"], &[("CHOUGH_SESSION", value)]));
            assert!(
                text.contains(&format!("CHOUGH_SESSION holds {held}")),
                "{text}"
            );
            let text = said(&fixture.run(&["next", "--session", value]));
            assert!(text.contains(&format!("--session holds {held}")), "{text}");
        }
        let text = said(&fixture.run_with_env(&["next"], &[("CHOUGH_SESSION", "alpha")]));
        assert!(text.contains("CHOUGH_SESSION names alpha"), "{text}");
        if open {
            let text = said(&fixture.run(&["session", "use", "alpha"]));
            assert!(text.contains("no session alpha"), "{text}");
        }
        assert_eq!(tree(fixture.root()), before);
    }
}
