//! `chough migrate` (REQ-records-009): a preview writes nothing; an apply needs the
//! previewed digest, no open native change, the human's consent and every file it
//! removes committed, then writes the 0.6.0 tree, a receipt, and removes the 0.5.x files.
//!
//! These tests carry sessions and their history. Evidence, bugs and knowledge join as
//! their mappers land (plan step 10), on the same release fixtures.

mod common;

use common::Fixture;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

/// A project holding `tag`'s 0.5.x sessions and history, committed.
fn sessions_project(tag: &str) -> Fixture {
    fn copy(from: &Path, to: &Path) {
        std::fs::create_dir_all(to).unwrap();
        for entry in std::fs::read_dir(from).unwrap().flatten() {
            let target = to.join(entry.file_name());
            if entry.path().is_dir() {
                copy(&entry.path(), &target);
            } else {
                std::fs::copy(entry.path(), target).unwrap();
            }
        }
    }
    let fixture = Fixture::new();
    let source = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/releases")
        .join(tag);
    copy(&source, fixture.root());
    // Evidence, bugs and their artifacts wait for their mappers.
    for folder in [".chough/evidence", ".chough/bugs", ".chough/artifacts"] {
        let _ = std::fs::remove_dir_all(fixture.root().join(folder));
    }
    fixture.commit("a 0.5.x project");
    fixture
}

/// Every file of the project except Git's, by path, with its bytes.
fn files(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, relative: &str, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = if relative.is_empty() {
                entry.file_name().to_string_lossy().to_string()
            } else {
                format!("{relative}/{}", entry.file_name().to_string_lossy())
            };
            if name == ".git" {
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
    walk(root, "", &mut out);
    out
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

#[test]
fn migrate_previews_applies_and_keeps_every_byte_in_git() {
    let fixture = sessions_project("v0.5.0");
    let before = files(fixture.root());
    let plan = fixture.ok(&["migrate"]);
    assert_eq!(files(fixture.root()), before, "the preview wrote something");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    assert_eq!(plan["unaccounted"], serde_json::json!([]), "{plan:#}");
    let writes: Vec<String> = plan["writes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|write| write["path"].as_str().unwrap().to_string())
        .collect();
    assert!(
        writes
            .iter()
            .any(|path| path.ends_with("-fixture-first/session.md")),
        "{writes:?}"
    );
    assert!(
        writes
            .iter()
            .any(|path| path.starts_with(".chough/archive/sessions/")),
        "{writes:?}"
    );

    // Every refusal writes nothing.
    let consent = "--consent";
    let words = "SYNTHETIC-human said migrate it (fixture)";
    let other = "0".repeat(64);
    for (args, why) in [
        (
            vec!["migrate", "--apply", other.as_str(), consent, words],
            "refused:plan-digest",
        ),
        (
            vec!["migrate", "--apply", digest.as_str()],
            "refused:no-consent",
        ),
    ] {
        let refused = fixture.fails(&args);
        assert!(refused.contains(why), "{args:?}: {refused}");
        assert_eq!(files(fixture.root()), before, "{args:?} wrote something");
    }
    fixture.write(
        ".specsync/changes/open-one/state.json",
        "{\"id\": \"open-one\"}\n",
    );
    let refused = fixture.fails(&["migrate", "--apply", &digest, consent, words]);
    assert!(
        refused.contains("refused:change-open") && refused.contains("open-one"),
        "{refused}"
    );
    std::fs::remove_dir_all(fixture.root().join(".specsync/changes")).unwrap();
    let session = ".chough/sessions/fixture-first.json";
    let committed = fixture.read(session);
    fixture.write(session, &committed.replace("\"plan\"", "\"plan\" "));
    let replanned = fixture.ok(&["migrate"])["plan_digest"]
        .as_str()
        .unwrap()
        .to_string();
    let refused = fixture.fails(&["migrate", "--apply", &replanned, consent, words]);
    assert!(
        refused.contains("refused:uncommitted") && refused.contains(session),
        "{refused}"
    );
    fixture.write(session, &committed);
    assert_eq!(files(fixture.root()), before);

    let replanned = fixture.ok(&["migrate"]);
    for key in plan.as_object().unwrap().keys() {
        assert_eq!(
            replanned[key], plan[key],
            "the plan's {key} changed after the refusals"
        );
    }
    // The apply.
    let applied = fixture.ok(&["migrate", "--apply", &digest, consent, words]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    for path in &writes {
        assert!(fixture.exists(path), "{path} was not written");
    }
    for path in plan["sources"].as_object().unwrap().keys() {
        assert!(!fixture.exists(path), "{path} was not removed");
    }
    assert!(
        fixture.exists(".chough/cases/fixture.cases.toml"),
        "a case definition was removed"
    );
    let receipt = applied["receipt"].as_str().unwrap();
    let text = fixture.read(receipt);
    assert!(text.contains(&format!("plan_digest: {digest}")), "{text}");
    assert!(text.contains("does not rewrite it"), "{text}");
    assert!(text.contains(words), "{text}");
    // The receipt's before digests are the files at the earlier commit.
    for (path, sha) in plan["sources"].as_object().unwrap() {
        assert_eq!(&sha256(&before[path]), sha.as_str().unwrap(), "{path}");
        assert!(
            text.contains(&format!("`{path}` {}", sha.as_str().unwrap())),
            "{path}"
        );
    }
    // The migrated records are readable, the old keys resolve as aliases.
    let list = fixture.ok(&["session", "list"]);
    let ids: Vec<&str> = list["sessions"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|session| session["id"].as_str())
        .collect();
    assert!(
        ids.iter().any(|id| id.ends_with("-fixture-first")),
        "{list:#}"
    );
    fixture.ok(&["session", "use", "fixture-first"]);
    let shown = fixture.ok(&["history", "show"]);
    assert!(
        !shown["entries"].as_array().unwrap().is_empty(),
        "{shown:#}"
    );
    fixture.ok(&["next", "--json"]);

    // A second apply has nothing to do.
    let again = fixture.ok(&["migrate", "--apply", &digest, consent, words]);
    assert_eq!(again["applied"], false, "{again:#}");
}

#[test]
fn every_release_fixture_previews_with_nothing_unaccounted_for_sessions() {
    for tag in ["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"] {
        let fixture = sessions_project(tag);
        let before = files(fixture.root());
        let plan = fixture.ok(&["migrate"]);
        assert_eq!(
            plan["unaccounted"],
            serde_json::json!([]),
            "{tag}: {plan:#}"
        );
        assert!(!plan["writes"].as_array().unwrap().is_empty(), "{tag}");
        assert_eq!(
            files(fixture.root()),
            before,
            "{tag}: the preview wrote something"
        );
        let again = fixture.ok(&["migrate"]);
        assert_eq!(
            again["plan_digest"], plan["plan_digest"],
            "{tag}: the plan is not stable"
        );
        let _: &Value = &plan;
    }
}
