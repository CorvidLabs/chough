#![allow(dead_code)]
//! Isolated project fixtures shared by the workflow integration tests.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

/// The checkpoint byte cap of REQ-cli-032 (DRIVE-14: 1 MiB, from the 2026-11-05 sizing study).
#[allow(dead_code)]
pub(super) const CHECKPOINT_CAP: usize = 1_048_576;
/// Where a test's user configuration folder is, inside its project (REQ-records-008).
pub(super) const USER_CONFIG: &str = ".user-config";
/// The environment variables that name the acting agent (REQ-records-007).
pub(super) const ACTOR_VARIABLES: [&str; 5] = [
    "CHOUGH_ACTOR",
    "CHOUGH_ACTOR_MODEL",
    "CHOUGH_HC_TERMINAL",
    "CLAUDECODE",
    "HC_TERMINAL_ID",
];

pub(super) struct Fixture(TempDir);

/// The slug Chough names a session by: lowercase letters and digits, other runs as `-`.
pub(super) fn slug(source: &str) -> String {
    let mut out = String::new();
    let mut separated = false;
    for character in source.chars() {
        let lower = character.to_ascii_lowercase();
        if lower.is_ascii_lowercase() || lower.is_ascii_digit() {
            if separated && !out.is_empty() {
                out.push('-');
            }
            separated = false;
            out.push(lower);
        } else {
            separated = true;
        }
    }
    out
}

/// Remove this fixture's checkpoint lock file from the system temporary directory when
/// the fixture is dropped, so the suite leaves no lock files behind (issue 62's lock
/// lives outside the project by design).
impl Drop for Fixture {
    fn drop(&mut self) {
        remove_checkpoint_lock(self.root());
    }
}

/// The lock file Chough uses for this project: `.chough/local/checkpoint.lock` (#83,
/// DRIVE-11). `None` when the project has no `.chough` directory.
pub(super) fn checkpoint_lock_path(root: &Path) -> Option<std::path::PathBuf> {
    root.join(".chough")
        .is_dir()
        .then(|| root.join(".chough/local/checkpoint.lock"))
}

/// Remove a project's checkpoint lock file, if any; for test cleanup only.
pub(super) fn remove_checkpoint_lock(root: &Path) {
    if let Some(lock) = checkpoint_lock_path(root) {
        let _ = fs::remove_file(lock);
    }
}

