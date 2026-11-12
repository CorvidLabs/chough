//! Record files and the records tree (REQ-records-001, REQ-records-002), date and slug
//! names with aliases (REQ-records-003), the session record (REQ-records-004), proof
//! files and external artifacts (REQ-records-005), removal only of committed bytes
//! (REQ-records-012) and key screenshots (REQ-records-013).

mod common;

use common::Fixture;
use std::collections::BTreeSet;
use std::path::Path;

/// Every file under `.chough/` except local state, by project-relative path.
fn chough_files(root: &Path) -> BTreeSet<String> {
    fn walk(root: &Path, relative: &str, out: &mut BTreeSet<String>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if name == ".chough/local" {
                continue;
            }
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else {
                out.insert(name);
            }
        }
    }
    let mut out = BTreeSet::new();
    walk(root, ".chough", &mut out);
    out
}

/// Whether `path` is one REQ-records-002 lists for Chough's committed records.
fn allowed(path: &str) -> bool {
    let parts: Vec<&str> = path.split('/').collect();
    match parts.as_slice() {
        [".chough", "README.md"] | [".chough", "preferences.json"] => true,
        [".chough", "cases", name] => name.ends_with(".cases.toml"),
        [".chough", "sessions", _, "session.md"] => true,
        [".chough", "sessions", _, "evidence" | "agent", ..] => true,
        [".chough", "archive", "sessions", name] => name.ends_with(".md"),
        [".chough", "archive", "sessions", _, _] => true,
        [".chough", "archive", "shelved", ..] => true,
        [".chough", "archive", "migrations", name] => name.ends_with(".md"),
        [".chough", "bugs", name] => name.ends_with(".md"),
        _ => false,
    }
}

/// REQ-records-002: init writes the reading guide create-only, and a full cycle leaves
/// committed files under `.chough/` only at the listed paths.
#[test]
fn the_tree_holds_only_the_listed_paths() {
    let fixture = Fixture::new();
    fixture.ok(&["init", "--intent-scope", "LIFE", "--module", "cli"]);
    let mut top: Vec<String> = std::fs::read_dir(fixture.root().join(".chough"))
        .unwrap()
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    top.sort();
    assert_eq!(top, ["README.md", "cases", "local"]);
    let readme = fixture.read(".chough/README.md");
    for folder in [
        "sessions/",
        "archive/sessions/",
        "archive/shelved/",
        "archive/migrations/",
        "bugs/",
        "cases/",
        "preferences.json",
        "local/",
        "never committed",
        "](../hi/)",
        "](../specs/)",
        "](../.specsync/archive/)",
    ] {
        assert!(
            readme.contains(folder),
            "the guide lacks {folder}:\n{readme}"
        );
    }
    // An existing guide is kept byte for byte.
    fixture.write(".chough/README.md", "A person's own guide.\n");
    fixture.ok(&["init", "--intent-scope", "LIFE", "--module", "cli"]);
    assert_eq!(fixture.read(".chough/README.md"), "A person's own guide.\n");

    // A full cycle: a session to done with a bug, a correction and an inbox entry.
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Alpha breaks",
        "--symptom",
        "It breaks",
        "--trigger",
        "Run it",
        "--expected",
        "It works",
        "--actual",
        "It broke",
        "--environment",
        "local",
        "--impact",
        "Low",
        "--scope",
        "in-scope",
        "--classification",
        "product-defect",
        "--source",
        "a test",
    ]);
    let id = fixture.id_of("alpha");
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &id,
        "--from",
        "helper",
        "--kind",
        "decision",
        "--text",
        "Keep it small",
    ]);
    fixture.ok(&["inbox", "fold"]);
    while !fixture.is_archived("alpha") {
        fixture.ok(&["stage", "advance", "--json"]);
    }
    let stray: Vec<String> = chough_files(fixture.root())
        .into_iter()
        .filter(|path| !allowed(path))
        .collect();
    assert!(
        stray.is_empty(),
        "files outside the records tree: {stray:?}"
    );
}

