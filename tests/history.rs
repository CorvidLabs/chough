mod common;

use common::Fixture;
use serde_json::Value;
use std::process::Command;

fn init(f: &Fixture) {
    f.ok(&[
        "init",
        "--objective",
        "History regression",
        "--module",
        "cli",
    ]);
}

fn git(f: &Fixture, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .current_dir(f.root())
        .args([
            "-c",
            "user.name=History test",
            "-c",
            "user.email=history@example.invalid",
        ])
        .args(args)
        .output()
        .unwrap()
}

fn git_ok(f: &Fixture, args: &[&str]) {
    let r = git(f, args);
    assert!(
        r.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&r.stderr)
    );
}

#[test]
fn new_projects_use_a_cursor_in_the_single_checkpoint() {
    let f = Fixture::new();
    init(&f);
    assert!(f.session()["history_cursor"].is_object());
    assert!(!f.exists(".chough/transitions.jsonl"));
}

#[test]
fn adoption_preview_preserves_legacy_bytes_and_apply_is_explicit() {
    let f = Fixture::new();
    init(&f);
    let mut session = f.session();
    session.as_object_mut().unwrap().remove("history_cursor");
    f.write(".chough/session.json", &session.to_string());
    let log = "{\"schema_version\":1,\"at_unix_seconds\":100,\"kind\":\"reset\",\"from\":\"done\",\"to\":\"intent\",\"refresh\":[]}\n";
    f.write(".chough/transitions.jsonl", log);
    f.write(".chough/session.v1.json", "original migration bytes\n");
    let before = f.read(".chough/session.json");
    let report = f.ok(&["history", "adopt"]);
    assert_eq!(report["applied"], false);
    assert_eq!(f.read(".chough/session.json"), before);
    f.ok(&["history", "adopt", "--apply"]);
    assert!(f.session()["history_cursor"].is_object());
    assert_eq!(f.read(".chough/transitions.jsonl"), log);
    assert_eq!(
        f.read(".chough/session.v1.json"),
        "original migration bytes\n"
    );
    let adopted = f.read(".chough/session.json");
    f.ok(&["history", "adopt", "--apply"]);
    assert_eq!(f.read(".chough/session.json"), adopted);
}

#[test]
fn real_git_branch_events_survive_while_checkpoint_selection_remains_explicit() {
    let f = Fixture::new();
    init(&f);
    git_ok(&f, &["init"]);
    git_ok(&f, &["checkout", "-b", "base"]);
    git_ok(&f, &["add", "."]);
    git_ok(&f, &["commit", "-m", "baseline"]);
    git_ok(&f, &["checkout", "-b", "left"]);
    f.ok(&["stage", "iterate", "--note", "left branch event"]);
    set_latest_time(&f, 1_700_000_000);
    let left: Value = f.session();
    git_ok(&f, &["add", ".chough"]);
    git_ok(&f, &["commit", "-m", "left"]);
    git_ok(&f, &["checkout", "-b", "right", "base"]);
    f.ok(&["stage", "iterate", "--note", "right branch event"]);
    set_latest_time(&f, 1_700_000_000);
    git_ok(&f, &["add", ".chough"]);
    git_ok(&f, &["commit", "-m", "right"]);
    git_ok(&f, &["checkout", "left"]);
    let _ = git(&f, &["merge", "right", "--no-edit"]);
    let conflicts = git(&f, &["diff", "--name-only", "--diff-filter=U"]);
    assert_eq!(
        String::from_utf8(conflicts.stdout).unwrap().trim(),
        ".chough/session.json",
        "immutable event paths must merge; only the current-cycle checkpoint needs selection"
    );
    git_ok(&f, &["checkout", "--ours", ".chough/session.json"]);
    git_ok(&f, &["add", ".chough/session.json"]);
    git_ok(&f, &["commit", "-m", "select coherent left checkpoint"]);
    assert_eq!(f.session()["history_cursor"], left["history_cursor"]);
    let history = f.ok(&["history", "show"]);
    assert!(history["cycles"].as_array().unwrap().iter().any(|cycle| {
        cycle["unselected_tips"]
            .as_array()
            .is_some_and(|tips| !tips.is_empty())
    }));
}

