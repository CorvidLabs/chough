//! The change log lists every shipped change under the release that first contained it
//! (LIFE-16). These tests read `CHANGELOG.md` and the fragments a change adds under
//! `changelog.d/` together: CI clones carry no tags, so where a change first shipped is
//! checked against the tags by hand and pinned here.

use std::path::{Path, PathBuf};

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

fn manifest_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

fn fragment_dir() -> PathBuf {
    manifest_dir().join("changelog.d")
}

/// The contents of every `changelog.d/*.md` fragment, in sorted file-name order.
fn fragments() -> Vec<String> {
    let directory = fragment_dir();
    let mut names: Vec<String> = std::fs::read_dir(&directory)
        .expect("changelog.d is readable")
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let is_markdown = path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension == "md");
            if is_markdown && path.is_file() {
                Some(path.file_name()?.to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            std::fs::read_to_string(directory.join(&name))
                .unwrap_or_else(|error| panic!("changelog.d/{name} is readable: {error}"))
        })
        .collect()
}

fn changelog_script() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tools")
        .join("changelog.py")
}

fn run_assemble(root: &Path, release: &str, date: Option<&str>) -> std::process::Output {
    let mut command = std::process::Command::new("python3");
    command
        .arg(changelog_script())
        .arg("--root")
        .arg(root)
        .arg("--release")
        .arg(release);
    if let Some(date) = date {
        command.arg("--date").arg(date);
    }
    command.output().expect("the changelog task runs python3")
}

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

/// Release preparation (docs/releasing.md step 4): the package version has a dated
/// `## [X.Y.Z] - YYYY-MM-DD` section with at least one entry and a `[X.Y.Z]` link to its
/// GitHub release, so a tag of this commit ships with its notes.
#[test]
fn the_package_version_has_a_dated_section_and_a_release_link() {
    let text = changelog();
    let version = env!("CARGO_PKG_VERSION");
    let heading = format!("\n## [{version}] - ");
    let start = text
        .find(&heading)
        .unwrap_or_else(|| panic!("CHANGELOG.md has no dated [{version}] section"));
    let date = &text[start + heading.len()..][..10];
    assert!(
        date.len() == 10
            && date.chars().enumerate().all(|(index, character)| {
                if index == 4 || index == 7 {
                    character == '-'
                } else {
                    character.is_ascii_digit()
                }
            }),
        "the [{version}] section is dated YYYY-MM-DD, not {date:?}"
    );
    assert!(
        !entries(section(&text, version)).is_empty(),
        "the [{version}] section lists what ships"
    );
    let link =
        format!("\n[{version}]: https://github.com/CorvidLabs/chough/releases/tag/v{version}\n");
    assert!(
        text.contains(&link),
        "CHANGELOG.md links [{version}] to v{version}"
    );
}

/// TC-cli-changelog-0-6-0-names-only-what-ships (t38j's fresh-eyes report): the 0.6.0
/// section's "Also in 0.6.0" list names no command or record 0.6.0 removed, so an
/// upgrader is not told about features that are gone.
#[test]
fn the_0_6_0_also_list_names_nothing_0_6_0_removed() {
    let text = changelog();
    let release = section(&text, "0.6.0");
    let also = release
        .split("### Also in 0.6.0")
        .nth(1)
        .expect("an Also in 0.6.0 list");
    let also = also.split_whitespace().collect::<Vec<_>>().join(" ");
    for removed in [
        "evidence prune",
        "history prune",
        "session adopt",
        "history adopt",
        "frozen legacy log",
        "write events",
        "rolled into the vault",
        "archive note",
    ] {
        assert!(
            !also.contains(removed),
            "Also in 0.6.0 still describes {removed:?}, which 0.6.0 removed"
        );
    }
}

/// LIFE-16, #290: before a release the change the working branch is on is named by the
/// changelog, from the fragment it adds under `changelog.d/` and not from a
/// `CHANGELOG.md` Unreleased bullet that two open changes would edit together.
#[test]
fn the_changelog_names_this_change_before_release() {
    let text = changelog();
    let unreleased = entries(section(&text, "Unreleased"));
    let mut named = unreleased.clone();
    for fragment in fragments() {
        named.extend(entries(&fragment));
    }
    let hits: Vec<String> = named
        .into_iter()
        .filter(|entry| cites(entry, 290))
        .collect();
    assert_eq!(
        hits.len(),
        1,
        "the changelog names this change once: {hits:?}"
    );
    assert!(
        hits[0].contains("changelog.d/"),
        "the #290 entry is the changelog fragment change: {}",
        hits[0]
    );
    assert!(
        !unreleased.iter().any(|entry| cites(entry, 290)),
        "before release the changelog names this change from its fragment, not from the Unreleased section: {unreleased:?}"
    );
}

/// The release task (LIFE-16, #290) assembles every fragment into the release heading
/// given and removes the fragments.
#[test]
fn the_changelog_task_assembles_fragments_into_the_release_and_removes_them() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(root.join("CHANGELOG.md"), "## [0.6.0] - 2026-11-12\n").expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md"),
        "- A changelog fragment names this change (#290).\n",
    )
    .expect("the fragment");

    let output = run_assemble(root, "0.7.0", Some("2026-11-16"));
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert!(
        assembled.contains("## [0.7.0] - 2026-11-16"),
        "the assembled changelog names the release: {assembled}"
    );
    assert!(
        assembled.contains("A changelog fragment names this change (#290)"),
        "the assembled changelog holds the fragment's bullet: {assembled}"
    );
    assert!(
        !root
            .join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md")
            .exists(),
        "the changelog task removes the fragments after assembling"
    );
}

