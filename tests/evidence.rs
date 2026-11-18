//! Reference-aware evidence retention and observation labels (REQ-history-001).
//! the age rules are exercised without waiting.

mod common;

use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

const DAY: u64 = 86_400;
const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn digest(content: &str) -> String {
    format!("{:x}", Sha256::digest(content.as_bytes()))
}

/// One fixture observation; artifacts are written with their content.
struct Obs<'a> {
    id: &'a str,
    case: Option<&'a str>,
    environment: &'a str,
    outcome: &'a str,
    method: Option<&'a str>,
    kind: &'a str,
    age_days: u64,
    artifacts: Vec<(&'a str, &'a str)>,
}

impl<'a> Obs<'a> {
    /// A routine automated product-test run of a case.
    fn run(id: &'a str, case: &'a str, outcome: &'a str, age_days: u64) -> Self {
        Self {
            id,
            case: Some(case),
            environment: "local",
            outcome,
            method: Some("automated"),
            kind: "product-test",
            age_days,
            artifacts: vec![],
        }
    }
}

fn write_obs(fixture: &Fixture, obs: &Obs<'_>) {
    let mut fingerprints = BTreeMap::new();
    for (path, content) in &obs.artifacts {
        fixture.write(path, content);
        fingerprints.insert(path.to_string(), digest(content));
    }
    let mut record = json!({
        "schema_version": 2,
        "id": obs.id,
        "criteria": ["LIFE-1"],
        "requirements": ["REQ-cli-001"],
        "contracts": ["specs/cli/cli.spec.md"],
        "kind": obs.kind,
        "outcome": obs.outcome,
        "environment": obs.environment,
        "summary": format!("fixture observation {}", obs.id),
        "artifacts": obs.artifacts.iter().map(|(path, _)| *path).collect::<Vec<_>>(),
        "artifact_fingerprints": fingerprints,
        "observed_at_unix_seconds": now() - obs.age_days * DAY - 60,
        "snapshot": {"revision": "0".repeat(40), "dirty": false, "fingerprint": ZERO},
        "provenance": "fixture",
    });
    if let Some(case) = obs.case {
        record["cases"] = json!([{"id": case, "digest": ZERO}]);
    }
    if let Some(method) = obs.method {
        record["method"] = json!(method);
    }
    fixture.write(
        &format!(".chough/evidence/{}.json", obs.id),
        &serde_json::to_string_pretty(&record).unwrap(),
    );
}

