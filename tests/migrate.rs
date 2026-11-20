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
            // The Git-ignored writer lock is not a write of the migration (REQ-records-009).
            if name == ".git" || name == ".chough/local" {
                continue;
            }
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else if entry.file_type().is_ok_and(|kind| kind.is_file()) {
                // A file without read permission is named, never read.
                let bytes = std::fs::read(entry.path()).unwrap_or_else(|_| b"<unreadable>".to_vec());
                out.insert(name, bytes);
            } else {
                // A FIFO or other special file is named, never opened, so no read waits.
                out.insert(name, b"<special>".to_vec());
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
    let plan = fixture.ok(&["migrate", "--json"]);
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
    let replanned = fixture.ok(&["migrate", "--json"])["plan_digest"]
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

    let replanned = fixture.ok(&["migrate", "--json"]);
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
        let plan = fixture.ok(&["migrate", "--json"]);
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
        let again = fixture.ok(&["migrate", "--json"]);
        assert_eq!(
            again["plan_digest"], plan["plan_digest"],
            "{tag}: the plan is not stable"
        );
        let _: &Value = &plan;
    }
}

// ---------------------------------------------------------------------------
// REQ-records-009's criteria on projects the real old builds wrote
// (tests/fixtures/migrate/regenerate.sh).

const CONSENT: &str = "SYNTHETIC-human said migrate it (fixture)";

/// A project holding `tests/fixtures/migrate/<name>`, with `extra` files added, committed.
fn migrate_project(name: &str, extra: &[(&str, &str)]) -> Fixture {
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
    copy(&fixture_dir(name), fixture.root());
    for (path, text) in extra {
        fixture.write(path, text);
    }
    fixture.commit("a 0.5.x project");
    fixture
}

fn fixture_dir(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/migrate")
        .join(name)
}

/// Preview, then apply the previewed digest with `consent`.
fn migrate(fixture: &Fixture, consent: Option<&str>) -> (Value, Value) {
    let plan = fixture.ok(&["migrate", "--json"]);
    assert_eq!(plan["unaccounted"], serde_json::json!([]), "{plan:#}");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let mut args = vec!["migrate", "--apply", digest.as_str()];
    if let Some(consent) = consent {
        args.extend(["--consent", consent]);
    }
    let applied = fixture.ok(&args);
    assert_eq!(applied["applied"], true, "{applied:#}");
    (plan, applied)
}

fn written(plan: &Value) -> Vec<String> {
    plan["writes"]
        .as_array()
        .unwrap()
        .iter()
        .map(|write| write["path"].as_str().unwrap().to_string())
        .collect()
}

/// The planned path ending in `suffix`.
fn planned(plan: &Value, suffix: &str) -> String {
    written(plan)
        .into_iter()
        .find(|path| path.ends_with(suffix))
        .unwrap_or_else(|| panic!("no planned write ends with {suffix}: {plan:#}"))
}

/// `YYYY-MM-DDTHH:MM:SSZ` for a Unix time.
fn iso(seconds: u64) -> String {
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
    let time = seconds % 86_400;
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        time / 3_600,
        time % 3_600 / 60,
        time % 60
    )
}

/// `(time, kind, from, to)` of a 0.5.x transition.
fn moved(transition: &Value) -> (String, String, String, String) {
    let text = |key: &str| transition[key].as_str().unwrap().to_string();
    (
        iso(transition["at_unix_seconds"].as_u64().unwrap()),
        text("kind"),
        text("from"),
        text("to"),
    )
}

/// The transitions a 0.5.x session's history cursor selects in the fixture, oldest first:
/// the legacy log's lines when the cursor anchors it, then the events from the oldest
/// cycle to the head, read straight from the fixture's files.
fn selected_lineage(name: &str, session: &str) -> Vec<(String, String, String, String)> {
    let root = fixture_dir(name);
    let session: Value =
        serde_json::from_str(&std::fs::read_to_string(root.join(session)).unwrap()).unwrap();
    let mut cursor = session["history_cursor"].clone();
    let mut events = vec![];
    let mut legacy = cursor["legacy"].is_object();
    while let Some(head) = cursor["head"].as_str().map(str::to_string) {
        let cycle = cursor["cycle"].as_str().unwrap().to_string();
        let event: Value = serde_json::from_str(
            &std::fs::read_to_string(
                root.join(format!(".chough/history/cycles/{cycle}/{head}.json")),
            )
            .unwrap(),
        )
        .unwrap();
        events.push(moved(&event["transition"]));
        match event["parent"].as_str() {
            Some(parent) => cursor["head"] = Value::from(parent),
            None if event["previous_cycle"].is_object() => {
                cursor = event["previous_cycle"].clone();
                legacy |= cursor["legacy"].is_object();
            }
            None => break,
        }
    }
    events.reverse();
    let mut lineage = vec![];
    if legacy {
        let log = std::fs::read_to_string(root.join(".chough/transitions.jsonl")).unwrap();
        for line in log.lines().filter(|line| !line.trim().is_empty()) {
            lineage.push(moved(&serde_json::from_str(line).unwrap()));
        }
    }
    lineage.extend(events);
    lineage
}

/// `(time, kind, from, to)` of each Timeline entry of a session record.
fn timeline_moves(text: &str) -> Vec<(String, String, String, String)> {
    common::timeline(text)
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.trim_start_matches("- ").split(" · ").collect();
            let (from, to) = parts[2].split_once(" → ").unwrap();
            (
                parts[0].to_string(),
                parts[1].to_string(),
                from.to_string(),
                to.to_string(),
            )
        })
        .collect()
}

/// REQ-records-009 (TC-records-migrate-from-0-5): a project 0.1.0 and 0.5.0 wrote, whose
/// legacy checkpoint's history a session adopted (its cursor anchors transitions.jsonl),
/// with two open and seven archived sessions, one more rolled into the vault, inbox
/// entries and a home path in a session's decision. Every transition of each selected
/// lineage appears once and in order, the legacy lines in the adopter's story; the
/// rolled session is a story; the inbox entries land in `agent/inbox/`; the home path is
/// redacted and counted.
#[test]
fn an_adopted_project_migrates_every_lineage_rolled_story_inbox_entry_and_redaction() {
    let fixture = migrate_project("adopted", &[]);
    let (plan, _) = migrate(&fixture, Some(CONSENT));

    // Every session's story or folder holds its selected lineage, once and in order.
    let mut sessions: Vec<(String, String, bool)> = vec![
        (
            "adopter".into(),
            ".chough/sessions/archive/adopter.json".into(),
            true,
        ),
        (
            "finisher".into(),
            ".chough/sessions/archive/finisher.json".into(),
            true,
        ),
        (
            "archived-1".into(),
            "knowledge/attachments/archived-1/sessions/archived-1.json".into(),
            true,
        ),
        (
            "open-one".into(),
            ".chough/sessions/open-one.json".into(),
            false,
        ),
        (
            "open-two".into(),
            ".chough/sessions/open-two.json".into(),
            false,
        ),
    ];
    for index in 2..=6 {
        sessions.push((
            format!("archived-{index}"),
            format!(".chough/sessions/archive/archived-{index}.json"),
            true,
        ));
    }
    for (key, source, archived) in sessions {
        let target = if archived {
            planned(&plan, &format!("-{key}.md"))
        } else {
            planned(&plan, &format!("-{key}/session.md"))
        };
        assert!(
            target.starts_with(if archived {
                ".chough/archive/sessions/"
            } else {
                ".chough/sessions/"
            }),
            "{key}: {target}"
        );
        assert_eq!(plan["aliases"][&key], target.as_str(), "{key}: {plan:#}");
        let expected = selected_lineage("adopted", &source);
        assert_eq!(
            timeline_moves(&fixture.read(&target)),
            expected,
            "{key}: the timeline is not the selected lineage"
        );
        if key == "adopter" {
            let log =
                std::fs::read_to_string(fixture_dir("adopted").join(".chough/transitions.jsonl"))
                    .unwrap();
            let legacy: Vec<_> = log
                .lines()
                .map(|line| moved(&serde_json::from_str(line).unwrap()))
                .collect();
            assert_eq!(legacy.len(), 3);
            assert_eq!(
                expected[..3],
                legacy[..],
                "the legacy lines open the adopter's story"
            );
        }
    }
    // The rolled session's 0.5.x file was carried and removed.
    assert!(!fixture.exists("knowledge/attachments/archived-1/sessions/archived-1.json"));
    assert!(!fixture.exists("knowledge/attachments/archived-1"));

    // The inbox entries land in the open session's agent/inbox/.
    let inbox = planned(&plan, "-open-two/session.md").replace("session.md", "agent/inbox/");
    let mut entries: Vec<String> = std::fs::read_dir(fixture.root().join(&inbox))
        .unwrap()
        .map(|entry| {
            fixture.read(&format!(
                "{inbox}{}",
                entry.unwrap().file_name().to_string_lossy()
            ))
        })
        .collect();
    entries.sort();
    assert_eq!(entries.len(), 2, "{entries:?}");
    assert!(
        entries
            .iter()
            .any(|entry| entry.contains("Keep the second run?"))
            && entries
                .iter()
                .any(|entry| entry.contains("Use the smaller fixture")),
        "{entries:?}"
    );

    // The home path is redacted and counted, by file and kind, never printed.
    let open_one = planned(&plan, "-open-one/session.md");
    assert!(
        plan["redactions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|redaction| redaction["file"] == open_one.as_str()
                && redaction["kind"] == "home-path"),
        "{plan:#}"
    );
    let text = fixture.read(&open_one);
    assert!(text.contains("<redacted:home-path>"), "{text}");
    assert!(!text.contains("SYNTHETIC-user"), "{text}");
    assert!(!plan.to_string().contains("SYNTHETIC-user"));
}

/// TC-records-migrate-vaulted-adopter: the regression for the local bug where history
/// writes refused once the adopting session rolled into the vault. The project the last
/// build before #142 wrote: its roll-up moved the adopter itself into
/// knowledge/attachments/ with transitions.jsonl unchanged, and that build refuses
/// `checkpoint --awaiting-human` and `stage advance` there ("legacy history anchor
/// changed"). It migrates; the legacy entries land in the adopter's story; afterwards an
/// awaiting-human gate, its close and a stage move all succeed.
#[test]
fn a_vaulted_adopter_migrates_and_writes_work() {
    let fixture = migrate_project("vaulted-adopter", &[]);
    assert!(fixture.exists("knowledge/attachments/adopter/sessions/adopter.json"));
    let (plan, _) = migrate(&fixture, Some(CONSENT));
    let story = planned(&plan, "-adopter.md");
    assert!(story.starts_with(".chough/archive/sessions/"), "{story}");
    let lineage = selected_lineage(
        "vaulted-adopter",
        "knowledge/attachments/adopter/sessions/adopter.json",
    );
    assert_eq!(timeline_moves(&fixture.read(&story)), lineage);
    let log =
        std::fs::read_to_string(fixture_dir("vaulted-adopter").join(".chough/transitions.jsonl"))
            .unwrap();
    let legacy: Vec<_> = log
        .lines()
        .map(|line| moved(&serde_json::from_str(line).unwrap()))
        .collect();
    assert_eq!(lineage[..legacy.len()], legacy[..]);
    assert!(!fixture.exists(".chough/transitions.jsonl"));
    assert!(!fixture.exists("knowledge/attachments/adopter"));

    fixture.ok(&["checkpoint", "--awaiting-human", "Review"]);
    // Every gate close names a configured human (REQ-records-008).
    fixture.configure_human();
    fixture.ok(&[
        "checkpoint",
        "--close-gate",
        "Review",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "Reviewed, go on",
        "--where",
        "fixture chat",
        "--recorded-by",
        "SYNTHETIC-agent",
        "--no-native",
    ]);
    let before = fixture.checkpoint()["stage"]["current"].clone();
    fixture.ok(&["stage", "advance", "--json"]);
    assert_ne!(fixture.checkpoint()["stage"]["current"], before);
}

