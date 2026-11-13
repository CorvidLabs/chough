//! No leaks in committed records (REQ-records-006): every value a write stores is
//! checked for local paths, user and host names, emails and credentials; a match refuses
//! the write naming the field and the kind, never the text, or with `--redact` is stored
//! as `<redacted:<kind>>`; and `chough doctor` finds what earlier writes left in the
//! files Git tracks, which the verify lane's `leaks` step fails on.

mod common;

use common::Fixture;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

/// The local user name the runs below see, so a user name leak is synthetic too.
const USER: &str = "SYNTHETICuser";

/// One synthetic value of each kind the guard matches, with the text that must never
/// be echoed. The host name kind depends on this machine's name, so the unit tests in
/// `src/leaks.rs` cover it with a synthetic host.
const SAMPLES: [(&str, &str, &str); 10] = [
    (
        "home-path",
        "ran in /Users/SYNTHETIC-person/src/app",
        "/Users/SYNTHETIC-person",
    ),
    (
        "system-path",
        "wrote /tmp/SYNTHETIC-scratch/out.txt",
        "/tmp/SYNTHETIC-scratch",
    ),
    (
        "windows-path",
        "C:\\Users\\SYNTHETIC\\x",
        "C:\\Users\\SYNTHETIC",
    ),
    (
        "user-name",
        "ssh SYNTHETICuser@build-box",
        "SYNTHETICuser@build-box",
    ),
    (
        "email",
        "mail synthetic.person+x@example.com now",
        "synthetic.person+x@example.com",
    ),
    (
        "token",
        concat!("token ", "ghp_", "abcdefghijklmnopqrstuvwxyz0123456789"),
        concat!("ghp_", "abcdefghijklmnop"),
    ),
    (
        "private-key",
        concat!("-----BEGIN OPENSSH ", "PRIVATE KEY-----"),
        concat!("BEGIN OPENSSH ", "PRIVATE KEY"),
    ),
    (
        "authorization",
        concat!(
            "curl -H 'Authorization: Bearer ",
            "abcdefgh",
            "ijklmnop1234'"
        ),
        "abcdefghijklmnop1234",
    ),
    (
        "jwt",
        concat!(
            "jwt ",
            "eyJ",
            "hbGciOiJIUzI1NiJ9.",
            "eyJ",
            "zdWIiOiIxMjM0In0.abcdefghijkl"
        ),
        concat!("eyJ", "hbGciOiJIUzI1NiJ9"),
    ),
    (
        "credential",
        concat!("password: ", "hunter2", "hunter2"),
        concat!("hunter2", "hunter2"),
    ),
];

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

fn run(fixture: &Fixture, args: &[&str]) -> std::process::Output {
    fixture.run_with_env(args, &[("USER", USER), ("LOGNAME", USER)])
}

/// Run a write that must be refused as a leak of `kind` in `field`; nothing changes and
/// neither stream holds `secret`.
fn refused(fixture: &Fixture, args: &[&str], field: &str, kind: &str, secret: &str) {
    let before = tree(fixture.root());
    let output = run(fixture, args);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.status.success(), "{args:?} was written: {said}");
    assert!(
        said.contains(&format!("refused:leak: {field} holds {kind}")),
        "{args:?}: {said}"
    );
    assert!(!said.contains(secret), "{args:?} echoed the value: {said}");
    assert_eq!(tree(fixture.root()), before, "{args:?} changed a file");
}

/// Run a write with `--redact`; it succeeds and says it redacted.
fn redacted(fixture: &Fixture, args: &[&str]) -> String {
    let mut all = vec!["--redact"];
    all.extend_from_slice(args);
    let output = run(fixture, &all);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.status.success(), "{all:?}: {said}");
    assert!(
        said.contains("redacted"),
        "{all:?} did not report the redaction: {said}"
    );
    said
}

/// Every file whose text holds `needle`.
fn holding(root: &Path, needle: &str) -> Vec<String> {
    tree(root)
        .into_iter()
        .filter(|(_, bytes)| String::from_utf8_lossy(bytes).contains(needle))
        .map(|(name, _)| name)
        .collect()
}

