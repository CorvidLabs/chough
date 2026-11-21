//! Optional GitHub stage labels (REQ-cli-070, LIFE-31).

mod common;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

fn read(relative: &str) -> String {
    fs::read_to_string(root().join(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The seven main stages, in order, from `const ORDER` in `src/stage.rs`.
fn main_stages() -> Vec<String> {
    let text = read("src/stage.rs");
    let start = text
        .find("const ORDER: [&str; 7] = [")
        .expect("src/stage.rs declares const ORDER");
    let end = text[start..].find("];").expect("const ORDER ends");
    let names: Vec<String> = text[start..start + end]
        .split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, name)| name.to_string())
        .collect();
    assert_eq!(
        names.len(),
        7,
        "const ORDER should name seven stages, got {names:?}"
    );
    names
}

fn workflow_rows(text: &str) -> Vec<(String, String)> {
    text.lines()
        .filter_map(|line| {
            let line = line.trim();
            let mut parts = line.split('|');
            let name = parts.next()?;
            let stage = name.strip_prefix("chough-stage:")?;
            let color = parts.next()?;
            let _description = parts.next()?;
            if parts.next().is_some() {
                panic!("label row has an extra field: {line}");
            }
            Some((stage.to_string(), color.to_string()))
        })
        .collect()
}

fn checkout_pin(text: &str) -> String {
    let line = text
        .lines()
        .find(|line| line.contains("actions/checkout@"))
        .expect("a workflow pins actions/checkout");
    let at = line.find("actions/checkout@").unwrap();
    line[at..].split_whitespace().next().unwrap().to_string()
}

#[test]
fn the_template_follows_the_main_stages_and_chough_runs_that_copy() {
    let stages = main_stages();
    let colors = [
        "FF0000", "FF4D00", "FF9900", "FFEE00", "00CC44", "00BBFF", "8C00FF",
    ];
    let workflow = read("integrations/github/chough-labels.yml");
    let rows = workflow_rows(&workflow);
    let got: Vec<String> = rows.iter().map(|(stage, _)| stage.clone()).collect();
    assert_eq!(got, stages, "workflow labels drifted from const ORDER");
    assert_eq!(
        rows.iter()
            .map(|(_, color)| color.as_str())
            .collect::<Vec<_>>(),
        colors
    );
    assert!(workflow.contains("chough-stage:implement"));
    assert!(workflow.contains("chough-stage:verify"));
    assert_eq!(
        checkout_pin(&workflow),
        checkout_pin(&read(".github/workflows/ci.yml"))
    );

    let script = read("integrations/github/chough-label-prs.sh");
    let accepted = script
        .lines()
        .find(|line| line.contains("intent|design|plan|build|test|accept|done"))
        .expect("the script names the seven main stages");
    let accepted = accepted.split(')').next().unwrap();
    assert_eq!(
        accepted.split('|').map(str::trim).collect::<Vec<_>>(),
        stages.iter().map(String::as_str).collect::<Vec<_>>()
    );
    assert!(
        script.contains("/^stage:/"),
        "the script reads stage: from the session front matter"
    );

    assert_eq!(
        fs::read(root().join("integrations/github/chough-labels.yml")).unwrap(),
        fs::read(root().join(".github/workflows/chough-labels.yml")).unwrap(),
        "the installed workflow drifted from the template"
    );
    assert_eq!(
        fs::read(root().join("integrations/github/chough-label-prs.sh")).unwrap(),
        fs::read(root().join("tools/chough-label-prs.sh")).unwrap(),
        "the installed script drifted from the template"
    );

    let readme = read("integrations/github/README.md");
    assert!(readme.contains("optional"));
    assert!(readme.contains("does not call GitHub"));
    assert!(readme.contains(".github/workflows/chough-labels.yml"));
    assert!(readme.contains("tools/chough-label-prs.sh"));
    let integrations = read("integrations/README.md");
    assert!(integrations.contains("github/README.md"));
    assert!(integrations.contains("does not call GitHub"));
    assert!(read("docs/integrations.md").contains("integrations/github/"));

    for path in files_under(&root().join("src")) {
        let text = fs::read_to_string(&path).unwrap();
        assert!(
            !text.contains("gh label") && !text.contains("chough-labels.yml"),
            "{} calls GitHub or installs the label workflow",
            path.display()
        );
    }
}

#[test]
fn chough_init_does_not_install_the_label_workflow() {
    let fixture = common::Fixture::new();
    fixture.init();
    assert!(
        !fixture
            .root()
            .join(".github/workflows/chough-labels.yml")
            .exists()
    );
}

#[test]
fn the_script_ignores_a_symlinked_session() {
    let (stdout, log) = run(
        |root| {
            git(root, &["checkout", "-b", "pr"]);
            let real = root.join("elsewhere.md");
            fs::write(&real, "---\nstage: build\n---\n# linked\n").unwrap();
            let session = root.join(".chough/sessions/one/session.md");
            fs::remove_file(&session).unwrap();
            std::os::unix::fs::symlink(&real, &session).unwrap();
            git(root, &["add", "."]);
            git(root, &["commit", "-m", "pr"]);
        },
        "",
    );
    assert!(
        stdout.contains("stage=none"),
        "a symlink must not supply a stage: {stdout}"
    );
    assert!(!log.contains("--add-label"), "{log}");
}

#[test]
fn the_workflow_does_not_recreate_matching_labels() {
    let workflow = read("integrations/github/chough-labels.yml");
    let installed = read(".github/workflows/chough-labels.yml");
    assert_eq!(
        workflow, installed,
        "the installed workflow drifted from the template"
    );
    assert_eq!(
        fs::read(root().join("integrations/github/chough-label-prs.sh")).unwrap(),
        fs::read(root().join("tools/chough-label-prs.sh")).unwrap(),
        "the installed script drifted from the template"
    );
    assert!(
        workflow.contains("github.event_name != 'pull_request' || github.event.pull_request.head.repo.full_name == github.repository"),
        "a fork pull request must skip the label job"
    );

    let rows = label_rows();
    let matching = label_table(&rows);
    let with_implement = format!("{matching}chough-stage:implement\t000000\told\n");
    let log = sync_labels(&rows, &with_implement);
    assert!(log.contains("label delete chough-stage:implement"), "{log}");
    assert!(!log.contains("label delete chough-stage:verify"), "{log}");
    assert!(!log.contains("label create"), "{log}");

    let missing_build = matching
        .lines()
        .filter(|line| !line.starts_with("chough-stage:build\t"))
        .collect::<Vec<_>>()
        .join("\n");
    let log = sync_labels(&rows, &format!("{missing_build}\n"));
    assert!(
        log.contains("label create chough-stage:build") && !log.contains("--force"),
        "{log}"
    );
    assert!(!log.contains("label create chough-stage:intent"), "{log}");

    let wrong_color = matching.replace(
        "chough-stage:build\tFFEE00\t",
        "chough-stage:build\tFFFFFF\t",
    );
    let log = sync_labels(&rows, &wrong_color);
    assert!(
        log.contains("label create chough-stage:build") && log.contains("--force"),
        "{log}"
    );
    assert!(!log.contains("label create chough-stage:intent"), "{log}");
}

fn label_rows() -> String {
    let workflow = read("integrations/github/chough-labels.yml");
    workflow
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.starts_with("chough-stage:")
                && line.contains('|')
                && !line.contains("implement")
                && !line.contains("verify")
            {
                Some(line.to_string())
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn label_table(rows: &str) -> String {
    rows.lines()
        .map(|line| line.replace('|', "\t"))
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

fn sync_labels(rows: &str, existing: &str) -> String {
    let body = label_sync_body();
    let tmp = tempfile::tempdir().unwrap();
    let bin = tmp.path().join("bin");
    fs::create_dir_all(&bin).unwrap();
    let gh = bin.join("gh");
    fs::write(
        &gh,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$GH_LOG\"\ncase \"$*\" in\n  *'label list'*) cat \"$GH_LABELS\" ;;\nesac\nexit 0\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&gh).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&gh, perms).unwrap();
    }
    let log = tmp.path().join("gh.log");
    let labels = tmp.path().join("labels.txt");
    fs::write(&labels, existing).unwrap();
    let path = std::env::var("PATH").unwrap_or_default();
    let output = Command::new("bash")
        .arg("-c")
        .arg(format!("set -euo pipefail\n{body}"))
        .env("PATH", format!("{}:{path}", bin.display()))
        .env("GH_LOG", &log)
        .env("GH_LABELS", &labels)
        .env("LABEL_ROWS", rows)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "label sync failed: {}\n{}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
    fs::read_to_string(&log).unwrap_or_default()
}

fn label_sync_body() -> String {
    let workflow = read("integrations/github/chough-labels.yml");
    let marker = "# label-sync: begin\n";
    let start = workflow
        .find(marker)
        .expect("workflow marks the label sync")
        + marker.len();
    let rest = &workflow[start..];
    let end = rest
        .find("# label-sync: end")
        .expect("workflow ends the label sync");
    rest[..end].to_string()
}

#[test]
fn the_script_labels_one_main_stage_and_refuses_anything_else() {
    let (stdout, log) = label("build", &[], "");
    assert!(stdout.contains("stage=build"), "{stdout}");
    assert!(log.contains("--add-label chough-stage:build"), "{log}");
    assert!(!log.contains("--remove-label"), "{log}");

    let (stdout, log) = label("accept", &[], "");
    assert!(stdout.contains("stage=accept"), "{stdout}");
    assert!(log.contains("chough-stage:accept"), "{log}");

    let (stdout, log) = label("implement", &[], "chough-stage:implement\n");
    assert!(stdout.contains("not a main stage"), "{stdout}");
    assert!(
        log.contains("--remove-label chough-stage:implement"),
        "{log}"
    );
    assert!(!log.contains("--add-label"), "{log}");

    let (stdout, log) = label("build", &["design"], "");
    assert!(stdout.contains("multiple distinct stages"), "{stdout}");
    assert!(!log.contains("--add-label"), "{log}");

    let (stdout, log) = label_readme_only();
    assert!(stdout.contains("stage=none"), "{stdout}");
    assert!(!stdout.contains("multiple"), "{stdout}");
    assert!(!log.contains("--add-label"), "{log}");
}

#[test]
fn the_script_follows_an_open_session_ahead_of_an_archived_one() {
    let (stdout, log) = run(
        |root| {
            git(root, &["checkout", "-b", "pr"]);
            fs::create_dir_all(root.join(".chough/archive/sessions")).unwrap();
            fs::write(
                root.join(".chough/archive/sessions/old.md"),
                "---\nstage: done\n---\n# old\n",
            )
            .unwrap();
            fs::write(
                root.join(".chough/sessions/one/session.md"),
                "---\nstage: intent\n---\n# new\n",
            )
            .unwrap();
            git(root, &["add", "."]);
            git(root, &["commit", "-m", "pr"]);
        },
        "chough-stage:done\n",
    );
    assert!(
        stdout.contains("stage=intent"),
        "the open session sets the label: {stdout}"
    );
    assert!(log.contains("--remove-label chough-stage:done"), "{log}");
    assert!(log.contains("--add-label chough-stage:intent"), "{log}");
    assert!(!log.contains("--add-label chough-stage:done"), "{log}");

    let (stdout, log) = run(
        |root| {
            git(root, &["checkout", "-b", "pr"]);
            fs::create_dir_all(root.join(".chough/archive/sessions")).unwrap();
            fs::write(
                root.join(".chough/archive/sessions/old.md"),
                "---\nstage: done\n---\n# old\n",
            )
            .unwrap();
            git(root, &["add", "."]);
            git(root, &["commit", "-m", "pr"]);
        },
        "",
    );
    assert!(
        stdout.contains("stage=done"),
        "an archived session still labels a pull request with no open session: {stdout}"
    );
    assert!(log.contains("--add-label chough-stage:done"), "{log}");

    let (stdout, log) = run(
        |root| {
            git(root, &["checkout", "-b", "pr"]);
            fs::create_dir_all(root.join(".chough/archive/sessions")).unwrap();
            fs::write(
                root.join(".chough/archive/sessions/old.md"),
                "---\nstage: done\n---\n# old\n",
            )
            .unwrap();
            fs::write(
                root.join(".chough/sessions/one/session.md"),
                "---\nstage: bugs\n---\n# side\n",
            )
            .unwrap();
            git(root, &["add", "."]);
            git(root, &["commit", "-m", "pr"]);
        },
        "chough-stage:done\n",
    );
    assert!(
        stdout.contains("not a main stage"),
        "an open side stage does not fall back to the archive: {stdout}"
    );
    assert!(log.contains("--remove-label chough-stage:done"), "{log}");
    assert!(!log.contains("--add-label"), "{log}");
}

#[test]
fn the_script_keeps_a_label_it_already_applied() {
    let (stdout, log) = label("build", &[], "chough-stage:build\n");
    assert!(stdout.contains("stage=build"), "{stdout}");
    assert!(
        !log.contains("--remove-label"),
        "the applied label must stay: {log}"
    );
    assert!(
        !log.contains("--add-label"),
        "an existing label must not be added again: {log}"
    );

    let (stdout, log) = label("build", &[], "chough-stage:build\nchough-stage:intent\n");
    assert!(stdout.contains("stage=build"), "{stdout}");
    assert!(log.contains("--remove-label chough-stage:intent"), "{log}");
    assert!(!log.contains("--remove-label chough-stage:build"), "{log}");
    assert!(!log.contains("--add-label"), "{log}");
}

/// `stage` is written on the pull request's introduced session. Each name in
/// `others` becomes another introduced session. `existing` is the labels `gh`
/// already reports.
fn label(stage: &str, others: &[&str], existing: &str) -> (String, String) {
    run(
        |root| {
            git(root, &["checkout", "-b", "pr"]);
            fs::write(
                root.join(".chough/sessions/one/session.md"),
                format!("---\nstage: \"{stage}\"\n---\n# one\n"),
            )
            .unwrap();
            for name in others {
                let dir = root.join(format!(".chough/sessions/{name}"));
                fs::create_dir_all(&dir).unwrap();
                fs::write(
                    dir.join("session.md"),
                    format!("---\nstage: {name}\n---\n# {name}\n"),
                )
                .unwrap();
            }
            git(root, &["add", "."]);
            git(root, &["commit", "-m", "pr"]);
        },
        existing,
    )
}

fn label_readme_only() -> (String, String) {
    run(
        |root| {
            git(root, &["checkout", "-b", "pr"]);
            fs::write(root.join("README.md"), "a readme edit\n").unwrap();
            git(root, &["add", "README.md"]);
            git(root, &["commit", "-m", "readme"]);
        },
        "",
    )
}

fn run(setup: impl FnOnce(&Path), existing: &str) -> (String, String) {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    git(root, &["init", "-b", "main"]);
    git(root, &["commit", "--allow-empty", "-m", "base"]);
    fs::create_dir_all(root.join(".chough/sessions/one")).unwrap();
    fs::write(
        root.join(".chough/sessions/one/session.md"),
        "---\nstage: intent\n---\n# one\n",
    )
    .unwrap();
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "session"]);
    setup(root);

    let bin = root.join("bin");
    fs::create_dir_all(&bin).unwrap();
    let gh = bin.join("gh");
    fs::write(
        &gh,
        "#!/bin/sh\nprintf '%s\\n' \"$*\" >> \"$GH_LOG\"\ncase \"$*\" in\n  *baseRefName*) printf '%s\\n' main ;;\n  *labels*) [ -f \"$GH_LABELS\" ] && cat \"$GH_LABELS\" ;;\nesac\nexit 0\n",
    )
    .unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&gh).unwrap().permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&gh, perms).unwrap();
    }
    let log = root.join("gh.log");
    let labels = root.join("labels.txt");
    fs::write(&labels, existing).unwrap();
    let path = std::env::var("PATH").unwrap_or_default();
    let output = Command::new("bash")
        .arg(root_script())
        .arg("7")
        .arg(root)
        .current_dir(root)
        .env("PATH", format!("{}:{path}", bin.display()))
        .env("GH_LOG", &log)
        .env("GH_LABELS", &labels)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "script failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    (
        String::from_utf8(output.stdout).unwrap(),
        fs::read_to_string(&log).unwrap_or_default(),
    )
}

fn root_script() -> PathBuf {
    root().join("integrations/github/chough-label-prs.sh")
}

fn git(dir: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(dir)
        .args(args)
        .env("GIT_AUTHOR_NAME", "Test")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn files_under(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut pending = vec![dir.to_path_buf()];
    while let Some(folder) = pending.pop() {
        for entry in fs::read_dir(&folder).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                pending.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files
}
