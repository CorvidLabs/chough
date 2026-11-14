//! Relayed decisions at human gates (REQ-cli-050, REQ-history-010, FLOW-16): an agent
//! that records a human's decision on their behalf keeps who decided, their words,
//! where and which agent recorded it, as a claim that never stands in for native
//! approval or review.

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const BOUNDARY: &str = "a relayed claim, not authenticated: it records who is said to have decided, not proof that they did";

/// The four relay flags with their values, for a command line.
fn relay<'a>(decided_by: &'a str, words: &'a str, place: &'a str, agent: &'a str) -> Vec<&'a str> {
    vec![
        "--decided-by",
        decided_by,
        "--words",
        words,
        "--where",
        place,
        "--recorded-by",
        agent,
    ]
}

fn close<'a>(gate: &'a str, rest: &[&'a str]) -> Vec<&'a str> {
    let mut args = vec!["checkpoint", "--close-gate", gate];
    args.extend_from_slice(rest);
    args
}

/// Every file under `.chough/` and `.specsync/` with its bytes, leaving out the
/// lock directory, which a writer creates and Git ignores.
fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries {
            let path = entry.unwrap().path();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if relative.starts_with(".chough/local") {
                continue;
            }
            if path.is_dir() {
                walk(root, &path, files);
            } else {
                files.insert(relative, fs::read(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    for top in [".chough", ".specsync"] {
        walk(root, &root.join(top), &mut files);
    }
    files
}

/// Timeline entries of the active session, as `chough history show` reads session.md.
fn events(fixture: &Fixture) -> Vec<Value> {
    fixture.ok(&["history", "show"])["entries"]
        .as_array()
        .unwrap()
        .clone()
}

fn new_history_project() -> Fixture {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.ok(&["init", "--objective", "Relay decisions", "--module", "cli"]);
    assert!(!fixture.session_file().is_empty());
    fixture
}

#[test]
fn a_relayed_decision_closes_only_its_gate_and_keeps_the_four_values() {
    let fixture = new_history_project();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Intent approval",
        "--awaiting-human",
        "Design approval",
    ]);
    let values = relay(
        "SYNTHETIC-human",
        "Approve intent + approach",
        "orc tab, 2026-11-05 20:18 MST, relayed by orc t18",
        "SYNTHETIC-agent ch-d7",
    );
    let output = fixture.ok(&close("Intent approval", &values));
    let expected = json!({
        "decided_by": "SYNTHETIC-human",
        "words": "Approve intent + approach",
        "where": "orc tab, 2026-11-05 20:18 MST, relayed by orc t18",
        "recorded_by": "SYNTHETIC-agent ch-d7",
    });
    assert_eq!(output["saved"], true);
    assert_eq!(output["relayed_decision"]["gate"], "Intent approval");
    assert_eq!(output["relayed_decision"]["relayed_by"], expected);
    assert_eq!(output["relayed_decision"]["provenance"], BOUNDARY);

    // Only that gate closed; the other still holds forward moves.
    assert_eq!(
        fixture.session()["awaiting_human"],
        json!(["Design approval"])
    );
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:awaiting-human"), "{refused}");
    assert!(refused.contains("Design approval"), "{refused}");
    assert!(!refused.contains("Intent approval"), "{refused}");

    // One gate-closed event carries the relay, byte for byte; no other event does.
    let relayed: Vec<Value> = events(&fixture)
        .into_iter()
        .filter(|entry| entry.get("relayed_by").is_some())
        .collect();
    assert_eq!(relayed.len(), 1, "{relayed:?}");
    assert_eq!(relayed[0]["kind"], "gate-closed");
    // A gate-closed entry's text is its gate, read back as the reason.
    assert_eq!(relayed[0]["reason"], "Intent approval");
    assert_eq!(relayed[0]["relayed_by"], expected);
    let last = events(&fixture).last().unwrap().clone();
    assert_eq!(last["relayed_by"], expected);

    // A fresh process hands the decision off, marked as a claim.
    let decision = "SYNTHETIC-human decided \"Intent approval\": \"Approve intent + approach\" (orc tab, 2026-11-05 20:18 MST, relayed by orc t18); recorded by SYNTHETIC-agent ch-d7; a relayed claim, not authenticated";
    assert_eq!(fixture.session()["decisions"], json!([decision]));
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["decisions"]["items"][0]["text"], decision);
    let text = String::from_utf8(fixture.run(&["next"]).stdout).unwrap();
    assert!(text.contains(decision), "{text}");
}

/// Each refusal saves nothing and writes no transition, and the recovery it names
/// then succeeds.
#[test]
fn relay_refusals_save_nothing_and_their_recovery_succeeds() {
    let fixture = new_history_project();
    let full = relay("SYNTHETIC-human", "Approve", "orc tab", "SYNTHETIC-agent");
    let refuses = |args: &[&str], expected: &[&str]| {
        let before = snapshot(fixture.root());
        let error = fixture.fails(args);
        for text in expected {
            assert!(error.contains(text), "{args:?}: {text}: {error}");
        }
        assert!(before == snapshot(fixture.root()), "{args:?} wrote records");
    };

    // Missing values: the refusal names them and the complete command.
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval"]);
    refuses(
        &close(
            "Scope approval",
            &["--decided-by", "SYNTHETIC-human", "--words", "Approve"],
        ),
        &[
            "refused:relay-incomplete",
            "--where",
            "--recorded-by",
            "chough checkpoint --close-gate \"Scope approval\" --decided-by",
            "nothing was saved",
        ],
    );
    // A blank value is missing too.
    refuses(
        &close(
            "Scope approval",
            &relay("SYNTHETIC-human", "Approve", "   ", "SYNTHETIC-agent"),
        ),
        &["refused:relay-incomplete", "--where"],
    );
    fixture.ok(&close("Scope approval", &full));
    assert_eq!(fixture.session()["awaiting_human"], json!([]));

    // Relay values without a gate.
    fixture.ok(&["checkpoint", "--awaiting-human", "Review of x"]);
    refuses(
        &["checkpoint", "--decided-by", "SYNTHETIC-human"],
        &["refused:relay-without-gate", "--close-gate", "chough next"],
    );
    fixture.ok(&close("Review of x", &full));

    // A gate that is not open: `chough next` lists the open gates, and closing the
    // listed one succeeds.
    fixture.ok(&["checkpoint", "--awaiting-human", "Release approval"]);
    refuses(
        &close("Release aproval", &full),
        &["refused:gate-not-open", "Release aproval", "chough next"],
    );
    let listed = fixture.ok(&["next", "--json"])["awaiting_human"]["items"][0]
        .as_str()
        .unwrap()
        .to_string();
    assert_eq!(listed, "Release approval");
    fixture.ok(&close(&listed, &full));

    // A decision longer than the 4096-byte checkpoint text bound.
    fixture.ok(&["checkpoint", "--awaiting-human", "Long gate"]);
    let long = "w".repeat(4_000);
    refuses(
        &close(
            "Long gate",
            &relay("SYNTHETIC-human", &long, "orc tab", "SYNTHETIC-agent"),
        ),
        &["refused:relay-too-long", "4096", "--words", "--where"],
    );
    fixture.ok(&close("Long gate", &full));

    // `--close-gate` does not combine with withdrawing or opening gates.
    fixture.ok(&["checkpoint", "--awaiting-human", "Last gate"]);
    for extra in [
        vec!["--clear-awaiting-human"],
        vec!["--awaiting-human", "Another gate"],
    ] {
        let mut args = close("Last gate", &full);
        args.extend(extra);
        refuses(&args, &["--close-gate"]);
    }
    fixture.ok(&close("Last gate", &full));
    assert_eq!(fixture.session()["awaiting_human"], json!([]));
    assert_eq!(fixture.session()["decisions"].as_array().unwrap().len(), 5);
}

/// A relay is not a native approval: SpecSync files stay byte-identical, no native
/// command runs, and the build gate still refuses a draft change.
#[cfg(unix)]
#[test]
fn a_relayed_decision_never_touches_native_state_or_its_gate() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.stubs();
    fixture.native("draft");
    fixture.init();
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["checkpoint", "--native-change", "x"]);
    fixture.write(
        ".specsync/changes/x/state.json",
        "{\"id\":\"x\",\"state\":\"draft\"}\n",
    );
    // A gate that drives nothing (REQ-cli-030) closes with no native step.
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Scope check of x",
        "--drives",
        "none",
    ]);
    let before: BTreeMap<String, Vec<u8>> = snapshot(fixture.root())
        .into_iter()
        .filter(|(path, _)| path.starts_with(".specsync"))
        .collect();
    fixture.write(".stub-count", "");
    fixture.ok(&close(
        "Scope check of x",
        &relay(
            "SYNTHETIC-human",
            "Approve the scope",
            "orc tab",
            "SYNTHETIC-agent",
        ),
    ));
    assert_eq!(fixture.native_calls(), 0, "a relay ran SpecSync");
    let after: BTreeMap<String, Vec<u8>> = snapshot(fixture.root())
        .into_iter()
        .filter(|(path, _)| path.starts_with(".specsync"))
        .collect();
    assert!(before == after, "a relay changed SpecSync files");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert_eq!(fixture.session()["stage"]["current"], "plan");
    // The scope gate, held at plan, drives the approval: closing it runs the approval
    // once as the configured human (REQ-records-008), and the advance then reads
    // native state as usual.
    fixture.write(
        ".chough/local/identity.toml",
        "[[human]]\nname = \"SYNTHETIC Human\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-approver\"\n",
    );
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval of x"]);
    fixture.native_calls();
    fixture.ok(&close(
        "Scope approval of x",
        &relay(
            "SYNTHETIC-human",
            "Approve the scope",
            "orc tab",
            "SYNTHETIC-agent",
        ),
    ));
    let calls = fixture.read(".stub-calls");
    assert!(
        calls.starts_with("change approve x --actor=SYNTHETIC-approver --note="),
        "{calls}"
    );
    assert_eq!(fixture.native_calls(), 1, "{calls}");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
}

