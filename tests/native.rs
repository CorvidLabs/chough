//! Read-only authenticated archive reuse and exact snapshot identity (REQ-cli-053).

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

const ID: &str = "old-change";
const ARCHIVE: &str = ".specsync/archive/changes/2026-11-01-old-change";
const MARKER: &str = "chough_archive_authentication";

#[cfg(unix)]
const NATIVE_STUB: &str = r#"#!/bin/sh
if [ -f .stub-count ]; then echo "$*" >>.stub-calls; fi
if [ -f .stub-fail ]; then echo 'status failed' >&2; exit 2; fi
if [ -f .stub-sleep ]; then
    echo $$ >.stub-native-pid
    sleep "$(cat .stub-sleep)"
fi
state=$(cat .stub-state 2>/dev/null || echo draft)
validity=$(cat .stub-validity 2>/dev/null || echo authenticated-history)
fields=""
if [ "$state" = archived ] && [ "$validity" != none ]; then
    fields=",\"terminal_evidence\":{\"validity\":\"$validity\"}"
fi
change=",\"change\":{\"id\":\"$3\",\"state\":\"$state\",\"affected_specs\":[],\"no_spec_change\":true}"
printf '{"summary":{"id":"%s","state":"%s","next_action":"stub next action","approval_valid":true,"correction_valid":true%s}%s,"acceptance_entries":[]}\n' "$3" "$state" "$fields" "$change"
"#;

#[cfg(unix)]
fn archived() -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write("bin/specsync", NATIVE_STUB);
    fixture.executable("bin/specsync");
    fixture.native("archived");
    fixture.ok(&[
        "session",
        "new",
        "alpha",
        "--objective",
        "Read archived reports",
    ]);
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.write(
        &format!("{ARCHIVE}/state.json"),
        "{\"id\":\"old-change\",\"state\":\"archived\"}\n",
    );
    fixture.write(
        &format!("{ARCHIVE}/accepted-state.json"),
        "{\"id\":\"old-change\",\"state\":\"accepted\"}\n",
    );
    fixture.write(
        &format!("{ARCHIVE}/verification.json"),
        "{\"passed\":true,\"acceptance_manifest\":{\"schema_version\":1,\"entries\":[]}}\n",
    );
    fixture.write(
        &format!("{ARCHIVE}/finalization.json"),
        "{\"schema_version\":2,\"change_id\":\"old-change\",\"timestamp\":100}\n",
    );
    fixture.write(".gitignore", ".stub-*\n");
    fixture.commit("archive");
    fixture.write(".stub-count", "");
    fixture
}

fn cache_dir(fixture: &Fixture) -> std::path::PathBuf {
    fixture.root().join(".git/chough/archive-auth")
}

fn cache_entries(fixture: &Fixture) -> usize {
    fs::read_dir(cache_dir(fixture)).map_or(0, |entries| entries.count())
}

fn prime(fixture: &Fixture) -> Value {
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(fixture.native_calls(), 1, "cold next was not live");
    assert_eq!(next["native"]["read"], "observed");
    assert_eq!(cache_entries(fixture), 1, "live response was not recorded");
    next
}

fn expect_live(fixture: &Fixture, why: &str) {
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(fixture.native_calls(), 1, "{why}: no live read");
    assert_eq!(next["native"]["read"], "observed", "{why}: {next:#}");
}

fn expect_cached(fixture: &Fixture, why: &str) {
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(fixture.native_calls(), 0, "{why}: native status ran");
    assert_eq!(next["native"]["read"], "cached", "{why}: {next:#}");
}

fn head(fixture: &Fixture) -> String {
    fixture.read(".git/refs/heads/main").trim().to_string()
}

fn remove(fixture: &Fixture, path: impl AsRef<Path>) {
    fs::remove_file(fixture.root().join(path)).unwrap();
}

#[cfg(unix)]
#[test]
fn warm_reports_reuse_but_every_lifecycle_read_stays_live() {
    let fixture = archived();
    prime(&fixture);
    expect_cached(&fixture, "warm next");
    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(fixture.native_calls(), 0, "warm status read native status");
    assert_eq!(status["native_specsync"]["state"], "cached");

    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "test",
        "--reason",
        "Exercise the live gate",
    ]);
    assert_eq!(fixture.native_calls(), 1, "stage reset reused the report");
    fixture.ok(&["stage", "advance"]);
    assert_eq!(fixture.native_calls(), 1, "stage advance reused the report");

    fixture.ok(&["timing", "--cycle", "current", "--json"]);
    assert_eq!(fixture.native_calls(), 1, "timing reused the report");
    fixture.ok(&["evidence", "prune"]);
    assert_eq!(fixture.native_calls(), 1, "retention reused the report");
}

