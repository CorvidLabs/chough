//! The knowledge directory as an Obsidian-style vault (REQ-cli-044, KNOW-14, KNOW-15):
//! YAML front matter on every generated note, standard Markdown links that resolve, and
//! module, intent and rules notes that connect archived changes to what they touched.

mod common;

use common::Fixture;
use serde_json::json;
use std::fs;
use std::path::{Path, PathBuf};

/// An archived change touching `modules`, verifying `requirements`.
fn archive(fixture: &Fixture, index: usize, id: &str, modules: &[&str], requirements: &[&str]) {
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
        &format!("{dir}/state.json"),
        &serde_json::to_string_pretty(
            &json!({"id": id, "state": "archived", "affected_specs": modules}),
        )
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/verification.json"),
        &serde_json::to_string_pretty(&json!({
            "commit": "2".repeat(40),
            "passed": true,
            "requirement_ids": requirements,
            "acceptance_manifest": {"schema_version": 1, "entries": []},
        }))
        .unwrap(),
    );
    fixture.write(
        &format!("{dir}/change.md"),
        &format!(
            "---\nid: {id}\nstate: archived\nbase_commit: {}\n---\n\n# Title of {id}: with a colon\n\n## Acceptance Criteria\n\n- {id} works\n",
            "3".repeat(40)
        ),
    );
    fixture.write(&format!("{dir}/lesson-bundle.md"), "# Lesson bundle\n");
}

fn vault() -> Fixture {
    let fixture = Fixture::new();
    fixture.write(
        "hi/knowledge.md",
        "---\nhi: 1\nfamilies: [KNOW]\n---\n\n# Knowledge\n\n## Criteria\n\n- **KNOW-1**  One knowledge directory, secret wording one.\n- **KNOW-7**  Old proof leaves, secret wording seven.\n",
    );
    fixture.write(
        "hi/life.md",
        "---\nhi: 1\nfamilies: [LIFE]\n---\n\n# Life\n\n## Criteria\n\n- **LIFE-2**  Resume, secret wording two.\n",
    );
    fixture.write("specs/cli/cli.spec.md", "---\nmodule: cli\n---\n\n# cli\n");
    fixture.write(
        "specs/cli/requirements.md",
        "# Requirements\n\n### REQ-cli-001\n\nThe helper SHALL keep one checkpoint. Trace: KNOW-1, LIFE-2.\n",
    );
    fixture.write(
        "specs/cli/context.md",
        "# Context\n\n## Decisions\n\n- One.\n\n## Lessons\n\n2026-01-01, first lesson (verified): text one.\n\n2026-01-02, second lesson (observation): text two.\n",
    );
    fixture.write(
        "specs/history/history.spec.md",
        "---\nmodule: history\n---\n\n# history\n",
    );
    fixture.write(
        "specs/history/requirements.md",
        "# Requirements\n\n### REQ-history-001\n\nRetention SHALL keep references. Trace: KNOW-7.\n",
    );
    fixture.write(
        "specs/history/context.md",
        "# Context\n\n## Lessons\n\nNone yet.\n",
    );
    fixture.write("AGENTS.md", "# Agents\n");
    fixture.write("docs/DECISIONS.md", "# Decisions\n");
    archive(&fixture, 0, "a1", &["cli", "history"], &["REQ-cli-001"]);
    archive(&fixture, 1, "a2", &["history"], &["REQ-history-001"]);
    fixture.init();
    fixture.ok(&["library", "index"]);
    fixture
}

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

/// The YAML front matter block of a note, without its fences.
fn front_matter(text: &str) -> &str {
    let rest = text
        .strip_prefix("---\n")
        .unwrap_or_else(|| panic!("no front matter:\n{text}"));
    &rest[..rest.find("\n---\n").expect("unterminated front matter")]
}

