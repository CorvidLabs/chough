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
    // A release assembles every fragment and removes it, and Git keeps no empty folder.
    let Ok(listing) = std::fs::read_dir(&directory) else {
        return Vec::new();
    };
    let mut names: Vec<String> = listing
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

/// The fragments the old Unreleased bullets became (#290) and #278's, kept as fixture
/// copies: a release removes the live ones from `changelog.d/`.
fn converted_fragment_dir() -> PathBuf {
    manifest_dir().join("tests/fixtures/changelog/unreleased-at-289")
}

/// Every `*.md` fragment in the fixture folder, in sorted file-name order, as (name,
/// contents).
fn fragment_files() -> Vec<(String, String)> {
    let directory = converted_fragment_dir();
    let mut names: Vec<String> = std::fs::read_dir(&directory)
        .expect("the fixture fragments are readable")
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
            let contents = std::fs::read_to_string(directory.join(&name))
                .unwrap_or_else(|error| panic!("fixture {name} is readable: {error}"));
            (name, contents)
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
/// `CHANGELOG.md` Unreleased bullet that two open changes would edit together. Once a
/// release has assembled the fragments, the same entry sits in that release's section,
/// 0.6.1 for this change, and still only once.
#[test]
fn the_changelog_names_this_change_before_release() {
    let text = changelog();
    let unreleased = entries(section(&text, "Unreleased"));
    let mut named = unreleased.clone();
    for fragment in fragments() {
        named.extend(entries(&fragment));
    }
    if text.contains("\n## [0.6.1]") {
        named.extend(entries(section(&text, "0.6.1")));
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
        hits[0].contains("changelog.d/<YYYY-MM-DD>-<change-id>.md"),
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

    let output = run_assemble(root, "0.7.0", Some("2026-11-16"));
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert_eq!(
        assembled,
        "## [Unreleased]\n\n## [0.7.0] - 2026-11-16\n\n- A changelog fragment names this change (#290).\n\n## [0.6.0] - 2026-11-12\n",
        "the release heading lands right after the Unreleased section"
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

/// A versioned release (LIFE-16, #290) refuses, writing nothing and keeping the
/// fragments, when the changelog has no Unreleased section to release after, instead of
/// appending the release at the end of the file.
#[test]
fn the_changelog_task_refuses_a_release_without_an_unreleased_section() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    let changelog = root.join("CHANGELOG.md");
    std::fs::write(
        &changelog,
        "## [0.6.0] - 2026-11-12\n\n- A shipped change (#249).\n",
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
        "a versioned release without an Unreleased section must fail, not append at the end"
    );
    let after = std::fs::read_to_string(&changelog).expect("CHANGELOG.md");
    assert_eq!(before, after, "a refused release writes nothing");
    assert!(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md")
            .exists(),
        "a refused release keeps the fragments"
    );
}

/// The release task (LIFE-16, #290) refuses a versioned release whose heading already
/// exists, whatever date the existing heading carries, and writes nothing.
#[test]
fn the_changelog_task_refuses_a_release_that_exists_at_any_date() {
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

    let output = run_assemble(root, "0.7.0", Some("2026-11-17"));
    assert!(
        !output.status.success(),
        "an assembly over an existing release heading must fail, whatever its date"
    );
    let after = std::fs::read_to_string(&changelog).expect("CHANGELOG.md");
    assert_eq!(before, after, "a refused release writes nothing");
    assert!(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md")
            .exists(),
        "a refused release keeps the fragments"
    );
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
    let expected = "## [Unreleased]\n\n- A hand-written open change (#290) is still being written.\n- A changelog fragment names this change (#290).\n\n## [0.6.0] - 2026-11-12\n";
    assert_eq!(
        assembled, expected,
        "the fragment is appended directly after the section's own entry, with no blank line"
    );
    assert!(
        !root
            .join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md")
            .exists(),
        "the changelog task removes the fragments after assembling"
    );
}

/// A fragment that holds prose instead of a bullet (LIFE-16, #290) is refused, naming the
/// file and the line, writing nothing and keeping the fragment, instead of silently
/// dropping the prose and shipping an empty release heading.
#[test]
fn the_changelog_task_refuses_a_fragment_that_is_not_a_bullet() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    let changelog = root.join("CHANGELOG.md");
    std::fs::write(&changelog, "## [Unreleased]\n\n## [0.6.0] - 2026-11-12\n")
        .expect("CHANGELOG.md");
    let fragment = root
        .join("changelog.d")
        .join("2026-11-14-290-changelog-fragments.md");
    std::fs::write(&fragment, "Some prose, not a bullet.\n").expect("the fragment");
    let before = std::fs::read_to_string(&changelog).expect("CHANGELOG.md");

    let output = run_assemble(root, "Unreleased", None);
    assert!(
        !output.status.success(),
        "an assembly over a fragment that is not a bullet must fail"
    );
    let after = std::fs::read_to_string(&changelog).expect("CHANGELOG.md");
    assert_eq!(before, after, "a refused release writes nothing");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("changelog.d/2026-11-14-290-changelog-fragments.md line 1"),
        "the refusal names the fragment file and line: {stderr}"
    );
    assert!(fragment.exists(), "a refused release keeps the fragments");
}

/// A fragment that holds no bullet at all, not even a blank line or whitespace, is
/// refused (LIFE-16, #290), naming the file and writing nothing, instead of assembling
/// an empty release and deleting the fragment.
#[test]
fn the_changelog_task_refuses_a_fragment_that_has_no_bullet() {
    for (name, contents) in [
        ("2026-11-15-99-blank.md", "\n\n"),
        ("2026-11-15-100-spaces.md", "   \n\t\n"),
    ] {
        let temp = tempfile::TempDir::new().expect("a temporary project root");
        let root = temp.path();
        std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
        std::fs::write(
            root.join("CHANGELOG.md"),
            "## [Unreleased]\n\n## [0.6.0] - 2026-11-12\n",
        )
        .expect("CHANGELOG.md");
        let fragment = root.join("changelog.d").join(name);
        std::fs::write(&fragment, contents).expect("the fragment");
        let before = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");

        let output = run_assemble(root, "Unreleased", None);
        assert!(
            !output.status.success(),
            "an assembly over a fragment that holds no bullet must fail"
        );
        let after = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
        assert_eq!(before, after, "a refused release writes nothing");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains(&format!("changelog.d/{name}")),
            "the refusal names the fragment file: {stderr}"
        );
        assert!(fragment.exists(), "a refused release keeps the fragments");
    }
}