fn event_path(f: &Fixture) -> String {
    let cursor = f.session()["history_cursor"].clone();
    format!(
        ".chough/history/cycles/{}/{}.json",
        cursor["cycle"].as_str().unwrap(),
        cursor["head"].as_str().unwrap()
    )
}

#[test]
fn changing_the_legacy_anchor_refuses_writes_without_advancing_state() {
    let f = Fixture::new();
    f.init();
    f.write(".chough/transitions.jsonl", "");
    f.ok(&["history", "adopt", "--apply"]);
    f.write(".chough/transitions.jsonl", "changed legacy proof\n");
    let checkpoint = f.read(".chough/session.json");
    assert!(
        f.fails(&["stage", "iterate", "--note", "must refuse"])
            .contains("anchor changed")
    );
    assert_eq!(f.read(".chough/session.json"), checkpoint);
    assert_eq!(
        f.read(".chough/transitions.jsonl"),
        "changed legacy proof\n"
    );
}

#[test]
fn missing_parents_are_explicit_in_guidance_and_timing() {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "first"]);
    let parent = event_path(&f);
    f.ok(&["stage", "iterate", "--note", "second"]);
    std::fs::remove_file(f.root().join(parent)).unwrap();
    let status = f.ok(&["status"]);
    assert!(
        !status["transition_history"]["partial"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let next = f.ok(&["next", "--json"]);
    assert!(next.to_string().contains("History partial or unknown"));
    let timing = f.ok(&["timing", "--cycle", "current", "--json"]);
    assert!(
        timing["partial"]
            .as_array()
            .unwrap()
            .iter()
            .any(|reason| reason.to_string().contains("unavailable history"))
    );
    let before = f.read(".chough/session.json");
    assert!(
        f.fails(&["stage", "iterate", "--note", "must refuse"])
            .contains("partial history")
    );
    assert_eq!(f.read(".chough/session.json"), before);
}

fn chain(f: &Fixture, count: usize, padded_size: Option<usize>) {
    let mut session = f.session();
    let cycle = session["history_cursor"]["cycle"]
        .as_str()
        .unwrap()
        .to_string();
    let mut parent: Option<String> = None;
    for index in 0..count {
        let id = format!("EVT-{index:024x}");
        let record = serde_json::json!({"schema_version":1,"cycle":cycle,"id":id,"parent":parent,"previous_cycle":null,"native_change":null,"transition":{"schema_version":1,"at_unix_seconds":100+index,"kind":"iterate","from":"intent","to":"intent","reason":"bounded fixture","gate":null,"revision":null,"refresh":[]}});
        let mut text = record.to_string();
        if let Some(size) = padded_size {
            assert!(text.len() <= size);
            text.extend(std::iter::repeat_n(' ', size - text.len()));
        }
        f.write(&format!(".chough/history/cycles/{cycle}/{id}.json"), &text);
        parent = Some(id);
    }
    session["history_cursor"]["head"] = serde_json::to_value(parent).unwrap();
    f.write(".chough/session.json", &session.to_string());
}

fn event_and_cycle_byte_limits_refuse_without_mutating_the_checkpoint() {
    let f = Fixture::new();
    init(&f);
    chain(&f, 1, Some(65_536));
    assert_eq!(
        f.ok(&["stage", "show"])["recent_transitions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let path = event_path(&f);
    f.write(&path, &(f.read(&path) + " "));
    let before = f.read(".chough/session.json");
    assert!(
        f.fails(&["stage", "iterate", "--note", "oversize"])
            .contains("partial history")
    );
    assert_eq!(f.read(".chough/session.json"), before);

    let f = Fixture::new();
    init(&f);
    chain(&f, 256, Some(65_536));
    let status = f.ok(&["status"]);
    assert!(status["transition_history"]["bytes_read"].as_u64().unwrap() <= 512 * 1024);
    let before = f.read(".chough/session.json");
    assert!(
        f.fails(&["stage", "iterate", "--note", "full byte cycle"])
            .contains("capacity reached")
    );
    assert_eq!(f.read(".chough/session.json"), before);
    f.ok(&[
        "stage",
        "reset",
        "--to",
        "intent",
        "--reason",
        "bounded new cycle",
    ]);
    assert_ne!(f.read(".chough/session.json"), before);
}

fn event_count_limit_and_recent_reader_work_are_bounded() {
    let f = Fixture::new();
    init(&f);
    chain(&f, 4_096, None);
    let status = f.ok(&["status"]);
    assert_eq!(status["transition_history"]["events_read"], 128);
    assert!(status["transition_history"]["bytes_read"].as_u64().unwrap() <= 512 * 1024);
    let before = f.read(".chough/session.json");
    assert!(
        f.fails(&["stage", "iterate", "--note", "full event cycle"])
            .contains("capacity reached")
    );
    assert_eq!(f.read(".chough/session.json"), before);
    f.ok(&[
        "stage",
        "reset",
        "--to",
        "intent",
        "--reason",
        "continue in a fresh bounded cycle",
    ]);
}

fn backdate_events(f: &Fixture) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        - 30 * 86_400;
    let mut cursor = f.session()["history_cursor"].clone();
    let mut paths = Vec::new();
    while let Some(head) = cursor["head"].as_str() {
        let path = format!(
            ".chough/history/cycles/{}/{head}.json",
            cursor["cycle"].as_str().unwrap()
        );
        let event: Value = serde_json::from_str(&f.read(&path)).unwrap();
        paths.push(path);
        if event["parent"].is_null() {
            cursor = event["previous_cycle"].clone();
        } else {
            cursor["head"] = event["parent"].clone();
        }
    }
    paths.reverse();
    for (index, path) in paths.iter().enumerate() {
        let mut event: Value = serde_json::from_str(&f.read(path)).unwrap();
        event["transition"]["at_unix_seconds"] = (at + index as u64).into();
        f.write(path, &event.to_string());
    }
}

#[test]
fn cycle_retention_is_read_only_until_authorized_and_revalidates_new_references() {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "initial event"]);
    for index in 0..6 {
        f.ok(&[
            "stage",
            "reset",
            "--to",
            "intent",
            "--reason",
            &format!("closed cycle {index}"),
        ]);
    }
    backdate_events(&f);
    let before = f.read(".chough/session.json");
    let preview = f.ok(&["history", "prune"]);
    assert_eq!(preview["applied"], false);
    assert_eq!(f.read(".chough/session.json"), before);
    assert_eq!(preview["eligible"].as_array().unwrap().len(), 3);
    let protected_path = preview["eligible"][0]["files"][0]["path"].as_str().unwrap();
    let protected_bytes = f.read(protected_path);
    assert!(
        f.fails(&["history", "prune", "--apply"])
            .contains("authorization")
    );
    f.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "fixture authorization",
    ]);
    f.ok(&["checkpoint", "--file", protected_path]);
    let applied = f.ok(&["history", "prune", "--apply"]);
    assert_eq!(applied["removed"].as_array().unwrap().len(), 2);
    assert_eq!(f.read(protected_path), protected_bytes);
    assert!(f.exists(".chough/session.json"));
    let timing = f.ok(&["timing", "--cycle", "all", "--json"]);
    assert!(
        timing["partial"]
            .as_array()
            .unwrap()
            .iter()
            .any(|part| part.to_string().contains("pruned"))
    );
}

