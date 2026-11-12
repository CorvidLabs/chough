//! Offline release-byte regression coverage for issue #102 and REQ-cli-016.
//! Corpus identity is checked completely before any child CLI invocation.

use serde::Deserialize;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use tempfile::TempDir;

const MANIFEST_SHA256: &str = "94e8aa07827937f39093c2b8781b8868fbe53440028f3304f6a0463b19c08c94";
const PINNED_RELEASES: &[(&str, &str, &str)] = &[
    (
        "v0.1.0",
        "c03d41956ef1d9f8380f02034f03d42bf52bc174",
        "1859f69abd8762826e5012cb4f85920f502ee383b45e78c0458f50a9a3bf48e9",
    ),
    (
        "v0.2.0",
        "edbc3c1adecebf6a1378a731965c3bfe8c8217df",
        "7120fd59d75d37e8e7f08874a0d38d323b08dfd2708fdade6e962bc476c1aadd",
    ),
    (
        "v0.3.0",
        "4d65542ff583ee2478f7d98c1dd4434a960e9dc1",
        "fd653290aef505e2fcdad9135581076e2a8d56d91d10237ef9d117661e55bca0",
    ),
    (
        "v0.4.0",
        "69950c7a9253230aeeee3bac0d0d6b580e291aad",
        "bbbea74e358a1ffd113932b6121831bf7576b1b5fc7108caca2b927b8b82d36c",
    ),
    (
        "v0.5.0",
        "455cb643cd91a9c804afdf5566e3889abf1bde49",
        "e3b211832d6f978248591d9854cf6f83e43d7d4b2ff3c00d648e91693cf16719",
    ),
];
const BUNDLE_NAMES: &[&str] = &[
    "SKILL.md",
    "agents/openai.yaml",
    "references/accept.md",
    "references/bugs.md",
    "references/build.md",
    "references/checkpoint.md",
    "references/design.md",
    "references/intent.md",
    "references/librarian.md",
    "references/lifecycle.md",
    "references/plan.md",
    "references/release.md",
    "references/roles.md",
    "references/stages.md",
    "references/testing.md",
    "references/validate.md",
    "templates/brief.md",
    "templates/bug-report.md",
    "templates/cases.toml",
    "templates/frd.md",
    "templates/prototype/COMPANION.md",
    "templates/prototype/index.html",
    "templates/prototype/styles.css",
    "templates/review-findings.md",
];
const HOSTS: &[(&str, &str)] = &[
    ("openai", ".agents/skills/chough-workflow"),
    ("claude", ".agents/skills/chough-workflow"),
    ("codex", ".codex/skills/chough-workflow"),
    ("cursor", ".cursor/skills/chough-workflow"),
    ("gemini", ".gemini/skills/chough-workflow"),
    ("grok", ".grok/skills/chough-workflow"),
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    schema_version: u8,
    releases: BTreeMap<String, Release>,
    current_bundle_sha256: BTreeMap<String, String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Release {
    source_commit: String,
    archive_sha256: String,
    files: BTreeMap<String, FileIdentity>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct FileIdentity {
    sha256: String,
    bytes: usize,
}

struct Corpus {
    releases: BTreeMap<String, BTreeMap<String, Vec<u8>>>,
    current: BTreeMap<String, Vec<u8>>,
}

fn require(condition: bool, message: impl Into<String>) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.into())
    }
}

fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn fixture_directory() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/skill-releases")
}