/// A versioned release (LIFE-16, #290) inserts its heading right after the Unreleased
/// section, keeps the open Unreleased text and consumes the fragments.
#[test]
fn the_changelog_task_assembles_a_versioned_release_after_the_unreleased_section() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(
        root.join("CHANGELOG.md"),
        "## [Unreleased]\n\n- An open change (#290) is still awaiting its release.\n\n## [0.6.0] - 2026-11-12\n",
    )
    .expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md"),
        "- A changelog fragment names this change (#290).\n",
    )
    .expect("the fragment");

    let output = run_assemble(root, "0.7.0", Some("2026-11-16"));
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert!(
        assembled.contains("## [0.7.0] - 2026-11-16"),
        "the assembled changelog names the release: {assembled}"
    );
    assert!(
        assembled.contains("A changelog fragment names this change (#290)"),
        "the assembled changelog holds the fragment's bullet: {assembled}"
    );
    assert!(
        assembled.contains("- An open change (#290) is still awaiting its release."),
        "a versioned release leaves the open Unreleased text in place: {assembled}"
    );
    assert!(
        !root
            .join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md")
            .exists(),
        "the changelog task removes the fragments after assembling"
    );
}

/// A release (LIFE-16, #290) is refused, writing nothing, when its version heading is
/// already there, so an assembly can never overwrite a shipped release.
#[test]
fn the_changelog_task_refuses_a_release_that_already_exists() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    let changelog = root.join("CHANGELOG.md");
    std::fs::write(
        &changelog,
        "## [0.7.0] - 2026-11-16\n\n- A changelog fragment names this change (#290).\n",
    )
    .expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md"),
        "- A changelog fragment names this change (#290).\n",
    )
    .expect("the fragment");
    let before = std::fs::read_to_string(&changelog).expect("CHANGELOG.md");

    let output = run_assemble(root, "0.7.0", Some("2026-11-16"));
    assert!(
        !output.status.success(),
        "an assembly over an existing release heading must fail, not overwrite"
    );
    let after = std::fs::read_to_string(&changelog).expect("CHANGELOG.md");
    assert_eq!(before, after, "a refused release writes nothing");
}

/// The release task (LIFE-16, #290) appends the fragments' bullets after what the
/// section already holds, so an open section never loses its hand-written entries.
#[test]
fn the_changelog_task_appends_fragments_after_what_the_section_holds() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(
        root.join("CHANGELOG.md"),
        "## [Unreleased]\n\n- A hand-written open change (#290) is still being written.\n\n## [0.6.0] - 2026-11-12\n",
    )
    .expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md"),
        "- A changelog fragment names this change (#290).\n",
    )
    .expect("the fragment");

    let output = run_assemble(root, "Unreleased", None);
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert!(
        assembled.contains("- A hand-written open change (#290) is still being written."),
        "the section keeps what it already held: {assembled}"
    );
    assert!(
        assembled.contains("- A changelog fragment names this change (#290)"),
        "the assembled changelog holds the fragment's bullet: {assembled}"
    );
    assert!(
        !root
            .join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md")
            .exists(),
        "the changelog task removes the fragments after assembling"
    );
}

/// The release task assembles the fragments the converted Unreleased bullets became
/// back into the Unreleased text, so a release is a pure copy of the fragments' bullets
/// in shipping order, newest first.
#[test]
fn the_changelog_task_assembles_the_unreleased_fragments_gives_back_the_same_text() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(
        root.join("CHANGELOG.md"),
        "## [Unreleased]\n\n## [0.6.0] - 2026-11-12\n",
    )
    .expect("CHANGELOG.md");
    for (index, fragment) in fragments().iter().enumerate() {
        std::fs::write(
            root.join("changelog.d").join(format!("{index:02}.md")),
            fragment,
        )
        .expect("the fragment");
    }

    let output = run_assemble(root, "Unreleased", None);
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    let mut expected = String::from("## [Unreleased]\n\n");
    for fragment in fragments().iter().rev() {
        expected.push_str(fragment.trim_end_matches('\n'));
        expected.push('\n');
    }
    expected.push_str("\n## [0.6.0] - 2026-11-12\n");
    assert_eq!(assembled, expected, "assembling gives back the same text");
    assert!(
        !root.join("changelog.d").join("00.md").exists(),
        "the changelog task removes the fragments after assembling"
    );
}

/// The release task is deterministic: the same fragments give the same bytes.
#[test]
fn the_changelog_task_assembles_the_same_bytes_twice() {
    let first = tempfile::TempDir::new().expect("a temporary project root");
    let second = tempfile::TempDir::new().expect("a temporary project root");
    for root in [first.path(), second.path()] {
        std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
        std::fs::write(
            root.join("CHANGELOG.md"),
            "## [Unreleased]\n\n## [0.6.0] - 2026-11-12\n",
        )
        .expect("CHANGELOG.md");
        std::fs::write(
            root.join("changelog.d")
                .join("2026-11-14-290-changelog-fragments.md"),
            "- A changelog fragment names this change (#290).\n",
        )
        .expect("the fragment");
    }
    let first_output = run_assemble(first.path(), "Unreleased", None);
    let second_output = run_assemble(second.path(), "Unreleased", None);
    assert!(
        first_output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&first_output.stderr)
    );
    assert!(
        second_output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&second_output.stderr)
    );
    let first_text =
        std::fs::read_to_string(first.path().join("CHANGELOG.md")).expect("CHANGELOG.md");
    let second_text =
        std::fs::read_to_string(second.path().join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert_eq!(first_text, second_text);
}
