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

/// REQ-records-001 and REQ-records-016, TC-records-tabs-in-values-read-back (R5, #366): a
/// tab given in an objective, a next action, a claim, a limitation or a bug title is
/// refused as `refused:control-character` naming U+0009, with nothing written, so each
/// record stays readable; a tab is a control character like any other.
#[test]
fn a_tab_in_a_given_value_is_refused_and_the_records_stay_readable() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let file = fixture.session_file();
    let before = fixture.read(&file);
    let evidence = |summary: &'static str, limitation: &'static str| {
        vec![
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
            "--limitation",
            limitation,
        ]
    };
    let writes: Vec<Vec<&str>> = vec![
        vec!["checkpoint", "--objective", "SYNTHETIC obj\tx"],
        vec!["checkpoint", "--next-action", "SYNTHETIC next\taction"],
        evidence("SYNTHETIC a\tb", "none"),
        evidence("SYNTHETIC claim", "tab\tinside"),
        vec![
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
        ],
    ];
    for args in &writes {
        for redact in [false, true] {
            let mut all: Vec<&str> = if redact { vec!["--redact"] } else { vec![] };
            all.extend(args.iter().copied());
            let refused = fixture.fails(&all);
            assert!(
                refused.contains("refused:control-character") && refused.contains("U+0009"),
                "{all:?}: {refused}"
            );
            reads_back(&fixture, "a refused write holding a tab");
        }
    }
    assert_eq!(
        fixture.read(&file),
        before,
        "a refused write changed the session"
    );
    assert!(
        !fixture.exists(".chough/bugs"),
        "a refused bug add wrote a bug"
    );
}
