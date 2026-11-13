//! Local-first bug backlog (REQ-cli-012): one Markdown record per bug.

mod common;

use common::Fixture;
use serde_json::{Value, json};

/// `YYYY-MM-DDTHH:MM:SSZ` for a Unix time.
fn iso(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    let time = seconds % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        time / 3_600,
        time % 3_600 / 60,
        time % 60
    )
}

/// HEAD's commit, or None outside Git.
fn head(fixture: &Fixture) -> Option<String> {
    let output = std::process::Command::new("git")
        .current_dir(fixture.root())
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// An evidence entry of the active session, with the REQ-cli-017 fields a bug reads,
/// observed at `seconds`; returns its ID. Written directly so these tests do not depend
/// on how `evidence add` names entries.
fn entry(fixture: &Fixture, name: &str, result: &str, seconds: u64) -> String {
    let id = format!("2026-11-10-{name}");
    let revision = head(fixture)
        .map(|revision| format!("revision: {revision}\n"))
        .unwrap_or_default();
    fixture.write(
        &format!(".chough/sessions/{}/evidence/{id}.md", fixture.active_id()),
        &format!(
            "---\nkind: evidence\nid: {id}\naliases: []\nclaim: Observed the shipping threshold check\nresult: {result}\nevidence_kind: product-test\n{revision}environment: test-local\nobserved: {}\n---\n**Observed the shipping threshold check.**\n",
            iso(seconds)
        ),
    );
    id
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

/// A new bug is named `<YYYY-MM-DD>-<slug>` (REQ-records-003), at most 80 bytes, with
/// no number taken from a count of other records.
fn assert_new_bug_name(id: &str) {
    assert!(id.len() <= 80, "bug name exceeds 80 bytes: {id}");
    let bytes = id.as_bytes();
    assert!(
        id.len() > 11
            && bytes[..10]
                .iter()
                .enumerate()
                .all(|(at, byte)| if at == 4 || at == 7 {
                    *byte == b'-'
                } else {
                    byte.is_ascii_digit()
                })
            && bytes[10] == b'-',
        "bug name does not start with a date: {id}"
    );
    let slug = &id[11..];
    assert!(
        slug.bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'),
        "slug is not lowercase ASCII: {id}"
    );
    assert!(!slug.starts_with('-') && !slug.ends_with('-') && !slug.contains("--"));
    assert!(!id.starts_with("BUG-"), "{id}");
}

fn bug_path(id: &str) -> String {
    format!(".chough/bugs/{id}.md")
}

/// The stored bug as `bug show` prints it.
fn bug_record(fixture: &Fixture, id: &str) -> Value {
    fixture.ok(&["bug", "show", id])["bug"].clone()
}

/// Overwrite front matter fields of bug `id`, as a hand edit would.
fn set_bug_fields(fixture: &Fixture, id: &str, fields: &[(&str, &str)]) {
    let path = bug_path(id);
    let text = fixture.read(&path);
    let (front, body) = text[4..].split_once("\n---\n").unwrap();
    let mut lines: Vec<String> = front.lines().map(str::to_string).collect();
    for (field, value) in fields {
        let prefix = format!("{field}:");
        match lines.iter_mut().find(|line| line.starts_with(&prefix)) {
            Some(line) => *line = format!("{field}: {value}"),
            None => lines.push(format!("{field}: {value}")),
        }
    }
    fixture.write(&path, &format!("---\n{}\n---\n{body}", lines.join("\n")));
}

/// The `## Log` lines of bug `id`.
fn log(fixture: &Fixture, id: &str) -> Vec<String> {
    let text = fixture.read(&bug_path(id));
    let log = text.split_once("\n## Log\n").unwrap().1;
    log.lines()
        .take_while(|line| !line.starts_with("## "))
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}

#[test]
fn unverified_out_of_scope_bugs_stay_visible_without_changing_scope() {
    let fixture = Fixture::new();
    fixture.init();
    let before = fixture.session();
    let added = json(&add_bug(&fixture, &["--scope", "out-of-scope"]));
    let id = added["id"].as_str().unwrap();
    assert_new_bug_name(id);
    assert_eq!(added["recorded"], bug_path(id));
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
    let repro_1 = entry(&fixture, "repro-1", "failed", 1_793_000_000);
    let added = json(&add_bug(
        &fixture,
        &[
            "--observation",
            &repro_1,
            "--classification",
            "product-defect",
            "--step",
            "Check out with a 50.00 cart",
        ],
    ));
    let id = added["id"].as_str().unwrap().to_string();
    assert_new_bug_name(&id);
    assert_eq!(added["verification"], "reproduced");

    // A second report of the same symptom is refused and pointed at the existing bug.
    let duplicate = add_bug(&fixture, &[]);
    assert!(!duplicate.status.success());
    let error = String::from_utf8_lossy(&duplicate.stderr);
    assert!(error.contains("similar bugs exist") && error.contains(&id));

    let repro_2 = entry(&fixture, "repro-2", "failed", 1_793_000_010);
    let updated = fixture.ok(&[
        "bug",
        "update",
        &id,
        "--observation",
        &repro_2,
        "--note",
        "Reported again from the mobile checkout",
    ]);
    // A link to a session's entry is stored qualified, so a merge cannot make it
    // ambiguous (REQ-cli-023).
    let session = fixture.active_id();
    assert_eq!(
        updated["bug"]["evidence"],
        json!([
            format!("{session}/{repro_1}"),
            format!("{session}/{repro_2}")
        ])
    );
    let last = log(&fixture, &id).pop().unwrap();
    assert!(
        last.contains(&format!("evidence: {session}/{repro_2}"))
            && last.contains("Reported again from the mobile checkout"),
        "{last}"
    );

    let distinct = json(&add_bug(
        &fixture,
        &["--distinct", "Different code path: gift cards"],
    ));
    let distinct_id = distinct["id"].as_str().unwrap().to_string();
    assert_new_bug_name(&distinct_id);
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
    // The fix must be observed strictly after the latest failure (REQ-cli-012).
    let fixed = entry(&fixture, "fixed-1", "passed", 1_793_000_011);
    let verified = fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "verified",
        "--observation",
        &fixed,
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
    let worked_before = entry(&fixture, "worked-before", "passed", 1_793_000_000);
    fixture.write("src/input.txt", "second version\n");
    fixture.commit("second");
    let fails_now = entry(&fixture, "fails-now", "failed", 1_793_000_100);
    let passes_now = entry(&fixture, "passes-now", "passed", 1_793_000_101);
    let regression = [
        "--classification",
        "regression",
        "--observation",
        fails_now.as_str(),
    ];
    let mut missing = regression.to_vec();
    missing.extend(["--distinct", "first report"]);
    assert!(
        !add_bug(&fixture, &missing).status.success(),
        "a regression without a baseline must be refused"
    );
    for (baseline, message) in [
        (fails_now.as_str(), "did not pass"),
        (passes_now.as_str(), "earlier revision"),
    ] {
        let mut args = regression.to_vec();
        args.extend(["--regression-baseline", baseline]);
        let output = add_bug(&fixture, &args);
        assert!(!output.status.success());
        assert!(String::from_utf8_lossy(&output.stderr).contains(message));
    }
    let mut args = regression.to_vec();
    args.extend(["--regression-baseline", &worked_before]);
    let added = json(&add_bug(&fixture, &args));
    assert_new_bug_name(added["id"].as_str().unwrap());
}

#[test]
fn an_open_blocking_bug_sends_work_back_to_build() {
    let fixture = Fixture::new();
    fixture.init();
    for _ in 0..4 {
        fixture.ok(&["stage", "advance", "--json"]);
    }
    assert_eq!(fixture.session()["stage"]["current"], "test");
    let added = json(&add_bug(&fixture, &["--scope", "blocking"]));
    let id = added["id"].as_str().unwrap().to_string();
    assert_new_bug_name(&id);
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
    fixture.ok(&[
        "stage", "back", "--json", "--to", "build", "--reason", &reason,
    ]);
    fixture.ok(&[
        "bug",
        "update",
        &id,
        "--scope",
        "out-of-scope",
        "--note",
        "The human moved it to a follow-up change",
    ]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(fixture.session()["stage"]["current"], "accept");
}

/// A committed fixture with the given evidence at the given times, linked to one bug.
fn ordered_bug(observations: &[(&str, &str, u64)]) -> Fixture {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    let mut ids = vec![];
    for (name, outcome, seconds) in observations {
        ids.push(entry(&fixture, name, outcome, *seconds));
    }
    let mut extra = vec!["--classification", "product-defect"];
    for id in &ids {
        extra.extend(["--observation", id.as_str()]);
    }
    let added = json(&add_bug(&fixture, &extra));
    assert_new_bug_name(added["id"].as_str().unwrap());
    fixture
}

fn only_bug_id(fixture: &Fixture) -> String {
    let bugs = fixture.ok(&["bug", "list", "--all"]);
    assert_eq!(bugs["bugs"].as_array().unwrap().len(), 1);
    let id = bugs["bugs"][0]["id"].as_str().unwrap().to_string();
    assert_new_bug_name(&id);
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
    let later = entry(&fixture, "later-pass", "passed", 1_793_000_101);
    let verified = fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "verified",
        "--observation",
        &later,
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
    assert_new_bug_name(&id);
    let path = bug_path(&id);
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

/// A bug migrated from 0.5.x: a Markdown bug named by its date and title, answering to
/// its old `BUG-0019` ID as an alias (REQ-records-003, REQ-records-009).
const LEGACY: &str = "2026-11-01-legacy-allocator-sentinel";

fn seed_migrated_legacy_bug(fixture: &Fixture) {
    fixture.write(
        &bug_path(LEGACY),
        &format!(
            "---\nkind: bug\nid: {LEGACY}\naliases: [BUG-0019]\ntitle: Legacy allocator sentinel\nstatus: open\nclassification: product-defect\nscope: in-scope\nimpact: Compatibility fixture\ncriteria: [FLOW-8, LIFE-5]\nrequirements: [REQ-cli-012]\ncases: [TC-cli-0125]\nevidence: []\nreceipts: []\nenvironment: test-local\nfound_by: legacy fixture\nrelated: []\ncreated: 2026-11-01T00:00:01Z\nupdated: 2026-11-01T00:00:01Z\n---\n# Legacy allocator sentinel\n\n**What happened:** Seeded legacy record remains readable\nActual: The legacy identity is present\n\n**Steps:** Load the independent root\n1. Read the seeded record\n\n**Expected:** The legacy identity remains usable\n\n**Why it matters:** Compatibility fixture\n\n**Fix:** Not fixed yet.\n\n## Log\n\n- 2026-11-01T00:00:01Z · Reported (legacy fixture)\n"
        ),
    );
}

fn manual_collision_root(title: &str, symptom: &str) -> (Fixture, String) {
    let fixture = Fixture::new();
    fixture.init();
    seed_migrated_legacy_bug(&fixture);
    fixture.commit("seed a migrated legacy bug");
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
    seed_migrated_legacy_bug(&fixture);
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

/// R8 (REQ-cli-012, REQ-cli-023): a link given by an entry's alias, such as the 0.5.x
/// ID a migrated entry keeps, is stored as the entry's canonical `<session>/<id>`, by
/// `bug add` and by `bug update`, as `--related` stores a bug's canonical name; so the
/// link survives the alias being dropped.
#[test]
fn a_link_by_an_entrys_alias_is_stored_as_its_canonical_session_qualified_id() {
    let fixture = Fixture::new();
    fixture.init();
    let session = fixture.active_id();
    let failed = entry(&fixture, "alias-fails", "failed", 1_793_000_000);
    let passed = entry(&fixture, "alias-passes", "passed", 1_793_000_100);
    for (id, alias) in [
        (&failed, "OBS-alias-fails-0123456789abcdef01234567"),
        (&passed, "OBS-alias-passes-0123456789abcdef01234567"),
    ] {
        let path = format!(".chough/sessions/{session}/evidence/{id}.md");
        let text = fixture.read(&path);
        fixture.write(
            &path,
            &text.replace("\naliases: []\n", &format!("\naliases: [{alias}]\n")),
        );
    }
    let added = json(&fixture.run(&[
        "bug",
        "add",
        "--title",
        "SYNTHETIC link by an alias",
        "--symptom",
        "SYNTHETIC symptom linked by an alias",
        "--trigger",
        "SYNTHETIC trigger linked by an alias",
        "--expected",
        "SYNTHETIC pass",
        "--actual",
        "SYNTHETIC fail",
        "--environment",
        "local",
        "--observation",
        "OBS-alias-fails-0123456789abcdef01234567",
    ]));
    let bug = added["id"].as_str().unwrap().to_string();
    let updated = fixture.ok(&[
        "bug",
        "update",
        &bug,
        "--observation",
        "OBS-alias-passes-0123456789abcdef01234567",
    ]);
    assert_eq!(
        updated["bug"]["evidence"],
        json!([format!("{session}/{failed}"), format!("{session}/{passed}")]),
        "{updated:#}"
    );
    let record = fixture.read(&bug_path(&bug));
    assert!(
        !record.contains("OBS-alias-"),
        "an alias was stored: {record}"
    );
    assert_eq!(
        fixture.ok(&["bug", "show", &bug])["verification"],
        "reproduced-and-later-passed"
    );
}

/// S2 (REQ-cli-012: "A migrated bug answers to its BUG-NNNN alias in list, search,
/// show, update and report"): `bug list` names each bug's aliases, and `bug search`
/// finds a bug by its alias or its name, first, before any word match; a search by
/// words alone is unchanged, and `bug add`'s duplicate check still compares words.
#[test]
fn a_migrated_bug_answers_to_its_alias_in_bug_list_and_bug_search() {
    let fixture = Fixture::new();
    fixture.init();
    seed_migrated_legacy_bug(&fixture);
    let listed = fixture.ok(&["bug", "list", "--all"]);
    let legacy = listed["bugs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|bug| bug["id"] == LEGACY)
        .unwrap_or_else(|| panic!("{listed:#}"))
        .clone();
    assert_eq!(legacy["aliases"], json!(["BUG-0019"]), "{legacy:#}");
    for query in ["BUG-0019", "what is BUG-0019 about?", LEGACY] {
        let found = fixture.ok(&["bug", "search", query]);
        let first = &found["matches"][0];
        assert_eq!(first["id"], LEGACY, "{query}: {found:#}");
        assert_eq!(first["matched"], "id or alias", "{query}: {found:#}");
    }
    // An alias that no bug holds finds nothing; words still score as before.
    assert_eq!(
        fixture.ok(&["bug", "search", "BUG-9999"])["matches"],
        json!([])
    );
    let words = fixture.ok(&["bug", "search", "legacy allocator sentinel"]);
    assert_eq!(words["matches"][0]["id"], LEGACY, "{words:#}");
    assert!(words["matches"][0].get("matched").is_none(), "{words:#}");
}

/// S5 (REQ-cli-023, REQ-cli-012): automatic failure capture links each failed attempt
/// as `bug add` links a session entry, `<session>/<id>`, both when it opens the bug for
/// the case and when it appends a later failure to it, and its Log names the same form,
/// so a merge that brings a same-named entry in another session cannot make it
/// ambiguous.
#[test]
fn automatic_failure_capture_links_each_failure_by_its_session_qualified_id() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.write(
        "specs/cli/requirements.md",
        "### REQ-cli-012\nLocal bug backlog.\n",
    );
    common::stand_in(
        &fixture.root().join("bin"),
        "failure-probe",
        "SYNTHETIC capture failure",
    );
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-9101\"\ntitle = \"Capture probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"FLOW-8\"]\nrequirements = [\"REQ-cli-012\"]\nenvironments = [\"test-local\"]\nartifacts = [\"test-log\"]\nexpected = [\"The probe reports its expected output\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"failure-probe\"]\nexpect_output = [\"SYNTHETIC output that is absent\"]\n",
    );
    fixture.init();
    fixture.commit("a capture fixture");
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "SYNTHETIC authorization for two captures",
    ]);
    let run = fixture.ok(&[
        "validate",
        "run",
        "TC-cli-9101",
        "--times",
        "2",
        "--environment",
        "test-local",
    ]);
    assert_eq!(run["run"]["failures"], 2, "{run}");
    let bug = run["captured_locally"][0].as_str().unwrap().to_string();
    let session = fixture.active_id();
    let record = fixture.read(&bug_path(&bug));
    let shown = fixture.ok(&["bug", "show", &bug]);
    let links: Vec<String> = shown["bug"]["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .map(|link| link.as_str().unwrap().to_string())
        .collect();
    assert_eq!(links.len(), 2, "{record}");
    for link in &links {
        let bare = link
            .strip_prefix(&format!("{session}/"))
            .unwrap_or_else(|| panic!("{link} is not qualified with {session}: {record}"));
        let entry = fixture.read(&format!(".chough/sessions/{session}/evidence/{bare}.md"));
        assert!(entry.contains("\nresult: failed\n"), "{entry}");
    }
    assert!(
        record.contains("Additional failure from")
            && record.contains(&format!("evidence: {}", links[1])),
        "the appended failure's Log line does not name {}: {record}",
        links[1]
    );
    for label in shown["observation_labels"].as_array().unwrap() {
        assert_eq!(label["label"], "entry", "{shown:#}");
    }
}

/// TC-cli-0125, rewritten for 0.6.0: independent roots name new bugs by date and title,
/// never by a number after the highest one, manual reports and automatic failure
/// capture alike; a migrated bug keeps answering to its 0.5.x ID as an alias in list,
/// search, show, update and report; and duplicate and related links resolve from alias
/// to new name and back.
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

    assert_ne!(manual_left_id, manual_right_id);
    assert_ne!(automatic_left_id, automatic_right_id);
    for id in [
        &manual_left_id,
        &manual_right_id,
        &automatic_left_id,
        &automatic_right_id,
    ] {
        assert_new_bug_name(id);
    }
    assert!(manual_left_id.ends_with("-manual-collision-alpha"));
    assert!(manual_right_id.ends_with("-manual-collision-beta"));
    assert!(
        automatic_left_id.contains("-tc-cli-9001-failed-during-"),
        "{automatic_left_id}"
    );
    for fixture in [
        &manual_left,
        &manual_right,
        &automatic_left,
        &automatic_right,
    ] {
        assert!(!fixture.exists(".chough/bugs/BUG-0020.md"));
        let legacy = fixture.read(&bug_path(LEGACY));
        assert!(legacy.contains("aliases: [BUG-0019]"), "{legacy}");
    }

    let shown = manual_left.ok(&["bug", "show", "BUG-0019"]);
    assert_eq!(shown["bug"]["id"], LEGACY);
    assert_eq!(shown["bug"]["aliases"], json!(["BUG-0019"]));
    assert!(
        manual_left.ok(&["bug", "list", "--all"])["bugs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|bug| bug["id"] == LEGACY)
    );
    assert!(
        manual_left.ok(&["bug", "search", "legacy allocator sentinel"])["matches"]
            .as_array()
            .unwrap()
            .iter()
            .any(|bug| bug["id"] == LEGACY)
    );
    let report = manual_left.run(&["bug", "report", "BUG-0019"]);
    assert!(report.status.success());
    let report = String::from_utf8_lossy(&report.stdout);
    assert!(
        report.contains("Legacy allocator sentinel") && report.contains("Also known as: BUG-0019"),
        "{report}"
    );
    let updated = manual_left.ok(&[
        "bug",
        "update",
        "BUG-0019",
        "--note",
        "Legacy content changed without renumbering",
        "--related",
        &manual_left_id,
    ]);
    assert_eq!(updated["bug"]["id"], LEGACY);
    assert_eq!(updated["bug"]["related"], json!([manual_left_id.clone()]));
    // A link written as the alias resolves to the new name, and back.
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
    assert_eq!(related["bug"]["related"], json!([LEGACY]));
    for alias in ["BUG-0019", LEGACY] {
        let again = manual_left.ok(&["bug", "update", &manual_left_id, "--related", alias]);
        assert_eq!(again["bug"]["related"], json!([LEGACY]), "{alias}");
    }
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
    assert_eq!(new_duplicate["bug"]["duplicate_of"], LEGACY);
    assert!(manual_left.exists(&bug_path(LEGACY)));
    assert!(manual_left.exists(&bug_path(&manual_left_id)));
    assert!(!manual_left.exists(".chough/bugs/BUG-0019.md"));
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
    let record = bug_record(&fixture, id);
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

/// `chough bug merge <base> <ours> <theirs>` on the three versions written to files.
fn merge_files(fixture: &Fixture, base: &str, ours: &str, theirs: &str) -> std::process::Output {
    for (name, text) in [("base", base), ("ours", ours), ("theirs", theirs)] {
        fixture.write(&format!("merge/{name}.md"), text);
    }
    let path = |name: &str| {
        fixture
            .root()
            .join(format!("merge/{name}.md"))
            .to_string_lossy()
            .into_owned()
    };
    fixture.run(&[
        "bug",
        "merge",
        &path("base"),
        &path("ours"),
        &path("theirs"),
    ])
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
    let original = fixture.read(&bug_path(&id));
    let updated = original
        .lines()
        .find_map(|line| line.strip_prefix("updated: "))
        .unwrap()
        .to_string();
    // Two later update times, ours before theirs.
    let (at_ours, at_theirs) = ("2099-01-01T00:00:01Z", "2099-01-01T00:00:02Z");
    let ours = format!(
        "{}- {at_ours} · status: open -> verified. Fixed on branch A\n",
        original
            .replace("status: open", "status: verified")
            .replace(
                &format!("updated: {updated}"),
                &format!("updated: {at_ours}")
            ),
    );
    let theirs = format!(
        "{}- {at_theirs} · evidence: 2026-11-10-seen-again. Seen again on branch B\n",
        original
            .replace("evidence: []", "evidence: [2026-11-10-seen-again]")
            .replace(
                &format!("updated: {updated}"),
                &format!("updated: {at_theirs}")
            ),
    );
    let output = merge_files(&fixture, &original, &ours, &theirs);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = fixture.read("merge/ours.md");
    let entries: Vec<&str> = text.lines().filter(|line| line.starts_with("- ")).collect();
    assert!(
        entries[entries.len() - 2].ends_with("Fixed on branch A")
            && entries[entries.len() - 1].ends_with("Seen again on branch B"),
        "{text}"
    );
    assert!(
        text.contains("\nevidence: [2026-11-10-seen-again]\n"),
        "{text}"
    );
    assert!(text.contains("\nstatus: verified\n"), "{text}");
    assert!(
        text.contains(&format!("\nupdated: {at_theirs}\n")),
        "{text}"
    );
    // Written the way bug commands write a record: their key order, one newline.
    let keys = |text: &str| -> Vec<String> {
        text[4..]
            .split_once("\n---\n")
            .unwrap()
            .0
            .lines()
            .filter_map(|line| line.split_once(':').map(|(key, _)| key.to_string()))
            .collect()
    };
    assert_eq!(keys(&text), keys(&original));
    assert!(text.ends_with('\n') && !text.ends_with("\n\n"));

    // Two different status changes are a human's call.
    let ours = original.replace("status: open", "status: investigating");
    let theirs = original.replace("status: open", "status: deferred");
    let output = merge_files(&fixture, &original, &ours, &theirs);
    assert!(!output.status.success(), "a status disagreement merged");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("status"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(fixture.read("merge/ours.md"), ours);
}

/// REQ-cli-012 and REQ-cli-023 (#138, audit assertion review): the CLI documentation says
/// in one sentence that Chough 0.5.x cannot read 0.6.0 bug records and names.
#[test]
fn the_cli_documentation_tells_older_versions_to_upgrade_for_new_form_bug_ids() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/cli.md");
    // Collapsing whitespace reads CRLF and LF checkouts alike (#131).
    let text = std::fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let upgrade: Vec<&str> = text
        .split(". ")
        .filter(|sentence| {
            sentence.contains("Chough 0.5.x")
                && sentence.contains("cannot read")
                && sentence.contains("0.6.0")
                && sentence.contains("bug")
        })
        .collect();
    assert_eq!(
        upgrade.len(),
        1,
        "docs/cli.md has no single note that Chough 0.5.x cannot read 0.6.0 bug records"
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

const STATUSES: [&str; 8] = [
    "open",
    "investigating",
    "fix-pending-verification",
    "verified",
    "deferred",
    "duplicate",
    "wont-fix",
    "fixed-upstream",
];
const CLOSED: [&str; 4] = ["verified", "duplicate", "wont-fix", "fixed-upstream"];

/// REQ-cli-054 (#153, audit Bugs row), with REQ-cli-012's `fixed-upstream`: add and
/// update accept each of the six classifications, three scopes and eight statuses, and
/// refuse any other value with the allowed list, leaving the record byte-identical. A
/// regression needs its baseline (see
/// regression_needs_a_passed_observation_at_an_earlier_ancestor_revision), verified its
/// evidence (see the verified tests) and fixed-upstream its note, so the first two are
/// set by hand here.
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
    let upstream = json(&fixture.run(&[
        "bug",
        "update",
        &id,
        "--status",
        "fixed-upstream",
        "--note",
        "Fixed upstream in SYNTHETIC-tool#1; checked by SYNTHETIC-agent on its main",
    ]));
    assert_eq!(upstream["bug"]["status"], "fixed-upstream");
    for (field, value) in [("classification", "regression"), ("status", "verified")] {
        set_bug_fields(&fixture, &id, &[(field, value)]);
        assert_eq!(bug_record(&fixture, &id)[field], value);
        set_bug_fields(
            &fixture,
            &id,
            &[("classification", "unclassified"), ("status", "open")],
        );
    }
    let before = fixture.read(&bug_path(&id));
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
            "status must be one of open, investigating, fix-pending-verification, verified, deferred, duplicate, wont-fix, fixed-upstream",
        ),
    ] {
        let refusal = fixture.fails(&["bug", "update", &id, flag, value]);
        assert!(refusal.contains(allowed), "{flag} {value}: {refusal}");
        assert_eq!(fixture.read(&bug_path(&id)), before);
    }
}

/// REQ-cli-054 (#153), with REQ-cli-012's `fixed-upstream`: of every scope and status,
/// only an open bug (any status but verified, duplicate, wont-fix or fixed-upstream)
/// with scope blocking is a blocking bug, and only it stops leaving test; closing it
/// releases the gate.
#[test]
fn only_an_open_blocking_bug_blocks_leaving_test() {
    let fixture = Fixture::new();
    fixture.init();
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    for scope in ["blocking", "in-scope", "out-of-scope"] {
        for status in STATUSES {
            set_bug_fields(&fixture, &id, &[("scope", scope), ("status", status)]);
            let blocking = fixture.ok(&["status", "--json"])["blocking_bugs"].clone();
            let expected = scope == "blocking" && !CLOSED.contains(&status);
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
        fixture.ok(&["stage", "advance", "--json"]);
    }
    assert_eq!(fixture.session()["stage"]["current"], "test");
    set_bug_fields(
        &fixture,
        &id,
        &[("scope", "blocking"), ("status", "deferred")],
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
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(fixture.session()["stage"]["current"], "accept");
}

/// REQ-cli-054 (#153): search scores word overlap of title, symptom and trigger, plus 0.25
/// for a shared criterion, requirement or case; it lists matches from 0.2 and refuses a
/// new bug only from 0.5, naming both ways forward. A distinct reason is recorded in the
/// new bug's Log and may not be blank.
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
    let distinct_id = distinct["id"].as_str().unwrap();
    // The same title on the same day gets one clash suffix (REQ-records-003).
    assert_eq!(distinct_id.len(), id.len() + 7, "{distinct_id}");
    assert!(distinct_id.starts_with(&format!("{id}-")), "{distinct_id}");
    assert!(
        log(&fixture, distinct_id).iter().any(
            |line| line.ends_with("Added despite similar reports: Only on the mobile checkout")
        ),
        "{:?}",
        log(&fixture, distinct_id)
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
    let path = bug_path(&id);
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
/// status, which clears `duplicate_of` and says so in its Log. Marking a bug verified
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
    let notes = reopened["bug"]["log"].to_string();
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
    let fails = entry(&fixture, "coupon-fails", "failed", 1_793_900_000);
    let passes = entry(&fixture, "coupon-passes", "passed", 1_793_900_001);
    fixture.ok(&["bug", "update", &other, "--observation", &fails]);
    let verified = json(&fixture.run(&[
        "bug",
        "update",
        &other,
        "--observation",
        &passes,
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
    let shipping_fails = entry(&fixture, "shipping-fails", "failed", 1_793_000_000);
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
            &shipping_fails,
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
        "Found by: manual report".into(),
        format!("Revision: {revision}"),
        "Environment: test-local".into(),
        "Fixture: repro/coupon.sh".into(),
        "What happened: Order of exactly 50.00 is charged shipping".into(),
        "Trigger: Checkout total equal to the free shipping threshold".into(),
        "Steps:\n1. Open checkout\n2. Enter 50.00".into(),
        "Expected: Free shipping at 50.00".into(),
        "Observed: Shipping fee of 4.99 applied".into(),
        "Why it matters: Every order at the threshold".into(),
        "Fix: Not fixed yet.; fixed in: none recorded".into(),
        format!("Evidence: {}/{shipping_fails}", fixture.active_id()),
        format!("Related: {other}; duplicate of: none"),
        "Not published".into(),
    ] {
        assert!(report.contains(&shown), "missing {shown:?}:\n{report}");
    }
    let empty =
        String::from_utf8_lossy(&fixture.run(&["bug", "report", &other]).stdout).into_owned();
    for shown in [
        "Traceability: HI none recorded; requirements none recorded; cases none recorded",
        "Also known as: none recorded",
        "Fixture: none recorded",
        "Steps:\nNot recorded.",
        "Evidence: none recorded",
        "Related: none recorded; duplicate of: none",
    ] {
        assert!(empty.contains(shown), "missing {shown:?}:\n{empty}");
    }
}

/// TC-cli-bugs-are-markdown (REQ-cli-012, REQ-records-001, REQ-records-003): a bug is one
/// Markdown record with the listed front matter, `# title`, the five paragraphs and an
/// append-only Log. Each update appends one Log entry and keeps every earlier Log line
/// and every paragraph a person wrote byte for byte; a changed title never renames the
/// file; a migrated bug answers to its alias; evidence moved into an archived story's
/// receipt keeps the bug's verified state; and `fixed-upstream` needs a note, closes the
/// bug and never reads as verified.
#[test]
fn bugs_are_markdown_with_an_append_only_log() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");

    // The record: front matter, title, paragraphs, Log; no JSON twin.
    let added = json(&add_bug(&fixture, &["--step", "Open checkout"]));
    let id = added["id"].as_str().unwrap().to_string();
    assert_new_bug_name(&id);
    assert!(
        id.ends_with("-shipping-is-charged-at-exactly-the-free-threshold"),
        "{id}"
    );
    let text = fixture.read(&bug_path(&id));
    assert!(
        text.starts_with(&format!("---\nkind: bug\nid: {id}\naliases: []\n")),
        "{text}"
    );
    for key in [
        "title",
        "status",
        "classification",
        "scope",
        "impact",
        "criteria",
        "requirements",
        "cases",
        "evidence",
        "receipts",
        "environment",
        "revision",
        "found_by",
        "related",
        "created",
        "updated",
    ] {
        assert!(text.contains(&format!("\n{key}:")), "no {key} in\n{text}");
    }
    let body = text.split_once("\n---\n").unwrap().1;
    assert!(
        body.starts_with(
            "# Shipping is charged at exactly the free threshold\n\n**What happened:** Order of exactly 50.00 is charged shipping\n\nActual: Shipping fee of 4.99 applied\n\n**Steps:** Trigger: Checkout total equal to the free shipping threshold\n1. Open checkout\n\n**Expected:** Free shipping at 50.00\n\n**Why it matters:** Not yet assessed.\n\n**Fix:** Not fixed yet.\n\n## Log\n\n- "
        ),
        "{body}"
    );
    let names: Vec<String> = std::fs::read_dir(fixture.root().join(".chough/bugs"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    assert_eq!(names, vec![format!("{id}.md")]);

    // A person's edits: the Steps paragraph, the title and a section of their own.
    let edited =
        text.replace(
            "1. Open checkout\n",
            "1. Open checkout\n2. Pay with a saved card (person's words)\n",
        )
        .replace(
            "title: Shipping is charged at exactly the free threshold",
            "title: Shipping charged at the threshold",
        ) + "\n## Notes\n\nA person's own section.\n";
    fixture.write(&bug_path(&id), &edited);
    let failure = entry(&fixture, "threshold-fails", "failed", 1_793_000_000);
    let pass = entry(&fixture, "threshold-passes", "passed", 1_793_000_100);
    let mut previous = log(&fixture, &id);
    for args in [
        vec!["--observation", failure.as_str()],
        vec!["--status", "investigating", "--note", "Looking at rounding"],
        vec!["--observation", pass.as_str(), "--status", "verified"],
    ] {
        let mut command = vec!["bug", "update", id.as_str()];
        command.extend(args);
        fixture.ok(&command);
        let now = fixture.read(&bug_path(&id));
        let entries = log(&fixture, &id);
        assert_eq!(entries.len(), previous.len() + 1, "{now}");
        assert_eq!(entries[..previous.len()], previous[..], "{now}");
        // Every paragraph a person wrote is unchanged; marking the bug verified with no
        // Fix set names what verified it in Fix (REQ-cli-012).
        let write_up = |text: &str| {
            text.split_once("\n---\n")
                .unwrap()
                .1
                .split_once("## Log")
                .unwrap()
                .0
                .to_string()
        };
        let expected = match now
            .lines()
            .find(|line| line.starts_with("**Fix:** Verified: "))
        {
            Some(fix) => write_up(&edited).replace("**Fix:** Not fixed yet.", fix),
            None => write_up(&edited),
        };
        assert_eq!(write_up(&now), expected);
        assert!(
            now.ends_with("\n## Notes\n\nA person's own section.\n"),
            "{now}"
        );
        previous = entries;
    }
    let last = previous.last().unwrap();
    assert!(
        last.contains("status: investigating -> verified") && last.contains(&pass),
        "{last}"
    );
    assert!(fixture.exists(&bug_path(&id)), "the bug was renamed");
    assert_eq!(
        bug_record(&fixture, &id)["title"],
        "Shipping charged at the threshold"
    );

    // The session holding the evidence is archived: the bug resolves each entry to the
    // story's receipt, and its verified state is unchanged.
    let session = fixture.active_id();
    let receipt = |id: &str, result: &str, seconds: u64| {
        format!(
            "  - id: {id}\n    aliases: OBS-{result}-000000000000000000000001\n    claim: Observed the shipping threshold check\n    result: {result}\n    environment: test-local\n    observed: {}\n",
            iso(seconds)
        )
    };
    fixture.write(
        &format!(".chough/archive/sessions/{session}.md"),
        &format!(
            "---\nkind: session\nid: {session}\nstage: done\nclosed: 2026-11-10\noutcome: done\nreceipts:\n{}{}---\n# Ship a focused change\n\nClosed 2026-11-10: done.\n",
            receipt(&failure, "failed", 1_793_000_000),
            receipt(&pass, "passed", 1_793_000_100)
        ),
    );
    std::fs::remove_dir_all(fixture.root().join(format!(".chough/sessions/{session}"))).unwrap();
    let shown = fixture.ok(&["bug", "show", &id]);
    assert_eq!(
        shown["verification"], "reproduced-and-later-passed",
        "{shown:#}"
    );
    assert_eq!(
        shown["observation_labels"][0]["label"], "story-receipt",
        "{shown:#}"
    );
    assert_eq!(
        shown["observation_labels"][0]["place"],
        format!("{session}/{failure}")
    );
    // A receipt's alias resolves too.
    let other = add_other_bug(&fixture);
    let linked = fixture.ok(&[
        "bug",
        "update",
        &other,
        "--observation",
        "OBS-failed-000000000000000000000001",
    ]);
    assert_eq!(linked["verification"], "reproduced");

    // fixed-upstream needs a note, closes the bug, and nothing reports it verified.
    let before = fixture.read(&bug_path(&other));
    let refusal = fixture.fails(&["bug", "update", &other, "--status", "fixed-upstream"]);
    assert!(refusal.contains("fixed-upstream needs --note"), "{refusal}");
    assert_eq!(fixture.read(&bug_path(&other)), before);
    let closed = fixture.ok(&[
        "bug",
        "update",
        &other,
        "--status",
        "fixed-upstream",
        "--note",
        "Fixed upstream in SYNTHETIC-tool#7; checked by SYNTHETIC-agent on its main",
    ]);
    assert_eq!(closed["bug"]["status"], "fixed-upstream");
    assert_ne!(closed["verification"], "reproduced-and-later-passed");
    let open = fixture.ok(&["bug", "list"]);
    assert!(!open.to_string().contains(&other), "{open:#}");
    let all = fixture.ok(&["bug", "list", "--all"]);
    let listed = all["bugs"]
        .as_array()
        .unwrap()
        .iter()
        .find(|bug| bug["id"] == other)
        .unwrap();
    assert_eq!(listed["status"], "fixed-upstream");
    assert_eq!(listed["closed_as"], "fixed upstream");
    assert!(!listed.to_string().contains("\"verified\""), "{listed}");

    // A 0.5.x JSON bug is refused as unmigrated, naming `chough migrate`.
    fixture.write(".chough/bugs/BUG-0001.json", "{}\n");
    let refusal = fixture.fails(&["bug", "list"]);
    assert!(
        refusal.contains("refused:unmigrated") && refusal.contains("chough migrate"),
        "{refusal}"
    );
}

/// TC-records-actor-recorded, the bug part (REQ-records-007): `bug add` records the
/// acting agent as front matter `actor` and in its Log entry; with only `CLAUDECODE` and
/// `HC_TERMINAL_ID` it records `claude-code` and that terminal; with none, no `actor`;
/// and a terminal holding a home path is left out with a warning while the bug is added.
#[test]
fn bugs_record_the_acting_agent() {
    let fixture = Fixture::new();
    fixture.init();
    let add = |title: &str, environment: &[(&str, &str)]| -> (String, String) {
        let output = fixture.run_with_env(
            &[
                "bug",
                "add",
                "--title",
                title,
                "--symptom",
                &format!("{title} symptom"),
                "--trigger",
                "SYNTHETIC trigger",
                "--expected",
                "SYNTHETIC expected",
                "--actual",
                "SYNTHETIC actual",
                "--environment",
                "test-local",
                "--distinct",
                "SYNTHETIC distinct",
            ],
            environment,
        );
        let id = json(&output)["id"].as_str().unwrap().to_string();
        (
            fixture.read(&bug_path(&id)),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        )
    };
    let (text, _) = add(
        "Overridden actor",
        &[
            ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
            ("CHOUGH_ACTOR_MODEL", "SYNTHETIC-model"),
            ("CHOUGH_HC_TERMINAL", "t999"),
            ("CLAUDECODE", "1"),
            ("HC_TERMINAL_ID", "t111"),
        ],
    );
    assert!(
        text.contains(
            "\nactor:\n  - agent: SYNTHETIC-agent\n    model: SYNTHETIC-model\n    terminal: t999\n"
        ),
        "{text}"
    );
    // `found_by` and each Log entry show the actor as one line for people, in whatever
    // form Actor::display gives, with every known part.
    let found_by = text
        .lines()
        .find_map(|line| line.strip_prefix("found_by: "))
        .unwrap_or_else(|| panic!("{text}"));
    assert!(
        found_by.starts_with("SYNTHETIC-agent")
            && found_by.contains("SYNTHETIC-model")
            && found_by.contains("t999")
            && found_by.ends_with(" (manual report)"),
        "{text}"
    );
    let entries: Vec<&str> = text.lines().filter(|line| line.starts_with("- ")).collect();
    assert!(
        !entries.is_empty()
            && entries.iter().all(|entry| {
                entry
                    .split_once(" · by ")
                    .is_some_and(|(_, by)| by.starts_with("SYNTHETIC-agent") && by.contains("t999"))
            }),
        "{text}"
    );
    let (text, _) = add(
        "Detected actor",
        &[("CLAUDECODE", "1"), ("HC_TERMINAL_ID", "t222")],
    );
    assert!(
        text.contains("\nactor:\n  - agent: claude-code\n    terminal: t222\n"),
        "{text}"
    );
    let (text, _) = add("Unknown actor", &[]);
    assert!(!text.contains("\nactor:"), "{text}");
    assert!(!text.contains(" · by "), "{text}");
    let (text, warning) = add(
        "Unsafe terminal",
        &[
            ("CHOUGH_ACTOR", "SYNTHETIC-agent"),
            ("CHOUGH_HC_TERMINAL", "/Users/SYNTHETIC-user/t1"),
        ],
    );
    assert!(warning.contains("terminal"), "{warning}");
    assert!(
        text.contains("\nactor:\n  - agent: SYNTHETIC-agent\nrelated:"),
        "{text}"
    );
    assert!(!text.contains("/Users/"), "{text}");
}

/// The bug part of TC-records-leak-guard (REQ-records-006): a bug note holding a home
/// path refuses as `refused:leak`, naming the field and kind and never the path, with the
/// file unchanged; with `--redact` it is stored as `<redacted:home-path>` and counted.
#[test]
fn bug_writes_refuse_or_redact_leaks() {
    let fixture = Fixture::new();
    fixture.init();
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let before = fixture.read(&bug_path(&id));
    let path = "/Users/SYNTHETIC-user/project/log.txt";
    let refusal = fixture.fails(&["bug", "update", &id, "--note", &format!("See {path}")]);
    assert!(
        refusal.contains("refused:leak") && refusal.contains("body"),
        "{refusal}"
    );
    assert!(!refusal.contains(path), "{refusal}");
    assert_eq!(fixture.read(&bug_path(&id)), before);
    let output = fixture.run(&[
        "--redact",
        "bug",
        "update",
        &id,
        "--note",
        &format!("See {path}"),
    ]);
    let redacted = json(&output);
    assert_eq!(redacted["redacted"], 1, "{redacted:#}");
    let text = fixture.read(&bug_path(&id));
    assert!(text.contains("See <redacted:home-path>"), "{text}");
    assert!(!text.contains(path), "{text}");
    // A title that leaks is refused before anything is named or written.
    let output = fixture.run(&[
        "bug",
        "add",
        "--title",
        path,
        "--symptom",
        "SYNTHETIC",
        "--trigger",
        "SYNTHETIC",
        "--expected",
        "SYNTHETIC",
        "--actual",
        "SYNTHETIC",
        "--environment",
        "test-local",
    ]);
    assert!(!output.status.success());
    assert_eq!(
        std::fs::read_dir(fixture.root().join(".chough/bugs"))
            .unwrap()
            .count(),
        1
    );
}

/// TC-cli-markdown-bug-merge (REQ-cli-047): with `chough bug merge` declared as the
/// merge driver of `.chough/bugs/*.md`, two branches that each update one bug, one with a
/// note and a status change and the other with a note, an evidence link and an edit to
/// the Fix paragraph, merge in Git with no conflict: both Log entries, the evidence, the
/// status, the Fix edit and the later update time. Two different status changes stop
/// the merge for a human, with ours in the working tree.
#[cfg(unix)]
#[test]
fn bug_merge_unions_logs_and_takes_one_sided_edits() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(".gitattributes", ".chough/bugs/*.md merge=chough-bug\n");
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    fixture.commit("base");
    let driver = format!("{} bug merge %O %A %B", env!("CARGO_BIN_EXE_chough"));
    fixture.git(&["config", "merge.chough-bug.driver", &driver]);
    fixture.git(&["checkout", "-q", "-b", "a"]);
    fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "investigating",
        "--note",
        "Branch A looked",
    ]);
    fixture.commit("a");
    fixture.git(&["checkout", "-q", "main"]);
    fixture.git(&["checkout", "-q", "-b", "b"]);
    let seen = entry(&fixture, "seen-on-b", "failed", 1_793_000_000);
    fixture.ok(&[
        "bug",
        "update",
        &id,
        "--observation",
        &seen,
        "--note",
        "Branch B saw it",
    ]);
    let text = fixture.read(&bug_path(&id));
    fixture.write(
        &bug_path(&id),
        &text.replace("**Fix:** Not fixed yet.", "**Fix:** Round once."),
    );
    fixture.commit("b");
    fixture.git(&["checkout", "-q", "a"]);
    fixture.git(&["merge", "-q", "--no-edit", "b"]);
    let merged = fixture.read(&bug_path(&id));
    let shown = fixture.ok(&["bug", "show", &id]);
    assert_eq!(shown["bug"]["status"], "investigating", "{merged}");
    assert_eq!(
        shown["bug"]["evidence"],
        json!([format!("{}/{seen}", fixture.active_id())]),
        "{merged}"
    );
    assert_eq!(shown["bug"]["fix"], "Round once.", "{merged}");
    let entries = log(&fixture, &id);
    assert_eq!(entries.len(), 3, "{merged}");
    assert!(
        entries[1].contains("Branch A looked") && entries[2].contains("Branch B saw it"),
        "{merged}"
    );
    assert!(!merged.contains("<<<<<<<"), "{merged}");

    // A real disagreement stays a conflict, with ours kept in the working tree.
    fixture.git(&["checkout", "-q", "-b", "c"]);
    fixture.ok(&["bug", "update", &id, "--status", "deferred"]);
    fixture.commit("c");
    fixture.git(&["checkout", "-q", "a"]);
    fixture.ok(&["bug", "update", &id, "--status", "wont-fix"]);
    fixture.commit("a2");
    let ours = fixture.read(&bug_path(&id));
    let output = std::process::Command::new("git")
        .current_dir(fixture.root())
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
            "merge",
            "-q",
            "--no-edit",
            "c",
        ])
        .output()
        .unwrap();
    assert!(!output.status.success(), "a status disagreement merged");
    assert_eq!(fixture.read(&bug_path(&id)), ours);
}

/// REQ-cli-012: a new bug's Steps start with its trigger, labelled `Trigger:`, then
/// each numbered step; the trigger reads back without the label.
#[test]
fn a_new_bugs_steps_start_with_the_labelled_trigger() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    let added = json(&add_bug(&fixture, &["--step", "Open checkout"]));
    let id = added["id"].as_str().unwrap().to_string();
    let text = fixture.read(&bug_path(&id));
    assert!(
        text.contains(
            "\n**Steps:** Trigger: Checkout total equal to the free shipping threshold\n1. Open checkout\n\n"
        ),
        "{text}"
    );
    let bug = bug_record(&fixture, &id);
    assert_eq!(
        bug["trigger"],
        "Checkout total equal to the free shipping threshold"
    );
    assert_eq!(bug["steps"], json!(["Open checkout"]));
}

/// REQ-cli-012: `bug update --fix <text>` sets the Fix paragraph and logs each set; a
/// Fix that is set stays when the bug is later verified.
#[test]
fn bug_update_fix_sets_the_fix_paragraph_and_logs_each_set() {
    let fixture = ordered_bug(&[("the-failure", "failed", 1_793_000_000)]);
    let id = only_bug_id(&fixture);
    let first = fixture.ok(&[
        "bug",
        "update",
        &id,
        "--fix",
        "Round the total once, at the end",
    ]);
    assert_eq!(first["bug"]["fix"], "Round the total once, at the end");
    fixture.ok(&["bug", "update", &id, "--fix", "Round once, at checkout"]);
    assert_eq!(bug_record(&fixture, &id)["fix"], "Round once, at checkout");
    let text = fixture.read(&bug_path(&id));
    assert!(
        text.contains("\n**Fix:** Round once, at checkout\n\n## Log\n"),
        "{text}"
    );
    let log = log(&fixture, &id);
    for fix in [
        "Round the total once, at the end",
        "Round once, at checkout",
    ] {
        assert!(
            log.iter().any(|line| line.contains(&format!("fix: {fix}"))),
            "no Log entry for {fix}: {log:?}"
        );
    }
    let pass = entry(&fixture, "the-pass", "passed", 1_793_000_100);
    fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "verified",
        "--observation",
        &pass,
    ]);
    assert_eq!(bug_record(&fixture, &id)["fix"], "Round once, at checkout");
}