/// REQ-records-009, REQ-cli-012, LIFE-8: an observation the 0.6.0 migration did not
/// carry reads `removed`, naming the migration's receipt; an ID no record ever held reads
/// `unavailable`; neither ever counts as current, passed or verifying evidence. On a
/// 0.5.x project the open session references `n1`, which becomes its entry and reads
/// historical (REQ-cli-021); nothing references `old-pass`, so the migration removes it;
/// and a bug links `old-fail`, which no file holds any more. After the migration the
/// checkpoint references `old-pass` and `never-recorded`.
#[test]
fn removed_and_missing_observations_are_labeled_and_never_count() {
    fn copy(from: &std::path::Path, to: &std::path::Path) {
        fs::create_dir_all(to).unwrap();
        for entry in fs::read_dir(from).unwrap().flatten() {
            let target = to.join(entry.file_name());
            if entry.path().is_dir() {
                copy(&entry.path(), &target);
            } else {
                fs::copy(entry.path(), target).unwrap();
            }
        }
    }
    let fixture = Fixture::new();
    let release = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/releases/v0.5.0/.chough");
    for folder in ["sessions", "history", "bugs"] {
        copy(
            &release.join(folder),
            &fixture.root().join(".chough").join(folder),
        );
    }
    write_obs(&fixture, &Obs::run("n1", "TC-a", "passed", 20));
    write_obs(&fixture, &Obs::run("old-pass", "TC-a", "passed", 41));
    let session = ".chough/sessions/fixture-first.json";
    let mut checkpoint: Value = serde_json::from_str(&fixture.read(session)).unwrap();
    checkpoint["observations"] = json!(["n1"]);
    fixture.write(session, &serde_json::to_string_pretty(&checkpoint).unwrap());
    let bug_file = fs::read_dir(fixture.root().join(".chough/bugs"))
        .unwrap()
        .flatten()
        .map(|entry| format!(".chough/bugs/{}", entry.file_name().to_string_lossy()))
        .find(|name| name.ends_with(".json"))
        .unwrap();
    let mut bug: Value = serde_json::from_str(&fixture.read(&bug_file)).unwrap();
    bug["observations"] = json!(["old-fail"]);
    fixture.write(&bug_file, &serde_json::to_string_pretty(&bug).unwrap());
    let old_bug = bug["id"].as_str().unwrap().to_string();
    fixture.commit("a 0.5.x project");
    let plan = fixture.ok(&["migrate", "--json"]);
    assert_eq!(plan["aliases"]["old-pass"], "removed", "{plan:#}");
    assert_eq!(plan["aliases"]["old-fail"], "removed", "{plan:#}");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let applied = fixture.ok(&[
        "migrate",
        "--apply",
        &digest,
        "--consent",
        "SYNTHETIC-human said migrate it (fixture)",
    ]);
    let receipt = applied["receipt"].as_str().unwrap().to_string();
    fixture.commit("the migrated records");

    // References written after the migration.
    fixture.ok(&[
        "checkpoint",
        "--observation",
        "old-pass",
        "--observation",
        "never-recorded",
    ]);

    // The focused next view lists only the referenced observations that need attention.
    let next = fixture.ok(&["next", "--environment", "local", "--json"]);
    let listed: BTreeMap<String, String> = next["observations"]["items"]
        .as_array()
        .unwrap()
        .iter()
        .map(|entry| {
            (
                entry["id"].as_str().unwrap().to_string(),
                entry["label"].as_str().unwrap().to_string(),
            )
        })
        .collect();
    assert_eq!(listed["old-pass"], "removed", "{next}");
    assert_eq!(listed["never-recorded"], "unavailable", "{next}");
    // PR 5 (F9): migrated front matter names the carried entry, so next lists it by its
    // new name, with the 0.5.x ID `n1` kept as its alias.
    let n1 = plan["aliases"]["n1"]
        .as_str()
        .unwrap()
        .rsplit('/')
        .next()
        .unwrap()
        .trim_end_matches(".md")
        .to_string();
    assert_eq!(listed[&n1], "historical", "{next}");
    assert!(listed.values().all(|label| label != "current"), "{next}");
    // A removed reference names the receipt that records its removal (REQ-records-009).
    let item = |id: &str| {
        next["observations"]["items"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["id"] == id)
            .unwrap_or_else(|| panic!("{id} missing from {next}"))
            .clone()
    };
    assert_eq!(item("old-pass")["receipt"], receipt.as_str(), "{next}");
    assert!(item("never-recorded").get("receipt").is_none(), "{next}");
    let report = fixture.ok(&["status", "--environment", "local"]);
    let references = report["observation_references"].as_array().unwrap();
    let label = |id: &str| {
        references
            .iter()
            .find(|entry| entry["id"] == id)
            .unwrap_or_else(|| panic!("{id} missing from {references:?}"))["label"]
            .clone()
    };
    assert_eq!(label("old-pass"), "removed");
    assert_eq!(label("never-recorded"), "unavailable");
    assert_eq!(label(&n1), "historical");
    let row = |id: &str| {
        references
            .iter()
            .find(|entry| entry["id"] == id)
            .unwrap_or_else(|| panic!("{id} missing from {references:?}"))
            .clone()
    };
    assert_eq!(row("old-pass")["receipt"], receipt.as_str(), "{report:#}");
    assert!(row("never-recorded").get("receipt").is_none(), "{report:#}");
    for observation in report["observations"].as_array().unwrap() {
        assert!(
            !observation["record"]["aliases"]
                .to_string()
                .contains("old-pass"),
            "{observation:#}"
        );
        assert_ne!(observation["label"], "current", "{observation:#}");
    }

    // The bug shows the link it lost as removed, naming the receipt, and never verifies.
    let shown = fixture.ok(&["bug", "show", &old_bug]);
    let lost = shown["observation_labels"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["id"] == "old-fail")
        .unwrap_or_else(|| panic!("{shown:#}"))
        .clone();
    assert_eq!(lost["label"], "removed", "{shown:#}");
    assert_eq!(lost["receipt"], receipt.as_str(), "{shown:#}");
    assert_eq!(shown["verification"], "unverified", "{shown:#}");
    let refused = fixture.fails(&["bug", "update", &old_bug, "--status", "verified"]);
    assert!(refused.contains("passed observation"), "{refused}");
}

#[test]
fn rebuilding_the_knowledge_index_keeps_the_observation_current() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    let run = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "test-local"]);
    let observation = run["observation"].as_str().unwrap().to_string();
    fixture.ok(&["checkpoint", "--observation", &observation]);
    let label = |fixture: &Fixture| {
        fixture.ok(&["status", "--environment", "test-local"])["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["record"]["id"] == observation)
            .map(|entry| entry["label"].as_str().unwrap().to_string())
    };
    assert_eq!(label(&fixture).as_deref(), Some("current"));

    fixture.ok(&["library", "index"]);
    assert_eq!(label(&fixture).as_deref(), Some("current"));
}

