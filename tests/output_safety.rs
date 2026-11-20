//! REQ-records-016 and REQ-records-006 (#366, #367): three black-box sweeps.
//!
//! - The write sweep: a control character in each field a person gives is refused at
//!   write as `refused:control-character`, naming the field and the code point, with
//!   every file unchanged, under `--redact` too.
//! - The print sweep: records holding control characters by hand, as an earlier release
//!   could write them, and every command in `chough --help`'s tree run over them, in text
//!   and JSON: no output holds a control character raw, and each JSON output parses.
//! - The echo sweep: every value a command takes, flag or argument, given a token and an
//!   escape sequence, shows neither in any output or error.
//!
//! Both command sweeps walk the help output, so a command added later is in them without
//! editing this file.

mod common;

use common::{ACTOR_VARIABLES, Fixture, USER_CONFIG};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Each control character the write sweep gives, as its code point and the text given.
const CONTROLS: [(&str, &str); 6] = [
    ("U+001B", "\u{1b}[31m"),
    ("U+0007", "\u{7}"),
    ("U+007F", "\u{7f}"),
    ("U+009B", "\u{9b}31m"),
    ("U+202E", "\u{202e}"),
    ("U+2066", "\u{2066}"),
];

/// A token form the leak guard knows, built so this file holds none.
const TOKEN: &str = concat!("ghp_", "SYNTHETICECHOPROBE", "000000000000000000");

/// A control character as REQ-records-016 defines it.
fn control(character: char) -> bool {
    matches!(
        character,
        '\u{0}'..='\u{1f}' | '\u{7f}'..='\u{9f}' | '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}'
    )
}

/// The code points of the control characters `text` holds raw, a line feed aside.
fn raw(text: &str) -> Vec<String> {
    let mut held: Vec<String> = text
        .chars()
        .filter(|character| *character != '\n' && control(*character))
        .map(|character| format!("U+{:04X}", character as u32))
        .collect();
    held.sort();
    held.dedup();
    held
}

