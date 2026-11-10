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
                case("TC-cli-1!", "functional", "REQ-cli-001", "")
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
        "OBS-dry-run-000000000000000000000005",
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
    let record: Value = serde_json::from_str(
        &fixture.read(".chough/evidence/OBS-dry-run-000000000000000000000005.json"),
    )
    .unwrap();
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
            "env = { RUST_BACKTRACE = \"1\", CARGO_TARGET_DIR = \"target\", CARGO_BUILD = \"x\", CARGO_TERM_COLOR = \"never\" }",
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

#[test]
fn independent_roots_and_identical_repeated_runs_allocate_distinct_observations() {
    let definition = bound(
        "TC-cli-0001",
        "adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"passes\"",
        "",
    );
    let roots = [bounded_fixture(&definition), bounded_fixture(&definition)];
    let mut observations = vec![];
    for fixture in &roots {
        common::stand_in(
            &fixture.root().join("bin"),
            "cargo",
            "running 1 test\ntest passes ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored",
        );
        for _ in 0..2 {
            let output = fixture.run(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            let report: Value = serde_json::from_slice(&output.stdout).unwrap();
            observations.push(report["observation"].as_str().unwrap().to_string());
        }
    }
    for id in &observations {
        let suffix = id
            .strip_prefix("OBS-tc-cli-0001-")
            .unwrap_or_else(|| panic!("old-form observation allocated: {id}"));
        assert_eq!(suffix.len(), 24, "{id}");
        assert!(
            suffix
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()),
            "{id}"
        );
        let artifact = format!(".chough/artifacts/{id}.log");
        let fixture = roots
            .iter()
            .find(|fixture| fixture.exists(&format!(".chough/evidence/{id}.json")))
            .unwrap();
        let record: Value =
            serde_json::from_str(&fixture.read(&format!(".chough/evidence/{id}.json"))).unwrap();
        assert_eq!(record["artifacts"][0], artifact);
        assert!(
            fixture
                .read(&artifact)
                .contains("test result: ok. 1 passed")
        );
        assert!(record["observed_at_unix_seconds"].is_u64());
        assert!(record["duration_seconds"].is_u64());
    }
    let unique = observations
        .iter()
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(unique.len(), 4, "observations: {observations:?}");
}

#[test]
fn slug_case_ids_validate_without_changing_legacy_ids_digests_or_global_uniqueness() {
    let fixture = Fixture::new();
    fixture.init();
    let legacy = case("TC-cli-0001", "functional", "REQ-cli-001", "");
    fixture.write(".chough/cases/cli.cases.toml", &format!("{HEADER}{legacy}"));
    let legacy_digest = list(&fixture)[0]["digest"].clone();
    let slug = case(
        "TC-cli-curated-login-check",
        "functional",
        "REQ-cli-002",
        "",
    );
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{legacy}{slug}"),
    );
    let cases = list(&fixture);
    assert_eq!(cases.len(), 2);
    assert_eq!(cases[0]["digest"], legacy_digest);

    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{slug}{}",
            legacy.replace("Case TC-cli-0001", "Renamed legacy case")
        ),
    );
    assert_eq!(list(&fixture)[1]["digest"], legacy_digest);

    for invalid in [
        "TC-cli-",
        "TC-cli-Uppercase",
        "TC-cli-repeated--separator",
        "TC-cli--leading",
        "TC-cli-trailing-",
        "TC-cli-punctuation!",
        "TC-cli-non-ascii-猫",
        "TC-cli-../traversal",
    ] {
        fixture.write(
            ".chough/cases/cli.cases.toml",
            &format!("{HEADER}{}", case(invalid, "functional", "REQ-cli-001", "")),
        );
        assert!(
            fixture.fails(&["cases", "list"]).contains("id must"),
            "accepted invalid slug {invalid}"
        );
    }

    fixture.write(".chough/cases/cli.cases.toml", HEADER);
    for module in ["m", "cli", "long-module"] {
        let file = format!(".chough/cases/{module}.cases.toml");
        for offset in [0, 1, 2] {
            let length = 96 - module.len() - 1 + offset;
            let id = format!("TC-{module}-{}", "a".repeat(length));
            assert_eq!(id.len(), 99 + offset);
            fixture.write(
                &file,
                &format!(
                    "profile = \"chough-testing/1\"\nmodule = \"{module}\"\n{}",
                    case(&id, "functional", "REQ-cli-001", "")
                ),
            );
            assert_eq!(
                fixture.run(&["cases", "list"]).status.success(),
                offset < 2,
                "{id}"
            );
        }
        if module == "cli" {
            fixture.write(&file, HEADER);
        } else {
            std::fs::remove_file(fixture.root().join(file)).unwrap();
        }
    }

    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{legacy}{slug}{slug}"),
    );
    assert!(
        fixture
            .fails(&["cases", "list"])
            .contains("duplicate case ID")
    );
    fixture.write(
        ".chough/cases/other.cases.toml",
        &format!("profile = \"chough-testing/1\"\nmodule = \"other\"\n{slug}"),
    );
    assert!(
        fixture
            .fails(&["cases", "list"])
            .contains("duplicate case ID TC-cli-curated-login-check")
    );
}