/// The `receipts:` maps of a story's front matter, each as its fields in order.
fn story_receipts(text: &str) -> Vec<Vec<(String, String)>> {
    // REQ-history-006: the `## Receipts` section's YAML block, or an older story's front
    // matter.
    let front = match text.split_once("\n## Receipts\n\n```yaml\n") {
        Some((_, rest)) => rest.split_once("```").map_or(rest, |(block, _)| block),
        None => {
            text.strip_prefix("---\n")
                .and_then(|rest| rest.split_once("\n---\n"))
                .expect("a story")
                .0
        }
    };
    let unquote = |value: &str| {
        value
            .strip_prefix('"')
            .and_then(|inner| inner.strip_suffix('"'))
            .map_or_else(
                || value.to_string(),
                |inner| inner.replace("\\\"", "\"").replace("\\\\", "\\"),
            )
    };
    let mut maps: Vec<Vec<(String, String)>> = vec![];
    let mut inside = false;
    for line in front.lines() {
        if line == "receipts:" {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        }
        let (fresh, rest) = if let Some(rest) = line.strip_prefix("  - ") {
            (true, rest)
        } else if let Some(rest) = line.strip_prefix("    ") {
            (false, rest)
        } else {
            break;
        };
        let (key, value) = rest.split_once(": ").expect("a map field");
        if fresh {
            maps.push(vec![]);
        }
        maps.last_mut()
            .expect("a map")
            .push((key.to_string(), unquote(value)));
    }
    maps
}

/// A small PNG with a metadata chunk, as a key screenshot (REQ-records-013).
fn png() -> Vec<u8> {
    let chunk = |kind: &[u8], data: &[u8]| {
        let mut out = (data.len() as u32).to_be_bytes().to_vec();
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        out.extend_from_slice(&[0, 0, 0, 0]);
        out
    };
    let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
    bytes.extend(chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0]));
    bytes.extend(chunk(b"tEXt", b"Comment\0SYNTHETIC"));
    bytes.extend(chunk(b"IDAT", &[7u8; 32]));
    bytes.extend(chunk(b"IEND", &[]));
    bytes
}

/// REQ-records-005 and REQ-cli-023 (t37f's finding): a raw log is named by its entry's
/// ID alone, so an entry's name must be free as a log too. The same case run the same
/// day in a second session takes a clash suffix rather than reuse the first session's
/// log, which keeps its bytes; an explicit ID whose log is taken is refused with nothing
/// written. A case run's body ends its claim once, `(exit 0).`, never `(exit 0)..`.
#[cfg(unix)]
#[test]
fn a_log_name_taken_in_another_session_is_a_clash_and_the_claim_ends_once() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    let first_session = fixture.active_id();
    let run = |fixture: &Fixture| {
        fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "local"])["observation"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let first = run(&fixture);
    let first_log = format!(".chough/local/logs/{first}.log");
    let first_bytes = fixture.read(&first_log);
    let text = fixture.read(&format!(
        ".chough/sessions/{first_session}/evidence/{first}.md"
    ));
    assert!(!text.contains(".."), "a doubled period: {text}");
    assert!(text.contains("(exit 0).**"), "{text}");

    fixture.ok(&[
        "session",
        "new",
        "second",
        "--objective",
        "SYNTHETIC second session",
    ]);
    let second_session = fixture.id_of("second");
    fixture.ok(&["session", "use", &second_session]);
    let second = run(&fixture);
    assert_ne!(
        second, first,
        "the second session reused the first log's name"
    );
    assert!(second.starts_with(&format!("{first}-")), "{second}");
    assert_eq!(
        fixture.read(&first_log),
        first_bytes,
        "the first log changed"
    );
    assert!(fixture.exists(&format!(".chough/local/logs/{second}.log")));
    assert!(fixture.exists(&format!(
        ".chough/sessions/{second_session}/evidence/{second}.md"
    )));

    // An explicit date-slug ID whose log another session holds is refused.
    let taken = "2020-01-01-taken-elsewhere";
    fixture.write(
        &format!(".chough/local/logs/{taken}.log"),
        "SYNTHETIC log\n",
    );
    let output = fixture.run(&[
        "cases",
        "run",
        "TC-cli-0001",
        "--environment",
        "local",
        "--observation-id",
        taken,
    ]);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("identity collision"), "{stderr}");
    assert_eq!(
        fixture.read(&format!(".chough/local/logs/{taken}.log")),
        "SYNTHETIC log\n"
    );
    assert!(!fixture.exists(&format!(
        ".chough/sessions/{second_session}/evidence/{taken}.md"
    )));
}