fn load_corpus(directory: &Path) -> Result<Corpus, String> {
    let manifest_bytes = fs::read(directory.join("manifest.json")).map_err(|e| e.to_string())?;
    require(
        digest(&manifest_bytes) == MANIFEST_SHA256,
        "manifest identity changed",
    )?;
    let manifest: Manifest = serde_json::from_slice(&manifest_bytes).map_err(|e| e.to_string())?;
    require(manifest.schema_version == 1, "manifest schema changed")?;
    require(
        manifest.releases.len() == 5,
        "expected exactly five pinned releases",
    )?;
    let expected_names: BTreeSet<_> = BUNDLE_NAMES
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    require(expected_names.len() == 24, "bundle path inventory is not24")?;
    require(
        manifest
            .current_bundle_sha256
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>()
            == expected_names,
        "current path inventory changed",
    )?;
    let mut current = BTreeMap::new();
    for (name, expected) in &manifest.current_bundle_sha256 {
        let bytes = fs::read(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("skills/chough-workflow")
                .join(name),
        )
        .map_err(|e| e.to_string())?;
        require(
            digest(&bytes) == *expected,
            format!("current bundle changed: {name}"),
        )?;
        current.insert(name.clone(), bytes);
    }
    let mut releases = BTreeMap::new();
    let mut expected_blobs = BTreeSet::new();
    let mut mapping_count = 0;
    for (tag, commit, archive) in PINNED_RELEASES {
        let release = manifest
            .releases
            .get(*tag)
            .ok_or_else(|| format!("missing release {tag}"))?;
        require(
            release.source_commit == *commit && release.archive_sha256 == *archive,
            format!("release source identity changed: {tag}"),
        )?;
        require(
            release.files.keys().cloned().collect::<BTreeSet<_>>() == expected_names,
            format!("release path inventory changed: {tag}"),
        )?;
        let mut files = BTreeMap::new();
        for (name, identity) in &release.files {
            require(
                identity.sha256.len() == 64
                    && identity
                        .sha256
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
                "invalid blob digest path",
            )?;
            let file = format!("{}.txt", identity.sha256);
            let path = directory.join("blobs").join(&file);
            let metadata = fs::symlink_metadata(&path).map_err(|e| e.to_string())?;
            require(
                metadata.file_type().is_file(),
                "fixture blob must be a regular file",
            )?;
            let bytes = fs::read(&path).map_err(|e| e.to_string())?;
            require(
                bytes.len() == identity.bytes && digest(&bytes) == identity.sha256,
                format!("released bytes changed: {tag}:{name}"),
            )?;
            files.insert(name.clone(), bytes);
            expected_blobs.insert(file);
            mapping_count += 1;
        }
        releases.insert((*tag).to_string(), files);
    }
    require(
        mapping_count == 120 && expected_blobs.len() == 51,
        "release corpus counts changed",
    )?;
    let actual_blobs: BTreeSet<String> = fs::read_dir(directory.join("blobs"))
        .map_err(|e| e.to_string())?
        .map(|entry| {
            entry
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .map_err(|e| e.to_string())
        })
        .collect::<Result<_, _>>()?;
    require(
        actual_blobs == expected_blobs,
        "fixture blob inventory changed",
    )?;
    Ok(Corpus { releases, current })
}

fn corpus() -> Corpus {
    load_corpus(&fixture_directory())
        .expect("validate every source/file/blob identity before running CLI")
}

fn cli_output(_validated: &Corpus, root: &Path, arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_chough"))
        .arg("--root")
        .arg(root)
        .args(arguments)
        .output()
        .expect("run private CLI fixture")
}

fn cli(validated: &Corpus, root: &Path, arguments: &[&str]) -> Value {
    let output = cli_output(validated, root, arguments);
    assert!(
        output.status.success(),
        "{arguments:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("CLI JSON output")
}

fn row<'a>(rows: &'a Value, name: &str) -> &'a Value {
    rows.as_array()
        .expect("file rows")
        .iter()
        .find(|r| r["file"] == name)
        .expect("named bundle row")
}

#[cfg(unix)]
fn mode(path: &Path) -> u32 {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).unwrap().permissions().mode() & 0o777
}

#[cfg(not(unix))]
fn mode(path: &Path) -> u32 {
    u32::from(fs::metadata(path).unwrap().permissions().readonly())
}

fn write_bytes(path: &Path, bytes: &[u8]) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(path, fs::Permissions::from_mode(0o640)).unwrap();
    }
}

fn seed(root: &Path, base: &str, files: &BTreeMap<String, Vec<u8>>) {
    for (name, bytes) in files {
        write_bytes(&root.join(base).join(name), bytes);
    }
}

