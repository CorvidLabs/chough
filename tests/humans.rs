//! A human's gate decision drives the native approval or review as that human
//! (REQ-records-008, #236), and an open gate holds only the stage it names
//! (REQ-cli-030, #237).

mod common;

use common::{Fixture, USER_CONFIG};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::Path;

const ID: &str = "human-change";
const SCOPE: &str = "Scope approval";
const REVIEW: &str = "Implementation review";
const HUMANS: &str = "[[human]]\nname = \"SYNTHETIC Person\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-actor\"\n";

/// A project with SpecSync stubbed, a native change recorded, one configured human and
/// the stub's calls counted from here.
#[cfg(unix)]
fn project() -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.write(".chough/local/identity.toml", HUMANS);
    fixture.write(".stub-count", "");
    fixture
}

fn relay(gate: &str, who: &str) -> Vec<String> {
    [
        "checkpoint",
        "--close-gate",
        gate,
        "--decided-by",
        who,
        "--words",
        "yes, go",
        "--where",
        "SYNTHETIC chat, 2026-11-11",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]
    .map(String::from)
    .to_vec()
}

fn args(list: &[String]) -> Vec<&str> {
    list.iter().map(String::as_str).collect()
}

/// The stub's calls since the count started, one line each.
fn calls(fixture: &Fixture) -> Vec<String> {
    if !fixture.exists(".stub-calls") {
        return vec![];
    }
    fixture
        .read(".stub-calls")
        .lines()
        .map(String::from)
        .collect()
}

/// Every file under `.chough/` except local state, by path.
fn records(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, relative: &str, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if name.starts_with(".chough/local") {
                continue;
            }
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else {
                out.insert(name, std::fs::read(entry.path()).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, ".chough", &mut out);
    out
}

fn decisions(fixture: &Fixture) -> Vec<String> {
    fixture.checkpoint()["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .map(|decision| decision.as_str().unwrap().to_string())
        .collect()
}

#[cfg(unix)]
#[test]
fn a_scope_gate_runs_the_approval_once_as_the_configured_human() {
    let fixture = project();
    fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    // The gate-opened entry records what the gate holds and drives (REQ-cli-030).
    let text = fixture.read(&fixture.session_file());
    assert!(
        text.contains(&format!(" · gate-opened · intent → intent · {SCOPE} ^"))
            && text.contains("  - holds: plan\n  - drives: approve\n"),
        "{text}"
    );
    let gates = fixture.ok(&["next", "--json"])["gates"].clone();
    assert_eq!(
        gates,
        serde_json::json!([{"gate": SCOPE, "holds": "plan", "drives": "approve"}])
    );
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let closed = fixture.ok(&args(&relay(SCOPE, "SYNTHETIC-human")));
    let calls = calls(&fixture);
    assert_eq!(calls.len(), 1, "{calls:?}");
    assert!(
        calls[0].starts_with(&format!(
            "change approve {ID} --actor=SYNTHETIC-actor --note=Approved by SYNTHETIC Person at the Chough gate \"{SCOPE}\": \"yes, go\" (SYNTHETIC chat, 2026-11-11). Relayed and recorded by SYNTHETIC-agent at "
        )),
        "{calls:?}"
    );
    // #249 (REQ-records-008): the relay also goes to SpecSync 6.3.0's own fields, so its
    // approval record holds the relayed decision.
    assert!(
        calls[0].ends_with(
            "; a relayed claim, not authenticated. --words=yes, go --where=SYNTHETIC chat, 2026-11-11 --recorded-by=SYNTHETIC-agent"
        ),
        "{calls:?}"
    );
    let step = &closed["relayed_decision"]["native_step"];
    assert_eq!(
        step["ran"],
        format!("ran specsync change approve {ID} as SYNTHETIC-actor")
    );
    assert_eq!(step["human"], "SYNTHETIC Person");
    assert_eq!(
        fixture.checkpoint()["awaiting_human"],
        serde_json::json!([])
    );
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with(&format!(
            "; ran specsync change approve {ID} as SYNTHETIC-actor"
        )),
        "{decision}"
    );
}

#[cfg(unix)]
#[test]
fn a_review_gate_needs_a_verdict_and_runs_the_review_with_it() {
    let fixture = project();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        REVIEW,
        "--holds",
        "accept",
    ]);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let before = records(fixture.root());
    let refused = fixture.fails(&args(&relay(REVIEW, "SYNTHETIC-human")));
    assert!(refused.contains("refused:verdict-required"), "{refused}");
    assert_eq!(records(fixture.root()), before);
    assert!(calls(&fixture).is_empty());
    let mut with = relay(REVIEW, "SYNTHETIC-human");
    with.extend(["--verdict".into(), "block".into()]);
    fixture.ok(&args(&with));
    // #249 (REQ-records-008): the review carries the relay in SpecSync 6.3.0's fields.
    assert_eq!(
        calls(&fixture),
        [format!(
            "change review {ID} --reviewer=SYNTHETIC-actor --verdict=block --words=yes, go --where=SYNTHETIC chat, 2026-11-11 --recorded-by=SYNTHETIC-agent"
        )]
    );
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with("as SYNTHETIC-actor with verdict block"),
        "{decision}"
    );
}

