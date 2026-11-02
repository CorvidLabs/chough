//! Real upgrades of the exact workflow guidance shipped by main 825a380.
//! This checks installer safety, not fresh-agent behavior or human authorization.

mod common;

use common::Fixture;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;

type PriorFile = (&'static str, &'static [u8], &'static str, &'static [u8]);

const PRIOR_FILES: [PriorFile; 2] = [
    (
        "SKILL.md",
        include_bytes!("fixtures/workflow-at-825a380-SKILL.md"),
        "6b81882bc2e8305dc1ae0181b1eb52ad9d586c0a9a6990e03e865d0de4149198",
        include_bytes!("../skills/chough-workflow/SKILL.md"),
    ),
    (
        "references/lifecycle.md",
        include_bytes!("fixtures/workflow-at-825a380-lifecycle.md"),
        "14fd2277d5e3f97f24fe6b995b1815eff1cfcd7b49d03669919bdcc7290c2200",
        include_bytes!("../skills/chough-workflow/references/lifecycle.md"),
    ),
];

fn file_state<'a>(report: &'a Value, name: &str) -> &'a Value {
    report["hosts"][0]["files"]
        .as_array()
        .unwrap()
        .iter()
        .find(|file| file["file"] == name)
        .unwrap_or_else(|| panic!("{name} missing from {report}"))
}

#[test]
fn workflow_guidance_from_main_825a380_updates_and_preserves_edits() {
    // Git may convert text fixtures on checkout. Reconstruct the exact Git blob's
    // LF bytes before checking its independently recorded identity.
    let previous_files: Vec<_> = PRIOR_FILES
        .iter()
        .map(|(name, previous, digest, current)| {
            let previous = String::from_utf8(previous.to_vec())
                .unwrap()
                .replace("\r\n", "\n")
                .into_bytes();
            assert_eq!(format!("{:x}", Sha256::digest(&previous)), *digest);
            assert_ne!(previous, *current, "{name} must exercise an older bundle");
            (*name, previous, *current)
        })
        .collect();

    for edited in [None, Some("SKILL.md"), Some("references/lifecycle.md")] {
        let fixture = Fixture::new();
        fixture.init();
        let installed = fixture.root().join(".agents/skills/chough-workflow");
        let checkpoint = fixture.read(".chough/session.json");
        let mut edited_bytes = None;
        for (name, previous, _) in &previous_files {
            let name = *name;
            let mut bytes = previous.to_vec();
            if edited == Some(name) {
                bytes.extend_from_slice(b"\nHuman project guidance must stay.\n");
                edited_bytes = Some(bytes.clone());
            }
            fs::write(installed.join(name), bytes).unwrap();
        }

        let status = fixture.ok(&["skill", "status", "--host", "openai"]);
        for (name, _, _) in &previous_files {
            let name = *name;
            assert_eq!(
                file_state(&status, name)["state"],
                if edited == Some(name) {
                    "edited"
                } else {
                    "known-outdated"
                },
                "{name}: {status}"
            );
        }

        let update = fixture.ok(&["skill", "update", "--host", "openai"]);
        let after = fixture.ok(&["skill", "status", "--host", "openai"]);
        for (name, _, current) in &previous_files {
            let name = *name;
            let bytes = fs::read(installed.join(name)).unwrap();
            if edited == Some(name) {
                assert_eq!(bytes, *edited_bytes.as_ref().unwrap(), "{name}");
                assert_eq!(file_state(&update, name)["action"], "preserved-edit");
                assert_eq!(file_state(&after, name)["state"], "edited");
            } else {
                assert_eq!(bytes, *current, "{name}");
                assert_eq!(file_state(&update, name)["action"], "updated");
                assert_eq!(file_state(&after, name)["state"], "current");
            }
        }
        assert_eq!(fixture.read(".chough/session.json"), checkpoint);
    }
}
