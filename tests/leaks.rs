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

/// REQ-records-006 (#267), TC-records-a-hyphen-glued-token-is-a-leak: a token glued to
/// a hyphen, the shape a slug or a run name gives it, is refused at the write like the
/// same token after a space, and the doctor reports it in a committed file.
#[test]
fn a_hyphen_glued_token_is_refused_at_the_write_and_found_by_the_doctor() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    fixture.commit("fixture");
    let glued = concat!("x-", "ghp_", "abcdefghij0123456789abcdefghij0123");
    let secret = concat!("ghp_", "abcdefghij0123456789");
    let decision = format!("decided: probe {glued}");
    refused(
        &fixture,
        &["checkpoint", "--decision", &decision],
        "decision",
        "token",
        secret,
    );
    let before = tree(fixture.root());
    let output = run(&fixture, &["session", "new", glued, "--objective", "probe"]);
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !output.status.success(),
        "the session key was written: {said}"
    );
    assert!(said.contains("refused:leak"), "{said}");
    assert!(said.contains("holds token"), "{said}");
    assert!(
        !said.contains(secret),
        "the refusal echoed the value: {said}"
    );
    assert_eq!(tree(fixture.root()), before, "session new changed a file");

    let lesson = "knowledge/lessons/2026-01-01-old-run.md";
    fixture.write(
        lesson,
        &format!(
            "---\nid: 2026-01-01-old-run\ntitle: \"Old\"\ntype: lesson\ntags: []\n---\n\nSee run {glued}\n"
        ),
    );
    fixture.git(&["add", lesson]);
    fixture.git(&["commit", "-qm", "an old run name"]);
    let output = run(&fixture, &["doctor", "--leaks"]);
    let said = String::from_utf8_lossy(&output.stdout).into_owned();
    assert_eq!(output.status.code(), Some(1), "{said}");
    assert!(
        !said.contains(secret),
        "the doctor echoed the value: {said}"
    );
    let report: serde_json::Value = serde_json::from_str(&said).unwrap();
    let leaks: Vec<&serde_json::Value> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "leak")
        .collect();
    assert_eq!(leaks.len(), 1, "{report:#}");
    assert_eq!(leaks[0]["target"], lesson);
    assert_eq!(leaks[0]["match"], "token");
}

/// REQ-records-006, TC-records-sk-after-a-hyphen-is-not-a-key (#316): `sk-` after a
/// hyphen continues a word, as in 0.6.0, so a bug whose generated ID holds `-sk-learn-…`
/// is written, a 0.6.0 bug with such an ID still updates and gives the doctor no leak,
/// and a distinctive token glued to a hyphen (#267) is still refused.
#[test]
fn a_slug_holding_sk_is_not_a_key_and_a_glued_token_still_is() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    fixture.commit("fixture");

    let output = run(
        &fixture,
        &[
            "bug",
            "add",
            "--title",
            "Fix flaky sk-learn tests in CI pipeline",
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
        ],
    );
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "the sk-learn bug was refused: {said}"
    );
    assert!(
        said.contains("-fix-flaky-sk-learn-tests-in-ci-pipeline"),
        "{said}"
    );

    let upgraded = "2026-11-18-use-sk-learn-for-the-classifier-model";
    fixture.bug_md(upgraded, "in-scope", "open", &[]);
    fixture.git(&["add", "-A"]);
    fixture.git(&["commit", "-qm", "a 0.6.0 bug and the sk-learn bug"]);
    let output = run(
        &fixture,
        &[
            "bug",
            "update",
            upgraded,
            "--note",
            "Pinned the sk-learn version",
        ],
    );
    assert!(
        output.status.success(),
        "the 0.6.0 bug did not update: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    fixture.git(&["add", "-A"]);
    fixture.git(&["commit", "-qm", "update the 0.6.0 bug"]);
    let output = run(&fixture, &["doctor", "--leaks"]);
    let said = String::from_utf8_lossy(&output.stdout).into_owned();
    let report: serde_json::Value = serde_json::from_str(&said).unwrap();
    let leaks: Vec<&serde_json::Value> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "leak")
        .collect();
    assert!(leaks.is_empty(), "{report:#}");

    let glued = concat!("x-", "ghp_", "abcdefghij0123456789abcdefghij0123");
    refused(
        &fixture,
        &[
            "checkpoint",
            "--decision",
            &format!("decided: probe {glued}"),
        ],
        "decision",
        "token",
        concat!("ghp_", "abcdefghij0123456789"),
    );
}