#[cfg(unix)]
#[test]
fn a_failed_checkpoint_save_leaves_an_unselected_orphan_and_preserves_active_state() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "selected event"]);
    let before = f.read(".chough/session.json");
    let directory = f.root().join(".chough");
    std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o555)).unwrap();
    let output = f.run(&["stage", "iterate", "--note", "unselected orphan"]);
    std::fs::set_permissions(&directory, std::fs::Permissions::from_mode(0o755)).unwrap();
    assert!(!output.status.success());
    assert_eq!(f.read(".chough/session.json"), before);
    let history = f.ok(&["history", "show"]);
    assert!(history["cycles"].as_array().unwrap().iter().any(|cycle| {
        cycle["unselected_tips"]
            .as_array()
            .is_some_and(|tips| !tips.is_empty())
    }));
    let shown = f.ok(&["stage", "show"]);
    assert!(!shown.to_string().contains("unselected orphan"));
}

#[cfg(unix)]
#[test]
fn recent_guidance_does_not_scan_unselected_files_and_maintenance_refuses_symlinks() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "selected"]);
    let cycle = f.session()["history_cursor"]["cycle"]
        .as_str()
        .unwrap()
        .to_string();
    symlink(
        "/etc/passwd",
        f.root()
            .join(format!(".chough/history/cycles/{cycle}/unsafe.json")),
    )
    .unwrap();
    assert_eq!(
        f.ok(&["stage", "show"])["recent_transitions"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    let report = f.ok(&["history", "prune"]);
    assert_eq!(report["complete"], false);
    assert!(report["eligible"].as_array().unwrap().is_empty());
}

fn closed_cycles() -> Fixture {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "initial a"]);
    f.ok(&["stage", "iterate", "--note", "initial b"]);
    for index in 0..6 {
        f.ok(&[
            "stage",
            "reset",
            "--to",
            "intent",
            "--reason",
            &format!("closed group {index}"),
        ]);
        f.ok(&["stage", "iterate", "--note", "second event"]);
    }
    backdate_events(&f);
    f
}

