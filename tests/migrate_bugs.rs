//! TC-records-migrate-from-0-5, the bug part (REQ-records-009, REQ-cli-012): `chough
//! migrate` previews each 0.5.x bug as a Markdown bug named by its creation date and
//! title, every 0.5.x ID an alias, links to observations that were not carried read
//! `removed`, an open bug's fixture is kept and its leak listed, a closed bug's
//! artifact fixture is removed, and every leak in a written bug is redacted and counted.
//! The preview writes nothing.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// Every file under `root`, with its bytes.
fn tree(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(base: &Path, dir: &Path, files: &mut Vec<(String, Vec<u8>)>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, files);
            } else {
                let name = path.strip_prefix(base).unwrap().display().to_string();
                files.push((name, fs::read(&path).unwrap()));
            }
        }
    }
    let mut files = vec![];
    walk(root, root, &mut files);
    files.sort();
    files
}

/// `chough migrate`'s preview of the project at `root`.
fn preview(root: &Path) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(root)
        .arg("migrate")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

/// `YYYY-MM-DD` for a Unix time, in UTC.
fn utc_date(seconds: u64) -> String {
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Fixtures captured from every released 0.x tag (#213) each hold one bug; the preview
/// carries it under its date and title with its 0.5.x ID as the alias, and changes
/// nothing on disk.
#[test]
fn every_release_fixtures_bug_migrates_under_its_date_and_title_with_its_old_id() {
    for tag in ["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"] {
        let directory = tempfile::tempdir().unwrap();
        let root = directory.path();
        copy_dir(
            &manifest_dir().join("tests/fixtures/releases").join(tag),
            root,
        );
        let bugs: Vec<_> = fs::read_dir(root.join(".chough/bugs"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension()
                    .is_some_and(|extension| extension == "json")
            })
            .collect();
        assert_eq!(bugs.len(), 1, "{tag}");
        let legacy: Value = serde_json::from_str(&fs::read_to_string(&bugs[0]).unwrap()).unwrap();
        let old = legacy["id"].as_str().unwrap();
        let expected = format!(
            ".chough/bugs/{}-fixture-bug-example-record.md",
            utc_date(legacy["created_at_unix_seconds"].as_u64().unwrap())
        );
        let before = tree(root);
        let plan = preview(root);
        assert_eq!(tree(root), before, "{tag}: the preview changed a file");
        assert_eq!(plan["aliases"][old], expected.as_str(), "{tag}: {plan:#}");
        assert!(
            plan["writes"]
                .as_array()
                .unwrap()
                .iter()
                .any(|write| write["path"] == expected.as_str()),
            "{tag}: {plan:#}"
        );
        let source = format!(
            ".chough/bugs/{}",
            bugs[0].file_name().unwrap().to_string_lossy()
        );
        assert!(
            plan["sources"].get(&source).is_some(),
            "{tag}: {source} not listed: {plan:#}"
        );
        assert_eq!(plan["kept_with_leaks"], serde_json::json!({}), "{tag}");
    }
}

/// A synthetic project with a bug of each ID form: an open `BUG-NNNN` bug with a fixture
/// holding a home path and a note naming one, linking an observation no record holds;
/// and a closed `BUG-<slug>-<hex>` bug whose artifact fixture no open bug names.
#[test]
fn a_bug_of_each_id_form_carries_its_links_fixtures_and_redactions() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let write = |relative: &str, text: &str| {
        let path = root.join(relative);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, text).unwrap();
    };
    let bug = |id: &str,
               title: &str,
               status: &str,
               fixture: &str,
               observations: &str,
               notes: &str| {
        format!(
            r#"{{"schema_version":1,"id":"{id}","title":"{title}","symptom":"It breaks","trigger":"Run it","expected":"It works","actual":"It broke","criteria":[],"requirements":[],"cases":[],"environment":"local","revision":null,"steps":[],"fixture":"{fixture}","observations":{observations},"impact":"Some","classification":"product-defect","scope":"in-scope","status":"{status}","regression_baseline":null,"duplicate_of":null,"related":[],"source":"manual report","notes":{notes},"created_at_unix_seconds":1793527200,"updated_at_unix_seconds":1793527300}}"#
        )
    };
    write(
        ".chough/bugs/BUG-0003.json",
        &bug(
            "BUG-0003",
            "Open with a fixture",
            "open",
            ".chough/artifacts/repro/case.log",
            r#"["OBS-never-recorded-000000000000000000000001"]"#,
            r#"[{"at_unix_seconds":1793527250,"text":"Seen in /Users/SYNTHETIC-user/project","observations":[]}]"#,
        ),
    );
    write(
        ".chough/artifacts/repro/case.log",
        "failed under /Users/SYNTHETIC-user/project\n",
    );
    write(
        ".chough/bugs/BUG-closed-fixture-0123456789abcdef01234567.json",
        &bug(
            "BUG-closed-fixture-0123456789abcdef01234567",
            "Closed fixture",
            "verified",
            ".chough/artifacts/closed/proof.log",
            "[]",
            "[]",
        ),
    );
    write(".chough/artifacts/closed/proof.log", "passed\n");
    let before = tree(root);
    let plan = preview(root);
    assert_eq!(tree(root), before, "the preview changed a file");

    let open = ".chough/bugs/2026-11-01-open-with-a-fixture.md";
    let closed = ".chough/bugs/2026-11-01-closed-fixture.md";
    assert_eq!(plan["aliases"]["BUG-0003"], open, "{plan:#}");
    assert_eq!(
        plan["aliases"]["BUG-closed-fixture-0123456789abcdef01234567"], closed,
        "{plan:#}"
    );
    assert_eq!(
        plan["aliases"]["OBS-never-recorded-000000000000000000000001"], "removed",
        "{plan:#}"
    );
    assert_eq!(
        plan["kept_with_leaks"],
        serde_json::json!({".chough/artifacts/repro/case.log": ["home-path"]}),
        "{plan:#}"
    );
    assert!(
        plan["removed"][".chough/artifacts/closed/proof.log"]
            .as_str()
            .is_some_and(|why| why.contains("BUG-closed-fixture-0123456789abcdef01234567")),
        "{plan:#}"
    );
    assert!(
        plan["removed"]
            .get(".chough/artifacts/repro/case.log")
            .is_none()
    );
    // The note's home path is redacted in the written bug and counted, never printed.
    assert!(
        plan["redactions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|redaction| redaction["file"] == open && redaction["kind"] == "home-path"),
        "{plan:#}"
    );
    assert!(!plan.to_string().contains("SYNTHETIC-user"), "{plan:#}");
}
