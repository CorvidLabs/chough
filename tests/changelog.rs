//! The change log lists every shipped change under the release that first contained it
//! (LIFE-16). These tests read only `CHANGELOG.md`: CI clones carry no tags, so where a
//! change first shipped is checked against the tags by hand and pinned here.

use std::path::Path;

fn changelog() -> String {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("CHANGELOG.md");
    std::fs::read_to_string(&path).expect("CHANGELOG.md is readable")
}

/// The body of the `## [<version>]` section, up to the next `## [` heading.
fn section<'a>(text: &'a str, version: &str) -> &'a str {
    let heading = format!("\n## [{version}]");
    let start = text
        .find(&heading)
        .unwrap_or_else(|| panic!("CHANGELOG.md has no [{version}] section"));
    let body = &text[start + heading.len()..];
    let end = body.find("\n## [").unwrap_or(body.len());
    &body[..end]
}

/// Top-level entries of a section: a line starting with `- ` and its indented lines.
fn entries(section: &str) -> Vec<String> {
    let mut entries: Vec<String> = Vec::new();
    for line in section.lines() {
        if let Some(rest) = line.strip_prefix("- ") {
            entries.push(rest.to_owned());
        } else if line.starts_with("  ")
            && let Some(entry) = entries.last_mut()
        {
            entry.push(' ');
            entry.push_str(line.trim_start());
        }
    }
    entries
}

/// Whether an entry cites pull request or issue `number`, as `#<number>` not followed by a digit.
fn cites(entry: &str, number: u32) -> bool {
    let tag = format!("#{number}");
    entry.match_indices(&tag).any(|(at, _)| {
        !entry[at + tag.len()..]
            .chars()
            .next()
            .is_some_and(|next| next.is_ascii_digit())
    })
}

const RELEASED: &[&str] = &["0.4.0", "0.3.0", "0.2.0", "0.1.0"];

#[test]
fn the_sessions_entry_is_under_0_4_0_where_77_first_shipped() {
    let text = changelog();
    let title = "Sessions per objective or native change";
    assert_eq!(
        text.matches(title).count(),
        1,
        "the sessions entry appears exactly once in CHANGELOG.md"
    );
    let found: Vec<(&str, String)> = RELEASED
        .iter()
        .flat_map(|version| {
            entries(section(&text, version))
                .into_iter()
                .filter(|entry| entry.starts_with(title))
                .map(move |entry| (*version, entry))
        })
        .collect();
    assert_eq!(
        found.len(),
        1,
        "one released section holds the sessions entry: {found:?}"
    );
    let (version, entry) = &found[0];
    assert_eq!(
        *version, "0.4.0",
        "#77 (a994efa2) is in v0.4.0 and not in v0.3.0, so its entry belongs under [0.4.0]"
    );
    assert!(
        entry.starts_with(&format!("{title} (#77, REQ-cli-037, REQ-history-006):")),
        "the sessions entry names #77: {entry}"
    );
}

#[test]
fn the_111_legacy_anchor_session_fix_is_under_0_4_0() {
    let text = changelog();
    let found: Vec<(&str, String)> = RELEASED
        .iter()
        .flat_map(|version| {
            entries(section(&text, version))
                .into_iter()
                .filter(|entry| cites(entry, 111))
                .map(move |entry| (*version, entry))
        })
        .collect();
    assert_eq!(found.len(), 1, "one released entry cites #111: {found:?}");
    let (version, entry) = &found[0];
    assert_eq!(
        *version, "0.4.0",
        "#111 (ded0e9ef) is in v0.4.0 and not in v0.3.0, so its entry belongs under [0.4.0]"
    );
    for words in [
        "`chough session new`",
        "legacy history anchor changed",
        "REQ-history-006",
    ] {
        assert!(
            entry.contains(words),
            "the #111 entry names {words}: {entry}"
        );
    }
}

#[test]
fn cites_matches_whole_numbers_only() {
    assert!(cites("fixed (#111, REQ-history-006)", 111));
    assert!(cites("see #111.", 111));
    assert!(!cites("see #1110", 111));
    assert!(!cites("see #11", 111));
}