/// REQ-records-001: records are Markdown with front matter, a person's edits survive
/// Chough's writes, front matter outside the subset is refused naming the file, line and
/// rule, and a CRLF copy reads as the LF original.
#[test]
fn every_record_is_markdown_and_keeps_human_edits() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let file = fixture.session_file();
    // A person edits the Goal paragraph, adds a key Chough does not know and a comment.
    let text = fixture.read(&file);
    let edited = text
        .replacen(
            "---\n",
            "---\nowner_note: \"keep me\"\n# a person's comment\n",
            1,
        )
        .replacen(
            "## Goal\n\nShip alpha\n",
            "## Goal\n\nShip alpha, the small version a person described.\n",
            1,
        );
    assert_ne!(edited, text);
    fixture.write(&file, &edited);
    fixture.ok(&["stage", "advance", "--json"]);
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "decided: small because time is short",
    ]);
    let after = fixture.read(&file);
    for kept in [
        "owner_note: \"keep me\"\n",
        "# a person's comment\n",
        "Ship alpha, the small version a person described.\n",
    ] {
        assert!(after.contains(kept), "lost {kept:?}:\n{after}");
    }
    assert!(after.starts_with("---\n"), "{after}");

    // A bug's Steps paragraph a person edits survives an update.
    let added = fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Alpha breaks",
        "--symptom",
        "It breaks",
        "--trigger",
        "Run it",
        "--expected",
        "It works",
        "--actual",
        "It broke",
        "--environment",
        "local",
        "--impact",
        "Low",
        "--scope",
        "in-scope",
        "--classification",
        "product-defect",
        "--source",
        "a test",
    ]);
    let bug = added["recorded"]
        .as_str()
        .or_else(|| added["bug"]["path"].as_str())
        .map(String::from)
        .unwrap_or_else(|| {
            let mut found = chough_files(fixture.root())
                .into_iter()
                .filter(|path| path.starts_with(".chough/bugs/"));
            found.next().unwrap()
        });
    let text = fixture.read(&bug);
    let steps = "**Steps:** Trigger: Run it, as a person wrote it out in full.\n";
    let edited = text.replacen("**Steps:** Trigger: Run it\n", steps, 1);
    assert_ne!(edited, text, "{text}");
    fixture.write(&bug, &edited);
    let bug_id = edited
        .lines()
        .find_map(|line| line.strip_prefix("id: "))
        .unwrap()
        .to_string();
    fixture.ok(&["bug", "update", &bug_id, "--note", "Seen again"]);
    assert!(fixture.read(&bug).contains(steps), "{}", fixture.read(&bug));

    // Front matter outside the subset is refused, naming the file, line and rule.
    let good = fixture.read(&file);
    for (bad, rule) in [
        (
            good.replacen("---\n", "---\nnested:\n  inner:\n    deep: x\n", 1),
            "line",
        ),
        (
            good.replacen("---\n", "---\nkind: session\n", 1),
            "duplicate",
        ),
        (good.replacen("---\n", "---\nanchor: &a x\n", 1), "line"),
    ] {
        fixture.write(&file, &bad);
        let refused = fixture.fails(&["checkpoint", "--decision", "never written"]);
        assert!(
            refused.contains(&file) && refused.contains(rule),
            "{rule}: {refused}"
        );
        assert_eq!(fixture.read(&file), bad, "a refused write changed the file");
    }
    fixture.write(&file, &good);

    // A CRLF copy reads the same as the LF original.
    let lf = fixture.ok(&["checkpoint"]);
    fixture.write(&file, &good.replace('\n', "\r\n"));
    assert_eq!(fixture.ok(&["checkpoint"]), lf);
    fixture.write(&file, &good);

    // No JSON twin under .chough/ but the preferences.
    let json: Vec<String> = chough_files(fixture.root())
        .into_iter()
        .filter(|path| path.ends_with(".json") && path != ".chough/preferences.json")
        .collect();
    assert!(json.is_empty(), "{json:?}");
}

// ---------------------------------------------------------------------------
// Helpers for the record tests below (REQ-records-003, -004, -005, -012, -013).

/// A front matter scalar, unquoted, or a flow list as written (`[a, b]`).
fn front(text: &str) -> std::collections::BTreeMap<String, String> {
    let (front, _) = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .unwrap_or_else(|| panic!("no front matter in {text}"));
    front
        .lines()
        .filter(|line| !line.starts_with(' '))
        .filter_map(|line| line.split_once(':'))
        .map(|(key, value)| {
            let value = value.trim();
            let value = if value.starts_with('"') {
                serde_json::from_str(value).unwrap_or_else(|_| value.to_string())
            } else {
                value.to_string()
            };
            (key.to_string(), value)
        })
        .collect()
}

/// The front matter's keys, in order.
fn front_keys(text: &str) -> Vec<String> {
    let (front, _) = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .unwrap_or_else(|| panic!("no front matter in {text}"));
    front
        .lines()
        .filter(|line| !line.starts_with(' '))
        .filter_map(|line| line.split_once(':').map(|(key, _)| key.to_string()))
        .collect()
}

/// A record's body, after its front matter.
fn body(text: &str) -> String {
    text.strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .map(|(_, body)| body.to_string())
        .unwrap_or_else(|| panic!("no front matter in {text}"))
}

/// The items of a flow list value (`[a, b]`).
fn items(value: &str) -> Vec<String> {
    value
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(", ")
        .filter(|item| !item.is_empty())
        .map(|item| item.trim_matches('"').to_string())
        .collect()
}

/// The lines of one `## <heading>` section of a body.
fn section(body: &str, heading: &str) -> Vec<String> {
    let start = format!("## {heading}\n");
    let Some(at) = body.find(&start) else {
        return vec![];
    };
    let rest = &body[at + start.len()..];
    let end = rest.find("\n## ").map_or(rest.len(), |end| end + 1);
    rest[..end]
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(str::to_string)
        .collect()
}

/// The active session's evidence entries: (file name, text), sorted.
fn entries(fixture: &Fixture) -> Vec<(String, String)> {
    let folder = fixture
        .root()
        .join(format!(".chough/sessions/{}/evidence", fixture.active_id()));
    let mut found: Vec<(String, String)> = std::fs::read_dir(folder)
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().to_string())
                .filter(|name| name.ends_with(".md"))
                .map(|name| {
                    let text = std::fs::read_to_string(fixture.root().join(format!(
                        ".chough/sessions/{}/evidence/{name}",
                        fixture.active_id()
                    )))
                    .unwrap();
                    (name, text)
                })
                .collect()
        })
        .unwrap_or_default();
    found.sort();
    found
}

/// Every file under `relative` with its bytes; `.chough/local/` is left out.
fn bytes_under(root: &Path, relative: &str) -> std::collections::BTreeMap<String, Vec<u8>> {
    fn walk(root: &Path, relative: &str, out: &mut std::collections::BTreeMap<String, Vec<u8>>) {
        let Ok(entries) = std::fs::read_dir(root.join(relative)) else {
            return;
        };
        for entry in entries.flatten() {
            let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
            if name == ".chough/local" {
                continue;
            }
            if entry.path().is_dir() {
                walk(root, &name, out);
            } else {
                out.insert(name, std::fs::read(entry.path()).unwrap());
            }
        }
    }
    let mut out = std::collections::BTreeMap::new();
    walk(root, relative, &mut out);
    out
}

