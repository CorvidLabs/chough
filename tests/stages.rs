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
        next["next_valid_action"]
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
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert!(refused.contains("specsync change status add-thing"));
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["checkpoint"]["gate"]["allowed"], false);
    assert!(
        next["next_valid_action"]
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
        "#!/bin/sh\nstate=$(cat .stub-state 2>/dev/null || echo draft)\nsize=20000\n[ -f .stub-huge ] && size=4200000\npad=$(head -c $size /dev/zero | tr '\\0' x)\nprintf '{\"summary\":{\"state\":\"%s\",\"next_action\":\"stub next action\",\"approval_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"acceptance_entries\":\"%s\"}\\n' \"$state\" \"$pad\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "large-change"]);
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
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "skip", "--reason", "No visuals"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "enter", "bugs", "--reason", "A failing case"]);
    fixture.write(".stub-approval", "false\n");
    assert!(
        fixture
            .fails(&["stage", "return"])
            .contains("refused:native-approval-invalid")
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
