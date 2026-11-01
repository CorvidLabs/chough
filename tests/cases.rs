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
            .contains("requirement REQ-cli-999 not found in specs/cli/requirements.md")
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
        "#!/bin/sh\nfor last; do :; done\ncase \"$last\" in\n  passes) printf 'running 1 test\\ntest passes ... ok\\n\\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out\\n' ;;\n  fails) printf 'running 1 test\\ntest fails ... FAILED\\n\\ntest result: FAILED. 0 passed; 1 failed; 0 ignored\\n'; exit 101 ;;\n  broken) echo 'error: could not compile `sample`' >&2; exit 101 ;;\n  *) printf 'running 0 tests\\n\\ntest result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out\\n' ;;\nesac\n",
    );
    fixture.executable("bin/cargo");
    let cargo =
        |test: &str| format!("adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"{test}\"");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}{}{}{}{}{}",
            bound("TC-cli-0001", &cargo("passes"), ""),
            bound("TC-cli-0002", &cargo("fails"), ""),
            bound("TC-cli-0003", &cargo("misspelled_name"), ""),
            bound("TC-cli-0006", &cargo("broken"), ""),
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
    // A build or environment failure is not evidence about the behavior.
    let (code, broken) = run_case(&fixture, "TC-cli-0006");
    assert_eq!(
        (code, broken["outcome"].as_str()),
        (Some(3), Some("not-run"))
    );
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
    assert!(
        record("TC-cli-0006")["record"]["limitations"]
            .to_string()
            .contains("build or environment failure")
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

/// A project whose cases bind the given commands, with native stand-ins on PATH.
#[cfg(unix)]
fn bounded_fixture(cases: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(".chough/cases/cli.cases.toml", &format!("{HEADER}{cases}"));
    fixture.init();
    fixture.commit("fixture");
    fixture
}

/// Windows has no shell stand-ins on PATH, so `hi` is compiled from a tiny program.
#[cfg(windows)]
fn bounded_fixture(cases: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.write(
        "bin/hi.rs",
        "fn main() { print!(\"{}\", r#\"{\"hi\":1,\"export\":1,\"files\":[{\"criteria\":[{\"id\":\"LIFE-1\"}],\"retired\":[]}]}\"#); }\n",
    );
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    let status = std::process::Command::new(rustc)
        .current_dir(fixture.root().join("bin"))
        .args(["--edition", "2021", "-o", "hi.exe", "hi.rs"])
        .status()
        .unwrap();
    assert!(status.success(), "could not build the hi stand-in");
    fixture.write(".chough/cases/cli.cases.toml", &format!("{HEADER}{cases}"));
    fixture.init();
    fixture.commit("fixture");
    fixture
}

/// Run a case with chough's output in files, so a descendant that inherited a handle
/// cannot hold the test open; returns the wall time until chough exits, its report and
/// the recorded observation.
fn timed_run(fixture: &Fixture, id: &str) -> (std::time::Duration, Value, Value) {
    use std::fs::{self, File};
    use std::process::{Command, Stdio};
    let outputs = tempfile::tempdir().unwrap();
    let stdout = outputs.path().join("stdout.json");
    let stderr = outputs.path().join("stderr.txt");
    let mut paths = vec![fixture.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    let started = std::time::Instant::now();
    Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(fixture.root())
        .args(["cases", "run", id, "--environment", "test-local"])
        .env("PATH", std::env::join_paths(paths).unwrap())
        .stdin(Stdio::null())
        .stdout(File::create(&stdout).unwrap())
        .stderr(File::create(&stderr).unwrap())
        .status()
        .unwrap();
    let elapsed = started.elapsed();
    let report: Value = serde_json::from_slice(&fs::read(&stdout).unwrap())
        .unwrap_or_else(|_| panic!("{}", fs::read_to_string(&stderr).unwrap()));
    let observation = report["observation"].as_str().unwrap();
    let record =
        serde_json::from_str(&fixture.read(&format!(".chough/evidence/{observation}.json")))
            .unwrap();
    (elapsed, report, record)
}

fn limitations(record: &Value) -> Vec<String> {
    record["limitations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|limitation| limitation.as_str().unwrap().to_string())
        .collect()
}

/// Whether a process whose command line contains `pattern` is still running.
#[cfg(unix)]
fn still_running(pattern: &str) -> bool {
    for _ in 0..30 {
        let found = std::process::Command::new("pgrep")
            .args(["-f", pattern])
            .output()
            .unwrap();
        if !found.status.success() {
            return false;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    true
}

fn command_case(id: &str, argv: &str, expect: &str, timeout: u64) -> String {
    bound(
        id,
        &format!(
            "adapter = \"command\"\nargv = {argv}\nexpect_output = [\"{expect}\"]\ntimeout_seconds = {timeout}"
        ),
        "",
    )
}

#[test]
fn a_descendant_holding_the_pipes_cannot_outlast_the_case_deadline() {
    #[cfg(unix)]
    let argv = r#"["sh", "-c", "sleep 41 & sleep 41"]"#;
    #[cfg(windows)]
    let argv = r#"["cmd", "/c", "start /b ping -n 41 127.0.0.1 & ping -n 41 127.0.0.1"]"#;
    let fixture = bounded_fixture(&command_case("TC-cli-0001", argv, "never printed", 1));
    let (elapsed, report, record) = timed_run(&fixture, "TC-cli-0001");
    // The unbounded runner returned only when the descendant exited, after 20 s or more.
    assert!(elapsed.as_secs() < 11, "returned after {elapsed:?}");
    assert_eq!(report["outcome"], "failed");
    assert!(
        limitations(&record)
            .contains(&"timed out after 1 s; the command and its descendants were stopped".into()),
        "{record}"
    );
    assert_eq!(record["exit_code"], Value::Null);
    #[cfg(unix)]
    assert!(
        !still_running("sleep 41"),
        "a descendant survived the deadline"
    );
}

#[test]
fn a_descendant_holding_the_pipes_after_exit_is_stopped_by_the_deadline() {
    #[cfg(unix)]
    let argv = r#"["sh", "-c", "sleep 42 & echo started"]"#;
    #[cfg(windows)]
    let argv = r#"["cmd", "/c", "start /b ping -n 42 127.0.0.1 & echo started"]"#;
    let fixture = bounded_fixture(&command_case("TC-cli-0001", argv, "started", 5));
    let (elapsed, report, record) = timed_run(&fixture, "TC-cli-0001");
    assert!(elapsed.as_secs() < 15, "returned after {elapsed:?}");
    let limitations = limitations(&record);
    assert!(
        limitations
            .iter()
            .any(|limitation| limitation.starts_with("descendants still held output after exit")),
        "{record}"
    );
    assert_eq!(record["exit_code"], 0);
    // Output that could not be read to its end never yields a pass.
    if limitations
        .iter()
        .any(|limitation| limitation.starts_with("output capture incomplete"))
    {
        assert_ne!(report["outcome"], "passed", "{record}");
    }
    #[cfg(unix)]
    {
        assert!(
            limitations
                .contains(&"descendants still held output after exit and were stopped".into()),
            "{record}"
        );
        assert_eq!(report["outcome"], "passed", "{record}");
        assert!(!still_running("sleep 42"), "a descendant survived");
    }
}

#[test]
fn output_beyond_the_per_stream_bound_is_counted_not_retained() {
    #[cfg(unix)]
    let argv =
        r#"["sh", "-c", "yes xxxxxxx | head -c 3000000; yes eeeeeee | head -c 2500000 >&2"]"#;
    #[cfg(windows)]
    let argv = r#"["cmd", "/c", "type out.txt & type err.txt 1>&2"]"#;
    let fixture = bounded_fixture(&command_case("TC-cli-0001", argv, "xxxx", 120));
    #[cfg(windows)]
    {
        fixture.write("out.txt", &"x".repeat(3_000_000));
        fixture.write("err.txt", &"e".repeat(2_500_000));
    }
    let (_, report, record) = timed_run(&fixture, "TC-cli-0001");
    assert_eq!(report["outcome"], "passed", "{record}");
    let artifact = fixture.read(record["artifacts"][0].as_str().unwrap());
    // The existing 1 MiB artifact bound plus the truncation notes.
    assert!(artifact.len() <= 1_048_576 + 300, "{}", artifact.len());
    assert!(artifact.contains("[truncated: captured output is limited to 1048576 bytes]"));
    assert!(
        artifact.contains("[truncated: stdout wrote 3000000 bytes; 1951439 discarded]"),
        "{}",
        &artifact[artifact.len() - 300..]
    );
    assert!(
        artifact.contains("[truncated: stderr wrote 2500000 bytes; 2500000 discarded]"),
        "{}",
        &artifact[artifact.len() - 300..]
    );
}

/// A project with the portable `hi` stand-in and the given cases, before any run.
fn checked_fixture(cases: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.write(".chough/cases/cli.cases.toml", &format!("{HEADER}{cases}"));
    fixture
}

fn refused_with(fixture: &Fixture, args: &[&str], fragments: &[&str]) -> String {
    let output = fixture.run(args);
    let error = String::from_utf8_lossy(&output.stderr).into_owned();
    // Validation failures keep the general error exit code.
    assert_eq!(output.status.code(), Some(1), "{args:?}: {error}");
    for fragment in fragments {
        assert!(error.contains(fragment), "expected {fragment:?} in {error}");
    }
    error
}

#[test]
fn a_requirement_of_another_module_names_the_cases_file_that_binds_it() {
    let fixture = checked_fixture(&case(
        "TC-cli-0001",
        "functional",
        "REQ-checkout-ui-004",
        "",
    ));
    fixture.write("specs/checkout-ui/checkout-ui.spec.md", "# Checkout UI\n");
    fixture.write(
        "specs/checkout-ui/requirements.md",
        "### REQ-checkout-ui-004\nShow the order total.\n",
    );
    let expected = [
        ".chough/cases/cli.cases.toml: case TC-cli-0001",
        "requirement REQ-checkout-ui-004 not found in specs/cli/requirements.md",
        "REQ-checkout-ui-004 belongs to module checkout-ui (specs/checkout-ui/requirements.md)",
        "a cases file binds one module's contract, so move the case to .chough/cases/checkout-ui.cases.toml",
    ];
    let error = refused_with(&fixture, &["cases", "check"], &expected);
    assert!(!error.contains("specsync change check"), "{error}");
    refused_with(
        &fixture,
        &[
            "cases",
            "record",
            "TC-cli-0001",
            "--environment",
            "local",
            "--method",
            "manual",
            "--verdict",
            "1=pass",
            "--verdict",
            "2=pass",
            "--summary",
            "Looked",
        ],
        &expected,
    );
    assert!(!fixture.exists(".chough/evidence"));
}

#[test]
fn an_unresolved_requirement_is_refused_with_the_check_first_hint() {
    let neutral = "it may be a typo, or a requirement an approved native change adds that is not yet materialized: run specsync change check <change> before running or recording cases that link it";
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.write("specs/checkout-ui/checkout-ui.spec.md", "# Checkout UI\n");
    fixture.write(
        "specs/checkout-ui/requirements.md",
        "### REQ-checkout-ui-004\nShow the order total.\n",
    );
    for requirement in [
        // A typo, or a delta requirement not yet materialized, in the searched module.
        "REQ-cli-999",
        // Another module that does not hold the ID, also under the token match.
        "REQ-checkout-ui-009",
        "REQ-checkout-ui-00",
        // A module that does not exist, and one that is not a valid name.
        "REQ-billing-001",
        "REQ-a.b-001",
    ] {
        fixture.write(
            ".chough/cases/cli.cases.toml",
            &format!(
                "{HEADER}{}",
                case("TC-cli-0001", "functional", requirement, "")
            ),
        );
        let error = refused_with(
            &fixture,
            &["cases", "check"],
            &[
                ".chough/cases/cli.cases.toml: case TC-cli-0001",
                &format!("requirement {requirement} not found in specs/cli/requirements.md"),
                neutral,
            ],
        );
        assert!(!error.contains("belongs to module"), "{error}");
    }
}

#[test]
fn binding_env_is_bounded_and_refused_with_its_location() {
    let command = "adapter = \"command\"\nargv = [\"tool\"]\nexpect_output = [\"ok\"]";
    let cargo = "adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"one\"";
    let many = (0..33)
        .map(|index| format!("V{index} = \"x\""))
        .collect::<Vec<_>>()
        .join(", ");
    let long_name = format!("N{}", "A".repeat(128));
    let long_value = "v".repeat(4_097);
    let mut refusals = vec![
        (
            command,
            "env = { \"1BAD\" = \"x\" }".to_string(),
            "env name 1BAD",
        ),
        (
            command,
            "env = { \"BAD-NAME\" = \"x\" }".into(),
            "env name BAD-NAME",
        ),
        (
            command,
            format!("env = {{ {long_name} = \"x\" }}"),
            "128 bytes",
        ),
        (
            command,
            "env = { Greeting = \"a\", GREETING = \"b\" }".into(),
            "equal ignoring ASCII case",
        ),
        (command, "env = { GREETING = \"a\\u0000b\" }".into(), "NUL"),
        (command, format!("env = {{ {many} }}"), "at most 32"),
        (
            command,
            format!("env = {{ BIG = \"{long_value}\" }}"),
            "4096 bytes",
        ),
        (
            command,
            "expect_output = [\"\\u001b[32mok\"]".into(),
            "contains ESC",
        ),
    ];
    for name in [
        "Path",
        "cargo",
        "Cargo_Home",
        "rustc",
        "RustDoc",
        "rustc_wrapper",
        "Rustc_Workspace_Wrapper",
        "rustflags",
        "Cargo_Encoded_Rustflags",
        "cargo_build_target",
        "CARGO_BUILD_RUSTC",
        "Cargo_Target_x86_64_unknown_linux_gnu_Runner",
        // A toolchain override, a linker or target-level flags (which can carry
        // `-C linker=...`) replace the test binary as surely as RUSTFLAGS does.
        "RUSTUP_TOOLCHAIN",
        "Rustup_Home",
        "CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER",
        "cargo_target_x86_64_pc_windows_msvc_rustflags",
        "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTDOCFLAGS",
        "CARGO_HOST_LINKER",
        "RUSTDOCFLAGS",
        "Cargo_Encoded_Rustdocflags",
        "CARGO_PROFILE_TEST_DEBUG_ASSERTIONS",
        "CARGO_UNSTABLE_BUILD_STD",
        "RUSTC_BOOTSTRAP",
    ] {
        refusals.push((
            cargo,
            format!("env = {{ {name} = \"x\" }}"),
            "chooses what cargo builds or runs",
        ));
    }
    let fixture = checked_fixture("");
    for (binding, extra, message) in refusals {
        // An inline `expect_output` replaces the one the command binding declares.
        let binding = if extra.starts_with("expect_output") {
            binding.replace("\nexpect_output = [\"ok\"]", "")
        } else {
            binding.to_string()
        };
        fixture.write(
            ".chough/cases/cli.cases.toml",
            &format!("{HEADER}{}", bound("TC-cli-0002", &binding, &extra)),
        );
        refused_with(
            &fixture,
            &["cases", "check"],
            &[".chough/cases/cli.cases.toml: case TC-cli-0002", message],
        );
    }
    // Names cargo reads but that do not choose what it builds or runs stay allowed,
    // as does any name for the other adapters.
    for (binding, extra) in [
        (
            cargo,
            "env = { RUST_BACKTRACE = \"1\", CARGO_TARGET_DIR = \"target\", Cargo_Target_Dir_Note = \"x\", CARGO_BUILD = \"x\", CARGO_TERM_COLOR = \"never\" }",
        ),
        (
            command,
            "env = { PATH = \"bin\", NO_COLOR = \"1\", _private = \"\" }",
        ),
    ] {
        fixture.write(
            ".chough/cases/cli.cases.toml",
            &format!("{HEADER}{}", bound("TC-cli-0002", binding, extra)),
        );
        assert_eq!(fixture.ok(&["cases", "check"])["valid"], true, "{extra}");
    }
}

#[test]
fn an_empty_env_keeps_the_case_digest_and_editing_env_changes_it() {
    let binding = "adapter = \"command\"\nargv = [\"tool\"]\nexpect_output = [\"ok\"]";
    let fixture = checked_fixture(&bound("TC-cli-0002", binding, ""));
    let digest = |fixture: &Fixture| list(fixture)[0]["digest"].as_str().unwrap().to_string();
    // The digest this definition had at main 2756dc2, before `env` existed.
    let original = digest(&fixture);
    assert_eq!(original, DIGEST_AT_2756DC2);
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{}", bound("TC-cli-0002", binding, "env = {}")),
    );
    assert_eq!(digest(&fixture), original);
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            bound("TC-cli-0002", binding, "env = { NO_COLOR = \"1\" }")
        ),
    );
    let declared = digest(&fixture);
    assert_ne!(declared, original);
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            bound("TC-cli-0002", binding, "env = { NO_COLOR = \"0\" }")
        ),
    );
    let edited = digest(&fixture);
    assert_ne!(edited, declared);
    assert_ne!(edited, original);
}