fn sha256(bytes: &[u8]) -> String {
    use sha2::Digest;
    format!("{:x}", sha2::Sha256::digest(bytes))
}

fn is_hex(text: &str, length: usize) -> bool {
    text.len() == length
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

/// `evidence add` with the stand-in criterion, requirement and contract, plus `extra`.
fn evidence_args<'a>(summary: &'a str, outcome: &'a str, extra: &[&'a str]) -> Vec<&'a str> {
    let mut args = vec![
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "product-test",
        "--outcome",
        outcome,
        "--environment",
        "local",
        "--summary",
        summary,
    ];
    args.extend_from_slice(extra);
    args
}

/// The status row of the entry whose ID is `id`.
fn status_row(fixture: &Fixture, id: &str) -> serde_json::Value {
    let status = fixture.ok(&["status", "--environment", "local"]);
    status["observations"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["record"]["id"] == id)
        .unwrap_or_else(|| panic!("{id} is not listed: {status:#}"))
        .clone()
}

/// TC-records-proof-split-and-external (REQ-records-005; #215, #14 item 3). A case run's
/// raw output stays in `.chough/local/logs/`, and its committed entry gives the exact
/// command and the output's SHA-256; no file Git tracks under `.chough/` holds a raw
/// log. A failing cargo-test case keeps its key lines from the first failure block and
/// the `test result:` line: at most 20 lines and 2,048 bytes, without escape sequences,
/// a home path redacted and counted. `evidence add --failing-lines` keeps the same for
/// a manual failure, its text checked as other text is (REQ-records-006). An absolute
/// artifact outside the project is fingerprinted in place under a neutral `ext-` ID,
/// its path only in `.chough/local/external-artifacts.toml`. A missing path, a
/// directory, a symlink and a 64 MiB + 1 byte file are refused, naming the path, with
/// nothing recorded. An edit reads `changed`; without `.chough/local/` the artifact
/// reads `external, not checkable here`, which neither passes nor stales the entry.
#[cfg(unix)]
#[test]
fn logs_stay_local_and_failures_keep_key_lines() {
    let fixture = Fixture::new();
    fixture.stubs();
    // A cargo stand-in whose run fails twice, coloured, with a home path in the first
    // failure block and a marker only the raw output holds.
    let mut output = String::from("RAW-OUTPUT-ONLY marker line\nrunning 3 tests\n");
    output.push_str("\x1b[31mtest first ... FAILED\x1b[0m\ntest second ... FAILED\ntest third ... ok\n\nfailures:\n\n");
    output.push_str("---- first stdout ----\nthread 'first' panicked at tests/x.rs:1:1:\n");
    output.push_str("FIRST-PANIC at /Users/SYNTHETIC-user/project/src/lib.rs\n");
    output.push_str("\x1b[1mleft: 1\x1b[0m\nright: 2\n");
    for line in 0..30 {
        output.push_str(&format!("first detail {line}\n"));
    }
    output.push_str(
        "\n---- second stdout ----\nthread 'second' panicked at tests/x.rs:2:1:\nSECOND-PANIC\n",
    );
    for line in 0..30 {
        output.push_str(&format!("second detail {line}\n"));
    }
    output.push_str("\nfailures:\n    first\n    second\n\ntest result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out\n");
    fixture.write("bin/cargo-output.txt", &output);
    fixture.write(
        "bin/cargo",
        "#!/bin/sh\ncat \"$(dirname \"$0\")/cargo-output.txt\"\nexit 101\n",
    );
    fixture.executable("bin/cargo");
    fixture.write(
        ".chough/cases/cli.cases.toml",
        "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Two tests fail\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\npaths = [\"src/input.txt\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"both tests pass\"]\n\n[case.bind]\nadapter = \"cargo-test\"\ntest_target = \"cli\"\ntest = \"first\"\n",
    );
    fixture.init();
    fixture.commit("a project whose case fails");

    // The case run: the raw log stays local; the entry carries command and digest.
    let run = fixture.run(&["cases", "run", "TC-cli-0001", "--environment", "local"]);
    let ran: serde_json::Value = serde_json::from_slice(&run.stdout)
        .unwrap_or_else(|_| panic!("{}", String::from_utf8_lossy(&run.stderr)));
    let id = ran["observation"].as_str().unwrap().to_string();
    let log = format!(".chough/local/logs/{id}.log");
    let raw = std::fs::read(fixture.root().join(&log)).unwrap();
    assert!(String::from_utf8_lossy(&raw).contains("RAW-OUTPUT-ONLY"));
    let (name, text) = entries(&fixture)
        .into_iter()
        .find(|(name, _)| name == &format!("{id}.md"))
        .unwrap_or_else(|| panic!("no entry {id}"));
    let entry = front(&text);
    assert_eq!(
        items(&entry["command"]),
        [
            "cargo", "test", "--locked", "--test", "cli", "--", "--exact", "first"
        ],
        "{text}"
    );
    assert_eq!(
        entry["fingerprint"],
        format!("sha256:{}", sha256(&raw)),
        "{name}: {text}"
    );
    assert!(!text.contains("RAW-OUTPUT-ONLY"), "{text}");
    // Key lines: the first failure block and the result line, bounded and clean.
    let key = section(&body(&text), "Key failing lines");
    assert!(!key.is_empty() && key.len() <= 20, "{text}");
    assert!(
        key.iter().map(|line| line.len() + 1).sum::<usize>() <= 2_048,
        "{text}"
    );
    assert!(
        !text.contains('\x1b'),
        "an escape sequence is kept: {text:?}"
    );
    assert!(
        key.iter().any(|line| line.contains("FIRST-PANIC")),
        "the first failure block is not kept: {key:#?}"
    );
    assert!(
        key.iter().all(|line| !line.contains("SECOND-PANIC")),
        "a later failure block is kept: {key:#?}"
    );
    assert!(
        key.iter()
            .any(|line| line.contains("test result: FAILED. 1 passed; 2 failed")),
        "{key:#?}"
    );
    assert!(!text.contains("/Users/SYNTHETIC-user"), "{text}");
    assert!(text.contains("<redacted:home-path>"), "{text}");
    assert!(text.contains("1 recorded line was redacted"), "{text}");
    // No file Git tracks under .chough/ holds a raw log.
    fixture.commit("the run's records");
    let tracked = std::process::Command::new("git")
        .args(["ls-files", ".chough"])
        .current_dir(fixture.root())
        .output()
        .unwrap();
    for path in String::from_utf8_lossy(&tracked.stdout).lines() {
        assert!(!path.ends_with(".log"), "{path} is tracked");
        let bytes = std::fs::read(fixture.root().join(path)).unwrap();
        assert!(
            !String::from_utf8_lossy(&bytes).contains("RAW-OUTPUT-ONLY"),
            "{path} holds the raw log"
        );
    }

    // A manual failure's key lines: the same bounds, escapes removed, and its text
    // checked as other text is: a home path refuses, or is redacted under --redact.
    let mut lines: Vec<String> = (0..30)
        .map(|line| format!("\x1b[31mmanual line {line}\x1b[0m"))
        .collect();
    lines.push("MANUAL-LAST at /Users/SYNTHETIC-user/notes.txt".into());
    let failing = lines.join("\n");
    let before = entries(&fixture);
    let refused = fixture.run(&evidence_args(
        "SYNTHETIC manual failure",
        "failed",
        &["--failing-lines", &failing],
    ));
    assert!(
        !refused.status.success(),
        "a home path was kept or redacted silently"
    );
    assert_eq!(entries(&fixture), before, "a refused add wrote an entry");
    let added = fixture.ok(&evidence_args(
        "SYNTHETIC manual failure",
        "failed",
        &["--failing-lines", &failing, "--redact"],
    ));
    let manual = added["observation"].as_str().unwrap().to_string();
    let (_, text) = entries(&fixture)
        .into_iter()
        .find(|(name, _)| name == &format!("{manual}.md"))
        .unwrap();
    let key = section(&body(&text), "Key failing lines");
    assert!(!key.is_empty() && key.len() <= 20, "{text}");
    assert!(
        key.iter().map(|line| line.len() + 1).sum::<usize>() <= 2_048,
        "{text}"
    );
    assert!(
        !text.contains('\x1b'),
        "an escape sequence is kept: {text:?}"
    );
    assert!(
        key.iter().any(|line| line.contains("MANUAL-LAST")),
        "{text}"
    );
    assert!(text.contains("<redacted:home-path>"), "{text}");

    // An external artifact: fingerprinted in place, its path only machine-local.
    let outside = tempfile::tempdir().unwrap();
    let file = outside.path().join("SYNTHETIC-proof.bin");
    std::fs::write(&file, b"external proof bytes\n").unwrap();
    let path = file.to_str().unwrap().to_string();
    let added = fixture.ok(&evidence_args(
        "SYNTHETIC external proof",
        "passed",
        &["--artifact", &path],
    ));
    let external_id = added["observation"].as_str().unwrap().to_string();
    let (_, text) = entries(&fixture)
        .into_iter()
        .find(|(name, _)| name == &format!("{external_id}.md"))
        .unwrap();
    let entry = front(&text);
    let neutral = entry.get("external").cloned().unwrap_or_default();
    assert!(
        neutral.starts_with("ext-") && is_hex(&neutral[4..], 6),
        "{text}"
    );
    assert_eq!(
        entry.get("fingerprint").cloned().unwrap_or_default(),
        format!("sha256:{}", sha256(b"external proof bytes\n")),
        "{text}"
    );
    let outside_name = outside.path().to_str().unwrap();
    assert!(
        !text.contains(outside_name),
        "the path is committed: {text}"
    );
    let local = fixture.read(".chough/local/external-artifacts.toml");
    assert!(local.contains(&neutral) && local.contains(&path), "{local}");
    assert!(
        status_row(&fixture, &external_id)
            .get("artifact")
            .is_none_or(|state| *state != "changed")
    );

    // Refusals: missing, a directory, a symlink, 64 MiB + 1 byte; nothing recorded.
    let directory = outside.path().join("SYNTHETIC-folder");
    std::fs::create_dir(&directory).unwrap();
    let link = outside.path().join("SYNTHETIC-link.bin");
    std::os::unix::fs::symlink(&file, &link).unwrap();
    let large = outside.path().join("SYNTHETIC-large.bin");
    std::fs::File::create(&large)
        .unwrap()
        .set_len(64 * 1_048_576 + 1)
        .unwrap();
    let missing = outside.path().join("SYNTHETIC-missing.bin");
    let before_entries = entries(&fixture);
    let before_local = fixture.read(".chough/local/external-artifacts.toml");
    for refused in [&missing, &directory, &link, &large] {
        let refused = refused.to_str().unwrap();
        let output = fixture.run(&evidence_args(
            "SYNTHETIC refused proof",
            "passed",
            &["--artifact", refused],
        ));
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{refused} was accepted");
        assert!(
            stderr.contains(refused),
            "the refusal does not name {refused}: {stderr}"
        );
        assert_eq!(
            entries(&fixture),
            before_entries,
            "{refused}: an entry was written"
        );
        assert_eq!(
            fixture.read(".chough/local/external-artifacts.toml"),
            before_local,
            "{refused}: an external artifact was recorded"
        );
    }

    // Changed on this machine; elsewhere, not checkable here and neither pass nor stale.
    std::fs::write(&file, b"external proof bytes, edited\n").unwrap();
    assert_eq!(status_row(&fixture, &external_id)["artifact"], "changed");
    std::fs::write(&file, b"external proof bytes\n").unwrap();
    std::fs::remove_dir_all(fixture.root().join(".chough/local")).unwrap();
    let row = status_row(&fixture, &external_id);
    assert_eq!(row["artifact"], "external, not checkable here", "{row:#}");
    assert_eq!(row["freshness"], "current-observation", "{row:#}");
}

