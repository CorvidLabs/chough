//! The knowledge journal and its feed (REQ-records-011, KNOW-33): one short post per
//! finished session, written by create-new and never rewritten, `chough journal add`
//! for any other post, and an Atom feed regenerated with identical bytes.
#![cfg(unix)]

mod common;

use common::Fixture;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::Path;

const ZERO: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const PR: &str = "https://example.invalid/SYNTHETIC-org/SYNTHETIC-repo/pull/7";
const ARCHIVE: &str = ".specsync/archive/changes/2026-05-01-c1";

/// One archived native change, c1, a stub `specsync` reporting it archived and
/// authenticated, and the checkpoint at accept on c1 with a PR, so `stage advance` moves
/// into done. Where it stands holds a paragraph longer than a post may carry.
fn at_accept() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
    fixture.write(
        "bin/specsync",
        "#!/bin/sh\nprintf '{\"summary\":{\"id\":\"%s\",\"state\":\"archived\",\"approval_valid\":true,\"correction_valid\":true,\"terminal_evidence\":{\"validity\":\"authenticated-history\"}},\"acceptance_entries\":[]}\\n' \"$3\"\n",
    );
    fixture.executable("bin/specsync");
    fixture.hi_stand_in();
    fixture.write(
        &format!("{ARCHIVE}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"c1\", \"timestamp\": 100, \"implementation_commit\": \"{}\"}}\n",
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{ARCHIVE}/verification.json"),
        &serde_json::to_string_pretty(&json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": [
                {"path": "specs/cli/cli.spec.md", "kind": "file", "payload_digest": ZERO}
            ]},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{ARCHIVE}/change.md"),
        &format!(
            "---\nid: c1\nstate: archived\nbase_commit: {}\n---\n\n# Title of c1\n\n## Intent\n\nx\n\n## Acceptance Criteria\n\n- c1 works\n",
            "3".repeat(40)
        ),
    );
    fixture.init();
    fixture.set_front("stage", "accept");
    fixture.set_front("native_change", "c1");
    fixture.set_front("pr", PR);
    let file = fixture.session_file();
    let text = fixture.read(&file);
    let long =
        "The SYNTHETIC change shipped and every case passed on the recorded revision. ".repeat(30);
    let stands = format!("## Where it stands\n\n{}\n\n", long.trim_end());
    let (head, rest) = text
        .split_once("## Where it stands\n")
        .expect("a Where it stands section");
    let rest = rest
        .find("\n## ")
        .map_or(String::new(), |at| rest[at + 1..].to_string());
    fixture.write(&file, &format!("{head}{stands}{rest}"));
    fixture.commit("fixture");
    fixture
}

/// Every file under `relative`, with its bytes.
fn tree(fixture: &Fixture, relative: &str) -> BTreeMap<String, Vec<u8>> {
    let mut found = BTreeMap::new();
    let mut stack = vec![fixture.root().join(relative)];
    while let Some(folder) = stack.pop() {
        for entry in std::fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                stack.push(path);
            } else {
                let name = path
                    .strip_prefix(fixture.root())
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                found.insert(name, std::fs::read(&path).unwrap());
            }
        }
    }
    found
}

/// The front matter and body of a post.
fn split(text: &str) -> (&str, &str) {
    text.strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .expect("a post starts with front matter")
}

/// The value of a scalar front matter field, unquoted.
fn field(front: &str, key: &str) -> String {
    front
        .lines()
        .find_map(|line| line.strip_prefix(&format!("{key}: ")))
        .map(|value| value.trim().trim_matches('"').to_string())
        .unwrap_or_default()
}

/// The items of a list front matter field, written inline (`[a, b]`) or as a block.
fn items(front: &str, key: &str) -> Vec<String> {
    let inline = field(front, key);
    if let Some(list) = inline
        .strip_prefix('[')
        .and_then(|rest| rest.strip_suffix(']'))
    {
        return list
            .split(", ")
            .filter(|item| !item.is_empty())
            .map(|item| item.trim_matches('"').to_string())
            .collect();
    }
    front
        .lines()
        .skip_while(|line| *line != format!("{key}:"))
        .skip(1)
        .map_while(|line| line.strip_prefix("  - "))
        .map(|item| item.trim_matches('"').to_string())
        .collect()
}

