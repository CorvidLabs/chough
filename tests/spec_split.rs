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
        // Modified by native change keep-one-session-per-objective (#77): per-session cursor.
        "bc8c769b2b2cde3bce9bfc1ce2314090293a44ec5d847c67aa56c2f7ffd326be",
    ),
    (
        "REQ-cli-026",
        "REQ-history-003",
        "d3167c2e28ffae2e850f3dcdbcec2cf6db2fc515c789f017006a1713b582adea",
    ),
    (
        "REQ-cli-027",
        "REQ-history-004",
        "128c08a8a763ba404099891f17da8c28e53da365375a44decfd8755c78208fd2",
    ),
];

const MOVED_REPORTING: [(&str, &str, &str); 3] = [
    (
        "REQ-cli-020",
        "REQ-reporting-001",
        // Modified by native change chough-timing-skips-direct-native-write-events-without-
        // marking-its-report-partial-189-drive-21 (#189): direct-native-write events
        // (REQ-history-013) start or end no segment and are not an unrecognised kind.
        "f10aaac5200dc31c8f976df83b11486f2a19a632c7737a9ad0b78352eebd0c3b",
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
        assert!(
            cli.contains_key(&id) != mapped(&id),
            "{id} must be in specs/cli or in a split mapping, not both or neither"
        );
    }
}
