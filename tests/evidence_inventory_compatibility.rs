//! Reader-only diagnostic inventory compatibility, REQ-cli-004/006, in 0.6.0: the nine
//! historical originals stay immutable, byte-pinned data. Each is carried from 0.5.x by
//! `chough migrate` (REQ-records-009) into an entry of the open session that references
//! it, and live `status` then reports the freshness 0.5.x reported. An invalid inventory
//! refuses the migration's preview, naming the file and writing nothing, as 0.5.x
//! `status` refused it. A valid one stays on the entry as diagnostic metadata and never
//! changes freshness. Common fixture native/HI stand-ins isolate record parsing; they are
//! TEST inputs, not claims of native scope approval or real human review.
#![cfg(unix)]
mod common;
use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const ORIGINALS: &[(&str, &str, usize)] = &[
    (
        "OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56.json",
        "eef88566047783ad44c244fb205e6021ec8a502cfbf89b987d9daf459f3c1d84",
        33157,
    ),
    (
        "OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23.json",
        "3feb28f66e781a02414fbac8f1ebabff8ec8eca51387eb4fa1a29fc3180c3f36",
        33162,
    ),
    (
        "OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62.json",
        "d17e7975311ca456d098357e733d3be9ac271224262fdcde24410151af68c1f1",
        33157,
    ),
    (
        "OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314.json",
        "60711e54532e5e287c07bd1c08138fb3de371c1dea803f684bf5d47842c4c460",
        33155,
    ),
    (
        "OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3.json",
        "8cefa2683f41815e013f85bcb1c127a3cec94aa5875dd809f7689c78b7e3fb45",
        33161,
    ),
    (
        "OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7.json",
        "d6c3f27880495cca8de70287170d93e670705fb6a08707e121b128d88263d5ff",
        33146,
    ),
    (
        "OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b.json",
        "006be4baf62fb33f8c008003e4be964a256fb383f78619952bc183cf3881a6fb",
        33150,
    ),
    (
        "OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619.json",
        "eb7f6638c470816b1624c7e0ef4fc9002bb05bb9270568c93b20c9a71d3b3136",
        33159,
    ),
    (
        "OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e.json",
        "042eb1f89d752370bb739a02242c549476f57f08b656e64c2115f2eb7c3fbd30",
        33163,
    ),
];
fn corpus() -> Vec<(String, Vec<u8>, Value)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/evidence-inventory");
    let count = fs::read_dir(&root).unwrap().count();
    assert_eq!(count, 9, "no extra or missing historical records");
    ORIGINALS
        .iter()
        .map(|(name, sha, length)| {
            let bytes = fs::read(root.join(name)).unwrap();
            assert_eq!(bytes.len(), *length);
            assert_eq!(format!("{:x}", Sha256::digest(&bytes)), *sha);
            let record: Value = serde_json::from_slice(&bytes).unwrap();
            assert_eq!(record["schema_version"], 2);
            assert_eq!(record["snapshot"]["input_inventory"]["version"], 1);
            assert_eq!(record["snapshot"]["input_inventory"]["complete"], false);
            assert_eq!(
                record["snapshot"]["input_inventory"]["entries"]
                    .as_array()
                    .unwrap()
                    .len(),
                128
            );
            (name.to_string(), bytes, record)
        })
        .collect()
}

/// The open 0.5.x session of the v0.5.0 release fixture, which references the
/// observations under test.
const SESSION: &str = ".chough/sessions/fixture-first.json";
const BASELINE: &str = "OBS-product-test-00000000000000000000f001";
const CONSENT: &str = "TEST consent relayed for the inventory migration fixture";

/// A 0.5.x project (the v0.5.0 release fixture's sessions and history) and a baseline
/// 0.5.x observation that is current once migrated: its content identity and revision
/// come from `evidence add` on the migrated tree, which the migration's own writes (the
/// knowledge index among them) are part of, after which the project is reset to its
/// 0.5.x commit, `start`.
struct Project {
    f: Fixture,
    baseline: Value,
    start: String,
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

/// Every file of the project except Git's, with its bytes.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.file_name().is_some_and(|name| name == ".git") {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, files);
            } else {
                let name = path.strip_prefix(base).unwrap().display().to_string();
                files.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    walk(root, root, &mut files);
    files
}