/// A fragment whose bullets carry a leading byte order mark (LIFE-16, #290) is accepted,
/// the mark stripped, so a UTF-8 encoded fragment assembles like any other.
#[test]
fn the_changelog_task_accepts_a_fragment_with_a_byte_order_mark() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(
        root.join("CHANGELOG.md"),
        "## [Unreleased]\n\n## [0.6.0] - 2026-11-12\n",
    )
    .expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md"),
        "\u{feff}- A changelog fragment names this change (#290).\n",
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
        assembled.contains("A changelog fragment names this change (#290)"),
        "the BOM'd fragment assembles: {assembled}"
    );
}

/// A blank line inside a bullet, before an indented continuation, is kept (LIFE-16, #290),
/// so the bullet's continuation is not lost on the way to the release heading.
#[test]
fn the_changelog_task_keeps_a_blank_line_inside_a_bullet() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(
        root.join("CHANGELOG.md"),
        "## [Unreleased]\n\n## [0.6.0] - 2026-11-12\n",
    )
    .expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d")
            .join("2026-11-14-290-changelog-fragments.md"),
        "- A changelog fragment names this change (#290).\n\n  With a continuation.\n",
    )
    .expect("the fragment");

    let output = run_assemble(root, "Unreleased", None);
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert_eq!(
        assembled,
        "## [Unreleased]\n\n- A changelog fragment names this change (#290).\n\n  With a continuation.\n\n## [0.6.0] - 2026-11-12\n",
        "the blank line inside the bullet is kept"
    );
}

/// The `## [Unreleased]` section as it read on main at cf82890 (#293). The converted
/// fragments and #278's fragment hold its bullets, so the round trip is compared against
/// this committed copy of that text.
const MAIN_UNRELEASED: &str = r#"## [Unreleased]

- A closed session's archived story writes its What was verified receipt lines with
  ` - ` between the claim and the command or the first fingerprint digits, and a gate
  summary names its change as `Change: <id> - <title>`, so Chough's own records no
  longer carry an em dash that fails the CorvidLabs prose lints (#278, REQ-history-006,
  REQ-cli-061). Stories archived earlier are not rewritten: the journal reads a claim
  from either join (REQ-records-011).
