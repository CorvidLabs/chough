//! Several Chough runs at once in one project (#386): each succeeds, or refuses only as a
//! run on its own would, and no output holds `File exists` or internal assertion text.

mod common;

use common::Fixture;
use serde_json::Value;
use std::sync::Barrier;

const SKILL: &str = ".agents/skills/chough-workflow";

/// Run `runs` copies of `args` released together, returning each exit code and its
/// standard output and error.
fn at_once(fixture: &Fixture, args: &[&str], runs: usize) -> Vec<(Option<i32>, String)> {
    let barrier = Barrier::new(runs);
    std::thread::scope(|scope| {
        let handles: Vec<_> = (0..runs)
            .map(|_| {
                scope.spawn(|| {
                    barrier.wait();
                    let output = fixture.run(args);
                    (
                        output.status.code(),
                        format!(
                            "{}{}",
                            String::from_utf8_lossy(&output.stdout),
                            String::from_utf8_lossy(&output.stderr)
                        ),
                    )
                })
            })
            .collect();
        handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect()
    })
}

fn race_text(said: &str) -> bool {
    said.contains("File exists")
        || said.contains("os error 17")
        || said.contains("Condition failed")
}

/// REQ-cli-016 (#386), TC-cli-parallel-skill-updates-each-succeed: eight `chough skill
/// update` runs at once over an install missing bundle files, repeated, each exit 0, no
/// output holds `File exists` or `Condition failed`, and every file is current afterwards.
#[test]
fn parallel_skill_updates_each_succeed() {
    for round in 0..8 {
        let fixture = Fixture::new();
        fixture.init();
        fixture.commit("fixture");
        for file in ["SKILL.md", "references/lifecycle.md", "references/build.md"] {
            std::fs::remove_file(fixture.root().join(SKILL).join(file)).unwrap();
        }
        let results = at_once(&fixture, &["skill", "update"], 8);
        for (code, said) in &results {
            assert_eq!(*code, Some(0), "round {round}: {said}");
            assert!(!race_text(said), "round {round}: {said}");
        }
        let status = fixture.ok(&["skill", "status"]);
        let files = status["hosts"]
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|host| host["files"].as_array().cloned().unwrap_or_default())
            .collect::<Vec<Value>>();
        assert!(!files.is_empty(), "{status:#}");
        assert!(
            files.iter().all(|file| file["state"] == "current"),
            "round {round}: {status:#}"
        );
    }
}

/// REQ-cli-001 (#386), TC-cli-parallel-inits-each-succeed-or-refuse-plainly: six `chough
/// init --objective` runs at once in a fresh repository, repeated, each exit 0 or refuse
/// only because a session is already open, naming `chough session new`; none fails on a
/// file another run wrote, and no output holds `File exists`.
#[test]
fn parallel_inits_each_succeed_or_refuse_plainly() {
    // The race is narrow (2 of 24 runs on Linux in a debug build), so many rounds.
    for round in 0..30 {
        let fixture = Fixture::new();
        fixture.commit("fresh repository");
        let results = at_once(
            &fixture,
            &["init", "--objective", "SYNTHETIC parallel objective"],
            6,
        );
        for (code, said) in &results {
            assert!(!race_text(said), "round {round}: {said}");
            assert!(
                *code == Some(0) || said.contains("chough session new"),
                "round {round}: exit {code:?}: {said}"
            );
        }
        assert!(
            results.iter().any(|(code, _)| *code == Some(0)),
            "round {round}: no init succeeded"
        );
    }
}