/// A PNG of `size` bytes with a `tEXt` and an `eXIf` chunk to strip.
fn png(size: usize) -> Vec<u8> {
    fn chunk(kind: &[u8], data: &[u8]) -> Vec<u8> {
        let mut out = (data.len() as u32).to_be_bytes().to_vec();
        out.extend_from_slice(kind);
        out.extend_from_slice(data);
        out.extend_from_slice(&[0, 0, 0, 0]);
        out
    }
    let mut bytes = b"\x89PNG\r\n\x1a\n".to_vec();
    bytes.extend(chunk(b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 0, 0, 0, 0]));
    bytes.extend(chunk(b"tEXt", b"Author\0SYNTHETIC-person"));
    bytes.extend(chunk(b"eXIf", b"SYNTHETIC-exif"));
    let fill = size.saturating_sub(bytes.len() + 24);
    bytes.extend(chunk(b"IDAT", &vec![7u8; fill]));
    bytes.extend(chunk(b"IEND", &[]));
    bytes
}

/// A JPEG with an `APP1` and a `COM` segment to strip.
fn jpeg() -> Vec<u8> {
    let mut bytes = vec![0xff, 0xd8];
    for (marker, data) in [
        (0xe1u8, &b"Exif\0\0SYNTHETIC-gps"[..]),
        (0xfe, b"SYNTHETIC comment"),
    ] {
        bytes.extend([0xff, marker]);
        bytes.extend(((data.len() + 2) as u16).to_be_bytes());
        bytes.extend_from_slice(data);
    }
    bytes.extend([0xff, 0xda, 0x00, 0x02, 1, 2, 3, 0xff, 0xd9]);
    bytes
}