fn add_evidence(
    fixture: &Fixture,
    id: Option<&str>,
    kind: &str,
    case: Option<&str>,
    summary: &str,
) -> std::process::Output {
    let mut args = vec!["evidence", "add"];
    if let Some(id) = id {
        args.push(id);
    }
    args.extend([
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        kind,
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        summary,
    ]);
    if let Some(case) = case {
        args.extend(["--case", case]);
    }
    fixture.run(&args)
}

fn recorded_id(output: &std::process::Output) -> String {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let path = report["recorded"].as_str().unwrap();
    path.strip_prefix(".chough/evidence/")
        .and_then(|name| name.strip_suffix(".json"))
        .unwrap()
        .to_string()
}

fn assert_observation_form(id: &str, slug: &str) {
    let suffix = id
        .strip_prefix(&format!("OBS-{slug}-"))
        .unwrap_or_else(|| panic!("wrong observation form: {id}"));
    assert_eq!(suffix.len(), 24, "{id}");
    assert!(
        suffix
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)),
        "{id}"
    );
}

#[test]
fn direct_evidence_allocates_accepts_new_explicit_ids_and_reads_legacy_records() {
    let fixture = bounded_fixture(&case("TC-cli-0001", "functional", "REQ-cli-001", ""));
    let automatic = recorded_id(&add_evidence(
        &fixture,
        None,
        "product-test",
        None,
        "Automatic from kind",
    ));
    assert_observation_form(&automatic, "product-test");
    let linked = recorded_id(&add_evidence(
        &fixture,
        None,
        "product-test",
        Some("TC-cli-0001"),
        "Automatic from case",
    ));
    assert_observation_form(&linked, "tc-cli-0001");

    let explicit = "OBS-product-test-000000000000000000000010";
    assert_eq!(
        recorded_id(&add_evidence(
            &fixture,
            Some(explicit),
            "product-test",
            None,
            "Explicit new form",
        )),
        explicit
    );
    let name = format!(".chough/evidence/{explicit}.json");
    let before = fixture.read(&name);
    let collision = add_evidence(
        &fixture,
        Some(explicit),
        "product-test",
        None,
        "Different content",
    );
    assert!(!collision.status.success());
    assert!(String::from_utf8_lossy(&collision.stderr).contains("identity collision"));
    assert_eq!(fixture.read(&name), before);

    let mut legacy: Value = serde_json::from_str(&before).unwrap();
    legacy["id"] = Value::from("legacy-free-form");
    legacy["summary"] = Value::from("Stored legacy record");
    fixture.write(
        ".chough/evidence/legacy-free-form.json",
        &(serde_json::to_string_pretty(&legacy).unwrap() + "\n"),
    );
    let status = fixture.ok(&["status", "--environment", "local", "--json"]);
    let ids = status["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| entry["record"]["id"].as_str().unwrap())
        .collect::<Vec<_>>();
    assert!(ids.contains(&"legacy-free-form"), "{ids:?}");
    assert!(ids.contains(&automatic.as_str()), "{ids:?}");
}

#[test]
fn old_style_explicit_observation_ids_have_an_omit_remedy_on_every_write_path() {
    let definitions = format!(
        "{}{}",
        bound(
            "TC-cli-0001",
            "adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"passes\"",
            ""
        ),
        case("TC-cli-0002", "functional", "REQ-cli-001", "")
    );
    let fixture = bounded_fixture(&definitions);
    common::stand_in(
        &fixture.root().join("bin"),
        "cargo",
        "running 1 test\ntest passes ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored",
    );
    for old in [
        "legacy-free-form",
        "tc-cli-0001-1793579540",
        "observation-1",
    ] {
        let output = add_evidence(&fixture, Some(old), "product-test", None, "Refused");
        assert!(!output.status.success());
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("omit the ID for automatic allocation")
        );
    }
    let automated = fixture.run(&[
        "cases",
        "run",
        "TC-cli-0001",
        "--environment",
        "local",
        "--observation-id",
        "legacy-free-form",
    ]);
    assert!(!automated.status.success());
    assert!(
        String::from_utf8_lossy(&automated.stderr).contains("omit the ID for automatic allocation")
    );
    let manual = fixture.run(&[
        "cases",
        "record",
        "TC-cli-0002",
        "--environment",
        "local",
        "--method",
        "agent-assisted",
        "--verdict",
        "1=pass",
        "--verdict",
        "2=pass",
        "--summary",
        "Refused",
        "--observation-id",
        "legacy-free-form",
    ]);
    assert!(!manual.status.success());
    assert!(String::from_utf8_lossy(&manual.stderr).contains("omit the ID"));
    assert!(!fixture.exists(".chough/evidence"));
    assert!(!fixture.exists(".chough/artifacts"));

    let explicit = "OBS-tc-cli-0002-000000000000000000000011";
    let accepted = fixture.run(&[
        "cases",
        "record",
        "TC-cli-0002",
        "--environment",
        "local",
        "--method",
        "agent-assisted",
        "--verdict",
        "1=pass",
        "--verdict",
        "2=pass",
        "--summary",
        "Accepted explicit new form",
        "--observation-id",
        explicit,
    ]);
    assert!(
        accepted.status.success(),
        "{}",
        String::from_utf8_lossy(&accepted.stderr)
    );
    assert!(fixture.exists(&format!(".chough/evidence/{explicit}.json")));
}