/// Every relative target of a Markdown link in `text`.
fn relative_links(text: &str) -> Vec<String> {
    text.split("](")
        .skip(1)
        .filter_map(|rest| rest.split_once(')').map(|(target, _)| target.to_string()))
        .filter(|target| !target.starts_with("http://") && !target.starts_with("https://"))
        .collect()
}

/// A small well-formedness check, since the crate takes no XML dependency: one root, every
/// tag closed in order, attributes quoted, and text holding only escaped `&` and no `<`.
fn well_formed(xml: &str) -> Result<(), String> {
    let mut rest = xml
        .strip_prefix("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n")
        .ok_or("no XML declaration")?;
    let mut open: Vec<String> = vec![];
    let mut roots = 0;
    while let Some(at) = rest.find('<') {
        let text = &rest[..at];
        for (index, _) in text.match_indices('&') {
            let entity = &text[index..];
            if !["&amp;", "&lt;", "&gt;", "&quot;", "&apos;"]
                .iter()
                .any(|known| entity.starts_with(known))
            {
                return Err(format!("unescaped & in {text:?}"));
            }
        }
        if open.is_empty() && !text.trim().is_empty() {
            return Err(format!("text outside the root: {text:?}"));
        }
        let end = rest[at..].find('>').ok_or("an unclosed tag")? + at;
        let tag = &rest[at + 1..end];
        if tag.contains('<') {
            return Err(format!("< inside a tag: {tag:?}"));
        }
        if tag.matches('"').count() % 2 != 0 {
            return Err(format!("an unquoted attribute: {tag:?}"));
        }
        if let Some(name) = tag.strip_prefix('/') {
            if open.pop().as_deref() != Some(name) {
                return Err(format!("</{name}> closes nothing open"));
            }
        } else {
            let name = tag
                .trim_end_matches('/')
                .split_whitespace()
                .next()
                .ok_or("an empty tag")?
                .to_string();
            if open.is_empty() {
                roots += 1;
            }
            if !tag.ends_with('/') {
                open.push(name);
            }
        }
        rest = &rest[end + 1..];
    }
    if !open.is_empty() || roots != 1 || !rest.trim().is_empty() {
        return Err(format!("{roots} roots, still open: {open:?}"));
    }
    Ok(())
}

