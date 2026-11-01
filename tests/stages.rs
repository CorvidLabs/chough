//! Checkpoint schema 2, explicit migration and stage routing (REQ-cli-007, REQ-cli-008).

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::fs;

fn transitions(fixture: &Fixture) -> Vec<Value> {
    fixture
        .read(".chough/transitions.jsonl")
        .lines()
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}

fn stage(fixture: &Fixture) -> String {
    fixture.session()["stage"]["current"]
        .as_str()
        .unwrap()
        .to_string()
}

#[test]
fn init_writes_a_schema_2_checkpoint_at_intent() {
    let fixture = Fixture::new();
    fixture.init();
    let session = fixture.session();
    assert_eq!(session["schema_version"], 2);
    assert_eq!(session["stage"]["current"], "intent");
    assert_eq!(session["stage"]["status"], "active");
    let shown = fixture.ok(&["stage", "show"]);
    assert!(
        shown["valid_transitions"][0]
            .as_str()
            .unwrap()
            .contains("intent -> design")
    );
}

#[test]
fn every_stage_is_documented_in_the_shared_stage_contract() {
    let fixture = Fixture::new();
    fixture.init();
    let reference = fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("skills/chough-workflow/references/stages.md"),
    )
    .unwrap()
    // Windows checkouts may convert line endings; headings are compared as lines.
    .replace("\r\n", "\n");
    let stages = fixture.ok(&["stage", "show"])["stages"].clone();
    assert_eq!(stages.as_array().unwrap().len(), 11);
    for stage in stages.as_array().unwrap() {
        let heading = format!("\n## {}\n", stage["name"].as_str().unwrap());
        assert!(reference.contains(&heading), "missing {heading:?}");
    }
}

#[test]
fn migration_is_explicit_and_preserves_schema_1_bytes() {
    let fixture = Fixture::new();
    let original = "{\n  \"schema_version\": 1,\n  \"objective\": \"Keep my old session\",\n  \"intent_scope\": \"LIFE\",\n  \"modules\": [\"cli\"],\n  \"decisions\": [\"Decided A because B\"],\n  \"open_questions\": [\"Q1\"],\n  \"next_action\": \"Do the next thing\"\n}\n";
    fixture.write(".chough/session.json", original);
    // Schema 1 remains readable for context and status, but not for stage changes.
    assert_eq!(
        fixture.ok(&["context", "--json"])["session"]["schema_version"],
        1
    );
    assert!(
        fixture.ok(&["status", "--json"])["checkpoint"]["migration"]
            .as_str()
            .unwrap()
            .contains("chough migrate")
    );
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("chough migrate")
    );
    assert_eq!(fixture.read(".chough/session.json"), original);

    let migrated = fixture.ok(&["migrate", "--stage", "plan"]);
    assert_eq!(migrated["migrated"], true);
    assert_eq!(fixture.read(".chough/session.v1.json"), original);
    let session = fixture.session();
    assert_eq!(session["schema_version"], 2);
    assert_eq!(session["stage"]["current"], "plan");
    assert_eq!(session["decisions"], json!(["Decided A because B"]));
    assert_eq!(session["open_questions"], json!(["Q1"]));
    assert_eq!(session["next_action"], "Do the next thing");
    assert_eq!(fixture.ok(&["migrate"])["migrated"], false);

    // A second schema 1 file never overwrites the preserved backup.
    fixture.write(".chough/session.json", original);
    assert!(fixture.fails(&["migrate"]).contains("already exists"));
    assert_eq!(fixture.read(".chough/session.json"), original);
}

#[test]
fn checkpoint_round_trips_across_processes_and_rejects_unknown_fields() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "Use TOML because lists are first-class",
        "--question",
        "Which viewport matters most?",
        "--criterion",
        "LIFE-3",
        "--requirement",
        "REQ-cli-002",
        "--design",
        "design/prototype/index.html",
        "--awaiting-human",
        "Scope approval of add-thing",
        "--next-action",
        "Wait for approval",
    ]);
    let resumed = fixture.ok(&["context", "--json"])["session"].clone();
    assert_eq!(resumed["references"]["criteria"], json!(["LIFE-3"]));
    assert_eq!(
        resumed["awaiting_human"],
        json!(["Scope approval of add-thing"])
    );
    assert_eq!(resumed["next_action"], "Wait for approval");
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        next["next_valid_action"]["action"]
            .as_str()
            .unwrap()
            .contains("Waiting for a human decision")
    );
    fixture.ok(&[
        "checkpoint",
        "--resolve-question",
        "Which viewport matters most?",
        "--clear-awaiting-human",
    ]);
    assert_eq!(fixture.session()["open_questions"], json!([]));
    assert!(
        fixture
            .fails(&["checkpoint", "--design", "../outside.html"])
            .contains("traversal")
    );

    let mut session = fixture.session();
    session["hidden_reasoning"] = json!("must not be accepted");
    fixture.write(".chough/session.json", &session.to_string());
    assert!(fixture.fails(&["context", "--json"]).contains("invalid"));
}

