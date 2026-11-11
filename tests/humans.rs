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
            "change approve {ID} --actor SYNTHETIC-actor --note Approved by SYNTHETIC Person at the Chough gate \"{SCOPE}\": \"yes, go\" (SYNTHETIC chat, 2026-11-11). Relayed and recorded by SYNTHETIC-agent at "
        )),
        "{calls:?}"
    );
    assert!(
        calls[0].ends_with("; a relayed claim, not authenticated."),
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
    assert_eq!(
        calls(&fixture),
        [format!(
            "change review {ID} --reviewer SYNTHETIC-actor --verdict block"
        )]
    );
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with("as SYNTHETIC-actor with verdict block"),
        "{decision}"
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
    let mut changes = relay(REVIEW, "someone not configured");
    changes.push("--no-native".into());
    fixture.ok(&args(&changes));
    assert!(calls(&fixture).is_empty());
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with("; no native review ran (--no-native)"),
        "{decision}"
    );
    // A gate that drives nothing closes with no native call and records no human.
    fixture.ok(&["checkpoint", "--awaiting-human", "Name the release"]);
    fixture.ok(&args(&relay("Name the release", "someone not configured")));
    assert!(calls(&fixture).is_empty());
    let decision = decisions(&fixture).pop().unwrap();
    assert!(
        decision.ends_with("a relayed claim, not authenticated"),
        "{decision}"
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
        calls(&fixture)[0].contains("--actor user-actor "),
        "{:?}",
        calls(&fixture)
    );
    fixture.write(".chough/local/identity.toml", HUMANS);
    let _ = std::fs::remove_file(fixture.root().join(".stub-calls"));
    fixture.ok(&args(&relay("Second scope", "SYNTHETIC-human")));
    assert!(
        calls(&fixture)[0].contains("--actor SYNTHETIC-actor "),
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
    assert!(calls[0].starts_with(&format!("change approve {ID} --actor SYNTHETIC-actor")));
    assert_eq!(
        fixture.checkpoint()["awaiting_human"],
        serde_json::json!(["Second scope"])
    );
}

/// #237: a gate that holds a later stage does not stop moves out of earlier ones; it
/// holds the stage it names.
#[test]
fn a_gate_holds_only_the_stage_it_names() {
    let fixture = Fixture::new();
    fixture.init();
    while fixture.checkpoint()["stage"]["current"] != "build" {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&[
        "checkpoint",
        "--awaiting-human",
        "Publish the release",
        "--holds",
        "release",
    ]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
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
    fixture.ok(&["stage", "reset", "--to", "plan", "--reason", "Rework"]);
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
    fixture.ok(&["stage", "advance"]);
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