/// The SpecSync stand-in for the archive guard: each `change status <id> --json` read
/// counts how many reads run at that moment, holds for 0.3 s, and reads the archive
/// authenticated while `.chough/sessions/open-two.json` exists and invalid once it is
/// gone.
const VALIDITY_STUB: &str = r#"#!/bin/sh
mkdir -p .stub-running
touch ".stub-running/$$"
ls .stub-running | wc -l | tr -d ' ' >> .stub-concurrency
sleep 0.3
rm -f ".stub-running/$$"
if [ -f .chough/sessions/open-two.json ]; then validity=authenticated-history; else validity=invalid; fi
printf '{"summary":{"id":"%s","state":"archived","terminal_evidence":{"validity":"%s"}}}\n' "$3" "$validity"
"#;

/// REQ-records-009, the archive guard: twenty archived changes list a 0.5.x file the
/// migration removes; with a SpecSync stand-in that reads them invalid once it is gone,
/// `--apply` restores every removed file byte for byte and refuses naming them, and at
/// most eight validity reads run at once.
#[cfg(unix)]
#[test]
fn a_weakened_archive_restores_every_removed_file_and_refuses() {
    let mut extra: Vec<(String, String)> = vec![];
    for index in 0..20 {
        let dir = format!(".specsync/archive/changes/2026-01-01-change-{index:02}");
        extra.push((
            format!("{dir}/verification.json"),
            r#"{"acceptance_manifest":{"entries":[{"path":".chough/sessions/open-two.json"}]}}"#
                .to_string(),
        ));
        extra.push((
            format!("{dir}/finalization.json"),
            format!(r#"{{"change_id":"change-{index:02}"}}"#),
        ));
    }
    let extra: Vec<(&str, &str)> = extra
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let fixture = migrate_project("adopted", &extra);
    std::fs::create_dir_all(fixture.root().join("bin")).unwrap();
    common::install_executable(
        &fixture.root().join("bin/specsync"),
        VALIDITY_STUB.as_bytes(),
    );
    let plan = fixture.ok(&["migrate", "--json"]);
    let digest = plan["plan_digest"].as_str().unwrap();
    let refused = fixture.fails(&["migrate", "--apply", digest, "--consent", CONSENT]);
    assert!(refused.contains("refused:archive-weakened"), "{refused}");
    for index in 0..20 {
        assert!(refused.contains(&format!("change-{index:02}")), "{refused}");
    }
    // Every removed 0.5.x file is back, byte for byte.
    for (path, sha) in plan["sources"].as_object().unwrap() {
        let bytes = std::fs::read(fixture.root().join(path))
            .unwrap_or_else(|_| panic!("{path} was not restored"));
        assert_eq!(&sha256(&bytes), sha.as_str().unwrap(), "{path}");
    }
    // Two rounds of twenty reads, never more than eight at once.
    let counts: Vec<usize> = fixture
        .read(".stub-concurrency")
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();
    assert_eq!(counts.len(), 40, "{counts:?}");
    let most = counts.iter().max().copied().unwrap_or(0);
    assert!(
        (2..=8).contains(&most),
        "{most} reads ran at once: {counts:?}"
    );
}

/// A 0.5.x bug, so an interrupted apply leaves a migrated bug behind too.
const LEGACY_BUG: &str = r#"{"schema_version":1,"id":"BUG-0001","title":"Fixture bug","symptom":"It breaks","trigger":"Run it","expected":"It works","actual":"It broke","criteria":[],"requirements":[],"cases":[],"environment":"fixture","revision":null,"steps":[],"fixture":null,"observations":[],"impact":"Some","classification":"product-defect","scope":"in-scope","status":"open","regression_baseline":null,"duplicate_of":null,"related":[],"source":"manual report","notes":[],"created_at_unix_seconds":1793527200,"updated_at_unix_seconds":1793527300}"#;

/// The project's files without Git's; in the receipt, its apply time is left out and the
/// commit holding the removed files reads `<commit>`, since two projects' commits differ.
fn settled_files(root: &Path, receipt: &str) -> BTreeMap<String, Vec<u8>> {
    let head = std::process::Command::new("git")
        .current_dir(root)
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let head = String::from_utf8_lossy(&head.stdout).trim().to_string();
    let mut all = files(root);
    // Machine-local state, such as the writer lock the apply holds, is never committed
    // and is not part of the migrated tree (REQ-records-002).
    all.retain(|path, _| !path.starts_with(".chough/local/"));
    if let Some(bytes) = all.get_mut(receipt) {
        let text: String = String::from_utf8_lossy(bytes)
            .lines()
            .filter(|line| !line.starts_with("applied: "))
            .map(|line| format!("{}\n", line.replace(&head, "<commit>")))
            .collect();
        *bytes = text.into_bytes();
    }
    all
}

/// REQ-records-009: an apply stopped between writes leaves some planned files written and
/// nothing removed. The next preview has the same digest, the next apply completes, the
/// result equals an uninterrupted apply, and a second apply has nothing to do.
#[test]
fn an_interrupted_apply_completes_to_the_uninterrupted_result() {
    let extra = [(".chough/bugs/BUG-0001.json", LEGACY_BUG)];
    let whole = migrate_project("adopted", &extra);
    let (plan, applied) = migrate(&whole, Some(CONSENT));
    let receipt = applied["receipt"].as_str().unwrap().to_string();
    let writes = written(&plan);
    assert!(
        writes.iter().any(|path| path.starts_with(".chough/bugs/")),
        "{writes:?}"
    );

    let stopped = migrate_project("adopted", &extra);
    let first = stopped.ok(&["migrate", "--json"]);
    assert_eq!(first["plan_digest"], plan["plan_digest"]);
    // The stop: every planned file but the last written, with its planned bytes. A
    // file that differs only in case from a planned removal (knowledge/index.md for
    // knowledge/INDEX.md) is written after the removals, so a stop before them has not
    // written it.
    let removals: Vec<String> = plan["removed"]
        .as_object()
        .unwrap()
        .keys()
        .chain(plan["sources"].as_object().unwrap().keys())
        .cloned()
        .collect();
    let deferred = |path: &String| {
        removals
            .iter()
            .any(|removed| removed != path && removed.eq_ignore_ascii_case(path))
    };
    let mut left: Vec<String> = vec![];
    for (index, path) in writes.iter().enumerate() {
        if index + 1 == writes.len() || deferred(path) {
            left.push(path.clone());
            continue;
        }
        stopped.write(path, &whole.read(path));
    }
    let again = stopped.ok(&["migrate", "--json"]);
    assert_eq!(
        again["plan_digest"], plan["plan_digest"],
        "the files an interrupted apply wrote changed the plan"
    );
    let resumed = stopped.ok(&[
        "migrate",
        "--apply",
        plan["plan_digest"].as_str().unwrap(),
        "--consent",
        CONSENT,
    ]);
    assert_eq!(resumed["applied"], true, "{resumed:#}");
    let mut resumed_written: Vec<String> = resumed["written"]
        .as_array()
        .unwrap()
        .iter()
        .map(|path| path.as_str().unwrap().to_string())
        .collect();
    resumed_written.sort();
    left.sort();
    assert_eq!(resumed_written, left, "{resumed:#}");
    let (resumed_tree, whole_tree) = (
        settled_files(stopped.root(), &receipt),
        settled_files(whole.root(), &receipt),
    );
    let differing: Vec<&String> = resumed_tree
        .keys()
        .chain(whole_tree.keys())
        .filter(|path| resumed_tree.get(*path) != whole_tree.get(*path))
        .collect();
    assert!(
        differing.is_empty(),
        "the resumed apply differs from an uninterrupted one in {differing:?}"
    );
    let nothing = stopped.ok(&[
        "migrate",
        "--apply",
        plan["plan_digest"].as_str().unwrap(),
        "--consent",
        CONSENT,
    ]);
    assert_eq!(nothing["applied"], false, "{nothing:#}");
    assert!(nothing["nothing_to_do"].is_string(), "{nothing:#}");
}

/// REQ-records-009: a planned target that already exists with other bytes refuses the
/// apply, which writes and removes nothing.
#[test]
fn an_existing_target_with_other_bytes_refuses_and_writes_nothing_more() {
    let fixture = migrate_project("adopted", &[]);
    let plan = fixture.ok(&["migrate", "--json"]);
    let target = planned(&plan, "-adopter.md");
    fixture.write(&target, "SYNTHETIC other bytes\n");
    let before = files(fixture.root());
    let refused = fixture.fails(&[
        "migrate",
        "--apply",
        plan["plan_digest"].as_str().unwrap(),
        "--consent",
        CONSENT,
    ]);
    assert!(
        refused.contains("refused:target-exists") && refused.contains(&target),
        "{refused}"
    );
    assert_eq!(
        files(fixture.root()),
        before,
        "the refused apply changed a file"
    );
}

const CASES: &str = "profile = \"chough-testing/1\"\nmodule = \"fixture\"\n\n[[case]]\nid = \"TC-fixture-0001\"\ntitle = \"The fixture's one case\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-fixture-001\"]\npaths = [\"README.md\"]\nenvironments = [\"fixture\"]\nartifacts = [\"test-log\"]\nexpected = [\"the README prints its heading\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"cat\", \"README.md\"]\nexpect_output = [\"# Migration fixture project\"]\n";

/// REQ-records-009: case definitions and `.chough/preferences.json` are byte-identical
/// before and after the migration.
#[test]
fn case_definitions_and_preferences_stay_byte_identical() {
    let fixture = migrate_project(
        "vaulted-adopter",
        &[(".chough/cases/fixture.cases.toml", CASES)],
    );
    let kept = [
        ".chough/cases/fixture.cases.toml",
        ".chough/preferences.json",
    ];
    let before: Vec<Vec<u8>> = kept
        .iter()
        .map(|path| std::fs::read(fixture.root().join(path)).unwrap())
        .collect();
    migrate(&fixture, Some(CONSENT));
    for (path, bytes) in kept.iter().zip(before) {
        assert_eq!(
            std::fs::read(fixture.root().join(path)).unwrap(),
            bytes,
            "{path} changed"
        );
    }
}

/// REQ-records-009, REQ-cli-013: a standing `migration` preference gives the consent
/// without `--consent`, and the receipt names its source.
#[test]
fn a_standing_migration_preference_gives_consent_and_the_receipt_names_it() {
    let source = "SYNTHETIC-human said migrate whenever a preview is clean (fixture chat)";
    let preferences = format!(
        "{{\"schema_version\":1,\"preferences\":[{{\"key\":\"migration\",\"value\":\"standing\",\"source\":\"{source}\",\"scope\":\"project\",\"set_at_unix_seconds\":1793500000}}]}}\n"
    );
    let fixture = migrate_project("adopted", &[(".chough/preferences.json", &preferences)]);
    let (_, applied) = migrate(&fixture, None);
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    let consent = receipt
        .lines()
        .find(|line| line.starts_with("consent: "))
        .unwrap_or_else(|| panic!("{receipt}"));
    assert!(
        consent.contains("standing preference") && consent.contains(source),
        "{consent}"
    );
    assert_eq!(fixture.read(".chough/preferences.json"), preferences);
}

/// The migration rehearsal on this repository (plan Migration step 2): every 0.5.x file
/// REQ-records-009 removes is in the plan. `.chough/session.v1.json` and
/// `.chough/pruned.jsonl` are removed with why, and a file under `.chough/artifacts/` or
/// `.chough/validation/` is a source the plan carries, removes or reports as
/// unaccounted, which the apply refuses; never silently left behind.
#[test]
fn every_legacy_file_and_artifact_is_accounted_for() {
    let stray = ".chough/artifacts/stray/run.log";
    let run = ".chough/validation/VAL-stray-000000000000000000000001.json";
    let fixture = migrate_project(
        "adopted",
        &[
            (".chough/session.v1.json", "{\"schema_version\": 1}\n"),
            (".chough/pruned.jsonl", "{\"archives\": []}\n"),
            (stray, "passed\n"),
            (run, "{\"schema_version\": 1}\n"),
        ],
    );
    let plan = fixture.ok(&["migrate", "--json"]);
    for path in [
        ".chough/session.v1.json",
        ".chough/pruned.jsonl",
        stray,
        run,
    ] {
        assert!(
            plan["sources"].get(path).is_some(),
            "{path} is not in the plan's sources: {plan:#}"
        );
    }
    for (path, why) in [
        (".chough/session.v1.json", "schema 1"),
        (".chough/pruned.jsonl", "retention"),
    ] {
        assert!(
            plan["removed"][path]
                .as_str()
                .is_some_and(|text| text.contains(why)),
            "{path} is not removed with why: {plan:#}"
        );
    }
    let unaccounted: Vec<&str> = plan["unaccounted"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .collect();
    assert!(
        !unaccounted.contains(&".chough/session.v1.json")
            && !unaccounted.contains(&".chough/pruned.jsonl"),
        "{plan:#}"
    );
    // Until the evidence mapper carries or removes it, the stray artifact is
    // unaccounted, and the apply refuses rather than leave it in a migrated tree.
    if unaccounted.contains(&stray) {
        let digest = plan["plan_digest"].as_str().unwrap();
        let before = files(fixture.root());
        let refused = fixture.fails(&["migrate", "--apply", digest, "--consent", CONSENT]);
        assert!(
            refused.contains("refused:unaccounted") && refused.contains(stray),
            "{refused}"
        );
        assert_eq!(files(fixture.root()), before, "a refused apply wrote");
    } else {
        assert!(plan["removed"].get(stray).is_some(), "{plan:#}");
    }
    // A validation run is carried, removed with why, or unaccounted; never left out.
    assert!(
        unaccounted.contains(&run)
            || plan["removed"].get(run).is_some()
            || plan["aliases"]
                .as_object()
                .unwrap()
                .contains_key("VAL-stray-000000000000000000000001"),
        "{plan:#}"
    );
}

/// A project holding the whole v0.5.0 release fixture, committed: sessions, history,
/// evidence, a JSON bug and its artifacts, all in the 0.5.x layout.
fn unmigrated_project() -> Fixture {
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
    let source = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases/v0.5.0");
    copy(&source, fixture.root());
    fixture.commit("a 0.5.x project");
    fixture
}

/// The refusal of REQ-records-010: typed, naming the layout and both migrate commands.
fn assert_refused_unmigrated(refusal: &str, context: &str) {
    for needed in [
        "refused:unmigrated",
        "Chough 0.5.x layout",
        "`chough migrate`",
        "`chough migrate --apply",
    ] {
        assert!(
            refusal.contains(needed),
            "{context}: no {needed:?} in {refusal}"
        );
    }
}

/// TC-records-unmigrated-fails-closed (REQ-records-010): while any 0.5.x record exists,
/// every command but migrate, doctor and skill (and clap's --help and --version) refuses
/// as `refused:unmigrated` and writes nothing; the refusal names up to 5 of the paths and
/// counts the rest; a 0.5.x file written into a migrated tree is refused the same way.
#[test]
fn an_unmigrated_tree_is_refused_with_the_migrate_command() {
    let fixture = unmigrated_project();
    let before = files(fixture.root());
    let refused: [&[&str]; 8] = [
        &["next"],
        &["status"],
        &["stage", "advance", "--json"],
        &["checkpoint", "--decision", "SYNTHETIC decision"],
        &[
            "evidence",
            "add",
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-fixture-001",
            "--contract",
            "specs/fixture/fixture.spec.md",
            "--kind",
            "product-test",
            "--outcome",
            "passed",
            "--environment",
            "fixture",
            "--summary",
            "SYNTHETIC observation",
        ],
        &[
            "bug",
            "add",
            "--title",
            "SYNTHETIC bug",
            "--symptom",
            "SYNTHETIC symptom",
            "--trigger",
            "SYNTHETIC trigger",
            "--expected",
            "SYNTHETIC expected",
            "--actual",
            "SYNTHETIC actual",
            "--environment",
            "fixture",
        ],
        &[
            "cases",
            "run",
            "TC-fixture-0001",
            "--environment",
            "fixture",
        ],
        &["init", "--objective", "SYNTHETIC objective"],
    ];
    for args in refused {
        let refusal = fixture.fails(args);
        assert_refused_unmigrated(&refusal, &args.join(" "));
        assert!(
            refusal.contains(".chough/bugs/BUG-fixture-bug-example-record-"),
            "{args:?}: the refusal names no 0.5.x path: {refusal}"
        );
        assert_eq!(
            files(fixture.root()),
            before,
            "{args:?} wrote something on an unmigrated tree"
        );
    }

    // migrate, doctor and skill run on the same tree; clap answers --help and --version.
    let plan = fixture.ok(&["migrate", "--json"]);
    assert!(
        plan["apply"]
            .as_str()
            .is_some_and(|apply| apply.starts_with("chough migrate --apply ")),
        "{plan:#}"
    );
    for args in [
        &["doctor"][..],
        &["skill", "status"],
        &["--help"],
        &["--version"],
    ] {
        let output = fixture.run(args);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("refused:unmigrated"), "{args:?}: {stderr}");
        assert!(
            output.status.success() || args == ["doctor"],
            "{args:?}: {stderr}"
        );
    }
    assert_eq!(
        files(fixture.root()),
        before,
        "an allowed command wrote a record"
    );

    // With 12 0.5.x paths the refusal names the first 5, sorted, and says 7 more.
    let fixture = Fixture::new();
    let legacy = [
        ".chough/bugs/BUG-0001.json",
        ".chough/bugs/BUG-0002.json",
        ".chough/evidence/OBS-1.json",
        ".chough/evidence/OBS-2.json",
        ".chough/history/cycles/CYC-1/EVT-1.json",
        ".chough/pruned.jsonl",
        ".chough/session.json",
        ".chough/sessions/archive/archived.json",
        ".chough/sessions/open.json",
        ".chough/sessions/shelved/set-aside.json",
        ".chough/sessions/second.json",
        ".chough/transitions.jsonl",
    ];
    for path in legacy {
        fixture.write(path, "{}\n");
    }
    let before = files(fixture.root());
    let refusal = fixture.fails(&["next"]);
    assert_refused_unmigrated(&refusal, "12 paths");
    for path in &legacy[..5] {
        assert!(refusal.contains(path), "{path} is not named: {refusal}");
    }
    for path in &legacy[5..] {
        assert!(
            !refusal.contains(path),
            "{path} is named past the 5: {refusal}"
        );
    }
    assert!(refusal.contains("and 7 more"), "{refusal}");
    assert_eq!(files(fixture.root()), before);

    // A mixed tree: 0.6.0 records, then a .chough/session.json written by an older
    // Chough. The next command refuses naming it, and doctor reports it with its fix.
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "session",
        "new",
        "mixed",
        "--objective",
        "SYNTHETIC mixed tree",
    ]);
    fixture.commit("0.6.0 records");
    fixture.ok(&["next", "--json"]);
    fixture.write(".chough/session.json", "{\"schema_version\": 2}\n");
    let before = files(fixture.root());
    let refusal = fixture.fails(&["next"]);
    assert_refused_unmigrated(&refusal, "mixed tree");
    assert!(refusal.contains(".chough/session.json"), "{refusal}");
    assert!(!refusal.contains("more"), "{refusal}");
    assert_eq!(files(fixture.root()), before);
    let output = fixture.run(&["doctor"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "doctor printed no report ({error}): {}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    assert!(
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .any(|finding| finding["fix"] == "chough migrate"),
        "{report:#}"
    );
}