/// TC-records-key-screenshots (REQ-records-013): `evidence add --screenshot` commits one
/// small key screenshot beside its entry, without its metadata, the entry naming it and
/// fingerprinting the committed bytes. A 513 KiB file, a GIF, a symlink, a sixth
/// screenshot and one that takes the session past 2 MiB are each refused with nothing
/// written, naming the remedies. At finish each screenshot moves beside the story with
/// identical bytes, the story links it, and a session with none has no folder there.
#[cfg(unix)]
#[test]
fn small_key_screenshots_are_committed_beside_the_story() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("start");
    let session = fixture.active_id();
    let outside = tempfile::tempdir().unwrap();
    let file = |name: &str, bytes: &[u8]| {
        let path = outside.path().join(name);
        std::fs::write(&path, bytes).unwrap();
        path.to_str().unwrap().to_string()
    };
    let add = |summary: &str, screenshot: &str| {
        fixture.run(&evidence_args(
            summary,
            "passed",
            &["--screenshot", screenshot],
        ))
    };
    // A 500 KiB PNG with metadata, committed beside its entry without it.
    let large = png(500 * 1024);
    let output = add("SYNTHETIC screen one", &file("one.png", &large));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let first: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let first = first["observation"].as_str().unwrap().to_string();
    let beside = format!(".chough/sessions/{session}/evidence/{first}.png");
    let committed = std::fs::read(fixture.root().join(&beside)).unwrap();
    for chunk in [&b"tEXt"[..], b"eXIf", b"SYNTHETIC-person"] {
        assert!(!committed.windows(chunk.len()).any(|window| window == chunk));
    }
    let (_, text) = entries(&fixture)
        .into_iter()
        .find(|(name, _)| name == &format!("{first}.md"))
        .unwrap();
    assert_eq!(front(&text)["screenshot"], format!("{first}.png"), "{text}");
    assert_eq!(
        front(&text).get("fingerprint").cloned().unwrap_or_default(),
        format!("sha256:{}", sha256(&committed)),
        "{text}"
    );
    // Three more of 500 KiB: four kept, 2,000 KiB in all.
    for name in ["two", "three", "four"] {
        let output = add(
            &format!("SYNTHETIC screen {name}"),
            &file(&format!("{name}.png"), &large),
        );
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    // Refused with nothing written, each naming the remedies.
    let gif = file("five.gif", b"GIF89a\x01\x00\x01\x00\x00\x00\x00;");
    let target = file("target.png", &png(1_000));
    let link = outside.path().join("link.png");
    std::os::unix::fs::symlink(&target, &link).unwrap();
    let refusals = [
        ("a 513 KiB file", file("over.png", &png(513 * 1024))),
        ("a GIF", gif),
        ("a symlink", link.to_str().unwrap().to_string()),
        ("past 2 MiB in all", file("total.png", &png(100 * 1024))),
    ];
    for (what, path) in refusals {
        let before = bytes_under(fixture.root(), ".chough");
        let output = add("SYNTHETIC refused screen", &path);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!output.status.success(), "{what} was accepted");
        assert!(stderr.contains("--artifact"), "{what}: no remedy: {stderr}");
        assert_eq!(
            bytes_under(fixture.root(), ".chough"),
            before,
            "{what}: something was written"
        );
    }
    // A small JPEG is the fifth, and keeps no APP1 or COM segment.
    let output = add("SYNTHETIC screen five", &file("five.jpg", &jpeg()));
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let fifth: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let fifth = fifth["observation"].as_str().unwrap().to_string();
    let committed = std::fs::read(
        fixture
            .root()
            .join(format!(".chough/sessions/{session}/evidence/{fifth}.jpg")),
    )
    .unwrap();
    for marker in [[0xff, 0xe1], [0xff, 0xfe]] {
        assert!(!committed.windows(2).any(|window| window == marker));
    }
    // A sixth is refused.
    let before = bytes_under(fixture.root(), ".chough");
    let output = add("SYNTHETIC screen six", &file("sixth.png", &png(1_000)));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "a sixth screenshot was accepted");
    assert!(
        stderr.contains("--artifact") && stderr.contains("crop or compress"),
        "{stderr}"
    );
    assert_eq!(
        bytes_under(fixture.root(), ".chough"),
        before,
        "a sixth screenshot wrote something"
    );

    // At finish the screenshots move beside the story, identical, and the story links
    // them; a session with no screenshot has no folder beside its story.
    let pngs: Vec<(String, Vec<u8>)> = bytes_under(
        fixture.root(),
        &format!(".chough/sessions/{session}/evidence"),
    )
    .into_iter()
    .filter(|(path, _)| path.ends_with(".png") || path.ends_with(".jpg"))
    .collect();
    assert_eq!(pngs.len(), 5);
    fixture.ok(&[
        "session",
        "new",
        "plain",
        "--objective",
        "SYNTHETIC no screenshots",
    ]);
    let plain = fixture.id_of("plain");
    fixture.commit("the screenshots");
    fixture.ok(&[
        "session",
        "close",
        &session,
        "--reason",
        "SYNTHETIC finished",
    ]);
    fixture.ok(&["session", "close", &plain, "--reason", "SYNTHETIC finished"]);
    let story = fixture.read(&format!(".chough/archive/sessions/{session}.md"));
    for (path, bytes) in &pngs {
        let name = path.rsplit('/').next().unwrap();
        let moved = format!(".chough/archive/sessions/{session}/{name}");
        assert_eq!(
            &std::fs::read(fixture.root().join(&moved)).unwrap(),
            bytes,
            "{moved}"
        );
        assert!(
            section(&body(&story), "What was verified")
                .iter()
                .any(|line| line.contains(&format!("{session}/{name}"))),
            "the story does not link {name}: {story}"
        );
    }
    assert!(fixture.exists(&format!(".chough/archive/sessions/{plain}.md")));
    assert!(!fixture.exists(&format!(".chough/archive/sessions/{plain}")));
}