#[test]
fn transitions_cover_every_kind_and_keep_history() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance", "--note", "Intent approved by the human"]);
    assert_eq!(stage(&fixture), "design");
    assert!(
        fixture
            .fails(&["stage", "skip", "--reason", "   "])
            .contains("reason")
    );
    fixture.ok(&[
        "stage",
        "skip",
        "--reason",
        "CLI-only change; nothing visual",
    ]);
    assert_eq!(stage(&fixture), "plan");
    let iterated = fixture.ok(&["stage", "iterate", "--note", "Second estimate"]);
    assert_eq!(iterated["applied"]["iteration"], 2);
    let built = fixture.ok(&["stage", "advance"]);
    assert_eq!(built["applied"]["gate"], "ungated:not-configured");
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "test");

    let back = fixture.ok(&[
        "stage",
        "back",
        "--to",
        "plan",
        "--reason",
        "Test exposed a missing requirement",
    ]);
    let refresh = back["applied"]["refresh"].to_string();
    for word in ["plan:", "build:", "test:"] {
        assert!(refresh.contains(word), "{refresh}");
    }
    assert!(
        fixture
            .fails(&["stage", "back", "--to", "test", "--reason", "x"])
            .contains("earlier")
    );

    fixture.ok(&["stage", "pause", "--reason", "Waiting on the human"]);
    assert!(fixture.fails(&["stage", "advance"]).contains("paused"));
    assert_eq!(
        fixture.ok(&["stage", "show"])["valid_transitions"],
        json!(["chough stage resume"])
    );
    fixture.ok(&["stage", "resume"]);

    fixture.ok(&[
        "stage",
        "enter",
        "librarian",
        "--reason",
        "Record a decision",
    ]);
    assert_eq!(stage(&fixture), "librarian");
    assert_eq!(
        fixture.session()["stage"]["callers"][0]["stage"],
        json!("plan")
    );
    assert!(fixture.fails(&["stage", "advance"]).contains("return"));
    fixture.ok(&["stage", "enter", "bugs", "--reason", "Nested capture"]);
    fixture.ok(&["stage", "return"]);
    fixture.ok(&["stage", "return", "--note", "Decision recorded"]);
    assert_eq!(stage(&fixture), "plan");

    fixture.ok(&["stage", "reset", "--reason", "The human changed direction"]);
    assert_eq!(stage(&fixture), "intent");

    let kinds: Vec<String> = transitions(&fixture)
        .iter()
        .map(|transition| transition["kind"].as_str().unwrap().to_string())
        .collect();
    assert_eq!(
        kinds,
        [
            "advance", "skip", "iterate", "advance", "advance", "back", "pause", "resume", "enter",
            "enter", "return", "return", "reset"
        ]
    );
    assert_eq!(
        transitions(&fixture)[1]["reason"],
        "CLI-only change; nothing visual"
    );
}

#[test]
fn optional_side_stages_and_release_need_authorization() {
    let fixture = Fixture::new();
    fixture.init();
    assert!(
        fixture
            .fails(&["stage", "enter", "validate", "--reason", "Repeat the check"])
            .contains("consent")
    );
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: yes, repeat it this time",
    ]);
    assert!(!fixture.exists(".chough/preferences.json"));
    fixture.ok(&["stage", "enter", "validate", "--reason", "Repeat the check"]);
    fixture.ok(&["stage", "return"]);

    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    assert_eq!(stage(&fixture), "done");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("release is entered only")
    );
    assert!(
        fixture
            .fails(&["stage", "enter", "release", "--reason", "Ship it"])
            .contains("not authorized")
    );
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "Human: please publish 0.2.0",
    ]);
    let entered = fixture.ok(&["stage", "enter", "release", "--reason", "Requested"]);
    assert!(
        entered["applied"]["authority"]
            .as_str()
            .unwrap()
            .starts_with("run consent")
    );
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "done");

    // A new objective ends run consents; standing preferences were never written.
    fixture.ok(&["checkpoint", "--objective", "Next piece of work"]);
    assert_eq!(fixture.session()["run_consents"], json!([]));
    assert!(!fixture.exists(".chough/preferences.json"));
}

#[cfg(unix)]
#[test]
fn native_state_gates_build_and_closing_without_inventing_it() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:no-native-change")
    );
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    fixture.no_spec_approval("add-thing");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert!(refused.contains("specsync change status add-thing"));
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["checkpoint"]["gate"]["allowed"], false);
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["gate"]["allowed"], false);
    assert!(
        next["next_valid_action"]["action"]
            .as_str()
            .unwrap()
            .contains("Gate not open")
    );

    fixture.write(".stub-fail", "");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-unknown")
    );
    fs::remove_file(fixture.root().join(".stub-fail")).unwrap();

    fixture.write(".stub-state", "approved\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    fixture.write(".stub-state", "verifying\n");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-verifying")
    );
    let back = fixture.ok(&["stage", "back", "--to", "plan", "--reason", "Scope gap"]);
    assert!(
        back["applied"]["refresh"]
            .to_string()
            .contains("needs native reapproval")
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.write(".stub-state", "archived\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:archived"
    );
    assert_eq!(stage(&fixture), "done");
}