/// REQ-records-011, TC-records-journal-and-feed: the move into done writes one `shipped`
/// post within 1,200 bytes that links the PR, the archived story and the change's archive
/// folder, and the feed and index list it.
#[test]
fn done_writes_a_short_post_and_the_feed_lists_it() {
    let fixture = at_accept();
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    let id = moved["applied"]["session_archive"]["id"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let story = format!(".chough/archive/sessions/{id}.md");
    assert!(fixture.exists(&story), "{moved:#}");
    let post = &moved["applied"]["journal_post"];
    assert_eq!(post["status"], "written", "{moved:#}");
    let path = post["post"].as_str().unwrap_or_default().to_string();
    assert!(
        path.starts_with("knowledge/journal/") && path.ends_with(".md"),
        "{moved:#}"
    );
    let name = path
        .trim_start_matches("knowledge/journal/")
        .trim_end_matches(".md")
        .to_string();
    let text = fixture.read(&path);
    let (front, body) = split(&text);
    assert_eq!(field(front, "id"), name, "{text}");
    assert_eq!(field(front, "type"), "journal", "{text}");
    assert_eq!(field(front, "kind"), "shipped", "{text}");
    assert_eq!(field(front, "title"), "Ship a focused change", "{text}");
    assert_eq!(field(front, "record"), story, "{text}");
    assert!(name.starts_with(&field(front, "date")), "{text}");
    assert_eq!(
        items(front, "tags"),
        ["chough/journal", "chough/shipped"],
        "{text}"
    );
    assert_eq!(
        items(front, "links"),
        [PR.to_string(), story.clone(), format!("{ARCHIVE}/")],
        "{text}"
    );
    // The body is short, cut at a sentence end, and the story holds the full record.
    assert!(body.len() <= 1_200, "{} bytes: {body}", body.len());
    let paragraph = body
        .lines()
        .find(|line| line.starts_with("The SYNTHETIC change"))
        .unwrap_or_else(|| panic!("no paragraph: {body}"));
    assert!(paragraph.ends_with("revision."), "{paragraph}");
    assert!(
        body.contains(&format!("[the pull request]({PR})")),
        "{body}"
    );
    for target in relative_links(body) {
        let resolved = Path::new("knowledge/journal").join(&target);
        assert!(
            fixture.root().join(&resolved).exists(),
            "{target} does not resolve from the post"
        );
    }
    let feed = fixture.read("knowledge/journal/feed.xml");
    well_formed(&feed).unwrap_or_else(|error| panic!("{error}: {feed}"));
    assert!(
        feed.contains(&format!("<id>urn:chough:journal:{name}</id>")),
        "{feed}"
    );
    assert!(
        feed.contains("<title>Ship a focused change</title>"),
        "{feed}"
    );
    let index = fixture.read("knowledge/index.md");
    assert!(
        index.contains(&format!("[Ship a focused change](journal/{name}.md)")),
        "{index}"
    );
    assert_eq!(
        moved["applied"]["librarian_index"]["status"], "refreshed",
        "{moved:#}"
    );
    fixture.ok(&["library", "index", "--check"]);
}

/// REQ-records-011, TC-records-done-touches-only-its-own-post: around the move into done,
/// nothing under `knowledge/` changes but the new post, `feed.xml`, and `index.md`'s
/// journal list; an earlier post a person edited, a lesson and a brief stay byte-identical.
#[test]
fn done_changes_no_knowledge_file_but_its_own_post_and_the_feed() {
    let fixture = at_accept();
    fixture.write(
        "knowledge/journal/2026-01-02-an-earlier-post.md",
        "---\nid: 2026-01-02-an-earlier-post\ntitle: \"An earlier post\"\ntype: journal\ntags:\n  - chough/journal\ndate: 2026-01-02\nkind: learned\nlinks: []\n---\n\n# An earlier post\n\nA person rewrote this paragraph by hand.\n",
    );
    fixture.write(
        "knowledge/lessons/2026-01-03-a-lesson.md",
        "---\nid: 2026-01-03-a-lesson\ntitle: \"A lesson\"\ntype: lesson\ntags: []\ndate: 2026-01-03\napplies_to: []\nsource: \"\"\n---\n\n# A lesson\n",
    );
    fixture.write("knowledge/briefs/brief.md", "A brief kept as it is.\n");
    // An earlier finished session, so the index already links the archived stories.
    fixture.write(
        ".chough/archive/sessions/2026-01-01-earlier.md",
        "---\nid: 2026-01-01-earlier\n---\n\n# Earlier\n\nClosed 2026-01-01: done.\n",
    );
    fixture.ok(&["library", "index"]);
    fixture.commit("vault");
    let before = tree(&fixture, "knowledge");
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(moved["stage"]["current"], "done", "{moved:#}");
    let post = moved["applied"]["journal_post"]["post"]
        .as_str()
        .unwrap_or_default()
        .to_string();
    let after = tree(&fixture, "knowledge");
    let created: Vec<&String> = after
        .keys()
        .filter(|key| !before.contains_key(*key))
        .collect();
    assert_eq!(created, [&post], "{moved:#}");
    let removed: Vec<&String> = before
        .keys()
        .filter(|key| !after.contains_key(*key))
        .collect();
    assert!(removed.is_empty(), "{removed:?}");
    let changed: Vec<&String> = before
        .keys()
        .filter(|key| after.get(*key).is_some_and(|bytes| *bytes != before[*key]))
        .collect();
    assert_eq!(
        changed,
        ["knowledge/index.md", "knowledge/journal/feed.xml"],
        "{moved:#}"
    );
    // Only the index's list of newest posts differs.
    let outside = |bytes: &[u8]| -> String {
        let text = String::from_utf8_lossy(bytes).into_owned();
        let (head, rest) = text.split_once("Newest posts:\n").expect("a journal list");
        let tail = rest.split_once("\n## ").map_or("", |(_, tail)| tail);
        format!("{head}{tail}")
    };
    assert_eq!(
        outside(&before["knowledge/index.md"]),
        outside(&after["knowledge/index.md"])
    );
    fixture.ok(&["library", "index", "--check"]);
}

/// REQ-records-011: `chough journal add` writes a post of each kind, and a body over
/// 1,200 bytes, an unknown kind or a link to nothing is refused with nothing written.
#[test]
fn journal_add_writes_a_post_of_each_kind_and_refuses_an_oversized_body() {
    let fixture = Fixture::new();
    fixture.init();
    // A link names a project path Git tracks (K4), so the index it links is committed.
    fixture.commit("init");
    for kind in ["shipped", "closed", "intent", "spec", "broke", "learned"] {
        let out = fixture.ok(&[
            "journal",
            "add",
            "--kind",
            kind,
            "--title",
            &format!("A SYNTHETIC {kind} post"),
            "--body",
            "One short paragraph.",
            "--link",
            "knowledge/index.md",
            "--link",
            PR,
        ]);
        let path = out["post"].as_str().unwrap_or_default().to_string();
        let text = fixture.read(&path);
        let (front, body) = split(&text);
        assert_eq!(field(front, "kind"), kind, "{text}");
        assert_eq!(field(front, "type"), "journal", "{text}");
        assert_eq!(
            items(front, "tags"),
            ["chough/journal".to_string(), format!("chough/{kind}")]
        );
        assert_eq!(
            items(front, "links"),
            ["knowledge/index.md".to_string(), PR.to_string()]
        );
        assert!(
            body.contains("[knowledge/index.md](../../knowledge/index.md)"),
            "{body}"
        );
        assert!(
            fixture
                .read("knowledge/journal/feed.xml")
                .contains(&format!("<category term=\"{kind}\"/>"))
        );
    }
    let before = tree(&fixture, "knowledge");
    let oversized = "x".repeat(1_201);
    for (args, says) in [
        (
            vec![
                "--kind", "learned", "--title", "Too long", "--body", &oversized,
            ],
            "1201 bytes",
        ),
        (
            vec!["--kind", "news", "--title", "Unknown", "--body", "x"],
            "not a journal kind",
        ),
        (
            vec![
                "--kind",
                "learned",
                "--title",
                "Dangling",
                "--body",
                "x",
                "--link",
                "SYNTHETIC/missing.md",
            ],
            "neither an http(s) URL",
        ),
    ] {
        let mut command = vec!["journal", "add"];
        command.extend(args);
        let error = fixture.fails(&command);
        assert!(error.contains(says), "{error}");
    }
    let exactly = "y".repeat(1_200);
    fixture.ok(&[
        "journal", "add", "--kind", "learned", "--title", "Exactly", "--body", &exactly,
    ]);
    let mut after = tree(&fixture, "knowledge");
    after.retain(|path, _| !path.contains("-exactly"));
    after.insert(
        "knowledge/index.md".into(),
        before["knowledge/index.md"].clone(),
    );
    after.insert(
        "knowledge/journal/feed.xml".into(),
        before["knowledge/journal/feed.xml"].clone(),
    );
    assert_eq!(after, before, "a refused post wrote something");
}

/// REQ-records-011: a post name that already exists is never overwritten, and a person's
/// edit to a post survives `library index`.
#[test]
fn an_existing_post_is_never_overwritten_and_a_persons_edit_is_kept() {
    let fixture = Fixture::new();
    fixture.init();
    let add = || {
        fixture.ok(&[
            "journal",
            "add",
            "--kind",
            "learned",
            "--title",
            "Same title",
            "--body",
            "First.",
        ])["post"]
            .as_str()
            .unwrap_or_default()
            .to_string()
    };
    let first = add();
    let edited = fixture
        .read(&first)
        .replace("First.", "Edited by a person.");
    fixture.write(&first, &edited);
    let second = add();
    assert_ne!(first, second);
    assert_eq!(fixture.read(&first), edited);
    fixture.ok(&["library", "index"]);
    assert_eq!(fixture.read(&first), edited);
    assert!(fixture.read(&second).contains("First."));
}

/// REQ-records-011, REQ-cli-047: the feed regenerates with identical bytes, `--check`
/// ignores line endings, and fails naming the feed after a hand edit.
#[test]
fn the_feed_regenerates_identically_and_check_names_it_after_a_hand_edit() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&[
        "journal",
        "add",
        "--kind",
        "spec",
        "--title",
        "A & B <c>",
        "--body",
        "Escaped.",
    ]);
    let feed = "knowledge/journal/feed.xml";
    let first = fixture.read(feed);
    well_formed(&first).unwrap_or_else(|error| panic!("{error}: {first}"));
    assert!(
        first.contains("<title>A &amp; B &lt;c&gt;</title>"),
        "{first}"
    );
    fixture.ok(&["library", "index"]);
    assert_eq!(fixture.read(feed), first);
    fixture.write(feed, &first.replace('\n', "\r\n"));
    fixture.ok(&["library", "index", "--check"]);
    fixture.write(feed, &first.replace("Project journal", "Edited"));
    let error = fixture.fails(&["library", "index", "--check"]);
    assert!(error.contains(feed), "{error}");
    fixture.ok(&["library", "index"]);
    assert_eq!(fixture.read(feed), first);
}