fn simulated_receipt(f: &Fixture, plan: &Value) -> (String, Vec<String>) {
    use std::time::{SystemTime, UNIX_EPOCH};
    let cycle = plan["cycle"].as_str().unwrap();
    let show = f.ok(&["history", "show"]);
    let group = show["cycles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|group| group["id"] == cycle)
        .unwrap();
    let paths: Vec<String> = plan["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|file| file["path"].as_str().unwrap().to_string())
        .collect();
    let at = paths
        .iter()
        .map(|path| {
            serde_json::from_str::<Value>(&f.read(path)).unwrap()["transition"]["at_unix_seconds"]
                .as_u64()
                .unwrap()
        })
        .max()
        .unwrap();
    let receipt = serde_json::json!({"schema_version":1,"cycle":cycle,"files":plan["files"],"authority":"fixture authorization","reason":"complete closed cycle outside every protected set","recorded_at_unix_seconds":SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),"closed_at":at,"tips":group["tips"]});
    let path = format!(".chough/history/pruned/{cycle}.json");
    f.write(&path, &receipt.to_string());
    (path, paths)
}

#[test]
fn interrupted_cleanup_resumes_only_the_identical_receipted_plan() {
    let f = closed_cycles();
    let preview = f.ok(&["history", "prune"]);
    let plan = preview["eligible"][0].clone();
    let (receipt, paths) = simulated_receipt(&f, &plan);
    let receipt_bytes = f.read(&receipt);
    std::fs::remove_file(f.root().join(&paths[0])).unwrap();
    let preview = f.ok(&["history", "prune"]);
    let remaining = preview["eligible"]
        .as_array()
        .unwrap()
        .iter()
        .find(|cycle| cycle["cycle"] == plan["cycle"])
        .unwrap();
    let actual_bytes: u64 = paths
        .iter()
        .filter(|path| f.exists(path))
        .map(|path| std::fs::metadata(f.root().join(path)).unwrap().len())
        .sum();
    assert_eq!(
        remaining["bytes"], actual_bytes,
        "reclaimable bytes must exclude already removed content"
    );
    assert_eq!(
        remaining["files"].as_array().unwrap().len(),
        paths.len() - 1
    );
    let show = f.ok(&["history", "show"]);
    assert!(
        show["cycles"]
            .as_array()
            .unwrap()
            .iter()
            .any(|cycle| cycle["id"] == plan["cycle"] && cycle["interrupted"] == true)
    );
    f.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "fixture authorization",
    ]);
    let applied = f.ok(&["history", "prune", "--apply"]);
    assert_eq!(applied["complete"], true);
    assert!(paths.iter().all(|path| !f.exists(path)));
    assert_eq!(f.read(&receipt), receipt_bytes);
}

#[test]
fn new_protection_prevents_resuming_an_interrupted_plan() {
    let f = closed_cycles();
    let preview = f.ok(&["history", "prune"]);
    let plan = preview["eligible"][0].clone();
    let (_, paths) = simulated_receipt(&f, &plan);
    std::fs::remove_file(f.root().join(&paths[0])).unwrap();
    let before = f.read(&paths[1]);
    f.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "fixture authorization",
    ]);
    f.ok(&["checkpoint", "--file", &paths[0]]);
    f.ok(&["history", "prune", "--apply"]);
    assert_eq!(f.read(&paths[1]), before);
}