fn snapshot(base: &Path) -> BTreeMap<String, (Vec<u8>, u32)> {
    fn walk(base: &Path, path: &Path, result: &mut BTreeMap<String, (Vec<u8>, u32)>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(base, &path, result);
            } else {
                result.insert(
                    path.strip_prefix(base)
                        .unwrap()
                        .to_string_lossy()
                        .replace('\\', "/"),
                    (fs::read(&path).unwrap(), mode(&path)),
                );
            }
        }
    }
    let mut result = BTreeMap::new();
    walk(base, base, &mut result);
    result
}

#[test]
fn released_corpus_identity_rejects_tampering_before_cli_invocation() {
    let validated = corpus();
    assert_eq!(validated.releases.len(), 5);
    let temporary = TempDir::new().unwrap();
    let directory = temporary.path();
    fs::copy(
        fixture_directory().join("manifest.json"),
        directory.join("manifest.json"),
    )
    .unwrap();
    fs::create_dir(directory.join("blobs")).unwrap();
    for entry in fs::read_dir(fixture_directory().join("blobs")).unwrap() {
        let entry = entry.unwrap();
        fs::copy(
            entry.path(),
            directory.join("blobs").join(entry.file_name()),
        )
        .unwrap();
    }
    assert!(load_corpus(directory).is_ok());
    let blob = fs::read_dir(directory.join("blobs"))
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .path();
    let original = fs::read(&blob).unwrap();
    let mut changed = original.clone();
    changed.push(b'!');
    fs::write(&blob, changed).unwrap();
    assert!(
        load_corpus(directory).is_err(),
        "changed blob must fail before any CLI helper is called"
    );
    fs::write(&blob, original).unwrap();
    fs::remove_file(&blob).unwrap();
    assert!(
        load_corpus(directory).is_err(),
        "missing blob must fail before CLI"
    );
    fs::write(directory.join("manifest.json"), b"{}\n").unwrap();
    assert!(
        load_corpus(directory).is_err(),
        "changed manifest must fail before CLI"
    );
}