/// Every file under the fixture outside `.git` and the Git-ignored `.chough/local/`.
fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, dir: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(dir).unwrap().flatten() {
            let path = entry.path();
            let name = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if name == ".git" || name.starts_with(".chough/local") {
                continue;
            }
            if path.is_dir() {
                walk(base, &path, out);
            } else {
                out.insert(name, fs::read(&path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn said(output: &std::process::Output) -> String {
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

fn evidence_add<'a>(summary: &'a str, environment: &'a str, limitation: &'a str) -> Vec<&'a str> {
    vec![
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        environment,
        "--summary",
        summary,
        "--limitation",
        limitation,
    ]
}

fn bug_add(title: &str) -> Vec<&str> {
    vec![
        "bug",
        "add",
        "--title",
        title,
        "--symptom",
        "s",
        "--trigger",
        "t",
        "--expected",
        "e",
        "--actual",
        "a",
        "--environment",
        "local",
        "--scope",
        "out-of-scope",
    ]
}

/// REQ-records-016, TC-records-a-control-character-is-refused-at-write: ESC, BEL, DEL,
/// U+009B, U+202E and U+2066 each in an objective, a decision, a `--no-native-change`
/// reason, a `--pr`, an evidence summary, environment and limitation, a bug title, an
/// inbox entry and relayed `--words` are refused as `refused:control-character` naming
/// the field and the code point; the output holds no control character, and every file
/// is unchanged, with `--redact` as well.
#[test]
fn a_control_character_is_refused_at_write_in_every_field() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.hi_stand_in();
    fixture.init();
    fixture.ok(&["checkpoint", "--awaiting-human", "SYNTHETIC gate"]);
    fixture.commit("fixture");
    let session = fixture.active_id();
    let mut problems: Vec<String> = vec![];
    for (point, character) in CONTROLS {
        let value = format!("probe {character} value");
        let words = vec![
            "checkpoint",
            "--close-gate",
            "SYNTHETIC gate",
            "--decided-by",
            "SYNTHETIC-human",
            "--words",
            &value,
            "--where",
            "a test",
            "--recorded-by",
            "SYNTHETIC-agent",
            "--no-native",
        ];
        let inbox = vec![
            "inbox",
            "add",
            "--session",
            &session,
            "--from",
            "SYNTHETIC-agent",
            "--kind",
            "question",
            "--text",
            &value,
        ];
        let writes: Vec<(Vec<&str>, &str)> = vec![
            (vec!["checkpoint", "--objective", &value], "objective"),
            (vec!["checkpoint", "--decision", &value], "decision"),
            (
                vec!["checkpoint", "--no-native-change", &value],
                "no_native_change",
            ),
            (vec!["checkpoint", "--pr", &value], "pr"),
            (evidence_add(&value, "local", "none"), "claim"),
            (evidence_add("probe", &value, "none"), "environment"),
            (evidence_add("probe", "local", &value), "limitation"),
            (bug_add(&value), "title"),
            (inbox, "--text"),
            (words, "--words"),
        ];
        for (args, field) in &writes {
            for redact in [false, true] {
                let mut all: Vec<&str> = if redact { vec!["--redact"] } else { vec![] };
                all.extend(args.iter().copied());
                let before = tree(fixture.root());
                let output = fixture.run(&all);
                let said = said(&output);
                let after = tree(fixture.root());
                let mut wrong = vec![];
                if output.status.success() {
                    wrong.push("it was written".to_string());
                }
                if !(said.contains("refused:control-character") && said.contains(point)) {
                    wrong.push(format!("not refused as a control character naming {point}"));
                }
                if !said.contains(field) {
                    wrong.push(format!("does not name the field {field}"));
                }
                if !raw(&said).is_empty() {
                    wrong.push(format!("printed {:?}", raw(&said)));
                }
                if said.contains("probe") {
                    wrong.push("echoed the value".to_string());
                }
                if after != before {
                    wrong.push("changed a file".to_string());
                    // Put the tree back, so the next write starts from the same files.
                    for name in after.keys().filter(|name| !before.contains_key(*name)) {
                        fs::remove_file(fixture.root().join(name)).ok();
                    }
                    for (name, bytes) in &before {
                        if after.get(name) != Some(bytes) {
                            fs::write(fixture.root().join(name), bytes).unwrap();
                        }
                    }
                }
                if !wrong.is_empty() {
                    problems.push(format!("{all:?}: {}", wrong.join("; ")));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// REQ-records-016, TC-records-a-line-feed-is-kept-where-a-field-takes-one: a decision
/// and an evidence summary holding a line feed are written, while one holding a tab is
/// refused naming U+0009; a CRLF line break is stored as a line feed; `--title` and
/// `--pr` holding a line feed are refused; a tab already stored is printed as `\u0009`;
/// a `--no-native-change` reason holding a line feed shows on `next`'s line as `\n`.
#[test]
fn a_line_feed_is_kept_where_a_field_takes_one_and_a_tab_is_refused() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.hi_stand_in();
    fixture.init();
    fixture.commit("fixture");
    fixture.ok(&["checkpoint", "--decision", "first line\nsecond line"]);
    fixture.ok(&evidence_add("seen once\nseen twice", "local", "none"));

    let tab = fixture.fails(&["checkpoint", "--decision", "a\tb"]);
    assert!(
        tab.contains("refused:control-character") && tab.contains("U+0009"),
        "{tab}"
    );
    // A reason keeps its line breaks, so a Windows one shows how it is stored.
    fixture.ok(&["checkpoint", "--no-native-change", "crlf one\r\ncrlf two"]);
    let stored = tree(fixture.root());
    assert!(
        !stored.values().any(|bytes| bytes.contains(&b'\r')),
        "a carriage return was stored"
    );
    assert_eq!(
        fixture.checkpoint()["no_native_change"],
        "crlf one\ncrlf two"
    );
    fixture.fails(&["checkpoint", "--title", "a\nb"]);
    let pr = fixture.fails(&["checkpoint", "--pr", "48\n17"]);
    assert!(
        pr.contains("refused:control-character") && pr.contains("U+000A"),
        "{pr}"
    );

    // A tab already stored, as a hand edit or an earlier release left it.
    let file = fixture.session_file();
    let text = fixture.read(&file).replace("first line", "tab\there");
    fixture.write(&file, &text);
    let next = fixture.ok_text(&["next"]);
    assert!(next.contains("tab\\u0009here"), "{next}");
    assert!(raw(&next).is_empty(), "{:?}", raw(&next));

    fixture.ok(&["checkpoint", "--no-native-change", "docs only\nsecond line"]);
    let next = fixture.ok_text(&["next"]);
    assert!(
        next.lines()
            .any(|line| line.contains("docs only\\nsecond line")),
        "{next}"
    );
}

// MARK: - The command sweeps

/// One leaf command of the help tree: its path, its positional slots and the long
/// options that take a value.
struct Leaf {
    path: Vec<String>,
    positionals: Vec<String>,
    /// The options the usage line requires, each with its value's name.
    required: Vec<(String, String)>,
    options: Vec<String>,
    json: bool,
}

fn help_text(path: &[String]) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_chough"))
        .args(path)
        .arg("--help")
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// Every leaf command `chough --help` lists, from the top down.
fn leaves() -> Vec<Leaf> {
    let mut out = vec![];
    let mut pending: Vec<Vec<String>> = vec![vec![]];
    while let Some(path) = pending.pop() {
        let text = help_text(&path);
        let mut subcommands = vec![];
        let mut in_commands = false;
        let mut options = vec![];
        let mut json = false;
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
                let after = &rest[name.len()..];
                if name == "json" {
                    json = true;
                }
                if after.starts_with(" <") && name != "help" {
                    options.push(format!("--{name}"));
                }
            }
        }
        if subcommands.is_empty() {
            let usage = text
                .lines()
                .find(|line| line.starts_with("Usage:"))
                .unwrap_or_default();
            // `Usage: chough <path> [OPTIONS] --title <TITLE> <ID>`: a slot is a `<NAME>`
            // or `[NAME]` that is not a required option's value.
            let words: Vec<&str> = usage.split_whitespace().skip(2 + path.len()).collect();
            let mut positionals = vec![];
            let mut required = vec![];
            for (at, word) in words.iter().enumerate() {
                let after_flag = at > 0 && words[at - 1].starts_with("--");
                let name = word.trim_matches(|c| "<>[].".contains(c)).to_string();
                if (word.starts_with('<') || word.starts_with('['))
                    && !word.contains("OPTIONS")
                    && !after_flag
                {
                    positionals.push(name);
                } else if after_flag && word.starts_with('<') {
                    required.push((words[at - 1].to_string(), name));
                }
            }
            options.sort();
            options.dedup();
            out.push(Leaf {
                path,
                positionals,
                required,
                options,
                json,
            });
        } else {
            for subcommand in subcommands {
                let mut child = path.clone();
                child.push(subcommand);
                pending.push(child);
            }
        }
    }
    assert!(
        out.len() > 40,
        "the help tree lists only {} commands",
        out.len()
    );
    out
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap().flatten() {
        let source = entry.path();
        let target = to.join(entry.file_name());
        let kind = entry.file_type().unwrap();
        if kind.is_dir() {
            copy_tree(&source, &target);
        } else if kind.is_symlink() {
            #[cfg(unix)]
            std::os::unix::fs::symlink(fs::read_link(&source).unwrap(), &target).unwrap();
        } else {
            fs::copy(&source, &target).unwrap();
        }
    }
}

/// Run `args` in `root` as the fixture does, with this run's own home and no standard
/// input, killed after a minute; returns whether it succeeded, standard output and both
/// streams.
fn sweep_run(root: &Path, args: &[&str]) -> (bool, String, String) {
    let home = root.join(".sweep-home");
    fs::create_dir_all(&home).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command
        .arg("--root")
        .arg(root)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for name in ACTOR_VARIABLES {
        command.env_remove(name);
    }
    command
        .env("HOME", &home)
        .env("XDG_CONFIG_HOME", root.join(USER_CONFIG))
        .env("XDG_DATA_HOME", home.join("data"))
        .env("XDG_STATE_HOME", home.join("state"))
        .env("XDG_CACHE_HOME", home.join("cache"));
    let mut child = command.spawn().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let mut stderr = child.stderr.take().unwrap();
    let out_reader = std::thread::spawn(move || {
        let mut text = vec![];
        stdout.read_to_end(&mut text).ok();
        text
    });
    let err_reader = std::thread::spawn(move || {
        let mut text = vec![];
        stderr.read_to_end(&mut text).ok();
        text
    });
    let deadline = Instant::now() + Duration::from_secs(60);
    let status = loop {
        if let Some(status) = child.try_wait().unwrap() {
            break status;
        }
        if Instant::now() >= deadline {
            child.kill().ok();
            panic!("{args:?} ran over a minute");
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let stdout = String::from_utf8_lossy(&out_reader.join().unwrap()).into_owned();
    let stderr = String::from_utf8_lossy(&err_reader.join().unwrap()).into_owned();
    let both = format!("{stdout}{stderr}");
    (status.success(), stdout, both)
}

/// A clean value for slot `name`, from the fixture's own IDs where the slot takes one.
fn filler(name: &str, ids: &BTreeMap<&str, String>) -> String {
    let key = match name {
        "KEY" | "SESSION" => "session",
        "ID" | "CASE" => "id",
        "ENTRY" => "entry",
        "BASE" | "OURS" | "THEIRS" => "file",
        "LIST" => "list",
        _ => "",
    };
    ids.get(key).cloned().unwrap_or_else(|| "probe".to_string())
}

/// The arguments to run `leaf` with: the leaf's path and a clean value in every slot,
/// with `id` for the `ID` slot.
fn invocations(leaf: &Leaf, ids: &BTreeMap<&str, String>) -> Vec<Vec<String>> {
    let id_choices: Vec<String> = match leaf.path.first().map(String::as_str) {
        Some("bug") => vec![ids["bug"].clone()],
        Some("cases") => vec![ids["case"].clone()],
        Some("validate") => vec![ids["case"].clone(), "probe".into()],
        _ => vec![ids["session"].clone()],
    };
    let mut out = vec![];
    for id in &id_choices {
        let mut args = leaf.path.clone();
        for slot in &leaf.positionals {
            if slot == "ID" || slot == "CASE" {
                args.push(id.clone());
            } else {
                args.push(filler(slot, ids));
            }
        }
        for (flag, name) in &leaf.required {
            args.push(flag.clone());
            args.push(filler(name, ids));
        }
        out.push(args.clone());
        if leaf.json {
            let mut json = args.clone();
            json.push("--json".into());
            out.push(json);
        }
    }
    out
}

/// Text holding every control character a record can hold raw in its front matter or
/// body: no tab, which the front matter subset does not hold, and no backslash.
const DIRTY: &str = "X\u{1b}[31m\u{7}\u{7f}\u{9b}31m\u{202e}\u{2066}\u{1b}]8;;https://example.com\u{7}link\u{1b}]8;;\u{7}Y";

/// The same characters as TOML escapes, which a case definition holds them as.
const DIRTY_TOML: &str = "X\\u001b[31m\\u0007\\u007f\\u009b31m\\u202e\\u2066Y";

const CASES: &str = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe PLANTTOML\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe prints ok PLANTTOML\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo ok\"]\nexpect_output = [\"ok\"]\n";

/// A project whose records hold `DIRTY` in every kind: a session (objective, decision,
/// gate, blocker, no-native-change reason), an archived story and its journal post, an
/// evidence entry, a bug, an inbox entry and a case definition; returns it and its IDs.
fn dirty_project() -> (Fixture, BTreeMap<&'static str, String>) {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.hi_stand_in();
    fixture.write(".chough/cases/cli.cases.toml", CASES);
    fixture.init();
    fixture.commit("fixture");
    let first = fixture.active_id();
    // An archived story and journal post first.
    fixture.ok(&[
        "session",
        "new",
        "beta",
        "--objective",
        "Beta PLANT objective",
    ]);
    fixture.ok(&["checkpoint", "--decision", "Beta PLANT decision"]);
    for _ in 0..10 {
        if fixture
            .run(&["stage", "advance", "--json"])
            .status
            .success()
        {
            continue;
        }
        break;
    }
    fixture.ok(&["session", "use", &first]);
    fixture.ok(&["checkpoint", "--objective", "Probe PLANT objective"]);
    fixture.ok(&["checkpoint", "--decision", "PLANT decision"]);
    fixture.ok(&["checkpoint", "--blocker", "PLANT blocker"]);
    fixture.ok(&["checkpoint", "--awaiting-human", "PLANT gate"]);
    fixture.ok(&["checkpoint", "--no-native-change", "docs only PLANT reason"]);
    let entry = fixture.ok(&evidence_add("PLANT summary", "local", "PLANT limitation"));
    let bug = fixture.ok(&bug_add("PLANT bug"));
    let session = fixture.active_id();
    fixture.ok(&[
        "inbox",
        "add",
        "--session",
        &session,
        "--from",
        "SYNTHETIC-agent",
        "--kind",
        "question",
        "--text",
        "PLANT question",
    ]);
    for (path, bytes) in tree(fixture.root()) {
        let text = String::from_utf8_lossy(&bytes);
        if path.ends_with(".md") && text.contains("PLANT") {
            fixture.write(&path, &text.replace("PLANT", DIRTY));
        } else if path.ends_with(".toml") && text.contains("PLANTTOML") {
            fixture.write(&path, &text.replace("PLANTTOML", DIRTY_TOML));
        }
    }
    fixture.commit("records holding control characters");
    let mut ids = BTreeMap::new();
    ids.insert("session", session);
    ids.insert(
        "bug",
        bug["id"]
            .as_str()
            .or(bug["bug"]["id"].as_str())
            .unwrap_or("probe")
            .to_string(),
    );
    ids.insert(
        "entry",
        entry["id"]
            .as_str()
            .or(entry["observation_id"].as_str())
            .unwrap_or("probe")
            .to_string(),
    );
    ids.insert("case", "TC-cli-0001".to_string());
    ids.insert(
        "file",
        format!(".chough/sessions/{}/session.md", ids["session"]),
    );
    (fixture, ids)
}

/// REQ-records-016, TC-records-no-command-prints-a-stored-control-character-raw: every
/// command in `chough --help`'s tree, run over a project whose records hold each control
/// character, in text and JSON where it has both and with the project's IDs where it
/// takes one, prints none raw, and each JSON output parses.
#[test]
fn no_command_prints_a_stored_control_character_raw() {
    let (fixture, ids) = dirty_project();
    let scratch = tempfile::tempdir().unwrap();
    let mut problems = vec![];
    let mut runs = 0;
    for (index, leaf) in leaves().iter().enumerate() {
        let copy = scratch.path().join(format!("p{index}"));
        copy_tree(fixture.root(), &copy);
        for args in invocations(leaf, &ids) {
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let (ok, stdout, both) = sweep_run(&copy, &args);
            runs += 1;
            let held = raw(&both);
            if !held.is_empty() {
                problems.push(format!("{args:?} printed {held:?}"));
            }
            let trimmed = stdout.trim_start();
            if ok && (trimmed.starts_with('{') || trimmed.starts_with('[')) {
                let parses = serde_json::from_str::<Value>(trimmed).is_ok()
                    || trimmed.lines().all(|line| {
                        line.trim().is_empty() || serde_json::from_str::<Value>(line).is_ok()
                    });
                if !parses {
                    problems.push(format!("{args:?} printed JSON that does not parse"));
                }
            }
        }
    }
    assert!(runs > 60, "only {runs} runs");
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

/// The `src/change.rs` sites #412 makes stop echoing (the description SpecSync repeats,
/// an answer, and `change ship`'s `--close-gate` text): not this change's (#367 closes
/// when both are merged). #412 removes this list.
const FOR_412: [(&str, &str); 3] = [
    ("change new", "DESCRIPTION"),
    ("change answer", "ANSWER"),
    ("change ship", "--close-gate"),
];

/// REQ-records-006, TC-records-no-command-echoes-a-given-value: every value slot of every
/// command in `chough --help`'s tree, each option that takes a value and each argument,
/// given a token and an escape sequence, prints neither, in any output or error (#367).
#[test]
fn no_command_echoes_a_value_given_on_the_command_line() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.hi_stand_in();
    fixture.write(
        ".chough/cases/cli.cases.toml",
        &CASES.replace(" PLANTTOML", ""),
    );
    fixture.init();
    let bug = fixture.ok(&bug_add("SYNTHETIC bug"));
    fixture.commit("fixture");
    let mut ids = BTreeMap::new();
    ids.insert("session", fixture.active_id());
    ids.insert(
        "bug",
        bug["id"]
            .as_str()
            .or(bug["bug"]["id"].as_str())
            .unwrap_or("probe")
            .to_string(),
    );
    ids.insert("case", "TC-cli-0001".to_string());
    ids.insert(
        "file",
        format!(".chough/sessions/{}/session.md", ids["session"]),
    );
    let value = format!("x {TOKEN} \u{1b}[31my");
    let scratch = tempfile::tempdir().unwrap();
    let mut problems = vec![];
    let mut runs = 0;
    for (index, leaf) in leaves().iter().enumerate() {
        let copy: PathBuf = scratch.path().join(format!("e{index}"));
        copy_tree(fixture.root(), &copy);
        let base = &invocations(leaf, &ids)[0];
        let command = leaf.path.join(" ");
        let mut tries: Vec<Vec<String>> = vec![];
        // Each argument slot in turn.
        for (slot, name) in leaf.positionals.iter().enumerate() {
            if FOR_412.contains(&(command.as_str(), name.as_str())) {
                continue;
            }
            let mut args = base.clone();
            args[leaf.path.len() + slot] = value.clone();
            tries.push(args);
        }
        // Each option that takes a value, in place of its filler when it is required.
        for option in &leaf.options {
            if FOR_412.contains(&(command.as_str(), option.as_str())) {
                continue;
            }
            let mut args: Vec<String> = vec![];
            let mut words = base.iter();
            while let Some(word) = words.next() {
                if word == option {
                    words.next();
                } else {
                    args.push(word.clone());
                }
            }
            args.push(option.clone());
            args.push(value.clone());
            tries.push(args);
        }
        for args in tries {
            let args: Vec<&str> = args.iter().map(String::as_str).collect();
            let (_, _, both) = sweep_run(&copy, &args);
            runs += 1;
            if both.contains(TOKEN) || both.contains('\u{1b}') {
                let shown = both.replace(TOKEN, "<TOKEN>").replace('\u{1b}', "<ESC>");
                problems.push(format!("{:?} echoed it: {shown}", &args[..leaf.path.len()]));
            }
        }
    }
    assert!(runs > 150, "only {runs} runs");
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}