/// t38b's F3 (REQ-records-005): an external artifact's path is kept on one quoted line
/// of `.chough/local/external-artifacts.toml`, so a path holding a newline or another
/// control character is refused, naming the path escaped, with no entry and nothing
/// recorded; a later read of the map is never broken by one.
#[cfg(unix)]
#[test]
fn an_external_artifact_path_with_a_control_character_is_refused() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let outside = tempfile::tempdir().unwrap();
    let add = |path: &str| {
        fixture.run(&[
            "evidence",
            "add",
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            "human-observation",
            "--outcome",
            "passed",
            "--environment",
            "local",
            "--summary",
            "SYNTHETIC external proof",
            "--artifact",
            path,
        ])
    };
    for name in [
        "SYNTHETIC\nproof.bin",
        "SYNTHETIC\tproof.bin",
        "SYNTHETIC\rproof.bin",
    ] {
        let file = outside.path().join(name);
        fs::write(&file, b"SYNTHETIC proof bytes\n").unwrap();
        let output = add(file.to_str().unwrap());
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{name:?} was accepted");
        assert!(stderr.contains("control character"), "{stderr}");
        assert!(
            stderr.contains(&format!("{:?}", file.to_str().unwrap())),
            "the refusal does not name the path: {stderr}"
        );
        assert!(!fixture.exists(".chough/local/external-artifacts.toml"));
        let session = fixture.active_id();
        assert!(
            fs::read_dir(
                fixture
                    .root()
                    .join(format!(".chough/sessions/{session}/evidence"))
            )
            .map(|entries| entries.count())
            .unwrap_or(0)
                == 0,
            "an entry was written"
        );
    }
    // A plain path still records, and the map reads back.
    let file = outside.path().join("SYNTHETIC-proof.bin");
    fs::write(&file, b"SYNTHETIC proof bytes\n").unwrap();
    let output = add(file.to_str().unwrap());
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let status = fixture.ok(&["status", "--environment", "local"]);
    assert_ne!(
        status["observations"][0]["artifact"], "changed",
        "{status:#}"
    );
}