- A relayed gate close's native approval or review, including the review `chough change
  ship` runs, passes the relay to SpecSync 6.3.0 as `--words`, `--where` and
  `--recorded-by`, so SpecSync's approval and review records hold the relayed decision
  (#249, REQ-records-008). Each value is passed joined to its flag (`--words=<words>`), so
  relay text that starts with `-`, which Chough's own flags accept, is not read as a flag.
  SpecSync 6.3.0 then records such a change at workflow version 3, which SpecSync
  6.2.x refuses, so every checkout and CI job of a project that uses it needs 6.3.0.
- `fledge run flake-window` computes the 1.0 flake window from CI history (REQ-cli-062,
  #217). It reads the CI workflow's runs and jobs, the issues labelled `flake` and
  `.chough/bugs/`, writes nothing, and prints a JSON receipt: `clean`, `running` (with
  the latest restart and when the window can first be clean) or `blocked`. A red or
  partly skipped main run, any rerun, a filed flake and a day without a green main run
  on both macOS and Linux restart the window. It exits 2 when GitHub cannot be read in
  full. Its fixtures run in the `CI gate fixtures` job.
- `integrations/` ships optional, versioned example hc templates for Claude Code,
  Codex, Kimi, Grok and a local agent, the local one unqualified until #220
  (REQ-cli-065, #219). Each has hc's three front-matter keys, resumes with
  `chough next`, loads the skill file its own `chough init` creates and holds no local
  path. `integrations/templates.toml` lists each template's status and an ordered list
  of its versions with their SHA-256, and a README says Chough has no runtime
  dependency on hc, Herdr or Rune.
- The workflow skill's stage contract opens with a fast path for a routine change: a
  bounded first read, the session and native-change commands in order, a delta for every
  affected spec, a stop at each human gate, an early first edit and the checks, so a
  local or small-context agent edits early instead of surveying the repository. SKILL.md
  routes to it, and an installed copy of the previous bundle updates to it (LIFE-22).
- CI also runs once a day on main at 05:23 UTC, so the flake window sees macOS and
  Linux every day even when nothing merges. On a scheduled run the `SpecSync Archive`
  jobs take the run's own commit as their trusted base, since the event has neither a
  pull request base nor a previous commit (REQ-cli-064, #217).
- A host-matrix harness in `tools/host-matrix/` drives one scripted Chough cycle per
  agent host, one fresh host run per turn, in a throwaway copy of the shipping sample.
  It stands in for every human gate only as `SIMULATED-host-matrix-not-a-human` and
  judges the result only from the files, Git and native status. Its self-test runs a
  scripted stand-in host with no model, and hosted CI runs it on Ubuntu and macOS. Real
  host adapters follow (REQ-cli-066, #218).
- `chough migrate`'s preview and the receipt's `## Follow-up` also name what `chough doctor` will report on the migrated tree, from doctor's own session check run on the planned records: the finished sessions `chough doctor --fix` archives, and each session whose open human gates are on an archived native change, with each gate's text and the decision it needs, `chough doctor --fix --reason` (#246, REQ-records-009, REQ-cli-055).
- The verify lane's feature-and-bug exercise (`examples/feature-bug-loop.sh`) waits until the clock is past the failing run's recorded second before rerunning the fixed case, so on a fast runner the bug's `verified` status is no longer refused for a pass in the same second (#247). REQ-cli-012's same-second rule is unchanged.
- Record polish: a bug's actual behaviour is its own `Actual:` paragraph under What
  happened; `chough migrate` names each carried entry or receipt in front matter, its
  0.5.x ID kept as the alias, so `chough next` lists the new names; a fresh project's
  `.chough/README.md` no longer lists `artifacts/`; and the README's init paragraph
  names each host's skill directory in one sentence.
