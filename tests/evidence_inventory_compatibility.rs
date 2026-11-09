//! Reader-only diagnostic inventory compatibility, REQ-cli-004/006.
//! Nine historical originals are immutable data, never re-recorded as fresh runs.
//! Common fixture native/HI stand-ins isolate record parsing; they are TEST inputs,
//! not claims of native scope approval or real human review.
#![cfg(unix)]
mod common;
use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

const ORIGINALS: &[(&str, &str, usize)] = &[
    ("OBS-tc-cli-release-corpus-identity-ef1726110e673bd961ec3f56.json", "eef88566047783ad44c244fb205e6021ec8a502cfbf89b987d9daf459f3c1d84", 33157),
    ("OBS-tc-cli-release-edited-preservation-0e13b37adb18b0e04c90dd23.json", "3feb28f66e781a02414fbac8f1ebabff8ec8eca51387eb4fa1a29fc3180c3f36", 33162),
    ("OBS-tc-cli-release-host-current-missing-ee55b3fe9e5380440663ed62.json", "d17e7975311ca456d098357e733d3be9ac271224262fdcde24410151af68c1f1", 33157),
    ("OBS-tc-cli-release-legacy-policy-9aaea89ee334873bcfa7b314.json", "60711e54532e5e287c07bd1c08138fb3de371c1dea803f684bf5d47842c4c460", 33155),
    ("OBS-tc-cli-release-name-byte-matching-9218194092c97b28451038d3.json", "8cefa2683f41815e013f85bcb1c127a3cec94aa5875dd809f7689c78b7e3fb45", 33161),
    ("OBS-tc-cli-release-pristine-status-3d85d0c9a5a4d2c1afd63fe7.json", "d6c3f27880495cca8de70287170d93e670705fb6a08707e121b128d88263d5ff", 33146),
    ("OBS-tc-cli-release-pristine-status-82d60293ee2debd19108f65b.json", "006be4baf62fb33f8c008003e4be964a256fb383f78619952bc183cf3881a6fb", 33150),
    ("OBS-tc-cli-release-pristine-update-0fe36d77c0cce7dc9abb0619.json", "eb7f6638c470816b1624c7e0ef4fc9002bb05bb9270568c93b20c9a71d3b3136", 33159),
    ("OBS-tc-cli-release-pristine-update-15820c959f00e33dec5a751e.json", "042eb1f89d752370bb739a02242c549476f57f08b656e64c2115f2eb7c3fbd30", 33163),
];
fn corpus() -> Vec<(String, Vec<u8>, Value)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/evidence-inventory");
    let count = fs::read_dir(&root).unwrap().count();
    assert_eq!(count, 9, "no extra or missing historical records");
    ORIGINALS.iter().map(|(name, sha, length)| {
        let bytes = fs::read(root.join(name)).unwrap();
        assert_eq!(bytes.len(), *length);
        assert_eq!(format!("{:x}", Sha256::digest(&bytes)), *sha);
        let record: Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(record["schema_version"], 2);
        assert_eq!(record["snapshot"]["input_inventory"]["version"], 1);
        assert_eq!(record["snapshot"]["input_inventory"]["complete"], false);
        assert_eq!(record["snapshot"]["input_inventory"]["entries"].as_array().unwrap().len(), 128);
        (name.to_string(), bytes, record)
    }).collect()
}
fn fixture() -> (Fixture, String, Value) {
    // Assert every immutable original before invoking any child CLI.
    assert_eq!(corpus().len(), 9);
    let f = Fixture::new();
    f.stubs();
    f.init();
    f.commit("TEST valid inventory-reader baseline");
    f.ok(&["evidence", "add", "--criterion", "LIFE-1", "--requirement", "REQ-cli-001",
        "--contract", "specs/cli/cli.spec.md", "--kind", "product-test", "--outcome", "passed",
        "--environment", "local", "--summary", "TEST current baseline, no inventory"]);
    let files: Vec<_> = fs::read_dir(f.root().join(".chough/evidence")).unwrap().collect();
    assert_eq!(files.len(), 1);
    let name = files[0].as_ref().unwrap().file_name().to_string_lossy().to_string();
    let record: Value = serde_json::from_slice(&fs::read(f.root().join(".chough/evidence").join(&name)).unwrap()).unwrap();
    assert!(record["snapshot"].get("input_inventory").is_none(), "writer policy stays unchanged");
    let status = f.ok(&["status", "--environment", "local"]);
    assert_eq!(status["observations"][0]["freshness"], "current-observation", "legacy valid setup must pass before inventory is introduced");
    assert_eq!(status["observations"][0]["label"], "current");
    (f, name, record)
}
fn write(f: &Fixture, name: &str, value: &Value) {
    f.write(&format!(".chough/evidence/{name}"), &serde_json::to_string(value).unwrap());
}
fn row(f: &Fixture, name: &str, value: &Value) -> Value {
    write(f, name, value);
    let before = f.read(&format!(".chough/evidence/{name}"));
    let status = f.ok(&["status", "--environment", "local"]);
    assert_eq!(before, f.read(&format!(".chough/evidence/{name}")), "reader must write nothing");
    let rows = status["observations"].as_array().unwrap();
    assert_eq!(rows.len(), 1);
    rows[0].clone()
}
fn entry(path: &str) -> Value {
    json!({"domain": 0, "key": path, "path": path, "digest": "a".repeat(64)})
}
fn inventory(entries: Vec<Value>, complete: bool) -> Value {
    json!({"version":1,"complete":complete,"entries":entries})
}
fn refuse(f: &Fixture, name: &str, baseline: &Value, inv: Value, label: &str) {
    let mut value = baseline.clone();
    value["snapshot"]["input_inventory"] = inv;
    write(f,name,&value);
    let before = f.read(&format!(".chough/evidence/{name}"));
    let out = f.run(&["status", "--environment", "local"]);
    assert!(!out.status.success(), "{label}: invalid inventory accepted: {}", String::from_utf8_lossy(&out.stdout));
    assert!(!out.stderr.is_empty(), "{label}: refusal must be visible");
    assert_eq!(before, f.read(&format!(".chough/evidence/{name}")), "{label}: refusal mutated record");
}