/// t37j's rehearsal on this repository: a real project's receipt lists thousands of
/// files, far past the 65,536-byte bound of an ordinary record. The receipt has its own
/// 4,194,304-byte bound (REQ-records-001), so a receipt of about 100 KB applies whole.
#[test]
fn a_receipt_past_the_ordinary_record_bound_applies() {
    let artifacts: Vec<(String, String)> = (0..400)
        .map(|index| {
            (
                format!(".chough/artifacts/stray/run-{index:04}.log"),
                format!("run {index} passed\n"),
            )
        })
        .collect();
    let extra: Vec<(&str, &str)> = artifacts
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let fixture = migrate_project("adopted", &extra);
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    for (path, _) in &artifacts {
        assert!(plan["sources"].get(path).is_some(), "{path}: {plan:#}");
        assert!(!fixture.exists(path), "{path} was not removed");
    }
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    assert!(
        receipt.len() > 65_536,
        "the receipt is {} bytes",
        receipt.len()
    );
    assert!(receipt.contains(".chough/artifacts/stray/run-0399.log"));
}

/// REQ-records-009, REQ-records-003, TC-records-migrated-inbox-entries-keep-their-ids
/// (U3): each migrated inbox entry keeps its 0.5.x `INB-` ID as an alias, and the plan's
/// and the receipt's ID map send that ID to the new `agent/inbox/` file, never to the
/// removed 0.5.x one.
#[test]
fn migrated_inbox_entries_keep_their_ids_and_the_map_names_the_new_files() {
    let fixture = migrate_project("adopted", &[]);
    let inbox = fixture.root().join(".chough/sessions/open-two.inbox");
    let old: Vec<String> = std::fs::read_dir(&inbox)
        .unwrap()
        .flatten()
        .map(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .trim_end_matches(".json")
                .to_string()
        })
        .filter(|name| name.starts_with("INB-"))
        .collect();
    assert_eq!(old.len(), 2, "{old:?}");
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    for id in &old {
        let home = plan["aliases"][id]
            .as_str()
            .unwrap_or_else(|| panic!("{id} has no alias home: {plan:#}"));
        assert!(
            home.contains("/agent/inbox/") && home.ends_with(".md"),
            "{id} maps to {home}"
        );
        let entry = fixture.read(home);
        assert!(
            entry.contains(&format!("aliases: [{id}]")),
            "{home} does not keep {id}:\n{entry}"
        );
        assert!(
            receipt.contains(&format!("- `{id}` → `{home}`")),
            "the receipt maps {id} elsewhere:\n{receipt}"
        );
    }
}