/// Withdrawing gates keeps working and records no relay and no decision; closing the
/// only gate with a relay lets the same advance apply, in the legacy log too.
#[test]
fn clearing_records_no_relay_and_a_relayed_close_lets_advance_apply() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Gate A",
        "--awaiting-human",
        "Gate B",
    ]);
    fixture.ok(&["checkpoint", "--clear-awaiting-human"]);
    assert_eq!(fixture.session()["decisions"], json!([]));
    let cleared: Vec<Value> = events(&fixture);
    assert!(
        cleared
            .iter()
            .all(|entry| entry.get("relayed_by").is_none())
    );
    assert_eq!(
        cleared
            .iter()
            .filter(|entry| entry["kind"] == "gate-closed")
            .count(),
        2
    );

    fixture.ok(&["checkpoint", "--awaiting-human", "Intent approval"]);
    fixture.ok(&close(
        "Intent approval",
        &relay("SYNTHETIC-human", "Approved", "chat", "SYNTHETIC-agent"),
    ));
    let all = events(&fixture);
    let last = all.last().unwrap().clone();
    assert_eq!(last["kind"], "gate-closed");
    assert_eq!(
        last["relayed_by"],
        json!({"decided_by": "SYNTHETIC-human", "words": "Approved", "where": "chat", "recorded_by": "SYNTHETIC-agent"})
    );
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(fixture.session()["stage"]["current"], "design");
    // Records without a relay carry none: one relayed entry in all.
    let all = events(&fixture);
    assert_eq!(
        all.iter()
            .filter(|entry| entry.get("relayed_by").is_some())
            .count(),
        1
    );
}

