//! Chough never writes front matter its own reader refuses (REQ-records-001): a value
//! the subset cannot hold as written, such as a block list item that starts with `-`
//! or a tab, is stored in a form the subset reads back, so one accepted write never
//! leaves `chough status` or `chough next` failing.

mod common;

use common::Fixture;

/// A command-bound case whose argv is long enough for a block list, with a `-c` item.
const CASES: &str = r#"profile = "chough-testing/1"
module = "cli"

[[case]]
id = "TC-cli-0001"
title = "Probe"
status = "active"
track = "functional"
method = "automated"
criteria = ["LIFE-1"]
requirements = ["REQ-cli-001"]
environments = ["local"]
artifacts = ["test-log"]
expected = ["Probe prints never-appears"]

[case.bind]
adapter = "command"
argv = ["sh", "-c", "echo SYNTHETIC-out-line; echo \"SYNTHETIC error line for the record\" >&2; echo plain-err >&2; exit 3"]
expect_output = ["never-appears"]
"#;

/// `status` and `next` both read every record and exit 0.
fn reads_back(fixture: &Fixture, after: &str) {
    for arguments in [vec!["status"], vec!["next", "--environment", "local"]] {
        let output = fixture.run(&arguments);
        assert!(
            output.status.success(),
            "after {after}, {arguments:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// REQ-records-001, TC-records-hyphen-led-block-items-read-back (R4): a case run of
/// `sh -c "<quoted text>"` writes its command as a block list, and the `-c` item reads
/// back as the scalar `-c`, so `status` and `next` still read the project.
#[test]
fn a_block_list_item_starting_with_a_hyphen_reads_back() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".chough/cases/cli.cases.toml", CASES);
    fixture.init();
    fixture.commit("fixture");
    let output = fixture.run(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
    assert!(
        !output.status.success(),
        "the probe was meant to fail: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let session = fixture.active_id();
    let entries = std::fs::read_dir(
        fixture
            .root()
            .join(format!(".chough/sessions/{session}/evidence")),
    )
    .unwrap()
    .flatten()
    .map(|entry| std::fs::read_to_string(entry.path()).unwrap())
    .collect::<Vec<_>>();
    assert!(
        entries
            .iter()
            .any(|entry| entry.contains("command:\n  - sh\n  - \"-c\"\n")),
        "no block-list command with a quoted -c: {entries:#?}"
    );
    reads_back(&fixture, "a failing sh -c case run");
}

/// REQ-records-001, TC-records-tabs-in-values-read-back (R5): a tab in an objective,
/// a next action, a claim, a limitation or a bug title is stored as a space, so each
/// write leaves the session, its evidence and the bug readable.
#[test]
fn a_tab_in_a_value_is_stored_as_a_space_and_reads_back() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    fixture.ok(&["checkpoint", "--objective", "SYNTHETIC obj\tx"]);
    reads_back(&fixture, "an objective with a tab");
    fixture.ok(&["checkpoint", "--next-action", "SYNTHETIC next\taction"]);
    reads_back(&fixture, "a next action with a tab");
    let session = fixture.read(&fixture.session_file());
    assert!(session.contains("SYNTHETIC obj x"), "{session}");
    assert!(session.contains("SYNTHETIC next action"), "{session}");
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
        "SYNTHETIC a\tb",
        "--limitation",
        "tab\tinside",
    ]);
    reads_back(&fixture, "a claim and a limitation with a tab");
    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "SYNTHETIC tab\ttitle",
        "--symptom",
        "SYNTHETIC symptom",
        "--trigger",
        "SYNTHETIC trigger",
        "--expected",
        "SYNTHETIC expected",
        "--actual",
        "SYNTHETIC actual",
        "--environment",
        "local",
    ]);
    reads_back(&fixture, "a bug title with a tab");
    let listed = fixture.ok(&["bug", "list"]);
    assert!(
        listed.to_string().contains("SYNTHETIC tab title"),
        "{listed:#}"
    );
}
