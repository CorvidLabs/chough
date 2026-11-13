//! The 2026-11-03 cli spec split kept every moved requirement's wording
//! (REQ-history-001 to REQ-history-004, formerly REQ-cli-018 and REQ-cli-025 to
//! REQ-cli-027). The digests are of each requirement body, trimmed, at main 7bd1b62.
//! A later native change that edits a moved requirement updates its digest here.
//! The 2026-11-04 split moved REQ-cli-020, REQ-cli-024 and REQ-cli-028 to the
//! reporting module as REQ-reporting-001 to REQ-reporting-003; their digests are of
//! each body, trimmed, at main a33c747.

use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::path::Path;

const MOVED: [(&str, &str, &str); 4] = [
    (
        "REQ-cli-018",
        "REQ-history-001",
        "1a34122d7aebfd557ab754b8684c9654506c15a0e66daf9980adffaa0f4becd8",
    ),
    (
        "REQ-cli-025",
        "REQ-history-002",
        // Modified by native change keep-one-session-per-objective (#77): per-session
        // cursor; then by chough-0-6-0-human-first-records-...: the append-only Timeline
        // of session.md; then by prepare-the-chough-0-6-0-release-...: labelled lines for
        // fields with parts, and the start entry.
        "336e87bfe0fe9d03609babc4f1bc68ed79b36243151b5a950f0c8a7ccf4f2347",
    ),
    (
        "REQ-cli-026",
        "REQ-history-003",
        // Modified by native change chough-0-6-0-human-first-records-...: the Timeline's
        // bounds.
        "271afbcaa66901f05181fb3adf538a47a894c0baa73f19d6d2190e0a8b450945",
    ),
    (
        "REQ-cli-027",
        "REQ-history-004",
        "d807b65dc985f3d8971b83fcb6c1e1cc4615beb80ca30dee2ca595e847acf0d2",
    ),
];

const MOVED_REPORTING: [(&str, &str, &str); 3] = [
    (
        "REQ-cli-020",
        "REQ-reporting-001",
        // Modified by native change chough-timing-skips-direct-native-write-events-without-
        // marking-its-report-partial-189-drive-21 (#189): direct-native-write events
        // (REQ-history-013) start or end no segment and are not an unrecognised kind; then
        // by chough-0-6-0-human-first-records-...: it reads timelines and receipts, and
        // duration_seconds; then by prepare-the-chough-0-6-0-release-...: the start entry
        // starts Cycle 1, and the wall-clock label is said once in the text header.
        // then by 0-6-1-chough-next-with-no-open-session-...: no gate-wait, segment or
        // estimate line repeats the elapsed wall-clock label.
        "7f663a875714902132cc652098cb17920c34ca85ada5edcbd2f4839ec507de19",
    ),
    (
        "REQ-cli-024",
        "REQ-reporting-002",
        // Modified by native change make-the-timing-trace-test-fail-only-when-tracing-
        // changes-the-report-never-because-its-runs-straddle-a-second-169-drive (#169):
        // the trace criterion now reads "identical ... except for the clock".
        "ca7a72a418f2bccc33f625aedbfa58c541a1ed9dbd77e290d50ccc24713db8f1",
    ),
    (
        "REQ-cli-028",
        "REQ-reporting-003",
        "a6d5d17f585dbbd531b337e2102791f0c42c89f211bf5987ea03eb30c4f3608c",
    ),
];

/// Requirements native change chough-0-6-0-human-first-records-... removed with the
/// behaviour they specified (retention, the vault trims and copies, the ledger, session
/// notes); none is in any spec.
const REMOVED: [&str; 9] = [
    "REQ-cli-033",
    "REQ-cli-041",
    "REQ-cli-045",
    "REQ-cli-046",
    "REQ-cli-048",
    "REQ-cli-049",
    "REQ-cli-051",
    "REQ-history-001",
    "REQ-history-004",
];

/// Whether a former cli requirement ID is in either split mapping.
fn mapped(id: &str) -> bool {
    MOVED
        .iter()
        .chain(MOVED_REPORTING.iter())
        .any(|(old, _, _)| *old == id)
}

fn companion(module: &str) -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("specs")
        .join(module)
        .join("requirements.md");
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()))
        .replace("\r\n", "\n")
}

/// Requirement ID to trimmed body, for every `### REQ-` block in a companion.
fn blocks(text: &str) -> BTreeMap<String, String> {
    let mut found = BTreeMap::new();
    let mut current: Option<(String, String)> = None;
    for line in text.split_inclusive('\n') {
        if let Some(rest) = line.strip_prefix("### REQ-") {
            if let Some((id, body)) = current.take() {
                found.insert(id, body.trim().to_string());
            }
            let id = rest.split_whitespace().next().unwrap_or_default();
            current = Some((format!("REQ-{id}"), String::new()));
        } else if let Some((_, body)) = current.as_mut() {
            body.push_str(line);
        }
    }
    if let Some((id, body)) = current {
        found.insert(id, body.trim().to_string());
    }
    found
}