/// REQ-records-006, TC-records-a-file-or-design-path-passes-the-leak-guard (#268): a
/// `checkpoint --file` or `--design` path is a value Chough writes into `session.md`, so
/// a token in it refuses the write naming the field, with the token never echoed and
/// every file unchanged; with `--redact` the path is stored redacted; a clean path is
/// saved as before.
#[test]
fn a_token_in_a_file_or_design_path_is_refused_at_the_write() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    fixture.commit("fixture");
    let path = concat!("src/", "ghp_", "abcdefghij0123456789abcdefghij0123", ".txt");
    let secret = concat!("ghp_", "abcdefghij0123456789");
    refused(
        &fixture,
        &["checkpoint", "--file", path],
        "file",
        "token",
        secret,
    );
    refused(
        &fixture,
        &["checkpoint", "--design", path],
        "design",
        "token",
        secret,
    );

    let said = redacted(&fixture, &["checkpoint", "--file", path, "--design", path]);
    assert!(!said.contains(secret), "--redact echoed the value: {said}");
    assert!(
        holding(fixture.root(), secret).is_empty(),
        "a file still holds the token"
    );
    let session = holding(fixture.root(), "src/<redacted:token>.txt");
    assert_eq!(session.len(), 1, "{session:?}");
    assert!(session[0].ends_with("/session.md"), "{session:?}");

    let output = run(
        &fixture,
        &[
            "checkpoint",
            "--file",
            "src/main.rs",
            "--design",
            "docs/flow.md",
        ],
    );
    assert!(output.status.success(), "{output:?}");
    let session = fs::read_to_string(fixture.root().join(&session[0])).unwrap();
    assert!(session.contains("src/main.rs"), "{session}");
    assert!(session.contains("docs/flow.md"), "{session}");
}

/// REQ-records-006 (#268): an ID reference is a value Chough writes too, so a token in a
/// `--criterion`, `--requirement`, `--case`, `--observation` or `--native-change` value
/// refuses the write naming the field; under `--redact` it still refuses, since an ID
/// would not keep its form as `<redacted:<kind>>`, with every file unchanged.
#[test]
fn a_token_in_an_id_reference_is_refused_even_under_redact() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    fixture.commit("fixture");
    let secret = concat!("ghp_", "abcdefghij0123456789");
    let github = concat!("ghp_", "abcdefghij0123456789abcdefghij0123");
    let criterion = format!("x-{github}");
    let case = format!("TC-x-{github}");
    let requirement = concat!("REQ-", "AKIA", "ABCDEFGHIJKLMNOP");
    let observation = concat!("o-", "glp", "at-abcdefghijklmnopqrstuvwx");
    for (flag, value, field, hidden) in [
        ("--criterion", criterion.as_str(), "criterion", secret),
        (
            "--requirement",
            requirement,
            "requirement",
            "ABCDEFGHIJKLMNOP",
        ),
        ("--case", case.as_str(), "case", secret),
        (
            "--observation",
            observation,
            "observation",
            "abcdefghijklmnopqrst",
        ),
        ("--native-change", github, "native_change", secret),
    ] {
        refused(
            &fixture,
            &["checkpoint", flag, value],
            field,
            "token",
            hidden,
        );
        refused(
            &fixture,
            &["--redact", "checkpoint", flag, value],
            field,
            "token",
            hidden,
        );
    }
    assert!(holding(fixture.root(), secret).is_empty());
}