#[cfg(unix)]
#[test]
fn large_native_status_still_gates_by_its_summary_state() {
    let fixture = Fixture::new();
    fixture.stubs();
    // An archived SpecSync change lists every delivery file, so real status output
    // exceeds the 16 KiB display bound; over 4 MiB it is refused as oversized.
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nstate=$(cat .stub-state 2>/dev/null || echo draft)\nsize=20000\n[ -f .stub-huge ] && size=4200000\npad=$(head -c $size /dev/zero | tr '\\0' x)\nprintf '{\"summary\":{\"state\":\"%s\",\"next_action\":\"stub next action\",\"approval_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"change\":{\"affected_specs\":[],\"no_spec_change\":true},\"acceptance_entries\":\"%s\"}\\n' \"$state\" \"$pad\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "large-change"]);
    fixture.no_spec_approval("large-change");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.write(".stub-huge", "");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-unknown"), "{refused}");
    assert!(refused.contains("oversized"), "{refused}");
    fs::remove_file(fixture.root().join(".stub-huge")).unwrap();
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");
    fixture.write(".stub-state", "archived\n");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["native_specsync"]["state"], "observed");
    assert_eq!(status["native_summary"]["state"], "archived");
    let shown = status["native_specsync"]["content"].as_str().unwrap();
    assert!(shown.contains("display only"), "{shown}");
    assert!(shown.len() < 17_000);
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:archived"
    );
    assert_eq!(stage(&fixture), "done");
}

#[test]
fn clear_flags_apply_before_new_entries() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Old gate",
        "--blocker",
        "Old blocker",
    ]);
    let updated = fixture.ok(&[
        "checkpoint",
        "--clear-awaiting-human",
        "--awaiting-human",
        "New gate",
        "--clear-blockers",
        "--blocker",
        "New blocker",
    ]);
    assert_eq!(updated["checkpoint"]["awaiting_human"], json!(["New gate"]));
    assert_eq!(updated["checkpoint"]["blockers"], json!(["New blocker"]));
    assert_eq!(fixture.session()["awaiting_human"], json!(["New gate"]));
}

#[cfg(unix)]
#[test]
fn invalid_native_approval_refuses_every_route_into_build_and_later_stages() {
    // BUG-0006: an eligible state name is not enough; approval_valid must be true.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "edited-scope"]);
    fixture.no_spec_approval("edited-scope");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.write(".stub-approval", "false\n");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-approval-invalid"),
        "{refused}"
    );
    assert!(refused.contains("approval_valid false"), "{refused}");
    assert!(
        refused.contains("stub next action for approved"),
        "{refused}"
    );
    let routes: [&[&str]; 4] = [
        &["stage", "skip", "--reason", "Skip ahead"],
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        &["stage", "reset", "--to", "test", "--reason", "Retest"],
        &["stage", "reset", "--to", "accept", "--reason", "Accept"],
    ];
    for route in routes {
        let refused = fixture.fails(route);
        assert!(
            refused.contains("refused:native-approval-invalid"),
            "{route:?}: {refused}"
        );
    }
    assert_eq!(stage(&fixture), "plan");
    fixture.write(".stub-approval", "none\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("approval_valid missing")
    );
    fixture.write(".stub-approval", "true\n");
    fixture.write(".stub-correction", "false\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-correction-invalid")
    );
    fixture.write(".stub-correction", "true\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    // An approval that becomes invalid during build stops the move to test.
    fixture.write(".stub-state", "implementing\n");
    fixture.write(".stub-approval", "false\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-approval-invalid")
    );
    assert_eq!(stage(&fixture), "build");
}

#[cfg(unix)]
#[test]
fn reset_back_and_release_meet_their_destination_prerequisites() {
    // BUG-0007: reset, back and release reach a stage only when its prerequisite holds.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "draft-change"]);
    for target in ["done", "accept", "test", "build"] {
        let refused = fixture.fails(&["stage", "reset", "--to", target, "--reason", "Jump"]);
        assert!(
            refused.contains("refused:native-draft"),
            "{target}: {refused}"
        );
    }
    assert_eq!(stage(&fixture), "intent");
    for target in ["plan", "design", "intent"] {
        fixture.ok(&["stage", "reset", "--to", target, "--reason", "Rework"]);
    }
    fixture.write(".stub-state", "archived\n");
    for target in ["test", "accept", "done"] {
        let moved = fixture.ok(&["stage", "reset", "--to", target, "--reason", "Closeout"]);
        assert_eq!(moved["applied"]["to"], target);
    }
    assert!(
        fixture
            .fails(&["stage", "reset", "--to", "build", "--reason", "More work"])
            .contains("refused:native-archived")
    );
    fixture.write(".stub-validity", "corrupt-history\n");
    assert!(
        fixture
            .fails(&["stage", "back", "--to", "accept", "--reason", "Recheck"])
            .contains("refused:native-archive-corrupt-history")
    );
    fixture.ok(&["stage", "reset", "--to", "plan", "--reason", "Again"]);
    fixture.write(".stub-validity", "none\n");
    assert!(
        fixture
            .fails(&["stage", "reset", "--to", "done", "--reason", "Close"])
            .contains("refused:native-archive-unreported")
    );
    fixture.write(".stub-validity", "authenticated-history\n");
    fixture.ok(&["stage", "reset", "--to", "done", "--reason", "Close"]);
    fixture.ok(&[
        "consent",
        "grant",
        "release",
        "--source",
        "SYNTHETIC test consent; authorizes nothing real",
    ]);
    fixture.ok(&["stage", "enter", "release", "--reason", "Release routing"]);
    fixture.write(".stub-validity", "corrupt-history\n");
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-archive-corrupt-history")
    );
    fixture.write(".stub-validity", "authenticated-history\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:archived"
    );
}