fn sha256(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[test]
fn moved_requirements_keep_their_wording_under_their_new_ids() {
    let cli = blocks(&companion("cli"));
    let history = blocks(&companion("history"));
    for (old, new, digest) in MOVED {
        assert!(!cli.contains_key(old), "{old} must be gone from specs/cli");
        assert!(
            !history.contains_key(old),
            "{old} must not reappear in specs/history"
        );
        if REMOVED.contains(&new) {
            assert!(
                !history.contains_key(new),
                "{new} was removed, yet is in specs/history"
            );
            continue;
        }
        let body = history
            .get(new)
            .unwrap_or_else(|| panic!("{new} (was {old}) missing from specs/history"));
        assert_eq!(sha256(body), digest, "{new} wording differs from {old}");
    }
}

#[test]
fn every_pre_split_cli_requirement_is_still_in_cli_or_mapped() {
    let cli = blocks(&companion("cli"));
    for number in 1..=32 {
        let id = format!("REQ-cli-{number:03}");
        assert!(
            cli.contains_key(&id) != mapped(&id),
            "{id} must be in specs/cli or in the split mapping, not both or neither"
        );
    }
}

#[test]
fn reporting_requirements_keep_their_wording_under_their_new_ids() {
    let cli = blocks(&companion("cli"));
    let history = blocks(&companion("history"));
    let reporting = blocks(&companion("reporting"));
    for (old, new, digest) in MOVED_REPORTING {
        assert!(!cli.contains_key(old), "{old} must be gone from specs/cli");
        assert!(
            !history.contains_key(old) && !reporting.contains_key(old),
            "{old} must not reappear in another module"
        );
        let body = reporting
            .get(new)
            .unwrap_or_else(|| panic!("{new} (was {old}) missing from specs/reporting"));
        assert_eq!(sha256(body), digest, "{new} wording differs from {old}");
    }
}

#[test]
fn every_cli_requirement_before_the_reporting_split_is_still_in_cli_or_mapped() {
    let cli = blocks(&companion("cli"));
    for number in (1..=34).chain([36]) {
        let id = format!("REQ-cli-{number:03}");
        if REMOVED.contains(&id.as_str()) {
            assert!(
                !cli.contains_key(&id),
                "{id} was removed, yet is in specs/cli"
            );
            continue;
        }
        assert!(
            cli.contains_key(&id) != mapped(&id),
            "{id} must be in specs/cli or in a split mapping, not both or neither"
        );
    }
}

/// REQ-cli-009, TC-cli-every-active-case-links-a-current-requirement (S1): every active
/// case links only requirements its module's companion holds, so `chough cases check`
/// in the verify and ci lanes passes once this change's removed requirements are gone
/// from the specs; a retired case keeps its links as history.
#[test]
fn every_active_case_links_a_requirement_the_specs_hold() {
    let cases = Path::new(env!("CARGO_MANIFEST_DIR")).join(".chough/cases");
    let mut missing = vec![];
    let mut files: Vec<_> = std::fs::read_dir(&cases)
        .unwrap()
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.to_string_lossy().ends_with(".cases.toml"))
        .collect();
    files.sort();
    for file in files {
        let text = std::fs::read_to_string(&file).unwrap();
        let parsed: toml::Value = toml::from_str(&text).unwrap();
        let module = parsed["module"].as_str().unwrap();
        let held = blocks(&companion(module));
        for case in parsed["case"].as_array().into_iter().flatten() {
            if case["status"].as_str() != Some("active") {
                continue;
            }
            for requirement in case["requirements"].as_array().into_iter().flatten() {
                let requirement = requirement.as_str().unwrap();
                if !held.contains_key(requirement) {
                    missing.push(format!(
                        "{}: {} links {requirement}",
                        file.file_name().unwrap().to_string_lossy(),
                        case["id"].as_str().unwrap()
                    ));
                }
            }
        }
    }
    assert!(missing.is_empty(), "{missing:#?}");
}

/// REQ-cli-044, REQ-cli-012, REQ-cli-059, TC-cli-send-back-wording-in-the-specs (K6, S6,
/// G1): once the approved deltas are materialized, the cli requirements say a vault link
/// may resolve to a folder, which bug keys are written only when set, and that a ship
/// refuses only for the active session's open gates.
#[test]
fn the_send_back_wording_is_in_the_cli_requirements() {
    let held = blocks(&companion("cli"));
    for (id, phrase) in [
        ("REQ-cli-044", "resolves to an existing file or folder"),
        ("REQ-cli-012", "are written only when set"),
        ("REQ-cli-059", "every open human gate of the active session"),
    ] {
        let body = held
            .get(id)
            .unwrap_or_else(|| panic!("{id} is not in the cli requirements"));
        assert!(body.contains(phrase), "{id} lacks {phrase:?}");
    }
}