/// REQ-history-006 (the plan's delta correction 2): a finished session's story holds
/// one receipt per entry, in the order and shape the migration writes: id; aliases as
/// one comma-and-space scalar; claim; result; the command; revision; environment;
/// content; fingerprint; raw_log; case and case_digest; screenshot; observed;
/// duration_seconds; the actor. Each only when known, and nothing else.
#[cfg(unix)]
#[test]
fn a_story_receipt_carries_each_field_its_entry_knows() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe reports healthy\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo healthy: yes\"]\nexpect_output = [\"healthy: yes\"]\n",
    );
    fixture.init();
    fixture.commit("fixture");
    let session = fixture.active_id();
    let run = fixture.ok(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
    let case_entry = run["observation"].as_str().unwrap().to_string();
    // Aliases, as a migration gives an entry its 0.5.x IDs.
    let file = format!(".chough/sessions/{session}/evidence/{case_entry}.md");
    let text = fixture.read(&file);
    assert!(text.contains("\naliases: []\n"), "{text}");
    fixture.write(
        &file,
        &text.replace(
            "\naliases: []\n",
            "\naliases: [OBS-synthetic-a, OBS-synthetic-b]\n",
        ),
    );
    let shot = fixture.root().join("shot.png");
    fs::write(&shot, png()).unwrap();
    let add = |extra: &[&str]| {
        let mut args = vec![
            "evidence",
            "add",
            "--criterion",
            "LIFE-1",
            "--requirement",
            "REQ-cli-001",
            "--contract",
            "specs/cli/cli.spec.md",
            "--kind",
            "human-observation",
            "--outcome",
            "passed",
            "--environment",
            "local",
        ];
        args.extend_from_slice(extra);
        fixture.ok(&args)["observation"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let shot_entry = add(&[
        "--summary",
        "SYNTHETIC screenshot check",
        "--screenshot",
        shot.to_str().unwrap(),
        "--raw-log",
        "SYNTHETIC-ci-run-7",
    ]);
    let plain_entry = add(&["--summary", "SYNTHETIC plain check"]);
    // #286: what an entry was verified against, and its stated limits.
    let limited_entry = add(&[
        "--summary",
        "SYNTHETIC limited check",
        "--limitation",
        "SYNTHETIC only the first page was read",
        "--limitation",
        "SYNTHETIC no network",
    ]);
    let screenshot = fixture
        .read(&format!(
            ".chough/sessions/{session}/evidence/{shot_entry}.md"
        ))
        .lines()
        .find_map(|line| line.strip_prefix("screenshot: ").map(str::to_string))
        .expect("the entry names its screenshot");
    fs::remove_file(&shot).unwrap();
    fixture.commit("the entries");
    fixture.ok(&["session", "close", &session, "--reason", "SYNTHETIC closed"]);
    let story = fixture.read(&format!(".chough/archive/sessions/{session}.md"));
    let receipts = story_receipts(&story);
    assert_eq!(receipts.len(), 4, "{story}");
    let order = [
        "id",
        "aliases",
        "claim",
        "result",
        "command",
        "revision",
        "environment",
        "content",
        "fingerprint",
        "raw_log",
        "case",
        "case_digest",
        "criteria",
        "requirements",
        "limitations",
        "screenshot",
        "observed",
        "duration_seconds",
        "actor",
    ];
    for receipt in &receipts {
        let keys: Vec<&str> = receipt.iter().map(|(key, _)| key.as_str()).collect();
        let positions: Vec<usize> = keys
            .iter()
            .map(|key| {
                order
                    .iter()
                    .position(|known| known == key)
                    .unwrap_or_else(|| panic!("{key} on a receipt: {story}"))
            })
            .collect();
        assert!(
            positions.windows(2).all(|pair| pair[0] < pair[1]),
            "out of order: {keys:?}"
        );
        for key in [
            "id",
            "claim",
            "result",
            "revision",
            "environment",
            "observed",
        ] {
            assert!(keys.contains(&key), "{key} missing: {keys:?}");
        }
    }
    let receipt = |id: &str| -> BTreeMap<String, String> {
        receipts
            .iter()
            .find(|map| map[0] == ("id".to_string(), id.to_string()))
            .unwrap_or_else(|| panic!("no receipt {id}: {story}"))
            .iter()
            .cloned()
            .collect()
    };
    let case = receipt(&case_entry);
    assert_eq!(case["aliases"], "OBS-synthetic-a, OBS-synthetic-b");
    assert_eq!(case["command"], "sh -c echo healthy: yes");
    assert_eq!(case["environment"], "local");
    assert_eq!(case["result"], "passed");
    assert_eq!(case["case"], "TC-cli-0001");
    assert!(
        case["case_digest"].len() == 64
            && case["case_digest"].bytes().all(|b| b.is_ascii_hexdigit()),
        "{case:?}"
    );
    assert!(case.contains_key("duration_seconds"), "{case:?}");
    assert!(!case.contains_key("screenshot"), "{case:?}");
    let shown = receipt(&shot_entry);
    assert_eq!(shown["screenshot"], screenshot);
    assert_eq!(shown["raw_log"], "SYNTHETIC-ci-run-7");
    assert_eq!(shown["environment"], "local");
    for key in ["aliases", "command", "case", "case_digest"] {
        assert!(!shown.contains_key(key), "{key}: {shown:?}");
    }
    let plain = receipt(&plain_entry);
    for key in [
        "aliases",
        "command",
        "screenshot",
        "raw_log",
        "case",
        "limitations",
    ] {
        assert!(!plain.contains_key(key), "{key}: {plain:?}");
    }
    // #286: a receipt keeps its entry's criteria and requirements, comma-and-space
    // separated, and its limitations, semicolon-and-space separated.
    assert_eq!(plain["criteria"], "LIFE-1");
    assert_eq!(plain["requirements"], "REQ-cli-001");
    let limited = receipt(&limited_entry);
    assert_eq!(limited["criteria"], "LIFE-1");
    assert_eq!(limited["requirements"], "REQ-cli-001");
    assert_eq!(
        limited["limitations"],
        "SYNTHETIC only the first page was read; SYNTHETIC no network"
    );
    assert_eq!(case["criteria"], "LIFE-1");
    assert_eq!(case["requirements"], "REQ-cli-001");
    assert!(
        case["limitations"].contains("inherited environment variables were not recorded"),
        "{case:?}"
    );
}

/// REQ-cli-004 (#203): an entry carries a 0.5.x diagnostic input inventory as front
/// matter and reads it back unchanged, and it never changes freshness; an inventory
/// edited out of #203's rule (a non-canonical or bookkeeping path, a wrong key or a
/// digest that is not SHA-256) refuses the read as invalid evidence.
#[cfg(unix)]
#[test]
fn an_entry_round_trips_its_input_inventory_and_refuses_an_invalid_one() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let added = fixture.ok(&[
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
        "SYNTHETIC inventory carrier",
    ]);
    let file = added["recorded"].as_str().unwrap().to_string();
    let written = fixture.read(&file);
    // The 0.6.0 writer generates no inventory.
    assert!(!written.contains("input_inventory"), "{written}");
    let with = |entries: &str| {
        let (front, body) = written
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .unwrap();
        format!(
            "---\n{front}\ninput_inventory_version: 1\ninput_inventory_complete: false\ninput_inventory:\n{entries}---\n{body}"
        )
    };
    let digest = "a".repeat(64);
    let entry = |path: &str, key: &str, digest: &str| {
        format!("  - domain: 0\n    key: {key}\n    path: {path}\n    digest: {digest}\n")
    };
    fixture.write(
        &file,
        &with(&entry("src/input.txt", "src/input.txt", &digest)),
    );
    let status = fixture.ok(&["status", "--environment", "local"]);
    let row = &status["observations"][0];
    assert_eq!(row["freshness"], "current-observation", "{row:#}");
    assert_eq!(
        row["record"]["input_inventory"],
        json!({"version": 1, "complete": false, "entries": [
            {"domain": 0, "key": "src/input.txt", "path": "src/input.txt", "digest": digest}
        ]}),
        "{row:#}"
    );
    // An edit of the listed file stales the entry by its content identity alone.
    fixture.write("src/input.txt", "SYNTHETIC edit\n");
    let status = fixture.ok(&["status", "--environment", "local"]);
    assert_eq!(
        status["observations"][0]["freshness"],
        "stale-revision-or-inputs"
    );
    for (label, bad) in [
        (
            "non-canonical",
            entry("src//input.txt", "src//input.txt", &digest),
        ),
        (
            "bookkeeping",
            entry(".chough/session.json", ".chough/session.json", &digest),
        ),
        ("wrong key", entry("src/input.txt", "other", &digest)),
        ("digest", entry("src/input.txt", "src/input.txt", "bad")),
    ] {
        fixture.write(&file, &with(&bad));
        let refused = fixture.fails(&["status", "--environment", "local"]);
        assert!(
            refused.contains("invalid evidence entry"),
            "{label}: {refused}"
        );
    }
}

#[cfg(unix)]
fn knowledge_fixture() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        r#"#!/bin/sh
id="$3"
validity=authenticated-history
if [ -f .stub-break ] && [ ! -f "$(cat .stub-break)" ]; then validity=unauthenticated; fi
if [ -f .stub-fail ] && [ "$(cat .stub-fail)" = "$id" ]; then echo "status unavailable" >&2; exit 1; fi
if [ -f .stub-hook ]; then
  # Status reads may run in parallel (REQ-cli-033), so the call count is kept under a lock.
  until mkdir .stub-lock 2>/dev/null; do sleep 0.01; done
  if [ -f .stub-hook ]; then
    calls=$(($(cat .stub-calls 2>/dev/null || echo 0) + 1)); echo "$calls" > .stub-calls
    if [ "$calls" -gt "$(cat .stub-hook-after)" ]; then sh .stub-hook; rm .stub-hook .stub-hook-after .stub-calls; fi
  fi
  rmdir .stub-lock
fi
printf '{"summary":{"id":"%s","state":"archived","terminal_evidence":{"validity":"%s"}},"acceptance_entries":[]}\n' "$id" "$validity"
"#,
    );
    fixture.executable("bin/specsync");
    fixture.write(
        "specs/cli/context.md",
        "# Context\n\nIntro.\n\n## Lessons\n\n- 2026-01-01: a lesson.\n",
    );
    // Five archived changes. The 0.5.x observations their manifests listed for the
    // retired trim are gone (REQ-records-010): a 0.5.x file would make every command
    // refuse as unmigrated.
    for (index, id) in ["a1", "a2", "a3", "a4", "a5"].iter().enumerate() {
        let dir = format!(".specsync/archive/changes/2026-0{}-01-{id}", index + 1);
        fixture.write(
            &format!("{dir}/finalization.json"),
            &format!(
                "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {}, \"implementation_commit\": \"{}\"}}\n",
                (index + 1) * 100,
                "1".repeat(40)
            ),
        );
        let mut entries: Vec<Value> =
            vec![json!({"path": "specs/cli/cli.spec.md", "kind": "file", "payload_digest": ZERO})];
        if *id == "a5" {
            // Real cumulative manifests exceed the 64 KiB structured-record bound.
            for index in 0..800 {
                entries.push(json!({"path": format!("docs/padding/{index:04}.md"), "kind": "file", "payload_digest": ZERO}));
            }
        }
        fixture.write(
            &format!("{dir}/verification.json"),
            &serde_json::to_string_pretty(&json!({
                "commit": "2".repeat(40),
                "passed": true,
                "requirement_ids": ["REQ-cli-001"],
                "acceptance_manifest": {"schema_version": 1, "entries": entries},
            }))
            .unwrap(),
        );
        fixture.write(
            &format!("{dir}/change.md"),
            &format!(
                "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- {id} works\n- {id} is documented\n\n## Other\n\n- not a criterion\n",
                "3".repeat(40)
            ),
        );
        fixture.write(
            &format!("{dir}/lesson-bundle.md"),
            &format!("# Lesson bundle: {id}\n"),
        );
    }
    fixture.init();
    fixture.commit("fixture");
    fixture
}