/// A case whose binding prints `ok`, for `validate run`.
const CASES: &str = "profile = \"chough-testing/1\"\nmodule = \"cli\"\n\n[[case]]\nid = \"TC-cli-0001\"\ntitle = \"Probe\"\nstatus = \"active\"\ntrack = \"functional\"\nmethod = \"automated\"\ncriteria = [\"LIFE-1\"]\nrequirements = [\"REQ-cli-001\"]\nenvironments = [\"local\"]\nartifacts = [\"test-log\"]\nexpected = [\"Probe prints ok\"]\n\n[case.bind]\nadapter = \"command\"\nargv = [\"sh\", \"-c\", \"echo ok\"]\nexpect_output = [\"ok\"]\n";

/// REQ-records-006, TC-records-leak-guard: each pattern kind in a write is refused as
/// `refused:leak` naming the field and the kind, with the value never echoed and every
/// file unchanged; the guarded writes (a decision, relayed words, a bug, a preference's
/// source, a consent's source, an inbox entry and a validation run) each refuse, and
/// with `--redact` store `<redacted:<kind>>` and say so; a committed record holding a
/// home path is a `leak` finding by file, line and kind, without the path, which the
/// verify lane's `leaks` step fails on, until `chough doctor --fix --redact` replaces it.
#[test]
fn no_write_leaks_and_doctor_finds_old_leaks() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.write(".chough/cases/cli.cases.toml", CASES);
    fixture.init();
    fixture.commit("fixture");

    for (kind, value, secret) in SAMPLES {
        let decision = format!("decided: keep it because {value}");
        refused(
            &fixture,
            &["checkpoint", "--decision", &decision],
            "decision",
            kind,
            secret,
        );
    }

    let (path, secret) = ("/Users/SYNTHETIC-person/notes", "/Users/SYNTHETIC-person");
    let text = format!("seen at {path}");
    fixture.ok(&["checkpoint", "--awaiting-human", "SYNTHETIC gate"]);
    fixture.ok(&["pref", "show"]);
    let session = fixture.active_id();
    let writes: Vec<(Vec<&str>, &str)> = vec![
        (vec!["checkpoint", "--decision", &text], "decision"),
        (
            vec![
                "checkpoint",
                "--close-gate",
                "SYNTHETIC gate",
                "--decided-by",
                "SYNTHETIC-human",
                "--words",
                &text,
                "--where",
                "a test",
                "--recorded-by",
                "SYNTHETIC-agent",
                "--no-native",
            ],
            "--words",
        ),
        (
            vec![
                "bug",
                "add",
                "--title",
                "SYNTHETIC bug",
                "--symptom",
                &text,
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
            ],
            "body",
        ),
        (
            vec!["pref", "set", "monitoring", "off", "--source", &text],
            "--source",
        ),
        (
            vec!["consent", "grant", "repeat-validation", "--source", &text],
            "consent source",
        ),
        (
            vec![
                "inbox",
                "add",
                "--session",
                &session,
                "--from",
                "SYNTHETIC-agent",
                "--kind",
                "question",
                "--text",
                &text,
            ],
            "--text",
        ),
    ];
    for (args, field) in &writes {
        refused(&fixture, args, field, "home-path", secret);
    }
    // A validation run is refused before its record is written or any attempt runs.
    fixture.ok(&[
        "consent",
        "grant",
        "repeat-validation",
        "--source",
        "Human: SYNTHETIC go ahead",
    ]);
    refused(
        &fixture,
        &[
            "validate",
            "run",
            "TC-cli-0001",
            "--times",
            "1",
            "--environment",
            &text,
        ],
        "--environment",
        "home-path",
        secret,
    );
    assert!(!fixture.exists(".chough/validation"));

    // With --redact each write proceeds, storing the kind in place of the value.
    for (args, _) in &writes {
        redacted(&fixture, args);
    }
    assert!(
        holding(fixture.root(), secret).is_empty(),
        "{:?}",
        holding(fixture.root(), secret)
    );
    let stored = holding(fixture.root(), "<redacted:home-path>");
    for expected in [
        ".chough/preferences.json",
        "/agent/inbox/",
        "session.md",
        ".chough/bugs/",
    ] {
        assert!(
            stored.iter().any(|path| path.contains(expected)),
            "no redaction stored in {expected}: {stored:?}"
        );
    }

    // A record an earlier write left with a home path, committed: a finding by file,
    // line and kind, never the path; the verify lane's `leaks` step fails on it.
    let lesson = "knowledge/lessons/2026-01-01-old-lesson.md";
    fixture.write(
        lesson,
        "---\nid: 2026-01-01-old-lesson\ntitle: \"Old\"\ntype: lesson\ntags: []\n---\n\nSee /Users/SYNTHETIC-person/old.txt\n",
    );
    fixture.write(
        "knowledge/untracked.md",
        "See /Users/SYNTHETIC-person/untracked.txt\n",
    );
    fixture.git(&["add", lesson]);
    fixture.git(&["commit", "-qm", "an old lesson"]);
    for args in [&["doctor"][..], &["doctor", "--leaks"][..]] {
        let output = run(&fixture, args);
        let said = String::from_utf8_lossy(&output.stdout).into_owned();
        assert_eq!(output.status.code(), Some(1), "{args:?}: {said}");
        assert!(!said.contains(secret), "{args:?} echoed the path: {said}");
        let report: serde_json::Value = serde_json::from_str(&said).unwrap();
        let leaks: Vec<&serde_json::Value> = report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|finding| finding["kind"] == "leak")
            .collect();
        assert_eq!(
            leaks.len(),
            1,
            "{args:?}: only the tracked lesson: {report:#}"
        );
        assert_eq!(leaks[0]["target"], lesson);
        assert_eq!(leaks[0]["line"], 8);
        assert_eq!(leaks[0]["match"], "home-path");
        assert_eq!(leaks[0]["fix"], "chough doctor --fix --redact");
    }
    let fledge =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("fledge.toml")).unwrap();
    assert!(
        fledge.contains("leaks = \"cargo run --quiet --locked -- doctor --leaks\""),
        "no leaks task"
    );
    let verify = fledge
        .split("[lanes.verify]")
        .nth(1)
        .and_then(|lane| lane.lines().find(|line| line.starts_with("steps")))
        .unwrap_or_default();
    assert!(
        verify.contains("\"leaks\""),
        "the verify lane has no leaks step: {verify}"
    );

    let output = run(&fixture, &["--redact", "doctor", "--fix"]);
    assert!(
        fixture.read(lesson).contains("<redacted:home-path>")
            && !fixture.read(lesson).contains(secret),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    // The untracked file is never scanned, so nothing is left.
    assert_eq!(run(&fixture, &["doctor", "--leaks"]).status.code(), Some(0));
}

