//! REQ-records-001 (#313): a new record can be created on a file system that has no
//! hard links. #306 linked a temporary file to the record's name, which such a file
//! system refuses, so `init`, `bug add`, `journal add` and `inbox add` failed there; that
//! writer was reverted. macOS only, and opt-in: the test makes and mounts a small exFAT
//! image with `hdiutil`, which can hang while other image operations run, so it is
//! ignored by default. Run it by hand: `cargo test --locked --test no_hard_links -- --ignored`.
#![cfg(target_os = "macos")]

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A mounted exFAT image, detached when dropped.
struct Volume {
    _work: tempfile::TempDir,
    mount: PathBuf,
}

impl Volume {
    fn new() -> Self {
        let work = tempfile::tempdir().unwrap();
        let image = work.path().join("volume.dmg");
        let mount = work.path().join("mount");
        let created = Command::new("hdiutil")
            .args(["create", "-size", "64m", "-fs", "ExFAT", "-volname", "x"])
            .arg(&image)
            .output()
            .unwrap();
        assert!(created.status.success(), "{created:?}");
        let attached = Command::new("hdiutil")
            .args(["attach", "-nobrowse", "-mountpoint"])
            .arg(&mount)
            .arg(&image)
            .output()
            .unwrap();
        assert!(attached.status.success(), "{attached:?}");
        Self { _work: work, mount }
    }
}

impl Drop for Volume {
    fn drop(&mut self) {
        let _ = Command::new("hdiutil")
            .args(["detach", "-force"])
            .arg(&self.mount)
            .output();
    }
}

fn chough(root: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(root)
        .args(args)
        .env("XDG_CONFIG_HOME", root.join(".user-config"))
        .env("CHOUGH_ACTOR", "SYNTHETIC-agent")
        .output()
        .unwrap()
}

fn ok(root: &Path, args: &[&str]) {
    let output = chough(root, args);
    assert!(
        output.status.success(),
        "chough {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

/// On an exFAT volume, where a
/// hard link is refused, `init`, `bug add`, `journal add` and `inbox add` write whole
/// records.
#[test]
#[ignore = "mounts an exFAT image with hdiutil; run by hand with -- --ignored"]
fn new_records_are_written_on_a_volume_without_hard_links() {
    let volume = Volume::new();
    let root = volume.mount.join("project");
    std::fs::create_dir_all(&root).unwrap();
    // The premise: this volume refuses a hard link.
    std::fs::write(root.join("a"), "a").unwrap();
    assert!(std::fs::hard_link(root.join("a"), root.join("b")).is_err());
    let git = Command::new("git")
        .args(["init", "-q"])
        .current_dir(&root)
        .output()
        .unwrap();
    assert!(git.status.success(), "{git:?}");

    ok(&root, &["init", "--objective", "SYNTHETIC probe"]);
    let readme = std::fs::read_to_string(root.join(".chough/README.md")).unwrap();
    assert!(!readme.is_empty());
    ok(
        &root,
        &[
            "bug",
            "add",
            "--title",
            "SYNTHETIC bug",
            "--symptom",
            "It breaks",
            "--trigger",
            "Running the probe",
            "--expected",
            "It works",
            "--actual",
            "It broke",
            "--environment",
            "local",
        ],
    );
    // macOS keeps extended attributes on exFAT in `._<name>` files of its own.
    let bugs: Vec<_> = std::fs::read_dir(root.join(".chough/bugs"))
        .unwrap()
        .flatten()
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with("._"))
        .collect();
    assert_eq!(bugs.len(), 1, "{bugs:?}");
    let bug = std::fs::read_to_string(bugs[0].path()).unwrap();
    assert!(
        bug.starts_with("---\n") && bug.contains("SYNTHETIC bug"),
        "{bug}"
    );
    ok(
        &root,
        &[
            "journal",
            "add",
            "--kind",
            "learned",
            "--title",
            "SYNTHETIC post",
            "--body",
            "A post.",
        ],
    );
    // The inbox add of an observation, on a volume without hard links, writes its entry
    // whole through the same writer.
    let session = std::fs::read_dir(root.join(".chough/sessions"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .find(|name| {
            std::fs::metadata(root.join(".chough/sessions").join(name).join("session.md")).is_ok()
        })
        .expect("the session the init made");
    ok(
        &root,
        &[
            "inbox",
            "add",
            "--session",
            session.as_str(),
            "--from",
            "SYNTHETIC-agent",
            "--kind",
            "observation",
            "--text",
            "OBS-synthetic-inbox",
        ],
    );
    // The entry the add wrote is there, and it is not empty; macOS keeps extended
    // attributes on exFAT in `._<name>` files of its own, so skip those.
    let inbox = root.join(format!(".chough/sessions/{session}/agent/inbox"));
    let entry = std::fs::read_dir(&inbox)
        .unwrap()
        .flatten()
        .find(|candidate| !candidate.file_name().to_string_lossy().starts_with("._"))
        .expect("the inbox entry the add wrote");
    let text = std::fs::read_to_string(entry.path()).unwrap();
    assert!(!text.is_empty(), "the inbox entry was empty");
    // No temporary file is left behind in the record folders.
    for folder in [".chough/bugs", ".chough/local"] {
        for entry in std::fs::read_dir(root.join(folder)).unwrap().flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            assert!(!name.ends_with(".tmp"), "{folder}/{name} was left behind");
        }
    }
}