#[test]
fn an_empty_change_directory_left_by_a_branch_switch_is_not_an_open_change() {
    let fixture = knowledge_fixture();
    fixture.ok(&["library", "index"]);
    let before = fixture.read("knowledge/index.md");
    fs::create_dir_all(fixture.root().join(".specsync/changes/left-behind/deltas")).unwrap();
    fixture.ok(&["library", "index", "--check"]);
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    assert_eq!(index, before);
    assert!(!index.contains("left-behind"), "{index}");
    let next = fixture.ok(&["next", "--json"]);
    assert!(!next.to_string().contains("left-behind"), "{next:#}");
    fixture.ok(&["gate", "list", "--json"]);
}

/// `chough <args>` in `fixture` as a child process the test can watch, with the same
/// PATH, actor and identity isolation as `Fixture::run`.
fn spawnable(fixture: &Fixture, args: &[&str]) -> std::process::Command {
    let mut command = std::process::Command::new(env!("CARGO_BIN_EXE_chough"));
    command.arg("--root").arg(fixture.root()).args(args);
    let mut paths = vec![fixture.root().join("bin")];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    command.env("PATH", std::env::join_paths(paths).unwrap());
    for name in common::ACTOR_VARIABLES {
        command.env_remove(name);
    }
    command.env("XDG_CONFIG_HOME", fixture.root().join(common::USER_CONFIG));
    command
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());
    command
}

