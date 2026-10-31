//! Test definitions, focused selection and honest execution (REQ-cli-009 to REQ-cli-011, REQ-cli-017).

mod common;

use common::Fixture;
use serde_json::Value;

const HEADER: &str = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n";

fn case(id: &str, track: &str, requirement: &str, extra: &str) -> String {
    format!(
        "\n[[case]]\nid = \"{id}\"\ntitle = \"Case {id}\"\nstatus = \"active\"\ntrack = \"{track}\"\nmethod = \"manual\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"{requirement}\"]\nenvironments = [\"local\"]\nartifacts = [\"reviewer-note\"]\nsetup = [\"Fixture\"]\nsteps = [\"Look\"]\nexpected = [\"It is right\", \"It is clear\"]\n{extra}"
    )
}

fn bound(id: &str, test: &str, extra: &str) -> String {
    format!(
        "\n[[case]]\nid = \"{id}\"\ntitle = \"Automated {id}\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"The bound test passes\"]\n\n[case.bind]\n{test}\n{extra}"
    )
}

fn list(fixture: &Fixture) -> Vec<Value> {
    fixture.ok(&["cases", "list"])["cases"]
        .as_array()
        .unwrap()
        .clone()
}

#[test]
fn definitions_with_bad_shapes_are_refused_with_their_location() {
    let fixture = Fixture::new();
    fixture.init();
    let cargo = "adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"one\"";
    for (content, message) in [
        (
            format!(
                "{HEADER}{}",
                case("TC-cli-0001", "functional", "REQ-cli-001", "surprise = 1\n")
            ),
            "unknown field",
        ),
        (
            "profile = \"chough-testing/9\"\nmodule = \"cli\"\n".to_string(),
            "unsupported profile",
        ),
        (
            format!(
                "{HEADER}{}{}",
                case("TC-cli-0001", "functional", "REQ-cli-001", ""),
                case("TC-cli-0001", "functional", "REQ-cli-002", "")
            ),
            "duplicate case ID",
        ),
        (
            format!(
                "{HEADER}{}",
                case("TC-cli-1", "functional", "REQ-cli-001", "")
            ),
            "TC-cli-0001",
        ),
        (
            format!(
                "{HEADER}{}",
                case("TC-cli-0001", "functional", "REQ-cli-001", "")
            )
            .replace(
                "expected = [\"It is right\", \"It is clear\"]",
                "expected = []",
            ),
            "expected must not be empty",
        ),
        (
            format!(
                "{HEADER}{}",
                case("TC-cli-0001", "functional", "REQ-cli-001", "")
            )
            .replace("method = \"manual\"", "method = \"automated\""),
            "needs a [case.bind]",
        ),
        (
            format!(
                "{HEADER}{}",
                case(
                    "TC-cli-0001",
                    "functional",
                    "REQ-cli-001",
                    &format!("\n[case.bind]\n{cargo}\n")
                )
            ),
            "manual case cannot have a binding",
        ),
        (
            format!(
                "{HEADER}{}",
                bound(
                    "TC-cli-0002",
                    "adapter = \"command\"\nargv = [\"true\"]",
                    ""
                )
            ),
            "exit code alone is not an assertion",
        ),
    ] {
        fixture.write(".chough/cases/cli.cases.toml", &content);
        let error = fixture.fails(&["cases", "list"]);
        assert!(error.contains(message), "expected {message:?} in {error}");
        assert!(error.contains("cli.cases.toml"), "{error}");
    }
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0001", "functional", "REQ-cli-001", "")
        ),
    );
    assert_eq!(list(&fixture).len(), 1);
    fixture.write(".chough/cases/other.cases.toml", HEADER);
    assert!(
        fixture
            .fails(&["cases", "list"])
            .contains("module must equal the file stem")
    );
}

#[test]
fn case_identity_survives_reordering_and_title_edits() {
    let fixture = Fixture::new();
    fixture.init();
    let first = case("TC-cli-0001", "functional", "REQ-cli-001", "");
    let second = case("TC-cli-0002", "design-qa", "REQ-cli-002", "");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{first}{second}"),
    );
    let before = list(&fixture);
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{second}{}",
            first.replace("Case TC-cli-0001", "Renamed title")
        ),
    );
    let after = list(&fixture);
    let digest = |cases: &[Value], id: &str| {
        cases.iter().find(|case| case["id"] == id).unwrap()["digest"].clone()
    };
    for id in ["TC-cli-0001", "TC-cli-0002"] {
        assert_eq!(digest(&before, id), digest(&after, id));
    }
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{second}{}",
            first.replace("\"It is clear\"", "\"It is very clear\"")
        ),
    );
    assert_ne!(
        digest(&before, "TC-cli-0001"),
        digest(&list(&fixture), "TC-cli-0001")
    );
}