/// #249 review (REQ-records-008): `checkpoint --close-gate` accepts relay words that start
/// with `-`, and the native review gets each relay field joined to its flag, so SpecSync's
/// parser reads `-x, go` as the words, not as a flag.
#[cfg(unix)]
#[test]
fn a_relay_that_starts_with_a_hyphen_reaches_the_native_review_joined_to_its_flag() {
    let fixture = project();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        REVIEW,
        "--holds",
        "accept",
    ]);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let mut with = relay(REVIEW, "SYNTHETIC-human");
    let words = with.iter().position(|value| value == "yes, go").unwrap();
    with[words] = "-x, go".into();
    with.extend(["--verdict".into(), "pass".into()]);
    fixture.ok(&args(&with));
    assert_eq!(
        calls(&fixture),
        [format!(
            "change review {ID} --reviewer=SYNTHETIC-actor --verdict=pass --words=-x, go --where=SYNTHETIC chat, 2026-11-11 --recorded-by=SYNTHETIC-agent"
        )]
    );
}

#[cfg(unix)]
#[test]
fn no_native_closes_a_driving_gate_and_runs_nothing() {
    let fixture = project();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        REVIEW,
        "--holds",
        "accept",
    ]);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let mut changes = relay(REVIEW, "SYNTHETIC-human");
    changes.push("--no-native".into());
    fixture.ok(&args(&changes));
    assert!(calls(&fixture).is_empty());
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with("; no native review ran (--no-native)"),
        "{decision}"
    );
    // A gate that drives nothing closes with no native call, still in a configured
    // human's name (REQ-records-008).
    fixture.ok(&["checkpoint", "--awaiting-human", "Name the release"]);
    fixture.ok(&args(&relay("Name the release", "SYNTHETIC-human")));
    assert!(calls(&fixture).is_empty());
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with("a relayed claim, not authenticated"),
        "{decision}"
    );
}

