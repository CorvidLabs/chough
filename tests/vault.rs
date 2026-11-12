//! The knowledge directory as an Obsidian-style vault (REQ-cli-044, KNOW-14, KNOW-31):
//! only what has no other home, `index.md` as its reading guide with YAML front matter and
//! standard Markdown links that resolve, the journal and its feed, and no copied text.

mod common;

use common::Fixture;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

/// An archived change with a lesson bundle, finalized at `(index + 1) * 100` seconds.
fn archive(fixture: &Fixture, index: usize, id: &str) {
    let dir = format!(".specsync/archive/changes/2026-0{}-01-{id}", index + 1);
    fixture.write(
        &format!("{dir}/finalization.json"),
        &format!(
            "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {}, \"implementation_commit\": \"{}\"}}\n",
            (index + 1) * 100,
            "1".repeat(40)
        ),
    );
    fixture.write(
        &format!("{dir}/verification.json"),
        &serde_json::to_string_pretty(&json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": ["REQ-cli-001"],
            "acceptance_manifest": {"schema_version": 1, "entries": []},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/change.md"),
        &format!(
            "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}\n\n## Acceptance Criteria\n\n- {id} secret criterion text\n",
            "3".repeat(40)
        ),
    );
    fixture.write(
        &format!("{dir}/lesson-bundle.md"),
        "# Lesson bundle\n\nsecret bundle text\n",
    );
}

/// HI intent, two modules (cli with two lessons in `context.md`, history with a native
/// lesson source of one lesson), rules and decisions, four archived changes, a brief, a
/// cross-project lesson and one journal post, with `library index` run.
fn vault() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(
        "hi/knowledge.md",
        "---\nhi: 1\nfamilies: [KNOW]\n---\n\n# Knowledge\n\n## Criteria\n\n- **KNOW-1**  One knowledge directory, secret wording one.\n",
    );
    fixture.write("specs/cli/cli.spec.md", "---\nmodule: cli\n---\n\n# cli\n");
    fixture.write(
        "specs/cli/requirements.md",
        "# Requirements\n\n### REQ-cli-001\n\nThe helper SHALL keep secret requirement text. Trace: KNOW-1.\n",
    );
    fixture.write(
        "specs/cli/context.md",
        "# Context\n\n## Decisions\n\n- One.\n\n## Lessons\n\n2026-01-01, first lesson (verified): secret lesson one.\n\n2026-01-02, second lesson (observation): secret lesson two.\n",
    );
    fixture.write(
        "specs/history/history.spec.md",
        "---\nmodule: history\n---\n\n# history\n",
    );
    fixture.write(
        "specs/history/history.3md",
        "@plane intent\nhistory\n@plane lesson-1\nsecret native lesson\n",
    );
    fixture.write("AGENTS.md", "# Agents\n");
    fixture.write("docs/DECISIONS.md", "# Decisions\n");
    for (index, id) in ["a1", "a2", "a3", "a4"].iter().enumerate() {
        archive(&fixture, index, id);
    }
    fixture.init();
    fixture.write(
        "knowledge/briefs/older-brief.md",
        "A brief kept as it is.\n",
    );
    fixture.write(
        "knowledge/lessons/2026-01-03-shared-lesson.md",
        "---\nid: 2026-01-03-shared-lesson\ntitle: \"A shared lesson\"\ntype: lesson\ntags: [chough/lesson]\ndate: 2026-01-03\napplies_to: []\nsource: \"SYNTHETIC\"\n---\n\n# A shared lesson\n",
    );
    // A journal link names a project path Git tracks (K4), so the sources are committed.
    fixture.commit("sources");
    fixture.ok(&[
        "journal",
        "add",
        "--kind",
        "learned",
        "--title",
        "A SYNTHETIC finding",
        "--body",
        "One short paragraph.",
        "--link",
        "AGENTS.md",
    ]);
    fixture.ok(&["library", "index"]);
    fixture
}

/// Every Markdown note under `dir`, sorted.
fn notes(dir: &Path) -> Vec<PathBuf> {
    let mut found = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            found.extend(notes(&path));
        } else if path.extension().is_some_and(|ext| ext == "md") {
            found.push(path);
        }
    }
    found.sort();
    found
}