#[cfg(unix)]
pub(super) fn alive(pid: &str) -> bool {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(1);
    loop {
        let present = Command::new("kill")
            .args(["-0", pid])
            .stderr(std::process::Stdio::null())
            .status()
            .is_ok_and(|status| status.success());
        if !present {
            return false;
        }
        if let Ok(output) = Command::new("ps").args(["-o", "stat=", "-p", pid]).output() {
            let state = String::from_utf8_lossy(&output.stdout);
            let state = state.trim();
            if state.is_empty() || state.starts_with('Z') {
                return false;
            }
        }
        if std::time::Instant::now() >= deadline {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}

impl Fixture {
    pub(super) fn new() -> Self {
        let fixture = Self(tempfile::tempdir().unwrap());
        fixture.write("AGENTS.md", "Project instructions\n");
        fixture.write("INTENT.md", "A user wants a reliable product.\n");
        fixture.write("specs/cli/cli.spec.md", "# CLI contract\n");
        fixture.write(
            "specs/cli/requirements.md",
            "### REQ-cli-001\nPreserve files.\n\n### REQ-cli-002\nResume.\n\n### REQ-cli-003\nDelegate.\n",
        );
        fixture.write("src/input.txt", "first version\n");
        fixture
    }

    pub(super) fn root(&self) -> &Path {
        self.0.path()
    }

    pub(super) fn write(&self, name: &str, content: &str) {
        let path = self.root().join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, content).unwrap();
    }

    pub(super) fn read(&self, name: &str) -> String {
        fs::read_to_string(self.root().join(name)).unwrap()
    }

    pub(super) fn exists(&self, name: &str) -> bool {
        self.root().join(name).exists()
    }

    pub(super) fn run(&self, args: &[&str]) -> Output {
        self.run_with_env(args, &[])
    }

    pub(super) fn run_with_env(&self, args: &[&str], environment: &[(&str, &str)]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
        command.arg("--root").arg(self.root()).args(args);
        let mut paths = vec![self.root().join("bin")];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        command.env("PATH", std::env::join_paths(paths).unwrap());
        // The suite gives the same records under Claude Code, hc or neither
        // (REQ-records-007): a test sets the actor it means through `environment`.
        for name in ACTOR_VARIABLES {
            command.env_remove(name);
        }
        // No test reads this machine's identity file (REQ-records-008); a test that
        // needs a user file writes it under USER_CONFIG.
        command.env("XDG_CONFIG_HOME", self.root().join(USER_CONFIG));
        command.envs(environment.iter().copied());
        command.output().unwrap()
    }

    /// Run a command that must succeed and print JSON.
    pub(super) fn ok(&self, args: &[&str]) -> Value {
        let output = self.run(args);
        assert!(
            output.status.success(),
            "{args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }

    /// Run a command that must fail; returns its error text.
    pub(super) fn fails(&self, args: &[&str]) -> String {
        let output = self.run(args);
        assert!(
            !output.status.success(),
            "{args:?} unexpectedly succeeded: {}",
            String::from_utf8_lossy(&output.stdout)
        );
        String::from_utf8_lossy(&output.stderr).into_owned()
    }

    pub(super) fn init(&self) {
        self.ok(&[
            "init",
            "--objective",
            "Ship a focused change",
            "--intent-scope",
            "LIFE",
            "--module",
            "cli",
        ]);
    }

    /// Set an observation's recorded time, since `now()` is not injectable and two
    /// observations recorded in a row often share a second.
    pub(super) fn observed_at(&self, id: &str, seconds: u64) {
        let name = format!(".chough/evidence/{id}.json");
        let mut record: Value = serde_json::from_str(&self.read(&name)).unwrap();
        record["observed_at_unix_seconds"] = Value::from(seconds);
        self.write(
            &name,
            &(serde_json::to_string_pretty(&record).unwrap() + "\n"),
        );
    }

    /// The recorded time of an observation.
    pub(super) fn observed_time(&self, id: &str) -> u64 {
        let record: Value =
            serde_json::from_str(&self.read(&format!(".chough/evidence/{id}.json"))).unwrap();
        record["observed_at_unix_seconds"].as_u64().unwrap()
    }

    /// Write bug `id` as a Markdown record (REQ-cli-012) with this scope, status and
    /// linked evidence IDs, as a hand-written fixture.
    pub(super) fn bug_md(&self, id: &str, scope: &str, status: &str, evidence: &[&str]) {
        self.write(
            &format!(".chough/bugs/{id}.md"),
            &format!(
                "---\nkind: bug\nid: {id}\naliases: []\ntitle: Fixture bug {id}\nstatus: {status}\nclassification: product-defect\nscope: {scope}\nimpact: Not yet assessed.\ncriteria: []\nrequirements: []\ncases: []\nevidence: [{}]\nreceipts: []\nenvironment: local\nfound_by: fixture\nrelated: []\ncreated: 2026-11-10T00:00:01Z\nupdated: 2026-11-10T00:00:01Z\n---\n# Fixture bug {id}\n\n**What happened:** It breaks\nActual: It broke\n\n**Steps:** Running the probe\n\n**Expected:** It works\n\n**Why it matters:** Not yet assessed.\n\n**Fix:** Not fixed yet.\n\n## Log\n\n- 2026-11-10T00:00:01Z · Reported (fixture)\n",
                evidence.join(", ")
            ),
        );
    }

    /// The active session's checkpoint, as `chough checkpoint` prints it.
    pub(super) fn session(&self) -> Value {
        self.checkpoint()
    }

    /// The active session's checkpoint, as `chough checkpoint` prints it (no native read).
    pub(super) fn checkpoint(&self) -> Value {
        self.ok(&["checkpoint"])["checkpoint"].clone()
    }

    /// The ID of the active session: the pointer, or the only open session.
    pub(super) fn active_id(&self) -> String {
        if let Ok(id) = fs::read_to_string(self.root().join(".chough/local/active-session")) {
            return id.trim().to_string();
        }
        let open = self.open_ids();
        assert_eq!(open.len(), 1, "no single open session: {open:?}");
        open[0].clone()
    }

    /// The project-relative `session.md` of the active session.
    pub(super) fn session_file(&self) -> String {
        format!(".chough/sessions/{}/session.md", self.active_id())
    }

    /// The IDs of the session folders under `.chough/sessions/`.
    pub(super) fn open_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = fs::read_dir(self.root().join(".chough/sessions"))
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| entry.path().join("session.md").is_file())
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .collect()
            })
            .unwrap_or_default();
        ids.sort();
        ids
    }

    /// The ID of the open session created with `key`: `<date>-<key>`.
    pub(super) fn id_of(&self, key: &str) -> String {
        let slug = slug(key);
        self.open_ids()
            .into_iter()
            .chain(self.archived_ids())
            .find(|id| id.get(11..) == Some(slug.as_str()))
            .unwrap_or_else(|| panic!("no session for key {key}"))
    }

    /// The `session.md` of the open session created with `key`.
    pub(super) fn session_file_of(&self, key: &str) -> String {
        format!(".chough/sessions/{}/session.md", self.id_of(key))
    }

    /// The story of the archived session created with `key`.
    pub(super) fn story_of(&self, key: &str) -> String {
        format!(".chough/archive/sessions/{}.md", self.id_of(key))
    }

    /// Whether the session created with `key` is archived (its story exists).
    pub(super) fn is_archived(&self, key: &str) -> bool {
        let slug = slug(key);
        self.archived_ids()
            .iter()
            .any(|id| id.get(11..) == Some(slug.as_str()))
    }

    /// The IDs of archived sessions, from `.chough/archive/sessions/<id>.md`.
    pub(super) fn archived_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = fs::read_dir(self.root().join(".chough/archive/sessions"))
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok())
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .filter_map(|name| name.strip_suffix(".md").map(str::to_string))
                    .collect()
            })
            .unwrap_or_default();
        ids.sort();
        ids
    }

    /// The checkpoint of the session created with `key`, open or archived, read from its
    /// file in the 0.5.x JSON shape the older tests use.
    pub(super) fn session_json(&self, key: &str) -> Value {
        let file = if self.is_archived(key) {
            self.story_of(key)
        } else {
            self.session_file_of(key)
        };
        parse_session(&self.read(&file))
    }

    /// Set one front matter key of the active session to `value`, as written in the
    /// file (`accept`, `[a, b]`, `"quoted text"`).
    pub(super) fn set_front(&self, key: &str, value: &str) {
        let file = self.session_file();
        let text = self.read(&file);
        let (front, body) = text
            .strip_prefix("---\n")
            .and_then(|rest| rest.split_once("\n---\n"))
            .expect("a session record");
        let mut lines: Vec<String> = vec![];
        let mut skipping = false;
        let mut found = false;
        for line in front.lines() {
            if skipping && line.starts_with("  ") {
                continue;
            }
            skipping = false;
            if line.split_once(':').map(|(name, _)| name) == Some(key) {
                lines.push(format!("{key}: {value}"));
                skipping = true;
                found = true;
            } else {
                lines.push(line.to_string());
            }
        }
        if !found {
            lines.push(format!("{key}: {value}"));
        }
        self.write(&file, &format!("---\n{}\n---\n{body}", lines.join("\n")));
    }

    pub(super) fn git(&self, args: &[&str]) {
        let output = Command::new("git")
            .current_dir(self.root())
            .args([
                "-c",
                "user.name=Test",
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "commit.gpgsign=false",
                "-c",
                "init.defaultBranch=main",
            ])
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    pub(super) fn commit(&self, message: &str) {
        if !self.exists(".git") {
            self.git(&["init"]);
        }
        self.git(&["add", "."]);
        self.git(&["commit", "--allow-empty", "-m", message]);
    }

    /// A `hi` stand-in that reports the criteria `stubs` reports, on every platform.
    pub(super) fn hi_stand_in(&self) {
        stand_in(&self.root().join("bin"), "hi", HI_EXPORT);
    }

    /// Stub `hi` and `specsync`; the native summary is read from `.stub-*` files.
    #[cfg(unix)]
    pub(super) fn stubs(&self) {
        self.write(
            "bin/hi",
            &format!("#!/bin/sh\nprintf '%s\\n' '{HI_EXPORT}'\n"),
        );
        self.write(
            "bin/specsync",
            r#"#!/bin/sh
# Reports the summary fields SpecSync 6.1.1 reports. Files change them: .stub-state,
# .stub-approval (true, false or none to omit), .stub-correction (true or false),
# .stub-validity (terminal evidence when archived; none omits it), .stub-next (the
# native next action), .stub-affected (the quoted modules of change.affected_specs;
# none omits the change object), .stub-no-spec (true or false), .stub-verification (a JSON
# array reported as summary.verification_commands; absent omits it) and .stub-fail. When
# .stub-count exists, every call appends a line to .stub-calls. summary.id echoes the
# requested change, as SpecSync's status reply names it.
if [ -f .stub-count ]; then echo "$*" >>.stub-calls; fi
if [ -f .stub-fail ]; then echo 'status failed' >&2; exit 2; fi
state=$(cat .stub-state 2>/dev/null || echo draft)
approval=$(cat .stub-approval 2>/dev/null || echo true)
correction=$(cat .stub-correction 2>/dev/null || echo true)
validity=$(cat .stub-validity 2>/dev/null || echo authenticated-history)
next=$(cat .stub-next 2>/dev/null || echo "stub next action for $state")
affected=$(cat .stub-affected 2>/dev/null || echo '')
nospec=$(cat .stub-no-spec 2>/dev/null || echo true)
fields=",\"correction_valid\":$correction"
if [ -f .stub-verification ]; then fields="$fields,\"verification_commands\":$(cat .stub-verification)"; fi
if [ "$approval" != none ]; then fields="$fields,\"approval_valid\":$approval"; fi
if [ "$state" = archived ] && [ "$validity" != none ]; then
    fields="$fields,\"terminal_evidence\":{\"validity\":\"$validity\"}"
fi
change=""
if [ "$affected" != none ]; then
    change=",\"change\":{\"affected_specs\":[$affected],\"no_spec_change\":$nospec}"
fi
printf '{"summary":{"id":"%s","state":"%s","next_action":"%s"%s}%s}\n' "$3" "$state" "$next" "$fields" "$change"
"#,
        );
        self.executable("bin/hi");
        self.executable("bin/specsync");
    }

    /// Write a native definition approval for `id` with no semantic deltas, as SpecSync
    /// records one for a no-spec-change scope, so the approved-delta guard passes.
    pub(super) fn no_spec_approval(&self, id: &str) {
        self.write(
            &format!(".specsync/changes/{id}/approvals.json"),
            "{\"approvals\":[{\"gate\":\"definition\",\"actor\":\"SYNTHETIC-test-approver\",\"approved_delta_digests\":{}}],\"reopenings\":[]}\n",
        );
    }

    /// How many times the `specsync` stub ran since the count was last reset; counting
    /// starts once `.stub-count` exists.
    pub(super) fn native_calls(&self) -> usize {
        let path = self.root().join(".stub-calls");
        let count = fs::read_to_string(&path).map_or(0, |text| text.lines().count());
        let _ = fs::remove_file(path);
        count
    }

    /// Make the script at `name` executable as a fresh file (see [`install_executable`]).
    #[cfg(unix)]
    pub(super) fn executable(&self, name: &str) {
        let path = self.root().join(name);
        let content = fs::read(&path).unwrap();
        install_executable(&path, &content);
    }

    /// Configure an enabled native lifecycle whose status the stub reports.
    pub(super) fn native(&self, state: &str) {
        self.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
        self.write(".specsync/sdd.json", "{\"enabled\": true}\n");
        self.write(".stub-state", &format!("{state}\n"));
    }
}