/// `evidence add` arguments for a passed human observation with this summary.
fn add_args(summary: &str) -> Vec<String> {
    [
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        summary,
    ]
    .iter()
    .map(|part| part.to_string())
    .collect()
}

/// The active session's folder, relative to the project.
fn session_folder(fixture: &Fixture) -> String {
    let sessions = fixture.root().join(".chough/sessions");
    let mut names: Vec<String> = fs::read_dir(&sessions)
        .unwrap()
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    assert_eq!(names.len(), 1, "{names:?}");
    format!(".chough/sessions/{}", names.remove(0))
}

/// REQ-cli-040 (#317), TC-cli-evidence-writers-reread-under-the-lock: `evidence add`
/// reads the session it changes under the project's writer lock, so an update another
/// writer saves while it waits for the lock survives its save. The test holds the lock,
/// lets `evidence add` block on it (before it writes anything, #325), changes the
/// session's next action as another writer would, and then releases the lock.
#[cfg(unix)]
#[test]
fn evidence_add_rereads_the_session_under_the_writer_lock() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let folder = session_folder(&fixture);
    let lock_path = common::checkpoint_lock_path(fixture.root()).unwrap();
    fs::create_dir_all(lock_path.parent().unwrap()).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap();
    <fs::File as fs4::FileExt>::lock(&lock).unwrap();

    let args = add_args("SYNTHETIC entry saved after the wait");
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let mut add = spawnable(&fixture, &args).spawn().unwrap();
    // The evidence save takes the lock before it writes anything (#325), so it is
    // blocked well within this second.
    std::thread::sleep(std::time::Duration::from_millis(1_000));
    assert!(
        add.try_wait().unwrap().is_none(),
        "evidence add did not wait for the writer lock"
    );

    // Another writer's update, saved while evidence add waits.
    let record = format!("{folder}/session.md");
    let text = fixture.read(&record);
    let next = text
        .lines()
        .find(|line| line.starts_with("next: "))
        .expect("a next action")
        .to_string();
    fixture.write(
        &record,
        &text.replacen(
            &next,
            "next: SYNTHETIC next action saved by another writer",
            1,
        ),
    );
    <fs::File as fs4::FileExt>::unlock(&lock).unwrap();

    let output = add.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = fixture.read(&record);
    assert!(
        text.contains("SYNTHETIC next action saved by another writer"),
        "the other writer's update was lost:\n{text}"
    );
    assert_eq!(
        fixture.checkpoint()["observations"]
            .as_array()
            .unwrap()
            .len(),
        1,
        "{text}"
    );
}

/// REQ-cli-040 (#317): ten `checkpoint --decision` and ten `evidence add` commands at
/// once each apply their change to the latest session, so all ten decisions and all ten
/// observation references are kept.
#[cfg(unix)]
#[test]
fn parallel_decisions_and_evidence_adds_all_land() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let mut children = vec![];
    for number in 1..=10 {
        let decision = format!("decided: SYNTHETIC storm decision {number}");
        children.push(
            spawnable(&fixture, &["checkpoint", "--decision", &decision])
                .spawn()
                .unwrap(),
        );
        let args = add_args(&format!("SYNTHETIC storm entry {number}"));
        let args: Vec<&str> = args.iter().map(String::as_str).collect();
        children.push(spawnable(&fixture, &args).spawn().unwrap());
    }
    for child in children {
        let output = child.wait_with_output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let text = fixture.read(&format!("{}/session.md", session_folder(&fixture)));
    assert_eq!(
        text.matches("SYNTHETIC storm decision ").count(),
        10,
        "decisions were lost:\n{text}"
    );
    assert_eq!(
        fixture.checkpoint()["observations"]
            .as_array()
            .unwrap()
            .len(),
        10,
        "observation references were lost:\n{text}"
    );
}