/// REQ-cli-012: marking a bug verified while its Fix still says `Not fixed yet.`
/// replaces that with a line naming the passing observation and its revision.
#[test]
fn verifying_a_bug_with_no_fix_names_the_passing_observation_and_its_revision() {
    let fixture = ordered_bug(&[("the-failure", "failed", 1_793_000_000)]);
    let id = only_bug_id(&fixture);
    assert_eq!(bug_record(&fixture, &id)["fix"], "Not fixed yet.");
    let pass = entry(&fixture, "the-pass", "passed", 1_793_000_100);
    fixture.ok(&[
        "bug",
        "update",
        &id,
        "--status",
        "verified",
        "--observation",
        &pass,
    ]);
    let bug = bug_record(&fixture, &id);
    let linked = bug["evidence"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .find(|link| link.ends_with(&pass))
        .unwrap_or_else(|| panic!("{pass} is not linked: {bug}"))
        .to_string();
    let revision = head(&fixture).unwrap();
    assert_eq!(
        bug["fix"],
        format!("Verified: {linked} passed at {revision}.")
    );
    let text = fixture.read(&bug_path(&id));
    assert!(!text.contains("Not fixed yet."), "{text}");
}

/// REQ-cli-012, F8 of the second fresh-eyes round: `bug add` and `bug update` given
/// `--observation` with an entry of the active session add the bug to that session's
/// checkpoint once, so its archived story names the bugs found in it; one naming another
/// session's entry leaves the active checkpoint unchanged.
#[test]
fn a_bug_linked_to_an_active_sessions_entry_is_in_its_checkpoint_and_story() {
    let fixture = Fixture::new();
    fixture.init();
    // Entries carry HEAD as their revision, which the archived story's receipts need.
    fixture.commit("a project");
    let session = fixture.active_id();
    let failed = entry(&fixture, "found-fails", "failed", 1_793_000_000);
    let passed = entry(&fixture, "found-passes", "passed", 1_793_000_100);
    let add = |title: &str, extra: &[&str]| {
        let mut args = vec![
            "bug",
            "add",
            "--title",
            title,
            "--symptom",
            title,
            "--trigger",
            title,
            "--expected",
            "SYNTHETIC pass",
            "--actual",
            "SYNTHETIC fail",
            "--environment",
            "local",
        ];
        args.extend(extra);
        json(&fixture.run(&args))["id"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let bugs = || fixture.ok(&["checkpoint"])["checkpoint"]["bugs"].clone();
    let first = add(
        "SYNTHETIC checkout charges shipping at the threshold",
        &["--observation", &failed],
    );
    assert_eq!(bugs(), json!([first]));
    let second = add("SYNTHETIC receipt email omits the order number", &[]);
    assert_eq!(bugs(), json!([first]));
    for bug in [&second, &first] {
        fixture.ok(&["bug", "update", bug, "--observation", &passed]);
    }
    assert_eq!(bugs(), json!([first, second]));

    fixture.ok(&["session", "new", "beta", "--objective", "SYNTHETIC beta"]);
    let beta = fixture.session_file_of("beta");
    let before = fixture.read(&beta);
    add(
        "SYNTHETIC search ignores accented letters",
        &["--observation", &format!("{session}/{failed}")],
    );
    assert_eq!(fixture.read(&beta), before, "the active checkpoint changed");

    fixture.commit("the bugs");
    fixture.ok(&["session", "close", &session, "--reason", "SYNTHETIC closed"]);
    let story = fixture.read(&format!(".chough/archive/sessions/{session}.md"));
    for bug in [&first, &second] {
        assert!(story.contains(bug.as_str()), "{bug} missing from\n{story}");
    }
}

/// Fresh-eyes round 3, F6 (2026-11-12; REQ-cli-054): a duplicate `bug add` printed
/// "similar bugs exist: [{\"id\":...,\"score\":0.71,...}]", a raw JSON array. Each similar
/// bug is a readable line with its ID, title and status, and the refusal keeps both ways
/// forward: append evidence to one, or add with --distinct.
#[test]
fn a_duplicate_bug_add_lists_each_similar_bug_as_a_line() {
    let fixture = Fixture::new();
    fixture.init();
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let refused = add_bug(&fixture, &[]);
    assert!(!refused.status.success());
    let error = String::from_utf8_lossy(&refused.stderr).into_owned();
    assert!(error.contains("similar bugs exist"), "{error}");
    assert!(
        error.lines().any(|line| line.trim()
            == format!("{id}: Shipping is charged at exactly the free threshold (open)")),
        "{error}"
    );
    for raw in ["[{", "\"id\"", "\"score\""] {
        assert!(!error.contains(raw), "raw JSON {raw}: {error}");
    }
    for way in [
        "chough bug update <id> --observation <id> --note <text>",
        "--distinct <why this is a different defect>",
    ] {
        assert!(error.contains(way), "{way}: {error}");
    }
}

/// t38n's round-4 F8 (REQ-cli-012): a bug's actual behaviour stands as its own paragraph
/// under What happened, so Markdown never runs it into the symptom, and both read back;
/// a bug written with `Actual:` on the symptom's next line still reads back the same.
#[test]
fn a_bug_body_gives_its_actual_line_its_own_paragraph() {
    let fixture = Fixture::new();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    let id = json(&add_bug(&fixture, &[]))["id"]
        .as_str()
        .unwrap()
        .to_string();
    let text = fixture.read(&bug_path(&id));
    assert!(
        text.contains(
            "**What happened:** Order of exactly 50.00 is charged shipping\n\nActual: Shipping fee of 4.99 applied\n\n**Steps:** "
        ),
        "{text}"
    );
    let shown = bug_record(&fixture, &id);
    assert_eq!(
        shown["symptom"],
        "Order of exactly 50.00 is charged shipping"
    );
    assert_eq!(shown["actual"], "Shipping fee of 4.99 applied");

    // A bug written before 0.6.1, with Actual on the symptom's next line.
    seed_migrated_legacy_bug(&fixture);
    let legacy = bug_record(&fixture, LEGACY);
    assert_eq!(legacy["symptom"], "Seeded legacy record remains readable");
    assert_eq!(legacy["actual"], "The legacy identity is present");
}