/// The exact names in `relative`, sorted; on a case-insensitive file system `exists`
/// cannot tell `INDEX.md` from `index.md`.
fn names(fixture: &Fixture, relative: &str) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(fixture.root().join(relative))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

/// The YAML front matter block of a note, without its fences.
fn front_matter(text: &str) -> &str {
    let rest = text
        .strip_prefix("---\n")
        .unwrap_or_else(|| panic!("no front matter:\n{text}"));
    &rest[..rest.find("\n---\n").expect("unterminated front matter")]
}

/// REQ-cli-044: every note Chough writes under `knowledge/`, the index and each journal
/// post, starts with front matter giving `id`, `title`, `type` and `tags`; the directory
/// holds only `index.md`, `journal/`, `lessons/` and what was there before.
#[test]
fn every_generated_vault_note_starts_with_front_matter() {
    let fixture = vault();
    assert_eq!(
        names(&fixture, "knowledge"),
        ["briefs", "index.md", "journal", "lessons"]
    );
    let knowledge = fixture.root().join("knowledge");
    let written: Vec<PathBuf> = notes(&knowledge)
        .into_iter()
        .filter(|path| !path.starts_with(knowledge.join("briefs")))
        .collect();
    assert_eq!(written.len(), 3, "{written:?}");
    for note in written {
        let text = fs::read_to_string(&note).unwrap();
        let properties = front_matter(&text);
        for key in ["id: ", "title: ", "type: ", "tags:"] {
            assert!(
                properties.lines().any(|line| line.starts_with(key)),
                "{} lacks {key:?}:\n{properties}",
                note.display()
            );
        }
    }
    let index = fixture.read("knowledge/index.md");
    assert!(front_matter(&index).contains("type: guide"), "{index}");
}

/// REQ-cli-043: the index links every rule, skill, decision and policy file that exists,
/// and none that is missing; the open bugs, each linked; and no `rules.md`, `README.md`,
/// `INDEX.md` or note folder is generated.
#[test]
fn the_rules_note_links_rules_skills_and_decisions_that_exist() {
    let fixture = vault();
    let added = fixture.ok(&[
        "bug",
        "add",
        "--title",
        "A SYNTHETIC bug title",
        "--symptom",
        "SYNTHETIC symptom",
        "--trigger",
        "SYNTHETIC trigger",
        "--expected",
        "SYNTHETIC expected",
        "--actual",
        "SYNTHETIC actual",
        "--environment",
        "test-local",
        "--requirement",
        "REQ-cli-001",
        "--scope",
        "out-of-scope",
    ]);
    let bug = added["id"].as_str().unwrap_or_default().to_string();
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    for expected in [
        "[AGENTS.md](../AGENTS.md)",
        "[docs/DECISIONS.md](../docs/DECISIONS.md)",
        "[specs/cli/context.md](../specs/cli/context.md)",
        "[specs/history/history.3md](../specs/history/history.3md)",
        "[.agents/skills/chough-workflow/SKILL.md](../.agents/skills/chough-workflow/SKILL.md)",
    ] {
        assert!(index.contains(expected), "missing {expected:?} in\n{index}");
    }
    for missing in ["CLAUDE.md", "hi/AGENTS.md", ".specsync/sdd.json"] {
        assert!(
            !index.contains(&format!("]({missing})"))
                && !index.contains(&format!("](../{missing})")),
            "a missing {missing} was linked:\n{index}"
        );
    }
    let bug_line = index
        .lines()
        .find(|line| line.contains(&bug))
        .unwrap_or_else(|| panic!("no line for {bug}:\n{index}"));
    assert!(
        bug_line.starts_with(&format!("- [{bug}](../.chough/bugs/{bug}."))
            && bug_line.contains("A SYNTHETIC bug title"),
        "{bug_line}"
    );
    let all = names(&fixture, "knowledge");
    for generated in [
        "rules.md",
        "README.md",
        "INDEX.md",
        "timeline.md",
        "archives",
        "modules",
        "intent",
        "bugs",
        "sessions",
        "attachments",
        "timeline",
    ] {
        assert!(
            !all.iter().any(|name| name == generated),
            "{generated}: {all:?}"
        );
    }
}

