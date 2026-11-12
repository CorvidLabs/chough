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
if [ "$1" = --version ]; then printf 'specsync 6.3.0\n'; exit 0; fi
if [ -f .stub-count ]; then echo "$*" >>.stub-calls; fi
if [ -f .stub-fail ]; then echo 'status failed' >&2; exit 2; fi
if [ -f .stub-sleep ]; then
    echo $$ >.stub-native-pid.tmp && mv .stub-native-pid.tmp .stub-native-pid
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
    // 0.6.0 has no retention, so `evidence prune` is gone (REQ-history-001 and
    // REQ-history-004 are removed).
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
    expect_cached(&fixture, "removed shallow boundary");

    fixture.write(
        &format!(".git/refs/replace/{revision}"),
        &format!("{revision}\n"),
    );
    expect_live(&fixture, "replace ref");
    remove(&fixture, format!(".git/refs/replace/{revision}"));
    expect_cached(&fixture, "removed replace ref");

    fixture.write(".git/info/grafts", &format!("{revision}\n"));
    expect_live(&fixture, "grafts");
    remove(&fixture, ".git/info/grafts");
    expect_cached(&fixture, "removed grafts");

    let objects = fixture.root().join("other-objects");
    fs::create_dir(&objects).unwrap();
    fixture.write(
        ".git/objects/info/alternates",
        &format!("{}\n", objects.display()),
    );
    expect_live(&fixture, "alternates");
    remove(&fixture, ".git/objects/info/alternates");
    expect_cached(&fixture, "removed alternates");

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
            "#!/bin/sh\nif [ \"$1\" = --version ]; then printf 'specsync 6.3.0\\n'; exit 0; fi\nif [ -f .stub-count ]; then echo \"$*\" >>.stub-calls; fi\nprintf '%s\\n' '{{\"summary\":{{\"id\":\"old-change\",\"state\":\"archived\",\"next_action\":\"none\",\"correction_valid\":true,\"approval_valid\":true,\"terminal_evidence\":{{\"validity\":\"authenticated-history\"}}}},\"change\":{{\"id\":\"old-change\",\"state\":\"archived\"}},\"acceptance_entries\":[],\"{MARKER}\":{forged}}}'\n"
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
fn a_killed_unreaped_child_is_not_alive_but_a_running_child_is() {
    let mut child = std::process::Command::new("sleep")
        .arg("60")
        .spawn()
        .unwrap();
    let pid = child.id().to_string();
    let running = common::alive(&pid);
    let killed = std::process::Command::new("kill")
        .args(["-9", &pid])
        .status()
        .unwrap();
    let zombie = common::alive(&pid);
    if killed.success() {
        child.wait().unwrap();
    } else {
        let _ = child.kill();
        let _ = child.wait();
    }
    assert!(killed.success(), "failed to kill child {pid}");
    assert!(running, "running child {pid} was not detected");
    assert!(!zombie, "killed, unreaped child {pid} still reads as alive");
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
    assert!(
        !common::alive(pid.trim()),
        "native process {} survived",
        pid.trim()
    );

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

#[cfg(unix)]
#[test]
fn failed_malformed_and_oversized_inputs_never_become_authenticated() {
    for mode in ["failed", "invalid-output"] {
        let fixture = archived();
        if mode == "failed" {
            fixture.write(".stub-fail", "");
        } else {
            fixture.write(
                "bin/specsync",
                "#!/bin/sh\nif [ \"$1\" = --version ]; then printf 'specsync 6.3.0\\n'; exit 0; fi\nif [ -f .stub-count ]; then echo \"$*\" >>.stub-calls; fi\nprintf 'not json\\n'\n",
            );
            fixture.executable("bin/specsync");
        }
        for _ in 0..2 {
            let next = fixture.ok(&["next", "--json"]);
            assert_eq!(fixture.native_calls(), 1, "{mode}: not read live");
            assert_eq!(next["native"]["read"], json!(mode), "{next:#}");
            assert_eq!(cache_entries(&fixture), 0, "{mode}: recorded");
        }
    }

    let fixture = archived();
    prime(&fixture);
    let entry = fs::read_dir(cache_dir(&fixture))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    fs::write(&entry, vec![b'x'; 8 * 1024 * 1024 + 1]).unwrap();
    expect_live(&fixture, "oversized entry");
}

#[cfg(unix)]
#[test]
fn a_resolved_specsync_symlink_can_prime_and_reuse() {
    let fixture = archived();
    let real = fixture.root().join("real-specsync");
    fs::rename(fixture.root().join("bin/specsync"), &real).unwrap();
    std::os::unix::fs::symlink(&real, fixture.root().join("bin/specsync")).unwrap();
    prime(&fixture);
    let entry = fs::read_dir(cache_dir(&fixture))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let record: Value = serde_json::from_str(&fs::read_to_string(entry).unwrap()).unwrap();
    assert_eq!(
        record["specsync"]["path"].as_str().unwrap(),
        real.canonicalize().unwrap().to_str().unwrap()
    );
    expect_cached(&fixture, "resolved SpecSync symlink");
}

// REQ-records-014: Chough's records stay out of SpecSync's verification. These run real
// SpecSync (the CI pin; 6.1.2 is the first release with `verification_ignored_paths`) on
// the shipping sample; without SpecSync on PATH they are skipped, and the Fledge CI job
// runs them.

/// Whether real SpecSync is on PATH.
#[cfg(unix)]
fn real_specsync() -> bool {
    let found = std::process::Command::new("specsync")
        .arg("--version")
        .output()
        .is_ok_and(|output| output.status.success());
    if !found {
        eprintln!("skipped: real SpecSync is not on PATH (the Fledge CI job runs this test)");
    }
    found
}

/// The shipping sample under real SpecSync, its change `free-shipping-at-the-threshold`
/// approved by a SYNTHETIC actor and implemented, everything committed. With `ENTRIES=yes`
/// the seed commit lists Chough's three record folders in `verification_ignored_paths`.
#[cfg(unix)]
const IMPLEMENTED: &str = r##"set -eu
rm -rf ./* .chough .specsync
cp -R "$SAMPLE/." .
rm -f .specsync/changes/.keep .specsync/archive/changes/.keep
git -c init.defaultBranch=main init -q
git config user.name "Chough test"; git config user.email test@example.invalid; git config commit.gpgsign false
"$CHOUGH" init --objective "SYNTHETIC verification exclusions" --intent-scope CHECKOUT --module shipping >/dev/null
specsync change adopt --json >/dev/null
if [ "$ENTRIES" = yes ]; then
  sed 's#"verification_commands": \[\]#"verification_ignored_paths": [".chough/archive/", ".chough/bugs/", ".chough/sessions/"],\n  "verification_commands": []#' .specsync/sdd.json >sdd.tmp
  mv sdd.tmp .specsync/sdd.json
  grep -q verification_ignored_paths .specsync/sdd.json
fi
git add -A; git commit -qm seed
id=free-shipping-at-the-threshold
specsync change new "Free shipping at the threshold" --spec shipping --path src/lib.rs --path tests/ --json >/dev/null
specsync change answer $id acceptance_criteria '["An order of exactly 5000 cents ships free"]' --json >/dev/null
specsync change answer $id public_contract yes --json >/dev/null
specsync change answer $id architecture_risk no --json >/dev/null
specsync change answer $id constraints none --json >/dev/null 2>&1 || true
for f in .specsync/changes/$id/*.md; do
  name=$(basename "$f" .md)
  [ "$name" = change ] && continue
  printf -- '---\nchange: %s\nartifact: %s\n---\n\n# %s\n\n' "$id" "$name" "$name" >"$f"
  case "$name" in
    tasks) printf -- '- [x] Add the threshold rule.\n' >>"$f" ;;
    testing) printf -- '## Requirement Evidence\n\n- `REQ-shipping-002`: tests/threshold.rs asserts a zero fee at the threshold.\n' >>"$f" ;;
    plan) printf '## Approach\n\nAdd the rule.\n\n## Out of scope\n\nNone.\n\n## Steps\n\n1. Add it.\n\n## Risks\n\nOff by one.\n\n## Constraints consulted\n\nNone.\n' >>"$f" ;;
    *) printf 'Orders of 5000 cents or more ship free.\n' >>"$f" ;;
  esac
done
mkdir -p .specsync/changes/$id/deltas
printf '## ADDED\n\n### REQUIREMENT REQ-shipping-002\n\nThe module SHALL charge no fee at or above the threshold. Trace: CHECKOUT-1.\n\nAcceptance Criteria\n- tests/threshold.rs asserts a zero fee at the threshold.\n' >.specsync/changes/$id/deltas/shipping.md
git add -A; git commit -qm draft
specsync change approve $id --actor "SYNTHETIC-test-approver-not-a-human" >/dev/null
mkdir -p tests; printf '#[test]\nfn threshold() {}\n' >tests/threshold.rs
git add -A; git commit -qm implement
"##;

#[cfg(unix)]
const CHANGE: &str = "free-shipping-at-the-threshold";

/// Run `script` with `sh` in `dir`, as the fixture's Chough would see it: no actor or
/// user identity from this machine. Returns the output, whatever the exit.
#[cfg(unix)]
fn sh_in(
    dir: &Path,
    fixture: &Fixture,
    script: &str,
    env: &[(&str, &str)],
) -> std::process::Output {
    let mut command = std::process::Command::new("sh");
    command
        .arg("-c")
        .arg(script)
        .current_dir(dir)
        .env("CHOUGH", env!("CARGO_BIN_EXE_chough"))
        .env(
            "SAMPLE",
            concat!(env!("CARGO_MANIFEST_DIR"), "/examples/shipping"),
        )
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("XDG_CONFIG_HOME", fixture.root().join(common::USER_CONFIG))
        .envs(env.iter().copied());
    for name in common::ACTOR_VARIABLES {
        command.env_remove(name);
    }
    command.output().unwrap()
}

/// [`sh_in`] in the fixture root, which must succeed; returns stdout.
#[cfg(unix)]
fn sh_ok(fixture: &Fixture, script: &str, env: &[(&str, &str)]) -> String {
    let output = sh_in(fixture.root(), fixture, script, env);
    assert!(
        output.status.success(),
        "script failed: {}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// The implemented sample, with or without the three entries.
#[cfg(unix)]
fn implemented(entries: bool) -> Fixture {
    let fixture = Fixture::new();
    sh_ok(
        &fixture,
        IMPLEMENTED,
        &[("ENTRIES", if entries { "yes" } else { "no" })],
    );
    fixture
}

/// `specsync change audit`'s errors and the change's handoff readiness, read in `dir`;
/// both are read-only. Verification is current when the audit has no error and the
/// readiness is `safe`.
#[cfg(unix)]
fn verification(dir: &Path, fixture: &Fixture) -> (Vec<String>, String) {
    let read = |args: &[&str]| -> Value {
        let output = sh_in(dir, fixture, &format!("specsync {}", args.join(" ")), &[]);
        serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
            panic!(
                "specsync {args:?}: {error}: {}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            )
        })
    };
    let audit = read(&["change", "audit", "--json"]);
    let errors = audit["report"]["errors"]
        .as_array()
        .unwrap_or_else(|| panic!("{audit:#}"))
        .iter()
        .map(|error| error.as_str().unwrap_or_default().to_string())
        .collect();
    let status = read(&["change", "status", CHANGE, "--json"]);
    let readiness = status["summary"]["handoff"]["readiness"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    (errors, readiness)
}

/// REQ-records-014, TC-records-untracked-receipt-keeps-verification (spec-sync #1045):
/// an untracked receipt under `.chough/sessions/` present while `change check --commit`
/// runs is left out of the commit. With the entries, verification is current in that tree
/// and in a clean clone of the pushed commit, whose workspace digest matches the recorded
/// one; without them the clone's verification is stale, as the control.
#[cfg(unix)]
#[test]
fn an_untracked_receipt_at_check_commit_matches_a_clean_clone() {
    if !real_specsync() {
        return;
    }
    for entries in [true, false] {
        let fixture = implemented(entries);
        let elsewhere = tempfile::tempdir().unwrap();
        let origin = elsewhere.path().join("origin.git");
        let clone = elsewhere.path().join("clone");
        sh_ok(
            &fixture,
            r##"set -eu
mkdir -p .chough/sessions/SYNTHETIC-session/evidence
printf 'SYNTHETIC receipt\n' >.chough/sessions/SYNTHETIC-session/evidence/r1.md
specsync change check free-shipping-at-the-threshold --commit >/dev/null
test -n "$(git status --porcelain --untracked-files=all .chough/sessions/SYNTHETIC-session)"
git init -q --bare --initial-branch=main "$ORIGIN"
git push -q "$ORIGIN" main
git clone -q "$ORIGIN" "$CLONE"
"##,
            &[
                ("ORIGIN", origin.to_str().unwrap()),
                ("CLONE", clone.to_str().unwrap()),
            ],
        );
        let (errors, readiness) = verification(fixture.root(), &fixture);
        assert!(errors.is_empty(), "entries {entries}: tree {errors:?}");
        assert_eq!(readiness, "safe", "entries {entries}: tree");
        assert!(!clone.join(".chough/sessions/SYNTHETIC-session").exists());
        let (errors, readiness) = verification(&clone, &fixture);
        if entries {
            assert!(errors.is_empty(), "clean clone: {errors:?}");
            assert_eq!(readiness, "safe", "clean clone");
        } else {
            assert!(
                errors
                    .iter()
                    .any(|error| error.contains("verification evidence is stale")),
                "control: the clean clone should be stale: {errors:?}"
            );
            assert_eq!(readiness, "conditional", "control: clean clone");
        }
    }
}

/// The Chough writes after `change check --commit` that the #212 case makes, then a
/// commit of them.
#[cfg(unix)]
const WRITES_AFTER_CHECK: &str = r##"set -eu
specsync change check free-shipping-at-the-threshold --commit >/dev/null
"$CHOUGH" checkpoint --native-change free-shipping-at-the-threshold >/dev/null
"$CHOUGH" checkpoint --completed "SYNTHETIC: verified the threshold" >/dev/null
"$CHOUGH" bug add --title "SYNTHETIC bug found after the check" --symptom s --trigger t --expected e --actual a --environment local --scope out-of-scope >/dev/null
test -n "$(git status --porcelain .chough/sessions .chough/bugs)"
git add -A; git commit -qm "chough records after the check"
"##;

/// REQ-records-014, TC-records-write-after-check-keeps-verification (#212): with the
/// entries, a `chough checkpoint` and a `chough bug add` written after `change check
/// --commit` and then committed leave verification current, the audit passing, and the
/// review then records; without the entries the same writes stale verification and the
/// review is refused, as the control. (`chough evidence add` is the ignored test below
/// until plan step 4 moves evidence entries under `.chough/sessions/`.)
#[cfg(unix)]
#[test]
fn chough_writes_after_check_commit_leave_verification_current() {
    if !real_specsync() {
        return;
    }
    for entries in [true, false] {
        let fixture = implemented(entries);
        sh_ok(&fixture, WRITES_AFTER_CHECK, &[]);
        let (errors, readiness) = verification(fixture.root(), &fixture);
        let review = sh_in(
            fixture.root(),
            &fixture,
            "specsync change review free-shipping-at-the-threshold --reviewer SYNTHETIC-test-reviewer-not-a-human",
            &[],
        );
        if entries {
            assert!(errors.is_empty(), "{errors:?}");
            assert_eq!(readiness, "safe");
            assert!(
                review.status.success(),
                "the review did not record: {}{}",
                String::from_utf8_lossy(&review.stdout),
                String::from_utf8_lossy(&review.stderr)
            );
        } else {
            assert!(
                errors
                    .iter()
                    .any(|error| error.contains("verification evidence is stale")),
                "control: {errors:?}"
            );
            assert_eq!(readiness, "conditional", "control");
            assert!(!review.status.success(), "control: the review recorded");
        }
    }
}

/// REQ-records-014 (#212): `chough evidence add` after `change check --commit` is a
/// Chough write like the others once its entry lives under `.chough/sessions/`.
#[cfg(unix)]
#[test]
fn chough_evidence_add_after_check_commit_leaves_verification_current() {
    if !real_specsync() {
        return;
    }
    let fixture = implemented(true);
    sh_ok(
        &fixture,
        r##"set -eu
specsync change check free-shipping-at-the-threshold --commit >/dev/null
"$CHOUGH" evidence add --criterion CHECKOUT-1 --requirement REQ-shipping-002 --contract specs/shipping/shipping.spec.md --kind product-test --outcome passed --environment local --summary "SYNTHETIC threshold run" >/dev/null
git add -A; git commit -qm "chough evidence after the check"
"##,
        &[],
    );
    let (errors, readiness) = verification(fixture.root(), &fixture);
    assert!(errors.is_empty(), "{errors:?}");
    assert_eq!(readiness, "safe");
}

/// REQ-records-014, TC-records-case-definitions-stay-verified: with the entries listed,
/// a case definition changed under `.chough/cases/` after `change check --commit` still
/// stales verification, so nothing a change's verification needs is hidden.
#[cfg(unix)]
#[test]
fn a_case_definition_change_still_stales_verification() {
    if !real_specsync() {
        return;
    }
    let fixture = implemented(true);
    sh_ok(
        &fixture,
        r##"set -eu
specsync change check free-shipping-at-the-threshold --commit >/dev/null
printf '\n# SYNTHETIC edit after the check\n' >>.chough/cases/shipping.cases.toml
git add -A; git commit -qm "edit a case definition"
"##,
        &[],
    );
    let (errors, readiness) = verification(fixture.root(), &fixture);
    assert!(
        errors
            .iter()
            .any(|error| error.contains("verification evidence is stale")),
        "{errors:?}"
    );
    assert_eq!(readiness, "conditional");
}

/// REQ-records-014: when SpecSync refuses the list because an open change's affected
/// paths overlap it, Chough passes the refusal through and names the change, in `chough
/// next` and in `chough change check`.
#[cfg(unix)]
#[test]
fn a_refused_exclusion_list_names_the_overlapping_change() {
    if !real_specsync() {
        return;
    }
    let fixture = implemented(false);
    sh_ok(
        &fixture,
        r##"set -eu
specsync change new "Bug records" --spec shipping --path .chough/bugs/ --json >/dev/null
sed 's#"verification_commands": \[\]#"verification_ignored_paths": [".chough/archive/", ".chough/bugs/", ".chough/sessions/"],\n  "verification_commands": []#' .specsync/sdd.json >sdd.tmp
mv sdd.tmp .specsync/sdd.json
"$CHOUGH" checkpoint --native-change bug-records >/dev/null
"##,
        &[],
    );
    let refusal = "`.chough/bugs/` overlaps affected path `.chough/bugs/` of change `bug-records`";
    let next = fixture.ok(&["next", "--json"]);
    let action = next["native"]["next_action"].as_str().unwrap_or_default();
    assert!(action.contains(refusal), "{next:#}");
    let check = fixture.run(&["change", "check", "bug-records"]);
    let said = String::from_utf8_lossy(&check.stderr).into_owned()
        + &String::from_utf8_lossy(&check.stdout);
    assert!(!check.status.success() && said.contains(refusal), "{said}");
}