/// Today's UTC date, `YYYY-MM-DD`.
fn today() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let days = (seconds / 86_400) as i64 + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted + 2) / 5 + 1;
    let month = if shifted < 10 {
        shifted + 3
    } else {
        shifted - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

/// TC-records-date-slug-names-and-aliases (REQ-records-003). Every new record is named
/// `<UTC date>-<slug of its source>` and gives that name as `id`: a session from its key,
/// an evidence entry from its claim, a bug from its title, an inbox entry from its text
/// and a journal post from its title, never from a count. Two bugs with one title on one
/// day get the name, then the name with `-` and 6 hex digits. A migrated bug's old ID
/// is an alias by which it is shown, updated and linked, and an alias two bugs hold is
/// refused by `bug show`. Proved elsewhere: the slug rules, the 80-byte bound and a
/// refused second clash with no second draw (src/identity.rs's unit tests); a session's
/// `OBS-` reference resolving to the migrated entry (tests/migrate_evidence.rs); `chough
/// doctor` reporting a shared alias (tests/doctor.rs); merges of branches that add
/// records (tests/bugs.rs, tests/sessions.rs, tests/cases.rs).
#[cfg(unix)]
#[test]
fn names_are_date_and_slug_with_aliases() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.commit("start");
    let today = today();
    let created = fixture.ok(&[
        "session",
        "new",
        "Release_Notes-2",
        "--objective",
        "SYNTHETIC session",
    ]);
    let session = format!("{today}-release-notes-2");
    assert_eq!(created["id"], session.as_str(), "{created:#}");
    let text = fixture.read(&format!(".chough/sessions/{session}/session.md"));
    assert_eq!(front(&text)["id"], session, "{text}");
    assert_eq!(
        items(&front(&text)["aliases"]),
        ["Release_Notes-2"],
        "{text}"
    );

    let added = fixture.ok(&evidence_args("SYNTHETIC Proof: one, two!", "passed", &[]));
    let entry = format!("{today}-synthetic-proof-one-two");
    assert_eq!(added["observation"], entry.as_str(), "{added:#}");
    let text = fixture.read(&format!(".chough/sessions/{session}/evidence/{entry}.md"));
    assert_eq!(front(&text)["id"], entry, "{text}");

    let bug = |title: &str, extra: &[&str]| {
        let mut args = vec![
            "bug",
            "add",
            "--title",
            title,
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
        ];
        args.extend_from_slice(extra);
        fixture.ok(&args)["id"].as_str().unwrap().to_string()
    };
    let first = bug("  Fix   LOGIN... now!  ", &[]);
    assert_eq!(first, format!("{today}-fix-login-now"));
    let twin = bug(
        "  Fix   LOGIN... now!  ",
        &["--distinct", "SYNTHETIC a second defect"],
    );
    let suffix = twin
        .strip_prefix(&format!("{first}-"))
        .unwrap_or_else(|| panic!("{twin}"));
    assert!(is_hex(suffix, 6), "{twin}");
    for name in [&first, &twin] {
        let text = fixture.read(&format!(".chough/bugs/{name}.md"));
        assert_eq!(&front(&text)["id"], name, "{text}");
    }
    // Names come from sources, never from a count of records.
    for title in ["alpha", "beta", "gamma"] {
        let distinct = format!("SYNTHETIC a separate defect, {title}");
        assert_eq!(
            bug(title, &["--distinct", &distinct]),
            format!("{today}-{title}")
        );
    }

    let inbox = fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &session,
        "--from",
        "subagent-1",
        "--kind",
        "decision",
        "--text",
        "Use the SMALLER fixture!",
    ]);
    assert_eq!(inbox["id"], format!("{today}-use-the-smaller-fixture"));
    let post = fixture.ok(&[
        "journal",
        "add",
        "--kind",
        "learned",
        "--title",
        "Week One: done",
        "--body",
        "SYNTHETIC body",
    ]);
    assert_eq!(
        post["post"],
        format!("knowledge/journal/{today}-week-one-done.md")
    );

    // A migrated bug: shown, updated and linked by its old ID and by its name.
    let migrated = "2026-11-10-a-migrated-bug";
    fixture.bug_md(migrated, "in-scope", "open", &[]);
    let path = format!(".chough/bugs/{migrated}.md");
    fixture.write(
        &path,
        &fixture
            .read(&path)
            .replacen("aliases: []", "aliases: [BUG-0012]", 1),
    );
    let by_alias = fixture.ok(&["bug", "show", "BUG-0012"]);
    let by_name = fixture.ok(&["bug", "show", migrated]);
    assert_eq!(by_alias, by_name);
    assert_eq!(by_alias["bug"]["id"], migrated, "{by_alias:#}");
    fixture.ok(&["bug", "update", "BUG-0012", "--note", "SYNTHETIC note"]);
    assert!(fixture.read(&path).contains("SYNTHETIC note"));
    // Linked by the old ID: the checkpoint resolves it, as it refuses an unknown one.
    fixture.ok(&["checkpoint", "--bug", "BUG-0012"]);
    let linked = front(&fixture.read(&fixture.session_file()))["bugs"].clone();
    assert!(
        items(&linked).iter().any(|bug| bug == "BUG-0012"),
        "{linked}"
    );
    // An alias two bugs hold is refused by bug show.
    let other = "2026-11-10-another-migrated-bug";
    fixture.bug_md(other, "in-scope", "open", &[]);
    let other_path = format!(".chough/bugs/{other}.md");
    fixture.write(
        &other_path,
        &fixture
            .read(&other_path)
            .replacen("aliases: []", "aliases: [BUG-0012]", 1),
    );
    let refused = fixture.fails(&["bug", "show", "BUG-0012"]);
    assert!(
        refused.contains(migrated) && refused.contains(other),
        "{refused}"
    );
    // A link resolves its ID, so an ID no record holds is refused.
    let unknown = fixture.run(&["checkpoint", "--bug", "BUG-9999"]);
    assert!(!unknown.status.success(), "an unknown bug was linked");
}