/// REQ-cli-043: the index links each module's lesson source with its lesson count, every
/// cross-project lesson, the newest 3 lesson bundles, the journal, its feed and the native
/// archive, and copies no lesson, criterion, requirement or record text.
#[test]
fn the_index_links_lessons_bundles_and_the_journal_and_copies_no_text() {
    let fixture = vault();
    let index = fixture.read("knowledge/index.md");
    for expected in [
        "cli: [specs/cli/context.md](../specs/cli/context.md), 2 lessons",
        "history: [specs/history/history.3md](../specs/history/history.3md), 1 lesson",
        "[lessons/2026-01-03-shared-lesson.md](lessons/2026-01-03-shared-lesson.md)",
        "[a4](../.specsync/archive/changes/2026-04-01-a4/lesson-bundle.md)",
        "[a3](../.specsync/archive/changes/2026-03-01-a3/lesson-bundle.md)",
        "[a2](../.specsync/archive/changes/2026-02-01-a2/lesson-bundle.md)",
        "[journal](journal/)",
        "[feed](journal/feed.xml)",
        "[.specsync/archive/changes/](../.specsync/archive/changes/)",
        "[A SYNTHETIC finding](journal/",
        "[briefs/older-brief.md](briefs/older-brief.md)",
    ] {
        assert!(index.contains(expected), "missing {expected:?} in\n{index}");
    }
    assert!(
        !index.contains("2026-01-01-a1"),
        "a4 to a2 are the newest 3:\n{index}"
    );
    assert!(!index.contains("secret"), "text was copied:\n{index}");
}

/// REQ-cli-044: every relative Markdown link in the vault resolves to an existing file.
#[test]
fn every_markdown_link_in_the_vault_resolves() {
    let fixture = vault();
    let knowledge = fixture.root().join("knowledge");
    let mut checked = 0;
    for note in notes(&knowledge) {
        let text = fs::read_to_string(&note).unwrap();
        let mut rest = text.as_str();
        while let Some(start) = rest.find("](") {
            rest = &rest[start + 2..];
            let end = rest.find(')').unwrap();
            let target = &rest[..end];
            rest = &rest[end..];
            if target.starts_with("http") || target.starts_with('#') {
                continue;
            }
            let path = target.split('#').next().unwrap();
            let resolved = note.parent().unwrap().join(path);
            assert!(
                resolved.exists(),
                "{} links to missing {target}",
                note.display()
            );
            checked += 1;
        }
    }
    assert!(checked >= 20, "only {checked} links checked");
}

/// REQ-cli-044: regenerating gives identical bytes for `index.md` and the feed, and leaves
/// every other note in the vault as it was.
#[test]
fn the_vault_regenerates_identically() {
    let fixture = vault();
    let knowledge = fixture.root().join("knowledge");
    let read = || {
        let mut files = notes(&knowledge);
        files.push(knowledge.join("journal/feed.xml"));
        files
            .into_iter()
            .map(|path| (path.clone(), fs::read(path).unwrap()))
            .collect::<Vec<_>>()
    };
    let first = read();
    fixture.ok(&["library", "index"]);
    assert_eq!(read(), first);
    fixture.ok(&["library", "index", "--check"]);
}

/// Review 5779587370, REQ-cli-044, REQ-records-012: a README byte-identical to one an
/// earlier Chough generated folds into `index.md` once its bytes are committed; an edited
/// README is kept byte-identical, by `library index` and `library init` alike.
#[test]
fn an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept() {
    let fixture = vault();
    let earlier = include_str!("fixtures/knowledge-readme-at-8fbbbe7.md");
    fixture.write("knowledge/README.md", earlier);
    // Not committed yet: kept and reported, never removed.
    let kept = fixture.ok(&["library", "index"]);
    assert_eq!(kept["readme"]["status"], "kept", "{kept:#}");
    assert_eq!(fixture.read("knowledge/README.md"), earlier);
    fixture.commit("an earlier README");
    let folded = fixture.ok(&["library", "index"]);
    assert_eq!(folded["readme"]["status"], "folded", "{folded:#}");
    assert!(!names(&fixture, "knowledge").contains(&"README.md".to_string()));
    let edited = format!("{earlier}\nOur own note.\n");
    fixture.write("knowledge/README.md", &edited);
    fixture.commit("an edited README");
    let out = fixture.ok(&["library", "index"]);
    assert_eq!(out["readme"]["status"], "kept", "{out:#}");
    fixture.ok(&["library", "init"]);
    assert_eq!(fixture.read("knowledge/README.md"), edited);
    fixture.ok(&["library", "index", "--check"]);
}

