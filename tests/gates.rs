//! Gate lists (REQ-cli-061, REQ-history-014, FLOW-19, FLOW-20, #184): `chough gate
//! list` numbers each open human gate with a one-screen summary and its digest, and
//! `chough gate record` closes only listed gates with one reply, each relay keeping
//! its own summary digest.
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const ID: &str = "g1";
const KEY: &str = "gates-g1";
const DIR: &str = ".specsync/changes/g1";
const SCOPE: &str = "Scope approval of g1";

/// A stand-in `specsync`: status reports `g1` archived; ship records its call.
const STUB: &str = r#"#!/bin/sh
case "$2" in
status)
    printf '{"summary":{"id":"%s","state":"archived","approval_valid":true,"correction_valid":true,"next_action":"next","terminal_evidence":{"validity":"authenticated-history"}},"change":{"affected_specs":["cli"],"no_spec_change":true},"acceptance_entries":[{"path":"specs/cli/cli.spec.md","kind":"file"}]}\n' "$3"
    ;;
ship)
    echo "$*" >>.stub-ship-calls
    echo 'shipped'
    ;;
*)
    echo '{}'
    ;;
esac
"#;

const CASES: &str = r#"profile = "chough-testing/1"
module = "cli"

[[case]]
id = "TC-cli-g1-pass"
title = "g1 passes"
status = "active"
track = "functional"
method = "manual"
criteria = ["LIFE-1"]
requirements = ["REQ-cli-001"]
environments = ["local"]
artifacts = ["reviewer-note"]
setup = ["Fixture"]
steps = ["Look"]
expected = ["it passes"]

[[case]]
id = "TC-cli-g1-fail"
title = "g1 fails"
status = "active"
track = "functional"
method = "manual"
criteria = ["LIFE-1"]
requirements = ["REQ-cli-001"]
environments = ["local"]
artifacts = ["reviewer-note"]
setup = ["Fixture"]
steps = ["Look"]
expected = ["it passes"]
"#;

fn git_head(fixture: &Fixture) -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// A Git project with a session for `g1`: its base commit, then a change to
/// `src/input.txt`; a plan with risks; one passing and one failing observation; and
/// `gates` open.
fn project(gates: &[&str]) -> Fixture {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(".specsync/sdd.json", "{\"enabled\": true}\n");
    fixture.write("bin/specsync", STUB);
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    fixture.write(".chough/cases/cli.g1.cases.toml", CASES);
    fixture.ok(&["session", "new", KEY, "--objective", "Gate g1"]);
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.commit("base");
    let base = git_head(&fixture);
    fixture.write(
        &format!("{DIR}/state.json"),
        &json!({"id": ID, "title": "Free shipping at the threshold", "state": "approved", "base_commit": base}).to_string(),
    );
    // The change's delta adds REQ-cli-001, which both cases link; nothing links their
    // observations to the session (redefined tests source, option A).
    fixture.write(
        &format!("{DIR}/deltas/cli.md"),
        "## ADDED\n\n### REQUIREMENT REQ-cli-001\n\nPreserve files.\n",
    );
    fixture.write(
        &format!("{DIR}/plan.md"),
        "# Plan\n\n## Approach\n\nx\n\n## Risks\n\n- A rounding edge at exactly fifty dollars.\n\n## Constraints consulted\n\nnone\n",
    );
    // The product change, larger than any one of Chough's own records in the diff.
    let body: String = (1..=60).map(|line| format!("line {line}\n")).collect();
    fixture.write("src/input.txt", &body);
    for (case, outcome) in [("TC-cli-g1-pass", "passed"), ("TC-cli-g1-fail", "failed")] {
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
            outcome,
            "--environment",
            "local",
            "--summary",
            "g1 case",
            "--case",
            case,
        ]);
    }
    for gate in gates {
        fixture.ok(&["checkpoint", "--awaiting-human", gate]);
    }
    fixture.commit("work");
    fixture
}

fn list(fixture: &Fixture) -> Value {
    fixture.ok(&["gate", "list", "--json"])
}

