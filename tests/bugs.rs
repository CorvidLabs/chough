//! Local-first bug backlog (REQ-cli-012).

mod common;

use common::Fixture;
use serde_json::{Value, json};

fn observe(fixture: &Fixture, id: &str, outcome: &str) {
    fixture.ok(&[
        "evidence",
        "add",
        id,
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "product-test",
        "--outcome",
        outcome,
        "--environment",
        "test-local",
        "--summary",
        "Observed the shipping threshold check",
    ]);
}

fn add_bug(fixture: &Fixture, extra: &[&str]) -> std::process::Output {
    let mut args = vec![
        "bug",
        "add",
        "--title",
        "Shipping is charged at exactly the free threshold",
        "--symptom",
        "Order of exactly 50.00 is charged shipping",
        "--trigger",
        "Checkout total equal to the free shipping threshold",
        "--expected",
        "Free shipping at 50.00",
        "--actual",
        "Shipping fee of 4.99 applied",
        "--environment",
        "test-local",
        "--requirement",
        "REQ-cli-001",
    ];
    args.extend(extra);
    fixture.run(&args)
}

fn json(output: &std::process::Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn unverified_out_of_scope_bugs_stay_visible_without_changing_scope() {
    let fixture = Fixture::new();
    fixture.init();
    let before = fixture.session();
    let added = json(&add_bug(&fixture, &["--scope", "out-of-scope"]));
    assert_eq!(added["id"], "BUG-0001");
    assert_eq!(added["verification"], "unverified");
    assert_eq!(added["published"], false);
    assert_eq!(fixture.session(), before);
    let open = fixture.ok(&["bug", "list"]);
    assert_eq!(open["bugs"][0]["scope"], "out-of-scope");
    let context = fixture.ok(&["context", "--json"]);
    assert!(context["sections"].to_string().contains("BUG-0001"));
    let report = fixture.run(&["bug", "report", "BUG-0001"]);
    let text = String::from_utf8_lossy(&report.stdout);
    assert!(text.contains("Not published"));
    assert!(text.contains("verification: unverified"));
    assert!(text.contains("# Shipping is charged at exactly the free threshold"));
}

#[cfg(unix)]
#[test]
fn a_reproduced_bug_is_deduplicated_and_keeps_added_evidence() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    observe(&fixture, "repro-1", "failed");
    let added = json(&add_bug(
        &fixture,
        &[
            "--observation",
            "repro-1",
            "--classification",
            "product-defect",
            "--step",
            "Check out with a 50.00 cart",
        ],
    ));
    assert_eq!(added["verification"], "reproduced");

    // A second report of the same symptom is refused and pointed at the existing bug.
    let duplicate = add_bug(&fixture, &[]);
    assert!(!duplicate.status.success());
    let error = String::from_utf8_lossy(&duplicate.stderr);
    assert!(error.contains("similar bugs exist") && error.contains("BUG-0001"));

    observe(&fixture, "repro-2", "failed");
    let updated = fixture.ok(&[
        "bug",
        "update",
        "BUG-0001",
        "--observation",
        "repro-2",
        "--note",
        "Reported again from the mobile checkout",
    ]);
    assert_eq!(
        updated["bug"]["observations"],
        json!(["repro-1", "repro-2"])
    );
    assert_eq!(
        updated["bug"]["notes"][0]["observations"],
        json!(["repro-2"])
    );

    let distinct = json(&add_bug(
        &fixture,
        &["--distinct", "Different code path: gift cards"],
    ));
    assert_eq!(distinct["id"], "BUG-0002");
    let search = fixture.ok(&["bug", "search", "exactly 50.00 shipping threshold"]);
    assert_eq!(search["matches"][0]["id"], "BUG-0001");

    assert!(
        fixture
            .fails(&["bug", "update", "BUG-0001", "--status", "verified"])
            .contains("passed observation")
    );
    observe(&fixture, "fixed-1", "passed");
    // The fix must be observed strictly after the latest failure (REQ-cli-012); both
    // records can land in one second, so the later time is written directly.
    fixture.observed_at("fixed-1", fixture.observed_time("repro-2") + 1);
    let verified = fixture.ok(&[
        "bug",
        "update",
        "BUG-0001",
        "--status",
        "verified",
        "--observation",
        "fixed-1",
    ]);
    assert_eq!(verified["verification"], "reproduced-and-later-passed");
    assert_eq!(
        fixture.ok(&["bug", "list"])["bugs"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

#[cfg(unix)]
#[test]
fn regression_needs_a_passed_observation_at_an_earlier_ancestor_revision() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("first");
    observe(&fixture, "worked-before", "passed");
    fixture.write("src/input.txt", "second version\n");
    fixture.commit("second");
    observe(&fixture, "fails-now", "failed");
    observe(&fixture, "passes-now", "passed");
    let regression = [
        "--classification",
        "regression",
        "--observation",
        "fails-now",
    ];
    let mut missing = regression.to_vec();
    missing.extend(["--distinct", "first report"]);
    assert!(
        !add_bug(&fixture, &missing).status.success(),
        "a regression without a baseline must be refused"
    );
    for (baseline, message) in [
        ("fails-now", "did not pass"),
        ("passes-now", "earlier revision"),
    ] {
        let mut args = regression.to_vec();
        args.extend(["--regression-baseline", baseline]);
        let output = add_bug(&fixture, &args);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(message));
    }
    let mut args = regression.to_vec();
    args.extend(["--regression-baseline", "worked-before"]);
    assert_eq!(json(&add_bug(&fixture, &args))["id"], "BUG-0001");
}

#[test]
fn an_open_blocking_bug_sends_work_back_to_build() {
    let fixture = Fixture::new();
    fixture.init();
    for _ in 0..4 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(fixture.session()["stage"]["current"], "test");
    json(&add_bug(&fixture, &["--scope", "blocking"]));
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:blocking-bugs")
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["blocking_bugs"], json!(["BUG-0001"]));
    assert!(
        next["next_valid_action"]
            .as_str()
            .unwrap()
            .contains("go back to build")
    );
    fixture.ok(&["stage", "back", "--to", "build", "--reason", "Fix BUG-0001"]);
    fixture.ok(&[
        "bug",
        "update",
        "BUG-0001",
        "--scope",
        "out-of-scope",
        "--note",
        "The human moved it to a follow-up change",
    ]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(fixture.session()["stage"]["current"], "accept");
}

/// A committed fixture with the given observations at the given times, linked to one bug.
fn ordered_bug(observations: &[(&str, &str, u64)]) -> Fixture {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    let mut extra = vec!["--classification", "product-defect"];
    for (id, outcome, seconds) in observations {
        observe(&fixture, id, outcome);
        fixture.observed_at(id, *seconds);
        extra.extend(["--observation", id]);
    }
    json(&add_bug(&fixture, &extra));
    fixture
}

/// The verification label on every surface that shows one.
fn labels(fixture: &Fixture) -> Vec<String> {
    let show = fixture.ok(&["bug", "show", "BUG-0001"]);
    let list = fixture.ok(&["bug", "list"]);
    let status = fixture.ok(&["status", "--json"]);
    let next = fixture.ok(&["next", "--json"]);
    let report = String::from_utf8(fixture.run(&["bug", "report", "BUG-0001"]).stdout).unwrap();
    let reported = report
        .lines()
        .find_map(|line| line.split_once("verification: "))
        .map(|(_, label)| label.to_string())
        .unwrap();
    vec![
        show["verification"].as_str().unwrap().to_string(),
        list["bugs"][0]["verification"]
            .as_str()
            .unwrap()
            .to_string(),
        status["open_bugs"][0]["verification"]
            .as_str()
            .unwrap()
            .to_string(),
        next["open_bugs"][0]["verification"]
            .as_str()
            .unwrap()
            .to_string(),
        reported,
    ]
}

#[test]
fn a_pass_observed_before_the_latest_failure_does_not_verify_a_bug() {
    let fixture = ordered_bug(&[
        ("old-pass", "passed", 1_793_000_000),
        ("new-failure", "failed", 1_793_000_100),
    ]);
    assert_eq!(labels(&fixture), vec!["reproduced"; 5]);
    let error = fixture.fails(&["bug", "update", "BUG-0001", "--status", "verified"]);
    assert!(
        error.contains("new-failure") && error.contains("passed observation"),
        "{error}"
    );
    assert_eq!(
        fixture.ok(&["bug", "show", "BUG-0001"])["bug"]["status"],
        "open"
    );
}

#[test]
fn a_pass_observed_after_the_latest_failure_verifies_a_bug() {
    let fixture = ordered_bug(&[
        ("first-failure", "failed", 1_793_000_000),
        ("between-pass", "passed", 1_793_000_050),
        ("second-failure", "failed", 1_793_000_100),
    ]);
    // A pass between two failures is older than the latest failure.
    assert_eq!(labels(&fixture), vec!["reproduced"; 5]);
    observe(&fixture, "later-pass", "passed");
    fixture.observed_at("later-pass", 1_793_000_101);
    let verified = fixture.ok(&[
        "bug",
        "update",
        "BUG-0001",
        "--status",
        "verified",
        "--observation",
        "later-pass",
    ]);
    assert_eq!(verified["verification"], "reproduced-and-later-passed");
    assert_eq!(verified["bug"]["status"], "verified");
    let show = fixture.ok(&["bug", "show", "BUG-0001"]);
    assert_eq!(show["verification"], "reproduced-and-later-passed");
    assert_eq!(
        fixture.ok(&["bug", "list", "--all"])["bugs"][0]["verification"],
        "reproduced-and-later-passed"
    );
}

#[test]
fn a_pass_in_the_same_second_as_the_failure_does_not_verify_a_bug() {
    let fixture = ordered_bug(&[
        ("repro-same-second", "failed", 1_793_000_000),
        ("fix-same-second", "passed", 1_793_000_000),
    ]);
    assert_eq!(labels(&fixture), vec!["reproduced"; 5]);
    let error = fixture.fails(&["bug", "update", "BUG-0001", "--status", "verified"]);
    assert!(
        error.contains("repro-same-second") && error.contains("passed observation"),
        "{error}"
    );
}

#[test]
fn a_bug_without_a_linked_failure_cannot_be_verified() {
    let fixture = ordered_bug(&[("only-pass", "passed", 1_793_000_000)]);
    assert_eq!(labels(&fixture), vec!["unverified"; 5]);
    let error = fixture.fails(&["bug", "update", "BUG-0001", "--status", "verified"]);
    assert!(error.contains("failed observation"), "{error}");
    assert_eq!(
        fixture.ok(&["bug", "show", "BUG-0001"])["bug"]["status"],
        "open"
    );
}

#[test]
fn a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag() {
    let fixture = Fixture::new();
    fixture.init();
    let refused = |output: std::process::Output, hint: &str| {
        // The refusal keeps the validation exit code; clap's usage errors exit 2.
        assert_eq!(
            output.status.code(),
            Some(1),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let error = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(error.contains(hint), "expected {hint:?} in {error}");
    };
    refused(
        add_bug(&fixture, &["--classification", "blocking"]),
        "`blocking` is a scope: use --scope blocking",
    );
    refused(
        add_bug(&fixture, &["--scope", "product-defect"]),
        "`product-defect` is a classification: use --classification product-defect",
    );
    assert!(!fixture.exists(".chough/bugs/BUG-0001.json"));

    json(&add_bug(&fixture, &[]));
    let before = fixture.read(".chough/bugs/BUG-0001.json");
    let update = |flag: &str, value: &str| fixture.run(&["bug", "update", "BUG-0001", flag, value]);
    refused(
        update("--classification", "out-of-scope"),
        "`out-of-scope` is a scope: use --scope out-of-scope",
    );
    refused(
        update("--scope", "regression"),
        "`regression` is a classification: use --classification regression",
    );
    assert_eq!(fixture.read(".chough/bugs/BUG-0001.json"), before);

    // A value that is neither keeps the plain refusal.
    let output = update("--scope", "urgent");
    assert_eq!(output.status.code(), Some(1));
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("scope must be one of"), "{error}");
    assert!(!error.contains("use --classification"), "{error}");

    for command in ["add", "update"] {
        let help = fixture.run(&["bug", command, "--help"]);
        let text = String::from_utf8_lossy(&help.stdout);
        for value in [
            "blocking",
            "in-scope",
            "out-of-scope",
            "unclassified",
            "product-defect",
            "regression",
            "environment",
            "requirement-gap",
            "evidence-gap",
        ] {
            assert!(
                text.contains(value),
                "bug {command} --help lacks {value}: {text}"
            );
        }
    }
}