#[test]
fn an_artifact_collision_preserves_bytes_and_creates_no_observation_or_fallback() {
    let fixture = bounded_fixture(&bound(
        "TC-cli-0001",
        "adapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"passes\"",
        "",
    ));
    common::stand_in(
        &fixture.root().join("bin"),
        "cargo",
        "running 1 test\ntest passes ... ok\n\ntest result: ok. 1 passed; 0 failed; 0 ignored",
    );
    let id = "OBS-tc-cli-0001-000000000000000000000012";
    let artifact = format!(".chough/artifacts/{id}.log");
    fixture.write(&artifact, "original artifact bytes\n");
    let output = fixture.run(&[
        "cases",
        "run",
        "TC-cli-0001",
        "--environment",
        "local",
        "--observation-id",
        id,
    ]);
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(error.contains("artifact identity collision"), "{error}");
    assert!(error.contains(id), "{error}");
    assert_eq!(fixture.read(&artifact), "original artifact bytes\n");
    assert!(!fixture.exists(&format!(".chough/evidence/{id}.json")));
    assert!(std::fs::read_dir(fixture.root().join(".chough/evidence")).is_err());
}

/// Generated records under `dir` in a fixture: relative path to bytes.
fn generated(fixture: &Fixture) -> std::collections::BTreeMap<String, Vec<u8>> {
    let mut files = std::collections::BTreeMap::new();
    for dir in [".chough/evidence", ".chough/artifacts", ".chough/bugs"] {
        let Ok(entries) = std::fs::read_dir(fixture.root().join(dir)) else {
            continue;
        };
        for entry in entries {
            let entry = entry.unwrap();
            let name = format!("{dir}/{}", entry.file_name().to_string_lossy());
            files.insert(name, std::fs::read(entry.path()).unwrap());
        }
    }
    files
}