/// F1 of the second fresh-eyes round (REQ-records-008): every gate close names a
/// configured human, whatever the gate drives and with or without `--no-native`, so an
/// intent or design approval cannot be recorded in any name.
#[cfg(unix)]
#[test]
fn every_gate_close_names_a_configured_human() {
    const INTENT: &str = "Approve the intent";
    let fixture = project();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        INTENT,
        "--holds",
        "intent",
    ]);
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        REVIEW,
        "--holds",
        "accept",
    ]);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let before = records(fixture.root());
    let stranger = "someone not configured";
    let refused = fixture.fails(&args(&relay(INTENT, stranger)));
    assert!(
        refused.contains("refused:human-identity-unknown"),
        "a gate that drives nothing took any name: {refused}"
    );
    let mut no_native = relay(REVIEW, stranger);
    no_native.push("--no-native".into());
    let refused = fixture.fails(&args(&no_native));
    assert!(
        refused.contains("refused:human-identity-unknown"),
        "--no-native took any name: {refused}"
    );
    let listed = fixture.ok(&["gate", "list", "--json"]);
    let list = listed["list"].as_str().unwrap().to_string();
    let number = listed["gates"]
        .as_array()
        .unwrap()
        .iter()
        .find(|listed| listed["gate"] == INTENT)
        .unwrap()["number"]
        .to_string();
    let refused = fixture.fails(&[
        "gate",
        "record",
        &list,
        "--gates",
        &number,
        "--decided-by",
        stranger,
        "--words",
        "yes",
        "--where",
        "SYNTHETIC chat",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    assert!(
        refused.contains("refused:human-identity-unknown"),
        "gate record took any name: {refused}"
    );
    assert!(calls(&fixture).is_empty(), "a SpecSync process started");
    assert_eq!(records(fixture.root()), before, "something was written");
    // A configured human closes both, and still nothing native runs.
    fixture.ok(&args(&relay(INTENT, "SYNTHETIC-human")));
    let mut no_native = relay(REVIEW, "SYNTHETIC-human");
    no_native.push("--no-native".into());
    fixture.ok(&args(&no_native));
    assert!(calls(&fixture).is_empty());
    assert_eq!(
        fixture.checkpoint()["awaiting_human"],
        serde_json::json!([])
    );
}

#[cfg(unix)]
#[test]
fn a_specsync_refusal_passes_through_and_nothing_is_written() {
    let fixture = project();
    fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    fixture.write(".stub-fail", "");
    let before = records(fixture.root());
    let output = fixture.run(&args(&relay(SCOPE, "SYNTHETIC-human")));
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&output.stderr), "status failed\n");
    assert_eq!(records(fixture.root()), before, "something was written");
    std::fs::remove_file(fixture.root().join(".stub-fail")).unwrap();
    assert_eq!(
        fixture.checkpoint()["awaiting_human"],
        serde_json::json!([SCOPE])
    );
}

#[cfg(unix)]
#[test]
fn an_unknown_or_ambiguous_human_is_refused_before_anything_runs() {
    let fixture = project();
    fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let before = records(fixture.root());
    let refused = fixture.fails(&args(&relay(SCOPE, "nobody")));
    assert!(
        refused.contains("refused:human-identity-unknown"),
        "{refused}"
    );
    assert!(refused.contains(".chough/local/identity.toml"), "{refused}");
    assert!(refused.contains("XDG_CONFIG_HOME"), "{refused}");
    fixture.write(
        ".chough/local/identity.toml",
        &format!("{HUMANS}\n[[human]]\nname = \"Twin\"\naliases = [\"synthetic-human\"]\nspecsync_actor = \"twin\"\n"),
    );
    let refused = fixture.fails(&args(&relay(SCOPE, "SYNTHETIC-human")));
    assert!(refused.contains("names 2 configured humans"), "{refused}");
    assert!(calls(&fixture).is_empty(), "a SpecSync process started");
    assert_eq!(records(fixture.root()), before);
}

#[cfg(unix)]
#[test]
fn the_project_file_overrides_the_user_file() {
    let fixture = project();
    std::fs::remove_file(fixture.root().join(".chough/local/identity.toml")).unwrap();
    fixture.write(
        &format!("{USER_CONFIG}/chough/identity.toml"),
        "[[human]]\nname = \"From the user file\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"user-actor\"\n",
    );
    for gate in ["First scope", "Second scope"] {
        fixture.ok(&["checkpoint", "--awaiting-human", gate, "--holds", "plan"]);
    }
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    fixture.ok(&args(&relay("First scope", "SYNTHETIC-human")));
    assert!(
        calls(&fixture)[0].contains("--actor=user-actor "),
        "{:?}",
        calls(&fixture)
    );
    fixture.write(".chough/local/identity.toml", HUMANS);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    fixture.ok(&args(&relay("Second scope", "SYNTHETIC-human")));
    assert!(
        calls(&fixture)[0].contains("--actor=SYNTHETIC-actor "),
        "{:?}",
        calls(&fixture)
    );
}

