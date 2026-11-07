//! SpecSync setup through `chough init` (REQ-cli-062, DRIVE-23): in a new project
//! with at least one named module, `chough init` runs SpecSync's init, scaffold,
//! check and adopt before its own writes; otherwise it runs none of them and says why.
#![cfg(unix)]

use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};

/// A stand-in `specsync` that appends each call to `.stub-calls` and acts like the
/// real setup steps: `init` writes `.specsync/config.toml`, `scaffold <m>` writes
/// `specs/<m>/<m>.spec.md`, and `change adopt` writes `.specsync/sdd.json`. A step
/// whose `.stub-fail-<step>` file exists exits 1 instead.
const STUB: &str = r##"#!/bin/sh
echo "$*" >>.stub-calls
step="$1"
[ "$1" = change ] && step="$2"
if [ -f ".stub-fail-$step" ]; then echo "$step failed" >&2; exit 1; fi
case "$step" in
init) mkdir -p .specsync && echo 'specs_dir = "specs"' >.specsync/config.toml ;;
scaffold) mkdir -p "specs/$2" && echo "# $2" >"specs/$2/$2.spec.md" ;;
adopt) echo '{"enabled": true}' >.specsync/sdd.json ;;
esac
exit 0
"##;

/// An empty Git project with the stand-in in `bin/`.
fn project() -> tempfile::TempDir {
    let project = tempfile::tempdir().unwrap();
    let root = project.path();
    for args in [
        &["init", "-q"][..],
        &["config", "user.name", "Test"],
        &["config", "user.email", "test@example.invalid"],
        &["commit", "-q", "--allow-empty", "-m", "start"],
    ] {
        assert!(
            Command::new("git")
                .args(args)
                .current_dir(root)
                .status()
                .unwrap()
                .success()
        );
    }
    install_stub(root);
    project
}

fn install_stub(root: &Path) {
    fs::create_dir_all(root.join("bin")).unwrap();
    let path = root.join("bin/specsync");
    fs::write(&path, STUB).unwrap();
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
}

/// `chough --root <root> <args>` with `path` as PATH: by default `bin/` first.
fn chough(root: &Path, args: &[&str], path: Option<&str>) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_chough"));
    command.arg("--root").arg(root).args(args);
    let path = match path {
        Some(path) => path.to_string(),
        None => {
            let mut paths = vec![root.join("bin")];
            paths.extend(std::env::split_paths(
                &std::env::var_os("PATH").unwrap_or_default(),
            ));
            std::env::join_paths(paths)
                .unwrap()
                .to_string_lossy()
                .into_owned()
        }
    };
    command.env("PATH", path);
    command.output().unwrap()
}

fn calls(root: &Path) -> Vec<String> {
    fs::read_to_string(root.join(".stub-calls"))
        .map(|text| text.lines().map(String::from).collect())
        .unwrap_or_default()
}

const INIT: [&str; 5] = [
    "init",
    "--objective",
    "Free shipping",
    "--module",
    "shipping",
];

#[test]
fn a_new_project_gets_specsync_setup_from_chough_init() {
    let project = project();
    let root = project.path();
    let output = chough(root, &INIT, None);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        calls(root),
        ["init", "scaffold shipping", "check", "change adopt"],
        "the setup steps, in order"
    );
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let steps = report["specsync_setup"]["steps"].as_array().unwrap();
    let commands: Vec<&str> = steps
        .iter()
        .map(|step| step["command"].as_str().unwrap())
        .collect();
    assert_eq!(
        commands,
        [
            "specsync init",
            "specsync scaffold shipping",
            "specsync check",
            "specsync change adopt"
        ]
    );
    assert!(
        steps.iter().all(|step| step["exit_code"] == 0),
        "{report:#}"
    );
    assert!(root.join(".chough/session.json").exists() || root.join(".chough/sessions").exists());
    assert!(root.join(".specsync/sdd.json").exists());
}

