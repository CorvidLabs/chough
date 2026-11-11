//! REQ-records-007's evidence part in the 0.5.x layout, red on 0.5.0 (#201). The 0.6.0
//! test of the same name on leif/0.6.0-records-coverage replaces it; this branch only
//! records the failing run on 0.5.0.

mod common;

use common::Fixture;
use serde_json::Value;

/// #201: an evidence entry and a case run record the acting agent from the override
/// variables as `actor` with agent, model and terminal. Every problem is collected.
#[cfg(unix)]
#[test]
fn the_evidence_entry_names_the_acting_agent() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"It prints ok\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/input.txt\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"it prints ok\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo ok\"]\nexpect_output = [\"ok\"]\n",
    );
    fixture.init();
    fixture.commit("start");
    let overrides = [
        ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
        ("CHOUGH_ACTOR_MODEL", "SYNTHETIC-model"),
        ("CHOUGH_HC_TERMINAL", "t999"),
    ];
    let mut problems = vec![];
    let added = fixture.run_with_env(
        &[
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
            "SYNTHETIC overridden actor",
        ],
        &overrides,
    );
    assert!(
        added.status.success(),
        "{}",
        String::from_utf8_lossy(&added.stderr)
    );
    let ran = fixture.run_with_env(
        &["cases", "run", "TC-cli-0001", "--environment", "local"],
        &overrides,
    );
    assert!(
        ran.status.success(),
        "{}",
        String::from_utf8_lossy(&ran.stderr)
    );
    for (what, output) in [("evidence add", &added), ("cases run", &ran)] {
        let id = serde_json::from_slice::<Value>(&output.stdout).unwrap()["observation"]
            .as_str()
            .unwrap()
            .to_string();
        let record: Value =
            serde_json::from_str(&fixture.read(&format!(".chough/evidence/{id}.json"))).unwrap();
        let actor = &record["actor"];
        if actor["agent"] != "SYNTHETIC-agent"
            || actor["model"] != "SYNTHETIC-model"
            || actor["terminal"] != "t999"
        {
            problems.push(format!(
                "#201: the observation that `{what}` records names no acting agent: actor = {actor}"
            ));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}