#[test]
fn inventory_exact_nine_originals_stay_historical_partial_and_byte_identical() {
    let originals = corpus();
    let (f, current_name, current) = fixture();
    fs::remove_file(f.root().join(".chough/evidence").join(current_name)).unwrap();
    for (name, bytes, record) in &originals {
        assert_ne!(record["snapshot"]["content"], current["snapshot"]["content"], "historical discriminator must be real");
        fs::write(f.root().join(".chough/evidence").join(name), bytes).unwrap();
    }
    let report = f.ok(&["status", "--environment", "local-macos-arm64"]);
    let rows=report["observations"].as_array().unwrap();
    assert_eq!(rows.len(),9);
    assert_eq!(rows.iter().filter(|r|r["record"]["outcome"]=="failed").count(),2);
    assert_eq!(rows.iter().filter(|r|r["record"]["outcome"]=="passed").count(),7);
    for (name,bytes,record) in originals {
        let r=rows.iter().find(|r|r["record"]["id"]==record["id"]).unwrap();
        assert_eq!(r["label"],"historical");
        assert_eq!(r["freshness"],"stale-revision-or-inputs");
        assert_eq!(r["record"],record,"every evidentiary value/provenance and partial inventory retained");
        assert_eq!(r["record"]["snapshot"]["input_inventory"]["complete"],false);
        assert_eq!(fs::read(f.root().join(".chough/evidence").join(name)).unwrap(),bytes);
    }
}
#[test]
fn inventory_absent_null_and_advisory_data_never_change_authoritative_freshness() {
    let (f,name,baseline)=fixture();
    let mut v=baseline.clone();
    assert_eq!(row(&f,&name,&v)["freshness"],"current-observation");
    v["snapshot"]["input_inventory"]=Value::Null;
    assert_eq!(row(&f,&name,&v)["freshness"],"current-observation");
    for complete in [false,true] {
        v["snapshot"]["input_inventory"]=inventory(vec![entry("src/input.txt")],complete);
        let r=row(&f,&name,&v);
        assert_eq!(r["freshness"],"current-observation","diagnostic digest intentionally differs from actual bytes");
        assert_eq!(r["record"]["snapshot"]["input_inventory"],v["snapshot"]["input_inventory"]);
    }
    f.write("src/input.txt","changed authoritative input\n");
    for inv in [Value::Null,inventory(vec![],false),inventory(vec![entry("src/input.txt")],true)] {
        v["snapshot"]["input_inventory"]=inv;
        let r=row(&f,&name,&v);
        assert_eq!(r["freshness"],"stale-revision-or-inputs");
        assert_eq!(r["label"],"historical","inventory must never promote a stale pass");
    }
}
#[test]
fn inventory_strict_schema_versions_and_unsafe_semantics_refuse() {
    let (f,name,baseline)=fixture();
    let good=inventory(vec![entry("src/input.txt")],false);
    assert_eq!(row(&f,&name,&baseline)["freshness"],"current-observation");
    let mut bads=vec![("not-object",json!([])),("missing-fields",json!({})),("wrong-complete",json!({"version":1,"complete":"false","entries":[]}))];
    for version in [json!(0),json!(2),json!(-1),json!(1.0),json!("1")] {
        let mut v=good.clone();v["version"]=version;bads.push(("version",v));
    }
    let mut v=good.clone();v["extra"]=json!(true);bads.push(("inventory-extra",v));
    let mut v=good.clone();v["entries"][0]["extra"]=json!(true);bads.push(("entry-extra",v));
    let mut v=good.clone();v["entries"]=json!([entry("src/input.txt"),entry("src/input.txt")]);bads.push(("duplicate-path",v));
    for path in ["","../escape","/absolute","src/../escape",".chough/session.json","target/out"] {
        bads.push(("unsafe-or-excluded-path",inventory(vec![entry(path)],false)));
    }
    for (field,value) in [("digest",json!("bad")),("digest",json!("A".repeat(64))),("key",json!("different")),("domain",json!(1)),("domain",json!(256)),("path",Value::Null)] {
        let mut v=good.clone();v["entries"][0][field]=value;bads.push(("entry-field",v));
    }
    for (label,inv) in bads {refuse(&f,&name,&baseline,inv,label);}
}
#[test]
fn inventory_entry_and_serialized_byte_caps_are_exact() {
    let (f,name,baseline)=fixture();
    let mut v=baseline.clone();
    let entries:Vec<_>=(0..128).map(|i|entry(&format!("file{i}"))).collect();
    v["snapshot"]["input_inventory"]=inventory(entries.clone(),false);
    assert_eq!(row(&f,&name,&v)["freshness"],"current-observation");
    let mut too_many=entries;too_many.push(entry("file128"));
    refuse(&f,&name,&baseline,inventory(too_many,false),"129 entries");
    let mut inv=inventory(vec![entry("x")],false);
    let base=serde_json::to_vec(&inv).unwrap().len();
    assert_eq!((32768-base)%2,0,"path and matching key add two bytes per character");
    let path="x".repeat(1+(32768-base)/2);
    inv["entries"][0]=entry(&path);
    assert_eq!(serde_json::to_vec(&inv).unwrap().len(),32768);
    v["snapshot"]["input_inventory"]=inv.clone();
    assert_eq!(row(&f,&name,&v)["freshness"],"current-observation");
    inv["complete"]=json!(true); // One byte shorter; grow path+key by two.
    inv["entries"][0]=entry(&(path+"x"));
    assert_eq!(serde_json::to_vec(&inv).unwrap().len(),32769);

    refuse(&f,&name,&baseline,inv,"inventory byte cap");
}
#[test]
fn inventory_change_key_classification_and_writer_omission_are_preserved() {
    let (f,name,baseline)=fixture();
    let mut v=baseline.clone();
    v["snapshot"]["input_inventory"]=inventory(vec![json!({"domain":1,"key":"specsync-change/test-change/design.md","path":".specsync/changes/test-change/design.md","digest":"b".repeat(64)})],true);
    assert_eq!(row(&f,&name,&v)["freshness"],"current-observation");
    let mut wrong=v["snapshot"]["input_inventory"].clone();wrong["entries"][0]["key"]=json!(".specsync/changes/test-change/design.md");
    refuse(&f,&name,&baseline,wrong,"wrong change domain key");
    // No change to record schema or producer: the fresh baseline has no inventory.
    assert!(baseline["snapshot"].get("input_inventory").is_none());
    assert_eq!(baseline["schema_version"],2);
}

