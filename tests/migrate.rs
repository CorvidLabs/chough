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
    let plan = fixture.ok(&["migrate"]);
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
    fixture.ok(&["stage", "advance"]);
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
    let plan = fixture.ok(&["migrate"]);
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
    let first = stopped.ok(&["migrate"]);
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
    let again = stopped.ok(&["migrate"]);
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
    let plan = fixture.ok(&["migrate"]);
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
    let plan = fixture.ok(&["migrate"]);
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
        &["stage", "advance"],
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
    let plan = fixture.ok(&["migrate"]);
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
    let again = fixture.ok(&["migrate"]);
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
    let plan = fixture.ok(&["migrate"]);
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
    let plan = fixture.ok(&["migrate"]);
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
        let preview = project.ok(&["migrate"]);
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
