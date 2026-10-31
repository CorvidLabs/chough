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
