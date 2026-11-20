//! `chough checkpoint` field setters that have no other home.

mod common;

use common::Fixture;

/// REQ-cli-007 (#282, #353), TC-cli-checkpoint-pr-sets-the-pull-request: `checkpoint --pr`
/// sets the session's pull request, a number or an `http(s)` URL, which another process
/// reads back, written as a double-quoted string; `#12` is stored as `12`; an empty value
/// removes `pr`. Any other value is refused naming the two forms, never the value, with
/// the session file byte-identical; a line break is a control character
/// (REQ-records-016); a URL holding a token is refused, or redacted under `--redact`.
/// Like `--objective` and `--title` it adds no Timeline entry.
#[test]
fn checkpoint_pr_sets_and_clears_the_pull_request() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let file = fixture.session_file();
    let timeline = |text: &str| text.split_once("## Timeline").unwrap().1.to_string();
    // The session's `pr` front matter line, as written.
    let pr_line = || {
        fixture
            .read(&file)
            .lines()
            .find(|line| line.starts_with("pr:"))
            .map(str::to_string)
    };
    let before = fixture.read(&file);

    fixture.ok(&["checkpoint", "--pr", "150"]);
    assert_eq!(pr_line().as_deref(), Some("pr: \"150\""));
    assert_eq!(fixture.session()["pr"], "150");
    assert_eq!(
        timeline(&fixture.read(&file)),
        timeline(&before),
        "--pr added a Timeline entry"
    );

    let url = "https://github.com/CorvidLabs/jackdaw/pull/150";
    fixture.ok(&["checkpoint", "--pr", url]);
    assert_eq!(pr_line(), Some(format!("pr: \"{url}\"")));
    fixture.ok(&["checkpoint", "--pr", "#12"]);
    assert_eq!(pr_line().as_deref(), Some("pr: \"12\""));

    let set = fixture.read(&file);
    for value in [
        "0",
        "#0",
        "012",
        "src/pr.txt",
        "not a pr",
        "ftp://x/1",
        "42](javascript:alert(1))",
        "https://x/1)[click](javascript:alert(1)",
        concat!("/Users/", "someone", "/notes/pr.txt"),
    ] {
        let refused = fixture.fails(&["checkpoint", "--pr", value]);
        assert!(
            refused.contains("a pull request number") && refused.contains("URL"),
            "{value:?}: {refused}"
        );
        assert!(!refused.contains(value), "{value:?} was echoed: {refused}");
        assert_eq!(fixture.read(&file), set, "--pr {value:?} changed the file");
    }
    let refused = fixture.fails(&["checkpoint", "--pr", "150\n151"]);
    assert!(
        refused.contains("refused:control-character") && refused.contains("U+000A"),
        "{refused}"
    );
    assert_eq!(fixture.read(&file), set, "a refused --pr changed the file");
    let token = concat!(
        "https://github.com/CorvidLabs/jackdaw/pull/150?token=",
        "ghp_",
        "abcdefghijklmnopqrstuvwxyz0123456789"
    );
    let leaked = fixture.fails(&["checkpoint", "--pr", token]);
    assert!(leaked.contains("refused:leak"), "{leaked}");
    assert_eq!(fixture.read(&file), set, "a refused --pr changed the file");
    fixture.ok(&["--redact", "checkpoint", "--pr", token]);
    assert!(
        pr_line().is_some_and(|line| line.contains("<redacted:") && !line.contains("ghp_")),
        "{:?}",
        pr_line()
    );

    fixture.ok(&["checkpoint", "--pr", ""]);
    assert_eq!(pr_line(), None, "--pr \"\" left a pr line");
    assert!(fixture.session()["pr"].is_null() || fixture.session()["pr"] == "");
}