/// An invalid relay on a Timeline entry is an invalid transition payload
/// (REQ-history-010): a good relay reads back; each invalid variant is refused.
#[test]
fn invalid_relays_are_refused_as_invalid_payloads() {
    let good = json!({"decided_by": "SYNTHETIC-human", "words": "Approved", "where": "chat", "recorded_by": "SYNTHETIC-agent"});
    let mut variants = vec![];
    for key in ["decided_by", "words", "where", "recorded_by"] {
        let mut missing = good.clone();
        missing.as_object_mut().unwrap().remove(key);
        variants.push(("gate-closed", missing));
        let mut blank = good.clone();
        blank[key] = json!(" ");
        variants.push(("gate-closed", blank));
        let mut long = good.clone();
        long[key] = json!("x".repeat(4_097));
        variants.push(("gate-closed", long));
    }
    let mut unknown = good.clone();
    unknown["signature"] = json!("abc");
    variants.push(("gate-closed", unknown));
    variants.push(("gate-opened", good.clone()));
    variants.push(("advance", good.clone()));

    let fixture = new_history_project();
    let entry = |kind: &str, relay: &Value| {
        let (from, to) = if kind == "advance" {
            ("intent", "design")
        } else {
            ("intent", "intent")
        };
        format!(
            "- 2026-11-11T00:00:00Z · {kind} · {from} → {to} · Gate ^t-aaaaaa\n  - relayed_by: {}\n",
            serde_json::to_string(relay).unwrap()
        )
    };
    let file = fixture.session_file();
    let original = fixture.read(&file);
    fixture.write(
        &file,
        &format!("{original}\n{}", entry("gate-closed", &good)),
    );
    let shown = events(&fixture);
    assert_eq!(shown.last().unwrap()["relayed_by"], good);
    for (kind, relay) in &variants {
        fixture.write(&file, &format!("{original}\n{}", entry(kind, relay)));
        let error = fixture.fails(&["history", "show"]);
        assert!(
            error.contains("invalid transition"),
            "{kind} {relay}: {error}"
        );
    }
}