/// REQ-records-006 (#268 review): in a session that already holds a value its save
/// would refuse, such as a token-bearing `--file` path written before #268, `evidence
/// add` refuses before it writes its entry, so a retry adds nothing either.
#[test]
fn evidence_add_in_a_session_holding_a_leak_writes_nothing() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.configure_human();
    fixture.init();
    let path = concat!("src/", "ghp_", "abcdefghij0123456789abcdefghij0123", ".txt");
    let session = tree(fixture.root())
        .into_keys()
        .find(|name| name.ends_with("/session.md"))
        .unwrap();
    let text = fs::read_to_string(fixture.root().join(&session)).unwrap();
    assert!(text.contains("\nfiles: []\n"), "{text}");
    fs::write(
        fixture.root().join(&session),
        text.replace("\nfiles: []\n", &format!("\nfiles: [{path}]\n")),
    )
    .unwrap();
    fixture.commit("a session written before #268");
    let arguments = evidence_add("SYNTHETIC entry in a leaking session");
    let arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    for _ in 0..2 {
        refused(
            &fixture,
            &arguments,
            "file",
            "token",
            concat!("ghp_", "abcdefghij0123456789"),
        );
    }
}

/// A committed session whose `cases` reference holds a token, as an older Chough let
/// one be saved, with an email in its Goal too: (fixture, session record, session ID).
fn session_with_a_leaking_id() -> (Fixture, String, String) {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let id = fixture.active_id();
    let record = format!(".chough/sessions/{id}/session.md");
    let text = fs::read_to_string(fixture.root().join(&record)).unwrap();
    assert!(text.contains("\ncases: []\n"), "{text}");
    let leaking = concat!("TC-x-", "ghp_", "abcdefghij0123456789abcdefghij0123");
    let email = concat!("someone", "@", "example.org");
    let text = text
        .replace("\ncases: []\n", &format!("\ncases: [{leaking}]\n"))
        .replacen(
            "## Goal\n\n",
            &format!("## Goal\n\nWrite to {email} for access.\n\n"),
            1,
        );
    fs::write(fixture.root().join(&record), text).unwrap();
    fixture.commit("a session an older Chough let hold a token");
    (fixture, record, id)
}

/// REQ-records-006 (#320): a save refused because a session already holds a leaking ID
/// reference names the recovery, `chough session shelve <id>`, with every file unchanged.
#[test]
fn a_leaking_id_reference_names_the_recovery() {
    let (fixture, _record, id) = session_with_a_leaking_id();
    let before = tree(fixture.root());
    let output = run(
        &fixture,
        &["checkpoint", "--decision", "decided: SYNTHETIC"],
    );
    let said = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!output.status.success(), "{said}");
    assert!(said.contains("refused:leak: case holds token"), "{said}");
    assert!(
        said.contains(&format!("chough session shelve {id}")),
        "the refusal names no recovery: {said}"
    );
    assert!(!said.contains(concat!("ghp_", "abcdefghij")), "{said}");
    assert_eq!(tree(fixture.root()), before);
}

/// REQ-history-012, REQ-cli-055 (#320), TC-cli-doctor-redact-never-rewrites-an-id: the
/// doctor's leak finding on an ID line names `chough session shelve <id>`, and `--fix
/// --redact` leaves that line byte for byte with the finding skipped, so the session
/// still reads, while it redacts the email on an ordinary line of the same record.
#[test]
fn doctor_redact_never_rewrites_an_id() {
    let (fixture, record, id) = session_with_a_leaking_id();
    let leaks = |args: &[&str]| -> Vec<serde_json::Value> {
        let output = run(&fixture, args);
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        report["findings"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|finding| finding["kind"] == "leak")
            .cloned()
            .collect()
    };
    let found = leaks(&["doctor", "--leaks"]);
    assert_eq!(found.len(), 2, "{found:#?}");
    let id_finding = found
        .iter()
        .find(|finding| finding["match"] == "token")
        .expect("the ID's finding");
    assert!(
        id_finding
            .to_string()
            .contains(&format!("chough session shelve {id}")),
        "{id_finding:#}"
    );

    let fixed = leaks(&["doctor", "--fix", "--redact"]);
    let token = fixed
        .iter()
        .find(|finding| finding["match"] == "token")
        .expect("the ID's finding");
    assert_eq!(token["status"], "skipped", "{fixed:#?}");
    let email = fixed
        .iter()
        .find(|finding| finding["match"] == "email")
        .expect("the email's finding");
    assert_eq!(email["status"], "fixed", "{fixed:#?}");
    let text = fs::read_to_string(fixture.root().join(&record)).unwrap();
    assert!(
        text.contains(concat!(
            "cases: [TC-x-",
            "ghp_",
            "abcdefghij0123456789abcdefghij0123]"
        )),
        "the ID was rewritten:\n{text}"
    );
    assert!(text.contains("<redacted:email>"), "{text}");
    let next = run(&fixture, &["next"]);
    assert!(
        next.status.success(),
        "the session no longer reads: {}",
        String::from_utf8_lossy(&next.stderr)
    );
}