/// REQ-cli-007 (#353), TC-cli-the-pull-request-shows-in-every-view: with `pr` set,
/// `chough next`, `chough status`, `chough context` and `chough checkpoint` each show
/// it, in text and JSON; a session holding `pr: ""`, as many existing ones do, shows no
/// pull request in any of them.
#[test]
fn the_pull_request_shows_in_next_status_context_and_the_checkpoint_view() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let views: [&[&str]; 6] = [
        &["next"],
        &["next", "--json"],
        &["status"],
        &["context"],
        &["context", "--json"],
        &["checkpoint"],
    ];
    // A session as many existing ones are, holding `pr: ""`.
    fixture.set_front("pr", "\"\"");
    for view in views {
        let shown = fixture.ok_text(view);
        assert!(
            !shown.contains("PR:"),
            "{view:?} shows an empty PR: {shown}"
        );
        assert!(
            !shown.contains("\"pr\": \"\"") && !shown.contains("\"pr\":\"\""),
            "{view:?} shows an empty pr: {shown}"
        );
    }
    let url = "https://github.com/CorvidLabs/jackdaw/pull/4817";
    fixture.ok(&["checkpoint", "--pr", url]);
    for view in views {
        let shown = fixture.ok_text(view);
        assert!(
            shown.contains(url),
            "{view:?} does not show the PR: {shown}"
        );
    }
    let next = fixture.ok_text(&["next"]);
    assert!(
        next.lines().any(|line| line == format!("PR: {url}")),
        "{next}"
    );
}

/// The active session's `pr` front matter line, as written.
fn pr_line_of(fixture: &Fixture, file: &str) -> Option<String> {
    fixture
        .read(file)
        .lines()
        .find(|line| line.starts_with("pr:"))
        .map(str::to_string)
}

/// REQ-cli-007 (#353), TC-cli-every-session-writer-keeps-the-pull-request-form: every
/// writer of a session file keeps the pull request double-quoted, and keeps `pr` out once
/// `--pr ""` cleared it, not only `checkpoint --pr`: a checkpoint update, a gate opened
/// and cleared, stage moves, an inbox fold, `session new` and `session use`, `doctor
/// --fix`, `session shelve` and the story `session close` writes; a session that never
/// set one keeps its `pr: ""`, as many existing ones do, which reads as none.
#[test]
fn every_session_writer_keeps_the_pull_request_quoted_and_a_cleared_one_out() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    fixture.commit("fixture");
    let first = fixture.active_id();
    let file = fixture.session_file();
    // A session as many existing ones are keeps its empty value, which reads as none.
    fixture.set_front("pr", "\"\"");
    fixture.ok(&["checkpoint", "--decision", "SYNTHETIC first decision"]);
    assert_eq!(pr_line_of(&fixture, &file).as_deref(), Some("pr: \"\""));
    assert!(fixture.checkpoint().get("pr").is_none());

    fixture.ok(&["checkpoint", "--pr", "42"]);
    let writers: Vec<Vec<&str>> = vec![
        vec!["checkpoint", "--decision", "SYNTHETIC decision"],
        vec!["checkpoint", "--awaiting-human", "SYNTHETIC gate"],
        vec!["checkpoint", "--clear-awaiting-human"],
        vec!["stage", "advance", "--json"],
        vec![
            "stage",
            "back",
            "--to",
            "intent",
            "--reason",
            "SYNTHETIC back",
        ],
        vec![
            "inbox",
            "add",
            "--session",
            &first,
            "--from",
            "SYNTHETIC-agent",
            "--kind",
            "question",
            "--text",
            "SYNTHETIC question",
        ],
        vec!["inbox", "fold"],
        vec!["session", "new", "other", "--objective", "SYNTHETIC other"],
        vec!["session", "use", &first],
        vec!["doctor", "--fix"],
    ];
    let run = |writer: &[&str]| {
        let output = fixture.run(writer);
        assert!(
            output.status.success(),
            "{writer:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    for writer in &writers {
        run(writer);
        assert_eq!(
            pr_line_of(&fixture, &file).as_deref(),
            Some("pr: \"42\""),
            "{writer:?} changed the pr form"
        );
    }
    fixture.ok(&["checkpoint", "--pr", ""]);
    for writer in &writers[..4] {
        run(writer);
        assert_eq!(
            pr_line_of(&fixture, &file),
            None,
            "{writer:?} wrote the cleared pr back"
        );
    }

    // Shelving moves the file as it is; the story a close writes keeps the form.
    fixture.ok(&[
        "checkpoint",
        "--pr",
        "https://github.com/CorvidLabs/jackdaw/pull/42",
    ]);
    let other = fixture.id_of("other");
    fixture.ok(&["session", "use", &other]);
    fixture.ok(&["checkpoint", "--pr", "7"]);
    fixture.ok(&["session", "shelve", &other]);
    let shelved = format!(".chough/archive/shelved/{other}/session.md");
    assert_eq!(pr_line_of(&fixture, &shelved).as_deref(), Some("pr: \"7\""));
    fixture.ok(&["session", "use", &first]);
    fixture.ok(&["session", "close", &first, "--reason", "SYNTHETIC closed"]);
    let story = format!(".chough/archive/sessions/{first}.md");
    assert_eq!(
        pr_line_of(&fixture, &story).as_deref(),
        Some("pr: \"https://github.com/CorvidLabs/jackdaw/pull/42\"")
    );
}