#[cfg(unix)]
#[test]
fn one_gate_record_runs_at_most_one_native_step() {
    let fixture = project();
    for (gate, holds) in [
        (SCOPE, "plan"),
        ("Second scope", "plan"),
        ("Name it", "intent"),
    ] {
        fixture.ok(&["checkpoint", "--awaiting-human", gate, "--holds", holds]);
    }
    let listed = fixture.ok(&["gate", "list", "--json"]);
    let list = listed["list"].as_str().unwrap().to_string();
    let number = |gate: &str| {
        listed["gates"]
            .as_array()
            .unwrap()
            .iter()
            .find(|listed| listed["gate"] == gate)
            .unwrap()["number"]
            .to_string()
    };
    assert_eq!(listed["gates"][0]["drives"], "approve");
    let reply = |gates: &str| {
        [
            "gate",
            "record",
            &list,
            "--gates",
            gates,
            "--decided-by",
            "SYNTHETIC-human",
            "--words",
            "yes",
            "--where",
            "SYNTHETIC chat",
            "--recorded-by",
            "SYNTHETIC-agent",
        ]
        .map(String::from)
        .to_vec()
    };
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let before = records(fixture.root());
    let both = format!("{},{}", number(SCOPE), number("Second scope"));
    let refused = fixture.fails(&args(&reply(&both)));
    assert!(
        refused.contains("refused:several-native-steps"),
        "{refused}"
    );
    assert!(calls(&fixture).is_empty());
    assert_eq!(records(fixture.root()), before);
    // The driving gate's step runs once, first; both gates close.
    let one = format!("{},{}", number("Name it"), number(SCOPE));
    fixture.ok(&args(&reply(&one)));
    let calls = calls(&fixture);
    assert_eq!(calls.len(), 1, "{calls:?}");
    assert!(calls[0].starts_with(&format!("change approve {ID} --actor=SYNTHETIC-actor")));
    assert_eq!(
        fixture.checkpoint()["awaiting_human"],
        serde_json::json!(["Second scope"])
    );
}

/// The fresh-eyes finding on gate record: a reply that closes a gate driving approval
/// reports the step it ran, and the approval note names the command the reply came
/// through, `chough gate record <list>`, not `chough checkpoint --close-gate`.
#[cfg(unix)]
#[test]
fn gate_record_reports_the_approval_it_ran_and_its_note_names_gate_record() {
    let fixture = project();
    fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    let listed = fixture.ok(&["gate", "list", "--json"]);
    let list = listed["list"].as_str().unwrap().to_string();
    assert_eq!(listed["gates"][0]["drives"], "approve");
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    let reply = fixture.ok(&[
        "gate",
        "record",
        &list,
        "--gates",
        "1",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "yes, go",
        "--where",
        "SYNTHETIC chat, 2026-11-11",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    let closed = &reply["closed"][0];
    assert_eq!(closed["gate"], SCOPE, "{reply:#}");
    assert_eq!(
        closed["native_step"]["ran"],
        format!("ran specsync change approve {ID} as SYNTHETIC-actor"),
        "{reply:#}"
    );
    assert_eq!(
        closed["native_step"]["specsync_actor"], "SYNTHETIC-actor",
        "{reply:#}"
    );
    let calls = calls(&fixture);
    assert_eq!(calls.len(), 1, "{calls:?}");
    // #249 (REQ-records-008): the note names gate record, and the relay follows it in
    // SpecSync 6.3.0's own fields.
    assert!(
        calls[0].contains("Relayed and recorded by SYNTHETIC-agent at ")
            && calls[0].ends_with(&format!(
                " through chough gate record {list}; a relayed claim, not authenticated. --words=yes, go --where=SYNTHETIC chat, 2026-11-11 --recorded-by=SYNTHETIC-agent"
            )),
        "{calls:?}"
    );
}

/// Fresh-eyes finding F12b (REQ-cli-050, REQ-records-008): a close that runs no native
/// step says why in its `native_step`, for a gate that drives none and for a close with
/// `--no-native`, through `checkpoint --close-gate` and `gate record` alike, and nothing
/// runs.
#[cfg(unix)]
#[test]
fn a_close_that_runs_no_native_step_says_why() {
    const GO: &str = "SYNTHETIC go-ahead";
    let fixture = project();
    let drives_none =
        serde_json::json!({"ran": null, "why": "no native step ran (the gate drives none)"});
    let no_native = serde_json::json!({"ran": null, "why": "no native approval ran (--no-native)"});
    let open = || {
        fixture.ok(&["checkpoint", "--awaiting-human", GO, "--drives", "none"]);
        fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    };
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));

    // checkpoint --close-gate, one gate at a time.
    open();
    let closed = fixture.ok(&args(&relay(GO, "SYNTHETIC-human")));
    assert_eq!(
        closed["relayed_decision"]["native_step"], drives_none,
        "{closed:#}"
    );
    let mut list = relay(SCOPE, "SYNTHETIC-human");
    list.push("--no-native".into());
    let closed = fixture.ok(&args(&list));
    assert_eq!(
        closed["relayed_decision"]["native_step"], no_native,
        "{closed:#}"
    );

    // gate record, both gates in one reply.
    open();
    let listed = fixture.ok(&["gate", "list", "--json"]);
    assert_eq!(listed["gates"][0]["gate"], GO, "{listed:#}");
    assert_eq!(listed["gates"][1]["gate"], SCOPE, "{listed:#}");
    let list = listed["list"].as_str().unwrap().to_string();
    let reply = fixture.ok(&[
        "gate",
        "record",
        &list,
        "--gates",
        "1,2",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "yes, go",
        "--where",
        "SYNTHETIC chat, 2026-11-11",
        "--recorded-by",
        "SYNTHETIC-agent",
        "--no-native",
    ]);
    let step = |gate: &str| {
        reply["closed"]
            .as_array()
            .unwrap()
            .iter()
            .find(|closed| closed["gate"] == gate)
            .unwrap_or_else(|| panic!("{gate} was not closed: {reply:#}"))["native_step"]
            .clone()
    };
    assert_eq!(step(GO), drives_none, "{reply:#}");
    assert_eq!(step(SCOPE), no_native, "{reply:#}");
    assert!(calls(&fixture).is_empty(), "{:?}", calls(&fixture));
}

