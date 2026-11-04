//! The 2026-11-03 cli spec split kept every moved requirement's wording
//! (REQ-history-001 to REQ-history-004, formerly REQ-cli-018 and REQ-cli-025 to
//! REQ-cli-027). The digests are of each requirement body, trimmed, at main 7bd1b62.
//! A later native change that edits a moved requirement updates its digest here.

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

/// The structured record bound every requirements companion must fit (src/records.rs).
const RECORD_LIMIT: usize = 65_536;

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
        let mapped = MOVED.iter().any(|(old, _, _)| *old == id);
        assert!(
            cli.contains_key(&id) != mapped,
            "{id} must be in specs/cli or in the split mapping, not both or neither"
        );
    }
}

#[test]
fn every_requirements_companion_fits_the_record_bound() {
    for module in ["cli", "history"] {
        let size = companion(module).len();
        assert!(
            size <= RECORD_LIMIT,
            "specs/{module}/requirements.md is {size} bytes, over {RECORD_LIMIT}"
        );
    }
}
