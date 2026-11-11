//! `chough migrate`, the knowledge part (REQ-records-009, REQ-cli-044): a 0.5.x vault's
//! generated and copied content is planned for removal with why, what a person made is
//! kept byte for byte with its leaks listed, and the planned `index.md` and feed are the
//! ones `chough library index` builds once the plan applies.

mod common;

use common::Fixture;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

/// Every file under `root`, outside `.git`, with its bytes.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            if entry.file_name() == ".git" {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                let name = path
                    .strip_prefix(base)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                out.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

/// Copy `from` into `to`, recursively.
fn copy(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap().flatten() {
        let target = to.join(entry.file_name());
        if entry.path().is_dir() {
            copy(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

/// Run `chough migrate` (the preview) in `root`, with no actor or user identity from
/// this machine, and return the plan.
fn preview(root: &Path) -> Value {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_chough"));
    command
        .arg("--root")
        .arg(root)
        .arg("migrate")
        .env("XDG_CONFIG_HOME", root.join(common::USER_CONFIG));
    for name in common::ACTOR_VARIABLES {
        command.env_remove(name);
    }
    let output = command.output().unwrap();
    assert!(
        output.status.success(),
        "migrate: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

/// The sha256 of each planned write, by path.
fn writes(plan: &Value) -> BTreeMap<String, String> {
    plan["writes"]
        .as_array()
        .unwrap_or_else(|| panic!("{plan:#}"))
        .iter()
        .map(|write| {
            (
                write["path"].as_str().unwrap_or_default().to_string(),
                write["sha256"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect()
}

/// REQ-records-009: the release fixtures with a vault-era session layout (v0.4.0 and
/// v0.5.0) and the earlier ones plan their knowledge with `chough migrate` and nothing is
/// written: no 0.5.x vault exists in them, so nothing under `knowledge/` is removed, and
/// with the librarian on by default `index.md` and the feed are planned.
#[test]
fn release_fixtures_plan_their_knowledge_and_write_nothing() {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases");
    for tag in ["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"] {
        let copy_of = tempfile::tempdir().unwrap();
        copy(&fixtures.join(tag), copy_of.path());
        let before = tree(copy_of.path());
        let plan = preview(copy_of.path());
        assert_eq!(tree(copy_of.path()), before, "{tag}: the preview wrote");
        let planned = writes(&plan);
        for path in ["knowledge/index.md", "knowledge/journal/feed.xml"] {
            assert!(
                planned.contains_key(path),
                "{tag}: {path} not planned: {plan:#}"
            );
        }
        let removed = plan["removed"]
            .as_object()
            .unwrap_or_else(|| panic!("{plan:#}"));
        assert!(
            removed.keys().all(|path| !path.starts_with("knowledge/")),
            "{tag}: {removed:#?}"
        );
    }
}

/// A 0.5.x vault with every generated kind, a person's lesson, brief and note, an earlier
/// snapshot and `library/` folder, a rolled session, and a home path in a kept brief.
fn vault_0_5(fixture: &Fixture) {
    fixture.init();
    fs::remove_dir_all(fixture.root().join("knowledge")).unwrap();
    let generated = [
        "knowledge/INDEX.md",
        "knowledge/rules.md",
        "knowledge/timeline.md",
        "knowledge/timeline/2026-11.md",
        "knowledge/archives/a1.md",
        "knowledge/modules/cli.md",
        "knowledge/intent/knowledge.md",
        "knowledge/bugs/BUG-0001.md",
        "knowledge/sessions/s0.md",
    ];
    for path in generated {
        fixture.write(path, "---\nid: x\ntype: generated\n---\n\n# Generated\n");
    }
    fixture.write(
        "knowledge/README.md",
        include_str!("fixtures/knowledge-readme-at-8fbbbe7.md"),
    );
    for path in [
        "knowledge/attachments/a1-0123abcd/evidence/OBS-key-item.json",
        "knowledge/attachments/a1-0123abcd/artifacts/run/screen.png",
        "knowledge/attachments/a1-0123abcd/pruned.jsonl",
        "knowledge/attachments/BUG-0001/evidence/OBS-fixture.json",
        "knowledge/snapshots/.staging-SNAP-x/copy.md",
    ] {
        fixture.write(path, "copied\n");
    }
    fixture.write(
        "knowledge/attachments/old-session/sessions/old-session.json",
        // A whole 0.5.x archived session, as the roll-up hard-linked it; the session
        // mapper (t382) carries it as an archived story.
        "{\"schema_version\": 2, \"objective\": \"Old work\", \"stage\": {\"current\": \"done\", \"status\": \"active\", \"callers\": [], \"iteration\": 1}, \"next_action\": \"\", \"history_cursor\": {\"cycle\": \"CYC-000000000000000000000001\", \"head\": null, \"legacy\": null}}\n",
    );
    fixture.write(
        "knowledge/lessons/2026-01-03-shared-lesson.md",
        "---\nid: 2026-01-03-shared-lesson\ntitle: \"A shared lesson\"\ntype: lesson\ntags: []\n---\n\n# A shared lesson\n",
    );
    fixture.write(
        "knowledge/briefs/brief.md",
        "A brief a person wrote, from /Users/SYNTHETIC-user/notes/brief.txt\n",
    );
    fixture.write("knowledge/notes/mine.md", "A note a person wrote.\n");
    fixture.write(
        "knowledge/snapshots/SNAP-approved/manifest.json",
        "{\"source\": \"SYNTHETIC\"}\n",
    );
    fixture.write("knowledge/snapshots/SNAP-approved/copy.md", "approved v1\n");
    fixture.write("library/briefs/earliest.md", "an earliest brief\n");
    fixture.commit("a 0.5.x vault");
}

/// REQ-records-009, REQ-cli-044 (TC-records-migrate-from-0-5, knowledge part): every
/// generated or copied 0.5.x vault file is planned for removal with why; lessons, briefs,
/// snapshots, the earlier `library/` folder and a person's note are kept byte for byte;
/// the kept brief's home path is listed as a leak, unchanged; a rolled session is left to
/// the session mapper; nothing is written; and the planned `index.md` and feed are
/// exactly what `chough library index` builds once the removals apply, so `library index
/// --check` passes on the written files.
#[test]
fn a_0_5_vault_is_planned_with_removals_kept_files_leaks_and_a_current_index() {
    let fixture = Fixture::new();
    vault_0_5(&fixture);
    let before = tree(fixture.root());
    let plan = preview(fixture.root());
    assert_eq!(tree(fixture.root()), before, "the preview wrote");

    let removed = plan["removed"]
        .as_object()
        .unwrap_or_else(|| panic!("{plan:#}"));
    let why = |path: &str| -> String {
        removed
            .get(path)
            .and_then(Value::as_str)
            .unwrap_or_else(|| panic!("{path} is not planned for removal: {removed:#?}"))
            .to_string()
    };
    assert!(why("knowledge/INDEX.md").contains("index.md"));
    assert!(why("knowledge/README.md").contains("README an earlier release generated"));
    for path in [
        "knowledge/rules.md",
        "knowledge/timeline.md",
        "knowledge/timeline/2026-11.md",
        "knowledge/archives/a1.md",
        "knowledge/modules/cli.md",
        "knowledge/intent/knowledge.md",
        "knowledge/bugs/BUG-0001.md",
        "knowledge/sessions/s0.md",
    ] {
        assert!(why(path).contains("REQ-cli-044"), "{path}: {}", why(path));
    }
    assert!(why("knowledge/attachments/a1-0123abcd/pruned.jsonl").contains("ledger"));
    for path in [
        "knowledge/attachments/a1-0123abcd/evidence/OBS-key-item.json",
        "knowledge/attachments/a1-0123abcd/artifacts/run/screen.png",
        "knowledge/attachments/BUG-0001/evidence/OBS-fixture.json",
    ] {
        assert!(why(path).contains("REQ-cli-046"), "{path}: {}", why(path));
    }
    assert!(why("knowledge/snapshots/.staging-SNAP-x/copy.md").contains("staging"));
    let rolled = "knowledge/attachments/old-session/sessions/old-session.json";
    assert!(
        !removed.contains_key(rolled),
        "the rolled session is the session mapper's"
    );

    // Each removal is a planned source with its digest, so the apply removes only those
    // bytes; nothing a person made is planned for removal.
    let sources = plan["sources"]
        .as_object()
        .unwrap_or_else(|| panic!("{plan:#}"));
    for path in removed.keys() {
        assert_eq!(
            sources.get(path).and_then(Value::as_str),
            Some(sha256(&fixture.read(path).into_bytes()).as_str()),
            "{path}"
        );
    }
    let kept = [
        "knowledge/lessons/2026-01-03-shared-lesson.md",
        "knowledge/briefs/brief.md",
        "knowledge/notes/mine.md",
        "knowledge/snapshots/SNAP-approved/manifest.json",
        "knowledge/snapshots/SNAP-approved/copy.md",
        "library/briefs/earliest.md",
    ];
    for path in kept {
        assert!(!removed.contains_key(path), "{path} would be removed");
    }
    assert_eq!(
        plan["kept_with_leaks"],
        serde_json::json!({"knowledge/briefs/brief.md": ["home-path"]}),
        "{plan:#}"
    );

    // Apply the plan for real (t382: the session mapper now carries the rolled session
    // and the framework writes .chough/README.md, both of which the index links), then
    // rebuild: the planned writes are byte for byte what `library index` builds, and
    // every kept file is unchanged.
    fixture.commit("the 0.5.x vault");
    let plan = fixture.ok(&["migrate"]);
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    fixture.ok(&[
        "migrate",
        "--apply",
        &digest,
        "--consent",
        "SYNTHETIC-human said migrate the vault (fixture)",
    ]);
    let written = writes(&plan);
    for path in removed.keys() {
        // On a case-insensitive file system knowledge/INDEX.md and the written
        // knowledge/index.md are one file, which the apply replaced in place.
        if written.keys().any(|kept| kept.eq_ignore_ascii_case(path)) {
            continue;
        }
        assert!(
            !fixture.root().join(path).exists(),
            "{path} was not removed"
        );
    }
    fixture.ok(&["library", "index", "--check"]);
    fixture.ok(&["library", "index"]);
    let planned = writes(&plan);
    for path in ["knowledge/index.md", "knowledge/journal/feed.xml"] {
        assert_eq!(
            planned.get(path),
            Some(&sha256(&bytes_of(&fixture, path))),
            "{path}: planned bytes differ from library index:\n{}",
            fixture.read(path)
        );
    }
    let index = fixture.read("knowledge/index.md");
    for linked in [
        "[lessons/2026-01-03-shared-lesson.md](lessons/2026-01-03-shared-lesson.md)",
        "[briefs/brief.md](briefs/brief.md)",
        "[snapshots/SNAP-approved/manifest.json](snapshots/SNAP-approved/manifest.json)",
        "[library/briefs/earliest.md](../library/briefs/earliest.md)",
    ] {
        assert!(index.contains(linked), "missing {linked} in\n{index}");
    }
    for path in kept.iter() {
        assert_eq!(
            fixture.read(path).into_bytes(),
            before[*path],
            "{path} changed"
        );
    }
    // The session mapper carried the rolled session into an archived story.
    assert!(!fixture.root().join(rolled).exists(), "{rolled} stayed");
    let story = plan["aliases"]["old-session"].as_str().unwrap();
    assert!(story.starts_with(".chough/archive/sessions/"), "{story}");
    assert!(
        fixture.root().join(story).is_file(),
        "{story} was not written"
    );
}

/// The bytes of `path` in the fixture.
fn bytes_of(fixture: &Fixture, path: &str) -> Vec<u8> {
    fs::read(fixture.root().join(path)).unwrap()
}

/// REQ-records-009: with the librarian off, the generated 0.5.x vault content is still
/// removed with why, and no index or feed is planned.
#[test]
fn with_the_librarian_off_the_vault_is_still_cleared_and_no_index_is_planned() {
    let fixture = Fixture::new();
    vault_0_5(&fixture);
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: SYNTHETIC no librarian here",
    ]);
    let plan = preview(fixture.root());
    assert!(
        plan["removed"]["knowledge/INDEX.md"].is_string(),
        "{plan:#}"
    );
    let planned = writes(&plan);
    assert!(
        planned.keys().all(|path| !path.starts_with("knowledge/")),
        "{planned:#?}"
    );
}