#[cfg(unix)]
#[test]
fn return_from_a_side_stage_meets_the_caller_prerequisite() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "side-trip"]);
    fixture.no_spec_approval("side-trip");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "enter", "bugs", "--reason", "A failing case"]);
    fixture.write(".stub-approval", "false\n");
    let refused = fixture.fails(&["stage", "return"]);
    assert!(
        refused.contains("refused:native-approval-invalid"),
        "{refused}"
    );
    // Every reset Chough suggests carries the reason clap requires.
    assert!(
        refused.contains("chough stage reset --to <stage> --reason"),
        "{refused}"
    );
    assert_eq!(stage(&fixture), "bugs");
    fixture.write(".stub-approval", "true\n");
    assert_eq!(fixture.ok(&["stage", "return"])["applied"]["to"], "test");
}

#[cfg(unix)]
#[test]
fn sdd_policy_is_read_with_specsync_semantics() {
    // BUG-0008: an invalid policy refuses; only a valid disabled or absent one is ungated.
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "policy-change"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    for (policy, why) in [
        ("{ \"enabled\": true, ", "truncated"),
        ("{\"enabled\": \"yes\"}", "enabled as a string"),
        ("{\"enabled\": null}", "enabled as null"),
        ("{\"meaningful_paths\": \"src/\"}", "paths as a string"),
        ("{\"enabled\": true, \"enabled\": false}", "duplicate key"),
        ("true", "not an object"),
    ] {
        fixture.write(".specsync/sdd.json", policy);
        let refused = fixture.fails(&["stage", "advance"]);
        assert!(
            refused.contains("refused:sdd-policy-invalid"),
            "{why}: {refused}"
        );
        assert!(refused.contains(".specsync/sdd.json"), "{why}: {refused}");
    }
    // As in SpecSync, a missing `enabled` means enabled, and unknown fields are accepted.
    fixture.write(
        ".specsync/sdd.json",
        "{\"version\": 2, \"future_field\": 1}\n",
    );
    assert!(
        fixture
            .fails(&["stage", "advance"])
            .contains("refused:native-draft")
    );
    fixture.write(".specsync/sdd.json", "{\"enabled\": false}\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "ungated:sdd-disabled"
    );
    fixture.ok(&["stage", "back", "--to", "plan", "--reason", "Again"]);
    fs::remove_file(fixture.root().join(".specsync/sdd.json")).unwrap();
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "ungated:sdd-disabled"
    );
}