/// REQ-records-009, TC-records-migrate-plan-says-what-each-file-becomes (U5): the plan
/// names, for every 0.5.x file it carries, the 0.6.0 files it becomes, and the receipt's
/// Removed section names them too, never only "the records above".
#[test]
fn the_plan_and_receipt_say_what_each_carried_file_becomes() {
    let fixture = migrate_project("adopted", &[]);
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    let sources = plan["sources"].as_object().unwrap();
    let becomes = plan["becomes"]
        .as_object()
        .unwrap_or_else(|| panic!("the plan has no becomes: {plan:#}"));
    let removed = plan["removed"].as_object().unwrap();
    let mut unnamed = vec![];
    for source in sources.keys() {
        if removed.contains_key(source) {
            continue;
        }
        let targets = becomes
            .get(source)
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        if targets.is_empty() {
            unnamed.push(source.clone());
        }
        for target in &targets {
            let target = target.as_str().unwrap();
            assert!(
                fixture.exists(target),
                "{source} becomes {target}, which was not written"
            );
        }
    }
    assert!(unnamed.is_empty(), "carried with no target: {unnamed:#?}");
    let session = ".chough/sessions/open-one.json";
    assert!(
        becomes[session]
            .as_array()
            .unwrap()
            .iter()
            .any(|target| target.as_str().unwrap().ends_with("-open-one/session.md")),
        "{:#}",
        becomes[session]
    );
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    assert!(
        !receipt.contains("carried into the 0.6.0 records above"),
        "{receipt}"
    );
}

/// REQ-records-009, TC-records-a-second-migrate-keeps-an-open-bugs-fixture (U6): after
/// the migration, an open bug's kept `.chough/artifacts/` fixture belongs to its 0.6.0
/// Markdown bug, so a second preview on the migrated tree carries and removes nothing,
/// and the fixture stays.
#[test]
fn a_second_migrate_keeps_an_open_bugs_fixture_and_has_nothing_to_remove() {
    let fixture_path = ".chough/artifacts/repro/case.txt";
    let bug = LEGACY_BUG.replace(
        r#""fixture":null"#,
        &format!(r#""fixture":"{fixture_path}""#),
    );
    let extra = [
        (".chough/bugs/BUG-0001.json", bug.as_str()),
        (fixture_path, "SYNTHETIC reproduction input\n"),
    ];
    let fixture = migrate_project("adopted", &extra);
    let (plan, _) = migrate(&fixture, Some(CONSENT));
    assert!(
        plan["kept"].as_object().unwrap().contains_key(fixture_path),
        "{plan:#}"
    );
    fixture.commit("migrated");
    // No 0.5.x record is left, so the preview carries and removes nothing: it says
    // there is nothing to migrate and gives no plan digest.
    let again = fixture.ok(&["migrate", "--json"]);
    assert!(again["nothing_to_do"].is_string(), "{again:#}");
    assert!(again["plan_digest"].is_null(), "{again:#}");
    assert!(fixture.exists(fixture_path));
}

/// REQ-records-010, REQ-records-009, TC-records-the-standing-migration-preference-is-set-before-migrating
/// (U2): on an unmigrated tree, `pref set migration` and `pref unset migration` run and
/// write no committed file other than `.chough/preferences.json`, which the migration keeps
/// byte for byte; the
/// standing preference then gives the apply its consent, and any other key is still
/// refused as unmigrated.
#[test]
fn the_standing_migration_preference_is_set_before_the_first_migration() {
    let fixture = migrate_project("adopted", &[]);
    let refused = fixture.fails(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "SYNTHETIC-human said no librarian (fixture)",
    ]);
    assert!(refused.contains("refused:unmigrated"), "{refused}");
    fixture.ok(&[
        "pref",
        "set",
        "migration",
        "standing",
        "--source",
        "SYNTHETIC-human said migrate whenever it is ready (fixture)",
    ]);
    fixture.ok(&[
        "pref",
        "unset",
        "migration",
        "--source",
        "SYNTHETIC-human withdrew it (fixture)",
    ]);
    fixture.ok(&[
        "pref",
        "set",
        "migration",
        "standing",
        "--source",
        "SYNTHETIC-human said migrate whenever it is ready (fixture)",
    ]);
    let preferences = fixture.read(".chough/preferences.json");
    fixture.commit("the standing migration preference");
    let (_, applied) = migrate(&fixture, None);
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    assert!(
        receipt.contains("SYNTHETIC-human said migrate whenever it is ready"),
        "{receipt}"
    );
    assert_eq!(fixture.read(".chough/preferences.json"), preferences);
}

/// REQ-records-009, REQ-cli-047, TC-records-the-migration-names-the-merge-setup (U7): a
/// migrated project's `.gitattributes` lacks Chough's merge lines, so the plan's and the
/// receipt's follow-ups name each missing line and `chough doctor --fix`, which sets the
/// merge drivers in a clone, as they name the verification exclusions.
#[test]
fn the_migration_follow_ups_name_the_merge_lines_and_drivers() {
    let fixture = migrate_project("adopted", &[]);
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    let follow_ups = plan["follow_ups"].to_string();
    for line in [
        ".chough/bugs/*.md merge=chough-bug",
        "knowledge/index.md merge=chough-generated",
        "knowledge/journal/feed.xml merge=chough-generated",
        "chough doctor --fix",
    ] {
        assert!(
            follow_ups.contains(line),
            "{line} missing from {follow_ups}"
        );
    }
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    assert!(
        receipt.contains("knowledge/journal/feed.xml merge=chough-generated"),
        "{receipt}"
    );
}

/// REQ-records-009, REQ-records-003, TC-records-a-0-5-pointer-still-names-its-session
/// (D1): migrating in place keeps the machine-local active-session pointer, which a
/// 0.5.x project wrote as the session's key; that key is now the session's alias, so
/// `status` reads the session it names and doctor reports no stale pointer.
#[test]
fn a_pointer_naming_a_migrated_key_still_names_its_session() {
    let fixture = migrate_project("adopted", &[]);
    fixture.write(".chough/local/active-session", "open-one\n");
    migrate(&fixture, Some(CONSENT));
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(
        status["objective"], "Open work one",
        "status does not read open-one: {status:#}"
    );
    let output = fixture.run(&["doctor"]);
    let report = String::from_utf8_lossy(&output.stdout);
    assert!(!report.contains("stale_pointer"), "{report}");
}

/// REQ-records-009, TC-records-the-migrate-preview-warns-of-open-changes: while a native
/// change is open, the preview says up front that `--apply` will refuse, naming it, and
/// writes nothing; the apply then refuses with `refused:change-open`. With no open change
/// the preview carries no such warning.
#[test]
fn the_preview_names_open_native_changes_that_make_apply_refuse() {
    let extra = [(
        ".specsync/changes/open-work/state.json",
        "{\"id\": \"open-work\", \"state\": \"draft\"}\n",
    )];
    let fixture = migrate_project("adopted", &extra);
    let before = files(fixture.root());
    let plan = fixture.ok(&["migrate", "--json"]);
    assert_eq!(
        plan["apply_refused"]["refusal"], "refused:change-open",
        "{plan:#}"
    );
    assert_eq!(
        plan["apply_refused"]["changes"],
        serde_json::json!(["open-work"]),
        "{plan:#}"
    );
    assert_eq!(files(fixture.root()), before, "the preview wrote");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let refused = fixture.fails(&["migrate", "--apply", &digest, "--consent", CONSENT]);
    assert!(refused.contains("refused:change-open"), "{refused}");
    std::fs::remove_dir_all(fixture.root().join(".specsync/changes/open-work")).unwrap();
    let plan = fixture.ok(&["migrate", "--json"]);
    assert!(plan["apply_refused"].is_null(), "{plan:#}");
}

/// TC-records-migration-docs-name-every-key (t38j's fresh-eyes report): the Migration
/// section of docs/cli.md names every key the preview and the apply print, so `becomes`
/// and `unstaged` are no longer undocumented.
#[test]
fn the_migration_docs_name_every_key_the_preview_and_apply_print() {
    let fixture = migrate_project("adopted", &[]);
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    let docs =
        std::fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/cli.md")).unwrap();
    let section = docs
        .split("\n## Migration\n")
        .nth(1)
        .expect("a Migration section");
    let section = &section[..section.find("\n## ").unwrap_or(section.len())];
    for (output, value) in [("preview", &plan), ("apply", &applied)] {
        for key in value.as_object().unwrap().keys() {
            if key == "schema_version" {
                continue;
            }
            assert!(
                section.contains(&format!("`{key}`")),
                "the Migration section does not name the {output}'s `{key}`"
            );
        }
    }
}

/// REQ-records-009: the receipt lists everything as before, with `none` under any of its
/// sections that has nothing to list. A migration with no redaction, no kept leak and
/// nothing left uncarried writes `none` under Redactions, Kept with leaks and Not
/// carried, and a section that lists something lists it as before.
#[test]
fn a_receipt_section_with_nothing_to_list_says_none() {
    let fixture = sessions_project("v0.5.0");
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    for key in ["redactions", "kept_with_leaks", "gaps"] {
        let empty = plan[key].as_array().is_some_and(Vec::is_empty)
            || plan[key].as_object().is_some_and(|map| map.is_empty());
        assert!(empty, "{key} is not empty: {plan:#}");
    }
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    let first_line = |heading: &str| {
        receipt
            .split_once(&format!("\n## {heading}\n\n"))
            .unwrap_or_else(|| panic!("no ## {heading} in\n{receipt}"))
            .1
            .lines()
            .next()
            .unwrap_or_default()
            .to_string()
    };
    for heading in ["Redactions", "Kept with leaks", "Not carried"] {
        assert_eq!(first_line(heading), "none", "## {heading} in\n{receipt}");
    }
    assert!(first_line("After").starts_with("- `"), "{receipt}");
}