#[test]
fn unsafe_or_changed_receipts_cannot_broaden_deletion() {
    let f = closed_cycles();
    let plan = f.ok(&["history", "prune"])["eligible"][0].clone();
    let (receipt, paths) = simulated_receipt(&f, &plan);
    let before = f.read(&paths[0]);
    let mut value: Value = serde_json::from_str(&f.read(&receipt)).unwrap();
    value["files"][0]["path"] = "../outside.json".into();
    f.write(&receipt, &value.to_string());
    assert_eq!(f.ok(&["history", "prune"])["complete"], false);
    f.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "fixture authorization",
    ]);
    assert!(!f.run(&["history", "prune", "--apply"]).status.success());
    assert_eq!(f.read(&paths[0]), before);
}

#[test]
fn malformed_receipts_leave_missing_history_unavailable() {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "missing event"]);
    let cursor = f.session()["history_cursor"].clone();
    std::fs::remove_file(f.root().join(event_path(&f))).unwrap();
    f.write(
        &format!(
            ".chough/history/pruned/{}.json",
            cursor["cycle"].as_str().unwrap()
        ),
        "{broken receipt",
    );
    let status = f.ok(&["status"]);
    let partial = status["transition_history"]["partial"].to_string();
    assert!(partial.contains("unavailable history"));
    assert!(!partial.contains("pruned or interrupted"));
}

#[test]
fn full_timing_keeps_unselected_tips_out_of_selected_stage_spans() {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "selected history"]);
    let path = event_path(&f);
    let mut orphan: Value = serde_json::from_str(&f.read(&path)).unwrap();
    let id = "EVT-ffffffffffffffffffffffff";
    orphan["id"] = id.into();
    orphan["transition"]["reason"] = "unselected history".into();
    f.write(
        &format!(
            ".chough/history/cycles/{}/{id}.json",
            orphan["cycle"].as_str().unwrap()
        ),
        &orphan.to_string(),
    );
    let timing = f.ok(&["timing", "--cycle", "all", "--json"]);
    assert_eq!(timing["transitions"]["records"], 1);
    let unselected = timing["transition_lineages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|lineage| lineage["selection"] == "unselected")
        .unwrap();
    assert_eq!(unselected["tip"], id);
    assert_eq!(unselected["completion"], "unknown");
    assert!(unselected["duration"].is_null());
}

fn source_observation(id: &str) -> String {
    serde_json::json!({"schema_version":1,"id":id,"criteria":["LIFE-3"],"requirements":["REQ-cli-008"],"contracts":["specs/cli/cli.spec.md"],"kind":"product-test","outcome":"passed","environment":"fixture","summary":"metadata source","artifacts":[],"artifact_fingerprints":{},"observed_at_unix_seconds":1,"snapshot":{"revision":"0".repeat(40),"dirty":false,"fingerprint":"0".repeat(64)},"provenance":"fixture"}).to_string()
}

fn metadata_scan_file_limit_is_exact_and_overflow_is_a_read_only_incomplete_preview() {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "one event"]);
    // One checkpoint and one event consume the first two source-file slots.
    for index in 0..16_382 {
        let id = format!("source-{index:05}");
        f.write(
            &format!(".chough/evidence/{id}.json"),
            &source_observation(&id),
        );
    }
    assert_eq!(f.ok(&["history", "prune"])["complete"], true);
    f.write(".chough/evidence/overflow.json", "{}");
    let checkpoint = f.read(".chough/session.json");
    let preview = f.ok(&["history", "prune"]);
    assert_eq!(preview["complete"], false);
    assert!(
        preview["reason"]
            .as_str()
            .unwrap()
            .contains("metadata bound")
    );
    assert_eq!(f.read(".chough/session.json"), checkpoint);
}

fn metadata_byte_limit_is_exact_and_protected_only_data_is_a_no_op() {
    let f = Fixture::new();
    init(&f);
    f.ok(&["stage", "iterate", "--note", "one event"]);
    let initial = f.read(".chough/session.json").len() + f.read(&event_path(&f)).len();
    for index in 0..64 {
        let size = if index == 63 {
            1_048_576 - initial
        } else {
            1_048_576
        };
        f.write(
            &format!(".chough/artifacts/source-{index:02}.log"),
            &" ".repeat(size),
        );
    }
    let preview = f.ok(&["history", "prune"]);
    assert_eq!(preview["complete"], true);
    assert!(preview["eligible"].as_array().unwrap().is_empty());
    f.write(".chough/artifacts/overflow.log", "x");
    assert_eq!(f.ok(&["history", "prune"])["complete"], false);
}