/// TC-records-session-story-sections (REQ-records-004). `session new` writes a
/// `session.md` whose front matter is the checkpoint, every listed key in it, and whose
/// body is the story in seven parts in order, its Goal the objective. A decision, a
/// completed item, an open question and an evidence reference each appear once, in
/// their section, and `next` shows the decision and the question from a fresh process.
/// Resolving the question keeps its line and `next` no longer lists it. A person's
/// rewrite of the Goal is kept after a decision and a stage move.
#[cfg(unix)]
#[test]
fn the_session_record_is_checkpoint_and_story() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.commit("start");
    fixture.ok(&[
        "session",
        "new",
        "story",
        "--objective",
        "Ship the SYNTHETIC parser",
    ]);
    let file = fixture.session_file();
    let text = fixture.read(&file);
    let keys = front_keys(&text);
    for key in [
        "kind",
        "id",
        "aliases",
        "objective",
        "intent_scope",
        "modules",
        "stage",
        "stage_status",
        "iteration",
        "callers",
        "native_change",
        "pr",
        "criteria",
        "requirements",
        "cases",
        "design",
        "files",
        "observations",
        "bugs",
        "blockers",
        "waiting_on_human",
        "run_consents",
        "next",
    ] {
        assert!(keys.iter().any(|known| known == key), "no {key}: {text}");
    }
    assert_eq!(front(&text)["kind"], "session");
    let story = body(&text);
    let parts = [
        "# Ship the SYNTHETIC parser\n",
        "\n## Goal\n",
        "\n## Where it stands\n",
        "\n## Decisions and why\n",
        "\n## What was verified\n",
        "\n## Open threads\n",
        "\n## Timeline\n",
    ];
    let mut at = 0;
    for part in parts {
        let found = story[at..]
            .find(part)
            .unwrap_or_else(|| panic!("{part:?} missing or out of order: {story}"));
        at += found + part.len();
    }
    assert!(
        section(&story, "Goal")
            .iter()
            .any(|line| line.contains("Ship the SYNTHETIC parser")),
        "{story}"
    );

    let decision = "Use TOML because lists are first-class";
    let question = "Which viewport matters most?";
    let completed = "Wrote the SYNTHETIC parser";
    let added = fixture.ok(&evidence_args("SYNTHETIC parser proof", "passed", &[]));
    let entry = added["observation"].as_str().unwrap().to_string();
    fixture.ok(&[
        "checkpoint",
        "--decision",
        decision,
        "--question",
        question,
        "--completed",
        completed,
        "--observation",
        &entry,
    ]);
    let story = body(&fixture.read(&file));
    for (heading, text) in [
        ("Decisions and why", decision),
        ("Where it stands", completed),
        ("Open threads", question),
        ("What was verified", entry.as_str()),
    ] {
        let lines = section(&story, heading);
        assert_eq!(
            lines.iter().filter(|line| line.contains(text)).count(),
            1,
            "{heading}: {lines:#?}"
        );
        assert_eq!(story.matches(text).count(), 1, "{text} repeated: {story}");
    }
    assert!(
        section(&story, "What was verified")
            .iter()
            .any(|line| line.contains(&format!("evidence/{entry}.md"))),
        "the reference does not link its file: {story}"
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["decisions"]["items"][0]["text"], decision, "{next:#}");
    assert_eq!(next["open_questions"]["items"][0], question, "{next:#}");

    fixture.ok(&[
        "checkpoint",
        "--resolve-question",
        question,
        "--answer",
        "The SYNTHETIC phone viewport",
    ]);
    let story = body(&fixture.read(&file));
    assert!(
        section(&story, "Open threads")
            .iter()
            .any(|line| line.contains(&format!(
                "{question} (resolved: The SYNTHETIC phone viewport)"
            ))),
        "the resolved question's line does not keep its answer: {story}"
    );
    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(next["open_questions"]["total"], 0, "{next:#}");

    // A person's Goal survives a decision and a stage move.
    let text = fixture.read(&file);
    let rewritten = text.replacen(
        "## Goal\n",
        "## Goal\n\nSYNTHETIC person: what we really want is a parser people trust.\n",
        1,
    );
    fixture.write(&file, &rewritten);
    fixture.ok(&[
        "checkpoint",
        "--decision",
        "Keep the grammar small, because it is read by people",
    ]);
    fixture.ok(&["stage", "advance", "--json"]);
    assert!(
        fixture
            .read(&file)
            .contains("SYNTHETIC person: what we really want is a parser people trust."),
        "the Goal rewrite was lost"
    );
}