/// REQ-cli-040 (#317), TC-cli-bug-add-holds-the-writer-lock: `bug add` given an
/// observation of the active session takes the project's writer lock before it reads
/// anything, so it writes no bug while another writer holds the lock, and then adds the
/// bug to the session without losing that writer's update. The test holds the lock,
/// starts `bug add`, changes the session's next action as another writer would, and
/// releases the lock.
#[cfg(unix)]
#[test]
fn bug_add_holds_the_writer_lock_and_keeps_another_writers_update() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let folder = session_folder(&fixture);
    let mut failed = add_args("SYNTHETIC failing check");
    let at = failed.iter().position(|part| part == "passed").unwrap();
    failed[at] = "failed".to_string();
    let failed: Vec<&str> = failed.iter().map(String::as_str).collect();
    let observation = fixture.ok(&failed)["observation"]
        .as_str()
        .expect("the failed observation's ID")
        .to_string();

    let lock_path = common::checkpoint_lock_path(fixture.root()).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap();
    <fs::File as fs4::FileExt>::lock(&lock).unwrap();
    let mut add = spawnable(
        &fixture,
        &[
            "bug",
            "add",
            "--title",
            "SYNTHETIC check fails at the boundary",
            "--symptom",
            "The check fails",
            "--trigger",
            "Running the check",
            "--expected",
            "It passes",
            "--actual",
            "It fails",
            "--environment",
            "local",
            "--observation",
            &observation,
        ],
    )
    .spawn()
    .unwrap();
    // bug add takes the lock before it reads anything, so it waits and writes nothing.
    let bugs = fixture.root().join(".chough/bugs");
    std::thread::sleep(std::time::Duration::from_millis(1_000));
    assert!(
        add.try_wait().unwrap().is_none(),
        "bug add did not wait for the writer lock"
    );
    assert_eq!(
        fs::read_dir(&bugs).map_or(0, |entries| entries.count()),
        0,
        "a bug was written under another writer's lock"
    );

    let record = format!("{folder}/session.md");
    let text = fixture.read(&record);
    let next = text
        .lines()
        .find(|line| line.starts_with("next: "))
        .expect("a next action")
        .to_string();
    fixture.write(
        &record,
        &text.replacen(
            &next,
            "next: SYNTHETIC next action saved by another writer",
            1,
        ),
    );
    <fs::File as fs4::FileExt>::unlock(&lock).unwrap();

    let output = add.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = fixture.read(&record);
    assert!(
        text.contains("SYNTHETIC next action saved by another writer"),
        "the other writer's update was lost:\n{text}"
    );
    assert_eq!(
        fixture.checkpoint()["bugs"].as_array().unwrap().len(),
        1,
        "the bug did not join the session:\n{text}"
    );
}

/// REQ-cli-040 (#325), TC-cli-a-busy-evidence-add-leaves-nothing: `evidence add` takes
/// the project's writer lock before it writes anything, so while another process holds
/// the lock it writes no entry, its `refused:checkpoint-busy` after the wait leaves every
/// file unchanged, and a rerun once the lock is free records one entry, not two.
#[cfg(unix)]
#[test]
fn a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let folder = session_folder(&fixture);
    let evidence = fixture.root().join(&folder).join("evidence");
    let entries = || fs::read_dir(&evidence).map_or(0, |entries| entries.count());
    let lock_path = common::checkpoint_lock_path(fixture.root()).unwrap();
    fs::create_dir_all(lock_path.parent().unwrap()).unwrap();
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap();
    <fs::File as fs4::FileExt>::lock(&lock).unwrap();
    let before = fs::read_to_string(fixture.root().join(&folder).join("session.md")).unwrap();

    let args = add_args("SYNTHETIC entry while the lock is held");
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let mut add = spawnable(&fixture, &args).spawn().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1_000));
    assert!(
        add.try_wait().unwrap().is_none(),
        "evidence add did not wait for the writer lock"
    );
    assert_eq!(
        entries(),
        0,
        "an entry was written before the lock was taken"
    );

    let output = add.wait_with_output().unwrap();
    let said = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "evidence add was saved: {said}");
    assert!(said.contains("refused:checkpoint-busy"), "{said}");
    assert_eq!(entries(), 0, "the refused evidence add left its entry");
    assert_eq!(
        fs::read_to_string(fixture.root().join(&folder).join("session.md")).unwrap(),
        before
    );

    <fs::File as fs4::FileExt>::unlock(&lock).unwrap();
    let output = spawnable(&fixture, &args).output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(entries(), 1, "the rerun did not record exactly one entry");
}