/// Issue #30: records generated independently on two branches in two roots integrate
/// through a real Git merge without add/add conflicts, silent replacement or duplicate
/// identity, and the merged tree still validates.
#[test]
fn independently_added_records_merge_without_conflict_or_replacement() {
    let a = Fixture::new();
    a.hi_stand_in();
    // Keep the bytes Git checks out identical to the bytes recorded on every host.
    a.write(".gitattributes", "* -text\n");
    let command =
        "adapter = \"command\"\nargv = [\"git\", \"--version\"]\nexpect_output = [\"git version\"]";
    a.write(
        ".chough/cases/cli.cases.toml",
        &format!("{HEADER}{}", bound("TC-cli-0001", command, "")),
    );
    a.init();
    a.commit("base");
    let b = Fixture::new();
    for entry in std::fs::read_dir(b.root()).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            std::fs::remove_dir_all(path).unwrap();
        } else {
            std::fs::remove_file(path).unwrap();
        }
    }
    b.git(&["clone", "-q", a.root().to_str().unwrap(), "."]);
    a.git(&["checkout", "-q", "-b", "a"]);
    b.git(&["checkout", "-q", "-b", "b"]);

    let mut ids = vec![];
    for (fixture, side, bug) in [
        (&a, "a", "Version check flakes"),
        (&b, "b", "Version check is slow"),
    ] {
        let run = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
        ids.push(run["observation"].as_str().unwrap().to_string());
        fixture.write(
            &format!(".chough/artifacts/{side}-proof.log"),
            &format!("proof from {side}\n"),
        );
        let mut args = vec![
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
            "--case",
            "TC-cli-0001",
            "--summary",
        ];
        let summary = format!("Reviewed on branch {side}");
        let artifact = format!(".chough/artifacts/{side}-proof.log");
        args.extend([summary.as_str(), "--artifact", artifact.as_str()]);
        ids.push(recorded_id(&fixture.run(&args)));
        let added = fixture.ok(&[
            "bug",
            "add",
            "--title",
            bug,
            "--symptom",
            "The check misbehaves",
            "--trigger",
            "Running the version check",
            "--expected",
            "A steady check",
            "--actual",
            "An unsteady check",
            "--environment",
            "local",
            "--requirement",
            "REQ-cli-001",
        ]);
        ids.push(added["id"].as_str().unwrap().to_string());
        fixture.commit(&format!("records from {side}"));
    }
    let unique: std::collections::BTreeSet<_> = ids.iter().collect();
    assert_eq!(unique.len(), ids.len(), "{ids:?}");
    let (from_a, from_b) = (generated(&a), generated(&b));
    assert!(
        from_a.keys().all(|name| !from_b.contains_key(name)),
        "shared record names"
    );

    // A plain merge: an add/add conflict or a rename would fail or show up below.
    a.git(&[
        "pull",
        "-q",
        "--no-rebase",
        "--no-ff",
        "--no-edit",
        b.root().to_str().unwrap(),
        "b",
    ]);
    let mut expected = from_a.clone();
    expected.extend(from_b.clone());
    assert_eq!(
        generated(&a),
        expected,
        "a record was lost or rewritten in the merge"
    );
    assert_eq!(a.read(".chough/artifacts/b-proof.log"), "proof from b\n");

    a.ok(&["cases", "check"]);
    let status = a.ok(&["status", "--environment", "local", "--json"]);
    let observations = status["observations"].as_array().unwrap();
    assert_eq!(observations.len(), 4, "{observations:?}");
    for observation in observations {
        assert_eq!(
            observation["freshness"], "current-observation",
            "{observation}"
        );
    }
    let bugs: Vec<_> = status["open_bugs"]
        .as_array()
        .unwrap()
        .iter()
        .map(|bug| bug["id"].as_str().unwrap())
        .collect();
    assert_eq!(bugs.len(), 2);
    assert!(
        ids.iter()
            .filter(|id| id.starts_with("BUG-"))
            .all(|id| bugs.contains(&id.as_str()))
    );
}

// ---------------------------------------------------------------------------
// Requirements an approved native change adds, linked before materialization
// (REQ-cli-009, DRIVE-6, Raven BUG-0004).

#[cfg(unix)]
const LINK_CHANGE: &str = "add-the-delta-requirement";

#[cfg(unix)]
const LINK_DELTA: &str = "## ADDED\n\n### REQUIREMENT REQ-cli-090\n\nThe helper SHALL do the thing.\n\n## MODIFIED\n\n### REQUIREMENT REQ-cli-001\n\nThe helper SHALL do it better.\n";

/// SpecSync 6.1.1's approved-delta digest (see tests/stages.rs).
#[cfg(unix)]
fn link_digest(module: &str, body: &str) -> String {
    use sha2::{Digest, Sha256};
    let body = body.replace("\r\n", "\n");
    let mut hash = Sha256::new();
    for (tag, value) in [
        ("domain", "specsync.approved-delta.v1".as_bytes()),
        ("module", module.as_bytes()),
        ("body", body.as_bytes()),
    ] {
        hash.update((tag.len() as u64).to_be_bytes());
        hash.update(tag.as_bytes());
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value);
    }
    format!("{:x}", hash.finalize())
}

/// Approve `LINK_CHANGE` with these module deltas, as SpecSync records the approval.
#[cfg(unix)]
fn approve_deltas(fixture: &Fixture, deltas: &[(&str, &str)]) {
    let mut digests = serde_json::Map::new();
    for (module, body) in deltas {
        fixture.write(
            &format!(".specsync/changes/{LINK_CHANGE}/deltas/{module}.md"),
            body,
        );
        digests.insert((*module).into(), Value::from(link_digest(module, body)));
    }
    let affected = deltas
        .iter()
        .map(|(module, _)| format!("\"{module}\""))
        .collect::<Vec<_>>()
        .join(",");
    fixture.write(".stub-affected", &format!("{affected}\n"));
    fixture.write(
        &format!(".specsync/changes/{LINK_CHANGE}/approvals.json"),
        &serde_json::json!({
            "approvals": [{"gate": "definition", "actor": "SYNTHETIC", "approved_delta_digests": digests}],
            "reopenings": [],
        })
        .to_string(),
    );
}

