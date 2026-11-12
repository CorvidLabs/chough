//! REQ-records-011 in the 0.5.x layout, red on 0.5.0: the move into done regenerated
//! the whole knowledge library, churning tracked files. The 0.6.0 test of the same name
//! in tests/journal.rs replaces it; this branch only records the failing run on 0.5.0.

mod common;

use common::Fixture;
use std::collections::BTreeMap;
use std::path::Path;

/// Every file under `relative`, with its bytes.
fn tree(root: &Path, relative: &str) -> BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, relative: &str, out: &mut BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else {
                out.insert(name, std::fs::read(entry.path()).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, relative, &mut out);
    out
}

/// Around the move into done, nothing under `knowledge/` changes but one new journal
/// post, `knowledge/index.md` and `knowledge/journal/feed.xml`; a lesson and a brief a
/// person wrote stay byte-identical. Every problem is collected.
#[test]
fn done_changes_no_knowledge_file_but_its_own_post_and_the_feed() {
    let fixture = Fixture::new();
    // A vault with archived native changes and a module lesson, as a real one has.
    fixture.write(
        "specs/cli/context.md",
        "# Context\n\nIntro.\n\n## Lessons\n\n- 2026-01-01: a lesson.\n",
    );
    for (index, id) in ["a1", "a2", "a3"].iter().enumerate() {
        let dir = format!(".specsync/archive/changes/2026-0{}-01-{id}", index + 1);
        fixture.write(
            &format!("{dir}/finalization.json"),
            &format!(
                "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {}, \"implementation_commit\": \"{}\"}}\n",
                (index + 1) * 100,
                "1".repeat(40)
            ),
        );
        fixture.write(
            &format!("{dir}/verification.json"),
            "{\"commit\": \"2222222222222222222222222222222222222222\", \"passed\": true, \"requirement_ids\": [\"REQ-cli-001\"], \"acceptance_manifest\": {\"schema_version\": 1, \"entries\": []}}\n",
        );
        fixture.write(
            &format!("{dir}/change.md"),
            &format!(
                "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- {id} works\n",
                "3".repeat(40)
            ),
        );
        fixture.write(
            &format!("{dir}/lesson-bundle.md"),
            &format!("# Lesson bundle: {id}\n"),
        );
    }
    fixture.ok(&["init", "--objective", "SYNTHETIC churn", "--module", "cli"]);
    // Six finished sessions before this one, as a project that has worked a while has.
    fixture.ok(&["session", "adopt", "first"]);
    for index in 1..=6 {
        let key = format!("earlier-{index}");
        fixture.ok(&[
            "session",
            "new",
            &key,
            "--objective",
            &format!("SYNTHETIC earlier {index}"),
        ]);
        fixture.ok(&[
            "session",
            "close",
            &key,
            "--reason",
            "SYNTHETIC earlier work done",
        ]);
    }
    fixture.ok(&[
        "session",
        "new",
        "current",
        "--objective",
        "SYNTHETIC current work",
    ]);
    fixture.write(
        "knowledge/lessons/2026-01-03-a-lesson.md",
        "# A lesson\n\nA person wrote this.\n",
    );
    fixture.write("knowledge/briefs/brief.md", "A brief kept as it is.\n");
    for _ in 0..5 {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&["library", "index"]);
    fixture.commit("the vault, at accept");
    let before = tree(fixture.root(), "knowledge");
    fixture.ok(&["stage", "advance"]);
    let after = tree(fixture.root(), "knowledge");
    let created: Vec<&String> = after
        .keys()
        .filter(|key| !before.contains_key(*key))
        .collect();
    let removed: Vec<&String> = before
        .keys()
        .filter(|key| !after.contains_key(*key))
        .collect();
    let changed: Vec<&String> = before
        .keys()
        .filter(|key| after.get(*key).is_some_and(|bytes| *bytes != before[*key]))
        .collect();
    let mut problems = vec![];
    let posts: Vec<&&String> = created
        .iter()
        .filter(|path| path.starts_with("knowledge/journal/") && path.ends_with(".md"))
        .collect();
    if posts.len() != 1 {
        problems.push(format!(
            "the move into done created {} journal posts, not one",
            posts.len()
        ));
    }
    let others: Vec<&&String> = created
        .iter()
        .filter(|path| !(path.starts_with("knowledge/journal/") && path.ends_with(".md")))
        .collect();
    if !others.is_empty() {
        problems.push(format!(
            "the move into done created {} other knowledge files: {others:?}",
            others.len()
        ));
    }
    let churned: Vec<&&String> = changed
        .iter()
        .filter(|path| {
            path.as_str() != "knowledge/index.md" && path.as_str() != "knowledge/journal/feed.xml"
        })
        .collect();
    if !churned.is_empty() {
        problems.push(format!(
            "the move into done changed {} other tracked knowledge files: {churned:?}",
            churned.len()
        ));
    }
    if !removed.is_empty() {
        problems.push(format!(
            "the move into done removed {} knowledge files: {removed:?}",
            removed.len()
        ));
    }
    assert!(problems.is_empty(), "{problems:#?}");
}