/// Windows CI on #107: an untouched earlier README checked out with CRLF line endings is
/// still recognized and folds. (The vault reads no `Trace:` list any more, so the
/// unmatched backtick this test once covered has nothing left to hide.)
#[test]
fn crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list() {
    let fixture = vault();
    let earlier = include_str!("fixtures/knowledge-readme-at-8fbbbe7.md").replace("\r\n", "\n");
    fixture.write("knowledge/README.md", &earlier.replace('\n', "\r\n"));
    fixture.commit("an earlier README with CRLF");
    let folded = fixture.ok(&["library", "index"]);
    assert_eq!(folded["readme"]["status"], "folded", "{folded:#}");
    assert!(!names(&fixture, "knowledge").contains(&"README.md".to_string()));
}

/// DRIVE-12, REQ-cli-047: a rebuild check names any stale or missing generated file,
/// `index.md` or `journal/feed.xml`, writes nothing, and ignores line endings.
#[test]
fn index_check_names_stale_files_and_passes_after_a_rebuild() {
    let fixture = vault();
    fixture.ok(&["library", "index", "--check"]);

    let index = fixture.read("knowledge/index.md");
    let merged = format!("{index}\n- a line a union merge kept\n");
    fixture.write("knowledge/index.md", &merged);
    let error = fixture.fails(&["library", "index", "--check"]);
    assert!(
        error.contains("knowledge/index.md") && error.contains("chough library index"),
        "{error}"
    );
    assert_eq!(
        fixture.read("knowledge/index.md"),
        merged,
        "the check writes nothing"
    );
    fixture.ok(&["library", "index"]);
    fixture.ok(&["library", "index", "--check"]);

    // A CRLF checkout of a current file is current.
    let feed = "knowledge/journal/feed.xml";
    let text = fixture.read(feed);
    fixture.write(feed, &text.replace('\n', "\r\n"));
    fixture.ok(&["library", "index", "--check"]);

    // A missing generated file is named.
    fs::remove_file(fixture.root().join(feed)).unwrap();
    let error = fixture.fails(&["library", "index", "--check"]);
    assert!(error.contains(feed), "{error}");
}

/// REQ-cli-015, KNOW-6: existing snapshots and briefs stay byte-identical and linked, and
/// `library snapshot` and `library archive` are gone, the help naming `library index`.
#[test]
fn snapshots_and_briefs_stay_as_they_are_and_snapshot_and_archive_are_gone() {
    let fixture = vault();
    let manifest = "{\"source\": \"SYNTHETIC\"}\n";
    fixture.write("knowledge/snapshots/SNAP-old/manifest.json", manifest);
    fixture.write("knowledge/snapshots/SNAP-old/copy.md", "an old copy\n");
    fixture.write("library/briefs/earliest.md", "an earliest brief\n");
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    for expected in [
        "[snapshots/SNAP-old/manifest.json](snapshots/SNAP-old/manifest.json)",
        "[library/briefs/earliest.md](../library/briefs/earliest.md)",
    ] {
        assert!(index.contains(expected), "missing {expected:?} in\n{index}");
    }
    assert_eq!(
        fixture.read("knowledge/snapshots/SNAP-old/manifest.json"),
        manifest
    );
    assert_eq!(
        fixture.read("knowledge/briefs/older-brief.md"),
        "A brief kept as it is.\n"
    );
    for gone in ["snapshot", "archive"] {
        let error = fixture.fails(&["library", gone]);
        assert!(error.contains("unrecognized subcommand"), "{error}");
    }
    let help = fixture.run(&["library", "--help"]);
    let help = String::from_utf8_lossy(&help.stdout);
    assert!(help.contains("index"), "{help}");
}

