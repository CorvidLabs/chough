//! The optional integrations folder: versioned example hc templates (REQ-cli-065,
//! LIFE-24, LIFE-22), and Chough never starting hc, Herdr or Rune.

use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Each host folder, the hc template name in it, and the `chough init --host` value
/// its template uses.
const HOSTS: &[(&str, &str, &str)] = &[
    ("claude-code", "chough-claude", "claude"),
    ("codex", "chough-codex", "openai"),
    ("kimi", "chough-kimi", "openai"),
    ("grok", "chough-grok", "grok"),
    ("local", "chough-local", "openai"),
];

fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    let path = manifest_dir().join(relative);
    fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn template_path(host: &str) -> String {
    format!("integrations/{host}/hc-template.md")
}

/// The front matter's `key: value` lines and the body after it.
fn split_template(text: &str) -> (Vec<(String, String)>, String) {
    let rest = text
        .strip_prefix("---\n")
        .expect("a template starts with a front matter line");
    let (front, body) = rest
        .split_once("\n---\n")
        .expect("a template's front matter ends with a --- line");
    let fields = front
        .lines()
        .map(|line| {
            let (key, value) = line
                .split_once(':')
                .unwrap_or_else(|| panic!("front matter line without a key: {line:?}"));
            (key.trim().to_string(), value.trim().to_string())
        })
        .collect();
    (fields, body.to_string())
}