fn set_latest_time(f: &Fixture, seconds: u64) {
    let path = event_path(f);
    let mut event: Value = serde_json::from_str(&f.read(&path)).unwrap();
    event["transition"]["at_unix_seconds"] = seconds.into();
    f.write(&path, &event.to_string());
}

#[test]
fn real_git_retention_integrates_with_another_branch_new_evidence_and_events() {
    use sha2::{Digest, Sha256};
    let f = closed_cycles();
    f.write(
        ".gitattributes",
        ".chough/session.json merge=chough-session\n",
    );
    git_ok(&f, &["init"]);
    let binary = env!("CARGO_BIN_EXE_chough")
        .replace('\\', "/")
        .replace('\'', "'\\''");
    let driver = format!("'{binary}' checkpoint --merge %O %A %B");
    git_ok(&f, &["config", "merge.chough-session.driver", &driver]);
    git_ok(&f, &["checkout", "-b", "baseline"]);
    git_ok(&f, &["add", "."]);
    git_ok(&f, &["commit", "-m", "shared retained history"]);
    git_ok(&f, &["checkout", "-b", "cleanup"]);
    f.ok(&[
        "consent",
        "grant",
        "retention",
        "--source",
        "disposable fixture consent",
    ]);
    let removal = f.ok(&["history", "prune", "--apply"]);
    assert!(!removal["removed"].as_array().unwrap().is_empty());
    git_ok(&f, &["add", ".chough"]);
    git_ok(&f, &["commit", "-m", "consented closed-cycle retention"]);
    git_ok(&f, &["checkout", "-b", "writer", "baseline"]);
    f.ok(&["stage", "iterate", "--note", "independent writer event"]);
    let selected = f.session()["history_cursor"].clone();
    let payload = "independent retained artifact\n";
    let expected = format!("{:x}", Sha256::digest(payload.as_bytes()));
    let mut record: Value = serde_json::from_str(&source_observation("independent-proof")).unwrap();
    record["artifacts"] = serde_json::json!([".chough/artifacts/independent-proof.log"]);
    record["artifact_fingerprints"] =
        serde_json::json!({".chough/artifacts/independent-proof.log":expected});
    f.write(
        ".chough/evidence/independent-proof.json",
        &record.to_string(),
    );
    f.write(".chough/artifacts/independent-proof.log", payload);
    git_ok(&f, &["add", ".chough"]);
    git_ok(&f, &["commit", "-m", "independent writer proof"]);
    let _ = git(&f, &["merge", "cleanup", "--no-edit"]);
    let conflicts = git(&f, &["diff", "--name-only", "--diff-filter=U"]);
    assert_eq!(
        String::from_utf8(conflicts.stdout).unwrap().trim(),
        ".chough/session.json"
    );
    git_ok(&f, &["checkout", "--ours", ".chough/session.json"]);
    git_ok(&f, &["add", ".chough/session.json"]);
    git_ok(
        &f,
        &[
            "commit",
            "-m",
            "select writer checkpoint and retain both histories",
        ],
    );
    assert_eq!(f.session()["history_cursor"], selected);
    assert_eq!(f.read(".chough/artifacts/independent-proof.log"), payload);
    assert_eq!(
        serde_json::from_str::<Value>(&f.read(".chough/evidence/independent-proof.json")).unwrap()
            ["artifact_fingerprints"][".chough/artifacts/independent-proof.log"],
        expected
    );
    assert!(
        f.ok(&["timing", "--cycle", "all", "--json"])["partial"]
            .as_array()
            .unwrap()
            .iter()
            .any(|partial| partial.to_string().contains("pruned"))
    );
    assert_eq!(f.ok(&["history", "show"])["complete"], true);
}

#[test]
fn history_bounds_cover_all_resource_limits() {
    event_and_cycle_byte_limits_refuse_without_mutating_the_checkpoint();
    event_count_limit_and_recent_reader_work_are_bounded();
    metadata_scan_file_limit_is_exact_and_overflow_is_a_read_only_incomplete_preview();
    metadata_byte_limit_is_exact_and_protected_only_data_is_a_no_op();
}