#[cfg(unix)]
#[test]
fn check_validates_links_against_native_records() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0001", "functional", "REQ-cli-999", "")
        ),
    );
    assert!(
        fixture
            .fails(&["cases", "check"])
            .contains("requirement not found")
    );
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0001", "functional", "REQ-cli-001", "")
        )
        .replace("LIFE-1", "LIFE-2"),
    );
    assert!(
        fixture
            .fails(&["cases", "check"])
            .contains("unknown or retired criterion")
    );
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0001", "functional", "REQ-cli-001", "")
        )
        .replace("LIFE-1", "LIFE-2")
        .replace("status = \"active\"", "status = \"retired\""),
    );
    assert_eq!(fixture.ok(&["cases", "check"])["valid"], true);
}

#[cfg(unix)]
#[test]
fn selection_is_focused_on_the_affected_scope_and_tracks() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}{}{}{}",
            case("TC-cli-0001", "functional", "REQ-cli-001", ""),
            case("TC-cli-0002", "functional", "REQ-cli-002", ""),
            case("TC-cli-0003", "design-qa", "REQ-cli-001", ""),
            case("TC-cli-0004", "copy-localization", "REQ-cli-001", ""),
        ),
    );
    let ids = |value: &Value, key: &str| -> Vec<String> {
        value[key]
            .as_array()
            .unwrap()
            .iter()
            .map(|case| case["id"].as_str().unwrap().to_string())
            .collect()
    };
    let selected = fixture.ok(&["cases", "select", "--requirement", "REQ-cli-001"]);
    assert_eq!(ids(&selected, "selected"), ["TC-cli-0001"]);
    assert_eq!(
        ids(&selected, "omitted"),
        ["TC-cli-0002", "TC-cli-0003", "TC-cli-0004"]
    );
    assert!(
        selected["omitted"][0]["reason"]
            .as_str()
            .unwrap()
            .contains("not linked")
    );
    assert!(
        selected["omitted"][1]["reason"]
            .as_str()
            .unwrap()
            .contains("design-qa track not requested")
    );
    let with_design = fixture.ok(&[
        "cases",
        "select",
        "--requirement",
        "REQ-cli-001",
        "--track",
        "design-qa",
    ]);
    assert_eq!(
        ids(&with_design, "selected"),
        ["TC-cli-0001", "TC-cli-0003"]
    );
    assert!(fixture.fails(&["cases", "select"]).contains("at least one"));
}

#[cfg(unix)]
fn execution_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    // A stand-in for cargo that prints real cargo summary lines for each bound test.
    fixture.write(
        "bin/cargo",
        "#!/bin/sh\nfor last; do :; done\ncase \"$last\" in\n  passes) printf 'running 1 test\\ntest passes ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out\\n' ;;\n  fails) printf 'running 1 test\\ntest fails ... FAILED\\n\\ntest result: FAILED. 0 passed; 1 failed; 0 ignored\\n'; exit 101 ;;\n  *) printf 'running 0 tests\\n\\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out\\n' ;;\nesac\n",
    );
    fixture.executable("bin/cargo");
    let cargo =
        |test: &str| format!("adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"{test}\"");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}{}{}{}{}",
            bound("TC-cli-0001", &cargo("passes"), ""),
            bound("TC-cli-0002", &cargo("fails"), ""),
            bound("TC-cli-0003", &cargo("misspelled_name"), ""),
            bound(
                "TC-cli-0004",
                "adapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo checked 3 rows\"]\nexpect_output = [\"checked 4 rows\"]",
                ""
            ),
            bound(
                "TC-cli-0005",
                "adapter = \"command\"\nargv = [\"sleep\", \"5\"]\nexpect_output = [\"never\"]\ntimeout_seconds = 1",
                ""
            ),
        ),
    );
    fixture.init();
    fixture.commit("fixture");
    fixture
}

#[cfg(unix)]
fn run_case(fixture: &Fixture, id: &str) -> (Option<i32>, Value) {
    let output = fixture.run(&["cases", "run", id, "--environment", "test-local"]);
    (
        output.status.code(),
        serde_json::from_slice(&output.stdout)
            .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&output.stderr))),
    )
}