/// Install an executable script at `path` as a fresh file (#132). A child `sh` writes it
/// to a temporary name, makes it executable and renames it into place. No handle in the
/// test process ever opens the new file for writing, so a child that another test
/// thread forked, still holding an old write handle until it execs, cannot make the
/// script "Text file busy" (ETXTBSY), the pattern of CorvidLabs/spec-sync#888.
#[cfg(unix)]
pub(super) fn install_executable(path: &Path, content: &[u8]) {
    use std::io::Write;
    let mut child = std::process::Command::new("sh")
        .args([
            "-c",
            "temporary=\"$1.install.$$\" && cat > \"$temporary\" && chmod 755 \"$temporary\" && mv -f \"$temporary\" \"$1\"",
            "sh",
        ])
        .arg(path)
        .stdin(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(content).unwrap();
    assert!(
        child.wait().unwrap().success(),
        "installing {}",
        path.display()
    );
}

/// The `hi export` response the stand-ins report.
const HI_EXPORT: &str = r#"{"hi":1,"export":1,"files":[{"criteria":[{"id":"LIFE-1"},{"id":"LIFE-3"},{"id":"FLOW-6"},{"id":"FLOW-7"},{"id":"FLOW-8"}],"retired":[{"id":"LIFE-2"}]}]}"#;

/// Put a `name` stand-in in `bin` that prints `stdout` and a newline and exits 0. Unix
/// gets a shell script; Windows, which has no shell stand-ins on PATH, gets a program
/// compiled once per test process and copied in.
pub(super) fn stand_in(bin: &Path, name: &str, stdout: &str) {
    fs::create_dir_all(bin).unwrap();
    #[cfg(unix)]
    {
        assert!(
            !stdout.contains('\''),
            "stand-in output must not contain a quote"
        );
        let path = bin.join(name);
        install_executable(
            &path,
            format!("#!/bin/sh\nprintf '%s\\n' '{stdout}'\n").as_bytes(),
        );
    }
    #[cfg(windows)]
    {
        use std::collections::HashMap;
        use std::path::PathBuf;
        use std::sync::{Mutex, OnceLock};
        static BUILT: OnceLock<Mutex<(TempDir, HashMap<String, PathBuf>)>> = OnceLock::new();
        let built =
            BUILT.get_or_init(|| Mutex::new((tempfile::tempdir().unwrap(), HashMap::new())));
        let mut built = built.lock().unwrap();
        let count = built.1.len();
        let program = match built.1.get(stdout) {
            Some(program) => program.clone(),
            None => {
                assert!(
                    !stdout.contains("\"#"),
                    "stand-in output must not contain \"#"
                );
                let directory = built.0.path().join(count.to_string());
                fs::create_dir_all(&directory).unwrap();
                fs::write(
                    directory.join("stand_in.rs"),
                    format!("fn main() {{ println!(\"{{}}\", r#\"{stdout}\"#); }}\n"),
                )
                .unwrap();
                let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
                let status = Command::new(rustc)
                    .current_dir(&directory)
                    .args(["--edition", "2021", "-o", "stand_in.exe", "stand_in.rs"])
                    .status()
                    .unwrap();
                assert!(status.success(), "could not build a stand-in");
                let program = directory.join("stand_in.exe");
                built.1.insert(stdout.to_string(), program.clone());
                program
            }
        };
        fs::copy(program, bin.join(format!("{name}.exe"))).unwrap();
    }
}

/// A session record's checkpoint in the 0.5.x JSON shape: front matter scalars and
/// lists, waiting gates as `awaiting_human`, and the Decisions, Where it stands and Open
/// threads items (without withdrawn or resolved ones) as lists.
pub(super) fn parse_session(text: &str) -> Value {
    let text = text.replace("\r\n", "\n");
    let (front, body) = text
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"))
        .expect("a session record");
    let mut fields = serde_json::Map::new();
    let mut key = String::new();
    let mut gates = vec![];
    for line in front.lines() {
        if let Some(item) = line.strip_prefix("  - ") {
            if key == "waiting_on_human" {
                if let Some(gate) = item.strip_prefix("gate: ") {
                    gates.push(Value::String(unquote(gate)));
                }
            } else if let Some(Value::Array(list)) = fields.get_mut(&key) {
                list.push(Value::String(unquote(item)));
            }
            continue;
        }
        if line.starts_with("    ") {
            continue;
        }
        let Some((name, value)) = line.split_once(':') else {
            continue;
        };
        key = name.to_string();
        let value = value.trim();
        let parsed = if value.is_empty() {
            Value::Array(vec![])
        } else if let Some(inner) = value.strip_prefix('[').and_then(|v| v.strip_suffix(']')) {
            Value::Array(
                inner
                    .split(", ")
                    .filter(|item| !item.is_empty())
                    .map(|item| Value::String(unquote(item)))
                    .collect(),
            )
        } else {
            Value::String(unquote(value))
        };
        fields.insert(key.clone(), parsed);
    }
    let text_of = |name: &str| {
        fields
            .get(name)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    };
    let list_of = |name: &str| fields.get(name).cloned().unwrap_or(Value::Array(vec![]));
    let section = |heading: &str, mark: &str| -> Value {
        let start = body.find(&format!("## {heading}\n"));
        let Some(start) = start else {
            return Value::Array(vec![]);
        };
        let rest = &body[start + heading.len() + 4..];
        let end = rest.find("\n## ").map_or(rest.len(), |at| at + 1);
        Value::Array(
            rest[..end]
                .lines()
                .filter_map(|line| line.strip_prefix("- "))
                .filter(|item| mark.is_empty() || !item.ends_with(mark))
                .map(|item| Value::String(item.to_string()))
                .collect(),
        )
    };
    let native = text_of("native_change");
    serde_json::json!({
        "id": text_of("id"),
        "aliases": list_of("aliases"),
        "objective": text_of("objective"),
        "intent_scope": text_of("intent_scope"),
        "modules": list_of("modules"),
        "stage": {
            "current": text_of("stage"),
            "status": text_of("stage_status"),
            "iteration": text_of("iteration").parse::<u64>().unwrap_or(1),
        },
        "native_change": if native.is_empty() { Value::Null } else { Value::String(native) },
        "references": {
            "criteria": list_of("criteria"),
            "requirements": list_of("requirements"),
            "cases": list_of("cases"),
            "design": list_of("design"),
            "files": list_of("files"),
        },
        "observations": list_of("observations"),
        "bugs": list_of("bugs"),
        "blockers": list_of("blockers"),
        "awaiting_human": gates,
        "next_action": text_of("next"),
        "decisions": section("Decisions and why", ""),
        "completed": section("Where it stands", " (withdrawn)"),
        "open_questions": section("Open threads", " (resolved)"),
        "closed": text_of("closed"),
        "outcome": text_of("outcome"),
    })
}

/// A front matter scalar as text: plain, or double-quoted with its escapes undone.
fn unquote(value: &str) -> String {
    let Some(inner) = value.strip_prefix('"').and_then(|v| v.strip_suffix('"')) else {
        return value.to_string();
    };
    inner
        .replace("\\n", "\n")
        .replace("\\\"", "\"")
        .replace("\\\\", "\\")
}

/// The Timeline entry lines of a session record.
pub(super) fn timeline(text: &str) -> Vec<String> {
    let Some(start) = text.find("\n## Timeline\n") else {
        return vec![];
    };
    text[start..]
        .lines()
        .filter(|line| line.starts_with("- "))
        .map(str::to_string)
        .collect()
}