/// A project whose checkpoint records an approved native change with a `cli` delta
/// adding REQ-cli-090, which `specs/cli/requirements.md` does not hold yet.
#[cfg(unix)]
fn delta_fixture(record_change: bool) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.write(".stub-no-spec", "false\n");
    fixture.init();
    if record_change {
        fixture.ok(&["checkpoint", "--native-change", LINK_CHANGE]);
    }
    approve_deltas(&fixture, &[("cli", LINK_DELTA)]);
    // `cases run` records the revision it observed.
    fixture.commit("base");
    fixture
}

#[cfg(unix)]
fn delta_cases(requirement: &str) -> String {
    format!(
        "{HEADER}{}\n[[case]]\nid = \"TC-cli-delta-run\"\ntitle = \"Delta run\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"{requirement}\"]\npaths = [\"src/\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"It says ok\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo ok\"]\nexpect_output = [\"ok\"]\n",
        case("TC-cli-0001", "functional", requirement, "")
    )
}

#[cfg(unix)]
const RECORD: [&str; 12] = [
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
];

#[cfg(unix)]
#[test]
fn a_requirement_the_approved_delta_adds_is_linkable_before_materialization() {
    let fixture = delta_fixture(true);
    assert!(
        !fixture
            .read("specs/cli/requirements.md")
            .contains("REQ-cli-090")
    );
    fixture.write(".chough/cases/cli.cases.toml", &delta_cases("REQ-cli-090"));
    fixture.write(".stub-count", "");
    fixture.native_calls();

    let checked = fixture.ok(&["cases", "check"]);
    assert_eq!(
        fixture.native_calls(),
        1,
        "one native read serves every case"
    );
    let links = checked["approved_delta_requirements"].as_array().unwrap();
    assert_eq!(links.len(), 2, "{checked}");
    for (link, case) in links.iter().zip(["TC-cli-0001", "TC-cli-delta-run"]) {
        assert_eq!(link["case"], case, "{checked}");
        assert_eq!(link["requirement"], "REQ-cli-090");
        assert_eq!(link["change"], LINK_CHANGE);
        assert_eq!(link["module"], "cli");
    }

    let ran = fixture.ok(&["cases", "run", "TC-cli-delta-run", "--environment", "local"]);
    assert_eq!(ran["outcome"], "passed", "{ran}");
    assert_eq!(fixture.native_calls(), 1);
    let mut record = RECORD.to_vec();
    record.push("Looked");
    let recorded = fixture.ok(&record);
    assert_eq!(recorded["outcome"], "passed", "{recorded}");
    assert_eq!(fixture.native_calls(), 1);

    // A link the companion holds needs no native read, and none is listed.
    fixture.write(".chough/cases/cli.cases.toml", &delta_cases("REQ-cli-001"));
    let checked = fixture.ok(&["cases", "check"]);
    assert_eq!(fixture.native_calls(), 0);
    assert_eq!(
        checked["approved_delta_requirements"],
        serde_json::json!([])
    );
}

#[cfg(unix)]
#[test]
fn an_unapproved_or_changed_delta_does_not_resolve_a_requirement() {
    let consulted = format!(
        "the checkpoint's native change {LINK_CHANGE} was consulted and did not resolve it:"
    );
    let refuses = |fixture: &Fixture, requirement: &str, reason: &str| {
        fixture.write(".chough/cases/cli.cases.toml", &delta_cases(requirement));
        let mut record = RECORD.to_vec();
        record.push("Looked");
        for args in [
            vec!["cases", "check"],
            vec!["cases", "run", "TC-cli-delta-run", "--environment", "local"],
            record,
        ] {
            refused_with(
                fixture,
                &args,
                &[
                    &format!("requirement {requirement} not found in specs/cli/requirements.md"),
                    &consulted,
                    reason,
                ],
            );
        }
        assert!(!fixture.exists(".chough/evidence"));
    };

    let fixture = delta_fixture(true);
    // A typo, and an ID only another module's delta adds.
    refuses(
        &fixture,
        "REQ-cli-999",
        "its cli delta does not add or modify REQ-cli-999",
    );
    approve_deltas(
        &fixture,
        &[
            ("cli", LINK_DELTA),
            (
                "billing",
                "## ADDED\n\n### REQUIREMENT REQ-cli-092\n\nElsewhere.\n",
            ),
        ],
    );
    refuses(
        &fixture,
        "REQ-cli-092",
        "its cli delta does not add or modify REQ-cli-092",
    );
    // A removed requirement is not linkable.
    approve_deltas(
        &fixture,
        &[("cli", "## REMOVED\n\n### REQUIREMENT REQ-cli-090\n")],
    );
    refuses(
        &fixture,
        "REQ-cli-090",
        "its cli delta does not add or modify REQ-cli-090",
    );
    approve_deltas(&fixture, &[("cli", LINK_DELTA)]);

    // A delta edited after the approval.
    fixture.write(
        &format!(".specsync/changes/{LINK_CHANGE}/deltas/cli.md"),
        &LINK_DELTA.replace("do the thing", "do another thing"),
    );
    refuses(&fixture, "REQ-cli-090", "refused:native-delta-changed");
    approve_deltas(&fixture, &[("cli", LINK_DELTA)]);

    // Native approval, correction and state decide, as for the build gate.
    for (file, value, reason) in [
        (".stub-approval", "false", "refused:native-approval-invalid"),
        (
            ".stub-correction",
            "false",
            "refused:native-correction-invalid",
        ),
        (".stub-state", "draft", "refused:native-draft"),
        (".stub-state", "archived", "refused:native-archived"),
        (".stub-fail", "", "native status"),
    ] {
        let before = fixture.exists(file).then(|| fixture.read(file));
        fixture.write(file, &format!("{value}\n"));
        refuses(&fixture, "REQ-cli-090", reason);
        match before {
            Some(before) => fixture.write(file, &before),
            None => std::fs::remove_file(fixture.root().join(file)).unwrap(),
        }
    }
    fixture.ok(&["cases", "check"]);

    // With no native change in the checkpoint, nothing is consulted.
    let fixture = delta_fixture(false);
    fixture.write(".chough/cases/cli.cases.toml", &delta_cases("REQ-cli-090"));
    let error = refused_with(
        &fixture,
        &["cases", "check"],
        &["requirement REQ-cli-090 not found in specs/cli/requirements.md"],
    );
    assert!(!error.contains("was consulted"), "{error}");
}