/// Run a shell script in the fixture with a local Git identity, as native tools commit.
#[cfg(unix)]
fn sh(fixture: &Fixture, script: &str) -> String {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(script)
        .current_dir(fixture.root())
        .env("CHOUGH", env!("CARGO_BIN_EXE_chough"))
        .env(
            "SAMPLE",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shipping"),
        )
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "script failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

#[cfg(unix)]
#[test]
fn real_specsync_approval_reported_invalid_refuses_build() {
    // BUG-0006 against real SpecSync. The approver is SYNTHETIC and approves nothing real.
    let available = std::process::Command::new("specsync")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());
    if !available {
        eprintln!("skipped: real SpecSync is not on PATH (the Fledge CI job runs this test)");
        return;
    }
    let fixture = Fixture::new();
    sh(
        &fixture,
        r##"set -eu
rm -rf ./* .chough .specsync
cp -R "$SAMPLE/." .
rm -f .specsync/changes/.keep .specsync/archive/changes/.keep
git -c init.defaultBranch=main init -q
git config user.name "Chough test"; git config user.email test@example.invalid; git config commit.gpgsign false
"$CHOUGH" init --objective "Synthetic approval validity regression" --intent-scope CHECKOUT --module shipping >/dev/null
specsync change adopt --json >/dev/null
git add -A; git commit -qm seed
id=free-shipping-at-the-threshold
specsync change new "Free shipping at the threshold" --spec shipping --path src/lib.rs --path tests/ --json >/dev/null
specsync change answer $id acceptance_criteria '["An order of exactly 5000 cents ships free"]' --json >/dev/null
specsync change answer $id public_contract yes --json >/dev/null
specsync change answer $id architecture_risk no --json >/dev/null
specsync change answer $id constraints none --json >/dev/null 2>&1 || true
for f in .specsync/changes/$id/*.md; do
  name=$(basename "$f" .md)
  [ "$name" = change ] && continue
  printf -- '---\nchange: %s\nartifact: %s\n---\n\n# %s\n\n' "$id" "$name" "$name" >"$f"
  case "$name" in
    tasks) printf -- '- [x] Add the threshold rule.\n' >>"$f" ;;
    plan) printf '## Approach\n\nAdd the rule.\n\n## Out of scope\n\nNone.\n\n## Steps\n\n1. Add it.\n\n## Risks\n\nOff by one.\n\n## Constraints consulted\n\nNone.\n' >>"$f" ;;
    *) printf 'Orders of 5000 cents or more ship free.\n' >>"$f" ;;
  esac
done
mkdir -p .specsync/changes/$id/deltas
printf '## ADDED\n\n### REQUIREMENT REQ-shipping-002\n\nThe module SHALL charge no fee at or above the threshold. Trace: CHECKOUT-1.\n\nAcceptance Criteria\n- tests/threshold.rs asserts a zero fee at the threshold.\n' >.specsync/changes/$id/deltas/shipping.md
git add -A; git commit -qm draft
specsync change approve $id --actor "SYNTHETIC-test-approver-not-a-human" >/dev/null
"$CHOUGH" checkpoint --native-change $id >/dev/null
"$CHOUGH" stage advance >/dev/null
"$CHOUGH" stage skip --reason "No visuals" >/dev/null
cp .specsync/changes/$id/state.json state.approved
sed 's#"tests/"#"tests/", "docs/"#' state.approved >.specsync/changes/$id/state.json
specsync change status $id --json
"##,
    )
    .contains("\"approval_valid\": false")
    .then_some(())
    .expect("real SpecSync should report the changed scope as an invalid approval");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(
        refused.contains("refused:native-approval-invalid"),
        "{refused}"
    );
    sh(
        &fixture,
        "cp state.approved .specsync/changes/free-shipping-at-the-threshold/state.json",
    );
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
}

#[test]
fn native_change_ids_up_to_the_specsync_ceiling_are_kept_across_processes() {
    // BUG-0013: SpecSync mints IDs up to 120 bytes and reads up to 255.
    let fixture = Fixture::new();
    fixture.init();
    let id = "prepare-a-first-release-candidate-with-reference-aware-evidence-retention-current-guidance-and-an-installable-package";
    assert_eq!(id.len(), 117);
    fixture.ok(&["checkpoint", "--native-change", id]);
    assert_eq!(fixture.session()["native_change"], id);
    assert_eq!(
        fixture.ok(&["checkpoint"])["checkpoint"]["native_change"],
        id
    );
    let ceiling = "a".repeat(255);
    fixture.ok(&["checkpoint", "--native-change", &ceiling]);
    let too_long = "a".repeat(256);
    assert!(
        fixture
            .fails(&["checkpoint", "--native-change", &too_long])
            .contains("invalid native change ID")
    );
    for malformed in ["has space", "dot.dot", "../escape", ""] {
        assert!(
            fixture
                .fails(&["checkpoint", "--native-change", malformed])
                .contains("invalid native change ID"),
            "{malformed:?}"
        );
    }
    assert_eq!(fixture.session()["native_change"], ceiling);
}

#[cfg(unix)]
/// Split a suggested command line into arguments, keeping double-quoted words whole.
fn words(command: &str) -> Vec<String> {
    let mut words = vec![];
    let mut word = String::new();
    let mut quoted = false;
    for character in command.chars() {
        match character {
            '"' => quoted = !quoted,
            ' ' if !quoted => {
                if !word.is_empty() {
                    words.push(std::mem::take(&mut word));
                }
            }
            _ => word.push(character),
        }
    }
    if !word.is_empty() {
        words.push(word);
    }
    words
}

/// A fixture in stage done whose recorded native change is archived.
#[cfg(unix)]
fn done_fixture(id: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("archived");
    fixture.write(".stub-validity", "exact\n");
    fixture.write(".stub-next", "merge the PR on GitHub if it is still open\n");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", id]);
    fixture.ok(&["stage", "reset", "--to", "done", "--reason", "Closeout"]);
    fixture.ok(&[
        "checkpoint",
        "--next-action",
        "specsync change review old-change --reviewer 0xLeif, then ship",
    ]);
    fixture
}

#[cfg(unix)]
#[test]
fn done_with_an_archived_change_recommends_a_new_cycle_with_a_reason() {
    let fixture = done_fixture("closed-change");
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    let text = action["action"].as_str().unwrap();
    assert!(
        text.starts_with("Start a new cycle: chough stage reset --to intent --reason \"<why>\""),
        "{text}"
    );
    for part in [
        "--to plan",
        "specsync change new \"<outcome>\" --spec <module> --path <path>",
        "chough checkpoint --native-change <id> --next-action \"<next step>\"",
    ] {
        assert!(text.contains(part), "missing {part:?}: {text}");
    }
    assert!(!text.contains("specsync change review"), "{text}");
    let reason = action["reason"].as_str().unwrap();
    assert!(
        reason.contains(
            "The cycle for closed-change is closed: native state archived, terminal evidence exact."
        ),
        "{reason}"
    );
    assert!(
        reason.contains(
            "Native next action, check it first: merge the PR on GitHub if it is still open."
        ),
        "{reason}"
    );
    assert_eq!(
        action["command"],
        "chough stage reset --to intent --reason \"<why>\""
    );
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(status["next_valid_action"], action["action"]);
    let shown = String::from_utf8(fixture.run(&["next"]).stdout).unwrap();
    assert!(shown.contains("Next: Start a new cycle"), "{shown}");
    assert!(
        shown.contains("Native next action: merge the PR on GitHub"),
        "{shown}"
    );

    // The suggested reset, with its placeholder filled, succeeds and replaces the stale
    // next action; the transition keeps the reason.
    let command = action["command"]
        .as_str()
        .unwrap()
        .replace("<why>", "New work: a focused view");
    let arguments = words(&command);
    assert_eq!(arguments[0], "chough");
    let arguments: Vec<&str> = arguments[1..].iter().map(String::as_str).collect();
    let reset = fixture.ok(&arguments);
    assert_eq!(reset["applied"]["to"], "intent");
    assert_eq!(
        fixture.session()["next_action"],
        "New cycle from done after reset: New work: a focused view. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action."
    );
    let last = transitions(&fixture).pop().unwrap();
    assert_eq!(last["kind"], "reset");
    assert_eq!(last["reason"], "New work: a focused view");

    // Back continues the same cycle and keeps the next action.
    fixture.ok(&["checkpoint", "--next-action", "Keep this step"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "back", "--to", "intent", "--reason", "Refine"]);
    assert_eq!(fixture.session()["next_action"], "Keep this step");
}

#[test]
fn reset_replaces_the_next_action_and_back_keeps_it() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["checkpoint", "--next-action", "Finish the old plan"]);
    fixture.ok(&["stage", "back", "--to", "design", "--reason", "Revisit"]);
    assert_eq!(fixture.session()["next_action"], "Finish the old plan");
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "plan",
        "--reason",
        "Different work",
    ]);
    assert_eq!(
        fixture.session()["next_action"],
        "New cycle from design after reset: Different work. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action."
    );
    let last = transitions(&fixture).pop().unwrap();
    assert_eq!(last["kind"], "reset");
    assert_eq!(last["reason"], "Different work");

    // The next action keeps the checkpoint's own 4096-byte bound and stays one line,
    // while the transition keeps the reason whole.
    let reason = format!("Line one\nline two {}", "r".repeat(4_096 - 18));
    assert_eq!(reason.len(), 4_096);
    fixture.ok(&["stage", "reset", "--to", "intent", "--reason", &reason]);
    let next_action = fixture.session()["next_action"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(next_action.len() <= 4_096, "{} bytes", next_action.len());
    assert!(
        next_action.starts_with("New cycle from plan after reset: Line one line two rrr"),
        "{next_action:.80}"
    );
    assert!(!next_action.contains('\n'));
    assert!(next_action.ends_with("and set the next action."));
    assert_eq!(transitions(&fixture).pop().unwrap()["reason"], reason);
    fixture.ok(&["checkpoint", "--next-action", &next_action]);
}

#[cfg(unix)]
#[test]
fn done_keeps_the_gate_refusal_for_an_archive_that_does_not_authenticate() {
    let fixture = done_fixture("unproven-archive");
    fixture.write(".stub-validity", "corrupt-history\n");
    let next = fixture.ok(&["next", "--json"]);
    let action = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        action.contains("refused:native-archive-corrupt-history"),
        "{action}"
    );
    assert!(!action.contains("Start a new cycle"), "{action}");
}

#[cfg(unix)]
#[test]
fn done_without_a_native_change_or_with_sdd_disabled_recommends_the_reset() {
    let fixture = done_fixture("forgotten");
    let mut session = fixture.session();
    session["native_change"] = Value::Null;
    fixture.write(".chough/session.json", &session.to_string());
    let next = fixture.ok(&["next", "--json"]);
    let action = &next["next_valid_action"];
    assert!(
        action["action"]
            .as_str()
            .unwrap()
            .contains("specsync change new"),
        "{action}"
    );
    assert!(
        action["reason"]
            .as_str()
            .unwrap()
            .contains("No native change is recorded"),
        "{action}"
    );
    assert_eq!(
        action["command"],
        "chough stage reset --to intent --reason \"<why>\""
    );

    fixture.write(".specsync/sdd.json", "{\"enabled\": false}\n");
    let next = fixture.ok(&["next", "--json"]);
    let text = next["next_valid_action"]["action"].as_str().unwrap();
    assert!(
        text.starts_with("Start a new cycle: chough stage reset --to intent --reason"),
        "{text}"
    );
    assert!(!text.contains("specsync change new"), "{text}");
}

// ---------------------------------------------------------------------------
// The approved-delta guard (REQ-cli-022, BUG-0002).

/// SpecSync 6.1.1's approved-delta digest: SHA-256 over the length-framed domain,
/// module name and delta body, with CRLF normalized to LF.
fn delta_digest(module: &str, body: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let body = String::from_utf8_lossy(body).replace("\r\n", "\n");
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

#[cfg(unix)]
const DELTA: &str = "## ADDED\n\n### REQUIREMENT REQ-cli-090\n\nThe helper SHALL do the thing.\n";

/// A fixture at plan whose native change `id` is approved with a `cli` delta.
#[cfg(unix)]
fn approved_delta(id: &str) -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.write(".stub-affected", "\"cli\"\n");
    fixture.write(".stub-no-spec", "false\n");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", id]);
    fixture.write(&format!(".specsync/changes/{id}/deltas/cli.md"), DELTA);
    fixture.write(
        &format!(".specsync/changes/{id}/approvals.json"),
        &json!({
            "approvals": [
                {"gate": "definition", "actor": "SYNTHETIC", "approved_delta_digests": {"cli": "0".repeat(64)}},
                {"gate": "review", "actor": "SYNTHETIC"},
                {"gate": "definition", "actor": "SYNTHETIC", "approved_delta_digests": {"cli": delta_digest("cli", DELTA.as_bytes())}},
            ],
            "reopenings": [],
        })
        .to_string(),
    );
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture
}

#[cfg(unix)]
/// The session and transition bytes, which a refused move must leave unchanged.
fn records(fixture: &Fixture) -> (Vec<u8>, Vec<u8>) {
    (
        fs::read(fixture.root().join(".chough/session.json")).unwrap(),
        fs::read(fixture.root().join(".chough/transitions.jsonl")).unwrap(),
    )
}

/// Assert that `route` is refused with `label`, naming `expected`, and changes nothing.
#[cfg(unix)]
fn refused_unchanged(fixture: &Fixture, route: &[&str], label: &str, expected: &str) {
    let before = records(fixture);
    let refused = fixture.fails(route);
    assert!(refused.contains(label), "{route:?}: {refused}");
    assert!(refused.contains(expected), "{route:?}: {refused}");
    assert!(records(fixture) == before, "{route:?} changed the records");
}

#[test]
fn recomputed_digests_match_every_archived_approval_in_this_repository() {
    let archive =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".specsync/archive/changes");
    let mut compared = 0;
    for entry in fs::read_dir(&archive).unwrap() {
        let directory = entry.unwrap().path();
        let ledger: Value =
            serde_json::from_str(&fs::read_to_string(directory.join("approvals.json")).unwrap())
                .unwrap();
        let approval = ledger["approvals"]
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .find(|approval| approval["gate"] == "definition")
            .unwrap();
        for (module, digest) in approval["approved_delta_digests"].as_object().unwrap() {
            let body = fs::read(directory.join(format!("deltas/{module}.md"))).unwrap();
            assert_eq!(
                delta_digest(module, &body),
                digest.as_str().unwrap(),
                "{}",
                directory.display()
            );
            compared += 1;
        }
    }
    assert!(compared >= 7, "only {compared} archived digests compared");
}

#[cfg(unix)]
#[test]
fn the_guard_accepts_every_archived_approval_in_this_repository() {
    // Each archived ledger and its deltas, presented as an approved change, pass the
    // guard: Chough's digests match the ones SpecSync recorded.
    let archive =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(".specsync/archive/changes");
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    let mut checked = 0;
    for entry in fs::read_dir(&archive).unwrap() {
        let directory = entry.unwrap().path();
        let finalization: Value =
            serde_json::from_str(&fs::read_to_string(directory.join("finalization.json")).unwrap())
                .unwrap();
        let id = finalization["change_id"].as_str().unwrap().to_string();
        let target = format!(".specsync/changes/{id}");
        let ledger = fs::read_to_string(directory.join("approvals.json")).unwrap();
        fixture.write(&format!("{target}/approvals.json"), &ledger);
        if let Ok(deltas) = fs::read_dir(directory.join("deltas")) {
            for delta in deltas {
                let delta = delta.unwrap().path();
                let name = delta.file_name().unwrap().to_str().unwrap().to_string();
                fixture.write(
                    &format!("{target}/deltas/{name}"),
                    &fs::read_to_string(&delta).unwrap(),
                );
            }
        }
        let ledger: Value = serde_json::from_str(&ledger).unwrap();
        let scope = &ledger["approvals"]
            .as_array()
            .unwrap()
            .iter()
            .rev()
            .find(|approval| approval["gate"] == "definition")
            .unwrap()["approved_scope"];
        let affected: Vec<String> = scope["affected_specs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|module| format!("\"{}\"", module.as_str().unwrap()))
            .collect();
        fixture.write(".stub-affected", &affected.join(","));
        fixture.write(".stub-no-spec", &scope["no_spec_change"].to_string());
        fixture.ok(&["checkpoint", "--native-change", &id]);
        let next = fixture.ok(&["next", "--json"]);
        assert_eq!(next["gate"]["label"], "native:approved", "{id}: {next}");
        let deltas = next["native"]["approved_deltas"].as_str().unwrap();
        assert!(
            deltas.starts_with("unchanged: cli") || deltas.starts_with("no semantic deltas"),
            "{id}: {deltas}"
        );
        checked += 1;
    }
    assert!(checked >= 10, "only {checked} archived changes checked");
}

#[cfg(unix)]
#[test]
fn a_changed_added_or_deleted_delta_refuses_every_route_with_native_delta_changed() {
    let id = "guarded-change";
    let fixture = approved_delta(id);
    let delta = format!(".specsync/changes/{id}/deltas/cli.md");
    let changed = "refused:native-delta-changed";
    // An approved delta edited after the approval, while status still reports
    // approval_valid true (BUG-0002).
    fixture.write(
        &delta,
        &format!("{DELTA}\n### REQUIREMENT REQ-cli-091\n\nMore.\n"),
    );
    let routes: [&[&str]; 5] = [
        &["stage", "advance"],
        &["stage", "skip", "--reason", "Skip ahead"],
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        &["stage", "reset", "--to", "test", "--reason", "Retest"],
        &["stage", "reset", "--to", "accept", "--reason", "Accept"],
    ];
    for route in routes {
        refused_unchanged(&fixture, route, changed, "cli");
    }
    let refused = fixture.fails(&["stage", "advance"]);
    for part in [
        "specsync change approve guarded-change --actor \"<their name>\"",
        "specsync change check guarded-change",
        "specsync.approved-delta.v1",
    ] {
        assert!(refused.contains(part), "missing {part:?}: {refused}");
    }
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["gate"]["label"], changed);
    assert!(
        next["native"]["approved_deltas"]
            .as_str()
            .unwrap()
            .starts_with("changed: cli"),
        "{next}"
    );
    assert_eq!(
        fixture.ok(&["status", "--json"])["checkpoint"]["gate"]["label"],
        changed
    );

    // A line-ending-only edit and the restored delta pass.
    fixture.write(&delta, &DELTA.replace('\n', "\r\n"));
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
    fixture.write(&delta, DELTA);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "test");

    // A module added after the approval.
    fixture.write(".stub-affected", "\"cli\",\"docs\"\n");
    fixture.write(&format!(".specsync/changes/{id}/deltas/docs.md"), DELTA);
    refused_unchanged(&fixture, &["stage", "advance"], changed, "docs");
    refused_unchanged(
        &fixture,
        &["stage", "back", "--to", "build", "--reason", "Fix"],
        changed,
        "docs",
    );
    fixture.write(".stub-affected", "\"cli\"\n");
    fs::remove_file(
        fixture
            .root()
            .join(format!(".specsync/changes/{id}/deltas/docs.md")),
    )
    .unwrap();

    // A deleted delta.
    fs::remove_file(fixture.root().join(&delta)).unwrap();
    refused_unchanged(&fixture, &["stage", "advance"], changed, "cli");
    fixture.write(&delta, DELTA);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "accept");

    // Module names SpecSync 6.1.1 accepts, such as `a.b` and `Módulo`, are compared
    // like any other: approved deltas pass and an edited one refuses.
    let modules = ["a.b", "Módulo"];
    fixture.write(".stub-affected", "\"cli\",\"a.b\",\"Módulo\"\n");
    let mut digests = serde_json::Map::new();
    digests.insert("cli".into(), json!(delta_digest("cli", DELTA.as_bytes())));
    for module in modules {
        fixture.write(&format!(".specsync/changes/{id}/deltas/{module}.md"), DELTA);
        digests.insert(module.into(), json!(delta_digest(module, DELTA.as_bytes())));
    }
    fixture.write(
        &format!(".specsync/changes/{id}/approvals.json"),
        &json!({"approvals": [{"gate": "definition", "approved_delta_digests": digests}], "reopenings": []})
            .to_string(),
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["native"]["approved_deltas"], "unchanged: Módulo, a.b, cli",
        "{next}"
    );
    fixture.write(
        &format!(".specsync/changes/{id}/deltas/a.b.md"),
        &format!("{DELTA}\nMore.\n"),
    );
    refused_unchanged(
        &fixture,
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        changed,
        "a.b (edited)",
    );
    fixture.write(&format!(".specsync/changes/{id}/deltas/a.b.md"), DELTA);
    fixture.ok(&["stage", "reset", "--to", "build", "--reason", "Restart"]);
    assert_eq!(stage(&fixture), "build");
}

#[cfg(unix)]
#[test]
fn a_comparison_that_cannot_be_made_refuses_with_native_delta_unverifiable() {
    let id = "unverifiable-change";
    let fixture = approved_delta(id);
    let ledger = format!(".specsync/changes/{id}/approvals.json");
    let approved = fixture.read(&ledger);
    let unverifiable = "refused:native-delta-unverifiable";
    let status = format!("specsync change status {id}");
    let digest = delta_digest("cli", DELTA.as_bytes());
    // An approval without digests can only be cleared by a new approval, so its
    // refusal names that rather than a status read.
    let approve = format!("specsync change approve {id} --actor \"<their name>\"");
    let cases: Vec<(&str, String, &str)> = vec![
        ("malformed", "{\"approvals\": [".into(), &status),
        ("malformed", "{\"approvals\": \"none\"}".into(), &status),
        (
            "no definition approval",
            json!({"approvals": [{"gate": "review"}]}).to_string(),
            &status,
        ),
        (
            "approved_delta_digests",
            json!({"approvals": [{"gate": "definition"}]}).to_string(),
            &approve,
        ),
        (
            "malformed",
            json!({"approvals": [{"gate": "definition", "approved_delta_digests": {"cli": "short"}}]})
                .to_string(),
            &status,
        ),
        (
            "malformed",
            json!({"approvals": [{"gate": "definition", "approved_delta_digests": {"../cli": digest}}]})
                .to_string(),
            &status,
        ),
        (
            "4194304",
            format!(
                "{{\"approvals\":[{{\"gate\":\"definition\",\"approved_delta_digests\":{{\"cli\":\"{digest}\"}}}}],\"padding\":\"{}\"}}",
                "x".repeat(4 * 1_048_576)
            ),
            &status,
        ),
    ];
    for (why, content, remedy) in cases {
        fixture.write(&ledger, &content);
        refused_unchanged(&fixture, &["stage", "advance"], unverifiable, why);
        refused_unchanged(&fixture, &["stage", "advance"], unverifiable, remedy);
    }
    fs::remove_file(fixture.root().join(&ledger)).unwrap();
    refused_unchanged(&fixture, &["stage", "advance"], unverifiable, "missing");
    fixture.write(&ledger, &approved);
    // A module name that is not one plain path segment is never read as a path.
    for module in ["../cli", "a/b", "nested\\cli", ".", "trailing."] {
        fixture.write(".stub-affected", &format!("{}\n", json!(module)));
        refused_unchanged(
            &fixture,
            &["stage", "reset", "--to", "build", "--reason", "Restart"],
            unverifiable,
            "invalid module name",
        );
    }
    fixture.write(".stub-affected", "none\n");
    refused_unchanged(
        &fixture,
        &["stage", "reset", "--to", "build", "--reason", "Restart"],
        unverifiable,
        "affected_specs",
    );
    let next = fixture.ok(&["next", "--json"]);
    assert!(
        next["native"]["approved_deltas"]
            .as_str()
            .unwrap()
            .starts_with("unverifiable:"),
        "{next}"
    );
    fixture.write(".stub-affected", "\"cli\"\n");
    assert_eq!(
        fixture.ok(&["stage", "advance"])["applied"]["gate"],
        "native:approved"
    );
}