fn head(f: &Fixture) -> String {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(f.root())
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

/// A front matter scalar of a record.
fn front_value(text: &str, key: &str) -> Option<String> {
    let (front, _) = text.strip_prefix("---\n")?.split_once("\n---\n")?;
    front
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{key}: ")))
        .map(|value| value.trim_matches('"').to_string())
}

fn project() -> Project {
    // Assert every immutable original before invoking any child CLI.
    assert_eq!(corpus().len(), 9);
    let f = Fixture::new();
    f.stubs();
    let release =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases/v0.5.0/.chough");
    for folder in ["sessions", "history"] {
        copy_dir(
            &release.join(folder),
            &f.root().join(".chough").join(folder),
        );
    }
    f.commit("TEST a 0.5.x project");
    let start = head(&f);
    // The migrated tree's content identity, from a 0.6.0 baseline entry.
    migrate(&f);
    f.ok(&[
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "product-test",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "TEST current baseline, no inventory",
    ]);
    let folder = f
        .root()
        .join(format!(".chough/sessions/{}/evidence", f.active_id()));
    let entries: Vec<_> = fs::read_dir(&folder).unwrap().collect();
    assert_eq!(entries.len(), 1);
    let text = fs::read_to_string(entries[0].as_ref().unwrap().path()).unwrap();
    assert!(
        !text.contains("input_inventory"),
        "writer policy stays unchanged: {text}"
    );
    let status = f.ok(&["status", "--environment", "local"]);
    assert_eq!(
        status["observations"][0]["freshness"], "current-observation",
        "legacy valid setup must pass before inventory is introduced"
    );
    let content = front_value(&text, "content").unwrap();
    let revision = front_value(&text, "revision").unwrap();
    let baseline = json!({
        "schema_version": 2,
        "id": BASELINE,
        "criteria": ["LIFE-1"],
        "requirements": ["REQ-cli-001"],
        "contracts": ["specs/cli/cli.spec.md"],
        "kind": "product-test",
        "outcome": "passed",
        "environment": "local",
        "summary": "TEST current baseline, no inventory",
        "artifacts": [],
        "artifact_fingerprints": {},
        "observed_at_unix_seconds": 1_794_414_607u64,
        "snapshot": {
            "revision": revision,
            "dirty": false,
            "fingerprint": "0".repeat(64),
            "content": content,
        },
        "provenance": "unauthenticated observation; not proof of human approval, test execution, or deployment",
        "method": "automated",
    });
    let p = Project { f, baseline, start };
    prepare(&p, &[]);
    p
}

/// Back to the 0.5.x project, with `records` as its observations, each referenced by the
/// open session, committed.
fn prepare(p: &Project, records: &[(&str, &[u8])]) {
    let f = &p.f;
    f.git(&["reset", "-q", "--hard", &p.start]);
    f.git(&["clean", "-q", "-fdx"]);
    let mut ids = vec![];
    for (name, bytes) in records {
        let path = f.root().join(".chough/evidence").join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, bytes).unwrap();
        ids.push(name.trim_end_matches(".json").to_string());
    }
    let mut session: Value = serde_json::from_str(&f.read(SESSION)).unwrap();
    session["observations"] = json!(ids);
    f.write(
        SESSION,
        &(serde_json::to_string_pretty(&session).unwrap() + "\n"),
    );
    f.commit("TEST the observations under test");
}