/// The arguments of an `evidence add` with `summary`, for the fixture's stubbed specs.
fn evidence_add(summary: &str) -> Vec<String> {
    [
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--requirement",
        "REQ-cli-001",
        "--contract",
        "specs/cli/cli.spec.md",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        summary,
    ]
    .iter()
    .map(|argument| argument.to_string())
    .collect()
}

/// The file names under `dir` in the fixture.
fn names_in(fixture: &Fixture, dir: &str) -> Vec<String> {
    fs::read_dir(fixture.root().join(dir))
        .map(|entries| {
            entries
                .flatten()
                .map(|entry| entry.file_name().to_string_lossy().into_owned())
                .collect()
        })
        .unwrap_or_default()
}

/// REQ-records-006, TC-records-credential-value-ends-at-markup (R1): only an
/// assignment whose value has 8 or more characters is a credential, measured without
/// the bold, full stop, bracket or link markup Chough adds around a claim. A claim
/// ending in a 5-character assignment is written; a 4-character one leaves no `leak`
/// finding on the session's What was verified line; an 8-character one is still
/// refused.
#[test]
fn a_short_assignment_ending_a_claim_is_written_and_leaves_no_leak_finding() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    for summary in ["SYNTHETIC password=abcde", "SYNTHETIC password=abcd"] {
        let arguments = evidence_add(summary);
        let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
        let output = run(&fixture, &arguments);
        assert!(
            output.status.success(),
            "{summary}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    fixture.commit("short assignments");
    let doctor = run(&fixture, &["doctor", "--leaks"]);
    let said = String::from_utf8_lossy(&doctor.stdout).into_owned();
    assert!(
        doctor.status.success() && !said.contains("credential"),
        "doctor reported a leak on Chough's own lines: {said}{}",
        String::from_utf8_lossy(&doctor.stderr)
    );
    let arguments = evidence_add(concat!("SYNTHETIC password=", "hunter2", "hunter2."));
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    let output = run(&fixture, &arguments);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !output.status.success() && stderr.contains("holds credential"),
        "an 8-character value was written: {stderr}"
    );
    assert!(!stderr.contains(concat!("hunter2", "hunter2")), "{stderr}");
}