#[test]
fn schema_one_migration_and_adoption_preserve_the_original_backup() {
    let f = Fixture::new();
    let original = serde_json::json!({"schema_version":1,"objective":"legacy migration","intent_scope":"LIFE","modules":["cli"],"decisions":[],"open_questions":[],"next_action":"continue legacy work"}).to_string();
    f.write(".chough/session.json", &original);
    f.ok(&["migrate"]);
    assert_eq!(f.read(".chough/session.v1.json"), original);
    f.ok(&["history", "adopt", "--apply"]);
    assert_eq!(f.read(".chough/session.v1.json"), original);
    f.ok(&[
        "stage",
        "iterate",
        "--note",
        "after explicit migration and adoption",
    ]);
    assert_eq!(f.read(".chough/session.v1.json"), original);
}

#[test]
fn adoption_refuses_future_legacy_payloads_without_changing_any_record() {
    let f = Fixture::new();
    f.init();
    let event = serde_json::json!({"schema_version":99,"at_unix_seconds":100,"kind":"reset","from":"done","to":"intent","refresh":[]});
    let legacy = event.to_string() + "\n";
    f.write(".chough/transitions.jsonl", &legacy);
    let checkpoint = f.read(".chough/session.json");
    assert!(
        f.fails(&["history", "adopt", "--apply"])
            .contains("adoption refused")
    );
    assert_eq!(f.read(".chough/session.json"), checkpoint);
    assert_eq!(f.read(".chough/transitions.jsonl"), legacy);
}

#[test]
fn directory_references_protect_cycles_across_case_and_separator_aliases() {
    let f = closed_cycles();
    assert!(
        !f.ok(&["history", "prune"])["eligible"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    f.ok(&["checkpoint", "--file", ".CHOUGH\\history\\CYCLES"]);
    let preview = f.ok(&["history", "prune"]);
    assert_eq!(preview["complete"], true);
    assert!(preview["eligible"].as_array().unwrap().is_empty());
    assert!(
        preview["protected"]
            .as_array()
            .unwrap()
            .iter()
            .any(|cycle| cycle["reasons"].to_string().contains("session.json"))
    );
}

#[test]
fn frozen_artifact_directory_aliases_protect_cycles_through_preview_and_apply() {
    fn saved_files(
        root: &std::path::Path,
    ) -> std::collections::BTreeMap<std::path::PathBuf, Vec<u8>> {
        fn visit(
            directory: &std::path::Path,
            files: &mut std::collections::BTreeMap<std::path::PathBuf, Vec<u8>>,
        ) {
            for entry in std::fs::read_dir(directory).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    visit(&path, files);
                } else {
                    files.insert(path.clone(), std::fs::read(path).unwrap());
                }
            }
        }
        let mut files = std::collections::BTreeMap::new();
        visit(&root.join(".chough"), &mut files);
        files
    }

    for alias in [
        ".chough/history/cycles",
        r".CHOUGH\history\CYCLES",
        r".CHOUGH\\history\\CYCLES",
        r".CHOUGH//history/./CYCLES\",
    ] {
        for quote in ['`', '"'] {
            let f = closed_cycles();
            assert!(
                !f.ok(&["history", "prune"])["eligible"]
                    .as_array()
                    .unwrap()
                    .is_empty()
            );
            let artifact = ".chough/artifacts/frozen-path-reference.log";
            f.write(
                artifact,
                &format!("Frozen reference: preserve {quote}{alias}{quote}.\n"),
            );
            let before = saved_files(f.root());
            let preview = f.ok(&["history", "prune"]);
            assert_eq!(preview["complete"], true);
            assert_eq!(before, saved_files(f.root()), "preview changed {alias}");
            assert!(
                preview["eligible"].as_array().unwrap().is_empty(),
                "{alias}: {preview}"
            );
            assert_eq!(preview["protected"].as_array().unwrap().len(), 7);
            assert!(
                preview["protected"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|cycle| { cycle["reasons"].to_string().contains(artifact) }),
                "{alias}: {preview}"
            );
            f.ok(&[
                "consent",
                "grant",
                "retention",
                "--source",
                "fixture authorization",
            ]);
            let authorized = saved_files(f.root());
            let applied = f.ok(&["history", "prune", "--apply"]);
            assert_eq!(applied["complete"], true);
            assert_eq!(
                applied["removed"],
                serde_json::json!([]),
                "{alias}: {applied}"
            );
            assert_eq!(applied["reclaimed_bytes"], 0);
            assert_eq!(authorized, saved_files(f.root()), "apply changed {alias}");
        }
    }
}