/// REQ-records-011: a session closed without done gets a `closed` post.
#[test]
fn a_session_closed_without_done_gets_a_closed_post() {
    let fixture = Fixture::new();
    fixture.init();
    let key = fixture.active_id();
    let closed: Value = fixture.ok(&["session", "close", &key, "--reason", "SYNTHETIC reason"]);
    let post = &closed["journal_post"];
    assert_eq!(post["status"], "written", "{closed:#}");
    let text = fixture.read(post["post"].as_str().unwrap_or_default());
    let (front, _) = split(&text);
    assert_eq!(field(front, "kind"), "closed", "{text}");
    assert_eq!(
        closed["librarian_index"]["status"], "refreshed",
        "{closed:#}"
    );
    fixture.ok(&["library", "index", "--check"]);
}

/// REQ-records-011: with the librarian off, neither done nor `session close` writes a post.
#[test]
fn the_librarian_off_writes_no_post() {
    let fixture = at_accept();
    fixture.ok(&[
        "pref",
        "set",
        "librarian",
        "off",
        "--source",
        "Human: SYNTHETIC no librarian here",
    ]);
    let _ = std::fs::remove_dir_all(fixture.root().join("knowledge"));
    let moved = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        moved["applied"]["journal_post"]["status"], "skipped",
        "{moved:#}"
    );
    assert!(!fixture.exists("knowledge"));
    assert!(
        fixture
            .fails(&[
                "journal", "add", "--kind", "learned", "--title", "x", "--body", "x"
            ])
            .contains("librarian is off")
    );
}