/// REQ-history-012, REQ-cli-055 (#320, #335 review): a session's `modules` and a bug's
/// `regression_baseline` give IDs too, a module name and an evidence ID. `--fix
/// --redact` leaves both lines byte for byte with their findings skipped, so the
/// session and the bug still read, as before the doctor ran.
#[test]
fn doctor_redact_never_rewrites_modules_or_a_regression_baseline() {
    let fixture = Fixture::new();
    fixture.configure_human();
    fixture.init();
    let email = concat!("someone", "@", "example.org");
    let token = concat!("sk-", "abcdefghijklmnopqrstuvwx");
    // `session new --module` refuses a leaking module name since #368, so an older
    // session that holds them is written by hand.
    fixture.ok(&["session", "new", "mods", "--objective", "module probe"]);
    fixture.set_front("modules", &format!("[{email}, {token}]"));
    let id = fixture.active_id();
    let record = format!(".chough/sessions/{id}/session.md");
    let session = fs::read_to_string(fixture.root().join(&record)).unwrap();
    let modules = session
        .lines()
        .find(|line| line.starts_with("modules:"))
        .expect("a modules line")
        .to_string();
    assert!(
        modules.contains(token) && modules.contains(email),
        "{session}"
    );
    let baseline = concat!("o-x-", "ghp_", "abcdefghij0123456789abcdefghij0123");
    let bug = ".chough/bugs/2026-11-10-totals.md";
    fixture.write(
        bug,
        &format!(
            "---\nkind: bug\nid: 2026-11-10-totals\ntitle: Totals\nstatus: open\nclassification: regression\nscope: in-scope\nimpact: High\ncriteria: []\nrequirements: []\ncases: []\nevidence: []\nregression_baseline: {baseline}\nreceipts: []\nenvironment: test-local\nfound_by: manual report\nrelated: []\ncreated: 2026-11-10T10:00:00Z\nupdated: 2026-11-10T10:00:00Z\n---\n# Totals\n\n**What happened:** Off by one\n\n**Steps:** Any order\n\n**Expected:** Exact\n\n**Why it matters:** High\n\n**Fix:** Not fixed yet.\n\n## Log\n\n- 2026-11-10T10:00:00Z · Reported (manual report)\n"
        ),
    );
    fixture.commit("leaking module names and a leaking regression baseline");
    let reads = |args: &[&str]| {
        let output = run(&fixture, args);
        assert!(
            output.status.success(),
            "{args:?} no longer reads: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    };
    reads(&["next"]);
    reads(&["bug", "show", "2026-11-10-totals"]);

    let output = run(&fixture, &["doctor", "--fix", "--redact"]);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let leaks: Vec<&serde_json::Value> = report["findings"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|finding| finding["kind"] == "leak")
        .collect();
    assert_eq!(leaks.len(), 3, "{report:#}");
    for finding in &leaks {
        assert_eq!(finding["status"], "skipped", "{finding:#}");
    }
    assert!(
        fs::read_to_string(fixture.root().join(&record))
            .unwrap()
            .lines()
            .any(|line| line == modules),
        "the modules line was rewritten"
    );
    assert!(
        fs::read_to_string(fixture.root().join(bug))
            .unwrap()
            .contains(&format!("\nregression_baseline: {baseline}\n")),
        "the regression baseline was rewritten"
    );
    reads(&["next"]);
    reads(&["status"]);
    reads(&["bug", "show", "2026-11-10-totals"]);
}

/// REQ-records-006 (#364), TC-records-an-answer-is-leak-checked-and-bounded: an answer
/// given with `--resolve-question` is checked like every other value Chough writes. Each
/// kind refuses naming `answer`, with nothing written and the value never echoed; under
/// `--redact` the answer holds `<redacted:<kind>>`; and an answer over 4,096 bytes is
/// refused.
#[test]
fn an_answer_is_leak_checked_redacted_and_bounded() {
    let fixture = Fixture::new();
    fixture.init();
    for (kind, value, secret) in SAMPLES {
        let question = format!("which {kind}");
        let output = run(&fixture, &["checkpoint", "--question", &question]);
        assert!(output.status.success(), "{output:?}");
        // `--answer=<value>`, since a private key block starts with hyphens.
        let answer = format!("--answer={value}");
        refused(
            &fixture,
            &["checkpoint", "--resolve-question", &question, &answer],
            "answer",
            kind,
            secret,
        );
    }
    let (_, token, secret) = SAMPLES
        .into_iter()
        .find(|(kind, _, _)| *kind == "token")
        .unwrap();
    let answer = format!("--answer={token}");
    redacted(
        &fixture,
        &["checkpoint", "--resolve-question", "which token", &answer],
    );
    let text = fixture.read(&fixture.session_file());
    assert!(text.contains("<redacted:token>"), "{text}");
    assert!(!text.contains(secret), "{text}");

    let output = run(&fixture, &["checkpoint", "--question", "how long"]);
    assert!(output.status.success(), "{output:?}");
    let before = tree(fixture.root());
    let long = "y".repeat(4_097);
    let output = run(
        &fixture,
        &[
            "checkpoint",
            "--resolve-question",
            "how long",
            "--answer",
            &long,
        ],
    );
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(!output.status.success(), "a 4,097-byte answer was written");
    assert!(said.contains("answer exceeds 4096 bytes"), "{said}");
    assert_eq!(tree(fixture.root()), before);
}

/// The leak forms #368 adds, each with the text that must never be echoed.
const LISTED_FORMS: [(&str, &str, &str); 6] = [
    (
        "credential",
        concat!("{\"password\": \"", "hunter2", "hunter2\"}"),
        concat!("hunter2", "hunter2"),
    ),
    (
        "credential",
        concat!("SECRET_KEY=", "abcdefgh", "12345678"),
        concat!("abcdefgh", "12345678"),
    ),
    (
        "credential",
        concat!("AWS_SECRET_ACCESS_KEY=", "wJalrXUtnFEMI", "K7MDENGbPxRfiCY"),
        concat!("wJalrXUtnFEMI", "K7MDENG"),
    ),
    (
        "token",
        concat!("xapp-", "1-A0123ABCDEF-", "1234567890123-abcdef"),
        concat!("A0123ABCDEF-", "1234567890123"),
    ),
    (
        "token",
        concat!("hf_", "abcdefghijklmnop", "qrstuvwxyzABCDEFGH"),
        concat!("abcdefghijklmnop", "qrstuvwxyz"),
    ),
    (
        "home-path",
        "see ~SYNTHETICother/projects/app",
        "~SYNTHETICother",
    ),
];

/// REQ-records-006 (#368), TC-records-the-listed-leak-forms-are-refused: a password JSON
/// key, `SECRET_KEY=` and `AWS_SECRET_ACCESS_KEY=` assignments, `xapp-` and `hf_` tokens
/// and a `~<user>/` path each refuse a write naming the field and the kind; a module name
/// holding an email is refused at `session new`, under `--redact` too; and `evidence add
/// --redact` says how many values it redacted.
#[test]
fn the_listed_leak_forms_are_refused_and_counted() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    for (kind, value, secret) in LISTED_FORMS {
        refused(
            &fixture,
            &["checkpoint", "--decision", value],
            "decision",
            kind,
            secret,
        );
    }
    let email = "synthetic.person@example.com";
    for redact in [false, true] {
        let mut args = vec![];
        if redact {
            args.push("--redact");
        }
        args.extend(["session", "new", "beta", "--module", email]);
        let before = tree(fixture.root());
        let output = run(&fixture, &args);
        let said = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!output.status.success(), "{args:?} was written: {said}");
        assert!(said.contains("module holds email"), "{args:?}: {said}");
        assert!(!said.contains(email), "{args:?} echoed the value: {said}");
        assert_eq!(tree(fixture.root()), before, "{args:?} changed a file");
    }

    fixture.commit("fixture");
    let arguments = evidence_add("SYNTHETIC check /Users/SYNTHETIC-person/notes.txt");
    let mut arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    arguments.insert(0, "--redact");
    let output = run(&fixture, &arguments);
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "{said}");
    assert!(
        said.contains("chough: redacted ") && said.contains("value(s) in evidence entry"),
        "{said}"
    );
}