/// #237: a gate that holds a later stage does not stop moves out of earlier ones; it
/// holds the stage it names.
#[test]
fn a_gate_holds_only_the_stage_it_names() {
    let fixture = Fixture::new();
    fixture.init();
    while fixture.checkpoint()["stage"]["current"] != "build" {
        fixture.ok(&["stage", "advance", "--json"]);
    }
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Publish the release",
        "--holds",
        "release",
    ]);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(fixture.checkpoint()["stage"]["current"], "accept");
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        next["gates"],
        serde_json::json!([{"gate": "Publish the release", "holds": "release", "drives": "none"}])
    );
    let text = String::from_utf8_lossy(&fixture.run(&["next"]).stdout).to_string();
    assert!(
        text.contains("  - Publish the release (holds release; drives none)"),
        "{text}"
    );
    assert!(!text.contains("Waiting for a human decision"), "{text}");
    // A gate holding build stops `advance --to` from plan, which leaves build.
    fixture.ok(&[
        "stage", "reset", "--json", "--to", "plan", "--reason", "Rework",
    ]);
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Build go",
        "--holds",
        "build",
    ]);
    let before = fixture.read(&fixture.session_file());
    let refused = fixture.fails(&["stage", "advance", "--to", "accept"]);
    assert!(refused.contains("refused:awaiting-human"), "{refused}");
    assert!(refused.contains("Build go"), "{refused}");
    assert!(!refused.contains("Publish the release"), "{refused}");
    assert_eq!(fixture.read(&fixture.session_file()), before);
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(fixture.checkpoint()["stage"]["current"], "build");
    let waiting = fixture.ok(&["next", "--json"]);
    let action: &Value = &waiting["next_valid_action"]["action"];
    assert!(
        action
            .as_str()
            .unwrap()
            .starts_with("Waiting for a human decision: Build go."),
        "{action}"
    );
}