/// jackdaw's migration (no contract text): every `session.md` that `chough migrate
/// --apply` writes ends with exactly one newline, so `git diff --check` passes on the
/// migration commit. Each open session's Timeline is its last section, and it closed
/// with a blank line: "new blank line at EOF".
#[test]
fn every_migrated_session_md_ends_with_exactly_one_newline() {
    let fixture = migrate_project("adopted", &[]);
    let (plan, _) = migrate(&fixture, Some(CONSENT));
    let sessions: Vec<String> = written(&plan)
        .into_iter()
        .filter(|path| path.ends_with("/session.md"))
        .collect();
    assert!(!sessions.is_empty(), "{plan:#}");
    for path in &sessions {
        let text = fixture.read(path);
        assert!(
            text.ends_with('\n') && !text.ends_with("\n\n"),
            "{path} does not end with exactly one newline:\n{text}"
        );
    }
    fixture.git(&["add", "-A"]);
    let check = std::process::Command::new("git")
        .args(["diff", "--cached", "--check"])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    assert!(
        check.status.success(),
        "git diff --check:\n{}",
        String::from_utf8_lossy(&check.stdout)
    );
}

/// REQ-records-009, F10 of the second fresh-eyes round: on a tree with no 0.5.x record the
/// preview says there is nothing to migrate, gives no plan digest and writes nothing, both
/// on a tree already migrated and on a project that never held a 0.5.x record. The apply's
/// nothing-to-do answer stays.
#[test]
fn a_preview_with_no_0_5_x_record_says_nothing_to_migrate_and_gives_no_digest() {
    let migrated = migrate_project("adopted", &[]);
    let (plan, _) = migrate(&migrated, Some(CONSENT));
    migrated.commit("migrated");
    let never = Fixture::new();
    never.stubs();
    never.init();
    never.commit("a 0.6.0 project");
    for (label, project) in [("migrated", &migrated), ("never 0.5.x", &never)] {
        let before = files(project.root());
        let preview = project.ok(&["migrate", "--json"]);
        assert!(
            files(project.root()) == before,
            "{label}: the preview wrote"
        );
        for key in ["plan_digest", "apply"] {
            assert!(
                preview[key].is_null(),
                "{label}: gives `{key}`: {preview:#}"
            );
        }
        let says = preview["nothing_to_do"]
            .as_str()
            .unwrap_or_else(|| panic!("{label}: no `nothing_to_do`: {preview:#}"));
        assert!(says.contains("nothing to migrate"), "{label}: {says}");
    }
    let applied = migrated.ok(&[
        "migrate",
        "--apply",
        plan["plan_digest"].as_str().unwrap(),
        "--consent",
        CONSENT,
    ]);
    assert_eq!(applied["applied"], false, "{applied:#}");
    assert!(applied["nothing_to_do"].is_string(), "{applied:#}");
}