/// Leif's review thread on #72 (src/cases.rs): a delta file for a module outside the
/// change's verified approved deltas does not resolve a case link, even when it adds
/// the requirement and the approved deltas are unchanged.
#[cfg(unix)]
#[test]
fn an_out_of_scope_delta_file_does_not_resolve_a_requirement() {
    let fixture = delta_fixture(true);
    approve_deltas(
        &fixture,
        &[(
            "billing",
            "## ADDED\n\n### REQUIREMENT REQ-billing-001\n\nBilling SHALL bill.\n",
        )],
    );
    // An extra file the approval does not cover, adding the linked requirement.
    fixture.write(
        &format!(".specsync/changes/{LINK_CHANGE}/deltas/cli.md"),
        LINK_DELTA,
    );
    fixture.write(".chough/cases/cli.cases.toml", &delta_cases("REQ-cli-090"));
    refused_with(
        &fixture,
        &["cases", "check"],
        &[
            "requirement REQ-cli-090 not found in specs/cli/requirements.md",
            &format!(
                "the checkpoint's native change {LINK_CHANGE} was consulted and did not resolve it:"
            ),
            "its cli delta is not among the change's verified approved deltas",
        ],
    );
    assert!(!fixture.exists(".chough/evidence"));
}

/// DRIVE-12: each change can add its cases in a file of its own,
/// `<module>.<part>.cases.toml`, so parallel branches never append to one file. Case IDs
/// stay unique across files, and the module still names the file.
#[test]
fn cases_split_per_change_load_together_and_ids_stay_unique() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0001", "functional", "REQ-cli-001", "")
        ),
    );
    fixture.write(
        ".chough/cases/cli.no-conflicts.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0002", "functional", "REQ-cli-002", "")
        ),
    );
    let listed = list(&fixture);
    let file_of = |id: &str| {
        listed
            .iter()
            .find(|case| case["id"] == id)
            .unwrap_or_else(|| panic!("{id} not listed: {listed:?}"))["file"]
            .clone()
    };
    assert_eq!(file_of("TC-cli-0001"), ".chough/cases/cli.cases.toml");
    assert_eq!(
        file_of("TC-cli-0002"),
        ".chough/cases/cli.no-conflicts.cases.toml"
    );

    fixture.write(
        ".chough/cases/cli.again.cases.toml",
        &format!(
            "{HEADER}{}",
            case("TC-cli-0001", "functional", "REQ-cli-003", "")
        ),
    );
    let error = fixture.fails(&["cases", "list"]);
    assert!(error.contains("duplicate case ID TC-cli-0001"), "{error}");
    std::fs::remove_file(fixture.root().join(".chough/cases/cli.again.cases.toml")).unwrap();

    for name in [
        "history.extra.cases.toml",
        "cli..cases.toml",
        "cli.a.b.cases.toml",
    ] {
        fixture.write(&format!(".chough/cases/{name}"), HEADER);
        let error = fixture.fails(&["cases", "list"]);
        assert!(
            error.contains("module must equal the file stem"),
            "{name}: {error}"
        );
        std::fs::remove_file(fixture.root().join(format!(".chough/cases/{name}"))).unwrap();
    }
    assert_eq!(list(&fixture).len(), 2);
}