/// REQ-records-016 (#366), TC-records-reserved-words-are-written-quoted: a front matter
/// value a YAML reader takes as other than text, `null`, `~`, `true`, `false`, `yes`,
/// `no`, `on` or `off` in any letter case, is written double-quoted and reads back as
/// text: an objective, a `--no-native-change` reason and a blocker; a decision `true` or
/// `yes` reads back as text; a word that is none of them stays plain.
#[test]
fn yaml_reserved_words_are_written_quoted_and_read_back_as_text() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let file = fixture.session_file();
    let front_line = |key: &str| {
        fixture
            .read(&file)
            .lines()
            .find(|line| line.starts_with(&format!("{key}:")))
            .map(str::to_string)
            .unwrap_or_default()
    };
    for word in [
        "null", "Null", "NULL", "~", "true", "True", "TRUE", "false", "False", "yes", "Yes", "YES",
        "no", "No", "on", "On", "ON", "off", "Off", "OFF",
    ] {
        fixture.ok(&["checkpoint", "--objective", word]);
        assert_eq!(front_line("objective"), format!("objective: \"{word}\""));
        assert_eq!(fixture.checkpoint()["objective"], word);
    }
    fixture.ok(&["checkpoint", "--objective", "maybe"]);
    assert_eq!(front_line("objective"), "objective: maybe");

    fixture.ok(&["checkpoint", "--no-native-change", "null"]);
    assert_eq!(front_line("no_native_change"), "no_native_change: \"null\"");
    assert_eq!(fixture.checkpoint()["no_native_change"], "null");
    fixture.ok(&["checkpoint", "--blocker", "Off"]);
    assert!(
        front_line("blockers").contains("\"Off\""),
        "{}",
        front_line("blockers")
    );
    assert!(
        fixture.checkpoint()["blockers"]
            .as_array()
            .unwrap()
            .iter()
            .any(|blocker| blocker == "Off")
    );
    fixture.ok(&["checkpoint", "--decision", "true"]);
    fixture.ok(&["checkpoint", "--decision", "yes"]);
    let decisions = fixture.checkpoint()["decisions"].clone();
    for word in ["true", "yes"] {
        assert!(
            decisions
                .as_array()
                .unwrap()
                .iter()
                .any(|decision| decision == word),
            "{decisions}"
        );
    }
}

/// REQ-records-016, TC-records-flags-stay-booleans: the two flags Chough writes as YAML
/// booleans, an inbox entry's `recorded` and a validation run's `validation_run`, stay
/// plain, and an inbox entry written by 0.6.1 with `recorded: true`, or quoted by a hand
/// edit as `recorded: "true"`, both still read.
#[test]
fn flags_stay_booleans_and_either_form_still_reads() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let session = fixture.active_id();
    for (name, recorded) in [
        ("2026-11-19-plain-flag", "true"),
        ("2026-11-19-quoted-flag", "\"true\""),
    ] {
        fixture.write(
            &format!(".chough/sessions/{session}/agent/inbox/{name}.md"),
            &format!(
                "---\nkind: question\nid: {name}\nsession: {session}\nfrom: SYNTHETIC-agent\nvalue: SYNTHETIC {name}\nrecorded: {recorded}\nadded: 2026-11-19T00:00:00Z\n---\nSYNTHETIC {name}\n"
            ),
        );
    }
    let listed = fixture.ok(&["inbox", "list"]).to_string();
    for name in ["2026-11-19-plain-flag", "2026-11-19-quoted-flag"] {
        assert!(listed.contains(name), "{name} did not read: {listed}");
    }
    fixture.ok(&["inbox", "fold"]);
    let questions = fixture.checkpoint()["open_questions"].to_string();
    for name in ["2026-11-19-plain-flag", "2026-11-19-quoted-flag"] {
        assert!(
            questions.contains(name),
            "{name} was not folded: {questions}"
        );
    }
}
