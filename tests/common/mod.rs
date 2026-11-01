#![allow(dead_code)]
//! Isolated project fixtures shared by the workflow integration tests.

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

pub(super) struct Fixture(TempDir);

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
        let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
        command.arg("--root").arg(self.root()).args(args);
        let mut paths = vec![self.root().join("bin")];
        paths.extend(std::env::split_paths(
            &std::env::var_os("PATH").unwrap_or_default(),
        ));
        command.env("PATH", std::env::join_paths(paths).unwrap());
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

    pub(super) fn session(&self) -> Value {
        serde_json::from_str(&self.read(".chough/session.json")).unwrap()
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
# .stub-validity (terminal evidence when archived; none omits it) and .stub-fail.
if [ -f .stub-fail ]; then echo 'status failed' >&2; exit 2; fi
state=$(cat .stub-state 2>/dev/null || echo draft)
approval=$(cat .stub-approval 2>/dev/null || echo true)
correction=$(cat .stub-correction 2>/dev/null || echo true)
validity=$(cat .stub-validity 2>/dev/null || echo authenticated-history)
fields=",\"correction_valid\":$correction"
if [ "$approval" != none ]; then fields="$fields,\"approval_valid\":$approval"; fi
if [ "$state" = archived ] && [ "$validity" != none ]; then
    fields="$fields,\"terminal_evidence\":{\"validity\":\"$validity\"}"
fi
printf '{"summary":{"state":"%s","next_action":"stub next action for %s"%s}}\n' "$state" "$state" "$fields"
"#,
        );
        self.executable("bin/hi");
        self.executable("bin/specsync");
    }

    #[cfg(unix)]
    pub(super) fn executable(&self, name: &str) {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(self.root().join(name), fs::Permissions::from_mode(0o755)).unwrap();
    }

    /// Configure an enabled native lifecycle whose status the stub reports.
    pub(super) fn native(&self, state: &str) {
        self.write(".specsync/config.toml", "specs_dir = \"specs\"\n");
        self.write(".specsync/sdd.json", "{\"enabled\": true}\n");
        self.write(".stub-state", &format!("{state}\n"));
    }
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
        use std::os::unix::fs::PermissionsExt;
        assert!(
            !stdout.contains('\''),
            "stand-in output must not contain a quote"
        );
        let path = bin.join(name);
        fs::write(&path, format!("#!/bin/sh\nprintf '%s\\n' '{stdout}'\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
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