/// Run Git in the fixture and report whether it succeeded.
fn git_succeeds(fixture: &Fixture, args: &[&str]) -> bool {
    std::process::Command::new("git")
        .current_dir(fixture.root())
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["-c", "commit.gpgsign=false"])
        .args(args)
        .output()
        .unwrap()
        .status
        .success()
}

/// REQ-cli-047: with `merge=chough-generated` declared, two branches that each add a post
/// conflict in `index.md` and `feed.xml` under Git's text merge, and merge clean with the
/// driver set up: the merged feed is ours byte for byte and parses, `library index --check`
/// fails naming both files until `library index` rebuilds them, and then lists both posts.
#[test]
fn two_branches_that_each_add_a_post_merge_clean_with_the_generated_driver() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(
        ".gitattributes",
        "knowledge/index.md merge=chough-generated\nknowledge/journal/feed.xml merge=chough-generated\n",
    );
    fixture.commit("base");
    let add = |title: &str| {
        fixture.ok(&[
            "journal",
            "add",
            "--kind",
            "learned",
            "--title",
            title,
            "--body",
            "One line.",
        ])["post"]
            .as_str()
            .unwrap_or_default()
            .to_string()
    };
    fixture.git(&["checkout", "-b", "theirs"]);
    let theirs = add("Their SYNTHETIC post");
    fixture.commit("their post");
    fixture.git(&["checkout", "main"]);
    let ours = add("Our SYNTHETIC post");
    fixture.commit("our post");
    let feed = "knowledge/journal/feed.xml";
    let ours_feed = fixture.read(feed);

    // Without the driver set up, Git falls back to its text merge and conflicts.
    assert!(!git_succeeds(&fixture, &["merge", "--no-edit", "theirs"]));
    fixture.git(&["merge", "--abort"]);

    fixture.git(&[
        "config",
        "--local",
        "merge.chough-generated.driver",
        &format!("'{}' library merge %O %A %B", env!("CARGO_BIN_EXE_chough")),
    ]);
    fixture.git(&["merge", "--no-edit", "theirs"]);
    assert_eq!(fixture.read(feed), ours_feed, "the driver keeps ours");
    well_formed(&fixture.read(feed)).unwrap_or_else(|error| panic!("{error}"));
    assert!(fixture.exists(&ours) && fixture.exists(&theirs));
    let error = fixture.fails(&["library", "index", "--check"]);
    assert!(
        error.contains(feed) && error.contains("knowledge/index.md"),
        "{error}"
    );
    fixture.ok(&["library", "index"]);
    fixture.ok(&["library", "index", "--check"]);
    let rebuilt = fixture.read(feed);
    for post in [&ours, &theirs] {
        let name = post
            .trim_start_matches("knowledge/journal/")
            .trim_end_matches(".md");
        assert!(
            rebuilt.contains(&format!("urn:chough:journal:{name}<")),
            "{rebuilt}"
        );
    }
    well_formed(&rebuilt).unwrap_or_else(|error| panic!("{error}"));
}