#[test]
fn inventory_legacy_schema_one_and_environment_rules_remain_authoritative() {
    let (f,name,baseline)=fixture();
    let mut legacy=baseline.clone();
    legacy["schema_version"]=json!(1);
    legacy["snapshot"].as_object_mut().unwrap().remove("content");
    assert_eq!(row(&f,&name,&legacy)["freshness"],"current-observation");
    legacy["snapshot"]["input_inventory"]=Value::Null;
    assert_eq!(row(&f,&name,&legacy)["freshness"],"current-observation");
    let mut modern=baseline;
    modern["snapshot"]["input_inventory"]=inventory(vec![],true);
    write(&f,&name,&modern);
    let unchecked=f.ok(&["status"]);
    assert_eq!(unchecked["observations"][0]["freshness"],"environment-unchecked");
    let mismatch=f.ok(&["status","--environment","another"]);
    assert_eq!(mismatch["observations"][0]["freshness"],"stale-environment");
    // An empty commit leaves modern content current but stales revision-bound schema1.
    f.git(&["commit","--allow-empty","-m","TEST ancestry only"]);
    assert_eq!(row(&f,&name,&modern)["freshness"],"current-observation");
    assert_eq!(row(&f,&name,&legacy)["freshness"],"stale-revision-or-inputs");
}