/// REQ-records-006 (#379), TC-records-the-user-name-comes-from-the-account: with `USER`
/// and `LOGNAME` empty, as under `env -i`, cron or launchd, the account's user name as
/// `<user>@<host>` is still refused; and with `USER=dev`, a `/dev/` path, a system root, is
/// written, while `dev` deeper in a path is still the user name.
#[test]
fn the_user_name_comes_from_the_account() {
    let fixture = Fixture::new();
    fixture.init();
    let account = String::from_utf8(
        std::process::Command::new("id")
            .arg("-un")
            .output()
            .unwrap()
            .stdout,
    )
    .unwrap()
    .trim()
    .to_string();
    assert!(account.len() >= 2, "{account:?}");
    // As `<user>@<host>`, which only the user-name rule matches, whatever the host is
    // called.
    let value = format!("ssh {account}@build-box");
    let before = tree(fixture.root());
    let output = fixture.run_with_env(
        &["checkpoint", "--decision", &value],
        &[("USER", ""), ("LOGNAME", "")],
    );
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(!output.status.success(), "{value} was written");
    assert!(said.contains("decision holds user-name"), "{said}");
    assert_eq!(tree(fixture.root()), before);

    let dev = [("USER", "dev"), ("LOGNAME", "dev")];
    let output = fixture.run_with_env(&["checkpoint", "--decision", "mounted /dev/disk2s1"], &dev);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let output = fixture.run_with_env(&["checkpoint", "--decision", "see /opt/dev/x"], &dev);
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(!output.status.success(), "/opt/dev/x was written");
    assert!(said.contains("decision holds user-name"), "{said}");
}

