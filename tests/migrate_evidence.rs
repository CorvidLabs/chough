//! TC-records-migrate-from-0-5, the evidence part (REQ-records-009, REQ-records-003,
//! REQ-records-005, REQ-records-013, REQ-history-006, REQ-cli-023): `chough migrate`
//! carries each 0.5.x observation an open session references as one of its evidence
//! entries, each one an archived session or a bug references as a receipt, and removes
//! the rest with why; every old ID stays an alias, raw logs leave with their digest kept
//! as the fingerprint, an in-project artifact stays where it is, key screenshots are
//! stripped and bounded, validation runs carry like observations, and what is not
//! carried is one counted gap line per field.

mod common;

use common::Fixture;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

const CONSENT: &str = "SYNTHETIC-human said migrate it (fixture)";
const CASE_RUN: &str = "OBS-tc-fixture-0001-b83687e52f4a30ec930beeda";
const PRODUCT: &str = "OBS-product-test-6853752a8d9a557c1f20b8d8";
const LOG: &str = ".chough/artifacts/OBS-tc-fixture-0001-b83687e52f4a30ec930beeda.log";
const LOG_DIGEST: &str = "1001193e0144d436dc23ca3c8aec883c36000f3f7ef23fddfefbd3cd9d7d5e98";
const ARTIFACT_DIGEST: &str = "f8f605ba28f5e6dbef49c04d41bece946dcefd37231e723c922f5a7f68dc6368";

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

/// A project holding the fixture at `tests/fixtures/<relative>`, not yet committed.
fn project(relative: &str) -> Fixture {
    let fixture = Fixture::new();
    copy_dir(
        &manifest_dir().join("tests/fixtures").join(relative),
        fixture.root(),
    );
    fixture
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

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
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

/// `YYYY-MM-DDTHH:MM:SSZ` for a Unix time.
fn iso(seconds: u64) -> String {
    let rest = seconds % 86_400;
    format!(
        "{}T{:02}:{:02}:{:02}Z",
        utc_date(seconds),
        rest / 3_600,
        rest % 3_600 / 60,
        rest % 60
    )
}

/// A front matter scalar as text: plain, or double-quoted with its escapes undone.
fn unquote(value: &str) -> String {
    if value.starts_with('"') {
        serde_json::from_str(value).unwrap_or_else(|_| value.to_string())
    } else {
        value.to_string()
    }
}

/// The front matter of a record: each top-level key with its raw value, unquoted; a
/// flow list stays as written (`[a, b]`), and a block of maps is left empty.
fn front(text: &str) -> BTreeMap<String, String> {
    let (front, _) = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .unwrap_or_else(|| panic!("no front matter in {text}"));
    front
        .lines()
        .filter(|line| !line.starts_with(' '))
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| (key.to_string(), unquote(value.trim())))
        .collect()
}

/// The items of a flow list value (`[a, b]`), unquoted.
fn items(value: &str) -> Vec<String> {
    value
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(", ")
        .filter(|item| !item.is_empty())
        .map(unquote)
        .collect()
}

/// The maps of a block list in front matter (`key:` then `  - a: 1` / `    b: 2`).
fn maps(text: &str, key: &str) -> Vec<BTreeMap<String, String>> {
    let mut found = vec![];
    let mut inside = false;
    for line in text.lines() {
        if line == "---" && inside {
            break;
        }
        if line == format!("{key}:") {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        }
        if let Some(first) = line.strip_prefix("  - ") {
            found.push(BTreeMap::new());
            if let Some((name, value)) = first.split_once(": ") {
                found
                    .last_mut()
                    .unwrap()
                    .insert(name.to_string(), unquote(value));
            }
        } else if let Some(more) = line.strip_prefix("    ") {
            if let Some((name, value)) = more.split_once(": ") {
                found
                    .last_mut()
                    .unwrap()
                    .insert(name.to_string(), unquote(value));
            }
        } else {
            break;
        }
    }
    found
}

/// Set the observations a 0.5.x checkpoint references.
fn reference(fixture: &Fixture, checkpoint: &str, ids: &[&str]) {
    let mut session: Value = serde_json::from_str(&fixture.read(checkpoint)).unwrap();
    session["observations"] = json!(ids);
    fixture.write(
        checkpoint,
        &(serde_json::to_string_pretty(&session).unwrap() + "\n"),
    );
}

/// Link observations to the one 0.5.x bug of a release fixture; returns its file.
fn link_bug(fixture: &Fixture, ids: &[&str]) -> String {
    let folder = fixture.root().join(".chough/bugs");
    let name = fs::read_dir(&folder)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().to_string())
        .find(|name| name.ends_with(".json"))
        .unwrap();
    let path = format!(".chough/bugs/{name}");
    let mut bug: Value = serde_json::from_str(&fixture.read(&path)).unwrap();
    bug["observations"] = json!(ids);
    fixture.write(&path, &(serde_json::to_string_pretty(&bug).unwrap() + "\n"));
    path
}