/// REQ-records-006, REQ-records-003, TC-records-redacted-writes-are-named-from-the-redacted-text
/// (R2): under `--redact`, an evidence entry and a journal post are named from the
/// redacted text, so no file name, `id`, link or feed entry keeps the value, and the
/// checkpoint a command prints holds what it saved.
#[test]
fn redacted_writes_are_named_from_the_redacted_text() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let arguments = evidence_add("SYNTHETIC check /Users/SYNTHETIC-person/notes.txt");
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    redacted(&fixture, &arguments);
    let session = fixture.active_id();
    let entries = names_in(&fixture, &format!(".chough/sessions/{session}/evidence"));
    assert!(
        entries
            .iter()
            .any(|name| name.contains("redacted-home-path")),
        "{entries:?}"
    );
    redacted(
        &fixture,
        &[
            "journal",
            "add",
            "--kind",
            "learned",
            "--title",
            concat!(
                "SYNTHETIC post ",
                "ghp_",
                "abcdefghijklmnopqrstuvwxyz0123456789"
            ),
            "--body",
            "SYNTHETIC body",
        ],
    );
    let posts = names_in(&fixture, "knowledge/journal");
    assert!(
        posts.iter().any(|name| name.contains("redacted-token")),
        "{posts:?}"
    );
    let said = redacted(
        &fixture,
        &[
            "checkpoint",
            "--decision",
            "decided: keep it because ran in /Users/SYNTHETIC-person/src/app",
        ],
    );
    for needle in [
        "synthetic-person",
        "SYNTHETIC-person",
        concat!("ghp_", "abcdefghij"),
        "ghp-abcdefghij",
        "abcdefghijklmnopqrstuvwxyz0123456789",
    ] {
        assert_eq!(
            holding(fixture.root(), needle),
            Vec::<String>::new(),
            "{needle}"
        );
        assert!(
            !said.contains(needle),
            "the printed checkpoint holds {needle}: {said}"
        );
    }
}

/// This machine's short host name as the leak guard reads it, when it is one the guard
/// matches (3 or more characters, not `localhost`).
fn guarded_host() -> Option<String> {
    let output = std::process::Command::new("hostname").output().ok()?;
    let host = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let short = host.split('.').next().unwrap_or_default().to_string();
    (short.len() >= 3 && !short.eq_ignore_ascii_case("localhost")).then_some(short)
}

/// REQ-records-006, TC-records-given-names-hold-no-host-name (R3): a session key or an
/// explicit evidence ID that holds this machine's host name is refused as
/// `refused:leak` naming the field and `host-name`, with every file unchanged.
#[test]
fn a_given_name_holding_the_host_name_is_refused() {
    let Some(host) = guarded_host() else {
        eprintln!("skipped: this machine's host name is one the guard never matches");
        return;
    };
    let words = common::slug(&host);
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    fixture.commit("fixture");
    let key = format!("synthetic-{words}-key");
    refused(
        &fixture,
        &["session", "new", &key, "--objective", "SYNTHETIC ok"],
        "key",
        "host-name",
        &words,
    );
    let id = format!("2026-11-12-built-on-{words}");
    let mut arguments = evidence_add("SYNTHETIC explicit id");
    arguments.insert(2, id);
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    refused(&fixture, &arguments, "id", "host-name", &words);
}
