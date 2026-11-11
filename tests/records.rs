//! REQ-records-005 in the 0.5.x layout, red on 0.5.0 (#215, #14 item 3). The 0.6.0 test
//! of the same name on leif/0.6.0-records-coverage replaces it; this branch only
//! records the failing run on 0.5.0.

mod common;

use common::Fixture;
use serde_json::Value;

/// #215: a case run's raw output stays machine-local and uncommitted, and its committed
/// observation gives the exact command, the output's SHA-256 as its fingerprint and the
/// key failing lines. #14 item 3: an absolute artifact outside the project is
/// fingerprinted in place. Every problem is collected, so the run shows them all.
#[cfg(unix)]
#[test]
fn logs_stay_local_and_failures_keep_key_lines() {
    let fixture = Fixture::new();
    fixture.stubs();
    let mut output = String::from("RAW-OUTPUT-ONLY marker line\nrunning 2 tests\n");
    output.push_str("test first ... FAILED\ntest second ... ok\n\nfailures:\n\n");
    output.push_str(
        "---- first stdout ----\nthread 'first' panicked at tests/x.rs:1:1:\nFIRST-PANIC\n\n",
    );
    output.push_str("failures:\n    first\n\ntest result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out\n");
    fixture.write("bin/cargo-output.txt", &output);
    fixture.write(
        "bin/cargo",
        "#!/bin/sh\ncat \"$(dirname \"$0\")/cargo-output.txt\"\nexit 101\n",
    );
    fixture.executable("bin/cargo");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"A test fails\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/input.txt\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"the test passes\"]\n\n[case.bind]\nadapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"first\"\n",
    );
    fixture.init();
    fixture.commit("a project whose case fails");
    let run = fixture.run(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
    let ran: Value = serde_json::from_slice(&run.stdout)
        .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&run.stderr)));
    let id = ran["observation"].as_str().unwrap().to_string();
    let mut problems = vec![];

    // #215: the raw log is not committed with the records.
    if let Ok(entries) = std::fs::read_dir(fixture.root().join(".chough/artifacts")) {
        for entry in entries.flatten() {
            let path = format!(".chough/artifacts/{}", entry.file_name().to_string_lossy());
            if fixture.read(&path).contains("RAW-OUTPUT-ONLY") {
                problems.push(format!(
                    "#215: the raw log is {path}, beside the committed records, not in .chough/local/logs/"
                ));
            }
        }
    }
    if !fixture.exists(&format!(".chough/local/logs/{id}.log")) {
        problems.push(format!(
            "#215: no machine-local log .chough/local/logs/{id}.log"
        ));
    }
    let record: Value =
        serde_json::from_str(&fixture.read(&format!(".chough/evidence/{id}.json"))).unwrap();
    if record.get("fingerprint").is_none() {
        problems.push(
            "#215: the committed observation has no fingerprint (the captured output's SHA-256)"
                .into(),
        );
    }
    if !record.to_string().contains("FIRST-PANIC") {
        problems.push("#215: the committed observation keeps no key failing lines".into());
    }

    // #14 item 3: an absolute artifact outside the project, fingerprinted in place.
    let outside = tempfile::tempdir().unwrap();
    let file = outside.path().join("SYNTHETIC-proof.bin");
    std::fs::write(&file, b"external proof bytes\n").unwrap();
    let added = fixture.run(&[
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
        "passed",
        "--environment",
        "local",
        "--summary",
        "SYNTHETIC external proof",
        "--artifact",
        file.to_str().unwrap(),
    ]);
    if !added.status.success() {
        problems.push(format!(
            "#14 item 3: an artifact outside the project is refused: {}",
            String::from_utf8_lossy(&added.stderr).trim()
        ));
    }
    assert!(problems.is_empty(), "{problems:#?}");
}
