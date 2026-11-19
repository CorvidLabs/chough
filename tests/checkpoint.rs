//! `chough checkpoint` field setters that have no other home.

mod common;

use common::Fixture;

/// REQ-cli-007 (#282), TC-cli-checkpoint-pr-sets-the-pull-request: `checkpoint --pr` sets
/// the session's pull request, a number or a URL, which another process reads back; an
/// empty value clears it. Like `--objective` and `--title` it adds no Timeline entry. A
/// line break, or a local path the leak guard catches, is refused with the session file
/// byte-identical.
#[test]
fn checkpoint_pr_sets_and_clears_the_pull_request() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let file = fixture.session_file();
    let timeline = |text: &str| text.split_once("## Timeline").unwrap().1.to_string();
    // The session's `pr` front matter value, unquoted, as a fresh process reads it.
    let pr = || {
        let text = fixture.read(&file);
        let line = text
            .lines()
            .find(|line| line.starts_with("pr:"))
            .unwrap_or_else(|| panic!("no pr line in {text}"))
            .to_string();
        line["pr:".len()..].trim().trim_matches('"').to_string()
    };
    let before = fixture.read(&file);

    fixture.ok(&["checkpoint", "--pr", "150"]);
    assert_eq!(pr(), "150");
    assert_eq!(
        timeline(&fixture.read(&file)),
        timeline(&before),
        "--pr added a Timeline entry"
    );

    let url = "https://github.com/CorvidLabs/jackdaw/pull/150";
    fixture.ok(&["checkpoint", "--pr", url]);
    assert_eq!(pr(), url);

    let set = fixture.read(&file);
    let refused = fixture.fails(&["checkpoint", "--pr", "150\n151"]);
    assert!(refused.contains("--pr is one line"), "{refused}");
    assert_eq!(fixture.read(&file), set, "a refused --pr changed the file");
    let path = concat!("/Users/", "someone", "/notes/pr.txt");
    let leaked = fixture.fails(&["checkpoint", "--pr", path]);
    assert!(leaked.contains("refused:leak"), "{leaked}");
    assert_eq!(fixture.read(&file), set, "a refused --pr changed the file");

    fixture.ok(&["checkpoint", "--pr", ""]);
    assert_eq!(pr(), "");
}