#[test]
fn every_pinned_release_file_is_recognized_before_update() {
    let corpus = corpus();
    let mut failures = Vec::new();
    let mut checked = 0;
    for (tag, files) in &corpus.releases {
        for (host, base) in HOSTS {
            let temporary = TempDir::new().unwrap();
            seed(temporary.path(), base, files);
            let before = snapshot(temporary.path());
            let report = cli(
                &corpus,
                temporary.path(),
                &["skill", "status", "--host", host],
            );
            assert_eq!(report["hosts"][0]["directory"], *base);
            assert_eq!(
                snapshot(temporary.path()),
                before,
                "status must write nothing"
            );
            for name in BUNDLE_NAMES {
                let actual = row(&report["hosts"][0]["files"], name)["state"]
                    .as_str()
                    .unwrap();
                let expected = if files[*name] == corpus.current[*name] {
                    "current"
                } else {
                    "known-outdated"
                };
                if actual != expected {
                    failures.push(format!(
                        "{tag}/{host}/{name}: pristine expected {expected}, actual {actual}"
                    ));
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 720);
    assert!(
        failures.is_empty(),
        "pristine releases misclassified as edits:\n{}",
        failures.join("\n")
    );
}

#[test]
fn every_pristine_release_updates_instead_of_staying_a_preserved_edit() {
    let corpus = corpus();
    let mut failures = Vec::new();
    let mut checked = 0;
    for (tag, files) in &corpus.releases {
        for (host, base) in HOSTS {
            let temporary = TempDir::new().unwrap();
            seed(temporary.path(), base, files);
            let report = cli(
                &corpus,
                temporary.path(),
                &["skill", "update", "--host", host],
            );
            for name in BUNDLE_NAMES {
                let current = &corpus.current[*name];
                let entry = row(&report["hosts"][0]["files"], name);
                let (state, action) = if files[*name] == *current {
                    ("current", "none")
                } else {
                    ("known-outdated", "updated")
                };
                if entry["state"] != state
                    || entry["action"] != action
                    || fs::read(temporary.path().join(base).join(name)).unwrap() != *current
                {
                    failures.push(format!("{tag}/{host}/{name}: expected {state}/{action}/current bytes, actual {}/{}", entry["state"], entry["action"]));
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 720);
    assert!(
        failures.is_empty(),
        "stale pristine references must not be preserved as edits:\n{}",
        failures.join("\n")
    );
}

#[test]
fn user_edited_release_bytes_and_modes_survive_every_host_update() {
    let corpus = corpus();
    let mut checked = 0;
    for files in corpus.releases.values() {
        for (host, base) in HOSTS {
            let edited: BTreeMap<_, _> = files
                .iter()
                .map(|(name, bytes)| {
                    let mut bytes = bytes.clone();
                    bytes.extend_from_slice(b"\nPRIVATE-USER-EDIT-102\n");
                    (name.clone(), bytes)
                })
                .collect();
            let temporary = TempDir::new().unwrap();
            seed(temporary.path(), base, &edited);
            let before = snapshot(temporary.path());
            let status = cli(
                &corpus,
                temporary.path(),
                &["skill", "status", "--host", host],
            );
            let update = cli(
                &corpus,
                temporary.path(),
                &["skill", "update", "--host", host],
            );
            for name in BUNDLE_NAMES {
                assert_eq!(row(&status["hosts"][0]["files"], name)["state"], "edited");
                assert_eq!(
                    row(&update["hosts"][0]["files"], name)["action"],
                    "preserved-edit"
                );
                checked += 1;
            }
            assert_eq!(
                snapshot(temporary.path()),
                before,
                "edited bytes and modes must be exact"
            );
        }
    }
    assert_eq!(checked, 720);
}

#[test]
fn recognition_requires_the_filename_and_exact_unnormalized_bytes() {
    let corpus = corpus();
    let known = &corpus.releases["v0.2.0"]["references/build.md"];
    let control = TempDir::new().unwrap();
    seed(control.path(), HOSTS[0].1, &corpus.current);
    write_bytes(
        &control.path().join(HOSTS[0].1).join("references/build.md"),
        known,
    );
    assert_eq!(
        row(
            &cli(
                &corpus,
                control.path(),
                &["skill", "status", "--host", "openai"]
            )["hosts"][0]["files"],
            "references/build.md"
        )["state"],
        "known-outdated",
        "wrong-name control must start with a recognized predecessor"
    );
    let mut one_byte = known.clone();
    one_byte[0] ^= 1;
    let crlf = String::from_utf8(known.clone())
        .unwrap()
        .replace('\n', "\r\n")
        .into_bytes();
    assert_ne!(crlf, *known);
    for (name, bytes) in [
        ("references/accept.md", known.clone()),
        ("references/build.md", one_byte),
        ("references/build.md", crlf),
    ] {
        let temporary = TempDir::new().unwrap();
        seed(temporary.path(), HOSTS[0].1, &corpus.current);
        write_bytes(&temporary.path().join(HOSTS[0].1).join(name), &bytes);
        write_bytes(
            &temporary.path().join(HOSTS[0].1).join("private-unknown.md"),
            b"user-owned\n",
        );
        let before = snapshot(temporary.path());
        let status = cli(
            &corpus,
            temporary.path(),
            &["skill", "status", "--host", "openai"],
        );
        assert_eq!(row(&status["hosts"][0]["files"], name)["state"], "edited");
        let update = cli(
            &corpus,
            temporary.path(),
            &["skill", "update", "--host", "openai"],
        );
        assert_eq!(
            row(&update["hosts"][0]["files"], name)["action"],
            "preserved-edit"
        );
        assert_eq!(snapshot(temporary.path()), before);
    }
}

#[test]
fn current_missing_and_shared_host_routing_stay_unchanged() {
    let corpus = corpus();
    for (host, base) in HOSTS {
        let temporary = TempDir::new().unwrap();
        seed(temporary.path(), base, &corpus.current);
        let before = snapshot(temporary.path());
        let report = cli(
            &corpus,
            temporary.path(),
            &["skill", "update", "--host", host],
        );
        assert_eq!(report["hosts"][0]["directory"], *base);
        for name in BUNDLE_NAMES {
            assert_eq!(row(&report["hosts"][0]["files"], name)["state"], "current");
            assert_eq!(row(&report["hosts"][0]["files"], name)["action"], "none");
        }
        assert_eq!(snapshot(temporary.path()), before);
        let missing = temporary.path().join(base).join("templates/brief.md");
        fs::remove_file(&missing).unwrap();
        let status = cli(
            &corpus,
            temporary.path(),
            &["skill", "status", "--host", host],
        );
        assert_eq!(
            row(&status["hosts"][0]["files"], "templates/brief.md")["state"],
            "missing"
        );
        let report = cli(
            &corpus,
            temporary.path(),
            &["skill", "update", "--host", host],
        );
        assert_eq!(
            row(&report["hosts"][0]["files"], "templates/brief.md")["action"],
            "added"
        );
        assert_eq!(
            fs::read(missing).unwrap(),
            corpus.current["templates/brief.md"]
        );
    }
    let temporary = TempDir::new().unwrap();
    seed(temporary.path(), HOSTS[0].1, &corpus.current);
    let report = cli(
        &corpus,
        temporary.path(),
        &["skill", "update", "--host", "openai", "--host", "claude"],
    );
    assert_eq!(report["hosts"].as_array().unwrap().len(), 1);
    assert_eq!(
        report["hosts"][0]["shared_by"],
        serde_json::json!(["openai", "claude"])
    );
    assert!(!temporary.path().join(".claude").exists());
}

#[test]
fn legacy_status_update_and_explicit_removal_keep_the_existing_policy() {
    let corpus = corpus();
    let temporary = TempDir::new().unwrap();
    let root = temporary.path();
    let legacy = ".claude/skills/chough-workflow";
    write_bytes(
        &root.join(legacy).join("SKILL.md"),
        &corpus.current["SKILL.md"],
    );
    write_bytes(
        &root.join(legacy).join("references/build.md"),
        &corpus.releases["v0.2.0"]["references/build.md"],
    );
    let mut edited = corpus.current["references/plan.md"].clone();
    edited.extend_from_slice(b"\nUSER EDIT\n");
    write_bytes(&root.join(legacy).join("references/plan.md"), &edited);
    write_bytes(
        &root.join(legacy).join("private-unknown.md"),
        b"keep unknown\n",
    );
    let before = snapshot(&root.join(legacy));
    let refusal = cli_output(
        &corpus,
        root,
        &["skill", "remove-legacy", "--host", "claude"],
    );
    assert!(!refusal.status.success());
    assert!(String::from_utf8_lossy(&refusal.stderr).contains("shared copy"));
    assert_eq!(snapshot(&root.join(legacy)), before);
    let status = cli(&corpus, root, &["skill", "status", "--host", "claude"]);
    assert_eq!(
        row(&status["legacy"][0]["files"], "references/build.md")["state"],
        "known-outdated"
    );
    assert_eq!(snapshot(&root.join(legacy)), before);
    assert!(!root.join(HOSTS[0].1).exists());
    let _update = cli(&corpus, root, &["skill", "update", "--host", "claude"]);
    assert_eq!(
        snapshot(&root.join(legacy)),
        before,
        "update may not remove or rewrite legacy files"
    );
    let _removed = cli(
        &corpus,
        root,
        &["skill", "remove-legacy", "--host", "claude"],
    );
    assert!(!root.join(legacy).join("SKILL.md").exists());
    assert!(!root.join(legacy).join("references/build.md").exists());
    let after = snapshot(&root.join(legacy));
    let expected: BTreeMap<_, _> = before
        .into_iter()
        .filter(|(name, _)| name == "references/plan.md" || name == "private-unknown.md")
        .collect();
    assert_eq!(
        after, expected,
        "explicit removal keeps edited and unknown bytes/modes"
    );
}