/// Write a 0.5.x observation: a copy of `template` with a new ID, summary and time.
fn observation_like(
    fixture: &Fixture,
    template: &str,
    id: &str,
    summary: &str,
    observed: u64,
) -> Value {
    let mut record: Value =
        serde_json::from_str(&fixture.read(&format!(".chough/evidence/{template}.json"))).unwrap();
    record["id"] = json!(id);
    record["summary"] = json!(summary);
    record["observed_at_unix_seconds"] = json!(observed);
    fixture.write(
        &format!(".chough/evidence/{id}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    record
}

fn legacy(fixture: &Fixture, id: &str) -> Value {
    serde_json::from_str(&fixture.read(&format!(".chough/evidence/{id}.json"))).unwrap()
}

/// Preview, then apply with the preview's digest and consent; returns both.
fn migrate(fixture: &Fixture) -> (Value, Value) {
    let plan = fixture.ok(&["migrate"]);
    assert_eq!(plan["unaccounted"], json!([]), "{plan:#}");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let applied = fixture.ok(&["migrate", "--apply", &digest, "--consent", CONSENT]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    (plan, applied)
}

/// The project-relative path the plan gives an old ID.
fn alias(plan: &Value, old: &str) -> String {
    plan["aliases"][old]
        .as_str()
        .unwrap_or_else(|| panic!("no alias for {old}: {plan:#}"))
        .to_string()
}

/// The new session ID a 0.5.x session key migrates to, from the plan's aliases.
fn session_folder(plan: &Value, key: &str) -> String {
    let path = alias(plan, key);
    path.trim_start_matches(".chough/sessions/")
        .trim_start_matches(".chough/archive/sessions/")
        .trim_end_matches("/session.md")
        .trim_end_matches(".md")
        .to_string()
}

fn gap_with(plan: &Value, needle: &str) -> String {
    plan["gaps"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(Value::as_str)
        .find(|gap| gap.contains(needle))
        .unwrap_or_else(|| panic!("no gap naming {needle}: {plan:#}"))
        .to_string()
}

fn reference_label(status: &Value, id: &str) -> String {
    status["observation_references"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["id"] == id)
        .unwrap_or_else(|| panic!("{id} is not referenced: {status:#}"))["label"]
        .as_str()
        .unwrap()
        .to_string()
}

/// v0.5.0: the open session references the case run and the product test with its
/// in-project artifact; the archived session and the bug reference a third observation;
/// a fourth is referenced by nothing. The case run and the product test become entries
/// of the open session, the third a receipt of the story and of the bug, the fourth is
/// removed, aliased `removed`. The raw log leaves and its digest is the fingerprint,
/// written `sha256:<hex>` as REQ-records-005 writes it;
/// `artifact.txt` stays the project's. Every old ID resolves after the apply.
#[test]
fn observations_become_entries_receipts_or_removals_and_every_old_id_resolves() {
    let fixture = project("releases/v0.5.0");
    let archived_id = "OBS-product-test-00000000000000000000a001";
    let stray_id = "OBS-product-test-00000000000000000000a002";
    let observed = legacy(&fixture, PRODUCT)["observed_at_unix_seconds"]
        .as_u64()
        .unwrap();
    observation_like(
        &fixture,
        PRODUCT,
        archived_id,
        "SYNTHETIC archived proof",
        observed + 60,
    );
    observation_like(
        &fixture,
        PRODUCT,
        stray_id,
        "SYNTHETIC stray proof",
        observed + 120,
    );
    reference(
        &fixture,
        ".chough/sessions/fixture-first.json",
        &[CASE_RUN, PRODUCT],
    );
    reference(
        &fixture,
        ".chough/sessions/archive/fixture-second.json",
        &[archived_id],
    );
    let bug = link_bug(&fixture, &[archived_id]);
    // An artifact no observation names, as a 0.5.x prune left them.
    let orphan = ".chough/artifacts/OBS-pruned-00000000000000000000a003.log";
    fixture.write(orphan, "SYNTHETIC log of a pruned observation\n");
    fixture.commit("a 0.5.x project whose sessions and bug reference observations");
    let case_run = legacy(&fixture, CASE_RUN);
    let product = legacy(&fixture, PRODUCT);
    let before = tree(fixture.root());

    let (plan, applied) = migrate(&fixture);
    let open = session_folder(&plan, "fixture-first");
    let story = session_folder(&plan, "fixture-second");
    let date = utc_date(case_run["observed_at_unix_seconds"].as_u64().unwrap());
    let case_entry = format!(".chough/sessions/{open}/evidence/{date}-tc-fixture-0001.md");
    let product_entry = format!(
        ".chough/sessions/{open}/evidence/{}-fixture-observation-the-one-case-passes.md",
        utc_date(observed)
    );
    assert_eq!(alias(&plan, CASE_RUN), case_entry, "{plan:#}");
    assert_eq!(alias(&plan, PRODUCT), product_entry, "{plan:#}");
    assert_eq!(
        alias(&plan, archived_id),
        format!(".chough/archive/sessions/{story}.md")
    );
    assert_eq!(alias(&plan, stray_id), "removed");
    for path in [
        format!(".chough/evidence/{CASE_RUN}.json"),
        format!(".chough/evidence/{PRODUCT}.json"),
        format!(".chough/evidence/{archived_id}.json"),
    ] {
        assert!(plan["sources"].get(&path).is_some(), "{path}: {plan:#}");
    }
    let removed = &plan["removed"];
    assert!(
        removed[format!(".chough/evidence/{stray_id}.json")]
            .as_str()
            .is_some_and(|why| why.contains("references")),
        "{removed:#}"
    );
    assert!(
        removed[LOG]
            .as_str()
            .is_some_and(|why| why.contains("raw log")),
        "{removed:#}"
    );
    assert!(
        removed[orphan]
            .as_str()
            .is_some_and(|why| why.contains("no carried observation")),
        "{removed:#}"
    );
    assert!(applied["receipt"].is_string(), "{applied:#}");

    // The case run's entry: every field step 4 writes that 0.5.x held.
    let text = fixture.read(&case_entry);
    let entry = front(&text);
    assert_eq!(entry["kind"], "evidence", "{text}");
    assert_eq!(entry["id"], format!("{date}-tc-fixture-0001"), "{text}");
    assert_eq!(items(&entry["aliases"]), [CASE_RUN], "{text}");
    assert_eq!(
        entry["claim"],
        case_run["summary"].as_str().unwrap(),
        "{text}"
    );
    assert_eq!(entry["result"], "passed", "{text}");
    assert_eq!(entry["evidence_kind"], "product-test", "{text}");
    assert_eq!(items(&entry["case"]), ["TC-fixture-0001"], "{text}");
    assert_eq!(
        items(&entry["case_digest"]),
        [case_run["cases"][0]["digest"].as_str().unwrap()],
        "{text}"
    );
    assert_eq!(items(&entry["command"]), ["cat", "README.md"], "{text}");
    assert_eq!(entry["exit_code"], "0", "{text}");
    assert_eq!(entry["duration_seconds"], "0", "{text}");
    assert_eq!(
        entry["revision"],
        case_run["snapshot"]["revision"].as_str().unwrap(),
        "{text}"
    );
    assert_eq!(
        entry["content"],
        case_run["snapshot"]["content"].as_str().unwrap(),
        "{text}"
    );
    assert_eq!(entry["environment"], "fixture", "{text}");
    assert_eq!(
        entry["fingerprint"],
        format!("sha256:{LOG_DIGEST}"),
        "{text}"
    );
    assert!(
        !entry.contains_key("artifacts"),
        "a raw log is never carried: {text}"
    );
    assert_eq!(items(&entry["criteria"]), ["LIFE-1"], "{text}");
    assert_eq!(items(&entry["requirements"]), ["REQ-fixture-001"], "{text}");
    assert_eq!(
        items(&entry["contracts"]),
        ["specs/fixture/fixture.spec.md"],
        "{text}"
    );
    assert_eq!(
        entry["observed"],
        iso(case_run["observed_at_unix_seconds"].as_u64().unwrap()),
        "{text}"
    );
    assert!(!fixture.exists(LOG));

    // The product test keeps its in-project artifact, which stays the project's.
    let text = fixture.read(&product_entry);
    let entry = front(&text);
    assert_eq!(items(&entry["aliases"]), [PRODUCT], "{text}");
    assert_eq!(items(&entry["artifacts"]), ["artifact.txt"], "{text}");
    assert_eq!(
        maps(&text, "artifact_fingerprints"),
        [BTreeMap::from([
            ("path".to_string(), "artifact.txt".to_string()),
            ("digest".to_string(), ARTIFACT_DIGEST.to_string()),
        ])],
        "{text}"
    );
    assert!(!entry.contains_key("fingerprint"), "{text}");
    assert_eq!(
        fixture.read("artifact.txt").as_bytes(),
        before["artifact.txt"]
    );
    assert_eq!(product["artifacts"], json!(["artifact.txt"]));

    // The archived story and the bug hold the same receipt: REQ-history-006's fields
    // plus observed and duration_seconds, never more.
    let story_text = fixture.read(&format!(".chough/archive/sessions/{story}.md"));
    let receipts = maps(&story_text, "receipts");
    assert_eq!(receipts.len(), 1, "{story_text}");
    let receipt = &receipts[0];
    assert_eq!(
        receipt["id"],
        format!("{}-synthetic-archived-proof", utc_date(observed + 60)),
        "{story_text}"
    );
    assert_eq!(receipt["aliases"], archived_id, "{story_text}");
    assert_eq!(receipt["claim"], "SYNTHETIC archived proof");
    assert_eq!(receipt["result"], "passed");
    assert_eq!(receipt["environment"], "fixture");
    assert_eq!(receipt["observed"], iso(observed + 60));
    assert_eq!(
        receipt["revision"],
        product["snapshot"]["revision"].as_str().unwrap()
    );
    assert_eq!(
        receipt["content"],
        product["snapshot"]["content"].as_str().unwrap()
    );
    // No artifacts field on a receipt: the in-project artifact's digest is its
    // fingerprint, and a gap line says the paths are not carried.
    assert_eq!(
        receipt["fingerprint"],
        format!("sha256:{ARTIFACT_DIGEST}"),
        "{story_text}"
    );
    let allowed = [
        "id",
        "aliases",
        "claim",
        "result",
        "command",
        "revision",
        "environment",
        "content",
        "fingerprint",
        "case",
        "case_digest",
        "screenshot",
        "observed",
        "duration_seconds",
    ];
    for key in receipt.keys() {
        assert!(
            allowed.contains(&key.as_str()),
            "{key} on a receipt: {story_text}"
        );
    }
    let bug_path = alias(&plan, &legacy_bug_id(&before, &bug));
    let bug_text = fixture.read(&bug_path);
    assert_eq!(maps(&bug_text, "receipts"), receipts, "{bug_text}");

    // What is not carried is one counted gap line per field.
    let gap = gap_with(&plan, "provenance");
    assert!(gap.contains("3 observations"), "{gap}");
    let gap = gap_with(&plan, "snapshot fingerprint");
    assert!(gap.contains("3 observations"), "{gap}");
    gap_with(&plan, "artifact paths");

    // Every old ID resolves: the open session's references and the bug's links.
    let status = fixture.ok(&["status", "--environment", "fixture"]);
    for id in [CASE_RUN, PRODUCT] {
        let label = reference_label(&status, id);
        assert_ne!(label, "unavailable", "{id}: {status:#}");
        assert_ne!(label, "removed", "{id}: {status:#}");
    }
    assert_ne!(reference_label(&status, archived_id), "unavailable");
    fixture.ok(&["next", "--json"]);
    assert!(!fixture.exists(".chough/evidence"));
}

/// The 0.5.x ID of the bug at `path` in the tree before the apply.
fn legacy_bug_id(before: &BTreeMap<String, Vec<u8>>, path: &str) -> String {
    let bug: Value = serde_json::from_slice(&before[path]).unwrap();
    bug["id"].as_str().unwrap().to_string()
}

/// v0.1.0 named observations without the `OBS-` prefix. A legacy checkpoint that
/// references both keeps them as aliases, and both resolve after the apply.
#[test]
fn v0_1_0_observation_ids_resolve_after_the_apply() {
    let fixture = project("releases/v0.1.0");
    let ids = ["fixture-observation", "tc-fixture-0001-1794414604"];
    reference(&fixture, ".chough/session.json", &ids);
    fixture.commit("a 0.1.0 project whose checkpoint references its observations");
    let (plan, _) = migrate(&fixture);
    for id in ids {
        let path = alias(&plan, id);
        assert!(path.contains("/evidence/"), "{id}: {path}");
        let text = fixture.read(&path);
        assert_eq!(items(&front(&text)["aliases"]), [id], "{text}");
    }
    let status = fixture.ok(&["status", "--environment", "fixture"]);
    for id in ids {
        assert_ne!(reference_label(&status, id), "unavailable", "{status:#}");
    }
}

/// One observation two open sessions reference is one entry, in the open session whose
/// new ID sorts first; the other session resolves it through the alias, and no ID is
/// held twice.
#[test]
fn one_observation_two_open_sessions_reference_is_one_entry_in_the_first() {
    let fixture = project("migrate/adopted");
    let template = manifest_dir()
        .join("tests/fixtures/releases/v0.5.0/.chough/evidence")
        .join(format!("{PRODUCT}.json"));
    fixture.write(
        &format!(".chough/evidence/{PRODUCT}.json"),
        &fs::read_to_string(template).unwrap(),
    );
    fixture.write("artifact.txt", "fixture artifact: one small line\n");
    reference(&fixture, ".chough/sessions/open-one.json", &[PRODUCT]);
    reference(&fixture, ".chough/sessions/open-two.json", &[PRODUCT]);
    fixture.commit("two open sessions reference one observation");
    let (plan, _) = migrate(&fixture);
    let first = session_folder(&plan, "open-one");
    let second = session_folder(&plan, "open-two");
    assert!(first < second);
    let path = alias(&plan, PRODUCT);
    assert!(
        path.starts_with(&format!(".chough/sessions/{first}/evidence/")),
        "{path}"
    );
    let entries = |id: &str| -> usize {
        fs::read_dir(
            fixture
                .root()
                .join(format!(".chough/sessions/{id}/evidence")),
        )
        .map(|entries| entries.count())
        .unwrap_or(0)
    };
    assert_eq!(entries(&first), 1);
    assert_eq!(entries(&second), 0);
    fixture.ok(&["session", "use", &second]);
    let status = fixture.ok(&["status", "--environment", "fixture"]);
    assert_ne!(
        reference_label(&status, PRODUCT),
        "unavailable",
        "{status:#}"
    );
    let (code, doctor) = {
        let output = fixture.run(&["doctor"]);
        (
            output.status.code(),
            serde_json::from_slice::<Value>(&output.stdout).unwrap(),
        )
    };
    assert!(
        !doctor.to_string().contains("alias_conflict"),
        "{code:?}: {doctor:#}"
    );
}

/// A PNG of `size` bytes with a `tEXt` and an `eXIf` chunk the migration strips.
fn png_with_metadata(size: usize) -> Vec<u8> {
    fn chunk(kind: &[u8], data: &[u8]) -> Vec<u8> {
        let mut out = (data.len() as u32).to_be_bytes().to_vec();
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        out.extend_from_slice(&[0, 0, 0, 0]);
        out
    }
    let mut png = b"\x89PNG\r\n\x1a\n".to_vec();
    png.extend(chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0]));
    png.extend(chunk(b"tEXt", b"Author\0SYNTHETIC-person"));
    png.extend(chunk(b"eXIf", b"SYNTHETIC-exif"));
    let fill = size.saturating_sub(png.len() + 12 + 12);
    png.extend(chunk(b"IDAT", &vec![7u8; fill]));
    png.extend(chunk(b"IEND", &[]));
    png
}

/// Key screenshots (REQ-records-013): a carried observation's PNG artifact is stripped
/// of its metadata and kept beside its open session's entry, or beside the story for an
/// archived one, with the committed bytes' SHA-256 as the fingerprint. Past 524,288
/// bytes after stripping, past five per session or past 2,097,152 bytes per session,
/// in observed-then-ID order, the image is removed with why.
#[test]
fn key_screenshots_are_stripped_bounded_and_kept_beside_their_session_or_story() {
    let fixture = project("releases/v0.5.0");
    let observed = legacy(&fixture, PRODUCT)["observed_at_unix_seconds"]
        .as_u64()
        .unwrap();
    let mut open_ids = vec![];
    // Eight images for the open session, in observed order: three fit; the fourth is
    // over its own bound; the fifth fits (1.6 MB kept); the sixth would pass 2 MiB; the
    // seventh is the fifth kept; the eighth would be a sixth.
    let sizes = [
        400_000usize,
        400_000,
        400_000,
        600_000,
        400_000,
        500_000,
        1_000,
        1_000,
    ];
    for (index, size) in sizes.iter().enumerate() {
        let id = format!("OBS-product-test-0000000000000000000{:05}", index);
        let image = format!(".chough/artifacts/{id}.png");
        let bytes = png_with_metadata(*size);
        fs::create_dir_all(fixture.root().join(".chough/artifacts")).unwrap();
        fs::write(fixture.root().join(&image), &bytes).unwrap();
        let mut record = observation_like(
            &fixture,
            PRODUCT,
            &id,
            &format!("SYNTHETIC screen {index}"),
            observed + index as u64,
        );
        record["artifacts"] = json!([image]);
        record["artifact_fingerprints"] = json!({ image.clone(): sha256(&bytes) });
        fixture.write(
            &format!(".chough/evidence/{id}.json"),
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
        open_ids.push(id);
    }
    // One for the archived session.
    let archived = "OBS-product-test-00000000000000000000b001";
    let archived_image = format!(".chough/artifacts/{archived}.png");
    let archived_bytes = png_with_metadata(10_000);
    fs::write(fixture.root().join(&archived_image), &archived_bytes).unwrap();
    let mut record = observation_like(
        &fixture,
        PRODUCT,
        archived,
        "SYNTHETIC archived screen",
        observed,
    );
    record["artifacts"] = json!([archived_image]);
    record["artifact_fingerprints"] = json!({ archived_image.clone(): sha256(&archived_bytes) });
    fixture.write(
        &format!(".chough/evidence/{archived}.json"),
        &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
    );
    let refs: Vec<&str> = open_ids.iter().map(String::as_str).collect();
    reference(&fixture, ".chough/sessions/fixture-first.json", &refs);
    reference(
        &fixture,
        ".chough/sessions/archive/fixture-second.json",
        &[archived],
    );
    fixture.commit("observations with key screenshots");

    let (plan, _) = migrate(&fixture);
    let open = session_folder(&plan, "fixture-first");
    let story = session_folder(&plan, "fixture-second");
    let kept: Vec<bool> = open_ids
        .iter()
        .map(|id| {
            let entry = alias(&plan, id);
            let text = fixture.read(&entry);
            let name = format!("{}.png", front(&text)["id"]);
            let beside = format!(".chough/sessions/{open}/evidence/{name}");
            let kept = fixture.exists(&beside);
            if kept {
                let bytes = fs::read(fixture.root().join(&beside)).unwrap();
                assert!(!bytes.windows(4).any(|w| w == b"tEXt" || w == b"eXIf"));
                assert!(bytes.len() <= 524_288);
                assert_eq!(front(&text)["screenshot"], name, "{text}");
                assert_eq!(
                    front(&text)["fingerprint"],
                    format!("sha256:{}", sha256(&bytes)),
                    "{text}"
                );
            } else {
                assert!(!front(&text).contains_key("screenshot"), "{text}");
            }
            assert!(!fixture.exists(&format!(".chough/artifacts/{id}.png")));
            kept
        })
        .collect();
    assert_eq!(
        kept,
        [true, true, true, false, true, false, true, false],
        "{plan:#}"
    );
    let removed = &plan["removed"];
    for (index, why) in [
        (3usize, "524288"),
        (5, "2097152"),
        (7, "the 5 a session keeps"),
    ] {
        let image = format!(".chough/artifacts/{}.png", open_ids[index]);
        assert!(
            removed[&image]
                .as_str()
                .is_some_and(|text| text.contains(why)),
            "{image}: {removed:#}"
        );
    }

    // The archived session's screenshot sits beside its story.
    let story_text = fixture.read(&format!(".chough/archive/sessions/{story}.md"));
    let receipt = &maps(&story_text, "receipts")[0];
    let name = &receipt["screenshot"];
    let beside = format!(".chough/archive/sessions/{story}/{name}");
    let bytes = fs::read(fixture.root().join(&beside)).unwrap();
    assert!(!bytes.windows(4).any(|w| w == b"tEXt" || w == b"eXIf"));
    assert_eq!(
        receipt["fingerprint"],
        format!("sha256:{}", sha256(&bytes)),
        "{story_text}"
    );
}

/// A 0.5.x validation run (REQ-cli-023): carried like an observation under its `VAL-`
/// ID. One an open session references is one entry in the format `validate run`
/// writes, named from its case ID and `run`, holding each attempt with its
/// observation's new ID; one nothing references is removed with why, aliased
/// `removed`.
#[test]
fn validation_runs_carry_like_observations() {
    let fixture = project("releases/v0.5.0");
    let case_run = legacy(&fixture, CASE_RUN);
    let started = case_run["observed_at_unix_seconds"].as_u64().unwrap();
    let mut attempts = vec![];
    for index in 0..3u64 {
        let id = format!("OBS-tc-fixture-0001-00000000000000000000c00{index}");
        let mut record = case_run.clone();
        record["id"] = json!(id);
        record["observed_at_unix_seconds"] = json!(started + index * 10);
        record["outcome"] = json!(if index == 1 { "failed" } else { "passed" });
        record["artifacts"] = json!([]);
        record["artifact_fingerprints"] = json!({});
        fixture.write(
            &format!(".chough/evidence/{id}.json"),
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
        attempts.push(id);
    }
    let run = |id: &str, observations: &[String]| {
        json!({
            "schema_version": 1,
            "id": id,
            "case": "TC-fixture-0001",
            "times": observations.len(),
            "interval_seconds": 0,
            "environment": "fixture",
            "authority": "consent: SYNTHETIC-human asked for three runs",
            "state": "complete",
            "completed": observations.len(),
            "failures": 1,
            "not_run": 0,
            "observations": observations,
            "bugs": [],
            "elapsed_seconds": 25,
            "started_at_unix_seconds": started,
        })
    };
    let referenced = "VAL-tc-fixture-0001-00000000000000000000d001";
    let unreferenced = "VAL-tc-fixture-0001-00000000000000000000d002";
    for (id, observations) in [(referenced, attempts.clone()), (unreferenced, vec![])] {
        fixture.write(
            &format!(".chough/validation/{id}.json"),
            &(serde_json::to_string_pretty(&run(id, &observations)).unwrap() + "\n"),
        );
    }
    reference(
        &fixture,
        ".chough/sessions/fixture-first.json",
        &[referenced],
    );
    fixture.commit("validation runs");

    let (plan, _) = migrate(&fixture);
    let open = session_folder(&plan, "fixture-first");
    let path = alias(&plan, referenced);
    assert_eq!(
        path,
        format!(
            ".chough/sessions/{open}/evidence/{}-tc-fixture-0001-run.md",
            utc_date(started)
        )
    );
    let text = fixture.read(&path);
    let entry = front(&text);
    assert_eq!(entry["validation_run"], "true", "{text}");
    assert_eq!(items(&entry["aliases"]), [referenced], "{text}");
    // The format `validate run` writes, so `validate resume <old ID>` continues it.
    assert_eq!(entry["run_case"], "TC-fixture-0001", "{text}");
    assert_eq!(
        entry["authority"], "consent: SYNTHETIC-human asked for three runs",
        "{text}"
    );
    assert_eq!(entry["times"], "3", "{text}");
    assert_eq!(entry["interval_seconds"], "0", "{text}");
    assert_eq!(entry["failures"], "1", "{text}");
    assert_eq!(entry["elapsed_seconds"], "25", "{text}");
    let held = maps(&text, "attempts");
    assert_eq!(held.len(), 3, "{text}");
    assert_eq!(held[1]["result"], "failed", "{text}");
    assert_eq!(held[2]["observed"], iso(started + 20), "{text}");
    // Each attempt names its observation's new ID: nothing else references the attempts,
    // so each is an entry of the run's session, its old ID an alias.
    for (attempt, old) in held.iter().zip(&attempts) {
        let entry = format!(
            ".chough/sessions/{open}/evidence/{}.md",
            attempt["observation"]
        );
        assert_eq!(alias(&plan, old), entry, "{plan:#}");
        let text = fixture.read(&entry);
        assert_eq!(items(&front(&text)["aliases"]), [old.as_str()], "{text}");
    }
    assert_eq!(alias(&plan, unreferenced), "removed");
    assert!(
        plan["removed"][format!(".chough/validation/{unreferenced}.json")]
            .as_str()
            .is_some(),
        "{plan:#}"
    );
    assert!(!fixture.exists(".chough/validation"));
    let status = fixture.ok(&["status", "--environment", "fixture"]);
    assert_ne!(
        reference_label(&status, referenced),
        "unavailable",
        "{status:#}"
    );
}

/// The plan is the same twice; two observations that would take one name get a clash
/// suffix from their old IDs, the same in every plan; and an apply interrupted after
/// writing an entry plans the same digest and resumes to the same entry.
#[test]
fn names_are_deterministic_and_an_interrupted_apply_resumes() {
    let fixture = project("releases/v0.5.0");
    let observed = legacy(&fixture, PRODUCT)["observed_at_unix_seconds"]
        .as_u64()
        .unwrap();
    let twin = "OBS-product-test-00000000000000000000e001";
    observation_like(
        &fixture,
        PRODUCT,
        twin,
        "fixture observation: the one case passes",
        observed,
    );
    reference(
        &fixture,
        ".chough/sessions/fixture-first.json",
        &[PRODUCT, twin],
    );
    fixture.commit("two observations that would take one name");
    let plan = fixture.ok(&["migrate"]);
    assert_eq!(fixture.ok(&["migrate"])["plan_digest"], plan["plan_digest"]);
    // Named in observed-then-old-ID order: the twin's ID sorts first and keeps the
    // name; the other takes the suffix of its own old ID.
    let first = alias(&plan, twin);
    let second = alias(&plan, PRODUCT);
    assert_ne!(first, second);
    assert!(
        first.ends_with("-fixture-observation-the-one-case-passes.md"),
        "{first}"
    );
    let suffix = &sha256(PRODUCT.as_bytes())[..6];
    assert!(second.ends_with(&format!("-{suffix}.md")), "{second}");

    // The same project applied elsewhere gives the first entry's planned bytes; an
    // apply stopped after that one write leaves exactly them.
    let elsewhere = Fixture::new();
    for (path, bytes) in tree(fixture.root()) {
        let target = elsewhere.root().join(&path);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        fs::write(target, bytes).unwrap();
    }
    elsewhere.commit("the same 0.5.x project");
    migrate(&elsewhere);
    let planned = fs::read(elsewhere.root().join(&first)).unwrap();
    let interrupted = fixture.root().join(&first);
    fs::create_dir_all(interrupted.parent().unwrap()).unwrap();
    fs::write(&interrupted, &planned).unwrap();
    let resumed = fixture.ok(&["migrate"]);
    assert_eq!(resumed["plan_digest"], plan["plan_digest"], "{resumed:#}");
    let digest = plan["plan_digest"].as_str().unwrap().to_string();
    let applied = fixture.ok(&["migrate", "--apply", &digest, "--consent", CONSENT]);
    assert_eq!(applied["applied"], true, "{applied:#}");
    assert_eq!(fs::read(&interrupted).unwrap(), planned);
    assert_eq!(
        fs::read(fixture.root().join(&second)).unwrap(),
        fs::read(elsewhere.root().join(&second)).unwrap()
    );
}

/// Observations the 0.5.x roll-up moved into the vault with their session
/// (`<library-dir>/attachments/<folder>/evidence/`, their artifacts under
/// `attachments/<folder>/artifacts/`): one the rolled session references is a receipt of
/// its story, its image read from the vault and kept beside the story; one nothing
/// references is removed, aliased `removed`.
#[test]
fn rolled_observations_in_the_vault_become_receipts_of_their_story() {
    let fixture = project("migrate/adopted");
    let base = "knowledge/attachments/archived-1";
    let template: Value = serde_json::from_str(
        &fs::read_to_string(
            manifest_dir()
                .join("tests/fixtures/releases/v0.5.0/.chough/evidence")
                .join(format!("{PRODUCT}.json")),
        )
        .unwrap(),
    )
    .unwrap();
    let rolled = "OBS-product-test-00000000000000000000g001";
    let stray = "OBS-product-test-00000000000000000000g002";
    let image = format!(".chough/artifacts/{rolled}.png");
    let bytes = png_with_metadata(2_000);
    for (id, artifacts) in [(rolled, json!([image])), (stray, json!([]))] {
        let mut record = template.clone();
        record["id"] = json!(id);
        record["summary"] = json!(format!("SYNTHETIC rolled proof {id}"));
        record["artifacts"] = artifacts.clone();
        record["artifact_fingerprints"] = if artifacts == json!([]) {
            json!({})
        } else {
            json!({ image.clone(): sha256(&bytes) })
        };
        fixture.write(
            &format!("{base}/evidence/{id}.json"),
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
    }
    let moved = format!("{base}/artifacts/{rolled}.png");
    fs::create_dir_all(fixture.root().join(format!("{base}/artifacts"))).unwrap();
    fs::write(fixture.root().join(&moved), &bytes).unwrap();
    reference(
        &fixture,
        &format!("{base}/sessions/archived-1.json"),
        &[rolled],
    );
    fixture.commit("rolled observations in the vault");

    let (plan, _) = migrate(&fixture);
    let story = session_folder(&plan, "archived-1");
    assert_eq!(
        alias(&plan, rolled),
        format!(".chough/archive/sessions/{story}.md")
    );
    assert_eq!(alias(&plan, stray), "removed");
    for path in [format!("{base}/evidence/{rolled}.json"), moved.clone()] {
        assert!(plan["removed"].get(&path).is_none(), "{path}: {plan:#}");
        assert!(!fixture.exists(&path), "{path}");
    }
    assert!(
        plan["removed"][format!("{base}/evidence/{stray}.json")]
            .as_str()
            .is_some(),
        "{plan:#}"
    );
    let text = fixture.read(&format!(".chough/archive/sessions/{story}.md"));
    let receipts = maps(&text, "receipts");
    let receipt = receipts
        .iter()
        .find(|receipt| receipt["aliases"] == rolled)
        .unwrap_or_else(|| panic!("{text}"));
    let name = &receipt["screenshot"];
    let beside = fs::read(
        fixture
            .root()
            .join(format!(".chough/archive/sessions/{story}/{name}")),
    )
    .unwrap();
    assert!(!beside.windows(4).any(|w| w == b"tEXt" || w == b"eXIf"));
    assert_eq!(
        receipt["fingerprint"],
        format!("sha256:{}", sha256(&beside))
    );
}

/// REQ-cli-017: 0.5.x compared content only for schema 2 records, so a schema 1
/// observation's freshness rested on its revision-bound fingerprint. Its entry carries no
/// content and reads stale once anything is committed, while a schema 2 twin with the
/// same content identity reads current.
#[test]
fn a_schema_1_observation_is_carried_without_content_and_stays_stale() {
    // The migrated tree's content identity, from `evidence add` on a migrated probe of
    // the same files (the migration's own writes are part of it).
    let content = {
        let probe = project("releases/v0.5.0");
        probe.stubs();
        probe.commit("the probe");
        migrate(&probe);
        probe.ok(&[
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
            "fixture",
            "--summary",
            "SYNTHETIC probe of the content identity",
        ]);
        let folder = probe
            .root()
            .join(format!(".chough/sessions/{}/evidence", probe.active_id()));
        let entry = fs::read_dir(folder)
            .unwrap()
            .next()
            .unwrap()
            .unwrap()
            .path();
        front(&fs::read_to_string(entry).unwrap())["content"].clone()
    };
    let fixture = project("releases/v0.5.0");
    fixture.stubs();
    let modern = "OBS-product-test-00000000000000000000h002";
    let schema_1 = "OBS-product-test-00000000000000000000h001";
    for (id, version) in [(modern, 2), (schema_1, 1)] {
        let mut record = observation_like(&fixture, PRODUCT, id, &format!("SYNTHETIC {id}"), 1);
        record["schema_version"] = json!(version);
        record["snapshot"]["content"] = json!(content);
        fixture.write(
            &format!(".chough/evidence/{id}.json"),
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
    }
    reference(
        &fixture,
        ".chough/sessions/fixture-first.json",
        &[modern, schema_1],
    );
    fixture.commit("a schema 1 and a schema 2 observation with one content identity");
    let (plan, _) = migrate(&fixture);
    let old_entry = front(&fixture.read(&alias(&plan, schema_1)));
    assert!(!old_entry.contains_key("content"), "{old_entry:?}");
    assert_eq!(
        front(&fixture.read(&alias(&plan, modern)))["content"],
        content
    );
    fixture.git(&["commit", "-q", "--allow-empty", "-m", "anything committed"]);
    let status = fixture.ok(&["status", "--environment", "fixture"]);
    let freshness = |old: &str| {
        status["observations"]
            .as_array()
            .unwrap()
            .iter()
            .find(|row| row["record"]["aliases"][0] == old)
            .unwrap_or_else(|| panic!("{old}: {status:#}"))["freshness"]
            .clone()
    };
    assert_eq!(freshness(modern), "current-observation", "{status:#}");
    assert_eq!(
        freshness(schema_1),
        "stale-revision-or-inputs",
        "{status:#}"
    );
}

/// 0.5.x accepted a SHA-256 digest in either case, and 0.6.0 reads only lowercase ones
/// (REQ-cli-017). A 0.5.x observation whose content, artifact, log and case digests are
/// uppercase migrates with each of them lowercased, the same digest, so `status` and
/// `next` read the entry and the story's receipt afterwards.
#[test]
fn uppercase_digests_migrate_lowercased_and_read() {
    let fixture = project("releases/v0.5.0");
    let upper = |value: &Value| json!(value.as_str().unwrap().to_ascii_uppercase());
    let mut product = legacy(&fixture, PRODUCT);
    product["snapshot"]["content"] = upper(&product["snapshot"]["content"]);
    product["artifact_fingerprints"]["artifact.txt"] =
        upper(&product["artifact_fingerprints"]["artifact.txt"]);
    let mut case_run = legacy(&fixture, CASE_RUN);
    case_run["snapshot"]["content"] = upper(&case_run["snapshot"]["content"]);
    case_run["artifact_fingerprints"][LOG] = upper(&case_run["artifact_fingerprints"][LOG]);
    case_run["cases"][0]["digest"] = upper(&case_run["cases"][0]["digest"]);
    for record in [&product, &case_run] {
        fixture.write(
            &format!(".chough/evidence/{}.json", record["id"].as_str().unwrap()),
            &(serde_json::to_string_pretty(record).unwrap() + "\n"),
        );
    }
    let archived_id = "OBS-product-test-00000000000000000000j001";
    let mut archived = product.clone();
    archived["id"] = json!(archived_id);
    archived["summary"] = json!("SYNTHETIC archived uppercase proof");
    fixture.write(
        &format!(".chough/evidence/{archived_id}.json"),
        &(serde_json::to_string_pretty(&archived).unwrap() + "\n"),
    );
    reference(
        &fixture,
        ".chough/sessions/fixture-first.json",
        &[PRODUCT, CASE_RUN],
    );
    reference(
        &fixture,
        ".chough/sessions/archive/fixture-second.json",
        &[archived_id],
    );
    fixture.commit("0.5.x observations with uppercase digests");

    let (plan, _) = migrate(&fixture);
    // Both read afterwards.
    fixture.ok(&["status", "--environment", "fixture"]);
    fixture.ok(&["next", "--json"]);
    let lower = |value: &Value| value.as_str().unwrap().to_ascii_lowercase();
    let text = fixture.read(&alias(&plan, PRODUCT));
    let entry = front(&text);
    assert_eq!(
        entry["content"],
        lower(&product["snapshot"]["content"]),
        "{text}"
    );
    assert_eq!(
        maps(&text, "artifact_fingerprints")[0]["digest"],
        lower(&product["artifact_fingerprints"]["artifact.txt"]),
        "{text}"
    );
    let text = fixture.read(&alias(&plan, CASE_RUN));
    let entry = front(&text);
    assert_eq!(
        entry["content"],
        lower(&case_run["snapshot"]["content"]),
        "{text}"
    );
    assert_eq!(
        entry["fingerprint"],
        format!("sha256:{}", lower(&case_run["artifact_fingerprints"][LOG])),
        "{text}"
    );
    assert_eq!(
        items(&entry["case_digest"]),
        [lower(&case_run["cases"][0]["digest"])],
        "{text}"
    );
    let story = fixture.read(&alias(&plan, archived_id));
    let receipt = maps(&story, "receipts")
        .into_iter()
        .find(|receipt| receipt["aliases"] == archived_id)
        .unwrap_or_else(|| panic!("{story}"));
    assert_eq!(
        receipt["content"],
        lower(&product["snapshot"]["content"]),
        "{story}"
    );
    assert_eq!(
        receipt["fingerprint"],
        format!(
            "sha256:{}",
            lower(&product["artifact_fingerprints"]["artifact.txt"])
        ),
        "{story}"
    );
}

/// The item lines of a record's What was verified section.
fn verified_lines(text: &str) -> Vec<String> {
    let start = text
        .find("\n## What was verified\n")
        .unwrap_or_else(|| panic!("no What was verified: {text}"));
    let section = &text[start + 1..];
    let section = &section[..section[3..]
        .find("\n## ")
        .map_or(section.len(), |at| at + 3)];
    section
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(String::from)
        .collect()
}

/// Fresh-eyes finding F10e (REQ-records-009): a migrated open session's What was
/// verified links each carried entry, `[<claim>](evidence/<entry>.md)`, as a 0.6.0 save
/// writes it, never the bare 0.5.x ID, which stays the entry's alias; a later save keeps
/// one linked line per entry and adds no bare one.
#[test]
fn a_migrated_sessions_verified_lines_link_the_carried_entries() {
    let fixture = project("releases/v0.5.0");
    reference(
        &fixture,
        ".chough/sessions/fixture-first.json",
        &[CASE_RUN, PRODUCT],
    );
    fixture.commit("a 0.5.x project whose open session references two observations");
    let claims = [
        (CASE_RUN, legacy(&fixture, CASE_RUN)),
        (PRODUCT, legacy(&fixture, PRODUCT)),
    ];
    let (plan, _) = migrate(&fixture);
    let open = session_folder(&plan, "fixture-first");
    let file = format!(".chough/sessions/{open}/session.md");
    let check = |when: &str| {
        let lines = verified_lines(&fixture.read(&file));
        for (old, record) in &claims {
            let entry = alias(&plan, old);
            let name = entry.rsplit('/').next().unwrap();
            let link = format!("](evidence/{name})");
            let linked: Vec<&String> = lines.iter().filter(|line| line.ends_with(&link)).collect();
            assert_eq!(linked.len(), 1, "{when}: {old} {lines:#?}");
            assert!(
                linked[0].starts_with("- [")
                    && linked[0].contains(record["summary"].as_str().unwrap()),
                "{when}: {}",
                linked[0]
            );
            assert!(
                !lines.iter().any(|line| line.contains(old)),
                "{when}: the bare {old} is still listed: {lines:#?}"
            );
            assert_eq!(
                items(&front(&fixture.read(&entry))["aliases"]),
                [old.to_string()],
                "{when}"
            );
        }
    };
    check("after the apply");
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "SYNTHETIC: a decision after migrating",
    ]);
    check("after a save");
}

/// Fresh-eyes finding F10e, the rest of it (REQ-records-009, REQ-history-006): a migrated
/// archived story's What was verified gives an observation carried as its receipt as a
/// 0.6.0 story writes it, `<claim> — <first 12 fingerprint digits> ^<receipt id>`, never
/// the bare 0.5.x ID.
#[test]
fn a_migrated_storys_verified_lines_are_receipt_lines() {
    let fixture = project("releases/v0.5.0");
    let archived_id = "OBS-product-test-00000000000000000000a001";
    let observed = legacy(&fixture, PRODUCT)["observed_at_unix_seconds"]
        .as_u64()
        .unwrap();
    observation_like(
        &fixture,
        PRODUCT,
        archived_id,
        "SYNTHETIC archived proof",
        observed + 60,
    );
    reference(
        &fixture,
        ".chough/sessions/archive/fixture-second.json",
        &[archived_id],
    );
    fixture.commit("a 0.5.x project whose archived session references an observation");
    let (plan, _) = migrate(&fixture);
    let story = fixture.read(&alias(&plan, archived_id));
    let receipt = maps(&story, "receipts")
        .into_iter()
        .find(|receipt| receipt["aliases"] == archived_id)
        .unwrap_or_else(|| panic!("no receipt for {archived_id}: {story}"));
    let digits: String = receipt["fingerprint"]
        .trim_start_matches("sha256:")
        .chars()
        .take(12)
        .collect();
    let lines = verified_lines(&story);
    assert!(
        !lines.iter().any(|line| line.contains(archived_id)),
        "the bare {archived_id} is still listed: {lines:#?}"
    );
    let expected = format!("- SYNTHETIC archived proof — {digits} ^{}", receipt["id"]);
    assert_eq!(
        lines.iter().filter(|line| **line == expected).count(),
        1,
        "{expected:?} in {lines:#?}"
    );
}
