//! Local-first bug backlog (REQ-cli-012).

mod common;

use common::Fixture;
use serde_json::{Value, json};

fn observe(fixture: &Fixture, id: &str, outcome: &str) {
    let allocated = format!("OBS-{id}-000000000000000000000001");
    fixture.ok(&[
        "evidence",
        "add",
        &allocated,
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
    let allocated_name = format!(".chough/evidence/{allocated}.json");
    let mut record: Value = serde_json::from_str(&fixture.read(&allocated_name)).unwrap();
    record["id"] = json!(id);
    fixture.write(
        &format!(".chough/evidence/{id}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    std::fs::remove_file(fixture.root().join(allocated_name)).unwrap();
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
    let id = added["id"].as_str().unwrap();
    assert_new_bug_id(id);
    assert_eq!(added["verification"], "unverified");
    assert_eq!(added["published"], false);
    assert_eq!(fixture.session(), before);
    let open = fixture.ok(&["bug", "list"]);
    assert_eq!(open["bugs"][0]["scope"], "out-of-scope");
    let context = fixture.ok(&["context", "--json"]);
    assert!(context["sections"].to_string().contains(id));
    let report = fixture.run(&["bug", "report", id]);
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
    let id = added["id"].as_str().unwrap().to_string();
    assert_new_bug_id(&id);
    assert_eq!(added["verification"], "reproduced");

    // A second report of the same symptom is refused and pointed at the existing bug.
    let duplicate = add_bug(&fixture, &[]);
    assert!(!duplicate.status.success());
    let error = String::from_utf8_lossy(&duplicate.stderr);
    assert!(error.contains("similar bugs exist") && error.contains(&id));

    observe(&fixture, "repro-2", "failed");
    let updated = fixture.ok(&[
        "bug",
        "update",
        &id,
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
    let distinct_id = distinct["id"].as_str().unwrap().to_string();
    assert_new_bug_id(&distinct_id);
    assert_ne!(distinct_id, id);
    let search = fixture.ok(&["bug", "search", "exactly 50.00 shipping threshold"]);
    assert!(
        search["matches"]
            .as_array()
            .unwrap()
            .iter()
            .any(|bug| bug["id"] == id)
    );

    assert!(
        fixture
            .fails(&["bug", "update", &id, "--status", "verified"])
            .contains("passed observation")
    );
    observe(&fixture, "fixed-1", "passed");
    // The fix must be observed strictly after the latest failure (REQ-cli-012); both
    // records can land in one second, so the later time is written directly.
    fixture.observed_at("fixed-1", fixture.observed_time("repro-2") + 1);
    let verified = fixture.ok(&[
        "bug",
        "update",
        &id,
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
    let added = json(&add_bug(&fixture, &args));
    assert_new_bug_id(added["id"].as_str().unwrap());
}

#[test]
fn an_open_blocking_bug_sends_work_back_to_build() {
    let fixture = Fixture::new();
    fixture.init();
    for _ in 0..4 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(fixture.session()["stage"]["current"], "test");
    let added = json(&add_bug(&fixture, &["--scope", "blocking"]));
    let id = added["id"].as_str().unwrap().to_string();
    assert_new_bug_id(&id);
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:blocking-bugs")
    );
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["blocking_bugs"], json!([id.clone()]));
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["blocking_bugs"]["items"], json!([id.clone()]));
    assert_eq!(next["blocking_bugs"]["total"], 1);
    assert_eq!(
        next["next_valid_action"]["action"],
        status["next_valid_action"]
    );
    assert!(
        next["next_valid_action"]["action"]
            .as_str()
            .unwrap()
            .contains("go back to build")
    );
    let reason = format!("Fix {id}");
    fixture.ok(&["stage", "back", "--to", "build", "--reason", &reason]);
    fixture.ok(&[
        "bug",
        "update",
        &id,
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
    let added = json(&add_bug(&fixture, &extra));
    assert_new_bug_id(added["id"].as_str().unwrap());
    fixture
}

fn only_bug_id(fixture: &Fixture) -> String {
    let bugs = fixture.ok(&["bug", "list", "--all"]);
    assert_eq!(bugs["bugs"].as_array().unwrap().len(), 1);
    let id = bugs["bugs"][0]["id"].as_str().unwrap().to_string();
    assert_new_bug_id(&id);
    id
}

/// The verification label on every surface that shows one. The focused `next` view
/// counts open bugs and names blocking ones, but shows no verification label.
fn labels(fixture: &Fixture) -> Vec<String> {
    let id = only_bug_id(fixture);
    let show = fixture.ok(&["bug", "show", &id]);
    let list = fixture.ok(&["bug", "list"]);
    let status = fixture.ok(&["status", "--json"]);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["other_open_bugs"].as_u64().unwrap()
            + next["blocking_bugs"]["total"].as_u64().unwrap(),
        status["open_bugs"].as_array().unwrap().len() as u64,
        "{next}"
    );
    assert!(next.get("open_bugs").is_none(), "{next}");
    let report = String::from_utf8(fixture.run(&["bug", "report", &id]).stdout).unwrap();
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
        reported,
    ]
}

#[test]
fn a_pass_observed_before_the_latest_failure_does_not_verify_a_bug() {
    let fixture = ordered_bug(&[
        ("old-pass", "passed", 1_793_000_000),
        ("new-failure", "failed", 1_793_000_100),
    ]);
    assert_eq!(labels(&fixture), vec!["reproduced"; 4]);
    let id = only_bug_id(&fixture);
    let error = fixture.fails(&["bug", "update", &id, "--status", "verified"]);
    assert!(
        error.contains("new-failure") && error.contains("passed observation"),
        "{error}"
    );
    assert_eq!(fixture.ok(&["bug", "show", &id])["bug"]["status"], "open");
}

#[test]
fn a_pass_observed_after_the_latest_failure_verifies_a_bug() {
    let fixture = ordered_bug(&[
        ("first-failure", "failed", 1_793_000_000),
        ("between-pass", "passed", 1_793_000_050),
        ("second-failure", "failed", 1_793_000_100),
    ]);
    // A pass between two failures is older than the latest failure.
    assert_eq!(labels(&fixture), vec!["reproduced"; 4]);
    let id = only_bug_id(&fixture);
    observe(&fixture, "later-pass", "passed");
    fixture.observed_at("later-pass", 1_793_000_101);
    let verified = fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "verified",
        "--observation",
        "later-pass",
    ]);
    assert_eq!(verified["verification"], "reproduced-and-later-passed");
    assert_eq!(verified["bug"]["status"], "verified");
    let show = fixture.ok(&["bug", "show", &id]);
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
    assert_eq!(labels(&fixture), vec!["reproduced"; 4]);
    let id = only_bug_id(&fixture);
    let error = fixture.fails(&["bug", "update", &id, "--status", "verified"]);
    assert!(
        error.contains("repro-same-second") && error.contains("passed observation"),
        "{error}"
    );
}

#[test]
fn a_bug_without_a_linked_failure_cannot_be_verified() {
    let fixture = ordered_bug(&[("only-pass", "passed", 1_793_000_000)]);
    assert_eq!(labels(&fixture), vec!["unverified"; 4]);
    let id = only_bug_id(&fixture);
    let error = fixture.fails(&["bug", "update", &id, "--status", "verified"]);
    assert!(error.contains("failed observation"), "{error}");
    assert_eq!(fixture.ok(&["bug", "show", &id])["bug"]["status"], "open");
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
    assert!(
        fixture.ok(&["bug", "list", "--all"])["bugs"]
            .as_array()
            .unwrap()
            .is_empty()
    );

    let added = json(&add_bug(&fixture, &[]));
    let id = added["id"].as_str().unwrap().to_string();
    assert_new_bug_id(&id);
    let path = format!(".chough/bugs/{id}.json");
    let before = fixture.read(&path);
    let update = |flag: &str, value: &str| fixture.run(&["bug", "update", &id, flag, value]);
    refused(
        update("--classification", "out-of-scope"),
        "`out-of-scope` is a scope: use --scope out-of-scope",
    );
    refused(
        update("--scope", "regression"),
        "`regression` is a classification: use --classification regression",
    );
    assert_eq!(fixture.read(&path), before);

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

fn seed_legacy_high_water(fixture: &Fixture) {
    let legacy = json!({
        "schema_version": 1,
        "id": "BUG-0019",
        "title": "Legacy allocator sentinel",
        "symptom": "Seeded legacy record remains readable",
        "trigger": "Load the independent root",
        "expected": "The legacy identity remains usable",
        "actual": "The legacy identity is present",
        "criteria": ["FLOW-8", "LIFE-5"],
        "requirements": ["REQ-cli-012"],
        "cases": ["TC-cli-0125"],
        "environment": "test-local",
        "revision": null,
        "steps": ["Read the seeded schema 1 record"],
        "fixture": null,
        "observations": [],
        "impact": "Compatibility fixture",
        "classification": "product-defect",
        "scope": "in-scope",
        "status": "open",
        "regression_baseline": null,
        "duplicate_of": null,
        "related": [],
        "source": "legacy fixture",
        "notes": [],
        "created_at_unix_seconds": 1,
        "updated_at_unix_seconds": 1,
    });
    fixture.write(
        ".chough/bugs/BUG-0019.json",
        &(serde_json::to_string_pretty(&legacy).unwrap() + "\n"),
    );
}

fn manual_collision_root(title: &str, symptom: &str) -> (Fixture, String) {
    let fixture = Fixture::new();
    fixture.init();
    seed_legacy_high_water(&fixture);
    fixture.commit("seed legacy high water");
    let added = json(&fixture.run(&[
        "bug",
        "add",
        "--title",
        title,
        "--symptom",
        symptom,
        "--trigger",
        "Create a report in an independent root",
        "--expected",
        "A distinct bug identity",
        "--actual",
        "The allocator returned an identity",
        "--environment",
        "test-local",
        "--classification",
        "product-defect",
        "--source",
        "manual collision regression",
    ]));
    let id = added["id"].as_str().unwrap().to_string();
    (fixture, id)
}

fn automatic_collision_root(case_id: &str, missing_output: &str) -> (Fixture, String) {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.write(
        "specs/cli/requirements.md",
        "### REQ-cli-012\nLocal bug backlog.\n",
    );
    common::stand_in(
        &fixture.root().join("bin"),
        "failure-probe",
        "automatic capture failure",
    );
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"{case_id}\"\ntitle = \"Automatic collision probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"FLOW-8\"]\nrequirements = [\"REQ-cli-012\"]\nenvironments = [\"test-local\"]\nartifacts = [\"test-log\"]\nexpected = [\"The probe reports its expected output\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"failure-probe\"]\nexpect_output = [\"{missing_output}\"]\n"
        ),
    );
    fixture.init();
    seed_legacy_high_water(&fixture);
    fixture.commit("seed automatic collision fixture");
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Test authorization for one automatic capture",
    ]);
    let run = fixture.ok(&[
        "validate",
        "run",
        case_id,
        "--times",
        "1",
        "--environment",
        "test-local",
    ]);
    assert_eq!(run["run"]["failures"], 1, "{run}");
    let id = run["captured_locally"][0].as_str().unwrap().to_string();
    (fixture, id)
}