#[test]
fn an_existing_setup_or_no_module_runs_no_specsync_step() {
    // An existing .specsync/, an existing specs/, and no module named.
    let cases: [(&str, &[&str]); 3] = [
        (".specsync", &INIT),
        ("specs", &INIT),
        ("", &["init", "--objective", "Free shipping"]),
    ];
    for (existing, args) in cases {
        let project = project();
        let root = project.path();
        if !existing.is_empty() {
            fs::create_dir_all(root.join(existing)).unwrap();
            fs::write(root.join(existing).join("keep"), "x\n").unwrap();
        }
        let output = chough(root, args, None);
        assert!(
            output.status.success(),
            "{existing:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            calls(root).is_empty(),
            "{existing:?}: SpecSync ran: {:?}",
            calls(root)
        );
        let report: Value = serde_json::from_slice(&output.stdout).unwrap();
        let status = report["specsync_setup"]["status"]
            .as_str()
            .unwrap_or_default();
        let expected = if existing.is_empty() {
            "no-module"
        } else {
            "existing-setup-left-as-is"
        };
        assert_eq!(status, expected, "{existing:?}: {report:#}");
    }
}

#[test]
fn a_failed_setup_step_stops_init_naming_the_steps_left() {
    let project = project();
    let root = project.path();
    fs::write(root.join(".stub-fail-scaffold"), "").unwrap();
    let output = chough(root, &INIT, None);
    assert!(!output.status.success());
    assert!(
        !root.join(".chough").exists(),
        "init wrote its own files after a failed step"
    );
    let error = String::from_utf8_lossy(&output.stderr);
    for named in [
        "`specsync scaffold shipping`",
        "`specsync check`",
        "`specsync change adopt`",
        "`chough init --objective \"Free shipping\" --module shipping`",
    ] {
        assert!(
            error.contains(named),
            "the refusal does not name {named}: {error}"
        );
    }
    // Running the steps it names, then the same init, succeeds with no SpecSync call.
    fs::remove_file(root.join(".stub-fail-scaffold")).unwrap();
    for step in [
        &["scaffold", "shipping"][..],
        &["check"],
        &["change", "adopt"],
    ] {
        assert!(
            Command::new(root.join("bin/specsync"))
                .args(step)
                .current_dir(root)
                .status()
                .unwrap()
                .success()
        );
    }
    fs::remove_file(root.join(".stub-calls")).unwrap();
    let output = chough(root, &INIT, None);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        calls(root).is_empty(),
        "the rerun ran SpecSync again: {:?}",
        calls(root)
    );
}

#[test]
fn a_missing_specsync_refuses_init_naming_install() {
    let project = project();
    let root = project.path();
    let output = chough(root, &INIT, Some("/usr/bin:/bin"));
    assert!(!output.status.success());
    let error = String::from_utf8_lossy(&output.stderr);
    assert!(
        error.contains("`cargo install --locked specsync@6.2.0`"),
        "{error}"
    );
    assert!(
        error.contains("`chough init --objective \"Free shipping\" --module shipping`"),
        "{error}"
    );
    assert!(!root.join(".chough").exists() && !root.join(".specsync").exists());
    // With SpecSync installed, the same init succeeds.
    let output = chough(root, &INIT, None);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(calls(root).len(), 4);
}

#[test]
fn the_walkthrough_starts_a_new_project_with_chough_init() {
    let text =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("docs/walkthrough.md"))
            .unwrap();
    let start = text.find("## New project").expect("no New project section");
    let end = text[start + 1..]
        .find("\n## ")
        .map_or(text.len(), |at| start + 1 + at);
    let section = &text[start..end];
    for direct in [
        "specsync init",
        "specsync scaffold",
        "specsync change adopt",
    ] {
        assert!(
            !section.contains(direct),
            "the new-project section still runs {direct}"
        );
    }
    assert!(
        section.contains("chough init") && section.contains("--module"),
        "{section}"
    );
}
