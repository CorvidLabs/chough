//! Record files and the records tree (REQ-records-001, REQ-records-002). Evidence
//! entries join these tests when plan step 4 lands; until then `evidence add` still
//! writes 0.5.x JSON.

mod common;

use common::Fixture;
use std::collections::BTreeSet;
use std::path::Path;

/// Every file under `.chough/` except local state, by project-relative path.
fn chough_files(root: &Path) -> BTreeSet<String> {
    fn walk(root: &Path, relative: &str, out: &mut BTreeSet<String>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if name == ".chough/local" {
                continue;
            }
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else {
                out.insert(name);
            }
        }
    }
    let mut out = BTreeSet::new();
    walk(root, ".chough", &mut out);
    out
}

/// Whether `path` is one REQ-records-002 lists for Chough's committed records.
fn allowed(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    match parts.as_slice() {
        [".chough", "README.md"] | [".chough", "preferences.json"] => true,
        [".chough", "cases", name] => name.ends_with(".cases.toml"),
        [".chough", "sessions", _, "session.md"] => true,
        [".chough", "sessions", _, "evidence" | "agent", ..] => true,
        [".chough", "archive", "sessions", name] => name.ends_with(".md"),
        [".chough", "archive", "sessions", _, _] => true,
        [".chough", "archive", "shelved", ..] => true,
        [".chough", "archive", "migrations", name] => name.ends_with(".md"),
        [".chough", "bugs", name] => name.ends_with(".md"),
        _ => false,
    }
}

/// REQ-records-002: init writes the reading guide create-only, and a full cycle leaves
/// committed files under `.chough/` only at the listed paths.
#[test]
fn the_tree_holds_only_the_listed_paths() {
    let fixture = Fixture::new();
    fixture.ok(&["init", "--intent-scope", "LIFE", "--module", "cli"]);
    let mut top: Vec<String> = std::fs::read_dir(fixture.root().join(".chough"))
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    top.sort();
    assert_eq!(top, ["README.md", "cases", "local"]);
    let readme = fixture.read(".chough/README.md");
    for folder in [
        "sessions/",
        "archive/sessions/",
        "archive/shelved/",
        "archive/migrations/",
        "bugs/",
        "cases/",
        "preferences.json",
        "local/",
        "never committed",
        "](../hi/)",
        "](../specs/)",
        "](../.specsync/archive/)",
    ] {
        assert!(
            readme.contains(folder),
            "the guide lacks {folder}:\n{readme}"
        );
    }
    // An existing guide is kept byte for byte.
    fixture.write(".chough/README.md", "A person's own guide.\n");
    fixture.ok(&["init", "--intent-scope", "LIFE", "--module", "cli"]);
    assert_eq!(fixture.read(".chough/README.md"), "A person's own guide.\n");

    // A full cycle: a session to done with a bug, a correction and an inbox entry.
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Alpha breaks",
        "--symptom",
        "It breaks",
        "--trigger",
        "Run it",
        "--expected",
        "It works",
        "--actual",
        "It broke",
        "--environment",
        "local",
        "--impact",
        "Low",
        "--scope",
        "in-scope",
        "--classification",
        "product-defect",
        "--source",
        "a test",
    ]);
    let id = fixture.id_of("alpha");
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &id,
        "--from",
        "helper",
        "--kind",
        "decision",
        "--text",
        "Keep it small",
    ]);
    fixture.ok(&["inbox", "fold"]);
    while !fixture.is_archived("alpha") {
        fixture.ok(&["stage", "advance"]);
    }
    let stray: Vec<String> = chough_files(fixture.root())
        .into_iter()
        .filter(|path| !allowed(path))
        .collect();
    assert!(
        stray.is_empty(),
        "files outside the records tree: {stray:?}"
    );
}

/// REQ-records-001: records are Markdown with front matter, a person's edits survive
/// Chough's writes, front matter outside the subset is refused naming the file, line and
/// rule, and a CRLF copy reads as the LF original.
#[test]
fn every_record_is_markdown_and_keeps_human_edits() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let file = fixture.session_file();
    // A person edits the Goal paragraph, adds a key Chough does not know and a comment.
    let text = fixture.read(&file);
    let edited = text
        .replacen(
            "---\n",
            "---\nowner_note: \"keep me\"\n# a person's comment\n",
            1,
        )
        .replacen(
            "## Goal\n\nShip alpha\n",
            "## Goal\n\nShip alpha, the small version a person described.\n",
            1,
        );
    assert_ne!(edited, text);
    fixture.write(&file, &edited);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: small because time is short",
    ]);
    let after = fixture.read(&file);
    for kept in [
        "owner_note: \"keep me\"\n",
        "# a person's comment\n",
        "Ship alpha, the small version a person described.\n",
    ] {
        assert!(after.contains(kept), "lost {kept:?}:\n{after}");
    }
    assert!(after.starts_with("---\n"), "{after}");

    // A bug's Steps paragraph a person edits survives an update.
    let added = fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Alpha breaks",
        "--symptom",
        "It breaks",
        "--trigger",
        "Run it",
        "--expected",
        "It works",
        "--actual",
        "It broke",
        "--environment",
        "local",
        "--impact",
        "Low",
        "--scope",
        "in-scope",
        "--classification",
        "product-defect",
        "--source",
        "a test",
    ]);
    let bug = added["recorded"]
        .as_str()
        .or_else(|| added["bug"]["path"].as_str())
        .map(String::from)
        .unwrap_or_else(|| {
            let mut found = chough_files(fixture.root())
                .into_iter()
                .filter(|path| path.starts_with(".chough/bugs/"));
            found.next().unwrap()
        });
    let text = fixture.read(&bug);
    let steps = "**Steps:** Run it, as a person wrote it out in full.\n";
    let edited = text.replacen("**Steps:** Run it\n", steps, 1);
    assert_ne!(edited, text, "{text}");
    fixture.write(&bug, &edited);
    let bug_id = edited
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .unwrap()
        .to_string();
    fixture.ok(&["bug", "update", &bug_id, "--note", "Seen again"]);
    assert!(fixture.read(&bug).contains(steps), "{}", fixture.read(&bug));

    // Front matter outside the subset is refused, naming the file, line and rule.
    let good = fixture.read(&file);
    for (bad, rule) in [
        (
            good.replacen("---\n", "---\nnested:\n  inner:\n    deep: x\n", 1),
            "line",
        ),
        (
            good.replacen("---\n", "---\nkind: session\n", 1),
            "duplicate",
        ),
        (good.replacen("---\n", "---\nanchor: &a x\n", 1), "line"),
    ] {
        fixture.write(&file, &bad);
        let refused = fixture.fails(&["checkpoint", "--decision", "never written"]);
        assert!(
            refused.contains(&file) && refused.contains(rule),
            "{rule}: {refused}"
        );
        assert_eq!(fixture.read(&file), bad, "a refused write changed the file");
    }
    fixture.write(&file, &good);

    // A CRLF copy reads the same as the LF original.
    let lf = fixture.ok(&["checkpoint"]);
    fixture.write(&file, &good.replace('\n', "\r\n"));
    assert_eq!(fixture.ok(&["checkpoint"]), lf);
    fixture.write(&file, &good);

    // No JSON twin under .chough/ but the preferences.
    let json: Vec<String> = chough_files(fixture.root())
        .into_iter()
        .filter(|path| path.ends_with(".json") && path != ".chough/preferences.json")
        .collect();
    assert!(json.is_empty(), "{json:?}");
}