/// Every file under `.chough/evidence` with its bytes, to show a refusal wrote nothing.
#[cfg(unix)]
fn evidence_files(fixture: &Fixture) -> Vec<(String, Vec<u8>)> {
    let dir = fixture.root().join(".chough/evidence");
    let mut files: Vec<(String, Vec<u8>)> = std::fs::read_dir(&dir)
        .map(|entries| {
            entries
                .map(|entry| {
                    let path = entry.unwrap().path();
                    (
                        path.file_name().unwrap().to_string_lossy().into_owned(),
                        std::fs::read(&path).unwrap(),
                    )
                })
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

/// REQ-cli-029, last criterion (#138; Leif's choice A, 2026-11-05): a fresh-agent cold
/// start is its own agent-assisted case, apart from the automated source and package
/// checks, and a skipped one is never promoted to a pass. A record that leaves an expected
/// outcome of the cold start without a verdict is refused and writes nothing, and a cold
/// start recorded as not run reads `not-run` in status, never `passed`.
#[cfg(unix)]
#[test]
fn a_skipped_cold_start_is_never_recorded_or_read_as_a_pass() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &format!(
            "{HEADER}\n[[case]]\nid = \"TC-cli-0009\"\ntitle = \"A fresh agent resumes cold from the records alone\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"agent-assisted\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"fresh-agent\"]\nartifacts = [\"transcript\"]\nsetup = [\"A new session with no earlier conversation\"]\nsteps = [\"Resume with chough next\"]\nexpected = [\"It states the handoff before its first edit\", \"It continues the recorded next task\", \"It records no human decision\"]\n"
        ),
    );
    fixture.init();
    fixture.commit("fixture");
    let before = evidence_files(&fixture);
    let refusal = fixture.fails(&[
        "cases",
        "record",
        "TC-cli-0009",
        "--environment",
        "fresh-agent",
        "--method",
        "agent-assisted",
        "--summary",
        "The cold start was skipped after the first step",
        "--verdict",
        "1=pass",
    ]);
    assert!(refusal.contains("every expected outcome"), "{refusal}");
    assert_eq!(
        evidence_files(&fixture),
        before,
        "a refused record wrote evidence"
    );

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
        "product-test",
        "--outcome",
        "not-run",
        "--environment",
        "fresh-agent",
        "--method",
        "agent-assisted",
        "--case",
        "TC-cli-0009",
        "--summary",
        "Fresh-agent cold start not run; source and package checks are separate cases",
        "--limitation",
        "no fresh agent was started",
    ]);
    let status = fixture.ok(&["status"]);
    let records: Vec<&Value> = status["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|observation| &observation["record"])
        .filter(|record| record["cases"][0]["id"] == "TC-cli-0009")
        .collect();
    assert_eq!(records.len(), 1, "{status:#}");
    assert_eq!(records[0]["outcome"], "not-run", "{status:#}");
    assert!(
        !status.to_string().contains("\"outcome\":\"passed\""),
        "a skipped cold start read as passed: {status:#}"
    );
}

/// REQ-cli-010, second criterion (#152, audit MA-01): requesting the copy-localization
/// track selects its linked cases. Design-QA cases stay omitted, with their reason, until
/// their own track is requested too, and with no track requested both are omitted, each
/// with its own reason.
#[cfg(unix)]
#[test]
fn requesting_the_copy_localization_track_selects_its_linked_cases() {
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
    let reason = |value: &Value, id: &str| -> String {
        value["omitted"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["id"] == id)
            .and_then(|case| case["reason"].as_str())
            .unwrap_or_default()
            .to_string()
    };
    let default = fixture.ok(&["cases", "select", "--requirement", "REQ-cli-001"]);
    assert_eq!(ids(&default, "selected"), ["TC-cli-0001"]);
    assert!(
        reason(&default, "TC-cli-0004").contains("copy-localization track not requested"),
        "{default:#}"
    );

    let copy = fixture.ok(&[
        "cases",
        "select",
        "--requirement",
        "REQ-cli-001",
        "--track",
        "copy-localization",
    ]);
    assert_eq!(ids(&copy, "selected"), ["TC-cli-0001", "TC-cli-0004"]);
    assert_eq!(ids(&copy, "omitted"), ["TC-cli-0002", "TC-cli-0003"]);
    assert!(
        reason(&copy, "TC-cli-0003").contains("design-qa track not requested"),
        "{copy:#}"
    );

    let both = fixture.ok(&[
        "cases",
        "select",
        "--requirement",
        "REQ-cli-001",
        "--track",
        "copy-localization",
        "--track",
        "design-qa",
    ]);
    assert_eq!(
        ids(&both, "selected"),
        ["TC-cli-0001", "TC-cli-0003", "TC-cli-0004"]
    );
    assert_eq!(ids(&both, "omitted"), ["TC-cli-0002"]);
}