#[test]
fn every_generated_vault_note_starts_with_front_matter() {
    let fixture = vault();
    let knowledge = fixture.root().join("knowledge");
    let all = notes(&knowledge);
    assert!(all.len() >= 7, "{all:?}");
    for note in all {
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
}

#[test]
fn an_archived_change_note_carries_its_properties_and_links() {
    let fixture = vault();
    let text = fixture.read("knowledge/archives/a1.md");
    let properties = front_matter(&text);
    for expected in [
        "type: archived-change",
        "status: hot",
        "finalized: 1970-01-01T00:01:40Z",
        "title: \"Title of a1: with a colon\"",
        "modules:\n  - cli\n  - history",
        "requirements:\n  - REQ-cli-001",
        "criteria:\n  - KNOW-1\n  - LIFE-2",
        &format!("implementation_commit: \"{}\"", "1".repeat(40)),
    ] {
        assert!(
            properties.contains(expected),
            "missing {expected:?} in\n{properties}"
        );
    }
    let body = &text[properties.len()..];
    for link in [
        "[cli](../modules/cli.md)",
        "[history](../modules/history.md)",
        "[KNOW-1](../intent/knowledge.md)",
        "[LIFE-2](../intent/life.md)",
    ] {
        assert!(body.contains(link), "missing {link} in\n{body}");
    }
    assert!(body.contains("Finalized: 1970-01-01T00:01:40Z"), "{body}");
}

#[test]
fn module_notes_link_their_sources_lessons_and_archived_changes() {
    let fixture = vault();
    let cli = fixture.read("knowledge/modules/cli.md");
    assert!(front_matter(&cli).contains("type: module"), "{cli}");
    for expected in [
        "[specs/cli/cli.spec.md](../../specs/cli/cli.spec.md)",
        "[specs/cli/requirements.md](../../specs/cli/requirements.md)",
        "[specs/cli/context.md](../../specs/cli/context.md)",
        "2 lessons",
        "[a1](../archives/a1.md)",
    ] {
        assert!(cli.contains(expected), "missing {expected:?} in\n{cli}");
    }
    assert!(!cli.contains("[a2](../archives/a2.md)"), "{cli}");
    assert!(!cli.contains("text one"), "lesson text copied:\n{cli}");
    let history = fixture.read("knowledge/modules/history.md");
    for expected in [
        "[a1](../archives/a1.md)",
        "[a2](../archives/a2.md)",
        "0 lessons",
    ] {
        assert!(
            history.contains(expected),
            "missing {expected:?} in\n{history}"
        );
    }
}

#[test]
fn intent_notes_link_each_criterion_to_the_changes_that_trace_it() {
    let fixture = vault();
    let knowledge = fixture.read("knowledge/intent/knowledge.md");
    let properties = front_matter(&knowledge);
    assert!(properties.contains("type: intent"), "{properties}");
    assert!(properties.contains("families:\n  - KNOW"), "{properties}");
    assert!(
        knowledge.contains("[hi/knowledge.md](../../hi/knowledge.md)"),
        "{knowledge}"
    );
    let line = |id: &str| {
        knowledge
            .lines()
            .find(|line| line.contains(&format!("**{id}**")))
            .unwrap_or_else(|| panic!("no {id} line in\n{knowledge}"))
            .to_string()
    };
    assert!(
        line("KNOW-1").contains("[a1](../archives/a1.md)"),
        "{knowledge}"
    );
    assert!(
        line("KNOW-7").contains("[a2](../archives/a2.md)"),
        "{knowledge}"
    );
    assert!(
        !knowledge.contains("secret wording"),
        "criterion text copied:\n{knowledge}"
    );
    let life = fixture.read("knowledge/intent/life.md");
    assert!(
        life.contains("**LIFE-2**") && life.contains("[a1](../archives/a1.md)"),
        "{life}"
    );
}

/// corvid-agent review of #107: HI sub-criterion IDs (DRIVE-1) are listed and traced,
/// and a `Trace:` list is read whole, not cut at the first dot.
#[test]
fn sub_criterion_ids_are_listed_and_traced_in_whole_trace_lists() {
    let fixture = vault();
    fixture.write(
        "hi/ux.md",
        "---\nhi: 1\nfamilies: [UX]\n---\n\n# UX\n\n## Criteria\n\n- **UX-1**  One.\n- **UX-1.a**  One a.\n- **UX-2**  Two.\n",
    );
    fixture.write(
        "specs/cli/requirements.md",
        "# Requirements\n\n### REQ-cli-001\n\nThe helper SHALL keep one checkpoint. Trace: KNOW-1, LIFE-2.\n\n### REQ-cli-002\n\nThe view SHALL show it. Trace: UX-1.a, UX-2.\n",
    );
    archive(&fixture, 2, "a3", &["cli"], &["REQ-cli-002"]);
    fixture.ok(&["library", "index"]);
    let a3 = fixture.read("knowledge/archives/a3.md");
    assert!(
        front_matter(&a3).contains("criteria:\n  - UX-1.a\n  - UX-2"),
        "{a3}"
    );
    assert!(a3.contains("[UX-1.a](../intent/ux.md)"), "{a3}");
    let ux = fixture.read("knowledge/intent/ux.md");
    assert!(
        front_matter(&ux).contains("criteria:\n  - UX-1\n  - UX-1.a\n  - UX-2"),
        "{ux}"
    );
    let line = |id: &str| {
        ux.lines()
            .find(|line| line.starts_with(&format!("- **{id}**")))
            .unwrap_or_else(|| panic!("no {id} line in\n{ux}"))
            .to_string()
    };
    assert!(line("UX-1.a").contains("[a3](../archives/a3.md)"), "{ux}");
    assert!(line("UX-2").contains("[a3](../archives/a3.md)"), "{ux}");
    assert!(
        !line("UX-1").contains("a3"),
        "UX-1 credited for UX-1.a:\n{ux}"
    );
}

/// Reviews of #107: a requirement that mentions `Trace:` inline is read from its real,
/// last `Trace:` list, and HI files that declare their family quoted or as a YAML block
/// list still link to their intent note.
#[test]
fn trace_lists_and_family_declarations_are_read_in_every_valid_form() {
    let fixture = vault();
    fixture.write(
        "hi/ux.md",
        "---\nhi: 1\nfamilies: [\"UX\"]\n---\n\n# UX\n\n## Criteria\n\n- **UX-1**  One.\n",
    );
    fixture.write(
        "hi/send.md",
        "---\nhi: 1\nfamilies:\n  - SEND\n---\n\n# Send\n\n## Criteria\n\n- **SEND-2**  Two.\n",
    );
    fixture.write(
        "specs/cli/requirements.md",
        "# Requirements\n\n### REQ-cli-001\n\nThe helper SHALL keep one checkpoint. Trace: KNOW-1, LIFE-2.\n\n### REQ-cli-002\n\nIt SHALL list the IDs that the `Trace:` lines name. Trace: UX-1, SEND-2.\n",
    );
    archive(&fixture, 2, "a3", &["cli"], &["REQ-cli-002"]);
    fixture.ok(&["library", "index"]);
    let a3 = fixture.read("knowledge/archives/a3.md");
    assert!(
        front_matter(&a3).contains("criteria:\n  - SEND-2\n  - UX-1"),
        "{a3}"
    );
    assert!(a3.contains("[UX-1](../intent/ux.md)"), "{a3}");
    assert!(a3.contains("[SEND-2](../intent/send.md)"), "{a3}");
    for (note, family) in [("ux", "UX"), ("send", "SEND")] {
        let text = fixture.read(&format!("knowledge/intent/{note}.md"));
        assert!(
            front_matter(&text).contains(&format!("families:\n  - {family}")),
            "{text}"
        );
        assert!(text.contains("[a3](../archives/a3.md)"), "{text}");
    }
}

/// corvid-agent review at 5326ec4: `Trace:` inside backticks is a mention, before or after
/// the real list, and CRLF front matter and YAML comments still give the family.
#[test]
fn inline_trace_mentions_crlf_and_comments_do_not_hide_criteria_or_families() {
    let fixture = vault();
    fixture.write(
        "hi/ux.md",
        "---\r\nhi: 1\r\nfamilies: [UX]\r\n---\r\n\r\n# UX\r\n\r\n## Criteria\r\n\r\n- **UX-1**  One.\r\n",
    );
    fixture.write(
        "hi/send.md",
        "---\nhi: 1\nfamilies: [SEND] # send flows\n---\n\n# Send\n\n## Criteria\n\n- **SEND-2**  Two.\n",
    );
    fixture.write(
        "specs/cli/requirements.md",
        "# Requirements\n\n### REQ-cli-001\n\nThe helper SHALL keep one checkpoint. Trace: KNOW-1, LIFE-2.\n\n### REQ-cli-002\n\nIt SHALL do it. Trace: UX-1, SEND-2. Readers grep the `Trace:` lines.\n",
    );
    archive(&fixture, 2, "a3", &["cli"], &["REQ-cli-002"]);
    fixture.ok(&["library", "index"]);
    let a3 = fixture.read("knowledge/archives/a3.md");
    assert!(
        front_matter(&a3).contains("criteria:\n  - SEND-2\n  - UX-1"),
        "{a3}"
    );
    assert!(a3.contains("[UX-1](../intent/ux.md)"), "{a3}");
    assert!(a3.contains("[SEND-2](../intent/send.md)"), "{a3}");
    for (note, family) in [("ux", "UX"), ("send", "SEND")] {
        let text = fixture.read(&format!("knowledge/intent/{note}.md"));
        assert!(
            front_matter(&text).contains(&format!("families:\n  - {family}\n")),
            "{text}"
        );
    }
}

/// Review 5779587370: a README byte-identical to the template an earlier Chough generated
/// is upgraded to the current one; an edited README is left alone.
#[test]
fn an_untouched_earlier_readme_is_upgraded_and_an_edited_one_kept() {
    let fixture = vault();
    let earlier = include_str!("fixtures/knowledge-readme-at-8fbbbe7.md");
    fixture.write("knowledge/README.md", earlier);
    fixture.ok(&["library", "index"]);
    let readme = fixture.read("knowledge/README.md");
    assert!(front_matter(&readme).contains("type: readme"), "{readme}");
    let edited = format!("{earlier}\nOur own note.\n");
    fixture.write("knowledge/README.md", &edited);
    fixture.ok(&["library", "index"]);
    fixture.ok(&["library", "init"]);
    assert_eq!(fixture.read("knowledge/README.md"), edited);
}

/// Windows CI on #107: an untouched earlier README checked out with CRLF line endings is
/// still recognized; and an unmatched backtick does not hide the Trace list after it.
#[test]
fn crlf_earlier_readmes_upgrade_and_unmatched_backticks_keep_the_trace_list() {
    let fixture = vault();
    let earlier = include_str!("fixtures/knowledge-readme-at-8fbbbe7.md").replace("\r\n", "\n");
    fixture.write("knowledge/README.md", &earlier.replace('\n', "\r\n"));
    fixture.write(
        "specs/cli/requirements.md",
        "# Requirements\n\n### REQ-cli-001\n\nThe helper SHALL keep one checkpoint. Trace: KNOW-1, LIFE-2.\n\n### REQ-cli-002\n\nUse the ` key to jump. Trace: KNOW-7.\n",
    );
    archive(&fixture, 2, "a3", &["cli"], &["REQ-cli-002"]);
    fixture.ok(&["library", "index"]);
    let readme = fixture.read("knowledge/README.md");
    assert!(front_matter(&readme).contains("type: readme"), "{readme}");
    let a3 = fixture.read("knowledge/archives/a3.md");
    assert!(front_matter(&a3).contains("criteria:\n  - KNOW-7"), "{a3}");
}

#[test]
fn the_rules_note_links_rules_skills_and_decisions_that_exist() {
    let fixture = vault();
    let rules = fixture.read("knowledge/rules.md");
    assert!(front_matter(&rules).contains("type: rules"), "{rules}");
    for expected in [
        "[AGENTS.md](../AGENTS.md)",
        "[docs/DECISIONS.md](../docs/DECISIONS.md)",
        "[specs/cli/context.md](../specs/cli/context.md)",
        "[.agents/skills/chough-workflow/SKILL.md](../.agents/skills/chough-workflow/SKILL.md)",
    ] {
        assert!(rules.contains(expected), "missing {expected:?} in\n{rules}");
    }
    assert!(
        !rules.contains("CLAUDE.md"),
        "a missing file was linked:\n{rules}"
    );
    let index = fixture.read("knowledge/INDEX.md");
    for expected in [
        "[rules.md](rules.md)",
        "[cli](modules/cli.md)",
        "[knowledge](intent/knowledge.md)",
    ] {
        assert!(index.contains(expected), "missing {expected:?} in\n{index}");
    }
}

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

#[test]
fn the_vault_regenerates_identically() {
    let fixture = vault();
    let knowledge = fixture.root().join("knowledge");
    let read = || {
        notes(&knowledge)
            .into_iter()
            .map(|path| (path.clone(), fs::read_to_string(path).unwrap()))
            .collect::<Vec<_>>()
    };
    let first = read();
    fixture.ok(&["library", "index"]);
    assert_eq!(read(), first);
}

/// DRIVE-12: generated knowledge can merge with Git's union driver, because a rebuild
/// check names any stale or missing generated file and writes nothing.
#[test]
fn index_check_names_stale_files_and_passes_after_a_rebuild() {
    let fixture = vault();
    fixture.ok(&["library", "index", "--check"]);

    let index = fixture.read("knowledge/INDEX.md");
    let merged = format!("{index}\n- a line a union merge kept\n");
    fixture.write("knowledge/INDEX.md", &merged);
    let error = fixture.fails(&["library", "index", "--check"]);
    assert!(
        error.contains("knowledge/INDEX.md") && error.contains("chough library index"),
        "{error}"
    );
    assert_eq!(
        fixture.read("knowledge/INDEX.md"),
        merged,
        "the check writes nothing"
    );
    fixture.ok(&["library", "index"]);
    fixture.ok(&["library", "index", "--check"]);

    // A CRLF checkout of a current note is current.
    let note = "knowledge/archives/a1.md";
    let text = fixture.read(note);
    fixture.write(note, &text.replace('\n', "\r\n"));
    fixture.ok(&["library", "index", "--check"]);

    // A missing generated note is named.
    fs::remove_file(fixture.root().join(note)).unwrap();
    let error = fixture.fails(&["library", "index", "--check"]);
    assert!(error.contains(note), "{error}");
}

/// #112: the index listed every archived change and outgrew its 65,536-byte bound at 104
/// archives here. It lists the newest, and a monthly timeline lists every one (KNOW-28).
#[test]
fn the_index_stays_bounded_and_a_monthly_timeline_lists_every_archive() {
    let fixture = vault();
    let october = 1_792_022_400; // 2026-10-15
    let november = 1_793_836_800; // 2026-11-05
    let mut ids = vec![];
    for index in 0..300 {
        let id = format!("{index:03}-{}", "a-long-archived-change-id-".repeat(4));
        let at = if index < 150 { october } else { november } + index as u64 * 60;
        let dir = format!(".specsync/archive/changes/2026-10-15-{id}");
        fixture.write(
            &format!("{dir}/finalization.json"),
            &format!(
                "{{\"schema_version\": 2, \"change_id\": \"{id}\", \"timestamp\": {at}, \"implementation_commit\": \"{}\"}}\n",
                "1".repeat(40)
            ),
        );
        fixture.write(
            &format!("{dir}/state.json"),
            &serde_json::to_string_pretty(
                &json!({"id": id, "state": "archived", "affected_specs": ["cli"]}),
            )
            .unwrap(),
        );
        fixture.write(
            &format!("{dir}/verification.json"),
            &serde_json::to_string_pretty(&json!({
                "commit": "2".repeat(40), "passed": true, "requirement_ids": ["REQ-cli-001"],
                "acceptance_manifest": {"schema_version": 1, "entries": []},
            }))
            .unwrap(),
        );
        fixture.write(
            &format!("{dir}/change.md"),
            &format!("---\nid: {id}\nstate: archived\n---\n\n# Title of {id}\n"),
        );
        fixture.write(&format!("{dir}/lesson-bundle.md"), "# Lesson bundle\n");
        ids.push(id);
    }
    fixture.ok(&["library", "index"]);
    let index = fixture.read("knowledge/INDEX.md");
    assert!(index.len() <= 65_536, "INDEX.md is {} bytes", index.len());
    assert_eq!(
        ids.iter()
            .filter(|id| index.contains(&format!("archives/{id}.md")))
            .count(),
        10,
        "INDEX.md lists the 10 newest"
    );
    assert!(index.contains("](timeline.md)"), "{index}");
    let timeline = fixture.read("knowledge/timeline.md");
    assert!(timeline.starts_with("---\n"), "{timeline}");
    for month in ["2026-11", "2026-10"] {
        assert!(
            timeline.contains(&format!("](timeline/{month}.md)")),
            "{timeline}"
        );
    }
    // A full month continues on further pages; every page is within the bound.
    let mut names: Vec<String> = fs::read_dir(fixture.root().join("knowledge/timeline"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert!(names.len() > 2, "{names:?}");
    let pages: Vec<String> = names
        .iter()
        .map(|name| fixture.read(&format!("knowledge/timeline/{name}")))
        .collect();
    for (name, page) in names.iter().zip(&pages) {
        assert!(page.starts_with("---\n") && page.len() <= 65_536, "{name}");
        let stem = name.trim_end_matches(".md");
        assert!(
            timeline.contains(&format!("](timeline/{stem}.md)")),
            "{stem}: {timeline}"
        );
    }
    for id in &ids {
        let link = format!("../archives/{id}.md");
        assert_eq!(
            pages.iter().filter(|page| page.contains(&link)).count(),
            1,
            "{id}"
        );
    }
    assert!(pages.iter().all(|page| page.contains("lesson-bundle.md")));
    fixture.ok(&["library", "index", "--check"]);
}

/// #112 review: only the notes `library index` generates, which `--check` compares,
/// merge by union. An attachment, a snapshot, a brief or the README still conflicts,
/// rather than merging into invalid content the check never sees.
#[test]
fn only_generated_knowledge_notes_merge_by_union() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let merge_attribute = |path: &str| {
        let output = std::process::Command::new("git")
            .args(["check-attr", "merge", "--", path])
            .current_dir(root)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .rsplit(": ")
            .next()
            .unwrap()
            .to_string()
    };
    for path in [
        "knowledge/INDEX.md",
        "knowledge/rules.md",
        "knowledge/timeline.md",
        "knowledge/timeline/2026-11.md",
        "knowledge/timeline/2026-11-p2.md",
        "knowledge/archives/some-change.md",
        "knowledge/modules/cli.md",
        "knowledge/intent/drive.md",
        // #128: the bug notes KNOW-22 generates are rebuilt like the rest.
        "knowledge/bugs/BUG-0001.md",
        // KNOW-29: the session notes are generated like the rest.
        "knowledge/sessions/s0.md",
        // KNOW-23: a ledger shard is append-only like the ledger it came from.
        "knowledge/attachments/some-change/pruned.jsonl",
    ] {
        assert_eq!(merge_attribute(path), "union", "{path}");
    }
    for path in [
        "knowledge/attachments/some-change/evidence/OBS-x.json",
        "knowledge/attachments/some-change/artifacts/x/screen.png",
        "knowledge/snapshots/SNAP-x/notes.md",
        "knowledge/briefs/older.md",
        "knowledge/README.md",
    ] {
        assert_ne!(merge_attribute(path), "union", "{path}");
    }
}