/// `bound("TC-cli-0002", <command binding>, "")` as digested at main 2756dc2.
const DIGEST_AT_2756DC2: &str = "7e6c13eda25ecad83cc42f630af994d06d9b9a4aed1d3fd33247aa726496825f";

fn env_printer(id: &str, extra: &str, expect: &str) -> String {
    #[cfg(unix)]
    let argv = r#"["sh", "-c", "printf 'greeting=%s\n' \"$CHOUGH_GREETING\""]"#;
    #[cfg(windows)]
    let argv = r#"["cmd", "/c", "echo greeting=%CHOUGH_GREETING%"]"#;
    bound(
        id,
        &format!(
            "adapter = \"command\"\nargv = {argv}\nexpect_output = [\"{expect}\"]\ntimeout_seconds = 60"
        ),
        extra,
    )
}

const INHERITED: &str = "inherited environment variables were not recorded";

#[test]
fn a_declared_variable_reaches_the_command_and_is_named_without_its_value() {
    let fixture = bounded_fixture(&format!(
        "{}{}",
        env_printer(
            "TC-cli-0001",
            "env = { CHOUGH_GREETING = \"hello-from-the-case\" }",
            "greeting=hello-from-the-case"
        ),
        env_printer("TC-cli-0002", "", "greeting="),
    ));
    let (_, report, record) = timed_run(&fixture, "TC-cli-0001");
    assert_eq!(report["outcome"], "passed", "{record}");
    let declared = limitations(&record);
    assert!(declared.contains(&INHERITED.to_string()), "{record}");
    assert!(
        declared.contains(&"declared environment set: CHOUGH_GREETING".to_string()),
        "{record}"
    );
    // The value is bound through the case digest, never copied into the observation.
    assert!(
        !record.to_string().contains("hello-from-the-case"),
        "{record}"
    );
    assert!(
        fixture
            .read(record["artifacts"][0].as_str().unwrap())
            .contains("greeting=hello-from-the-case")
    );

    let (_, report, record) = timed_run(&fixture, "TC-cli-0002");
    assert_eq!(report["outcome"], "passed", "{record}");
    let undeclared = limitations(&record);
    assert!(undeclared.contains(&INHERITED.to_string()), "{record}");
    assert!(
        !undeclared
            .iter()
            .any(|limitation| limitation.starts_with("declared environment")),
        "{record}"
    );
    assert!(
        !fixture
            .read(record["artifacts"][0].as_str().unwrap())
            .contains("hello-from-the-case")
    );
}