/// REQ-cli-043, KNOW-12 (BUG-the-lesson-count-misses-dated-lessons-not-written-as-a-date-followed-by-bbd611173226150c9f13f54c):
/// the index counts every lesson entry of a `## Lessons` section that starts with a
/// `YYYY-MM-DD` date followed by a space, comma or colon, as a list item or a paragraph,
/// and every `### YYYY-MM-DD` heading; other lines, and dates outside the section, do
/// not count.
#[test]
fn the_lesson_count_reads_every_dated_entry() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.write("specs/csv/csv.spec.md", "# CSV\n");
    fixture.write(
        "specs/csv/context.md",
        "# Context\n\n2026-10-01, a dated line before the section does not count.\n\n\
         ## Lessons\n\n\
         - 2026-11-01 a list item with a space.\n\
         - 2026-11-02: a list item with a colon.\n\
         * 2026-11-03, a starred item with a comma.\n\n\
         2026-11-04 a paragraph lesson that wraps\n\
         onto a second line.\n\n\
         ### 2026-11-05\n\n\
         The heading above is the lesson.\n\n\
         ### 2026-11-06: a heading with its gist\n\n\
         Not dated.\n\
         - 2026-1-07 a date with one-digit month.\n\
         - 20261108 a date without dashes.\n\
         - 2026-11-09x a date with no separator.\n\
         - Learned on 2026-11-10, a date not at the start.\n\n\
         ## Decisions\n\n\
         - 2026-11-11, a dated decision after the section does not count.\n",
    );
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    assert!(
        index.contains("csv: [specs/csv/context.md](../specs/csv/context.md), 6 lessons\n"),
        "{index}"
    );
}

/// The link targets of `line`, read as a Markdown renderer reads them: a `](` whose `]`
/// a backslash escapes is text, not a link.
fn unescaped_targets(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut targets = vec![];
    let mut at = 0;
    while let Some(found) = line[at..].find("](") {
        let close = at + found;
        let slashes = bytes[..close]
            .iter()
            .rev()
            .take_while(|byte| **byte == b'\\')
            .count();
        if slashes % 2 == 0 {
            let start = close + 2;
            let end = line[start..]
                .find(')')
                .map_or(line.len(), |end| start + end);
            targets.push(line[start..end].to_string());
        }
        at = close + 2;
    }
    targets
}

/// `text` as a Markdown renderer shows it: a backslash before ASCII punctuation is
/// dropped and the punctuation kept.
fn rendered(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek().is_some_and(char::is_ascii_punctuation) {
            out.push(chars.next().unwrap_or_default());
        } else {
            out.push(c);
        }
    }
    out
}

/// REQ-cli-044, KNOW-14 (BUG-the-knowledge-index-puts-post-bug-and-archive-titles-into-link-text-une-f7c4bba9d926154fd504513d):
/// a post, bug or archived change title is inert in the index. On its line, the only link
/// is the record's own, no `<` is left unescaped, so no HTML, and the text a reader sees is
/// the title itself.
#[test]
fn index_titles_are_inert_markdown() {
    let title = r#"Tags <b> & "quotes" [x](y) ]] `code` back\slash"#;
    let fixture = Fixture::new();
    fixture.init();
    archive(&fixture, 0, "inert");
    fixture.write(
        ".specsync/archive/changes/2026-01-01-inert/change.md",
        "---\nid: inert\nstate: archived\n---\n\n# Archive <script>alert(1)</script> [v](w) title\n",
    );
    fixture.write(
        ".specsync/archive/changes/2026-01-01-inert/lesson-bundle.md",
        "bundle\n",
    );
    fixture.commit("sources");
    fixture.ok(&[
        "journal", "add", "--kind", "learned", "--title", title, "--body", "x",
    ]);
    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Bug <i>title</i> with [z](q) inside",
        "--symptom",
        "SYNTHETIC symptom",
        "--trigger",
        "SYNTHETIC trigger",
        "--expected",
        "SYNTHETIC expected",
        "--actual",
        "SYNTHETIC actual",
        "--environment",
        "local",
    ]);
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/index.md");
    let line_with = |needle: &str| {
        index
            .lines()
            .find(|line| line.contains(needle))
            .unwrap_or_else(|| panic!("no line with {needle:?} in\n{index}"))
            .to_string()
    };
    let post = line_with("journal/2026-");
    let bug = line_with(".chough/bugs/");
    let bundle = line_with("lesson-bundle.md");
    for (line, own) in [
        (&post, "journal/"),
        (&bug, ".chough/bugs/"),
        (&bundle, "lesson-bundle.md"),
    ] {
        let targets = unescaped_targets(line);
        assert_eq!(targets.len(), 1, "one link on {line:?}: {targets:?}");
        assert!(targets[0].contains(own), "{line:?}: {targets:?}");
        let bytes = line.as_bytes();
        for (at, byte) in bytes.iter().enumerate() {
            if *byte == b'<' {
                assert!(at > 0 && bytes[at - 1] == b'\\', "raw HTML on {line:?}");
            }
        }
    }
    let shown = post
        .strip_prefix("- [")
        .and_then(|rest| rest.rsplit_once("](journal/"))
        .map(|(text, _)| text)
        .unwrap_or_else(|| panic!("{post:?}"));
    assert_eq!(rendered(shown), title, "{post:?}");
    assert!(
        rendered(&bug).contains("Bug <i>title</i> with [z](q) inside"),
        "{bug:?}"
    );
    assert!(
        rendered(&bundle).contains("Archive <script>alert(1)</script> [v](w) title"),
        "{bundle:?}"
    );
}