#[cfg(unix)]
#[test]
fn execution_records_failures_and_vacuous_runs_honestly() {
    let fixture = execution_fixture();
    let (code, passed) = run_case(&fixture, "TC-cli-0001");
    assert_eq!(
        (code, passed["outcome"].as_str()),
        (Some(0), Some("passed"))
    );
    let (code, failed) = run_case(&fixture, "TC-cli-0002");
    assert_eq!(
        (code, failed["outcome"].as_str()),
        (Some(3), Some("failed"))
    );
    let (code, vacuous) = run_case(&fixture, "TC-cli-0003");
    assert_eq!(
        (code, vacuous["outcome"].as_str()),
        (Some(3), Some("not-run"))
    );
    let (_, missing) = run_case(&fixture, "TC-cli-0004");
    assert_eq!(missing["outcome"], "failed");
    let (_, slow) = run_case(&fixture, "TC-cli-0005");
    assert_eq!(slow["outcome"], "failed");

    let status = fixture.ok(&["status", "--environment", "test-local", "--json"]);
    let observations = status["observations"].as_array().unwrap();
    let record = |case: &str| {
        observations
            .iter()
            .find(|observation| observation["record"]["cases"][0]["id"] == case)
            .unwrap()
            .clone()
    };
    // A passing case does not hide the failure recorded beside it.
    assert_eq!(record("TC-cli-0001")["record"]["outcome"], "passed");
    assert_eq!(record("TC-cli-0002")["record"]["outcome"], "failed");
    assert_eq!(record("TC-cli-0002")["freshness"], "current-observation");
    assert_eq!(record("TC-cli-0003")["record"]["tests_run"], 0);
    assert!(
        record("TC-cli-0003")["record"]["limitations"]
            .to_string()
            .contains("no test matched")
    );
    assert!(
        record("TC-cli-0004")["record"]["limitations"]
            .to_string()
            .contains("expected output missing")
    );
    assert!(
        record("TC-cli-0005")["record"]["limitations"]
            .to_string()
            .contains("timed out")
    );
    let artifact = record("TC-cli-0002")["record"]["artifacts"][0]
        .as_str()
        .unwrap()
        .to_string();
    assert!(fixture.read(&artifact).contains("1 failed"));
    assert_eq!(
        record("TC-cli-0001")["record"]["argv"],
        serde_json::json!([
            "cargo", "test", "--locked", "--test", "cli", "--", "--exact", "passes"
        ])
    );

    // Editing what a case asserts makes its earlier observations stale.
    let content = fixture.read(".chough/cases/cli.cases.toml");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &content.replacen("The bound test passes", "The bound test passes quickly", 1),
    );
    let status = fixture.ok(&["status", "--environment", "test-local", "--json"]);
    let stale = status["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|observation| observation["record"]["cases"][0]["id"] == "TC-cli-0001")
        .unwrap()
        .clone();
    assert_eq!(stale["freshness"], "stale-case-definition");
}

#[cfg(unix)]
#[test]
fn manual_records_need_a_verdict_for_every_expected_outcome() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0007", "design-qa", "REQ-cli-001", "")
        ),
    );
    fixture.init();
    fixture.commit("fixture");
    fixture.write(".chough/artifacts/shot.png", "not really a png");
    let base = [
        "cases",
        "record",
        "TC-cli-0007",
        "--environment",
        "chromium-1280",
        "--method",
        "agent-assisted",
        "--summary",
        "Compared states",
        "--artifact",
        ".chough/artifacts/shot.png",
    ];
    let mut partial = base.to_vec();
    partial.extend(["--verdict", "1=pass"]);
    assert!(fixture.fails(&partial).contains("every expected outcome"));
    let mut full = base.to_vec();
    full.extend(["--verdict", "1=pass", "--verdict", "2=fail"]);
    let output = fixture.run(&full);
    assert_eq!(output.status.code(), Some(3));
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["outcome"], "failed");
    let recorded = fixture.read(value["recorded"].as_str().unwrap());
    assert!(recorded.contains("Verdicts 1=pass, 2=fail"));
    assert!(recorded.contains("\"method\": \"agent-assisted\""));
    assert!(
        fixture
            .fails(&["cases", "run", "TC-cli-0007", "--environment", "x"])
            .contains("cases record")
    );
}

#[cfg(unix)]
#[test]
fn schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    fixture.ok(&[
        "evidence",
        "add",
        "deploy-rehearsal",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "dry-run",
        "--outcome",
        "passed",
        "--environment",
        "staging-simulated",
        "--summary",
        "Simulated deploy script printed success; nothing was deployed",
        "--limitation",
        "simulated adapter",
    ]);
    let record: Value =
        serde_json::from_str(&fixture.read(".chough/evidence/deploy-rehearsal.json")).unwrap();
    assert_eq!(record["schema_version"], 2);
    assert_eq!(record["kind"], "dry-run");
    assert_eq!(record["limitations"][0], "simulated adapter");
    // Schema 1 records written by the bootstrap helper remain readable.
    let mut legacy = record.clone();
    legacy["schema_version"] = 1.into();
    legacy["id"] = "legacy".into();
    legacy["kind"] = "product-test".into();
    legacy.as_object_mut().unwrap().remove("limitations");
    fixture.write(".chough/evidence/legacy.json", &legacy.to_string());
    let status = fixture.ok(&["status", "--environment", "staging-simulated", "--json"]);
    assert_eq!(status["observations"].as_array().unwrap().len(), 2);
    legacy["outcome"] = "not-run".into();
    fixture.write(".chough/evidence/legacy.json", &legacy.to_string());
    assert!(fixture.fails(&["status", "--json"]).contains("schema 1"));
}