- The intent gate summary in `chough gate list` gives each criterion the session references by ID with its sentence from the `hi/` file that holds it, and no diff line, and each delta at a scope gate says which requirement IDs it adds and which it modifies instead of "adds or modifies" (REQ-cli-061).
- `chough library index` links no rule, skill, decision or policy file that Git ignores, such as a host's skill copy under an ignored `.grok/`, so `knowledge/index.md` is the same in a working tree and a fresh clone, and `library index --check` passes there (#245, REQ-cli-043).
- `chough next` with no open session, the normal state after done, says so, names `chough session new <key>` and exits 0 instead of failing, while `chough status` still refuses; `chough timing` says elapsed wall-clock once, in its header, with no gate-wait, case run or fix-time line repeating it, and its native note no longer reads "read read-only" (REQ-cli-021, REQ-history-006, REQ-reporting-001).
- At the 400-observation cap, `chough cases run`, `cases record`, `evidence add` and `checkpoint --observation` refuse before running or writing anything, naming the bound and `chough session shelve <id>`, and `next` and the leave-test gate take a case's latest run only from the observations the checkpoint references (REQ-cli-032).
- `chough change ship` prints SpecSync's output, then a short text summary of at most 12
  lines naming the review, the ship, the gate close and the move into done, in place of
  the gate close's long JSON report and the move's summary; when the summary is longer,
  its last line names how many lines were dropped and points at `--json`, which prints
  every step's full report (REQ-cli-059, #276).
- The leak check no longer refuses an ordinary word that equals a short all-letter host
  name: such a name is a leak only inside a path or a URL or as `<user>@<host>`, while a
  host name of 8 or more characters, or holding a digit, a hyphen or a dot, is still
  refused anywhere, in writes and in `chough doctor` alike (REQ-records-006).
- The leak check now flags an scp-style remote path, `host:/path`, as the host name and
  the path it carries: a home or system root after the colon refuses as a path leak, and
  the host part counts as the local host name when it matches, in writes and in
  `chough doctor` alike (REQ-records-006, #275).

## [0.6.0] - 2026-11-12
"#;

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
    // Only the fragments MAIN_UNRELEASED was fixed from: the numbered conversions of the
    // old Unreleased bullets and #278's, read from their fixture copies, since a release
    // removes the live fragments. A later change adds its own fragment, which this round
    // trip must not read (#289).
    let converted: Vec<(String, String)> = fragment_files()
        .into_iter()
        .filter(|(name, _)| {
            let numbered = name.len() > 3
                && name.as_bytes()[..2].iter().all(u8::is_ascii_digit)
                && name.as_bytes()[2] == b'-';
            numbered || name == "2026-11-15-fix-278-receipt-dash.md"
        })
        .collect();
    for (name, contents) in &converted {
        std::fs::write(root.join("changelog.d").join(name), contents).expect("the fragment");
    }

    let output = run_assemble(root, "Unreleased", None);
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert_eq!(
        assembled, MAIN_UNRELEASED,
        "assembling gives back the same text"
    );
    for (name, _) in &converted {
        assert!(
            !root.join("changelog.d").join(name).exists(),
            "the changelog task removes the fragments after assembling: changelog.d/{name}"
        );
    }
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

/// The release task (LIFE-16, #290) assembles fragments newest first, so on the same
/// date a higher numbered change, here `#100`, lands above a lower numbered change,
/// here `#99`, instead of sorting the numbers as text.
#[test]
fn the_changelog_task_assembles_fragments_newest_first() {
    let temp = tempfile::TempDir::new().expect("a temporary project root");
    let root = temp.path();
    std::fs::create_dir_all(root.join("changelog.d")).expect("changelog.d");
    std::fs::write(
        root.join("CHANGELOG.md"),
        "# Changelog\n\n## [Unreleased]\n",
    )
    .expect("CHANGELOG.md");
    std::fs::write(
        root.join("changelog.d").join("2026-11-15-99-older.md"),
        "- older (#99).\n",
    )
    .expect("the older fragment");
    std::fs::write(
        root.join("changelog.d").join("2026-11-15-100-newer.md"),
        "- newer (#100).\n",
    )
    .expect("the newer fragment");

    let output = run_assemble(root, "0.7.0", Some("2026-11-16"));
    assert!(
        output.status.success(),
        "the changelog task failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let assembled = std::fs::read_to_string(root.join("CHANGELOG.md")).expect("CHANGELOG.md");
    assert_eq!(
        assembled,
        "# Changelog\n\n## [Unreleased]\n## [0.7.0] - 2026-11-16\n\n- newer (#100).\n- older (#99).\n",
        "the fragments assemble newest first"
    );
}