/// REQ-records-008: no source file, skill, template or document gives a person as the
/// default human. A configured identity names a SpecSync actor; anywhere else an actor,
/// reviewer or identity value is a placeholder, a synthetic value or an agent account.
#[test]
fn no_source_names_a_person_as_the_default_human() {
    fn files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if path.is_dir() {
                files(&path, out);
            } else if path.extension().is_some_and(|extension| {
                extension == "rs" || extension == "md" || extension == "toml"
            }) {
                out.push(path);
            }
        }
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut sources = vec![root.join("README.md")];
    for dir in ["src", "skills", "docs"] {
        files(&root.join(dir), &mut sources);
    }
    let allowed = |value: &str| {
        let value = value.trim_matches(|c| c == '"' || c == '\'' || c == '`' || c == '\\');
        value.is_empty()
            || value.starts_with('<')
            || value.starts_with('{')
            || value.starts_with('$')
            || value.starts_with("SYNTHETIC")
            || value.starts_with("SIMULATED")
            || value.starts_with('…')
            || value.starts_with("...")
            || ["corvid-agent", "corvid", "human.specsync_actor"].contains(&value)
            || value.ends_with("-t382")
    };
    let mut problems = vec![];
    for path in sources {
        let text = std::fs::read_to_string(&path).unwrap();
        for (number, line) in text.lines().enumerate() {
            for key in [
                "specsync_actor = ",
                "--actor ",
                "--reviewer ",
                "--decided-by ",
            ] {
                for (at, _) in line.match_indices(key) {
                    let value: String = line[at + key.len()..]
                        .chars()
                        .take_while(|c| !c.is_whitespace() && *c != ',' && *c != ')')
                        .collect();
                    if !allowed(&value) {
                        problems.push(format!(
                            "{}:{}: {key}{value}",
                            path.strip_prefix(root).unwrap().display(),
                            number + 1
                        ));
                    }
                }
            }
        }
    }
    assert!(
        problems.is_empty(),
        "a person or account named as an identity: {problems:#?}"
    );
}

/// The checkpoint's next action.
#[cfg(unix)]
fn next_action(fixture: &Fixture) -> String {
    fixture.checkpoint()["next_action"]
        .as_str()
        .unwrap_or_default()
        .to_string()
}

/// The next action a close leaves (fresh-eyes finding 12, t382's rule): the gate, the
/// decision, said once (F12a), and the step that decision allows, never `chough next`
/// (F3): `closed` is a gate that drives nothing and `no-native` a close with
/// --no-native.
fn closed_next(gate: &str, decision: &str) -> String {
    let (said, step) = match decision {
        "approved" => (" (approved)", "run `chough stage advance` to move on".to_string()),
        "closed" => ("", "run `chough stage advance` to move on".into()),
        "review pass" => (
            " (review pass)",
            format!("run `chough change ship {ID}` to ship"),
        ),
        "review block" => (
            " (review block)",
            "run `chough stage back --to build --reason \"<why>\"` to address the review".into(),
        ),
        "no-native" => (
            "",
            "record the next action the decision calls for with `chough checkpoint --next-action \"<action>\"`".into(),
        ),
        other => panic!("no next action for {other}"),
    };
    format!("Gate \"{gate}\" closed{said}; {step}.")
}

/// A project at plan, as the fresh-eyes tester was when the scope gate opened.
#[cfg(unix)]
fn at_plan() -> Fixture {
    let fixture = project();
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&[
        "stage",
        "skip",
        "--json",
        "--reason",
        "SYNTHETIC: no visuals",
    ]);
    assert_eq!(fixture.checkpoint()["stage"]["current"], "plan");
    fixture
}

/// Fresh-eyes finding 12 (t38j, 2026-11-12): a next action set while the scope gate was
/// open ("Present the scope to tester...") went stale once the human approved, and build
/// still showed it. Closing the last open gate holding the current stage, with no
/// --next-action in the same call, replaces it, naming the gate and the decision.
#[cfg(unix)]
#[test]
fn approving_the_scope_gate_replaces_the_next_action_it_left_stale() {
    let fixture = at_plan();
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        SCOPE,
        "--holds",
        "plan",
        "--next-action",
        "Present the scope to SYNTHETIC-human and wait for the decision.",
    ]);
    fixture.ok(&args(&relay(SCOPE, "SYNTHETIC-human")));
    assert_eq!(next_action(&fixture), closed_next(SCOPE, "approved"));
    // The Timeline keeps the decision itself.
    assert!(decisions(&fixture).pop().unwrap().ends_with(&format!(
        "; ran specsync change approve {ID} as SYNTHETIC-actor"
    )));
}

