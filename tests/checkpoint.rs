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
        pr_line().is_some_and(|line| line.contains("<redacted:token>")),
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
        assert!(!shown.contains("PR:"), "{view:?} shows an empty PR: {shown}");
        assert!(
            !shown.contains("\"pr\": \"\"") && !shown.contains("\"pr\":\"\""),
            "{view:?} shows an empty pr: {shown}"
        );
    }
    let url = "https://github.com/CorvidLabs/jackdaw/pull/4817";
    fixture.ok(&["checkpoint", "--pr", url]);
    for view in views {
        let shown = fixture.ok_text(view);
        assert!(shown.contains(url), "{view:?} does not show the PR: {shown}");
    }
    let next = fixture.ok_text(&["next"]);
    assert!(next.lines().any(|line| line == format!("PR: {url}")), "{next}");
}