/// TC-records-only-committed-bytes-leave (REQ-records-012). With 40 archived stories of
/// every age committed, a move into done, `session new`, `doctor --fix` and `library
/// index` leave all 40 byte-identical in place, and nothing removes a file whose bytes
/// HEAD does not hold: a finished session keeps its uncommitted evidence entry and
/// reports it. Proved elsewhere: an uncommitted `agent/` file kept until a commit and
/// `doctor --fix` (tests/sessions.rs
/// `an_uncommitted_file_keeps_the_folder_until_a_commit_and_doctor_fix`); `migrate
/// --apply` refusing a changed 0.5.x file and removing it once committed (tests/migrate.rs
/// `migrate_previews_applies_and_keeps_every_byte_in_git`).
#[cfg(unix)]
#[test]
fn nothing_leaves_the_tree_unless_head_holds_it() {
    let fixture = Fixture::new();
    fixture.stubs();
    for index in 0..40 {
        let key = format!("earlier-{index}");
        fixture.ok(&[
            "session",
            "new",
            &key,
            "--objective",
            &format!("SYNTHETIC earlier objective {index}"),
        ]);
        fixture.ok(&[
            "session",
            "close",
            &key,
            "--reason",
            "SYNTHETIC earlier work",
        ]);
    }
    // Every age: closed dates from years ago to today.
    for index in 0..40 {
        let story = fixture.story_of(&format!("earlier-{index}"));
        let text = fixture.read(&story);
        let year = 2016 + index / 4;
        let closed = front(&text)["closed"].clone();
        fixture.write(
            &story,
            &text.replacen(
                &format!("closed: {closed}"),
                &format!("closed: {year}-0{}-1{}", 1 + index % 9, index % 10),
                1,
            ),
        );
    }
    fixture.ok(&[
        "session",
        "new",
        "current",
        "--objective",
        "SYNTHETIC current work",
    ]);
    fixture.commit("forty earlier stories");
    let stories = bytes_under(fixture.root(), ".chough/archive/sessions");
    assert_eq!(
        stories.keys().filter(|path| path.ends_with(".md")).count(),
        40
    );

    // An entry recorded but not committed when the session finishes.
    let added = fixture.ok(&evidence_args("SYNTHETIC uncommitted proof", "passed", &[]));
    let entry = added["observation"].as_str().unwrap().to_string();
    let current = fixture.active_id();
    let uncommitted = format!(".chough/sessions/{current}/evidence/{entry}.md");
    let entry_bytes = fixture.read(&uncommitted);
    for _ in 0..10 {
        if fixture.checkpoint()["stage"]["current"] == "accept" {
            break;
        }
        fixture.ok(&["stage", "advance", "--json"]);
    }
    let moved = fixture.ok(&["stage", "advance", "--json"]);
    assert_eq!(
        moved["applied"]["session_archive"]["status"], "archived",
        "{moved:#}"
    );
    assert_eq!(
        fixture.read(&uncommitted),
        entry_bytes,
        "an uncommitted entry left its place"
    );
    assert!(
        moved.to_string().contains(&uncommitted),
        "the kept file is not reported: {moved:#}"
    );
    fixture.ok(&[
        "session",
        "new",
        "after",
        "--objective",
        "SYNTHETIC later work",
    ]);
    let _ = fixture.run(&["doctor", "--fix"]);
    fixture.ok(&["library", "index"]);
    let now = bytes_under(fixture.root(), ".chough/archive/sessions");
    for (path, bytes) in &stories {
        assert_eq!(
            now.get(path),
            Some(bytes),
            "{path} changed or left its place"
        );
    }
}