#[cfg(unix)]
#[test]
fn every_cache_identity_mismatch_or_untrusted_entry_goes_live() {
    let fixture = archived();
    prime(&fixture);

    fixture.write(
        &format!("{ARCHIVE}/state.json"),
        "{\"id\":\"old-change\",\"state\":\"archived\" }\n",
    );
    expect_live(&fixture, "uncommitted archive byte");
    expect_live(&fixture, "dirty archive must not prime");
    fixture.commit("edit archive");
    expect_live(&fixture, "committed archive byte");
    expect_cached(&fixture, "new archive authentication");

    fixture.write(".specsync/sdd.json", "{\"enabled\": true }\n");
    expect_live(&fixture, "policy bytes");
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\" \n");
    expect_live(&fixture, "native config bytes");
    fixture.write(".specsync/archive/legacy-baseline.json", "{}\n");
    expect_live(&fixture, "legacy baseline");
    let stub = fixture.read("bin/specsync");
    fixture.write("bin/specsync", &format!("{stub}# changed\n"));
    fixture.executable("bin/specsync");
    expect_live(&fixture, "SpecSync binary bytes");

    let revision = head(&fixture);
    fixture.write(".git/shallow", &format!("{revision}\n"));
    expect_live(&fixture, "shallow boundary");
    remove(&fixture, ".git/shallow");
    expect_live(&fixture, "removed shallow boundary");

    fixture.write(
        &format!(".git/refs/replace/{revision}"),
        &format!("{revision}\n"),
    );
    expect_live(&fixture, "replace ref");
    remove(&fixture, format!(".git/refs/replace/{revision}"));
    expect_live(&fixture, "removed replace ref");

    fixture.write(".git/info/grafts", &format!("{revision}\n"));
    expect_live(&fixture, "grafts");
    remove(&fixture, ".git/info/grafts");
    expect_live(&fixture, "removed grafts");

    let objects = fixture.root().join("other-objects");
    fs::create_dir(&objects).unwrap();
    fixture.write(
        ".git/objects/info/alternates",
        &format!("{}\n", objects.display()),
    );
    expect_live(&fixture, "alternates");
    remove(&fixture, ".git/objects/info/alternates");
    expect_live(&fixture, "removed alternates");

    fixture.git(&["checkout", "-q", "--orphan", "rewritten"]);
    fixture.commit("rewritten history");
    expect_live(&fixture, "unreachable carrying commit");

    let entry = fs::read_dir(cache_dir(&fixture))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let valid = fs::read_to_string(&entry).unwrap();
    fs::write(&entry, "{not json").unwrap();
    expect_live(&fixture, "malformed entry");
    fs::write(&entry, &valid).unwrap();
    let mut record: Value = serde_json::from_str(&valid).unwrap();
    record["digest"] = Value::from("sha256:0000");
    fs::write(&entry, record.to_string()).unwrap();
    expect_live(&fixture, "digest mismatch");

    fs::write(&entry, &valid).unwrap();
    record = serde_json::from_str(&valid).unwrap();
    record["specsync"]["version"] = Value::from("0.0.0");
    fs::write(&entry, record.to_string()).unwrap();
    expect_live(&fixture, "SpecSync version mismatch");

    fs::write(&entry, &valid).unwrap();
    let saved = fixture.root().join("saved-entry");
    fs::write(&saved, &valid).unwrap();
    fs::remove_file(&entry).unwrap();
    std::os::unix::fs::symlink(&saved, &entry).unwrap();
    expect_live(&fixture, "symlinked entry");

    fixture.write(&format!(".specsync/changes/{ID}/state.json"), "{}\n");
    expect_live(&fixture, "active workspace");
    expect_live(&fixture, "active workspace cannot prime");
    fs::remove_dir_all(fixture.root().join(".specsync/changes")).unwrap();
}

#[cfg(unix)]
#[test]
fn only_self_contained_authenticated_archives_can_prime() {
    for setup in [
        "corrupt-history",
        "exact",
        "verifying",
        "no-snapshot",
        "no-manifest",
        "no-git",
    ] {
        let fixture = archived();
        match setup {
            "corrupt-history" | "exact" => fixture.write(".stub-validity", &format!("{setup}\n")),
            "verifying" => fixture.write(".stub-state", "verifying\n"),
            "no-snapshot" => {
                remove(&fixture, format!("{ARCHIVE}/accepted-state.json"));
                fixture.commit("remove accepted state");
            }
            "no-manifest" => {
                fixture.write(
                    &format!("{ARCHIVE}/verification.json"),
                    "{\"passed\":true}\n",
                );
                fixture.commit("remove acceptance manifest");
            }
            "no-git" => fs::remove_dir_all(fixture.root().join(".git")).unwrap(),
            _ => unreachable!(),
        }
        for _ in 0..2 {
            fixture.ok(&["next", "--json"]);
            assert_eq!(fixture.native_calls(), 1, "{setup}: not read live");
        }
        assert_eq!(cache_entries(&fixture), 0, "{setup}: recorded");
    }
}