/// REQ-history-002, TC-history-relay-fields-are-labelled-lines (t38j's fresh-eyes
/// report): a relayed decision's `relayed_by` is written as labelled lines under its
/// field, never a JSON object, with a value that starts with `"` written as a JSON
/// string; `history show` reads both that entry and an earlier one whose value is JSON.
#[test]
fn a_relayed_decision_is_written_as_labelled_lines_and_reads_back() {
    let fixture = new_history_project();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Pick a colour",
        "--awaiting-human",
        "Pick a size",
    ]);
    fixture.ok(&close(
        "Pick a colour",
        &relay(
            "SYNTHETIC-human",
            "Blue: the calm one",
            "a test, 2026-11-12",
            "SYNTHETIC-agent",
        ),
    ));
    let file = fixture.session_file();
    let text = fixture.read(&file);
    assert!(!text.contains("relayed_by: {"), "{text}");
    assert!(
        text.contains(
            "  - relayed_by:\n    - decided_by: SYNTHETIC-human\n    - words: Blue: the calm one\n    - where: a test, 2026-11-12\n    - recorded_by: SYNTHETIC-agent\n"
        ),
        "{text}"
    );
    fixture.ok(&close(
        "Pick a size",
        &relay(
            "SYNTHETIC-human",
            "\"Large\" please",
            "a test",
            "SYNTHETIC-agent",
        ),
    ));
    let text = fixture.read(&file);
    assert!(
        text.contains("    - words: \"\\\"Large\\\" please\"\n"),
        "{text}"
    );
    // An entry written before this rule, with a JSON value, still reads.
    fixture.write(
        &file,
        &format!(
            "{}- 2026-11-01T00:00:00Z · gate-closed · intent → intent · An old gate ^t-0abc00\n  - relayed_by: {{\"decided_by\":\"SYNTHETIC-human\",\"words\":\"ok\",\"where\":\"old\",\"recorded_by\":\"SYNTHETIC-agent\"}}\n",
            text
        ),
    );
    let closed: Vec<Value> = events(&fixture)
        .into_iter()
        .filter(|event| event["kind"] == "gate-closed")
        .collect();
    let words: Vec<&str> = closed
        .iter()
        .map(|event| event["relayed_by"]["words"].as_str().unwrap_or_default())
        .collect();
    assert_eq!(
        words,
        ["Blue: the calm one", "\"Large\" please", "ok"],
        "{closed:#?}"
    );
    assert_eq!(closed[0]["relayed_by"]["where"], "a test, 2026-11-12");
}

/// Fresh-eyes round 3, F6 (2026-11-12; REQ-cli-050): closing a gate by a near-miss text
/// gave refused:gate-not-open with no "did you mean". The refusal names the closest open
/// gate when one is close, and none when no open gate is; nothing is saved either way.
#[test]
fn a_near_miss_gate_close_names_the_closest_open_gate() {
    let fixture = new_history_project();
    let full = relay("SYNTHETIC-human", "Approve", "orc tab", "SYNTHETIC-agent");
    fixture.ok(&["checkpoint", "--awaiting-human", "Release approval"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval"]);
    for (given, suggested) in [
        ("Release aproval", Some("Release approval")),
        ("release approval", Some("Release approval")),
        ("Scope  approvl", Some("Scope approval")),
        ("Ship it to production now", None),
    ] {
        let before = snapshot(fixture.root());
        let error = fixture.fails(&close(given, &full));
        assert!(error.contains("refused:gate-not-open"), "{given}: {error}");
        match suggested {
            Some(gate) => assert!(
                error.contains(&format!("Did you mean \"{gate}\"?")),
                "{given}: {error}"
            ),
            None => assert!(!error.contains("Did you mean"), "{given}: {error}"),
        }
        assert!(before == snapshot(fixture.root()), "{given} wrote records");
    }
}