fn relay(words: &str) -> Vec<&str> {
    vec![
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        words,
        "--where",
        "a test",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]
}

fn record<'a>(list_id: &'a str, gates: &'a str, words: &'a str) -> Vec<&'a str> {
    let mut args = vec!["gate", "record", list_id, "--gates", gates];
    args.extend(relay(words));
    args
}

fn session(fixture: &Fixture) -> Value {
    fixture.session_json(KEY)
}

/// Every gate-closed relay on the active session's Timeline, by gate.
fn closed_relays(fixture: &Fixture) -> BTreeMap<String, Value> {
    let mut found = BTreeMap::new();
    for entry in fixture.ok(&["history", "show"])["entries"]
        .as_array()
        .unwrap()
    {
        if entry["kind"] == "gate-closed" && !entry["relayed_by"].is_null() {
            // A gate-closed entry's text is its gate (the reason stays empty).
            let gate = entry["gate"]
                .as_str()
                .or(entry["reason"].as_str())
                .unwrap()
                .to_string();
            found.insert(gate, entry["relayed_by"].clone());
        }
    }
    found
}

/// Every tracked file under `.chough/`, with its bytes (the lock and local state are
/// left out).
fn records(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries {
            let path = entry.unwrap().path();
            let name = path.strip_prefix(base).unwrap().display().to_string();
            if name.starts_with(".chough/local") {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, files);
            } else {
                files.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    walk(root, &root.join(".chough"), &mut files);
    files
}

fn sha256(text: &str) -> String {
    use std::io::Write;
    let mut child = std::process::Command::new("shasum")
        .args(["-a", "256"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(text.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    String::from_utf8(output.stdout).unwrap()[..64].to_string()
}

#[test]
fn gate_list_shows_each_open_gate_with_a_one_screen_summary_and_its_digest() {
    let fixture = project(&[SCOPE]);
    let before = records(fixture.root());
    let listed = list(&fixture);
    let gates = listed["gates"].as_array().unwrap();
    assert_eq!(gates.len(), 1, "{listed:#}");
    let gate = &gates[0];
    assert_eq!(gate["number"], 1);
    assert_eq!(gate["gate"], SCOPE);
    assert_eq!(gate["change"], ID);
    let summary = gate["summary"].as_str().unwrap();
    for part in [
        "Free shipping at the threshold",
        "What changed:",
        "src/input.txt",
        "Tests: 1 passed, 1 failed, 0 not run",
        "TC-cli-g1-fail",
        "Risks:",
        "A rounding edge at exactly fifty dollars.",
        "Diff: git diff ",
    ] {
        assert!(
            summary.contains(part),
            "the summary lacks {part:?}:\n{summary}"
        );
    }
    assert!(
        summary.lines().count() <= 24 && summary.len() <= 4_096,
        "{summary}"
    );
    assert_eq!(gate["digest"], sha256(summary));
    let list_id = listed["list"].as_str().unwrap();
    assert_eq!(list_id.len(), 12);
    assert!(fixture.exists(&format!(".chough/local/gate-lists/{list_id}.json")));
    assert_eq!(
        records(fixture.root()),
        before,
        "listing changed a tracked file"
    );
}

#[test]
fn one_reply_closes_only_the_listed_gates_each_with_its_own_digest() {
    let gates = ["Gate A of g1", "Gate B of g1", "Gate C of g1"];
    let fixture = project(&gates);
    let listed = list(&fixture);
    let list_id = listed["list"].as_str().unwrap().to_string();
    let digest = |number: usize| {
        listed["gates"][number - 1]["digest"]
            .as_str()
            .unwrap()
            .to_string()
    };
    fixture.ok(&record(&list_id, "1,3", "approve both"));
    assert_eq!(session(&fixture)["awaiting_human"], json!(["Gate B of g1"]));
    let relays = closed_relays(&fixture);
    assert_eq!(relays.len(), 2, "{relays:#?}");
    for (gate, number) in [("Gate A of g1", 1), ("Gate C of g1", 3)] {
        let relay = &relays[gate];
        assert_eq!(relay["words"], "approve both");
        assert_eq!(relay["summary_digest"], digest(number), "{gate}");
        assert_eq!(relay["list"], list_id.as_str());
    }
    assert_ne!(digest(1), digest(3), "two gates share a summary");
}

#[test]
fn gate_record_refuses_unknown_unlisted_closed_or_changed_gates_with_nothing_written() {
    let fixture = project(&[SCOPE]);
    let listed = list(&fixture);
    let list_id = listed["list"].as_str().unwrap().to_string();
    let refused = |args: &[&str], why: &str| {
        let before = records(fixture.root());
        let error = fixture.fails(args);
        assert!(error.contains("`chough gate list`"), "{why}: {error}");
        assert_eq!(
            records(fixture.root()),
            before,
            "{why}: something was written"
        );
    };
    refused(&record("000000000000", "1", "yes"), "an unknown list");
    refused(&record(&list_id, "2", "yes"), "a number not on the list");
    // The summary changes when the work does.
    fixture.write("src/input.txt", "a later edit\n");
    fixture.commit("later");
    refused(&record(&list_id, "1", "yes"), "a changed summary");
    // A gate closed since listing.
    let relisted = list(&fixture);
    let fresh = relisted["list"].as_str().unwrap().to_string();
    fixture.ok(&["checkpoint", "--awaiting-human", "Another gate of g1"]);
    fixture.ok(&[
        "checkpoint",
        "--close-gate",
        SCOPE,
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "closed elsewhere",
        "--where",
        "a test",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    refused(&record(&fresh, "1", "yes"), "a gate no longer open");
    // Listing again and recording succeeds.
    let again = list(&fixture);
    let id = again["list"].as_str().unwrap().to_string();
    assert_eq!(again["gates"][0]["gate"], "Another gate of g1");
    fixture.ok(&record(&id, "1", "yes"));
    assert_eq!(session(&fixture)["awaiting_human"], json!([]));
}

#[test]
fn a_verifying_gate_is_refused_and_closes_through_ship_with_the_list() {
    let review = "Implementation review of g1";
    let fixture = project(&[]);
    // The review gate drives the native review as the decided human (REQ-records-008).
    fixture.write(
        ".chough/local/identity.toml",
        "[[human]]\nname = \"SYNTHETIC Human\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-reviewer\"\n",
    );
    let archive = ".specsync/archive/changes/2026-11-01-g1";
    fixture.write(
        &format!("{archive}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"g1\", \"timestamp\": 500, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{archive}/verification.json"),
        &json!({
            "commit": "2".repeat(40), "passed": true, "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": [{"path": "specs/cli/cli.spec.md", "kind": "file"}]},
        })
        .to_string(),
    );
    fixture.write(
        &format!("{archive}/change.md"),
        "---\nid: g1\nstate: archived\n---\n\n# Title of g1\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- g1 works\n",
    );
    let mut state: Value =
        serde_json::from_str(&fixture.read(&format!("{DIR}/state.json"))).unwrap();
    state["state"] = json!("verifying");
    fixture.write(&format!("{DIR}/state.json"), &state.to_string());
    fixture.ok(&[
        "stage",
        "reset",
        "--json",
        "--to",
        "accept",
        "--reason",
        "Ready to ship",
    ]);
    fixture.ok(&["checkpoint", "--awaiting-human", review]);
    fixture.commit("verifying");
    let listed = list(&fixture);
    let list_id = listed["list"].as_str().unwrap().to_string();
    let digest = listed["gates"][0]["digest"].as_str().unwrap().to_string();
    let before = records(fixture.root());
    let error = fixture.fails(&record(&list_id, "1", "pass"));
    assert!(
        error.contains(&format!(
            "`chough change ship {ID} --list {list_id} --gate 1"
        )),
        "{error}"
    );
    assert_eq!(records(fixture.root()), before);
    let mut ship = vec!["change", "ship", ID, "--list", &list_id, "--gate", "1"];
    ship.extend(relay("pass"));
    let shipped = fixture.run(&ship);
    assert!(
        shipped.status.success(),
        "{}",
        String::from_utf8_lossy(&shipped.stderr)
    );
    assert_eq!(session(&fixture)["stage"]["current"], "done");
    let relays = closed_relays(&fixture);
    assert_eq!(relays[review]["summary_digest"], digest.as_str());
    assert_eq!(relays[review]["list"], list_id.as_str());
}

#[test]
fn relay_summary_fields_are_both_or_neither() {
    let fixture = Fixture::new();
    fixture.init();
    let digest = "a".repeat(64);
    let good = json!({"decided_by": "SYNTHETIC-human", "words": "yes", "where": "chat", "recorded_by": "SYNTHETIC-agent", "summary_digest": digest, "list": "0123456789ab"});
    let entry = |relay: &Value| {
        format!(
            "- 2026-11-11T00:00:00Z · gate-closed · intent → intent · Gate ^t-aaaaaa\n  - relayed_by: {}\n",
            serde_json::to_string(relay).unwrap()
        )
    };
    let file = fixture.session_file();
    let original = fixture.read(&file);
    fixture.write(&file, &format!("{original}\n{}", entry(&good)));
    let shown = fixture.ok(&["history", "show"]);
    let entries = shown["entries"].as_array().unwrap();
    assert_eq!(entries.last().unwrap()["relayed_by"], good);
    let mut variants = vec![];
    for key in ["summary_digest", "list"] {
        let mut alone = good.clone();
        alone.as_object_mut().unwrap().remove(key);
        variants.push(alone);
    }
    let mut short = good.clone();
    short["summary_digest"] = json!("abc");
    variants.push(short);
    let mut upper = good.clone();
    upper["list"] = json!("0123456789AB");
    variants.push(upper);
    for relay in variants {
        fixture.write(&file, &format!("{original}\n{}", entry(&relay)));
        let error = fixture.fails(&["history", "show"]);
        assert!(error.contains("invalid transition"), "{relay}: {error}");
    }
}

#[test]
fn a_gate_of_another_open_session_is_refused_naming_session_use() {
    let fixture = project(&[SCOPE]);
    // A second session becomes active; the first stays open with its gate.
    fixture.ok(&["session", "new", "other-g2", "--objective", "Other work"]);
    let listed = list(&fixture);
    let gate = listed["gates"]
        .as_array()
        .unwrap()
        .iter()
        .find(|gate| gate["gate"] == SCOPE)
        .unwrap()
        .clone();
    // The list names the session by its ID, which `session use` takes as it is.
    let id = fixture.id_of(KEY);
    assert_eq!(gate["session"], id.as_str());
    let number = gate["number"].to_string();
    let list_id = listed["list"].as_str().unwrap().to_string();
    let before = records(fixture.root());
    let error = fixture.fails(&record(&list_id, &number, "yes"));
    assert!(
        error.contains(&format!("`chough session use {id}`")),
        "{error}"
    );
    assert_eq!(records(fixture.root()), before, "something was written");
    // The recovery it names succeeds.
    fixture.ok(&["session", "use", &id]);
    fixture.ok(&record(&list_id, &number, "yes"));
    assert_eq!(session(&fixture)["awaiting_human"], json!([]));
}

/// #190 (REQ-cli-061): a gate summary counts the change from the merge base with the
/// default branch and leaves out Chough's records, the knowledge directory and SpecSync
/// bookkeeping: a one-line product edit, 300 records and 200 files merged from main
/// count as one file, which the summary names.
#[test]
fn gate_summary_counts_only_the_changes_own_files() {
    let fixture = project(&[SCOPE]);
    fixture.commit("the product so far, on main");
    fixture.git(&["checkout", "-q", "-B", "main"]);
    fixture.git(&["checkout", "-q", "-b", "feature"]);
    fixture.write("src/product.txt", "one line\n");
    for index in 0..150 {
        fixture.write(
            &format!(".chough/sessions/2026-11-11-feature/evidence/2026-11-11-entry-{index}.md"),
            "---\nkind: evidence\n---\n",
        );
        fixture.write(
            &format!("knowledge/journal/2026-11-11-post-{index}.md"),
            "---\nkind: journal\n---\n",
        );
    }
    fixture.commit("the change, with its records");
    fixture.git(&["checkout", "-q", "main"]);
    for index in 0..200 {
        fixture.write(&format!("lib/merged-{index}.txt"), "from main\n");
    }
    fixture.commit("work on main after the base");
    fixture.git(&["checkout", "-q", "feature"]);
    fixture.git(&["merge", "-q", "--no-edit", "main"]);
    let listed = list(&fixture);
    let summary = listed["gates"][0]["summary"].as_str().unwrap();
    assert!(
        summary.contains("What changed: 1 files, +1 −0 since "),
        "{summary}"
    );
    assert!(summary.contains("  src/product.txt +1 −0"), "{summary}");
    assert!(!summary.contains("lib/merged-"), "{summary}");
}

/// D2 (REQ-cli-061): a second change on one branch counts from its own recorded base
/// commit, so the first change's work, archived earlier on the same branch, is not
/// counted again.
#[test]
fn a_second_change_on_one_branch_counts_from_its_own_base_commit() {
    let fixture = project(&[SCOPE]);
    fixture.commit("the product so far, on main");
    fixture.git(&["checkout", "-q", "-B", "main"]);
    fixture.git(&["checkout", "-q", "-b", "feature"]);
    for index in 0..3 {
        fixture.write(
            &format!("lib/first-change-{index}.txt"),
            "the first change\n",
        );
    }
    fixture.commit("the first change, archived on this branch");
    let second_base = git_head(&fixture);
    fixture.write(
        &format!("{DIR}/state.json"),
        &json!({"id": ID, "title": "Free shipping at the threshold", "state": "approved", "base_commit": second_base}).to_string(),
    );
    fixture.write("src/second.txt", "one line\n");
    fixture.commit("the second change");
    let listed = list(&fixture);
    let summary = listed["gates"][0]["summary"].as_str().unwrap();
    assert!(
        summary.contains(&format!(
            "What changed: 1 files, +1 −0 since {}",
            &second_base[..8]
        )),
        "{summary}"
    );
    assert!(!summary.contains("lib/first-change-"), "{summary}");
}

/// REQ-cli-061, TC-cli-gate-summary-says-when-there-is-no-base (G2): with no merge base
/// and no recorded base commit, the summary's Diff line says the base is unknown instead
/// of giving a `git diff unknown..<sha>` command that fails.
#[test]
fn a_summary_with_no_base_says_so_instead_of_a_broken_diff_command() {
    let fixture = project(&[SCOPE]);
    fixture.write(
        &format!("{DIR}/state.json"),
        &json!({"id": ID, "title": "Free shipping at the threshold", "state": "approved"})
            .to_string(),
    );
    fixture.commit("no recorded base");
    let listed = list(&fixture);
    let summary = listed["gates"][0]["summary"].as_str().unwrap();
    let diff = summary
        .lines()
        .find(|line| line.starts_with("Diff:"))
        .unwrap_or_else(|| panic!("no Diff line:\n{summary}"));
    assert!(!diff.contains("unknown.."), "{diff}");
    assert!(diff.contains("unknown"), "{diff}");
}

/// REQ-cli-061, TC-cli-gate-summary-shows-the-scope (t38j's fresh-eyes report): at a gate
/// that drives approval, the summary counts and names the change's own Markdown files
/// (its `change.md`, deltas and plan) first, since they are what the human approves,
/// while its `state.json` and `approvals.json` stay left out as bookkeeping.
#[test]
fn a_scope_gate_summary_names_the_changes_delta_and_plan_first() {
    let fixture = project(&[]);
    fixture.write(&format!("{DIR}/approvals.json"), "{}\n");
    fixture.write(
        &format!("{DIR}/change.md"),
        "# g1\n\n## Intent\n\nFree shipping.\n",
    );
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        SCOPE,
        "--drives",
        "approve",
    ]);
    fixture.commit("the scope to approve");
    let listed = list(&fixture);
    let summary = listed["gates"][0]["summary"].as_str().unwrap();
    let lines: Vec<&str> = summary.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.starts_with("What changed:"))
        .expect(summary);
    let named: Vec<&str> = lines[at + 1..]
        .iter()
        .take_while(|line| line.starts_with("  "))
        .map(|line| line.trim())
        .collect();
    for (index, file) in ["change.md", "deltas/cli.md", "plan.md"].iter().enumerate() {
        assert!(
            named
                .get(index)
                .is_some_and(|line| line.starts_with(&format!("{DIR}/"))),
            "the change's own files come first: {summary}"
        );
        assert!(
            summary.contains(&format!("{DIR}/{file} +")),
            "{file}: {summary}"
        );
    }
    assert!(!summary.contains("state.json"), "{summary}");
    assert!(!summary.contains("approvals.json"), "{summary}");
    assert!(summary.contains("src/input.txt"), "{summary}");
}

/// Fresh-eyes F6 (second round, 2026-11-12; REQ-cli-061): the list ended by suggesting
/// `chough gate record` for every gate, which refuses a gate whose change is verifying,
/// and that refusal said nothing may write .chough/ before the ship, which
/// verification_ignored_paths contradicts. The list ends with `chough gate record` for
/// the gates whose change is not verifying and `chough change ship <id> --list <list>
/// --gate <n>` for each verifying one, and the refusal says such a gate closes through
/// the ship.
#[test]
fn the_list_names_the_ship_for_a_verifying_gate_and_the_refusal_says_why() {
    let fixture = project(&[SCOPE]);
    // A second open session, now active, whose change is verifying, with its review gate.
    let review = "Implementation review of g2";
    fixture.ok(&["session", "new", "other-g2", "--objective", "Other work"]);
    fixture.ok(&["checkpoint", "--native-change", "g2"]);
    fixture.write(
        ".specsync/changes/g2/state.json",
        &json!({"id": "g2", "state": "verifying"}).to_string(),
    );
    fixture.ok(&["checkpoint", "--awaiting-human", review]);
    fixture.commit("g2 verifying");
    let output = fixture.run(&["gate", "list"]);
    assert!(output.status.success());
    let shown = String::from_utf8(output.stdout).unwrap();
    let listed = list(&fixture);
    let list_id = listed["list"].as_str().unwrap().to_string();
    let number = |gate: &str| {
        listed["gates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|listed| listed["gate"] == gate)
            .unwrap()["number"]
            .to_string()
    };
    let (scope, verifying) = (number(SCOPE), number(review));
    // The list ends with how to reply, after the last gate's digest.
    let footer = &shown[shown.rfind("Digest: ").unwrap()..];
    let reply = footer
        .lines()
        .find(|line| line.contains("`chough gate record "))
        .unwrap_or_else(|| panic!("no gate record in:\n{footer}"));
    assert!(reply.contains(&format!("for gate {scope} with")), "{reply}");
    assert!(
        footer.contains(&format!(
            "`chough change ship g2 --list {list_id} --gate {verifying} --decided-by"
        )),
        "{footer}"
    );

    let before = records(fixture.root());
    let error = fixture.fails(&record(&list_id, &verifying, "yes"));
    assert!(error.contains("closes through the ship"), "{error}");
    assert!(error.contains("runs the review as the human"), "{error}");
    assert!(!error.contains("nothing may write"), "{error}");
    assert!(
        error.contains(&format!(
            "`chough change ship g2 --list {list_id} --gate {verifying} --decided-by"
        )),
        "{error}"
    );
    assert_eq!(records(fixture.root()), before, "something was written");
}

/// Fresh-eyes round 3, F5 (2026-11-12; REQ-cli-061): at the scope gate the summary named
/// the five own files with the most changed lines, and deltas/<module>.md, the contract
/// change, fell outside them. The change's deltas come first, before its other files,
/// each with the requirement IDs it adds or modifies.
#[test]
fn a_scope_gate_summary_names_each_delta_first_with_its_requirements() {
    let fixture = project(&[]);
    fixture.write(&format!("{DIR}/approvals.json"), "{}\n");
    fixture.write(
        &format!("{DIR}/deltas/history.md"),
        "## MODIFIED\n\n### REQUIREMENT REQ-history-002\n\nThe Timeline SHALL read.\n\n## REMOVED\n\n### REQUIREMENT REQ-history-099\n",
    );
    // Larger own files than either delta, so a cap by changed lines leaves the deltas out.
    let long: String = (1..=40).map(|line| format!("- line {line}\n")).collect();
    for file in [
        "change.md",
        "plan.md",
        "testing.md",
        "tasks.md",
        "context.md",
    ] {
        fixture.write(&format!("{DIR}/{file}"), &format!("# {file}\n\n{long}"));
    }
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        SCOPE,
        "--drives",
        "approve",
    ]);
    fixture.commit("the scope to approve");
    let summary = list(&fixture)["gates"][0]["summary"]
        .as_str()
        .unwrap()
        .to_string();
    let lines: Vec<&str> = summary.lines().collect();
    let at = lines
        .iter()
        .position(|line| line.starts_with("What changed:"))
        .expect(&summary);
    let named: Vec<&str> = lines[at + 1..]
        .iter()
        .take_while(|line| line.starts_with("  "))
        .map(|line| line.trim())
        .collect();
    let mut deltas: Vec<&str> = named[..2].to_vec();
    deltas.sort_unstable();
    assert!(
        deltas[0].starts_with(&format!("{DIR}/deltas/cli.md +"))
            && deltas[0].ends_with("(adds or modifies REQ-cli-001)"),
        "{summary}"
    );
    assert!(
        deltas[1].starts_with(&format!("{DIR}/deltas/history.md +"))
            && deltas[1].ends_with("(adds or modifies REQ-history-002)"),
        "{summary}"
    );
    assert!(
        named[2].starts_with(&format!("{DIR}/")) && !named[2].contains("/deltas/"),
        "the change's other files follow its deltas: {summary}"
    );
    assert!(lines.len() <= 24 && summary.len() <= 4_096, "{summary}");
}

/// Fresh-eyes round 3, F4 (2026-11-12; REQ-cli-061): the intent gate's summary said
/// "Change: none recorded / What changed: unknown ...", nothing about the intent. At a gate
/// held at intent with no native change recorded, the summary gives the intent being
/// approved in place of what changed: the criterion IDs the session references and the
/// HI files that hold them.
#[test]
fn an_intent_gate_summary_names_the_criteria_and_their_hi_files() {
    let fixture = Fixture::new();
    fixture.write(
        "hi/life.md",
        "# Life\n\n- LIFE-1 I can resume where I left off.\n- LIFE-10 Something else.\n",
    );
    fixture.write(
        "hi/flow.md",
        "# Flow\n\n- FLOW-2 I can see the next step.\n",
    );
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--criterion",
        "LIFE-1",
        "--criterion",
        "FLOW-2",
        "--criterion",
        "LIFE-99",
        "--awaiting-human",
        "Approve the intent",
    ]);
    fixture.commit("the intent to approve");
    let summary = list(&fixture)["gates"][0]["summary"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(!summary.contains("Change: none recorded"), "{summary}");
    assert!(!summary.contains("What changed"), "{summary}");
    let lines: Vec<&str> = summary.lines().collect();
    let intent = lines
        .iter()
        .position(|line| line.starts_with("Intent: "))
        .expect(&summary);
    for id in ["LIFE-1", "FLOW-2", "LIFE-99"] {
        assert!(lines[intent].contains(id), "{summary}");
    }
    let held: Vec<&str> = lines[intent + 1..]
        .iter()
        .take_while(|line| line.starts_with("  "))
        .map(|line| line.trim())
        .collect();
    assert!(held.contains(&"hi/life.md: LIFE-1"), "{summary}");
    assert!(held.contains(&"hi/flow.md: FLOW-2"), "{summary}");
    assert!(held.contains(&"in no hi/ file: LIFE-99"), "{summary}");
    assert!(lines.len() <= 24 && summary.len() <= 4_096, "{summary}");
}