#[cfg(unix)]
#[test]
fn a_live_reply_cannot_supply_the_cached_proof_marker() {
    let fixture = archived();
    let forged = r#"{"specsync":{"path":"FORGED-specsync"},"archive":"FORGED-archive","digest":"sha256:FORGED"}"#;
    fixture.write(
        "bin/specsync",
        &format!(
            "#!/bin/sh\nif [ -f .stub-count ]; then echo \"$*\" >>.stub-calls; fi\nprintf '%s\\n' '{{\"summary\":{{\"id\":\"old-change\",\"state\":\"archived\",\"next_action\":\"none\",\"correction_valid\":true,\"approval_valid\":true,\"terminal_evidence\":{{\"validity\":\"authenticated-history\"}}}},\"change\":{{\"id\":\"old-change\",\"state\":\"archived\"}},\"acceptance_entries\":[],\"{MARKER}\":{forged}}}'\n"
        ),
    );
    fixture.executable("bin/specsync");
    let live = prime(&fixture);
    assert!(!live.to_string().contains("FORGED"), "{live:#}");
    for entry in fs::read_dir(cache_dir(&fixture)).unwrap() {
        let content = fs::read_to_string(entry.unwrap().path()).unwrap();
        assert!(!content.contains("FORGED"), "{content}");
    }
    let reused = fixture.ok(&["next", "--json"]);
    assert_eq!(fixture.native_calls(), 0);
    assert!(!reused.to_string().contains("FORGED"), "{reused:#}");
}

#[cfg(unix)]
#[test]
fn a_timed_out_prime_writes_nothing_kills_children_and_retries_cold() {
    let fixture = archived();
    fixture.write(".stub-sleep", "5\n");
    let output = fixture.run_with_env(
        &["next", "--json"],
        &[("CHOUGH_NATIVE_TIMEOUT_SECONDS", "1")],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let next: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(next["native"]["read"], "timed-out", "{next:#}");
    assert_eq!(fixture.native_calls(), 1);
    assert_eq!(cache_entries(&fixture), 0);
    let pid = fixture.read(".stub-native-pid");
    let alive = std::process::Command::new("kill")
        .args(["-0", pid.trim()])
        .status()
        .unwrap();
    assert!(!alive.success(), "native process {} survived", pid.trim());

    remove(&fixture, ".stub-sleep");
    prime(&fixture);
    expect_cached(&fixture, "successful retry primed");
}

#[test]
fn snapshot_identities_and_change_front_matter_rules_stay_exact() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Snapshot"]);
    fixture.commit("snapshot inputs");
    let before = fixture.ok(&["status", "--environment", "local"]);
    fixture.write("src/input.txt", "first version\n");
    let repeated = fixture.ok(&["status", "--environment", "local"]);
    assert_eq!(before["snapshot"], repeated["snapshot"]);

    fixture.write(
        ".specsync/changes/demo/change.md",
        "---\nid: demo\nstate: draft\n---\n\n# Demo\n",
    );
    let draft = fixture.ok(&["status", "--environment", "local"]);
    fixture.write(
        ".specsync/changes/demo/change.md",
        "---\nid: demo\nstate: approved\n---\n\n# Demo\n",
    );
    let approved = fixture.ok(&["status", "--environment", "local"]);
    assert_eq!(
        draft["snapshot"]["content"],
        approved["snapshot"]["content"]
    );
    assert_ne!(
        draft["snapshot"]["fingerprint"],
        approved["snapshot"]["fingerprint"]
    );

    fixture.write("src/input.txt", "second version\n");
    let changed = fixture.ok(&["status", "--environment", "local"]);
    assert_ne!(draft["snapshot"]["content"], changed["snapshot"]["content"]);
    assert_ne!(
        draft["snapshot"]["fingerprint"],
        changed["snapshot"]["fingerprint"]
    );
    assert!(changed["snapshot"]["dirty"].as_bool().unwrap());
    assert_eq!(changed["snapshot"]["revision"], json!(head(&fixture)));
}