/// The `--host` values `chough init --help` lists.
fn accepted_init_hosts() -> BTreeSet<String> {
    let output = Command::new(env!("CARGO_BIN_EXE_chough"))
        .args(["init", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8_lossy(&output.stdout).into_owned();
    let line = help
        .lines()
        .find(|line| line.trim_start().starts_with("--host"))
        .expect("init --help documents --host");
    let values = line
        .split("[possible values: ")
        .nth(1)
        .and_then(|rest| rest.split(']').next())
        .expect("--host lists its possible values");
    values.split(", ").map(str::to_string).collect()
}

fn manifest() -> toml::Table {
    read("integrations/templates.toml")
        .parse::<toml::Table>()
        .expect("integrations/templates.toml is TOML")
}

fn manifest_entries() -> Vec<toml::Table> {
    manifest()["template"]
        .as_array()
        .expect("templates.toml has [[template]] entries")
        .iter()
        .map(|entry| entry.as_table().expect("a [[template]] is a table").clone())
        .collect()
}

fn text(entry: &toml::Table, key: &str) -> String {
    entry
        .get(key)
        .and_then(toml::Value::as_str)
        .unwrap_or_else(|| panic!("template entry {entry:?} has no text {key}"))
        .to_string()
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(folder) = pending.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    files
}

/// Whether `text` names `word` as a whole word, ignoring case.
fn names_word(text: &str, word: &str) -> bool {
    let lower = text.to_lowercase();
    lower.match_indices(word).any(|(start, _)| {
        let before = lower[..start].chars().next_back();
        let after = lower[start + word.len()..].chars().next();
        let boundary = |c: Option<char>| c.is_none_or(|c| !c.is_alphanumeric() && c != '_');
        boundary(before) && boundary(after)
    })
}

#[test]
fn each_template_has_hcs_front_matter_resumes_with_chough_next_and_holds_no_local_path() {
    let accepted = accepted_init_hosts();
    for (host, name, init_host) in HOSTS {
        let path = template_path(host);
        let (fields, body) = split_template(&read(&path));
        let keys: Vec<&str> = fields.iter().map(|(key, _)| key.as_str()).collect();
        assert_eq!(keys, ["name", "description", "command"], "{path}");
        assert_eq!(fields[0].1, *name, "{path}");
        assert!(!fields[1].1.is_empty() && !fields[2].1.is_empty(), "{path}");
        assert!(
            body.contains("chough next --environment local"),
            "{path} resumes with chough next"
        );
        let named: Vec<&str> = body
            .split("--host ")
            .skip(1)
            .filter_map(|rest| rest.split(|c: char| !c.is_ascii_alphanumeric()).next())
            .collect();
        assert_eq!(named, [*init_host], "{path} names one init host");
        assert!(accepted.contains(*init_host), "{init_host} is accepted");
        for (line_number, line) in read(&path).lines().enumerate() {
            for token in line.split(|c: char| c.is_whitespace() || "`'\"()".contains(c)) {
                let local = token.starts_with('/')
                    || token.starts_with("~/")
                    || token.starts_with("$HOME")
                    || token.contains(":\\");
                assert!(
                    !local,
                    "{path}:{} holds a local path: {token}",
                    line_number + 1
                );
            }
        }
    }
}

#[test]
fn the_manifest_versions_every_template_with_its_digest() {
    let table = manifest();
    assert_eq!(
        table.get("schema").and_then(toml::Value::as_str),
        Some("chough-integrations/1")
    );
    let entries = manifest_entries();
    let hosts: Vec<String> = entries.iter().map(|entry| text(entry, "host")).collect();
    let expected: Vec<String> = HOSTS.iter().map(|(host, _, _)| host.to_string()).collect();
    assert_eq!(hosts, expected);
    for (entry, (host, name, init_host)) in entries.iter().zip(HOSTS) {
        assert_eq!(text(entry, "file"), template_path(host));
        assert_eq!(text(entry, "name"), *name);
        assert_eq!(text(entry, "init_host"), *init_host);
        let version = entry.get("version").and_then(toml::Value::as_integer);
        assert!(version.is_some_and(|version| version >= 1), "{host} has a version");
        let status = if *host == "local" {
            "unqualified"
        } else {
            "supported"
        };
        assert_eq!(text(entry, "status"), status, "{host}");
        let bytes = fs::read(manifest_dir().join(template_path(host))).unwrap();
        assert_eq!(
            text(entry, "sha256"),
            format!("{:x}", Sha256::digest(&bytes)),
            "{host}'s template changed: bump its version and update its sha256 in integrations/templates.toml"
        );
    }
}

#[test]
fn a_changed_template_byte_no_longer_matches_its_listed_digest() {
    let entries = manifest_entries();
    let entry = &entries[0];
    let mut bytes = fs::read(manifest_dir().join(text(entry, "file"))).unwrap();
    bytes.push(b'\n');
    assert_ne!(
        text(entry, "sha256"),
        format!("{:x}", Sha256::digest(&bytes))
    );
}

#[test]
fn chough_never_starts_hc_herdr_or_rune_and_its_skill_never_names_them() {
    for path in files_under(&manifest_dir().join("src")) {
        let source = fs::read_to_string(&path).unwrap();
        for (start, _) in source.match_indices("Command::new(") {
            let argument = source[start + "Command::new(".len()..]
                .split(')')
                .next()
                .unwrap_or_default()
                .trim();
            for program in ["\"hc\"", "\"herdr\"", "\"rune\"", "HC"] {
                assert_ne!(argument, program, "{} starts {program}", path.display());
            }
        }
        for tool in ["herdr", "rune"] {
            assert!(!names_word(&source, tool), "{} names {tool}", path.display());
        }
    }
    for path in files_under(&manifest_dir().join("skills")) {
        let text = fs::read_to_string(&path).unwrap();
        for tool in ["hc", "herdr", "rune"] {
            assert!(!names_word(&text, tool), "{} names {tool}", path.display());
        }
    }
}

#[test]
fn the_readme_names_every_template_and_says_chough_has_no_runtime_dependency() {
    let readme = read("integrations/README.md");
    for (host, _, _) in HOSTS {
        assert!(
            readme.contains(&format!("{host}/hc-template.md")),
            "the README names {host}'s template"
        );
    }
    assert!(readme.contains("optional"));
    assert!(readme.contains("no runtime dependency on hc, Herdr or Rune"));
    assert!(readme.contains("separate Fledge plugin"));
    assert!(readme.contains("templates.toml"));
}