/// Fresh-eyes round 4, F2 (2026-11-12; REQ-cli-061): with the change's delta, plan and
/// other files written but not committed, the scope gate's summary said "What changed: 0
/// files, +0 −0" and named no delta; after a commit it named them. What changed is
/// counted in the working tree, so uncommitted and untracked files count: the summary
/// names the deltas first, with their requirements, as it does after the commit, leaves
/// Chough's records out, and gives the same digest for the same tree.
#[test]
fn a_scope_gate_summary_counts_uncommitted_and_untracked_change_files() {
    let fixture = project(&[]);
    // The change begins at HEAD, so only what is not committed yet is its work.
    let head = git_head(&fixture);
    fixture.write(
        &format!("{DIR}/state.json"),
        &json!({"id": ID, "title": "Free shipping at the threshold", "state": "approved", "base_commit": head}).to_string(),
    );
    fixture.write(&format!("{DIR}/approvals.json"), "{}\n");
    // An untracked delta, a committed delta and plan edited in place, and an untracked
    // product file.
    fixture.write(
        &format!("{DIR}/deltas/history.md"),
        "## MODIFIED\n\n### REQUIREMENT REQ-history-002\n\nThe Timeline SHALL read.\n",
    );
    fixture.write(
        &format!("{DIR}/deltas/cli.md"),
        "## ADDED\n\n### REQUIREMENT REQ-cli-001\n\nPreserve files.\n\n### REQUIREMENT REQ-cli-002\n\nKeep them.\n",
    );
    fixture.write(
        &format!("{DIR}/plan.md"),
        "# Plan\n\n## Approach\n\nx\ny\nz\n\n## Risks\n\n- A rounding edge at exactly fifty dollars.\n\n## Constraints consulted\n\nnone\n",
    );
    fixture.write("src/added.txt", "one\ntwo\nthree\n");
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        SCOPE,
        "--drives",
        "approve",
    ]);
    let changed = |summary: &str| -> Vec<String> {
        let lines: Vec<&str> = summary.lines().collect();
        let at = lines
            .iter()
            .position(|line| line.starts_with("What changed:"))
            .unwrap_or_else(|| panic!("{summary}"));
        std::iter::once(lines[at].split(" since ").next().unwrap())
            .chain(
                lines[at + 1..]
                    .iter()
                    .take_while(|line| line.starts_with("  "))
                    .copied(),
            )
            .map(String::from)
            .collect()
    };
    let listed = list(&fixture);
    let summary = listed["gates"][0]["summary"].as_str().unwrap().to_string();
    let uncommitted = changed(&summary);
    assert_eq!(uncommitted[0], "What changed: 4 files, +14 −0", "{summary}");
    let mut deltas: Vec<String> = uncommitted[1..3]
        .iter()
        .map(|line| line.trim().to_string())
        .collect();
    deltas.sort();
    assert!(
        deltas[0].starts_with(&format!("{DIR}/deltas/cli.md +"))
            && deltas[0].ends_with("(adds or modifies REQ-cli-001, REQ-cli-002)"),
        "{summary}"
    );
    assert!(
        deltas[1].starts_with(&format!("{DIR}/deltas/history.md +5 −0"))
            && deltas[1].ends_with("(adds or modifies REQ-history-002)"),
        "{summary}"
    );
    for named in [
        format!("{DIR}/plan.md +"),
        "src/added.txt +3 −0".to_string(),
    ] {
        assert!(
            uncommitted
                .iter()
                .any(|line| line.trim().starts_with(&named)),
            "{named}: {summary}"
        );
    }
    assert!(!summary.contains(".chough/"), "{summary}");
    assert!(
        summary.lines().count() <= 24 && summary.len() <= 4_096,
        "{summary}"
    );
    // The same tree gives the same summary and digest.
    let again = list(&fixture);
    assert_eq!(again["gates"][0]["digest"], listed["gates"][0]["digest"]);
    assert_eq!(again["gates"][0]["summary"], listed["gates"][0]["summary"]);

    // Committed, the change's files count the same.
    fixture.commit("the change's own files");
    let committed = list(&fixture)["gates"][0]["summary"]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(changed(&committed), uncommitted, "{committed}");
}