/// REQ-records-006 (#367), TC-records-errors-show-a-given-value-only-when-clean: across the
/// errors that name a value a person gave, a value holding a token is named by what it
/// holds, never shown: an unknown question or completed entry, an invalid criterion ID,
/// a stage `--holds` does not know, an unknown case for `cases run` or `evidence add`,
/// an invalid linked ID for `bug add`, and a value the argument parser refuses.
#[test]
fn errors_show_a_given_value_only_when_clean() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.init();
    let token = concat!("x ghp_", "abcdefghij0123456789", "abcdefghij0123 y");
    let bug = [
        "bug",
        "add",
        "--title",
        "SYNTHETIC probe",
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
    ];
    let evidence = [
        "evidence",
        "add",
        "--criterion",
        "LIFE-1",
        "--kind",
        "human-observation",
        "--outcome",
        "passed",
        "--environment",
        "local",
        "--summary",
        "SYNTHETIC probe",
    ];
    let mut cases: Vec<Vec<&str>> = vec![
        vec!["checkpoint", "--resolve-question", token],
        vec!["checkpoint", "--remove-completed", token],
        vec!["checkpoint", "--criterion", token],
        vec![
            "checkpoint",
            "--awaiting-human",
            "SYNTHETIC gate",
            "--holds",
            token,
        ],
        vec!["cases", "run", "--environment", "local", token],
        vec!["evidence", "add", "--kind", token],
    ];
    let mut linked = bug.to_vec();
    linked.extend(["--criterion", token]);
    cases.push(linked);
    let mut case = evidence.to_vec();
    case.extend(["--case", token]);
    cases.push(case);
    let before = tree(fixture.root());
    for args in &cases {
        let output = run(&fixture, args);
        let said = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!output.status.success(), "{args:?} succeeded: {said}");
        assert!(!said.contains("ghp_"), "{args:?} showed the value: {said}");
        assert!(said.contains("holding token"), "{args:?}: {said}");
    }
    assert_eq!(tree(fixture.root()), before);
}