/// REQ-cli-014 and REQ-cli-017 (#152, audit MA-02): every delivery kind is accepted and
/// reported as itself: publication, installation, deployment and live health, and the
/// older delivery-observation kind still. A dry run of the same case and environment is
/// reported as a dry run, never as any delivery kind.
#[cfg(unix)]
#[test]
fn every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let kinds = [
        "publication-observation",
        "installation-observation",
        "deployment-observation",
        "live-health-observation",
        "delivery-observation",
        "dry-run",
    ];
    for (index, kind) in kinds.iter().enumerate() {
        let id = format!("OBS-{kind}-{:024x}", index + 1);
        fixture.ok(&[
            "evidence",
            "add",
            &id,
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            kind,
            "--outcome",
            "passed",
            "--environment",
            "production",
            "--summary",
            &format!("A {kind} of the 1.2.0 release"),
        ]);
        let record: Value =
            serde_json::from_str(&fixture.read(&format!(".chough/evidence/{id}.json"))).unwrap();
        assert_eq!(record["schema_version"], 2, "{record:#}");
        assert_eq!(record["kind"], *kind, "{record:#}");
    }
    let status = fixture.ok(&["status", "--environment", "production", "--json"]);
    let reported: std::collections::BTreeMap<String, String> = status["observations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|observation| {
            (
                observation["record"]["id"].as_str().unwrap().to_string(),
                observation["record"]["kind"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(reported.len(), kinds.len(), "{status:#}");
    for (index, kind) in kinds.iter().enumerate() {
        assert_eq!(
            reported.get(&format!("OBS-{kind}-{:024x}", index + 1)),
            Some(&kind.to_string()),
            "{status:#}"
        );
    }
    let dry_runs: Vec<&String> = reported
        .iter()
        .filter(|(id, _)| id.starts_with("OBS-dry-run-"))
        .map(|(_, kind)| kind)
        .collect();
    assert_eq!(dry_runs, ["dry-run"], "a dry run read as a delivery kind");
}

/// #60: `cases record` returns the case and observation IDs that `cases run` returns,
/// beside every key it already returned; the observation equals the recorded file's
/// stem and the stored record's `id` (REQ-cli-011).
#[test]
fn manual_case_record_returns_its_case_and_observation_ids() {
    let fixture = bounded_fixture(&case("TC-cli-0002", "functional", "REQ-cli-001", ""));
    let report = fixture.ok(&[
        "cases",
        "record",
        "TC-cli-0002",
        "--environment",
        "local",
        "--method",
        "manual",
        "--verdict",
        "1=pass",
        "--verdict",
        "2=pass",
        "--summary",
        "Reviewed",
    ]);
    for key in ["schema_version", "recorded", "outcome", "provenance"] {
        assert!(report.get(key).is_some(), "missing {key}: {report}");
    }
    assert_eq!(report["outcome"], "passed", "{report}");
    assert_eq!(report["case"], "TC-cli-0002", "{report}");
    let observation = report["observation"]
        .as_str()
        .unwrap_or_else(|| panic!("no observation: {report}"));
    assert_observation_form(observation, "tc-cli-0002");
    let recorded = report["recorded"].as_str().unwrap();
    assert_eq!(recorded, format!(".chough/evidence/{observation}.json"));
    let stored: Value = serde_json::from_str(&fixture.read(recorded)).unwrap();
    assert_eq!(stored["id"], observation);
}

/// #60: `evidence add` returns the observation ID it stored, beside every key it
/// already returned; it equals the recorded file's stem and the stored record's `id`
/// (REQ-cli-017).
#[test]
fn evidence_add_returns_its_observation_id() {
    let fixture = bounded_fixture(&case("TC-cli-0001", "functional", "REQ-cli-001", ""));
    let output = add_evidence(
        &fixture,
        None,
        "product-test",
        Some("TC-cli-0001"),
        "Returned",
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    for key in ["schema_version", "recorded", "provenance"] {
        assert!(report.get(key).is_some(), "missing {key}: {report}");
    }
    let observation = report["observation"]
        .as_str()
        .unwrap_or_else(|| panic!("no observation: {report}"));
    assert_observation_form(observation, "tc-cli-0001");
    let recorded = report["recorded"].as_str().unwrap();
    assert_eq!(recorded, format!(".chough/evidence/{observation}.json"));
    let stored: Value = serde_json::from_str(&fixture.read(recorded)).unwrap();
    assert_eq!(stored["id"], observation);
}