/// REQ-cli-043, KNOW-12 (BUG-the-knowledge-index-lists-every-open-bug-with-no-bound-so-a-few-hundred-539000056d8d9cc6f09b429a):
/// the index lists at most 20 open bugs, blocking first and then the newest, with their
/// total, and links `.chough/bugs/` naming `chough bug list` for the rest, so 1,500 open
/// bugs with long titles keep it within its bound and `--check` passes. A closed bug is
/// neither listed nor counted.
#[test]
fn the_index_lists_at_most_twenty_open_bugs_blocking_first_then_newest() {
    let fixture = Fixture::new();
    fixture.init();
    let bug = |n: usize, scope: &str, status: &str, created: &str| {
        let id = format!("2026-10-01-synthetic-open-bug-{n:04}");
        let title = format!(
            "SYNTHETIC open bug {n:04} with a long title that fills the line {}",
            "x".repeat(40)
        );
        fixture.write(
            &format!(".chough/bugs/{id}.md"),
            &format!(
                "---\nkind: bug\nid: {id}\naliases: []\ntitle: {title}\nstatus: {status}\n\
                 classification: product-defect\nscope: {scope}\nimpact: Not yet assessed.\n\
                 criteria: []\nrequirements: []\ncases: []\nevidence: []\nreceipts: []\n\
                 environment: local\nfound_by: tester\nrelated: []\ncreated: {created}\n\
                 updated: {created}\n---\n# {title}\n\n**What happened:** x\n\n\
                 **Steps:** y\n\n**Expected:** z\n\n**Why it matters:** w\n\n\
                 **Fix:** Not fixed yet.\n\n## Log\n\n- {created} · Reported (tester)\n"
            ),
        );
    };
    for n in 0..1_500 {
        // Created a minute apart, so a higher number is newer.
        let created = format!(
            "2026-10-{:02}T{:02}:{:02}:00Z",
            1 + n / 1_440,
            n % 1_440 / 60,
            n % 60
        );
        let scope = if n == 3 || n == 7 {
            "blocking"
        } else {
            "in-scope"
        };
        bug(n, scope, "open", &created);
    }
    bug(1_500, "blocking", "verified", "2026-10-31T00:00:00Z");
    fixture.ok(&["library", "index"]);
    fixture.ok(&["library", "index", "--check"]);
    let index = fixture.read("knowledge/index.md");
    assert!(index.len() <= 65_536, "{} bytes", index.len());
    let section = index
        .split("## Open bugs\n\n")
        .nth(1)
        .and_then(|rest| rest.split("\n## ").next())
        .unwrap_or_default();
    let listed: Vec<&str> = section
        .lines()
        .filter_map(|line| line.strip_prefix("- [")?.split(']').next())
        .collect();
    let id = |n: usize| format!("2026-10-01-synthetic-open-bug-{n:04}");
    let mut expected = vec![id(7), id(3)];
    expected.extend((1_482..1_500).rev().map(id));
    assert_eq!(listed, expected, "{section}");
    assert!(section.contains("1500 open bugs"), "{section}");
    assert!(
        section.contains("[.chough/bugs/](../.chough/bugs/)"),
        "{section}"
    );
    assert!(section.contains("`chough bug list`"), "{section}");
    assert!(!section.contains(&id(1_500)), "{section}");
}