/// F1 of the fourth fresh-eyes round (REQ-cli-021): a gate close's next action names the
/// move its decision allows; once that move is made it is replaced with
/// `Continue <stage>` for the stage reached, so `chough next` in build never says to run
/// `chough stage advance` with nothing built. The same at intent, then design.
#[cfg(unix)]
#[test]
fn a_stage_move_replaces_the_next_action_a_gate_close_left() {
    let fixture = at_plan();
    fixture.no_spec_approval(ID);
    fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    fixture.ok(&args(&relay(SCOPE, "SYNTHETIC-human")));
    assert_eq!(next_action(&fixture), closed_next(SCOPE, "approved"));
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(fixture.checkpoint()["stage"]["current"], "build");
    assert_eq!(next_action(&fixture), "Continue build");
    let action = fixture.ok(&["next", "--json"])["next_valid_action"]["action"]
        .as_str()
        .unwrap()
        .to_string();
    assert!(
        !action.contains("chough stage advance")
            && !action.contains("Continue build: Continue build"),
        "next in build: {action}"
    );

    // At intent: a gate that drives nothing, then the move to design.
    let fixture = project();
    fixture.ok(&["checkpoint", "--awaiting-human", "Approve the intent"]);
    fixture.ok(&args(&relay("Approve the intent", "SYNTHETIC-human")));
    assert_eq!(
        next_action(&fixture),
        closed_next("Approve the intent", "closed")
    );
    fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(next_action(&fixture), "Continue design");
    // A next action the agent wrote is the agent's, and a move leaves it.
    fixture.ok(&[
        "checkpoint",
        "--next-action",
        "SYNTHETIC: sketch the screen",
    ]);
    fixture.ok(&[
        "stage",
        "skip",
        "--json",
        "--reason",
        "SYNTHETIC: no visuals",
    ]);
    assert_eq!(next_action(&fixture), "SYNTHETIC: sketch the screen");
}

/// F1 of the fourth round, continued (REQ-cli-021): a batched advance makes several moves
/// on one read, and each carries the replacement on, so the next action names the stage
/// reached, never a stage it passed through.
#[cfg(unix)]
#[test]
fn a_batched_advance_leaves_continue_for_the_stage_it_reaches() {
    let fixture = at_plan();
    fixture.no_spec_approval(ID);
    fixture.ok(&["checkpoint", "--awaiting-human", SCOPE, "--holds", "plan"]);
    fixture.ok(&args(&relay(SCOPE, "SYNTHETIC-human")));
    fixture.ok(&["stage", "advance", "--to", "test", "--json"]);
    assert_eq!(fixture.checkpoint()["stage"]["current"], "test");
    assert_eq!(next_action(&fixture), "Continue test");
}

/// The rest of t382's rule: a review gate names its verdict, a gate that drives no native
/// step or is closed with --no-native says "closed"; a next action given in the same call
/// is kept; and a close that leaves another gate holding the stage, or of a gate that
/// holds another stage, leaves the next action as it is.
#[cfg(unix)]
#[test]
fn a_gate_close_replaces_the_next_action_only_when_no_gate_holds_the_stage() {
    let fixture = at_plan();
    let stale = "SYNTHETIC: wait for the human.";
    let close = |gate: &str, extra: &[&str]| {
        let mut list = relay(gate, "SYNTHETIC-human");
        list.extend(extra.iter().map(|value| value.to_string()));
        fixture.ok(&args(&list));
    };
    let open = |gate: &str, holds: &str, drives: &str| {
        fixture.ok(&[
            "checkpoint",
            "--awaiting-human",
            gate,
            "--holds",
            holds,
            "--drives",
            drives,
            "--next-action",
            stale,
        ]);
    };
    for (gate, drives, extra, decision) in [
        (
            "SYNTHETIC review to pass",
            "review",
            &["--verdict", "pass"][..],
            "review pass",
        ),
        (
            "SYNTHETIC review to block",
            "review",
            &["--verdict", "block"][..],
            "review block",
        ),
        ("SYNTHETIC go-ahead", "none", &[][..], "closed"),
        (
            "SYNTHETIC changes requested",
            "approve",
            &["--no-native"][..],
            "no-native",
        ),
    ] {
        open(gate, "plan", drives);
        close(gate, extra);
        assert_eq!(next_action(&fixture), closed_next(gate, decision), "{gate}");
    }

    // A next action given with the close is the one kept.
    open("SYNTHETIC own next", "plan", "none");
    close(
        "SYNTHETIC own next",
        &["--next-action", "SYNTHETIC: the agent's own next"],
    );
    assert_eq!(next_action(&fixture), "SYNTHETIC: the agent's own next");

    // Another gate still holds plan: nothing changes until the last one closes.
    open("SYNTHETIC first", "plan", "none");
    open("SYNTHETIC second", "plan", "none");
    close("SYNTHETIC first", &[]);
    assert_eq!(next_action(&fixture), stale);
    close("SYNTHETIC second", &[]);
    assert_eq!(
        next_action(&fixture),
        closed_next("SYNTHETIC second", "closed")
    );

    // A gate that holds another stage does not describe the current one.
    open("SYNTHETIC later", "accept", "none");
    close("SYNTHETIC later", &[]);
    assert_eq!(next_action(&fixture), stale);
}

