//! Documentation that matches main (FLOW-18, FR-docs-1 to FR-docs-6): the README, the
//! docs, the workflow skill and the diagrams say what the helper, `fledge.toml`,
//! `.gitattributes` and CI actually do, and these checks fail when they drift.

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// A repository file as text, with CRLF read as LF so a Windows checkout reads the same.
fn read(relative: &str) -> String {
    fs::read_to_string(manifest_dir().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
        .replace("\r\n", "\n")
}

/// Runs the helper and returns its stdout; a failure fails the test.
fn chough(root: Option<&Path>, args: &[&str]) -> String {
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    if let Some(root) = root {
        command.arg("--root").arg(root);
    }
    let output = command.args(args).output().unwrap();
    assert!(
        output.status.success(),
        "chough {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn git(root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// A Git project with one commit and an initialized checkpoint.
fn initialized_project() -> tempfile::TempDir {
    let project = tempfile::tempdir().unwrap();
    let root = project.path();
    git(root, &["init", "-q"]);
    git(root, &["config", "user.name", "Test"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "commit.gpgsign", "false"]);
    git(root, &["commit", "-q", "--allow-empty", "-m", "start"]);
    chough(Some(root), &["init", "--objective", "Check the docs"]);
    project
}

/// Markdown files under `relative`, sorted, as repository-relative `/` paths.
fn markdown_under(relative: &str) -> Vec<String> {
    let mut found = vec![];
    let mut pending = vec![manifest_dir().join(relative)];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "md") {
                let relative = path.strip_prefix(manifest_dir()).unwrap();
                found.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    found.sort();
    found
}

/// Every page FLOW-18 covers: the README, agent instructions, docs, examples index and
/// the workflow skill with its shared host copy.
fn documentation() -> Vec<String> {
    let mut pages: Vec<String> = ["README.md", "AGENTS.md", "CLAUDE.md", "examples/README.md"]
        .map(String::from)
        .to_vec();
    pages.extend(markdown_under("docs"));
    pages.extend(markdown_under("skills/chough-workflow"));
    pages.extend(markdown_under(".agents/skills/chough-workflow"));
    pages
}

/// The text with every fenced code block's lines blanked, so line numbers still match.
fn without_fences(text: &str) -> String {
    let mut fenced = false;
    text.lines()
        .map(|line| {
            if line.trim_start().starts_with("```") {
                fenced = !fenced;
                ""
            } else if fenced {
                ""
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The fenced blocks of one info string (`mermaid`, `gitattributes`), with the line of
/// each opening fence.
fn fenced_blocks(text: &str, info: &str) -> Vec<(usize, String)> {
    let mut blocks = vec![];
    let mut open: Option<(usize, Vec<&str>)> = None;
    for (index, line) in text.lines().enumerate() {
        let trimmed = line.trim();
        match &mut open {
            Some((start, body)) => {
                if trimmed.starts_with("```") {
                    blocks.push((*start, body.join("\n")));
                    open = None;
                } else {
                    body.push(line);
                }
            }
            None if trimmed == format!("```{info}") => open = Some((index + 1, vec![])),
            None => {}
        }
    }
    blocks
}

// MARK: - The helper's own lists

/// One command's long options and subcommands, read from its `--help`.
struct Help {
    options: BTreeSet<String>,
    subcommands: Vec<String>,
}

fn help(path: &[String]) -> Help {
    let mut args: Vec<&str> = path.iter().map(String::as_str).collect();
    args.push("--help");
    let text = chough(None, &args);
    let mut options = BTreeSet::new();
    let mut subcommands = vec![];
    let mut in_commands = false;
    for line in text.lines() {
        if line == "Commands:" {
            in_commands = true;
            continue;
        }
        if in_commands {
            if line.trim().is_empty() {
                in_commands = false;
            } else if line.starts_with("  ") && !line.starts_with("    ") {
                let name = line.split_whitespace().next().unwrap().to_string();
                if name != "help" {
                    subcommands.push(name);
                }
            }
            continue;
        }
        let trimmed = line.trim_start();
        let trimmed = match trimmed.split_once(", ") {
            Some((short, rest)) if short.len() == 2 && short.starts_with('-') => rest,
            _ => trimmed,
        };
        if let Some(rest) = trimmed.strip_prefix("--") {
            let name: String = rest
                .chars()
                .take_while(|character| character.is_ascii_alphanumeric() || *character == '-')
                .collect();
            if !name.is_empty() {
                options.insert(format!("--{name}"));
            }
        }
    }
    Help {
        options,
        subcommands,
    }
}

/// Every command path the helper lists, from `chough --help` down.
fn help_tree() -> BTreeMap<Vec<String>, Help> {
    let mut tree = BTreeMap::new();
    let mut pending = vec![vec![]];
    while let Some(path) = pending.pop() {
        let node = help(&path);
        for subcommand in &node.subcommands {
            let mut child = path.clone();
            child.push(subcommand.clone());
            pending.push(child);
        }
        tree.insert(path, node);
    }
    tree
}

/// The helper's stages in its own order, with their kinds, from `chough stage show`.
fn helper_stages() -> Vec<(String, String)> {
    let project = initialized_project();
    let shown: Value =
        serde_json::from_str(&chough(Some(project.path()), &["stage", "show"])).unwrap();
    shown["stages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|stage| {
            (
                stage["name"].as_str().unwrap().to_string(),
                stage["kind"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

// MARK: - Mermaid

/// A flowchart: node labels by ID, and edges as (from, to, label).
struct Flowchart {
    labels: BTreeMap<String, String>,
    edges: Vec<(String, String, String)>,
}

/// The source without a leading `---` front matter block.
fn without_front_matter(source: &str) -> &str {
    let Some(rest) = source.strip_prefix("---\n") else {
        return source;
    };
    rest.split_once("\n---\n").map_or(source, |(_, body)| body)
}

/// Reads one node reference (`id`, `id["label"]`, `id(["label"])`, `id[("label")]`,
/// `id{"label"}` or `id("label")`) and returns its ID, label and the rest of the line.
fn node(text: &str) -> (String, Option<String>, &str) {
    let text = text.trim_start();
    let end = text
        .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
        .unwrap_or(text.len());
    let (id, rest) = text.split_at(end);
    assert!(!id.is_empty(), "no node at {text:?}");
    for (open, close) in [
        ("([\"", "\"])"),
        ("[(\"", "\")]"),
        ("[\"", "\"]"),
        ("{\"", "\"}"),
        ("(\"", "\")"),
    ] {
        if let Some(inner) = rest.strip_prefix(open) {
            let (label, rest) = inner
                .split_once(close)
                .unwrap_or_else(|| panic!("unclosed label at {text:?}"));
            return (id.to_string(), Some(label.to_string()), rest);
        }
    }
    (id.to_string(), None, rest)
}

/// Reads one edge (`-->`, `-->|"label"|`, `-.->` or `-. "label" .->`) and returns its
/// label and the rest of the line.
fn edge(text: &str) -> (String, &str) {
    let text = text.trim_start();
    if let Some(rest) = text.strip_prefix("-->") {
        if let Some(inner) = rest.strip_prefix("|\"") {
            let (label, rest) = inner.split_once("\"|").unwrap();
            return (label.to_string(), rest);
        }
        return (String::new(), rest);
    }
    if let Some(rest) = text.strip_prefix("-.->") {
        return (String::new(), rest);
    }
    if let Some(inner) = text.strip_prefix("-. \"") {
        let (label, rest) = inner.split_once("\" .->").unwrap();
        return (label.to_string(), rest);
    }
    panic!("unknown edge at {text:?}");
}

fn flowchart(source: &str) -> Flowchart {
    let mut labels = BTreeMap::new();
    let mut edges = vec![];
    for line in without_front_matter(source).lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("flowchart") || line.starts_with("%%") {
            continue;
        }
        let (mut from, label, mut rest) = node(line);
        if let Some(label) = label {
            labels.insert(from.clone(), label);
        }
        while !rest.trim().is_empty() {
            let (edge_label, after_edge) = edge(rest);
            let (to, label, after_node) = node(after_edge);
            if let Some(label) = label {
                labels.insert(to.clone(), label);
            }
            edges.push((from, to.clone(), edge_label));
            from = to;
            rest = after_node;
        }
    }
    Flowchart { labels, edges }
}

/// The visible text of an SVG: tags removed, entities decoded, whitespace collapsed.
fn svg_text(svg: &str) -> String {
    let mut text = String::new();
    let mut in_tag = false;
    for character in svg.chars() {
        match character {
            '<' => {
                in_tag = true;
                text.push(' ');
            }
            '>' => in_tag = false,
            _ if !in_tag => text.push(character),
            _ => {}
        }
    }
    let text = text
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&#x27;", "'")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&");
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn diagram_sources() -> Vec<String> {
    let mut sources: Vec<String> = fs::read_dir(manifest_dir().join("docs/diagrams"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".mmd"))
        .collect();
    sources.sort();
    sources
}

/// FR-docs-1: the stage diagram draws every stage the helper has, and only the moves
/// it makes. `chough stage skip` leaves the current stage for the next main one, `back`
/// goes to an earlier main stage, every other move between stages is an advance, and a
/// move into a stage with a native prerequisite carries its gate, naming each refusal.
#[test]
fn stage_diagrams_draw_only_the_helpers_stages_and_moves() {
    let stages = helper_stages();
    let order: Vec<&str> = stages
        .iter()
        .filter(|(_, kind)| matches!(kind.as_str(), "main" | "conditional" | "terminal"))
        .map(|(name, _)| name.as_str())
        .collect();
    let position = |name: &str| order.iter().position(|stage| *stage == name);
    let next_main = |name: &str| {
        if stages
            .iter()
            .any(|(stage, kind)| stage == name && kind == "optional")
        {
            return Some("done");
        }
        position(name).and_then(|index| order.get(index + 1).copied())
    };
    let is_stage = |name: &str| stages.iter().any(|(stage, _)| stage == name);
    // Destinations with a native prerequisite (src/stage.rs, `valid_transitions`).
    let gated = ["build", "test", "accept", "done"];
    let stage_source = read("src/stage.rs");

    let file = "docs/diagrams/stages.mmd";
    let chart = flowchart(&read(file));
    let mut problems = vec![];
    for (name, kind) in &stages {
        if kind == "side" {
            if !chart
                .labels
                .values()
                .any(|label| label.contains(name.as_str()))
            {
                problems.push(format!("{file}: no node names the side stage {name}"));
            }
        } else if !chart.labels.contains_key(name) {
            problems.push(format!("{file}: no node for the {kind} stage {name}"));
        }
    }
    for (from, to, label) in &chart.edges {
        if !is_stage(from) || !is_stage(to) {
            continue;
        }
        let lower = label.to_lowercase();
        if lower.contains("skip") {
            if next_main(from) != Some(to.as_str()) {
                problems.push(format!(
                    "{file}: a skip edge {from} -> {to} (\"{label}\"), but `chough stage skip` at {from} moves to {}",
                    next_main(from).unwrap_or("nothing")
                ));
            }
        } else if lower.contains("back") {
            match (position(from), position(to)) {
                (Some(from_index), Some(to_index)) if to_index < from_index => {}
                _ => problems.push(format!(
                    "{file}: a back edge {from} -> {to}, but back goes only to an earlier main stage"
                )),
            }
        } else if from == "done" && to == "release" {
            if !lower.contains("request") {
                problems.push(format!(
                    "{file}: done -> release must say release is entered only on request or a standing setting"
                ));
            }
        } else if next_main(from) != Some(to.as_str()) {
            problems.push(format!(
                "{file}: an edge {from} -> {to}, but an advance from {from} goes to {}",
                next_main(from).unwrap_or("nothing")
            ));
        }
        let advance = !lower.contains("skip") && !lower.contains("back") && to != "release";
        if advance && gated.contains(&to.as_str()) && !lower.contains("gate") {
            problems.push(format!(
                "{file}: {from} -> {to} has a native prerequisite but its label names no gate"
            ));
        }
    }
    // Leaving test refuses an open blocking bug and an unlinked failed case
    // (`chough stage advance` at test); the label names both while the helper has both.
    let test_to_accept = chart
        .edges
        .iter()
        .find(|(from, to, _)| from == "test" && to == "accept")
        .map(|(_, _, label)| label.to_lowercase())
        .unwrap_or_default();
    for (refusal, words) in [
        ("refused:blocking-bugs", "blocking bug"),
        ("refused:unlinked-failure", "unlinked failure"),
    ] {
        assert!(
            stage_source.contains(refusal),
            "src/stage.rs no longer has {refusal}; update this check and the diagram"
        );
        if !test_to_accept.contains(words) {
            problems.push(format!(
                "{file}: the test -> accept gate does not name \"{words}\" ({refusal})"
            ));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// FR-docs-2: every Mermaid block in a page is one of the `docs/diagrams/*.mmd`
/// sources (front matter aside), and each source's SVG carries every label it has, so
/// a source edited without a re-render, or a page edited without its source, fails.
#[test]
fn inline_diagrams_equal_their_sources_and_svgs_carry_every_label() {
    let mut problems = vec![];
    let mut sources = BTreeMap::new();
    for name in diagram_sources() {
        let source = read(&format!("docs/diagrams/{name}"));
        let body = without_front_matter(&source).trim_end().to_string();
        let chart = flowchart(&source);
        let svg_name = format!("docs/diagrams/{}.svg", name.trim_end_matches(".mmd"));
        let text = svg_text(&read(&svg_name));
        let labels = chart
            .labels
            .values()
            .chain(chart.edges.iter().map(|(_, _, label)| label))
            .filter(|label| !label.is_empty());
        for label in labels {
            if !text.contains(label.as_str()) {
                problems.push(format!(
                    "{svg_name} lacks \"{label}\" from {name}; re-render with docs/diagrams/render.mjs"
                ));
            }
        }
        sources.insert(body, name);
    }
    for page in documentation() {
        for (line, block) in fenced_blocks(&read(&page), "mermaid") {
            if !sources.contains_key(block.trim_end()) {
                problems.push(format!(
                    "{page}:{line}: this Mermaid block is not any docs/diagrams/*.mmd source; make them the same"
                ));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The README's "What the helper does" table rows: (command cell, whole row).
fn readme_helper_rows(readme: &str) -> Vec<(String, String)> {
    let section = readme
        .split("## What the helper does")
        .nth(1)
        .expect("README has a What the helper does section");
    let section = section.split("\n## ").next().unwrap();
    section
        .lines()
        .filter(|line| line.starts_with("| `"))
        .map(|line| {
            let cell = line.split('|').nth(1).unwrap().to_string();
            (cell, line.to_string())
        })
        .collect()
}

/// FR-docs-3: the README helper table names every command `chough --help` lists. A
/// cell that lists a command's subcommands (`bug add/update/...`) lists all of them; a
/// bare command or `stage …` stands for every subcommand.
#[test]
fn readme_helper_table_names_every_command_the_helper_lists() {
    let readme = read("README.md");
    let rows = readme_helper_rows(&readme);
    let tree = help_tree();
    let mut problems = vec![];
    for command in &tree[&vec![]].subcommands {
        let mentions: Vec<&str> = rows
            .iter()
            .flat_map(|(cell, _)| cell.split('`').skip(1).step_by(2))
            .filter(|code| code.split_whitespace().next() == Some(command.as_str()))
            .collect();
        if mentions.is_empty() {
            problems.push(format!(
                "README.md What the helper does: no row names `{command}`"
            ));
            continue;
        }
        let listed: BTreeSet<&str> = mentions
            .iter()
            .filter_map(|code| code.split_whitespace().nth(1))
            .flat_map(|list| list.split('/'))
            .collect();
        let covers_all = mentions.iter().any(|code| {
            code.split_whitespace()
                .nth(1)
                .is_none_or(|rest| rest == "…")
        });
        if covers_all {
            continue;
        }
        for subcommand in &tree[&vec![command.clone()]].subcommands {
            if !listed.contains(subcommand.as_str()) {
                problems.push(format!(
                    "README.md What the helper does: `{command}` lists subcommands but not {subcommand}"
                ));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// FR-docs-3: README's Tested platforms names every step of the Fledge verify lane.
#[test]
fn readme_verify_lane_list_names_every_fledge_verify_step() {
    let fledge = read("fledge.toml");
    let lane = fledge
        .split("[lanes.verify]")
        .nth(1)
        .expect("fledge.toml has a verify lane");
    let steps_line = lane
        .lines()
        .find(|line| line.trim_start().starts_with("steps"))
        .expect("the verify lane lists its steps");
    let steps: Vec<&str> = steps_line.split('"').skip(1).step_by(2).collect();
    assert!(!steps.is_empty(), "no steps in {steps_line}");
    let readme = read("README.md").replace('\n', " ");
    let list = readme
        .split("the Fledge `verify` lane (")
        .nth(1)
        .and_then(|rest| rest.split(')').next())
        .expect("README names the Fledge verify lane's steps in parentheses");
    // The README's words for each task in fledge.toml.
    let words = [
        ("toolchain", "tool versions"),
        ("fmt", "format"),
        ("lint", "lint"),
        ("test", "tests"),
        ("intent", "HI"),
        ("spec", "SpecSync checks"),
        ("audit", "change audit"),
        ("cases", "case definitions"),
        ("knowledge", "knowledge index"),
        ("leaks", "leak scan"),
        ("exercise", "feature and bug exercise"),
        ("build", "build"),
    ];
    let mut problems = vec![];
    for step in steps {
        match words.iter().find(|(task, _)| *task == step) {
            None => problems.push(format!(
                "fledge.toml verify step `{step}` has no README wording in this check; add it here and to README.md"
            )),
            Some((_, word)) if !list.contains(word) => problems.push(format!(
                "README.md Tested platforms: the verify lane list ({list}) does not name `{step}` (\"{word}\")"
            )),
            Some(_) => {}
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// FR-docs-4: every `chough` command a page names exists, and every flag it gives that
/// command is in that command's `--help`. A command line ends at a backtick, `|`, `;`,
/// `)` or the end of the line, or where another command or a prose join starts.
#[test]
fn documented_chough_commands_and_flags_exist() {
    let tree = help_tree();
    let global = ["--help", "--root", "--version"];
    let mut problems = vec![];
    let mut checked = 0;
    for page in documentation() {
        let text = read(&page).replace("\\\n", " ");
        for (index, line) in text.lines().enumerate() {
            let mut rest = line;
            while let Some(at) = rest.find("chough ") {
                let before = rest[..at].chars().last();
                let after = &rest[at + "chough ".len()..];
                rest = after;
                if before.is_some_and(|character| {
                    character.is_ascii_alphanumeric() || "-_/.".contains(character)
                }) {
                    continue;
                }
                let segment = after.split(['`', '|', ';', ')']).next().unwrap_or_default();
                let words: Vec<&str> = segment.split_whitespace().collect();
                let mut path: Vec<String> = vec![];
                for word in &words {
                    let mut candidate = path.clone();
                    candidate.push((*word).to_string());
                    if tree.contains_key(&candidate) {
                        path = candidate;
                    } else {
                        break;
                    }
                }
                if path.is_empty() {
                    continue;
                }
                let options = &tree[&path].options;
                for word in &words[path.len()..] {
                    if [
                        "chough", "specsync", "git", "then", "or", "and", "hi", "fledge",
                    ]
                    .contains(word)
                    {
                        break;
                    }
                    let Some(flag) = word.strip_prefix("--") else {
                        continue;
                    };
                    let flag: String = flag
                        .chars()
                        .take_while(|character| {
                            character.is_ascii_alphanumeric() || *character == '-'
                        })
                        .collect();
                    if flag.is_empty() {
                        continue;
                    }
                    checked += 1;
                    let flag = format!("--{flag}");
                    if !options.contains(&flag) && !global.contains(&flag.as_str()) {
                        problems.push(format!(
                            "{page}:{}: `chough {}` has no {flag}",
                            index + 1,
                            path.join(" ")
                        ));
                    }
                }
            }
        }
    }
    assert!(
        checked > 100,
        "only {checked} flags found; the scan is broken"
    );
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// A heading's GitHub anchor: lowercase, code marks and punctuation other than `-`
/// and `_` dropped, spaces as hyphens.
fn anchor(heading: &str) -> String {
    heading
        .trim()
        .to_lowercase()
        .chars()
        .filter(|character| character.is_alphanumeric() || " -_".contains(*character))
        .map(|character| if character == ' ' { '-' } else { character })
        .collect()
}

fn anchors(page: &Path) -> BTreeSet<String> {
    let text = fs::read_to_string(page).unwrap().replace("\r\n", "\n");
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    let mut found = BTreeSet::new();
    for line in without_fences(&text).lines() {
        let hashes = line
            .chars()
            .take_while(|character| *character == '#')
            .count();
        if !(1..=6).contains(&hashes) || !line[hashes..].starts_with(' ') {
            continue;
        }
        let base = anchor(&line[hashes..]);
        let count = counts.entry(base.clone()).or_insert(0);
        found.insert(if *count == 0 {
            base.clone()
        } else {
            format!("{base}-{count}")
        });
        *count += 1;
    }
    found
}

/// FR-docs-4: every relative link a page gives reaches a file, and every anchor reaches
/// a heading of the page it names.
#[test]
fn relative_links_and_anchors_resolve() {
    let mut problems = vec![];
    for page in documentation() {
        let text = without_fences(&read(&page));
        let directory = manifest_dir().join(&page).parent().unwrap().to_path_buf();
        for (index, line) in text.lines().enumerate() {
            let mut rest = line;
            while let Some(at) = rest.find("](") {
                rest = &rest[at + 2..];
                let target = rest.split([')', ' ']).next().unwrap_or_default();
                if target.is_empty() || target.contains("://") || target.starts_with("mailto:") {
                    continue;
                }
                let (path, fragment) = target.split_once('#').unwrap_or((target, ""));
                let destination: PathBuf = if path.is_empty() {
                    manifest_dir().join(&page)
                } else {
                    directory.join(path)
                };
                if !destination.exists() {
                    problems.push(format!("{page}:{}: no file for {target}", index + 1));
                    continue;
                }
                let is_page = destination
                    .extension()
                    .is_some_and(|extension| extension == "md");
                if !fragment.is_empty() && is_page && !anchors(&destination).contains(fragment) {
                    problems.push(format!(
                        "{page}:{}: no heading for #{fragment} in {target}",
                        index + 1
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// FR-docs-4: the `.gitattributes` lines the CLI reference tells a project to declare
/// are exactly the merge lines this repository declares.
#[test]
fn cli_reference_declares_the_merge_drivers_this_repository_declares() {
    let declared: BTreeSet<String> = read(".gitattributes")
        .lines()
        .map(str::trim)
        .filter(|line| !line.starts_with('#') && line.contains(" merge="))
        .map(String::from)
        .collect();
    let cli = read("docs/cli.md");
    let section = cli
        .split("Declare the drivers in `.gitattributes`:")
        .nth(1)
        .expect("docs/cli.md tells a project which drivers to declare");
    let (_, block) = fenced_blocks(section, "gitattributes")
        .into_iter()
        .next()
        .expect("a gitattributes block follows");
    let documented: BTreeSet<String> = block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(String::from)
        .collect();
    let missing: Vec<_> = declared.difference(&documented).collect();
    let extra: Vec<_> = documented.difference(&declared).collect();
    assert!(
        missing.is_empty() && extra.is_empty(),
        "docs/cli.md Declare the drivers: missing {missing:?}; not in .gitattributes {extra:?}"
    );
}

/// The command cells of docs/cli.md's Commands table, with the table's `\|` read as `|`.
fn cli_reference_command_cells(reference: &str) -> Vec<String> {
    let section = reference
        .split("\n## Commands\n")
        .nth(1)
        .expect("docs/cli.md has a Commands section");
    let section = section.split("\n## ").next().unwrap();
    section
        .lines()
        .filter(|line| line.starts_with("| `"))
        .map(|line| {
            let line = line.replace("\\|", "\u{0}");
            line.split('|').nth(1).unwrap().replace('\u{0}', "|")
        })
        .collect()
}

/// What follows a command in a Commands table code, split at each `|` outside brackets,
/// so `show|adopt [--apply]` is two alternatives and `[--cycle current|all]` stays one.
fn command_alternatives(rest: &str) -> Vec<String> {
    let mut parts = vec![String::new()];
    let mut depth = 0usize;
    for character in rest.chars() {
        match character {
            '[' => depth += 1,
            ']' => depth = depth.saturating_sub(1),
            '|' if depth == 0 => {
                parts.push(String::new());
                continue;
            }
            _ => {}
        }
        parts.last_mut().unwrap().push(character);
    }
    parts
}

/// FR-docs-4 (#173): docs/cli.md's Commands table matches `chough --help`. Every
/// command has a row, the rows name every subcommand of each command, and every
/// subcommand and flag a row names exists for that command.
#[test]
fn cli_reference_command_table_matches_the_helpers_help() {
    let reference = read("docs/cli.md");
    let cells = cli_reference_command_cells(&reference);
    assert!(
        cells.len() > 15,
        "only {} Commands rows found; the scan is broken",
        cells.len()
    );
    let tree = help_tree();
    let global = ["--help", "--root", "--version"];
    let mut named: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut problems = vec![];
    for cell in &cells {
        for code in cell.split('`').skip(1).step_by(2) {
            let (command, rest) = code.split_once(' ').unwrap_or((code, ""));
            if !tree.contains_key(&vec![command.to_string()]) {
                problems.push(format!(
                    "docs/cli.md Commands: `{command}` is not a chough command"
                ));
                continue;
            }
            let listed = named.entry(command.to_string()).or_default();
            for alternative in command_alternatives(rest) {
                let mut path = vec![command.to_string()];
                let first = alternative.split_whitespace().next().unwrap_or_default();
                if !first.is_empty()
                    && first
                        .chars()
                        .all(|character| character.is_ascii_lowercase() || character == '-')
                {
                    listed.insert(first.to_string());
                    path.push(first.to_string());
                    if !tree.contains_key(&path) {
                        problems.push(format!(
                            "docs/cli.md Commands: `{command}` has no subcommand {first}"
                        ));
                        continue;
                    }
                }
                for word in alternative
                    .split(|character: char| character.is_whitespace() || character == '|')
                {
                    let Some(flag) = word.trim_start_matches('[').strip_prefix("--") else {
                        continue;
                    };
                    let flag: String = flag
                        .chars()
                        .take_while(|character| {
                            character.is_ascii_alphanumeric() || *character == '-'
                        })
                        .collect();
                    if flag.is_empty() {
                        continue;
                    }
                    let flag = format!("--{flag}");
                    if !tree[&path].options.contains(&flag) && !global.contains(&flag.as_str()) {
                        problems.push(format!(
                            "docs/cli.md Commands: `{}` has no {flag}",
                            path.join(" ")
                        ));
                    }
                }
            }
        }
    }
    for command in &tree[&vec![]].subcommands {
        let Some(listed) = named.get(command) else {
            problems.push(format!("docs/cli.md Commands: no row names `{command}`"));
            continue;
        };
        for subcommand in &tree[&vec![command.clone()]].subcommands {
            if !listed.contains(subcommand) {
                problems.push(format!(
                    "docs/cli.md Commands: `{command}` rows do not name {subcommand}"
                ));
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The `X.Y` or `X.Y.Z` version tokens in a clause, without a leading `v` (`v1.8.0`
/// reads as `1.8.0`).
fn versions(clause: &str) -> Vec<String> {
    let mut found = vec![];
    let characters: Vec<char> = clause.chars().collect();
    let boundary =
        |at: usize| at == 0 || !(characters[at - 1].is_alphanumeric() || characters[at - 1] == '.');
    let mut index = 0;
    while index < characters.len() {
        let starts = characters[index].is_ascii_digit()
            && (boundary(index) || (characters[index - 1] == 'v' && boundary(index - 1)));
        if !starts {
            index += 1;
            continue;
        }
        let start = index;
        while index < characters.len()
            && (characters[index].is_ascii_digit() || characters[index] == '.')
        {
            index += 1;
        }
        let token: String = characters[start..index].iter().collect();
        let token = token.trim_end_matches('.').to_string();
        if token.contains('.') && token.split('.').all(|part| !part.is_empty()) {
            found.push(token);
        }
    }
    found
}

fn contains_word(text: &str, word: &str) -> bool {
    text.split(|character: char| !character.is_alphanumeric() && character != '-')
        .any(|token| token == word)
}

/// FR-docs-5: a clause that states which SpecSync, Fledge or HI version is used, pinned
/// or followed names the version CI pins. Dated records keep the versions they ran
/// with: a clause in the past tense, and the decision log and roadmap, are history.
#[test]
fn docs_state_only_the_pinned_tool_versions() {
    let ci = read(".github/workflows/ci.yml");
    let pin = |marker: &str| {
        let line = ci
            .lines()
            .find(|line| line.contains(marker))
            .unwrap_or_else(|| panic!("ci.yml has no {marker}"));
        versions(line).into_iter().next().unwrap()
    };
    let pins = [
        pin("version: '"),
        pin("version: v"),
        pin("human-intent --version"),
    ];
    let allowed: BTreeSet<String> = pins
        .iter()
        .flat_map(|version| {
            let parts: Vec<&str> = version.split('.').collect();
            [version.clone(), format!("{}.{}", parts[0], parts[1])]
        })
        .collect();
    let history = ["docs/DECISIONS.md", "docs/ROADMAP.md"];
    let present = ["uses", "use", "pins", "follows"];
    let past = [
        "observed",
        "used",
        "was",
        "were",
        "earlier",
        "before",
        "first",
        "until",
        "since",
        "older",
        "later",
        "supersedes",
        "superseding",
        "superseded",
    ];
    let tools = ["specsync", "spec-sync", "fledge", "hi", "human-intent"];
    let mut problems = vec![];
    for page in documentation() {
        if history.contains(&page.as_str()) {
            continue;
        }
        let text = without_fences(&read(&page)).replace('\n', " ");
        let clauses = text
            .split(". ")
            .flat_map(|sentence| sentence.split("; "))
            .flat_map(|sentence| sentence.split(": "))
            .flat_map(|sentence| sentence.split(", and "))
            .flat_map(|sentence| sentence.split(", but "));
        for clause in clauses {
            let lower = clause.to_lowercase();
            let says_current = present.iter().any(|word| contains_word(&lower, word));
            let names_tool = tools.iter().any(|tool| contains_word(&lower, tool))
                || contains_word(&lower, "pins");
            let is_history =
                past.iter().any(|word| contains_word(&lower, word)) || lower.contains("d-0");
            if !says_current || !names_tool || is_history {
                continue;
            }
            for version in versions(clause) {
                if !allowed.contains(&version) {
                    problems.push(format!(
                        "{page}: \"{}\" states {version}; CI pins {}",
                        clause.trim(),
                        pins.join(", ")
                    ));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// FR-docs-6 (FLOW-16): after finalize, the implementation-review gate is closed with
/// the human's relayed decision (`chough checkpoint --close-gate`), never "cleared":
/// `--clear-awaiting-human` withdraws a gate and records no decision.
#[test]
fn closeout_guidance_closes_the_review_gate_with_the_relayed_decision() {
    let mut pages = documentation();
    pages.push("docs/diagrams/lifecycle.mmd".into());
    let mut problems = vec![];
    for page in &pages {
        let text = read(page).to_lowercase().replace('\n', " ");
        for phrase in [
            "clear the gate",
            "clears the gate",
            "clearing the gate",
            "clear gate",
        ] {
            if text.contains(phrase) {
                problems.push(format!(
                    "{page}: \"{phrase}\"; close it with `chough checkpoint --close-gate` and the relayed decision"
                ));
            }
        }
    }
    for page in [
        "docs/lifecycle.md",
        "docs/walkthrough.md",
        "docs/diagrams/lifecycle.mmd",
    ] {
        if !read(page).contains("close-gate") {
            problems.push(format!("{page}: the closeout names no `--close-gate`"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// FR-docs-6 (REQ-cli-019): the three skill files this change corrects, as main shipped
/// them at 615b587b, read as known-outdated and update; an edited copy stays.
#[test]
fn skill_files_from_main_615b587b_are_known_outdated_and_edited_copies_stay() {
    let files = [
        (
            "SKILL.md",
            SKILL_AT_615B587B,
            "860cc96169ac2c63de5f60aae1fc7fb417c99a799746a208fcedb968ad6039d5",
        ),
        (
            "references/checkpoint.md",
            CHECKPOINT_AT_615B587B,
            "40afaeaf8494d52c4492861f9983fba0babddbe987673ba4981046aea0218016",
        ),
        (
            "references/lifecycle.md",
            LIFECYCLE_AT_615B587B,
            "9f989fd4b2a9a9b1edd7288a6385a12e4dcaeba324e14ad5308c68233e2d1644",
        ),
    ];
    let state = |status: &Value, file: &str| {
        status["hosts"][0]["files"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["file"] == file)
            .unwrap_or_else(|| panic!("{file} missing from {status}"))["state"]
            .as_str()
            .unwrap()
            .to_string()
    };
    let status = |root: &Path| -> Value {
        serde_json::from_str(&chough(Some(root), &["skill", "status"])).unwrap()
    };

    let project = initialized_project();
    let root = project.path();
    let installed = root.join(".agents/skills/chough-workflow");
    for (file, content, digest) in files {
        assert_eq!(
            format!("{:x}", Sha256::digest(content.as_bytes())),
            digest,
            "{file}"
        );
        fs::write(installed.join(file), content).unwrap();
    }
    let before = status(root);
    for (file, _, _) in files {
        assert_eq!(state(&before, file), "known-outdated", "{file}");
    }
    chough(Some(root), &["skill", "update"]);
    let after = status(root);
    for (file, _, _) in files {
        assert_eq!(state(&after, file), "current", "{file}");
        let delivered = fs::read_to_string(installed.join(file)).unwrap();
        assert_eq!(
            delivered,
            read(&format!("skills/chough-workflow/{file}")),
            "{file}"
        );
    }

    // An edited copy of the same file is kept as it is.
    let edited_project = initialized_project();
    let edited_root = edited_project.path();
    let edited = edited_root.join(".agents/skills/chough-workflow/SKILL.md");
    let content = format!("{SKILL_AT_615B587B}\nA local note.\n");
    fs::write(&edited, &content).unwrap();
    assert_eq!(state(&status(edited_root), "SKILL.md"), "edited");
    chough(Some(edited_root), &["skill", "update"]);
    assert_eq!(fs::read_to_string(&edited).unwrap(), content);
}

/// `skills/chough-workflow/SKILL.md` as main shipped it at 615b587b.
const SKILL_AT_615B587B: &str = r###"---
name: chough-workflow
description: Carry a request or bug from human intent through design, one scope approval, focused testing, local bugs, human acceptance and optional release, with a checkpoint any agent can resume from. Use for Chough-guided development, not unrelated single-file edits.
---

# Chough workflow

You are the agent doing the work. Chough keeps what was wanted, what was checked and
what comes next in project files, so you or another agent can continue later.
Native tools keep their records: HI owns human wording and criterion IDs, SpecSync
owns contracts, changes, approval, verification, review and archive, Fledge runs
tasks, and the actual environment produces evidence. Never copy their state into a
second ledger, and never record a human decision that a human did not make.

## Drive the workflow

Run HI, SpecSync and Fledge yourself. Native commands in references are agent
instructions; give the human outcomes and PRs.

Use explicit prior or standing human authorization for a concrete scope. Record the
agent actor, human source, scope and limits before native approval; do not ask again
just to run a tool. Ask when authority is missing or exceeded. Continuation, bot
reviews and actor labels are not human decisions; approvals are claims.

Keep native checks and separate implementation acceptance and delivery authority.
See [lifecycle](references/lifecycle.md) for scope, provenance and ordering.

## Start or resume

1. Read applicable `AGENTS.md` files. Prefer Fledge for tasks it supports.
2. Find the checkpoint: run `chough next --environment <env>`. Without the helper,
   read `.chough/session.json`, the last lines of `.chough/transitions.jsonl`,
   `.chough/preferences.json`, open bugs in `.chough/bugs/`, and
   `specsync change status <native_change> --json`.
3. Check what is still true: native state, evidence freshness, open blocking bugs,
   and `awaiting_human`. A saved stage or next action is a suggestion; native status
   decides gates.
4. Before your first edit, state the handoff in your own message. Reading it in tool
   output is not the same as stating it. Say the goal; each recorded decision that
   binds the next task, with its reason, naming a rejected approach as rejected; the
   last failure, its bug and that bug's status; the lessons that apply and where they
   are recorded; and the next task. An edit is any file change, including a Chough
   command that writes a record. Where the records do not say one of these, say so
   instead of guessing.
5. Read [the stage contract](references/stages.md), then only the reference for the
   current stage.

With no checkpoint, run `chough init --objective "<outcome>" --intent-scope <HI-family>
--module <module>`; it never overwrites existing files. Without `--host` it installs
the skill for `openai`; any `--host` replaces it; repeat it per host, as in
`--host openai --host codex`. A schema 1 session is migrated only with `chough migrate`.

## Stages

| Stage | Reference | Human decision at the end |
|---|---|---|
| intent | [intent.md](references/intent.md), [roles.md](references/roles.md) | Approve the intent as a whole |
| design (only when visuals or interactions change) | [design.md](references/design.md) | Approve rendered prototype and companion together |
| plan | [plan.md](references/plan.md) | Apply existing human authorization, otherwise one scope decision |
| build | [build.md](references/build.md) | None for routine work inside scope |
| test | [testing.md](references/testing.md) | None; failures go to bugs |
| accept | [accept.md](references/accept.md), [lifecycle.md](references/lifecycle.md) | Native implementation review |
| release (optional) | [release.md](references/release.md) | Only on request or standing setting |
| bugs, validate, librarian (side stages) | [bugs.md](references/bugs.md), [validate.md](references/validate.md), [librarian.md](references/librarian.md) | Publication or optional runs need a human |

Checkpointing is mandatory in every stage: see [checkpoint.md](references/checkpoint.md).

## Working rules

- Talk naturally. Map requests such as "go back to design", "pause" or "skip design,
  it is a CLI change" to `chough stage …` commands, and say what you recorded.
- Resolve routine engineering and tool choices within applicable authorization.
  Ask the human when a consequential product choice, authority or optional execution
  policy remains unresolved. Synthesize findings first; do not return internal tool
  operations as new approval questions.
- Keep doing independent authorized work while a human gate is pending, and record
  the gate with `chough checkpoint --awaiting-human "<decision>"`.
- Evidence states what ran, where and on which inputs. A structural pass, exit code,
  actor label or agent review is not behavioral proof, human approval or deployment.
- Nothing is published, released or deployed as a side effect of finishing work.
- Chough's size bounds cover only its `.chough/` records and displays. Never edit HI
  intent or criteria, specs or SpecSync files to fit one; report a Chough bug.

## Host use

Chough runs in the current agent, with no provider API or runtime. `chough init
--host <openai|claude|codex|cursor|gemini|grok>` copies this bundle into that host's
project skill directory: openai, the default, and claude share one `.agents/skills`
copy; gemini writes `.gemini/skills`, which no run has exercised. Kimi Code CLI uses
`--host openai` (no `kimi` value). `chough skill status` shows whether a copy is
current, edited or outdated; an old `.claude/skills` copy is legacy, removed only by
`chough skill remove-legacy --host claude`. Discovery is host behavior: Claude Code
2.1.316 did not discover `.agents/skills` alone, so it reaches this skill through
`CLAUDE.md` and `AGENTS.md`; Kimi Code CLI 0.41.0 discovered this skill in
`.agents/skills` once on macOS, and an unverified narrative in closed PR #20 said
Google Antigravity read `.agents/skills`. Confirm the host lists the skill; otherwise
give it this file. Subagents are optional: [roles.md](references/roles.md) also
works as sequential passes.
"###;

/// `skills/chough-workflow/references/checkpoint.md` as main shipped it at 615b587b.
const CHECKPOINT_AT_615B587B: &str = r###"# Checkpoint and cold resume

Essential state must never live only in model context. `.chough/session.json` is the
single writable checkpoint. It works without the librarian and without any other
storage adapter.

## What it holds

| Field | Content |
|---|---|
| `objective`, `stage` | Current outcome, stage, status, side-stage callers and iteration |
| `native_change` | The SpecSync change ID; native status decides its state |
| `references` | Criteria, requirements, cases, design sources and files in scope |
| `decisions`, `open_questions` | Decisions with their reason in the sentence; questions that matter |
| `completed`, `observations`, `bugs`, `blockers` | Work done and the records that show it |
| `awaiting_human` | Real gates still waiting for a person |
| `run_consents` | Consent for this objective only |
| `next_action` | Suggested next step; never overrides native state |

Standing preferences are in `.chough/preferences.json`. Legacy transitions stay in
`.chough/transitions.jsonl`; adopted and new projects use immutable events under
`.chough/history/cycles/`, selected by `history_cursor` in the same checkpoint.
Observations are in `.chough/evidence/`, bugs in
`.chough/bugs/`, case definitions in `.chough/cases/` and retention removals in
`.chough/pruned.jsonl`.

## When to save

Save after each meaningful increment, before handing off, before context is cleared,
and whenever a human gate opens or closes:

```sh
chough checkpoint --completed "<what was done>" --observation <id> \
  --decision "<decision, because reason>" --next-action "<next step>"
```

Opening a gate with `--awaiting-human "<gate>"` appends one `gate-opened` record to
`.chough/transitions.jsonl` (only when that gate is not already open), and
`--clear-awaiting-human` appends one `gate-closed` record per gate it clears. They
record when a gate waited, and `chough timing` reads them to show waiting-on-human
time. Open and close gates through these flags rather than by editing `session.json`,
or the waiting time is lost.

When the human decides a gate, record their decision on their behalf with
`chough checkpoint --close-gate "<gate>" --decided-by "<who>" --words "<their exact
words>" --where "<where and when, and who relayed it>" --recorded-by "<agent>"`. It
closes only that gate, puts `relayed_by` on its `gate-closed` record and adds one
decision. It is a relayed claim, not authenticated, and never a native approval or
review. Each refusal saves nothing and names its recovery.

`--clear-awaiting-human` withdraws every open gate and records no decision: use it
for gates no human decision closes. To withdraw some gates and keep the others, pass
the gates still waiting with `--awaiting-human` in the same call; they are logged
closed and opened again, so their waiting time restarts there.

Write decisions with a prefix and a reason, so a fresh agent can tell what was chosen,
what was ruled out and what already failed:

```text
decided: <what> because <reason>
rejected: <approach> because <reason>
failed: <experiment> because <reason>
```

This is a convention, read best-effort: the prefix starts the entry, lowercase, with a
colon, and the reason is the text after the last ` because `. Any other entry, such as
older free text or a dated note, is still kept and shown whole. Decisions stay plain
strings in the checkpoint; nothing validates the convention. Record a human's decision
only in their words, and label an agent's own assumption as the agent's, without a
`decided:` prefix.

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly); it is
   the focused view, and `chough status` is the full one. Besides the stage, native
   state, next action and gates, it shows the 5 newest decisions with their kind and
   reason, the latest failed observation by recorded time with the status of each bug
   that links it, and where each module's lessons live (`<module>.3md`, or the
   `## Lessons` line of `context.md`) with the newest archived SpecSync lesson
   bundles. It points to lessons and never prints them; read the source it names.
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`. A referenced observation is `current`,
   `historical` (present but stale), `pruned` (removed by retention, in the ledger) or
   `unavailable` (missing, not in the ledger); only a current one counts, so rerun the
   others rather than treat them as passed.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself. In stage done with the native
   change archived, the cycle is closed: `chough next` recommends `chough stage reset
   --to intent --reason "<why>"` (or `--to plan` when the approved intent already
   covers the new work), then `specsync change new "<outcome>" --spec <module> --path
   <path>` and `chough checkpoint --native-change <id> --next-action "<next step>"`.
   The same holds at an earlier stage when the native change is already archived:
   `chough next` names the archive and the new cycle instead of continuing, after
   `chough stage advance` to done at accept. A reset replaces the checkpoint's next
   action.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns live
checkpoint writes; others report to it. APS was evaluated as a backend and deferred:
it lost concurrent read-modify-write updates without error and accepts unknown fields.

For committed parallel schema 2 edits, the repository declares:

```gitattributes
.chough/session.json merge=chough-session
```

Each clone that wants automatic merging configures the driver locally:

```sh
git config --local merge.chough-session.name "Chough schema 2 checkpoint"
git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"
```

Git supplies base, ours and theirs as `%O`, `%A` and `%B`. `chough init` never sets
these values. Without a driver, use the explicit fallback:

```sh
chough checkpoint --merge <base> <ours> <theirs>
```

The merge preserves ordered exact-string history and reference changes. It treats
objective, intent scope, modules, stage, native change, next action, run consents,
blockers and awaiting-human gates as one current-cycle group. If both sides change
that group differently, leave the conflict unresolved and ask the checkpoint owner
or human to select a coherent cycle. Never hand-resolve it by combining fields. A
merge is not approval, consent, gate closure or a lifecycle transition.

Success replaces only ours. Every refusal preserves all three inputs and writes no
Chough record. The merge adds no second checkpoint store and does not change the
one-owner rule for live writes.

## Sessions per objective

When `.chough/sessions/` exists, each objective or native change has its own
checkpoint at `.chough/sessions/<key>.json`, and every command uses the active one
(`chough session new|adopt|use|list`). Start one with `chough session new <change-id>`
or move the existing checkpoint with `chough session adopt <key>`. Finished sessions
move to `.chough/sessions/archive/` when you switch away. Subagents do not edit the
session: they run `chough inbox add --session <key> --from <name> --kind
observation|decision|completed|question --text <text>`, and the agent that owns the
session runs `chough inbox fold`. Inbox entries never approve, review, consent, clear a
gate or move a stage. Declare the merge driver for session files as well:

```gitattributes
.chough/sessions/*.json merge=chough-session
.chough/sessions/archive/*.json merge=chough-session
```

## Immutable transition history

New projects use event history. Existing schema-2 projects remain in legacy mode
until the owner previews `chough history adopt` and explicitly runs
`chough history adopt --apply`. Adoption preserves the legacy log and schema-1
migration backup byte-for-byte. A digest-bound anchor detects later legacy changes.
An older helper may reject an adopted checkpoint; use a compatible helper and never
remove its cursor to make an old reader accept it.

The optional `history_cursor` is part of the atomic current-cycle merge group.
Immutable event paths from independent branches can integrate, while divergent
cursors require selecting one coherent checkpoint. Other branch events and orphan
writes remain visible in `chough history show`; they do not prove a completed stage.
Only parent links on the selected lineage order current work. Native gates still
decide lifecycle transitions.

Recent guidance follows at most 128 events and 512 KiB. Local appends to a new cycle allow at most 4,096
events and 16 MiB; a capacity refusal preserves the checkpoint, and a reasoned
`chough stage reset` can begin a new cycle without deleting the old proof. Timing
bounds transition content at 4,096 events / 16 MiB and keeps other tips separate
with unknown durations. Missing or intentionally pruned history is partial, never
zero or verified.

`chough history prune` is a read-only preview. It preserves active or referenced
cycles, unresolved branches/orphans, the newest three closed cycles including time
ties, and cycles younger than 14 days. Protected proof can prevent reclamation.
`--apply` requires both an explicit history-prune command and human retention
authorization; evidence auto-retention never starts history pruning. Maintenance
has one 16,384-read / 64-MiB metadata budget, including revalidation. An incomplete
scan refuses apply, and an interruption leaves an immutable receipt with an
explicit partial result. Applying adoption or cleanup to a real project is a
separate human operation, never a side effect of finishing implementation.

Before integrating a branch that pruned history, check the other branch's new
references and retained hashes. If new proof references removed content, restore
the exact bytes from Git before accepting integration; a pruned or unavailable
reference is not passing evidence. The one-owner live checkpoint and maintenance
rule still applies; arbitrary external filesystem writers are not serialized.

Git may combine independently written event sets above that local capacity. Retain
all imported proof, refuse further ordinary appends to the full cycle, and use a
reasoned reset for new work. The limit never silently discards merged events.
"###;

/// `skills/chough-workflow/references/lifecycle.md` as main shipped it at 615b587b.
const LIFECYCLE_AT_615B587B: &str = r###"# Native lifecycle and evidence

Installed `specsync change status <id> --json`, `specsync change ship-status <id>` and
command help are authoritative for the current state and recovery path. This
reference follows SpecSync 6.1; do not hardcode it over another version's output.
Match the SpecSync version used by lifecycle writers and CI.

You run the native commands in this reference as part of Chough's current-agent
workflow. The human gives the goal and authorization and reviews the resulting PR;
they do not need to dispatch SpecSync or Fledge operations themselves. Report the
result and genuine decisions still needed, then perform authorized recovery steps.

## Shape of a change

```text
new → interview → artifacts and delta → one human scope approval → implement →
check (when requirements change) → cases run → gate → commit with .chough/ →
check --commit → present → human implementation review → review → ship/finalize →
clear gate, stage advance → commit → push, then CI on the archived tip → merge
```

Between `check --commit` and finalize, nothing writes to `.chough/` and nothing is
committed: SpecSync's verification digest covers `.chough/`. SpecSync 6.1.2 ships an
opt-in exclusion, `verification_ignored_paths`, but Chough's own repository has not
adopted it, because entries are directory prefixes and covering `.chough/session.json`
would also hide `.chough/cases/`. Unless a project lists only safe directories, any
Chough write in that window forces another `change check`. See [accept.md](accept.md)
for the full order.

| Native state | Chough stage | Gate |
|---|---|---|
| draft | intent, design, plan | build, test, accept and done refused |
| approved, implementing, verifying, with `approval_valid` true | build, test, accept | build, test and accept allowed; done refused |
| archived, with authenticated `terminal_evidence` | test, accept, done | done allowed; build refused (new work needs a new change) |

Chough reads `summary.state`, `summary.approval_valid`, `summary.correction_valid` and,
for archived changes, `summary.terminal_evidence.validity`. The same prerequisite
applies whichever command moves the stage (advance, skip, back, reset, return or
release → done). It refuses when status is missing, cannot be decoded or reports an
invalid approval or a correction reported invalid (`correction_valid` false), when no
native change is recorded in the checkpoint (`refused:no-native-change`; record one
with `chough checkpoint --native-change <id>`), and when `.specsync/sdd.json` is not a
valid SpecSync policy. Two cases are not gated at all: with no SpecSync configuration
the gate reads `ungated:not-configured`, and with no `.specsync/sdd.json` or one with
`"enabled": false` it reads `ungated:sdd-disabled`. It never runs `approve`, `review`
or `finalize` itself. The current agent invokes those native writers when the
applicable human authority exists.

## Approval

Before recording `specsync change approve <id> --actor "<agent-account>"`, identify the
actual human authority that covers this concrete definition. It may be an explicit
prior decision or a standing instruction authorizing a bounded class of changes,
such as repository ticket implementations delivered through PRs. Record the source,
covered scope and limits in the checkpoint decisions and native context, and cite
that source in the native approval note. Record the acting agent account as the
native actor, separately from the human authorization source, then continue without
asking the human to repeat permission for this internal operation. Do not use the
human's name merely because they authorized the agent's work.

If the proposed work exceeds that authority, ask the consequential scope question.
An ordinary keep-working instruction without applicable authority, an agent label,
a bot review or a passing check does not create human approval. A changed approved
definition follows native reapproval rules; existing standing authority applies
only while it still covers the new scope. Actor text is a portable claim, not
identity. Scope authorization leaves implementation acceptance and delivery
boundaries separate.

## Verification

`specsync change check <id>` materializes the approved delta. Cases may link the
requirements it adds or modifies before then while the checkpoint records the
approved change (see testing.md). `specsync change check <id>
--commit` also records targeted structural verification; run it after committing the
source with `.chough/`. Stage new source files first. It does not run the project's
tests; product checks are separate observations. It refuses an edited delta and
incomplete tasks. Fix real failures; if evidence is stale, follow the exact recovery
action status names.

## Review, archive, merge

The hosted Archive gate refuses while any native change is open, so the closeout is
archive-first: finish it locally before you wait on CI. After the applicable local
gates, present the work; after the human reviews the change package, diff, delta and
evidence:

```sh
specsync change review <id> --reviewer "<agent-account>"
specsync change ship <id>
```

Use the project agent account for the native record and retain the actual human
acceptance source separately, with `chough checkpoint --close-gate`; the actor label
does not supply that acceptance, and the native review is a recorded claim bound to
the reviewed content, not proof of who reviewed. The
reviewer may be the scope approver. Do not commit between review and ship or
finalize. Archive on the same pull request, then push and observe required CI at the
actual tip, and merge only when every change on that pull request is archived. A push
before the review only shows the work early: expect the Archive gate red and do not
wait on it. Review and
finalize several changes one at a time.

Merge, package publication, release, deployment and live health are separate
observations; see [release.md](release.md).

## Evidence records

`chough cases run` and `chough evidence add` record with an observation the Git
revision, a working content identity, the environment, artifacts and, for cases, the
definition digest. Freshness compares the content identity, not the revision: it
covers every tracked or untracked non-ignored file except `.chough/`, `target/` and
SpecSync lifecycle bookkeeping (a change's `state.json`, `approvals.json`,
`verification.json`, `verification-attempts.json`, `review.json`,
`review-attempts.json`, `finalization.json`, `accepted-state.json`,
`lesson-bundle.md` and the front-matter `state:` value of `change.md`). So committing
only Chough records, an empty commit, `change check --commit` and finalize keep an
observation current, while a change to source, specs, configuration or a change's
deltas and other artifacts makes it stale. A record without a content identity, from
an earlier Chough, keeps the revision-bound comparison and is stale after any commit.
The records do not authenticate a witness or verify a remote artifact. A changed
input, artifact, case definition or environment makes the observation stale.

## Moves that already happened

When a stage moved outside Chough (for example in a subagent), record it with
`chough stage backfill --at <original time> --commit <rev> --quote "<the words it rests
on>"` rather than advancing now with "recorded after the fact" in a note. Native state
is still read now and every advance refusal applies. The quote is evidence only: it
never records a human approval or review, so record human gates and native approvals
the usual way when they happen.
"###;
