//! The Linux test environment is rebuilt from the repository with one command, using the
//! tool versions the project pins (REQ-cli-058, DRIVE-22). These checks need no Docker.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    fs::read_to_string(repo().join(path)).unwrap_or_else(|error| panic!("{path}: {error}"))
}

/// The tool pins in the CI workflow, read independently of the scripts under test.
fn ci_pins() -> Vec<(String, String)> {
    let ci = read(".github/workflows/ci.yml");
    let mut specsync: Vec<String> = vec![];
    let mut fledge = None;
    let mut lines = ci.lines().peekable();
    while let Some(line) = lines.next() {
        if line.contains("uses: CorvidLabs/spec-sync@") || line.contains("uses: CorvidLabs/fledge@")
        {
            for next in lines.by_ref() {
                if let Some(version) = next.trim().strip_prefix("version:") {
                    let version = version.trim().trim_matches('\'').trim_start_matches('v');
                    if line.contains("spec-sync") {
                        specsync.push(version.to_string());
                    } else {
                        fledge = Some(version.to_string());
                    }
                    break;
                }
                if next.trim_start().starts_with("- ") {
                    break;
                }
            }
        }
    }
    specsync.dedup();
    assert_eq!(
        specsync.len(),
        1,
        "CI pins one specsync version: {specsync:?}"
    );
    let human_intent = ci
        .lines()
        .find_map(|line| {
            line.split("cargo install human-intent --version ")
                .nth(1)
                .map(|rest| rest.split_whitespace().next().unwrap().to_string())
        })
        .expect("CI installs a pinned human-intent");
    vec![
        ("specsync".into(), specsync.remove(0)),
        ("fledge".into(), fledge.expect("CI pins fledge")),
        ("human-intent".into(), human_intent),
    ]
}

fn sh(script: &str, args: &[&str], env: &[(&str, &Path)]) -> std::process::Output {
    let mut command = Command::new("sh");
    command
        .arg(repo().join(script))
        .args(args)
        .current_dir(repo());
    for (key, value) in env {
        command.env(key, value);
    }
    command.output().unwrap()
}

/// DRIVE-22: `fledge run linux-image` and `fledge run linux-verify` are project tasks.
#[test]
fn fledge_declares_the_linux_image_and_verify_tasks() {
    let fledge = read("fledge.toml");
    assert!(
        fledge.contains("linux-image = \"sh tools/linux/image.sh\""),
        "{fledge}"
    );
    assert!(
        fledge.contains("linux-verify = \"sh tools/linux/verify.sh\""),
        "{fledge}"
    );
}

/// DRIVE-22: the image's versions come from the CI workflow's pins, never a copy.
#[cfg(unix)]
#[test]
fn the_image_pins_are_the_ci_pins() {
    let output = sh("tools/linux/pins.sh", &[], &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let printed = String::from_utf8(output.stdout).unwrap();
    let expected: String = ci_pins()
        .iter()
        .map(|(tool, version)| format!("{tool}={version}\n"))
        .collect();
    assert_eq!(printed, expected);
}

/// DRIVE-22: a workflow whose specsync pins disagree, or that lacks a pin, refuses
/// before anything is built.
#[cfg(unix)]
#[test]
fn missing_or_disagreeing_pins_refuse() {
    let dir = tempfile::tempdir().unwrap();
    let ci = read(".github/workflows/ci.yml");
    let pinned = &ci_pins()[0].1;
    let mut replaced = false;
    let disagreeing: String = ci
        .lines()
        .map(|line| {
            if !replaced && line.trim() == format!("version: '{pinned}'") {
                replaced = true;
                line.replace(pinned, "6.1.2")
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(replaced);
    let file = dir.path().join("disagree.yml");
    fs::write(&file, disagreeing).unwrap();
    let output = sh("tools/linux/pins.sh", &[], &[("CHOUGH_CI_FILE", &file)]);
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("specsync"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let missing = dir.path().join("missing.yml");
    fs::write(
        &missing,
        ci.replace("cargo install human-intent", "cargo install nothing"),
    )
    .unwrap();
    let output = sh("tools/linux/pins.sh", &[], &[("CHOUGH_CI_FILE", &missing)]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("human-intent"));
}

/// DRIVE-22: the Dockerfile names no tool version itself; every one is a build argument
/// that the image script fills from the pins.
#[test]
fn the_dockerfile_takes_every_tool_version_from_a_build_argument() {
    let dockerfile = read("tools/linux/Dockerfile");
    for (tool, version) in ci_pins() {
        assert!(
            !dockerfile.contains(&version),
            "the Dockerfile hard-codes {tool} {version}"
        );
    }
    for argument in ["SPECSYNC_VERSION", "FLEDGE_VERSION", "HUMAN_INTENT_VERSION"] {
        assert!(
            dockerfile.contains(&format!("ARG {argument}\n")),
            "{argument}"
        );
        assert!(
            dockerfile.contains(&format!("${{{argument}}}")),
            "{argument}"
        );
    }
    let image = read("tools/linux/image.sh");
    for target in ["chough-linux:1", "chough-linux:2", "chough-linux:3"] {
        assert!(image.contains(target), "{target}");
    }
}

/// DRIVE-22: `linux-verify` refuses with exit 3, before verifying, when a tool in the
/// image is not the pinned version, and names the command that rebuilds the image.
#[cfg(unix)]
#[test]
fn the_version_check_refuses_a_tool_that_is_not_pinned() {
    use std::os::unix::fs::PermissionsExt;
    let pins = ci_pins();
    let version = |tool: &str| {
        pins.iter()
            .find(|(name, _)| name == tool)
            .unwrap()
            .1
            .clone()
    };
    let bin = tempfile::tempdir().unwrap();
    let stub = |name: &str, output: &str| {
        let path = bin.path().join(name);
        fs::write(&path, format!("#!/bin/sh\necho '{output}'\n")).unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    };
    let check = |stale: Option<&str>| {
        stub(
            "specsync",
            &format!(
                "specsync {}",
                if stale == Some("specsync") {
                    "6.1.2".into()
                } else {
                    version("specsync")
                }
            ),
        );
        stub("fledge", &format!("fledge {}", version("fledge")));
        stub("hi", &format!("hi {}", version("human-intent")));
        let path = format!("{}:/usr/bin:/bin", bin.path().display());
        Command::new("sh")
            .arg(repo().join("tools/linux/check-versions.sh"))
            .current_dir(repo())
            .env("PATH", path)
            .output()
            .unwrap()
    };
    let pinned = check(None);
    assert!(
        pinned.status.success(),
        "{}",
        String::from_utf8_lossy(&pinned.stderr)
    );
    let stale = check(Some("specsync"));
    assert_eq!(stale.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&stale.stderr);
    assert!(
        stderr.contains("specsync") && stderr.contains("fledge run linux-image"),
        "{stderr}"
    );
}