/// Fresh-eyes F12a (second round, 2026-11-12): a gate that drives no native step, or a
/// close with --no-native, decides nothing beyond the close, and its next action read
/// "Gate ... closed (closed)". It says "closed" once.
#[cfg(unix)]
#[test]
fn a_close_that_runs_no_native_step_says_closed_once() {
    let fixture = at_plan();
    for (gate, drives, extra) in [
        ("SYNTHETIC go-ahead", "none", &[][..]),
        (
            "SYNTHETIC changes requested",
            "approve",
            &["--no-native"][..],
        ),
    ] {
        fixture.ok(&[
            "checkpoint",
            "--awaiting-human",
            gate,
            "--holds",
            "plan",
            "--drives",
            drives,
            "--next-action",
            "SYNTHETIC: wait for the human.",
        ]);
        let mut list = relay(gate, "SYNTHETIC-human");
        list.extend(extra.iter().map(|value| value.to_string()));
        fixture.ok(&args(&list));
        let next = next_action(&fixture);
        assert!(
            next.starts_with(&format!("Gate \"{gate}\" closed")),
            "{next}"
        );
        assert_eq!(next.matches("closed").count(), 1, "{next}");
    }
}

/// Fresh-eyes F3 (second round, 2026-11-12; REQ-cli-021): after a gate closed, the next
/// action said to run `chough next`, which showed that same next action again. A close
/// that leaves no gate holding the stage names the step its decision allows: `chough
/// stage advance` after an approval or a gate that drives nothing, `chough change ship
/// <id>` after a review pass, `chough stage back --to build` after a review block, and
/// `chough checkpoint --next-action` after a --no-native close; next never names itself.
#[cfg(unix)]
#[test]
fn a_gate_close_names_the_step_its_decision_allows_never_chough_next() {
    let fixture = at_plan();
    for (gate, drives, extra, step) in [
        (SCOPE, "approve", &[][..], "`chough stage advance`"),
        (
            "SYNTHETIC go-ahead",
            "none",
            &[][..],
            "`chough stage advance`",
        ),
        (
            "SYNTHETIC review to pass",
            "review",
            &["--verdict", "pass"][..],
            "`chough change ship human-change`",
        ),
        (
            "SYNTHETIC review to block",
            "review",
            &["--verdict", "block"][..],
            "`chough stage back --to build",
        ),
        (
            "SYNTHETIC changes requested",
            "approve",
            &["--no-native"][..],
            "`chough checkpoint --next-action",
        ),
    ] {
        fixture.ok(&[
            "checkpoint",
            "--awaiting-human",
            gate,
            "--holds",
            "plan",
            "--drives",
            drives,
            "--next-action",
            "SYNTHETIC: wait for the human.",
        ]);
        let mut list = relay(gate, "SYNTHETIC-human");
        list.extend(extra.iter().map(|value| value.to_string()));
        fixture.ok(&args(&list));
        let next = next_action(&fixture);
        assert!(next.contains(step), "{gate}: {next}");
        let shown = fixture.ok(&["next", "--json"])["next_valid_action"].clone();
        for text in [next.as_str(), shown["action"].as_str().unwrap()] {
            assert!(!text.contains("`chough next`"), "{gate}: {text}");
        }
    }
}