/// REQ-records-009, F12d of the second fresh-eyes round: a receipt with two follow-ups,
/// the verification exclusions and the merge lines, has one `## Follow-up` heading
/// holding both.
#[test]
fn a_receipt_has_one_follow_up_heading_for_every_follow_up() {
    let policy =
        "{\n  \"version\": 2,\n  \"enabled\": true,\n  \"verification_ignored_paths\": []\n}\n";
    let fixture = migrate_project("adopted", &[(".specsync/sdd.json", policy)]);
    let (plan, applied) = migrate(&fixture, Some(CONSENT));
    let follow_ups: Vec<&str> = plan["follow_ups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|follow_up| follow_up.as_str().unwrap())
        .collect();
    assert_eq!(follow_ups.len(), 2, "{plan:#}");
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    assert_eq!(receipt.matches("\n## Follow-up\n").count(), 1, "{receipt}");
    let section = receipt.split_once("\n## Follow-up\n").unwrap().1;
    for follow_up in follow_ups {
        assert!(
            section.contains(follow_up),
            "{follow_up} missing from\n{section}"
        );
    }
}

/// REQ-records-009, F12b of the third fresh-eyes round: a person consents to the plan, so
/// `chough migrate` prints it as text, naming each 0.5.x file with its SHA-256 and what it
/// becomes, each file it writes, each redaction by file, field and kind, each removal and
/// the digest with the apply command, and writes nothing; `chough migrate --json` prints
/// the same plan and digest as JSON.
#[test]
fn the_preview_prints_the_plan_as_text_and_the_same_plan_as_json() {
    let fixture = migrate_project("adopted", &[]);
    let before = files(fixture.root());
    let output = fixture.run(&["migrate"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(
        serde_json::from_str::<Value>(&text).is_err(),
        "the preview is JSON:\n{text}"
    );
    let plan = fixture.ok(&["migrate", "--json"]);
    assert!(files(fixture.root()) == before, "the preview wrote");
    let digest = plan["plan_digest"].as_str().unwrap();
    assert!(
        text.contains(&format!("chough migrate --apply {digest}")),
        "{text}"
    );
    let mut named: Vec<String> = vec![];
    for (path, sha) in plan["sources"].as_object().unwrap() {
        named.push(format!("`{path}` {}", sha.as_str().unwrap()));
    }
    for targets in plan["becomes"].as_object().unwrap().values() {
        for target in targets.as_array().unwrap() {
            named.push(format!("`{}`", target.as_str().unwrap()));
        }
    }
    for write in written(&plan) {
        named.push(format!("`{write}`"));
    }
    let redactions = plan["redactions"].as_array().unwrap();
    assert!(!redactions.is_empty(), "{plan:#}");
    for redaction in redactions {
        named.push(format!(
            "`{}` {}: {}",
            redaction["file"].as_str().unwrap(),
            redaction["field"].as_str().unwrap(),
            redaction["kind"].as_str().unwrap()
        ));
    }
    let removed = plan["removed"].as_object().unwrap();
    assert!(!removed.is_empty(), "{plan:#}");
    for path in removed.keys() {
        named.push(format!("`{path}`"));
    }
    for part in named {
        assert!(text.contains(&part), "{part} is not named in\n{text}");
    }
}

/// Install a skill bundle as the project's shared copy, `.agents/skills/chough-workflow/`:
/// release `tag`'s files from tests/fixtures/skill-releases, or the current bundle.
fn install_skill(fixture: &Fixture, tag: Option<&str>) {
    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill-releases");
    let manifest: Value =
        serde_json::from_str(&std::fs::read_to_string(fixtures.join("manifest.json")).unwrap())
            .unwrap();
    let files: Vec<(String, Vec<u8>)> = match tag {
        Some(tag) => manifest["releases"][tag]["files"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(file, entry)| {
                let blob = format!("blobs/{}.txt", entry["sha256"].as_str().unwrap());
                (file.clone(), std::fs::read(fixtures.join(blob)).unwrap())
            })
            .collect(),
        None => manifest["current_bundle_sha256"]
            .as_object()
            .unwrap()
            .keys()
            .map(|file| {
                let source = Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("skills/chough-workflow")
                    .join(file);
                (file.clone(), std::fs::read(source).unwrap())
            })
            .collect(),
    };
    assert!(!files.is_empty());
    for (file, bytes) in files {
        let target = fixture
            .root()
            .join(".agents/skills/chough-workflow")
            .join(file);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        std::fs::write(target, bytes).unwrap();
    }
}

/// REQ-records-009, F3 of the fourth fresh-eyes round: a migrated project's installed
/// skill copy is still the 0.5.x one, naming removed commands and paths, so while it is
/// not current, as `chough skill status` reports it, the preview's and the receipt's
/// follow-ups name `chough skill update`; with a current copy they do not.
#[test]
fn the_migration_follow_ups_name_skill_update_while_the_skill_copy_is_not_current() {
    for (tag, named) in [(Some("v0.5.0"), true), (None, false)] {
        let fixture = migrate_project("adopted", &[]);
        install_skill(&fixture, tag);
        fixture.commit("the installed skill copy");
        let status = fixture.ok(&["skill", "status"]);
        let states = status["hosts"].to_string();
        assert_eq!(
            states.contains("known-outdated"),
            named,
            "{tag:?}: {status:#}"
        );
        let (plan, applied) = migrate(&fixture, Some(CONSENT));
        let follow_ups = plan["follow_ups"].to_string();
        assert_eq!(
            follow_ups.contains("chough skill update"),
            named,
            "{tag:?}: {follow_ups}"
        );
        let receipt = fixture.read(applied["receipt"].as_str().unwrap());
        assert_eq!(
            receipt.contains("chough skill update"),
            named,
            "{tag:?}: {receipt}"
        );
    }
}

/// A 0.5.x session record at accept for native change `change`, waiting on `gates`, with
/// no history cursor, as a session an older build left.
fn session_0_5(objective: &str, change: &str, gates: &[&str]) -> String {
    serde_json::to_string_pretty(&serde_json::json!({
        "schema_version": 2,
        "objective": objective,
        "intent_scope": "",
        "modules": [],
        "stage": {"current": "accept", "status": "active", "callers": [], "iteration": 1},
        "native_change": change,
        "references": {"criteria": [], "requirements": [], "cases": [], "design": [], "files": []},
        "decisions": [],
        "open_questions": [],
        "completed": [],
        "observations": [],
        "bugs": [],
        "blockers": [],
        "awaiting_human": gates,
        "run_consents": [],
        "next_action": "Wait for the human's decision.",
    }))
    .unwrap()
        + "\n"
}

/// #246 (REQ-records-009, REQ-cli-055): the preview's Follow-ups named the verification
/// exclusions, the merge lines and the skill update, yet right after the apply
/// `chough doctor` reported `finished_session` and `stale_gates` findings the preview
/// never named, so the one consent round missed the gates' decisions. The preview, and
/// the receipt's `## Follow-up`, name each session doctor reports on the migrated tree:
/// a finished one, and one holding open gates on an archived change with each gate's
/// text and the decision it needs, `chough doctor --fix --reason <text>`.
#[test]
fn the_preview_names_the_sessions_doctor_reports_after_the_apply() {
    let gates = [
        "SYNTHETIC-human's go for the tag and merge",
        "Acceptance review of gated-work",
    ];
    let archive = |change: &str| {
        (
            format!(".specsync/archive/changes/2026-11-01-{change}/finalization.json"),
            format!(
                "{{\"schema_version\": 2, \"change_id\": \"{change}\", \"timestamp\": 500, \"implementation_commit\": \"{}\"}}\n",
                "1".repeat(40)
            ),
        )
    };
    let files = [
        archive("gated-work"),
        archive("finished-work"),
        (
            ".chough/sessions/gated.json".to_string(),
            session_0_5("Gated work", "gated-work", &gates),
        ),
        (
            ".chough/sessions/finished.json".to_string(),
            session_0_5("Finished work", "finished-work", &[]),
        ),
    ];
    let extra: Vec<(&str, &str)> = files
        .iter()
        .map(|(path, text)| (path.as_str(), text.as_str()))
        .collect();
    let fixture = migrate_project("adopted", &extra);
    let (plan, applied) = migrate(&fixture, Some(CONSENT));

    let output = fixture.run(&["doctor"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "doctor: {error}: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    });
    let reported: Vec<(String, String)> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|finding| {
            (
                finding["kind"].as_str().unwrap().to_string(),
                finding["target"].as_str().unwrap().to_string(),
            )
        })
        .filter(|(kind, _)| kind == "finished_session" || kind == "stale_gates")
        .collect();
    // The fixture reproduces the report: one session with stale gates, one finished.
    for kind in ["finished_session", "stale_gates"] {
        assert!(
            reported.iter().any(|(found, _)| found == kind),
            "doctor reports no {kind}: {report:#}"
        );
    }

    let follow_ups = plan["follow_ups"]
        .as_array()
        .unwrap()
        .iter()
        .map(|follow_up| follow_up.as_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n\n");
    let receipt = fixture.read(applied["receipt"].as_str().unwrap());
    let section = receipt
        .split_once("\n## Follow-up\n")
        .map_or("", |(_, section)| section);
    for (place, text) in [("preview", follow_ups.as_str()), ("receipt", section)] {
        for (kind, session) in &reported {
            assert!(
                text.contains(session.as_str()),
                "the {place}'s follow-ups do not name {kind} {session}:\n{text}"
            );
            if kind == "stale_gates" {
                for gate in gates {
                    assert!(
                        text.contains(gate),
                        "the {place} does not name the gate {gate:?} of {session}:\n{text}"
                    );
                }
                assert!(
                    text.contains("chough doctor --fix --reason"),
                    "the {place} names no decision for {session}'s gates:\n{text}"
                );
            }
        }
    }
}

/// REQ-records-015 (#314 review), TC-records-migrate-skips-macos-metadata: the
/// `.DS_Store` and AppleDouble `._*` files macOS writes beside 0.5.x records, in
/// `.chough/` and in the knowledge directory, are not records, so `chough migrate`
/// neither reads, carries nor lists them: the preview gives the same plan digest as
/// without them and that digest applies. A folder the migration emptied of its records
/// goes with the metadata it held and is named; metadata beside what stays stays (#375).
#[test]
fn migrate_skips_macos_metadata_beside_0_5_records() {
    let fixture = migrate_project("adopted", &[(".chough/bugs/BUG-0001.json", LEGACY_BUG)]);
    let digest = fixture.ok(&["migrate", "--json"])["plan_digest"]
        .as_str()
        .unwrap()
        .to_string();
    let apple_double: &[u8] = b"\x00\x05\x16\x07\x00\x02\x00\x00Mac OS X        \x00\x02\xff\xfe";
    let metadata = [
        ".chough/.DS_Store",
        ".chough/bugs/.DS_Store",
        ".chough/bugs/._BUG-0001.json",
        ".chough/sessions/.DS_Store",
        ".chough/sessions/._open-one.json",
        ".chough/sessions/archive/._archived-2.json",
        ".chough/sessions/open-two.inbox/._INB-0590e45a5d1e2245e91e1121.json",
        ".chough/history/.DS_Store",
        ".chough/history/cycles/CYC-5a72dc37cbbdb146702c0e2a/._EVT-ce87b276f18c71b7feab3cfb.json",
        ".chough/evidence/.DS_Store",
        ".chough/evidence/._OBS-1.json",
        "knowledge/.DS_Store",
        "knowledge/._INDEX.md",
        "knowledge/._rules.md",
        "knowledge/sessions/._archived-2.md",
        "knowledge/intent/._widget.md",
        "knowledge/modules/._widget.md",
    ];
    for path in metadata {
        let full = fixture.root().join(path);
        std::fs::create_dir_all(full.parent().unwrap()).unwrap();
        std::fs::write(full, apple_double).unwrap();
    }
    // A metadata folder holding only an empty folder (#321 review P3).
    let metadata_folder = ".chough/history/._metadata-folder/empty-child";
    std::fs::create_dir_all(fixture.root().join(metadata_folder)).unwrap();
    let plan = fixture.ok(&["migrate", "--json"]);
    assert_eq!(plan["plan_digest"], digest.as_str(), "{plan:#}");
    assert_eq!(plan["unaccounted"], serde_json::json!([]), "{plan:#}");
    let applied = fixture.ok(&["migrate", "--apply", &digest, "--consent", CONSENT]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    // #375: a folder the migration emptied of its 0.5.x records, left holding only such
    // files, is removed with them and named in the output with how many it held; such a
    // file beside a record or folder that stays, stays.
    let emptied: Vec<(String, u64)> = applied["emptied_folders"]
        .as_array()
        .unwrap_or_else(|| panic!("no emptied_folders: {applied:#}"))
        .iter()
        .map(|entry| {
            (
                entry["folder"].as_str().unwrap().to_string(),
                entry["metadata_files"].as_u64().unwrap(),
            )
        })
        .collect();
    // `.chough/evidence` held no record in this fixture, so the migration did not empty
    // it and it stays with its metadata.
    assert!(fixture.root().join(".chough/evidence/.DS_Store").is_file());
    for folder in [".chough/history", ".chough/sessions/archive"] {
        assert!(
            !fixture.root().join(folder).exists(),
            "{folder} stayed with only metadata in it"
        );
        assert!(
            emptied.iter().any(|(named, count)| named == folder && *count > 0),
            "{folder} is not named: {emptied:?}"
        );
    }
    for path in [
        ".chough/.DS_Store",
        ".chough/bugs/.DS_Store",
        ".chough/bugs/._BUG-0001.json",
        ".chough/sessions/.DS_Store",
        ".chough/sessions/._open-one.json",
        "knowledge/.DS_Store",
        "knowledge/._rules.md",
    ] {
        assert!(
            fixture.root().join(path).is_file(),
            "{path} beside records that stay was removed"
        );
    }
    for path in metadata {
        let gone_with = emptied
            .iter()
            .any(|(folder, _)| path.starts_with(&format!("{folder}/")));
        assert_eq!(
            fixture.root().join(path).is_file(),
            !gone_with,
            "{path}: removed without its folder being named, or kept in a removed folder"
        );
    }
    // The migrated tree reads with the metadata files still beside its records.
    for args in [&["session", "list"][..], &["bug", "list"][..]] {
        let output = fixture.run(args);
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

/// REQ-records-001 (#265), TC-records-migrate-refuses-conflict-markers: a 0.5.x inbox
/// entry's text becomes the body of its 0.6 entry as written, so an entry whose text
/// holds a Git conflict block would migrate into a record with conflict markers in its
/// body; `migrate --apply` checks every planned file first and refuses as
/// `refused:conflict-markers`, naming the file and the marker, before it writes anything.
#[test]
fn migrate_refuses_a_plan_whose_files_would_hold_conflict_markers() {
    let entry = ".chough/sessions/open-two.inbox/INB-0590e45a5d1e2245e91e1121.json";
    let original = std::fs::read_to_string(fixture_dir("adopted").join(entry)).unwrap();
    let conflicted = original.replace(
        "\"text\": \"Keep the second run?\"",
        "\"text\": \"Keep the second run?\\n<<<<<<< HEAD\\nyes\\n=======\\nno\\n>>>>>>> other\"",
    );
    assert_ne!(conflicted, original);
    let fixture = migrate_project("adopted", &[(entry, &conflicted)]);
    let preview = fixture.ok(&["migrate", "--json"]);
    let digest = preview["plan_digest"].as_str().unwrap().to_string();
    // A plan the apply would refuse is not offered to apply, in JSON either.
    assert!(preview.get("apply").is_none(), "{preview:#}");
    // #339: the preview says up front that the apply refuses, naming the 0.5.x source.
    assert_eq!(
        preview["apply_refused"]["refusal"], "refused:conflict-markers",
        "{preview:#}"
    );
    assert!(
        preview["apply_refused"]["why"]
            .as_str()
            .is_some_and(|why| why.starts_with(&format!("refused:conflict-markers: {entry} holds"))),
        "{preview:#}"
    );
    let text = String::from_utf8_lossy(&fixture.run(&["migrate"]).stdout).into_owned();
    assert!(
        text.contains("The apply would refuse now (refused:conflict-markers)") && text.contains(entry),
        "{text}"
    );
    // A plan the apply would refuse is not offered to apply.
    assert!(text.contains("Plan digest:"), "{text}");
    assert!(!text.contains("Apply this plan"), "{text}");
    let before = files(fixture.root());
    let refused = fixture.fails(&["migrate", "--apply", &digest, "--consent", CONSENT]);
    assert!(refused.contains("refused:conflict-markers"), "{refused}");
    // The refusal names the 0.5.x file to resolve, then the record it would become.
    assert!(
        refused.contains(&format!("{entry} holds a Git conflict marker")),
        "{refused}"
    );
    assert!(refused.contains("/agent/inbox/"), "{refused}");
    assert!(refused.contains("`<<<<<<<`"), "{refused}");
    assert!(!refused.contains("<<<<<<< HEAD"), "{refused}");
    assert_eq!(
        files(fixture.root()),
        before,
        "the refused apply wrote a file"
    );
}

/// REQ-records-009 (#264), TC-records-a-second-same-day-migration-names-its-own-receipt:
/// a 0.5.x record written into an already migrated tree on the same day is migrated by
/// a second plan, which names its receipt `<date>-from-0-5-2.md` in the preview, as
/// part of the plan and its digest. The apply writes that receipt, leaves the first
/// receipt byte for byte, removes the stray record, and writes nothing before it has
/// checked every target.
#[test]
fn a_second_same_day_migration_names_its_own_receipt() {
    let fixture = migrate_project("adopted", &[]);
    let (first_plan, first) = migrate(&fixture, Some(CONSENT));
    let first_receipt = first["receipt"].as_str().unwrap().to_string();
    assert!(first_receipt.ends_with("-from-0-5.md"), "{first_receipt}");
    assert_eq!(first_plan["receipt"], first["receipt"], "{first_plan:#}");
    fixture.commit("migrated");
    let first_bytes = fixture.read(&first_receipt);

    // A 0.5.x binary writes one more record into the migrated tree.
    fixture.write(
        ".chough/sessions/stray-work.json",
        &session_0_5("Stray work", "", &[]),
    );
    fixture.commit("a stray 0.5.x record");
    let plan = fixture.ok(&["migrate", "--json"]);
    let second_receipt = first_receipt.replace("-from-0-5.md", "-from-0-5-2.md");
    assert_eq!(plan["receipt"], second_receipt.as_str(), "{plan:#}");
    let text = fixture.ok_text(&["migrate"]);
    assert!(text.contains(&second_receipt), "{text}");

    let digest = plan["plan_digest"].as_str().unwrap();
    let applied = fixture.ok(&["migrate", "--apply", digest, "--consent", CONSENT]);
    assert_eq!(applied["receipt"], second_receipt.as_str(), "{applied:#}");
    assert_eq!(fixture.read(&first_receipt), first_bytes);
    assert!(
        fixture
            .read(&second_receipt)
            .contains(&format!("plan_digest: {digest}"))
    );
    assert!(!fixture.exists(".chough/sessions/stray-work.json"));
    let again = fixture.ok(&["migrate", "--json"]);
    assert!(again["nothing_to_do"].is_string(), "{again:#}");
}

/// REQ-records-009 (#264): the apply checks every target, the receipt included, before
/// it writes anything. A receipt for another plan that takes the previewed name after
/// the preview makes the previewed digest stale, so the apply refuses with nothing
/// written; a fresh preview then names the next receipt and applies.
#[test]
fn a_receipt_name_taken_after_the_preview_refuses_before_any_write() {
    let fixture = migrate_project("adopted", &[]);
    let plan = fixture.ok(&["migrate", "--json"]);
    let receipt = plan["receipt"].as_str().unwrap().to_string();
    fixture.write(
        &receipt,
        "---\nkind: receipt\nplan_digest: SYNTHETIC-another-plan\n---\n\n# Migration from Chough 0.5.x\n\n## Before\n\nnone\n",
    );
    let before = files(fixture.root());
    let refused = fixture.fails(&[
        "migrate",
        "--apply",
        plan["plan_digest"].as_str().unwrap(),
        "--consent",
        CONSENT,
    ]);
    assert!(refused.contains("refused:plan-digest"), "{refused}");
    assert_eq!(
        files(fixture.root()),
        before,
        "the refused apply changed a file"
    );
    let (next_plan, applied) = migrate(&fixture, Some(CONSENT));
    let second = receipt.replace("-from-0-5.md", "-from-0-5-2.md");
    assert_eq!(next_plan["receipt"], second.as_str(), "{next_plan:#}");
    assert_eq!(applied["receipt"], second.as_str(), "{applied:#}");
    assert!(
        fixture
            .read(&receipt)
            .contains("plan_digest: SYNTHETIC-another-plan")
    );
}

/// The adopted fixture, migrated with consent and committed, with the receipt's path.
fn migrated_adopted() -> (Fixture, String) {
    let fixture = migrate_project("adopted", &[]);
    let (_, applied) = migrate(&fixture, Some(CONSENT));
    fixture.commit("migrated");
    let receipt = applied["receipt"].as_str().unwrap().to_string();
    (fixture, receipt)
}

/// The 0.5.x file at `path` of the adopted fixture, as it was before the migration.
fn adopted_bytes(path: &str) -> String {
    std::fs::read_to_string(fixture_dir("adopted").join(path)).unwrap()
}

/// A preview that refuses: non-zero, no plan digest on standard output, and its refusal.
fn refused_preview(fixture: &Fixture) -> String {
    let output = fixture.run(&["migrate"]);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    assert!(!output.status.success(), "the preview did not refuse: {stdout}");
    assert!(!stdout.contains("Plan digest"), "a refused preview gave a plan: {stdout}");
    String::from_utf8_lossy(&output.stderr).into_owned()
}

/// An apply of an arbitrary digest that refuses, with its refusal.
fn refused_apply(fixture: &Fixture) -> String {
    fixture.fails(&["migrate", "--apply", &"0".repeat(64), "--consent", CONSENT])
}

const OPEN_ONE: &str = ".chough/sessions/open-one.json";
const EVENT: &str =
    ".chough/history/cycles/CYC-35b47ded993bd490850611ba/EVT-4edf61033f58689200e93314.json";

/// #370 (REQ-records-009), TC-records-migrate-judges-returning-files-by-the-receipt: on a
/// migrated tree, a 0.5.x open session file that comes back with an edited objective
/// makes the preview and the apply refuse, naming the file, the receipt that lists it and
/// the way on, with nothing written and no second session; once its removal is committed
/// there is nothing to migrate.
#[test]
fn a_returning_edited_session_is_refused_naming_it_and_its_receipt() {
    let (fixture, receipt) = migrated_adopted();
    let sessions_before = fixture.open_ids();
    fixture.write(
        OPEN_ONE,
        &adopted_bytes(OPEN_ONE).replace("Open work one", "Open work one, edited on a branch"),
    );
    fixture.commit("a 0.5.x session returns, edited");
    let before = files(fixture.root());
    for refusal in [refused_preview(&fixture), refused_apply(&fixture)] {
        assert!(
            refusal.contains("refused:already-migrated")
                && refusal.contains(OPEN_ONE)
                && refusal.contains(&receipt)
                && refusal.contains("by hand")
                && refusal.contains("`chough migrate` again"),
            "{refusal}"
        );
        assert!(!refusal.contains("appeared"), "{refusal}");
    }
    assert_eq!(files(fixture.root()), before, "a refused migrate changed a file");
    assert_eq!(fixture.open_ids(), sessions_before);
    fixture.git(&["rm", "-q", OPEN_ONE]);
    fixture.commit("the returning file removed");
    let text = fixture.ok_text(&["migrate"]);
    assert!(text.contains("Nothing to migrate"), "{text}");
}

/// #370: a returning history event with other bytes is refused the same way, not planned
/// as removed; two returning files are named in one refusal.
#[test]
fn returning_files_with_other_bytes_are_all_named_in_one_refusal() {
    let (fixture, receipt) = migrated_adopted();
    fixture.write(EVENT, &format!("{} ", adopted_bytes(EVENT)));
    fixture.commit("an event returns, edited");
    let refusal = refused_preview(&fixture);
    assert!(
        refusal.contains("refused:already-migrated")
            && refusal.contains(EVENT)
            && refusal.contains(&receipt),
        "{refusal}"
    );
    fixture.write(
        OPEN_ONE,
        &adopted_bytes(OPEN_ONE).replace("Open work one", "Open work one, edited"),
    );
    fixture.commit("a session returns too");
    let refusal = refused_preview(&fixture);
    assert!(refusal.contains(EVENT) && refusal.contains(OPEN_ONE), "{refusal}");
    assert_eq!(refusal.matches("refused:").count(), 1, "{refusal}");
}

/// #370: a returning 0.5.x file with the bytes its receipt lists is planned as removed as
/// already migrated, naming the receipt, and the apply removes it without carrying it
/// again, so no second session is made.
#[test]
fn a_returning_file_with_its_receipt_bytes_is_removed_as_already_migrated() {
    let (fixture, receipt) = migrated_adopted();
    let sessions_before = fixture.open_ids();
    fixture.write(OPEN_ONE, &adopted_bytes(OPEN_ONE));
    fixture.commit("a 0.5.x session returns unchanged");
    let plan = fixture.ok(&["migrate", "--json"]);
    let why = plan["removed"][OPEN_ONE].as_str().unwrap_or_default();
    assert!(
        why.contains("already migrated") && why.contains(&receipt),
        "{plan:#}"
    );
    let sessions_written: Vec<String> = written(&plan)
        .into_iter()
        .filter(|path| path.starts_with(".chough/sessions/") || path.starts_with(".chough/archive/sessions/"))
        .collect();
    assert!(sessions_written.is_empty(), "{sessions_written:?}");
    let digest = plan["plan_digest"].as_str().unwrap();
    let applied = fixture.ok(&["migrate", "--apply", digest, "--consent", CONSENT]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    assert!(!fixture.exists(OPEN_ONE));
    assert_eq!(fixture.open_ids(), sessions_before);
}

/// #370: a committed receipt changed on disk and not committed makes the preview and the
/// apply refuse while a 0.5.x file is present, naming it and saying to commit it or
/// restore it; once restored, the migration judges by it again.
#[test]
fn an_edited_committed_receipt_is_refused_until_committed_or_restored() {
    let (fixture, receipt) = migrated_adopted();
    fixture.write(OPEN_ONE, &adopted_bytes(OPEN_ONE));
    fixture.commit("a 0.5.x session returns unchanged");
    let committed = fixture.read(&receipt);
    fixture.write(&receipt, &format!("{committed}\nA hand edit.\n"));
    let before = files(fixture.root());
    for refusal in [refused_preview(&fixture), refused_apply(&fixture)] {
        assert!(
            refusal.contains("refused:migration-receipt")
                && refusal.contains(&receipt)
                && refusal.contains("commit")
                && refusal.contains("restore"),
            "{refusal}"
        );
    }
    assert_eq!(files(fixture.root()), before);
    fixture.git(&["checkout", "--", &receipt]);
    let plan = fixture.ok(&["migrate", "--json"]);
    assert!(
        plan["removed"][OPEN_ONE]
            .as_str()
            .is_some_and(|why| why.contains("already migrated")),
        "{plan:#}"
    );
}

/// #370 (guard): a receipt Git has never held is this migration's own, left by an apply
/// stopped once it was written; the preview plans nothing as already migrated and the
/// next apply completes as an uninterrupted one. Once that receipt is committed with the
/// 0.5.x files it lists still there, they are planned as removed as already migrated.
#[test]
fn an_uncommitted_receipt_judges_nothing_and_a_committed_one_does() {
    let done = migrate_project("adopted", &[]);
    let (_, applied) = migrate(&done, Some(CONSENT));
    let receipt = applied["receipt"].as_str().unwrap().to_string();
    let fixture = migrate_project("adopted", &[]);
    fixture.write(&receipt, &done.read(&receipt));
    let plan = fixture.ok(&["migrate", "--json"]);
    assert!(
        !plan.to_string().contains("already migrated"),
        "an uncommitted receipt judged a file: {plan:#}"
    );
    fixture.commit("the receipt committed with the 0.5.x files");
    let plan = fixture.ok(&["migrate", "--json"]);
    assert!(
        plan["removed"][OPEN_ONE]
            .as_str()
            .is_some_and(|why| why.contains("already migrated")),
        "{plan:#}"
    );
}

/// #337 (REQ-records-009): an entry named `*.md` in `.chough/archive/migrations/` that is
/// not a receipt Chough can read makes the preview refuse without waiting, naming it and
/// saying to move or remove it, and the apply refuses naming it, with nothing written and
/// no "appeared while migrating". A `.DS_Store`, a `._` file and a `.gitkeep` there change
/// neither the plan nor its digest.
#[test]
fn an_entry_in_the_receipt_folder_that_is_no_receipt_is_refused_without_waiting() {
    let name = ".chough/archive/migrations/2026-01-01-from-0-5.md";
    let kinds: [(&str, fn(&Path)); 5] = [
        ("a directory", |path| std::fs::create_dir_all(path).unwrap()),
        ("unreadable", |path| {
            std::fs::write(path, "# receipt\n").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(path, std::fs::Permissions::from_mode(0)).unwrap();
            }
        }),
        ("not UTF-8", |path| std::fs::write(path, b"# receipt \xff\xfe\n").unwrap()),
        ("a FIFO", |path| {
            #[cfg(unix)]
            assert!(
                std::process::Command::new("mkfifo")
                    .arg(path)
                    .status()
                    .unwrap()
                    .success()
            );
        }),
        ("no list of files read", |path| {
            std::fs::write(path, "# Migration receipt\n\nNo list here.\n").unwrap()
        }),
    ];
    for (kind, place) in kinds {
        let fixture = migrate_project("adopted", &[]);
        let path = fixture.root().join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        place(&path);
        let started = std::time::Instant::now();
        let preview = refused_preview(&fixture);
        assert!(started.elapsed().as_secs() < 10, "{kind}: the preview waited");
        let apply = refused_apply(&fixture);
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if kind == "unreadable" {
                std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o644)).unwrap();
            }
        }
        for refusal in [&preview, &apply] {
            assert!(
                refusal.contains("refused:migration-receipt")
                    && refusal.contains(name)
                    && refusal.contains("move or remove it")
                    && !refusal.contains("appeared"),
                "{kind}: {refusal}"
            );
        }
        assert!(preview.contains("the apply would refuse"), "{kind}: {preview}");
        assert!(fixture.exists(OPEN_ONE), "{kind}: a 0.5.x file went");
        assert!(fixture.open_ids().is_empty(), "{kind}: a 0.6.0 session was written");
    }
    let plain = migrate_project("adopted", &[]);
    let with_metadata = migrate_project("adopted", &[]);
    for metadata in [".DS_Store", "._2026-01-01-from-0-5.md", ".gitkeep"] {
        with_metadata.write(&format!(".chough/archive/migrations/{metadata}"), "x");
    }
    let plain = plain.ok(&["migrate", "--json"]);
    let with = with_metadata.ok(&["migrate", "--json"]);
    assert_eq!(plain["plan_digest"], with["plan_digest"], "{with:#}");
    migrate(&with_metadata, Some(CONSENT));
}

/// #337 (REQ-records-009), TC-records-migrate-a-special-target-refuses-before-writing:
/// with a directory, a file without read permission or a FIFO at `knowledge/index.md`,
/// the preview returns without waiting and says the apply would refuse, and the apply
/// refuses naming the project-relative path, with every 0.5.x file still there and no
/// 0.6.0 file or receipt written.
#[test]
fn a_directory_unreadable_file_or_fifo_at_a_target_is_refused_without_waiting() {
    let kinds: [(&str, fn(&Path)); 3] = [
        ("a directory", |path| std::fs::create_dir_all(path).unwrap()),
        ("unreadable", |path| {
            std::fs::write(path, "# x\n").unwrap();
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(path, std::fs::Permissions::from_mode(0)).unwrap();
            }
        }),
        ("a FIFO", |path| {
            #[cfg(unix)]
            assert!(
                std::process::Command::new("mkfifo")
                    .arg(path)
                    .status()
                    .unwrap()
                    .success()
            );
        }),
    ];
    for (kind, place) in kinds {
        let fixture = migrate_project("adopted", &[]);
        // On a file system that folds case INDEX.md is index.md, so the 0.5.x index goes
        // first, and the entry is placed at the target itself.
        fixture.git(&["rm", "-q", "knowledge/INDEX.md"]);
        fixture.commit("no 0.5.x index");
        let plan = fixture.ok(&["migrate", "--json"]);
        planned(&plan, "knowledge/index.md");
        let target = fixture.root().join("knowledge/index.md");
        place(&target);
        let before = files(fixture.root());
        let started = std::time::Instant::now();
        let preview = fixture.run(&["migrate"]);
        assert!(started.elapsed().as_secs() < 10, "{kind}: the preview waited");
        let text = String::from_utf8_lossy(&preview.stdout).into_owned();
        assert!(
            text.contains("would refuse") && text.contains("knowledge/index.md"),
            "{kind}: {text}"
        );
        let digest = fixture.ok(&["migrate", "--json"])["plan_digest"]
            .as_str()
            .unwrap()
            .to_string();
        let refused = fixture.fails(&["migrate", "--apply", &digest, "--consent", CONSENT]);
        let after = files(fixture.root());
        #[cfg(unix)]
        if kind == "unreadable" {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&target, std::fs::Permissions::from_mode(0o644)).unwrap();
        }
        assert!(
            refused.contains("refused:target-exists")
                && refused.contains("knowledge/index.md")
                && !refused.contains(&fixture.root().display().to_string())
                && !refused.contains("appeared"),
            "{kind}: {refused}"
        );
        assert_eq!(after, before, "{kind}: the refused apply changed a file");
    }
}

/// #378 (REQ-records-009, REQ-cli-040), TC-records-migrate-one-of-several-applies: three
/// applies of one digest started together leave one receipt holding one consent; each of
/// the other two refuses or reports that nothing is left to migrate, and none of them
/// reports `applied: true`.
#[test]
fn of_three_applies_of_one_digest_only_one_applies() {
    for round in 0..5 {
        let fixture = migrate_project("adopted", &[]);
        let digest = fixture.ok(&["migrate", "--json"])["plan_digest"]
            .as_str()
            .unwrap()
            .to_string();
        let barrier = std::sync::Arc::new(std::sync::Barrier::new(3));
        let root = fixture.root().to_path_buf();
        let outputs: Vec<std::process::Output> = (0..3)
            .map(|agent| {
                let barrier = barrier.clone();
                let root = root.clone();
                let digest = digest.clone();
                std::thread::spawn(move || {
                    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_chough"));
                    command.arg("--root").arg(&root).args([
                        "migrate",
                        "--apply",
                        &digest,
                        "--consent",
                        &format!("SYNTHETIC agent {agent} consent"),
                    ]);
                    barrier.wait();
                    command.output().unwrap()
                })
            })
            .collect::<Vec<_>>()
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();
        let applied = outputs
            .iter()
            .filter(|output| String::from_utf8_lossy(&output.stdout).contains("\"applied\": true"))
            .count();
        assert_eq!(applied, 1, "round {round}: {outputs:#?}");
        let receipts: Vec<String> = std::fs::read_dir(root.join(".chough/archive/migrations"))
            .unwrap()
            .flatten()
            .map(|entry| entry.file_name().to_string_lossy().into_owned())
            .filter(|name| name.ends_with(".md"))
            .collect();
        assert_eq!(receipts.len(), 1, "round {round}: {receipts:?}");
        let receipt = fixture.read(&format!(".chough/archive/migrations/{}", receipts[0]));
        assert_eq!(
            receipt.matches("SYNTHETIC agent").count(),
            1,
            "round {round}: {receipt}"
        );
    }
}

/// #370: on a migrated tree, a 0.5.x bug the tree never held migrates as a new record,
/// and `knowledge/index.md`, a regular file with other bytes now, is listed under
/// `replaces` and rebuilt in place, spelled so.
#[test]
fn a_new_0_5_x_bug_on_a_migrated_tree_migrates_with_the_index_rebuilt_in_place() {
    let (fixture, _) = migrated_adopted();
    fixture.write(".chough/bugs/BUG-0001.json", LEGACY_BUG);
    fixture.commit("a new 0.5.x bug from an old branch");
    let plan = fixture.ok(&["migrate", "--json"]);
    assert_eq!(plan["replaces"], serde_json::json!(["knowledge/index.md"]), "{plan:#}");
    let bug = written(&plan)
        .into_iter()
        .find(|path| path.starts_with(".chough/bugs/") && path.ends_with(".md"))
        .unwrap_or_else(|| panic!("no bug planned: {plan:#}"));
    let index = planned(&plan, "knowledge/index.md");
    let digest = plan["plan_digest"].as_str().unwrap();
    let applied = fixture.ok(&["migrate", "--apply", digest, "--consent", CONSENT]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    assert!(fixture.exists(&bug));
    let written = sha256(fixture.read(&index).as_bytes());
    let wanted = plan["writes"]
        .as_array()
        .unwrap()
        .iter()
        .find(|write| write["path"] == index.as_str())
        .unwrap()["sha256"]
        .clone();
    assert_eq!(serde_json::json!(written), wanted);
    let names: Vec<String> = std::fs::read_dir(fixture.root().join("knowledge"))
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert!(names.contains(&"index.md".to_string()), "{names:?}");
}

/// #370 (REQ-records-003): a 0.5.x record at another path whose ID a 0.6.0 record
/// already holds as an alias makes the preview and the apply refuse, naming that record,
/// so no second record takes the alias.
#[test]
fn a_0_5_x_record_whose_id_a_record_holds_as_an_alias_is_refused() {
    let fixture = migrate_project("adopted", &[(".chough/bugs/BUG-0001.json", LEGACY_BUG)]);
    let (plan, _) = migrate(&fixture, Some(CONSENT));
    fixture.commit("migrated");
    let holder = written(&plan)
        .into_iter()
        .find(|path| path.starts_with(".chough/bugs/") && path.ends_with(".md"))
        .unwrap_or_else(|| panic!("no bug planned: {plan:#}"));
    fixture.write(".chough/bugs/BUG-a-copy-under-another-name.json", LEGACY_BUG);
    fixture.commit("the same bug ID at another path");
    let before = files(fixture.root());
    for refusal in [refused_preview(&fixture), refused_apply(&fixture)] {
        assert!(
            refusal.contains("refused:alias-held")
                && refusal.contains("BUG-0001")
                && refusal.contains(&holder),
            "{refusal}"
        );
    }
    assert_eq!(files(fixture.root()), before);
}

/// #370: a `.chough/session.json` that returns holding a different session than the one
/// migrated is refused with the way on naming a new session, and no second session is
/// created.
#[test]
fn a_returning_legacy_checkpoint_holding_another_session_is_refused() {
    let fixture = sessions_project("v0.3.0");
    let plan = fixture.ok(&["migrate", "--json"]);
    let digest = plan["plan_digest"].as_str().unwrap();
    let applied = fixture.ok(&["migrate", "--apply", digest, "--consent", CONSENT]);
    let receipt = applied["receipt"].as_str().unwrap().to_string();
    fixture.commit("migrated");
    let sessions = fixture.open_ids();
    let other = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/releases/v0.2.0/.chough/session.json"),
    )
    .unwrap();
    fixture.write(".chough/session.json", &other);
    fixture.commit("session.json returns holding another session");
    for refusal in [refused_preview(&fixture), refused_apply(&fixture)] {
        assert!(
            refusal.contains("refused:already-migrated")
                && refusal.contains(".chough/session.json")
                && refusal.contains(&receipt)
                && refusal.contains("chough session new"),
            "{refusal}"
        );
    }
    assert_eq!(fixture.open_ids(), sessions);
}

/// #338 (REQ-records-009): an apply whose planned write fails leaves every 0.5.x file in
/// place, since no source is removed before every planned file is on disk.
#[cfg(unix)]
#[test]
fn an_apply_whose_planned_write_fails_leaves_every_0_5_x_file() {
    use std::os::unix::fs::PermissionsExt;
    let fixture = migrate_project("adopted", &[]);
    let before: Vec<String> = files(fixture.root())
        .into_keys()
        .filter(|path| path.starts_with(".chough/") || path.starts_with("knowledge/"))
        .filter(|path| !path.starts_with(".chough/local/"))
        .collect();
    let plan = fixture.ok(&["migrate", "--json"]);
    let digest = plan["plan_digest"].as_str().unwrap();
    // The last folder a planned file goes into cannot be written.
    let blocked = fixture.root().join("knowledge");
    std::fs::set_permissions(&blocked, std::fs::Permissions::from_mode(0o555)).unwrap();
    let output = fixture.run(&["migrate", "--apply", digest, "--consent", CONSENT]);
    std::fs::set_permissions(&blocked, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(!output.status.success(), "the apply succeeded with knowledge/ read-only");
    for path in before {
        assert!(fixture.exists(&path), "{path} was removed although a planned write failed");
    }
}