/// Vitest-style coloured summary: bold, dim and colour codes split the visible text
/// ` Test Files  1 passed (1)`, followed by an OSC 8 hyperlink and a two-byte escape.
const COLOURED: &[u8] = b"\x1b[2m Test Files \x1b[22m \x1b[1m\x1b[32m1 passed\x1b[39m\x1b[22m\x1b[90m (1)\x1b[39m\n\x1b]8;;file:///report\x07report\x1b]8;;\x1b\\ written\x1bM\n";

fn printer(id: &str, file: &str, expect: &str) -> String {
    #[cfg(unix)]
    let argv = format!(r#"["cat", "{file}"]"#);
    #[cfg(windows)]
    let argv = format!(r#"["cmd", "/c", "type {file}"]"#);
    bound(
        id,
        &format!(
            "adapter = \"command\"\nargv = {argv}\nexpect_output = [\"{expect}\"]\ntimeout_seconds = 60"
        ),
        "",
    )
}

const ESCAPES_REMOVED: &str =
    "terminal escape sequences were removed before matching; the artifact keeps the raw output";

#[test]
fn coloured_output_matches_its_visible_text_and_the_artifact_keeps_the_raw_bytes() {
    let fixture = bounded_fixture(&format!(
        "{}{}{}",
        printer("TC-cli-0001", "coloured.txt", "Test Files  1 passed (1)"),
        printer("TC-cli-0002", "coloured.txt", "Test Files  2 passed"),
        printer("TC-cli-0003", "plain.txt", "Test Files  1 passed (1)"),
    ));
    std::fs::write(fixture.root().join("coloured.txt"), COLOURED).unwrap();
    fixture.write("plain.txt", " Test Files  1 passed (1)\n");

    let (_, report, record) = timed_run(&fixture, "TC-cli-0001");
    assert_eq!(report["outcome"], "passed", "{record}");
    assert!(
        limitations(&record).contains(&ESCAPES_REMOVED.to_string()),
        "{record}"
    );
    let artifact = std::fs::read(
        fixture
            .root()
            .join(record["artifacts"][0].as_str().unwrap()),
    )
    .unwrap();
    let mut raw = b"--- stdout ---\n".to_vec();
    raw.extend(COLOURED);
    raw.extend(b"\n--- stderr ---\n");
    assert_eq!(artifact, raw, "the artifact must keep the captured bytes");

    // A fragment absent from the visible text still fails.
    let (_, report, record) = timed_run(&fixture, "TC-cli-0002");
    assert_eq!(report["outcome"], "failed", "{record}");
    assert!(
        limitations(&record)
            .iter()
            .any(|limitation| limitation.starts_with("expected output missing")),
        "{record}"
    );

    // Output without escapes carries no escape limitation.
    let (_, report, record) = timed_run(&fixture, "TC-cli-0003");
    assert_eq!(report["outcome"], "passed", "{record}");
    assert!(
        !limitations(&record).contains(&ESCAPES_REMOVED.to_string()),
        "{record}"
    );
}

#[test]
fn an_unterminated_escape_never_hides_a_second_test_result() {
    // Two test result lines, each for one test, with an OSC that a line break
    // interrupts between them: a cleaner that ran on to the later BEL would hide the
    // first line and count one test.
    let output = "running 1 test\ntest first ... ok\n\x1b]0;printed by a test\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\nrunning 1 test\ntest second ... ok\x07\ntest result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out";
    let fixture = bounded_fixture(&bound(
        "TC-cli-0001",
        "adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"first\"",
        "",
    ));
    common::stand_in(&fixture.root().join("bin"), "cargo", output);
    let (_, report, record) = timed_run(&fixture, "TC-cli-0001");
    assert_eq!(report["outcome"], "not-run", "{record}");
    assert_eq!(record["tests_run"], 2, "{record}");
    assert!(
        limitations(&record).contains(&"the binding should select exactly one test; 2 ran".into()),
        "{record}"
    );
}