/// REQ-cli-047: the driver keeps ours byte for byte with the librarian off too, and
/// refuses when Git did not hand it a file.
#[test]
fn library_merge_keeps_ours_and_refuses_a_missing_file() {
    let fixture = Fixture::new();
    fixture.write("base.xml", "base\n");
    fixture.write("ours.xml", "ours\r\n");
    fixture.write("theirs.xml", "theirs\n");
    // Git hands the driver paths from the repository root; here they are absolute.
    let path = |name: &str| fixture.root().join(name).to_string_lossy().into_owned();
    let (base, ours, theirs) = (path("base.xml"), path("ours.xml"), path("theirs.xml"));
    let out = fixture.run(&["library", "merge", &base, &ours, &theirs]);
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(out.stdout.is_empty());
    assert_eq!(fixture.read("ours.xml"), "ours\r\n");
    assert_eq!(fixture.read("theirs.xml"), "theirs\n");
    let error = fixture.fails(&["library", "merge", &base, &path("gone.xml"), &theirs]);
    assert!(error.contains("gone.xml"), "{error}");
}

/// REQ-cli-044, KNOW-14, KNOW-33 (BUG-journal-add-link-accepts-untracked-ignored-and-git-paths-which-are-dead-2adf94a75196d7f71102d782):
/// `journal add --link` takes an http(s) URL or a project path Git tracks, a file or a
/// folder holding one, so every link resolves in a clone. A path under `.git/`, an
/// ignored path (even one forced into the index), an untracked file and a missing path
/// are refused, naming a project path, and the refusal writes nothing.
#[test]
fn journal_links_name_only_project_paths_git_tracks() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write(".gitignore", "SYNTHETIC-ignored.log\n");
    fixture.write("SYNTHETIC-ignored.log", "forced into the index\n");
    fixture.commit("init");
    fixture.git(&["add", "-f", "SYNTHETIC-ignored.log"]);
    fixture.commit("force-add an ignored file");
    fixture.write("SYNTHETIC-untracked.md", "never committed\n");
    let add = |title: &str, link: &str| {
        let args = [
            "journal", "add", "--kind", "learned", "--title", title, "--body", "x", "--link", link,
        ];
        fixture.run(&args)
    };
    for (title, link) in [
        ("Tracked file", "specs/cli/requirements.md"),
        ("Tracked folder", "specs"),
        ("A URL", PR),
    ] {
        let output = add(title, link);
        assert!(
            output.status.success(),
            "{link}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let before = tree(&fixture, "knowledge");
    for link in [
        ".git/config",
        ".chough/local/active-session",
        "SYNTHETIC-ignored.log",
        "SYNTHETIC-untracked.md",
        "SYNTHETIC/missing.md",
    ] {
        let output = add("Refused link", link);
        let error = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{link} was accepted");
        assert!(
            error.contains("neither an http(s) URL nor a project path Git tracks"),
            "{link}: {error}"
        );
        assert_eq!(
            tree(&fixture, "knowledge"),
            before,
            "{link} wrote something"
        );
    }
}

/// The value of a Git config key in the fixture's repository, or "" when it is unset.
fn git_config(fixture: &Fixture, key: &str) -> String {
    let output = std::process::Command::new("git")
        .current_dir(fixture.root())
        .args(["config", "--local", "--get", key])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

/// Run Git with this build's `chough` first on PATH, as a person's shell would find it,
/// and report whether it succeeded.
fn git_with_chough(fixture: &Fixture, args: &[&str]) -> bool {
    let bin = Path::new(env!("CARGO_BIN_EXE_chough")).parent().unwrap();
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    std::process::Command::new("git")
        .current_dir(fixture.root())
        .env("PATH", path)
        .args([
            "-c",
            "user.name=Test",
            "-c",
            "user.email=test@example.invalid",
        ])
        .args(["-c", "commit.gpgsign=false"])
        .args(args)
        .output()
        .unwrap()
        .status
        .success()
}

/// REQ-cli-047, DRIVE-12 (BUG-a-project-as-chough-init-leaves-it-conflicts-on-knowledge-index-md-and-4390e553d9921f2d81c48108):
/// in a Git repository, `chough init` declares Chough's merge attributes and sets both
/// drivers in the clone's local Git config, so two branches that each add a post merge
/// clean with no hand setup; one `library index` then makes `--check` pass and lists both.
#[test]
fn init_sets_up_the_merge_drivers_so_two_branches_merge_clean() {
    let fixture = Fixture::new();
    fixture.git(&["init"]);
    let out = fixture.ok(&[
        "init",
        "--objective",
        "Ship a focused change",
        "--module",
        "cli",
    ]);
    let attributes = fixture.read(".gitattributes");
    for line in [
        ".chough/bugs/*.md merge=chough-bug",
        "knowledge/index.md merge=chough-generated",
        "knowledge/journal/feed.xml merge=chough-generated",
    ] {
        assert!(
            attributes.lines().any(|declared| declared == line),
            "{line} missing: {attributes:?} {out:#}"
        );
    }
    assert_eq!(
        git_config(&fixture, "merge.chough-bug.driver"),
        "chough bug merge %O %A %B"
    );
    assert_eq!(
        git_config(&fixture, "merge.chough-generated.driver"),
        "chough library merge %O %A %B"
    );
    fixture.commit("base");
    let add = |title: &str| {
        fixture.ok(&[
            "journal",
            "add",
            "--kind",
            "learned",
            "--title",
            title,
            "--body",
            "One line.",
        ])["post"]
            .as_str()
            .unwrap_or_default()
            .to_string()
    };
    fixture.git(&["checkout", "-b", "theirs"]);
    let theirs = add("Their SYNTHETIC post");
    fixture.commit("their post");
    fixture.git(&["checkout", "main"]);
    let ours = add("Our SYNTHETIC post");
    fixture.commit("our post");
    assert!(
        git_with_chough(&fixture, &["merge", "--no-edit", "theirs"]),
        "the merge conflicted"
    );
    let feed = "knowledge/journal/feed.xml";
    well_formed(&fixture.read(feed)).unwrap_or_else(|error| panic!("{error}"));
    fixture.ok(&["library", "index"]);
    fixture.ok(&["library", "index", "--check"]);
    let rebuilt = fixture.read(feed);
    for post in [&ours, &theirs] {
        let name = post
            .trim_start_matches("knowledge/journal/")
            .trim_end_matches(".md");
        assert!(
            rebuilt.contains(&format!("urn:chough:journal:{name}<")),
            "{rebuilt}"
        );
    }
}

/// REQ-cli-047: `chough init` appends only the merge lines a `.gitattributes` lacks, after
/// its last line, changing no line; it honours `library-dir`, and a driver the clone already
/// sets is kept.
#[test]
fn init_appends_only_the_missing_merge_lines_and_keeps_a_set_driver() {
    let fixture = Fixture::new();
    fixture.git(&["init"]);
    let existing = "* text=auto\n.chough/bugs/*.md   merge=chough-bug";
    fixture.write(".gitattributes", existing);
    fixture.git(&[
        "config",
        "--local",
        "merge.chough-bug.driver",
        "SYNTHETIC-own-driver %O %A %B",
    ]);
    fixture.ok(&[
        "pref",
        "set",
        "library-dir",
        "notes",
        "--source",
        "SYNTHETIC: call it notes",
    ]);
    fixture.ok(&[
        "init",
        "--objective",
        "Ship a focused change",
        "--module",
        "cli",
    ]);
    assert_eq!(
        fixture.read(".gitattributes"),
        format!(
            "{existing}\nnotes/index.md merge=chough-generated\nnotes/journal/feed.xml merge=chough-generated\n"
        )
    );
    assert_eq!(
        git_config(&fixture, "merge.chough-bug.driver"),
        "SYNTHETIC-own-driver %O %A %B"
    );
    assert_eq!(
        git_config(&fixture, "merge.chough-generated.driver"),
        "chough library merge %O %A %B"
    );
}

/// REQ-records-011, KNOW-33 (Q1, Leif's decision: posts are time-based): each new post
/// records `time`, the UTC time it was written to the second, and keeps its
/// `<date>-<slug>` name. The index's newest posts and the feed sort newest first by
/// time; a post without one sorts by date and then name after the timed posts of its
/// date, and the index marks it "(no time)". An entry's `<updated>` is its time, and
/// the same posts give identical bytes.
#[test]
fn posts_record_a_time_and_sort_newest_first_by_it() {
    let fixture = Fixture::new();
    fixture.init();
    let added = fixture.ok(&[
        "journal",
        "add",
        "--kind",
        "learned",
        "--title",
        "Written now",
        "--body",
        "One line.",
    ]);
    let name = added["post"].as_str().unwrap_or_default().to_string();
    let text = fixture.read(&name);
    let (front, _) = split(&text);
    let (date, time) = (field(front, "date"), field(front, "time"));
    let stem = name
        .trim_start_matches("knowledge/journal/")
        .trim_end_matches(".md")
        .to_string();
    assert!(stem.starts_with(&format!("{date}-")), "{text}");
    let shape = time.len() == 20
        && time.starts_with(&format!("{date}T"))
        && time.ends_with('Z')
        && [13, 16].iter().all(|at| time.as_bytes()[*at] == b':');
    assert!(
        shape,
        "time {time:?} is not UTC ISO 8601 to the second: {text}"
    );
    let post = |name: &str, date: &str, time: Option<&str>| {
        let time = time.map_or(String::new(), |time| format!("time: {time}\n"));
        fixture.write(
            &format!("knowledge/journal/{name}.md"),
            &format!(
                "---\nid: {name}\ntitle: SYNTHETIC {name}\ntype: journal\ndate: {date}\n{time}kind: learned\n---\n\n# SYNTHETIC {name}\n\nBody.\n"
            ),
        );
    };
    post(
        "2026-05-01-older",
        "2026-05-01",
        Some("2026-05-01T23:00:00Z"),
    );
    post(
        "2026-05-02-a-early",
        "2026-05-02",
        Some("2026-05-02T08:00:00Z"),
    );
    post("2026-05-02-m-untimed", "2026-05-02", None);
    post("2026-05-02-b-untimed", "2026-05-02", None);
    post(
        "2026-05-02-z-late",
        "2026-05-02",
        Some("2026-05-02T20:00:00Z"),
    );
    post("2026-05-03-next-day", "2026-05-03", None);
    fixture.ok(&["library", "index"]);
    let order = [
        stem.as_str(),
        "2026-05-03-next-day",
        "2026-05-02-z-late",
        "2026-05-02-a-early",
        "2026-05-02-b-untimed",
        "2026-05-02-m-untimed",
        "2026-05-01-older",
    ];
    let index = fixture.read("knowledge/index.md");
    let listed: Vec<&str> = index
        .lines()
        .filter(|line| line.starts_with("- ["))
        .filter_map(|line| line.split("](journal/").nth(1)?.split(".md)").next())
        .collect();
    assert_eq!(listed, order, "{index}");
    for (name, untimed) in [
        ("2026-05-03-next-day", true),
        ("2026-05-02-b-untimed", true),
        ("2026-05-02-z-late", false),
        (stem.as_str(), false),
    ] {
        let line = index
            .lines()
            .find(|line| line.contains(&format!("](journal/{name}.md)")))
            .unwrap_or_default();
        assert_eq!(line.ends_with(" (no time)"), untimed, "{line}");
    }
    let feed = fixture.read("knowledge/journal/feed.xml");
    let entries: Vec<&str> = feed
        .split("<id>urn:chough:journal:")
        .skip(1)
        .filter_map(|rest| rest.split('<').next())
        .collect();
    assert_eq!(entries, order, "{feed}");
    for (name, updated) in [
        ("2026-05-02-z-late", "2026-05-02T20:00:00Z"),
        ("2026-05-02-b-untimed", "2026-05-02T00:00:00Z"),
        (stem.as_str(), time.as_str()),
    ] {
        assert!(
            feed.contains(&format!(
                "<id>urn:chough:journal:{name}</id>\n    <updated>{updated}</updated>"
            )),
            "{name}: {feed}"
        );
    }
    assert_eq!(
        feed.split("<updated>")
            .nth(1)
            .and_then(|rest| rest.split('<').next()),
        Some(time.as_str()),
        "the feed's updated is the newest entry's time: {feed}"
    );
    fixture.ok(&["library", "index", "--check"]);
    let first = (fixture.read("knowledge/index.md"), feed.clone());
    fixture.ok(&["library", "index"]);
    assert_eq!(
        (
            fixture.read("knowledge/index.md"),
            fixture.read("knowledge/journal/feed.xml")
        ),
        first
    );
}