fn assert_new_bug_id(id: &str) {
    assert!(id.len() <= 100, "bug ID exceeds 100 bytes: {id}");
    let rest = id.strip_prefix("BUG-").expect("new bug ID prefix");
    let (slug, digest) = rest.rsplit_once('-').expect("new bug ID separator");
    assert!(!slug.is_empty() && slug.len() <= 71, "invalid slug: {id}");
    assert!(
        slug.bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
        "slug is not lowercase ASCII: {id}"
    );
    assert!(!slug.starts_with('-') && !slug.ends_with('-') && !slug.contains("--"));
    assert_eq!(digest.len(), 24, "digest length is not 24: {id}");
    assert!(
        digest
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "digest is not lowercase hex: {id}"
    );
}

#[test]
fn independent_roots_at_the_same_high_water_mark_get_distinct_ids_without_breaking_legacy_links() {
    let (manual_left, manual_left_id) = manual_collision_root(
        "Manual collision alpha",
        "The alpha manual report receives the next legacy number",
    );
    let (manual_right, manual_right_id) = manual_collision_root(
        "Manual collision beta",
        "The beta manual report receives the next legacy number",
    );
    let (automatic_left, automatic_left_id) =
        automatic_collision_root("TC-cli-9001", "alpha expected output is absent");
    let (automatic_right, automatic_right_id) =
        automatic_collision_root("TC-cli-9002", "beta expected output is absent");

    let manual_collision = manual_left_id == manual_right_id;
    let automatic_collision = automatic_left_id == automatic_right_id;
    println!(
        "legacy high-water BUG-0019: manual left {manual_left_id}, manual right {manual_right_id}, automatic left {automatic_left_id}, automatic right {automatic_right_id}"
    );
    assert!(
        !manual_collision && !automatic_collision,
        "independent roots at legacy high-water BUG-0019 collided: manual IDs {manual_left_id} and {manual_right_id}; automatic IDs {automatic_left_id} and {automatic_right_id}"
    );

    for id in [
        &manual_left_id,
        &manual_right_id,
        &automatic_left_id,
        &automatic_right_id,
    ] {
        assert_new_bug_id(id);
    }
    assert!(manual_left_id.starts_with("BUG-manual-collision-alpha-"));
    assert!(manual_right_id.starts_with("BUG-manual-collision-beta-"));
    for fixture in [
        &manual_left,
        &manual_right,
        &automatic_left,
        &automatic_right,
    ] {
        assert!(!fixture.exists(".chough/bugs/BUG-0020.json"));
        let legacy: Value =
            serde_json::from_str(&fixture.read(".chough/bugs/BUG-0019.json")).unwrap();
        assert_eq!(legacy["schema_version"], 1);
        assert_eq!(legacy["id"], "BUG-0019");
    }

    let shown = manual_left.ok(&["bug", "show", "BUG-0019"]);
    assert_eq!(shown["bug"]["id"], "BUG-0019");
    assert_eq!(shown["bug"]["schema_version"], 1);
    assert!(
        manual_left.ok(&["bug", "list", "--all"])["bugs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|bug| bug["id"] == "BUG-0019")
    );
    assert!(
        manual_left.ok(&["bug", "search", "legacy allocator sentinel"])["matches"]
            .as_array()
            .unwrap()
            .iter()
            .any(|bug| bug["id"] == "BUG-0019")
    );
    let report = manual_left.run(&["bug", "report", "BUG-0019"]);
    assert!(report.status.success());
    assert!(String::from_utf8_lossy(&report.stdout).contains("Legacy allocator sentinel"));
    let updated = manual_left.ok(&[
        "bug",
        "update",
        "BUG-0019",
        "--note",
        "Legacy content changed without renumbering",
        "--related",
        &manual_left_id,
    ]);
    assert_eq!(updated["bug"]["id"], "BUG-0019");
    assert_eq!(updated["bug"]["related"], json!([manual_left_id.clone()]));
    let related = manual_left.ok(&[
        "bug",
        "update",
        &manual_left_id,
        "--related",
        "BUG-0019",
        "--note",
        "New-form content changed without recomputing its identity",
    ]);
    assert_eq!(related["bug"]["id"], Value::from(manual_left_id.clone()));
    assert_eq!(related["bug"]["related"], json!(["BUG-0019"]));
    let legacy_duplicate = manual_left.ok(&[
        "bug",
        "update",
        "BUG-0019",
        "--duplicate-of",
        &manual_left_id,
    ]);
    assert_eq!(legacy_duplicate["bug"]["duplicate_of"], manual_left_id);
    let new_duplicate = manual_left.ok(&[
        "bug",
        "update",
        &manual_left_id,
        "--duplicate-of",
        "BUG-0019",
    ]);
    assert_eq!(new_duplicate["bug"]["id"], manual_left_id);
    assert_eq!(new_duplicate["bug"]["duplicate_of"], "BUG-0019");
    assert!(manual_left.exists(".chough/bugs/BUG-0019.json"));
    assert!(manual_left.exists(&format!(".chough/bugs/{manual_left_id}.json")));
}

#[test]
fn bug_text_values_may_start_with_two_hyphens() {
    let fixture = Fixture::new();
    fixture.init();
    let output = fixture.run(&[
        "bug",
        "add",
        "--title",
        "--name prints nothing",
        "--symptom",
        "--name exits 0 with no output",
        "--trigger",
        "chough --name",
        "--expected",
        "-- a usage error",
        "--actual",
        "--- nothing",
        "--environment",
        "test-local",
        "--step",
        "--run chough --name",
        "--requirement",
        "REQ-cli-001",
    ]);
    let added = json(&output);
    let id = added["id"].as_str().unwrap();
    let record: Value =
        serde_json::from_str(&fixture.read(&format!(".chough/bugs/{id}.json"))).unwrap();
    assert_eq!(record["title"], "--name prints nothing");
    assert_eq!(record["symptom"], "--name exits 0 with no output");
    assert_eq!(record["expected"], "-- a usage error");
    assert_eq!(record["actual"], "--- nothing");
    assert_eq!(record["steps"], json!(["--run chough --name"]));
}

/// Every option of `bug add` and `cases run` carries a description in `--help`.
#[test]
fn bug_add_and_cases_run_options_have_help_text() {
    let fixture = Fixture::new();
    for command in [
        &["bug", "add", "--help"][..],
        &["cases", "run", "--help"][..],
    ] {
        let output = fixture.run(command);
        assert!(output.status.success());
        let help = String::from_utf8(output.stdout).unwrap();
        let lines: Vec<&str> = help.lines().collect();
        let mut options = 0;
        for (index, line) in lines.iter().enumerate() {
            let trimmed = line.trim_start();
            if !trimmed.starts_with('-') && !trimmed.starts_with('<') {
                continue;
            }
            options += 1;
            // Clap puts the description after the flag on the same line, or on the
            // next, more indented line when the descriptions are long.
            let flag_end = trimmed.find("  ").map(|at| trimmed[at..].trim());
            let same_line = flag_end.is_some_and(|rest| !rest.is_empty() && !rest.starts_with('['));
            let next_line = lines.get(index + 1).is_some_and(|next| {
                let indent = next.len() - next.trim_start().len();
                indent > line.len() - trimmed.len()
                    && !next.trim_start().starts_with('-')
                    && !next.trim_start().starts_with('[')
            });
            assert!(
                same_line || next_line,
                "{command:?}: no help text for {trimmed:?} in:\n{help}"
            );
        }
        assert!(options >= 4, "{help}");
    }
}

/// DRIVE-12: `chough bug merge <base> <ours> <theirs>`, Git's merge driver for bug
/// records, merges two branches' edits of one bug; a real disagreement stays a conflict
/// for a human, with ours unchanged.
#[test]
fn two_edits_of_one_bug_merge_and_a_real_disagreement_stays_a_conflict() {
    let fixture = Fixture::new();
    fixture.init();
    let added = json(&add_bug(&fixture, &[]));
    let id = added["id"].as_str().unwrap().to_string();
    let original = fixture.read(&format!(".chough/bugs/{id}.json"));
    let base: Value = serde_json::from_str(&original).unwrap();
    let at = base["updated_at_unix_seconds"].as_u64().unwrap();
    let write = |name: &str, value: &Value| {
        fixture.write(
            &format!("merge/{name}.json"),
            &(serde_json::to_string_pretty(value).unwrap() + "\n"),
        );
        fixture.root().join(format!("merge/{name}.json"))
    };
    let mut ours = base.clone();
    ours["notes"]
        .as_array_mut()
        .unwrap()
        .push(json!({"at_unix_seconds": at + 10, "text": "Fixed on branch A", "observations": []}));
    ours["status"] = json!("verified");
    ours["updated_at_unix_seconds"] = json!(at + 10);
    let mut theirs = base.clone();
    theirs["notes"].as_array_mut().unwrap().push(json!({
        "at_unix_seconds": at + 20,
        "text": "Seen again on branch B",
        "observations": ["OBS-seen-again-000000000000000000000001"],
    }));
    theirs["observations"] = json!(["OBS-seen-again-000000000000000000000001"]);
    theirs["updated_at_unix_seconds"] = json!(at + 20);
    let (b, o, t) = (
        write("base", &base),
        write("ours", &ours),
        write("theirs", &theirs),
    );
    let output = fixture.run(&[
        "bug",
        "merge",
        b.to_str().unwrap(),
        o.to_str().unwrap(),
        t.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = fixture.read("merge/ours.json");
    let merged: Value = serde_json::from_str(&text).unwrap();
    let notes: Vec<&str> = merged["notes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|note| note["text"].as_str().unwrap())
        .collect();
    assert_eq!(
        &notes[notes.len() - 2..],
        ["Fixed on branch A", "Seen again on branch B"]
    );
    assert_eq!(
        merged["observations"],
        json!(["OBS-seen-again-000000000000000000000001"])
    );
    assert_eq!(merged["status"], "verified");
    assert_eq!(merged["updated_at_unix_seconds"], json!(at + 20));
    // Written the way bug commands write a record: their field order, pretty, one newline.
    let fields = |text: &str| -> Vec<String> {
        text.lines()
            .filter(|line| line.starts_with("  \"") && !line.starts_with("   "))
            .map(|line| line.split('"').nth(1).unwrap().to_string())
            .collect()
    };
    assert_eq!(fields(&text), fields(&original));
    assert!(text.ends_with("}\n") && !text.ends_with("\n\n"));

    // Two different status changes are a human's call.
    let mut ours = base.clone();
    ours["status"] = json!("investigating");
    let mut theirs = base.clone();
    theirs["status"] = json!("deferred");
    let (b, o, t) = (
        write("base", &base),
        write("ours", &ours),
        write("theirs", &theirs),
    );
    let before = fixture.read("merge/ours.json");
    let output = fixture.run(&[
        "bug",
        "merge",
        b.to_str().unwrap(),
        o.to_str().unwrap(),
        t.to_str().unwrap(),
    ]);
    assert!(!output.status.success(), "a status disagreement merged");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("status"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fixture.read("merge/ours.json"), before);
}

/// The stored record of bug `id`.
fn bug_record(fixture: &Fixture, id: &str) -> Value {
    serde_json::from_str(&fixture.read(&format!(".chough/bugs/{id}.json"))).unwrap()
}

/// Overwrite fields of bug `id`'s stored record, as a hand edit would.
fn set_bug_fields(fixture: &Fixture, id: &str, fields: &[(&str, Value)]) {
    let mut record = bug_record(fixture, id);
    for (field, value) in fields {
        record[*field] = value.clone();
    }
    fixture.write(
        &format!(".chough/bugs/{id}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
}

/// A second bug with no words in common with `add_bug`'s, and no shared IDs.
fn add_other_bug(fixture: &Fixture) -> String {
    json(&fixture.run(&[
        "bug",
        "add",
        "--title",
        "Coupon codes expire one day early",
        "--symptom",
        "Coupon rejected one day before expiry",
        "--trigger",
        "Applying a coupon near midnight",
        "--expected",
        "Accepted until expiry",
        "--actual",
        "Rejected",
        "--environment",
        "test-local",
    ]))["id"]
        .as_str()
        .unwrap()
        .to_string()
}

/// REQ-cli-054 (#153, audit Bugs row): add and update accept each of the six
/// classifications, three scopes and seven statuses, and refuse any other value with the
/// allowed list, leaving the record byte-identical. A regression needs its baseline (see
/// regression_needs_a_passed_observation_at_an_earlier_ancestor_revision) and verified its
/// observations (see the verified tests), so both are set by hand here.
#[test]
fn every_classification_scope_and_status_is_accepted_and_others_are_refused() {
    let fixture = Fixture::new();
    fixture.init();
    let added = json(&add_bug(
        &fixture,
        &["--classification", "environment", "--scope", "out-of-scope"],
    ));
    let id = added["id"].as_str().unwrap().to_string();
    let record = bug_record(&fixture, &id);
    assert_eq!(
        (
            &record["classification"],
            &record["scope"],
            &record["status"]
        ),
        (
            &json!("environment"),
            &json!("out-of-scope"),
            &json!("open")
        )
    );
    for (flag, values) in [
        (
            "--classification",
            &[
                "unclassified",
                "product-defect",
                "environment",
                "requirement-gap",
                "evidence-gap",
            ][..],
        ),
        ("--scope", &["blocking", "in-scope", "out-of-scope"][..]),
        (
            "--status",
            &[
                "open",
                "investigating",
                "fix-pending-verification",
                "deferred",
                "duplicate",
                "wont-fix",
            ][..],
        ),
    ] {
        for value in values {
            let updated = json(&fixture.run(&["bug", "update", &id, flag, value]));
            assert_eq!(updated["bug"][&flag[2..]], *value, "{flag} {value}");
        }
    }
    for (field, value) in [("classification", "regression"), ("status", "verified")] {
        set_bug_fields(&fixture, &id, &[(field, json!(value))]);
        assert_eq!(bug_record(&fixture, &id)[field], value);
        assert!(fixture.run(&["bug", "show", &id]).status.success());
        set_bug_fields(
            &fixture,
            &id,
            &[
                ("classification", json!("unclassified")),
                ("status", json!("open")),
            ],
        );
    }
    let before = fixture.read(&format!(".chough/bugs/{id}.json"));
    for (flag, value, allowed) in [
        (
            "--classification",
            "flaky",
            "classification must be one of unclassified, product-defect, regression, environment, requirement-gap, evidence-gap",
        ),
        (
            "--scope",
            "urgent",
            "scope must be one of blocking, in-scope, out-of-scope",
        ),
        (
            "--status",
            "closed",
            "status must be one of open, investigating, fix-pending-verification, verified, deferred, duplicate, wont-fix",
        ),
    ] {
        let refusal = fixture.fails(&["bug", "update", &id, flag, value]);
        assert!(refusal.contains(allowed), "{flag} {value}: {refusal}");
        assert_eq!(fixture.read(&format!(".chough/bugs/{id}.json")), before);
    }
}

/// REQ-cli-054 (#153): of every scope and status, only an open bug (any status but
/// verified, duplicate or wont-fix) with scope blocking is a blocking bug, and only it
/// stops leaving test; closing it releases the gate.
#[test]
fn only_an_open_blocking_bug_blocks_leaving_test() {
    let fixture = Fixture::new();
    fixture.init();
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    for scope in ["blocking", "in-scope", "out-of-scope"] {
        for status in [
            "open",
            "investigating",
            "fix-pending-verification",
            "verified",
            "deferred",
            "duplicate",
            "wont-fix",
        ] {
            set_bug_fields(
                &fixture,
                &id,
                &[("scope", json!(scope)), ("status", json!(status))],
            );
            let blocking = fixture.ok(&["status", "--json"])["blocking_bugs"].clone();
            let expected =
                scope == "blocking" && !["verified", "duplicate", "wont-fix"].contains(&status);
            assert_eq!(
                blocking == json!([id.clone()]),
                expected,
                "{scope} {status}: {blocking}"
            );
            assert_eq!(
                blocking == json!([]),
                !expected,
                "{scope} {status}: {blocking}"
            );
        }
    }
    for _ in 0..4 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(fixture.session()["stage"]["current"], "test");
    set_bug_fields(
        &fixture,
        &id,
        &[("scope", json!("blocking")), ("status", json!("deferred"))],
    );
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:blocking-bugs")
    );
    fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "wont-fix",
        "--note",
        "Not fixing",
    ]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(fixture.session()["stage"]["current"], "accept");
}

/// REQ-cli-054 (#153): search scores word overlap of title, symptom and trigger, plus 0.25
/// for a shared criterion, requirement or case; it lists matches from 0.2 and refuses a
/// new bug only from 0.5, naming both ways forward. A distinct reason is recorded in the
/// new bug's notes and may not be blank.
#[test]
fn search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates() {
    let fixture = Fixture::new();
    fixture.init();
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let words = "Coupon codes expire one day early";
    let shared = fixture.ok(&["bug", "search", words, "--requirement", "REQ-cli-001"]);
    assert_eq!(
        shared["matches"],
        json!([{"id": id, "title": "Shipping is charged at exactly the free threshold", "status": "open", "score": 0.25}]),
        "{shared:#}"
    );
    assert_eq!(fixture.ok(&["bug", "search", words])["matches"], json!([]));
    // Sharing only a requirement lists the bug without stopping a new report.
    let other = json(&fixture.run(&[
        "bug",
        "add",
        "--title",
        words,
        "--symptom",
        "Coupon rejected one day before expiry",
        "--trigger",
        "Applying a coupon near midnight",
        "--expected",
        "Accepted until expiry",
        "--actual",
        "Rejected",
        "--environment",
        "test-local",
        "--requirement",
        "REQ-cli-001",
    ]));
    assert_eq!(other["similar"][0]["score"], 0.25, "{other:#}");

    let count = || {
        std::fs::read_dir(fixture.root().join(".chough/bugs"))
            .unwrap()
            .count()
    };
    let bugs = count();
    let refusal = String::from_utf8_lossy(&add_bug(&fixture, &[]).stderr).into_owned();
    assert!(
        refusal.contains("similar bugs exist")
            && refusal.contains(&id)
            && refusal.contains("chough bug update <id> --observation <id> --note <text>")
            && refusal.contains("--distinct <why this is a different defect>"),
        "{refusal}"
    );
    let blank =
        String::from_utf8_lossy(&add_bug(&fixture, &["--distinct", " "]).stderr).into_owned();
    assert!(
        blank.contains("distinct reason must not be blank"),
        "{blank}"
    );
    assert_eq!(count(), bugs, "a refused report wrote a bug");
    let distinct = json(&add_bug(
        &fixture,
        &["--distinct", "Only on the mobile checkout"],
    ));
    let record = bug_record(&fixture, distinct["id"].as_str().unwrap());
    assert_eq!(
        record["notes"][0]["text"],
        "Added despite similar reports: Only on the mobile checkout"
    );
    assert_eq!(count(), bugs + 1);
}

/// REQ-cli-054 (#153): a duplicate or related link names another existing bug. Linking a
/// bug to itself, or to an unknown bug, is refused with nothing written, and the refusal
/// names `chough bug list --all`, which lists the bugs that can be named.
#[test]
fn links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery() {
    let fixture = Fixture::new();
    fixture.init();
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let path = format!(".chough/bugs/{id}.json");
    let before = fixture.read(&path);
    for (args, refused) in [
        (
            vec!["bug", "update", &id, "--duplicate-of", &id],
            "a bug cannot duplicate itself",
        ),
        (
            vec!["bug", "update", &id, "--related", &id],
            "a bug cannot relate to itself",
        ),
        (
            vec!["bug", "update", &id, "--related", "BUG-0099"],
            "unknown bug: BUG-0099",
        ),
        (
            vec!["bug", "update", &id, "--duplicate-of", "BUG-0099"],
            "unknown bug: BUG-0099",
        ),
        (
            vec!["bug", "update", "BUG-0099", "--status", "open"],
            "unknown bug: BUG-0099",
        ),
        (vec!["bug", "report", "BUG-0099"], "unknown bug: BUG-0099"),
    ] {
        let refusal = fixture.fails(&args);
        assert!(
            refusal.contains(refused) && refusal.contains("chough bug list --all"),
            "{args:?}: {refusal}"
        );
        assert_eq!(fixture.read(&path), before, "{args:?} changed the bug");
    }
    let listed = fixture.ok(&["bug", "list", "--all"]);
    assert!(listed.to_string().contains(&id), "{listed:#}");
}

/// REQ-cli-054 (#153): a bug marked duplicate by mistake is reopened by changing its
/// status, which clears `duplicate_of` and says so in its notes. Marking a bug verified
/// with no linked failure names `chough bug update <id> --observation <id>`, and following
/// it with a failure and then a later pass verifies the bug.
#[test]
fn a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    let original = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let other = add_other_bug(&fixture);
    let marked = json(&fixture.run(&["bug", "update", &other, "--duplicate-of", &original]));
    assert_eq!(marked["bug"]["status"], "duplicate");
    let reopened = json(&fixture.run(&["bug", "update", &other, "--status", "open"]));
    assert_eq!(reopened["bug"]["status"], "open", "{reopened:#}");
    assert_eq!(reopened["bug"]["duplicate_of"], Value::Null, "{reopened:#}");
    let notes = reopened["bug"]["notes"].to_string();
    assert!(
        notes.contains(&format!("no longer a duplicate of {original}")),
        "{notes}"
    );
    let report =
        String::from_utf8_lossy(&fixture.run(&["bug", "report", &other]).stdout).into_owned();
    assert!(report.contains("duplicate of: none"), "{report}");

    let refusal = fixture.fails(&["bug", "update", &other, "--status", "verified"]);
    assert!(
        refusal.contains(&format!("chough bug update {other} --observation <id>")),
        "{refusal}"
    );
    observe(&fixture, "coupon-fails", "failed");
    fixture.observed_at("coupon-fails", 1_793_900_000);
    observe(&fixture, "coupon-passes", "passed");
    fixture.observed_at("coupon-passes", 1_793_900_001);
    fixture.ok(&["bug", "update", &other, "--observation", "coupon-fails"]);
    let verified = json(&fixture.run(&[
        "bug",
        "update",
        &other,
        "--observation",
        "coupon-passes",
        "--status",
        "verified",
    ]));
    assert_eq!(verified["bug"]["status"], "verified", "{verified:#}");
}

/// REQ-cli-054 (#153): the report candidate shows every stored field of a fully recorded
/// bug, and `none recorded` (or `Not recorded.` for steps) for each empty one.
#[test]
fn the_report_shows_every_stored_field_and_none_recorded_for_empty_ones() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.write("repro/coupon.sh", "#!/bin/sh\n");
    fixture.commit("fixture");
    observe(&fixture, "shipping-fails", "failed");
    let other = add_other_bug(&fixture);
    let full = json(&add_bug(
        &fixture,
        &[
            "--criterion",
            "LIFE-1",
            "--case",
            "TC-cli-0001",
            "--step",
            "Open checkout",
            "--step",
            "Enter 50.00",
            "--fixture",
            "repro/coupon.sh",
            "--impact",
            "Every order at the threshold",
            "--classification",
            "product-defect",
            "--scope",
            "in-scope",
            "--observation",
            "shipping-fails",
        ],
    ))["id"]
        .as_str()
        .unwrap()
        .to_string();
    fixture.ok(&["bug", "update", &full, "--related", &other]);
    let revision = bug_record(&fixture, &full)["revision"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let report =
        String::from_utf8_lossy(&fixture.run(&["bug", "report", &full]).stdout).into_owned();
    for shown in [
        "# Shipping is charged at exactly the free threshold".to_string(),
        "Classification and confidence: product-defect; verification: reproduced".into(),
        "Impact: Every order at the threshold".into(),
        "Traceability: HI LIFE-1; requirements REQ-cli-001; cases TC-cli-0001".into(),
        "Scope and status: in-scope, open".into(),
        format!("Revision: {revision}"),
        "Environment: test-local".into(),
        "Fixture: repro/coupon.sh".into(),
        "Steps:\n1. Open checkout\n2. Enter 50.00".into(),
        "Expected: Free shipping at 50.00".into(),
        "Observed: Shipping fee of 4.99 applied".into(),
        "Observations: shipping-fails".into(),
        format!("Related: {other}; duplicate of: none"),
        "Not published".into(),
    ] {
        assert!(report.contains(&shown), "missing {shown:?}:\n{report}");
    }
    let empty =
        String::from_utf8_lossy(&fixture.run(&["bug", "report", &other]).stdout).into_owned();
    for shown in [
        "Traceability: HI none recorded; requirements none recorded; cases none recorded",
        "Fixture: none recorded",
        "Steps:\nNot recorded.",
        "Observations: none recorded",
        "Related: none recorded; duplicate of: none",
    ] {
        assert!(empty.contains(shown), "missing {shown:?}:\n{empty}");
    }
}