/// Preview and apply the migration; returns the plan.
fn migrate(f: &Fixture) -> Value {
    let plan = f.ok(&["migrate"]);
    assert_eq!(plan["unaccounted"], json!([]), "{plan:#}");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let applied = f.ok(&["migrate", "--apply", &digest, "--consent", CONSENT]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    plan
}

fn name_of(value: &Value) -> String {
    format!("{}.json", value["id"].as_str().unwrap())
}

/// Migrate `value` as the session's one observation, then its live status row; status
/// writes nothing.
fn row(p: &Project, value: &Value) -> Value {
    let bytes = serde_json::to_vec(value).unwrap();
    prepare(p, &[(&name_of(value), &bytes)]);
    migrate(&p.f);
    status_row(p, &["status", "--environment", "local"])
}

/// The one observation row of `args`'s status report; the report writes nothing.
fn status_row(p: &Project, args: &[&str]) -> Value {
    let before = tree(p.f.root());
    let status = p.f.ok(args);
    assert_eq!(before, tree(p.f.root()), "reader must write nothing");
    let rows = status["observations"].as_array().unwrap();
    assert_eq!(rows.len(), 1, "{status:#}");
    rows[0].clone()
}

fn entry(path: &str) -> Value {
    json!({"domain": 0, "key": path, "path": path, "digest": "a".repeat(64)})
}
fn inventory(entries: Vec<Value>, complete: bool) -> Value {
    json!({"version":1,"complete":complete,"entries":entries})
}

/// An invalid inventory refuses the migration's preview, naming the record, and the
/// project keeps every byte, the 0.5.x record included.
fn refuse(p: &Project, baseline: &Value, inv: Value, label: &str) {
    let mut value = baseline.clone();
    value["snapshot"]["input_inventory"] = inv;
    let name = name_of(&value);
    prepare(p, &[(&name, &serde_json::to_vec(&value).unwrap())]);
    let before = tree(p.f.root());
    let out = p.f.run(&["migrate"]);
    assert!(
        !out.status.success(),
        "{label}: invalid inventory accepted: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains(&name),
        "{label}: refusal must name the record: {stderr}"
    );
    assert_eq!(
        before,
        tree(p.f.root()),
        "{label}: refusal mutated the project"
    );
}

#[test]
fn inventory_exact_nine_originals_stay_historical_partial_and_byte_identical() {
    let originals = corpus();
    let p = project();
    let records: Vec<(&str, &[u8])> = originals
        .iter()
        .map(|(name, bytes, _)| (name.as_str(), bytes.as_slice()))
        .collect();
    prepare(&p, &records);
    for (_, _, record) in &originals {
        assert_ne!(
            record["snapshot"]["content"], p.baseline["snapshot"]["content"],
            "historical discriminator must be real"
        );
    }
    let plan = migrate(&p.f);
    let before = tree(p.f.root());
    let report = p.f.ok(&["status", "--environment", "local-macos-arm64"]);
    assert_eq!(before, tree(p.f.root()), "reader must write nothing");
    let rows = report["observations"].as_array().unwrap();
    assert_eq!(rows.len(), 9);
    assert_eq!(
        rows.iter()
            .filter(|r| r["record"]["outcome"] == "failed")
            .count(),
        2
    );
    assert_eq!(
        rows.iter()
            .filter(|r| r["record"]["outcome"] == "passed")
            .count(),
        7
    );
    for (_, _, record) in &originals {
        let id = record["id"].as_str().unwrap();
        let r = rows
            .iter()
            .find(|r| r["record"]["aliases"][0] == id)
            .unwrap_or_else(|| panic!("{id} is not an entry: {report:#}"));
        assert_ne!(r["label"], "current", "{r:#}");
        assert_eq!(r["freshness"], "stale-revision-or-inputs", "{r:#}");
        // Every evidentiary value and the partial inventory are retained.
        let carried = &r["record"];
        for key in [
            "criteria",
            "requirements",
            "contracts",
            "kind",
            "outcome",
            "environment",
            "summary",
            "observed_at_unix_seconds",
            "method",
            "track",
            "limitations",
        ] {
            assert_eq!(carried[key], record[key], "{id}: {key}");
        }
        assert_eq!(carried["revision"], record["snapshot"]["revision"], "{id}");
        assert_eq!(carried["content"], record["snapshot"]["content"], "{id}");
        assert_eq!(
            carried["input_inventory"], record["snapshot"]["input_inventory"],
            "{id}: the partial inventory is retained"
        );
        assert_eq!(carried["input_inventory"]["complete"], false, "{id}");
        assert!(
            plan["sources"]
                .get(format!(".chough/evidence/{id}.json"))
                .is_some(),
            "{id}"
        );
    }
    // The originals themselves are untouched.
    assert_eq!(corpus().len(), 9);
}

#[test]
fn inventory_absent_null_and_advisory_data_never_change_authoritative_freshness() {
    let p = project();
    let mut v = p.baseline.clone();
    assert_eq!(row(&p, &v)["freshness"], "current-observation");
    v["snapshot"]["input_inventory"] = Value::Null;
    assert_eq!(row(&p, &v)["freshness"], "current-observation");
    for complete in [false, true] {
        v["snapshot"]["input_inventory"] = inventory(vec![entry("src/input.txt")], complete);
        let r = row(&p, &v);
        assert_eq!(
            r["freshness"], "current-observation",
            "diagnostic digest intentionally differs from actual bytes"
        );
        assert_eq!(
            r["record"]["input_inventory"],
            v["snapshot"]["input_inventory"]
        );
    }
    // Corrected oracle: the existing SHA validator accepts either ASCII hex case.
    // This token is retained exactly and remains diagnostic, not input authority.
    v["snapshot"]["input_inventory"] = inventory(vec![entry("src/input.txt")], true);
    v["snapshot"]["input_inventory"]["entries"][0]["digest"] = json!("A".repeat(64));
    let uppercase = row(&p, &v);
    assert_eq!(uppercase["freshness"], "current-observation");
    assert_eq!(
        uppercase["record"]["input_inventory"]["entries"][0]["digest"],
        "A".repeat(64)
    );
    p.f.write("src/input.txt", "changed authoritative input\n");
    let uppercase_stale = status_row(&p, &["status", "--environment", "local"]);
    assert_eq!(uppercase_stale["freshness"], "stale-revision-or-inputs");
    assert_ne!(
        uppercase_stale["label"], "current",
        "uppercase metadata cannot promote stale content"
    );
    for inv in [
        Value::Null,
        inventory(vec![], false),
        inventory(vec![entry("src/input.txt")], true),
    ] {
        v["snapshot"]["input_inventory"] = inv;
        row(&p, &v);
        p.f.write("src/input.txt", "changed authoritative input\n");
        let r = status_row(&p, &["status", "--environment", "local"]);
        assert_eq!(r["freshness"], "stale-revision-or-inputs");
        assert_ne!(
            r["label"], "current",
            "inventory must never promote a stale pass"
        );
    }
}

#[test]
fn inventory_strict_schema_versions_and_unsafe_semantics_refuse() {
    let p = project();
    let baseline = p.baseline.clone();
    let good = inventory(vec![entry("src/input.txt")], false);
    assert_eq!(row(&p, &baseline)["freshness"], "current-observation");
    let mut bads = vec![
        ("not-object", json!([])),
        ("missing-fields", json!({})),
        (
            "wrong-complete",
            json!({"version":1,"complete":"false","entries":[]}),
        ),
    ];
    for version in [json!(0), json!(2), json!(-1), json!(1.0), json!("1")] {
        let mut v = good.clone();
        v["version"] = version;
        bads.push(("version", v));
    }
    let mut v = good.clone();
    v["extra"] = json!(true);
    bads.push(("inventory-extra", v));
    let mut v = good.clone();
    v["entries"][0]["extra"] = json!(true);
    bads.push(("entry-extra", v));
    let mut v = good.clone();
    v["entries"] = json!([entry("src/input.txt"), entry("src/input.txt")]);
    bads.push(("duplicate-path", v));
    for path in [
        "",
        "../escape",
        "/absolute",
        "src/../escape",
        ".chough/session.json",
        "target/out",
    ] {
        bads.push((
            "unsafe-or-excluded-path",
            inventory(vec![entry(path)], false),
        ));
    }
    for (field, value) in [
        ("digest", json!("bad")),
        ("key", json!("different")),
        ("domain", json!(1)),
        ("domain", json!(256)),
        ("path", Value::Null),
    ] {
        let mut v = good.clone();
        v["entries"][0][field] = value;
        bads.push(("entry-field", v));
    }
    for (label, inv) in bads {
        refuse(&p, &baseline, inv, label);
    }
}

#[test]
fn inventory_entry_and_serialized_byte_caps_are_exact() {
    let p = project();
    let baseline = p.baseline.clone();
    let mut v = baseline.clone();
    let entries: Vec<_> = (0..128).map(|i| entry(&format!("file{i}"))).collect();
    v["snapshot"]["input_inventory"] = inventory(entries.clone(), false);
    assert_eq!(row(&p, &v)["freshness"], "current-observation");
    let mut too_many = entries;
    too_many.push(entry("file128"));
    refuse(&p, &baseline, inventory(too_many, false), "129 entries");
    let mut inv = inventory(vec![entry("x")], false);
    let base = serde_json::to_vec(&inv).unwrap().len();
    assert_eq!(
        (32768 - base) % 2,
        0,
        "path and matching key add two bytes per character"
    );
    let path = "x".repeat(1 + (32768 - base) / 2);
    inv["entries"][0] = entry(&path);
    assert_eq!(serde_json::to_vec(&inv).unwrap().len(), 32768);
    v["snapshot"]["input_inventory"] = inv.clone();
    assert_eq!(row(&p, &v)["freshness"], "current-observation");
    inv["complete"] = json!(true); // One byte shorter; grow path+key by two.
    inv["entries"][0] = entry(&(path + "x"));
    assert_eq!(serde_json::to_vec(&inv).unwrap().len(), 32769);

    refuse(&p, &baseline, inv, "inventory byte cap");
}

#[test]
fn inventory_change_key_classification_and_writer_omission_are_preserved() {
    let p = project();
    let baseline = p.baseline.clone();
    let mut v = baseline.clone();
    v["snapshot"]["input_inventory"] = inventory(
        vec![
            json!({"domain":1,"key":"specsync-change/test-change/design.md","path":".specsync/changes/test-change/design.md","digest":"b".repeat(64)}),
        ],
        true,
    );
    assert_eq!(row(&p, &v)["freshness"], "current-observation");
    let mut wrong = v["snapshot"]["input_inventory"].clone();
    wrong["entries"][0]["key"] = json!(".specsync/changes/test-change/design.md");
    refuse(&p, &baseline, wrong, "wrong change domain key");
    // No change to record schema or producer: project() asserts the 0.6.0 writer's
    // entry has no inventory, and the 0.5.x baseline carries none.
    assert!(baseline["snapshot"].get("input_inventory").is_none());
    assert_eq!(baseline["schema_version"], 2);
}

#[test]
fn inventory_legacy_schema_one_and_environment_rules_remain_authoritative() {
    let p = project();
    let baseline = p.baseline.clone();
    // A schema 1 record has no content identity: its revision-bound identity cannot
    // survive the commit the migration needs, so it reads stale, and an inventory
    // cannot change that.
    let mut legacy = baseline.clone();
    legacy["schema_version"] = json!(1);
    legacy["snapshot"]
        .as_object_mut()
        .unwrap()
        .remove("content");
    assert_eq!(row(&p, &legacy)["freshness"], "stale-revision-or-inputs");
    legacy["snapshot"]["input_inventory"] = Value::Null;
    assert_eq!(row(&p, &legacy)["freshness"], "stale-revision-or-inputs");
    let mut modern = baseline;
    modern["snapshot"]["input_inventory"] = inventory(vec![], true);
    assert_eq!(row(&p, &modern)["freshness"], "current-observation");
    assert_eq!(
        status_row(&p, &["status"])["freshness"],
        "environment-unchecked"
    );
    assert_eq!(
        status_row(&p, &["status", "--environment", "another"])["freshness"],
        "stale-environment"
    );
    // An empty commit leaves modern content current.
    p.f.git(&["commit", "-q", "--allow-empty", "-m", "TEST ancestry only"]);
    assert_eq!(
        status_row(&p, &["status", "--environment", "local"])["freshness"],
        "current-observation"
    );
}

#[test]
fn inventory_noncanonical_aliases_cannot_claim_distinct_normal_inputs() {
    let p = project();
    let baseline = p.baseline.clone();
    let mut canonical = baseline.clone();
    canonical["snapshot"]["input_inventory"] = inventory(vec![entry("src/input.txt")], true);
    let control = row(&p, &canonical);
    assert_eq!(
        control["freshness"], "current-observation",
        "valid normal path must load before alias discriminator"
    );
    assert_eq!(
        control["record"]["input_inventory"],
        canonical["snapshot"]["input_inventory"]
    );
    // Path::components normalizes these spellings, but raw strings are not unique
    // canonical input names. Reject metadata; never rewrite original OBS bytes.
    for alias in ["src//input.txt", "src/./input.txt", "src/input.txt/"] {
        refuse(&p, &baseline, inventory(vec![entry(alias)], true), alias);
        refuse(
            &p,
            &baseline,
            inventory(vec![entry("src/input.txt"), entry(alias)], true),
            "two spellings for one input",
        );
    }
}

#[test]
fn inventory_noncanonical_alias_cannot_bypass_change_bookkeeping_classification() {
    let p = project();
    let baseline = p.baseline.clone();
    let mut canonical = baseline.clone();
    canonical["snapshot"]["input_inventory"] = inventory(
        vec![
            json!({"domain":1,"key":"specsync-change/test-change/design.md","path":".specsync/changes/test-change/design.md","digest":"b".repeat(64)}),
        ],
        false,
    );
    assert_eq!(
        row(&p, &canonical)["freshness"],
        "current-observation",
        "valid change input must load before alias discriminator"
    );
    refuse(
        &p,
        &baseline,
        inventory(
            vec![entry(".specsync/changes/test-change/state.json")],
            true,
        ),
        "canonical bookkeeping excluded",
    );
    // A repeated separator currently hides the lifecycle path from raw prefix
    // classification; it must not turn excluded bookkeeping into domain0 input.
    for alias in [
        ".specsync//changes/test-change/state.json",
        ".specsync/changes//test-change/state.json",
        ".specsync/./changes/test-change/state.json",
    ] {
        refuse(&p, &baseline, inventory(vec![entry(alias)], true), alias);
    }
}
