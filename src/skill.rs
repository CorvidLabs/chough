//! The embedded workflow skill and its per-host project installs.

use crate::records::{digest_bytes, local_path, print_json, write_new, write_replace};
use anyhow::{Result, ensure};
use clap::Subcommand;
use serde_json::{Value, json};
use std::fs;
use std::path::Path;

macro_rules! bundle {
    ($($name:literal),* $(,)?) => {
        &[$(($name, include_str!(concat!("../skills/chough-workflow/", $name)))),*]
    };
}

/// Every file of the canonical bundle, embedded at build time.
pub(super) const BUNDLE: &[(&str, &str)] = bundle!(
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
);

/// SHA-256 digests of files shipped by earlier releases; such copies may be updated.
const KNOWN_PREVIOUS: &[(&str, &str)] = &[
    (
        "SKILL.md",
        "6eb4980c784757c2898ce871551282fe251a5c0715dd625e95806cff2be7f06a",
    ),
    (
        "references/lifecycle.md",
        "4b2d2c8246ca5bc6f2b9b37c9f608f35b767190629f274c45d97b8e6ad812e08",
    ),
    (
        "references/bugs.md",
        "83714902fba86e418dcf29045127fb6f619af6d2168b8f1e226cba7c45736b33",
    ),
    (
        "agents/openai.yaml",
        "fbe806654af0b04c0befc6f2e16fb006f0b65f87318d84e7d844a426615ec1dd",
    ),
    // Main f19823d (PR #2 and #4); superseded by PR #7.
    (
        "references/lifecycle.md",
        "c85f605af49b3615281a0509171340e894fff6e0cebce1bc76a7d39612887ba6",
    ),
    (
        "references/stages.md",
        "3a6d304596b52c88fe33b45ce28ae70701c2c075511e81da18f0d006adc2575a",
    ),
    // Main 9cb1c35 (PR #7), unchanged since 4008346; superseded by this release candidate.
    (
        "references/checkpoint.md",
        "569ff9df89f3564bd1b91c8f231cc6b56af377ab8e2411c57cf052bf5aff71aa",
    ),
    (
        "references/testing.md",
        "26029a2ee652ddc55b2ef6a68ed3edfa6a699f3a61a8fffa8d4d485f1180f4fb",
    ),
    // Main 2756dc2 (PR #9); superseded by the evidence freshness change.
    (
        "references/accept.md",
        "3dea3612392c3f02757bbe53d4e329f054dea587d931c437246e7c83083f3252",
    ),
    (
        "references/lifecycle.md",
        "c751bb4d8f624ca6c9d1b8f01247feb44a4d380c81c0bcf0362113818816547f",
    ),
    (
        "references/bugs.md",
        "a5497e45df7d2f7593f85101e47062d6bc605bfd1389ac27084d9f6ab1479b32",
    ),
];

/// Host names and project skill directories, matching the CorvidLabs skills installer.
pub(super) const HOSTS: &[(&str, &str)] = &[
    ("openai", ".agents/skills"),
    ("claude", ".claude/skills"),
    ("codex", ".codex/skills"),
    ("cursor", ".cursor/skills"),
    ("gemini", ".gemini/skills"),
    ("grok", ".grok/skills"),
];

pub(super) fn host_names() -> Vec<&'static str> {
    HOSTS.iter().map(|(name, _)| *name).collect()
}

fn directory(host: &str) -> Result<String> {
    HOSTS
        .iter()
        .find(|(name, _)| *name == host)
        .map(|(_, path)| format!("{path}/chough-workflow"))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "unknown host {host}; supported: {}",
                host_names().join(", ")
            )
        })
}

#[derive(Subcommand)]
pub(super) enum SkillCommand {
    /// Compare installed copies with this binary's bundle.
    Status {
        #[arg(long = "host")]
        hosts: Vec<String>,
    },
    /// Add missing files and replace only unmodified files from a known earlier bundle.
    Update {
        #[arg(long = "host")]
        hosts: Vec<String>,
    },
}

/// Paths the installation would create, validated before any file is written.
pub(super) fn planned_files(hosts: &[String]) -> Result<Vec<(String, &'static str)>> {
    let mut files = vec![];
    for host in hosts {
        let base = directory(host)?;
        for (name, content) in BUNDLE {
            files.push((format!("{base}/{name}"), *content));
        }
    }
    Ok(files)
}

fn file_state(root: &Path, base: &str, name: &str, content: &str) -> Result<&'static str> {
    let path = local_path(root, &format!("{base}/{name}"))?;
    let Ok(bytes) = fs::read(&path) else {
        return Ok("missing");
    };
    if bytes == content.as_bytes() {
        return Ok("current");
    }
    let digest = digest_bytes(&bytes);
    Ok(
        if KNOWN_PREVIOUS
            .iter()
            .any(|(known, previous)| *known == name && *previous == digest)
        {
            "known-outdated"
        } else {
            "edited"
        },
    )
}

fn installed_hosts(root: &Path, requested: Vec<String>) -> Result<Vec<String>> {
    if !requested.is_empty() {
        for host in &requested {
            directory(host)?;
        }
        return Ok(requested);
    }
    let mut hosts = vec![];
    for (host, _) in HOSTS {
        if local_path(root, &directory(host)?)?.is_dir() {
            hosts.push((*host).to_string());
        }
    }
    Ok(hosts)
}

fn report(root: &Path, hosts: &[String], update: bool) -> Result<Value> {
    let mut reports = vec![];
    for host in hosts {
        let base = directory(host)?;
        let mut files = vec![];
        for (name, content) in BUNDLE {
            let before = file_state(root, &base, name, content)?;
            let action = match (update, before) {
                (true, "missing") => {
                    ensure!(write_new(root, &format!("{base}/{name}"), content)?);
                    "added"
                }
                (true, "known-outdated") => {
                    write_replace(root, &format!("{base}/{name}"), content)?;
                    "updated"
                }
                (true, "edited") => "preserved-edit",
                _ => "none",
            };
            files.push(json!({"file": name, "state": before, "action": action}));
        }
        reports.push(json!({"host": host, "directory": base, "files": files}));
    }
    Ok(json!({
        "schema_version": 1,
        "bundle_files": BUNDLE.len(),
        "hosts": reports,
        "note": "Edited files are never replaced. A host directory is a convention; check that the host actually discovers it.",
    }))
}

pub(super) fn run(root: &Path, command: SkillCommand) -> Result<()> {
    match command {
        SkillCommand::Status { hosts } => {
            let hosts = installed_hosts(root, hosts)?;
            print_json(&report(root, &hosts, false)?)
        }
        SkillCommand::Update { hosts } => {
            let hosts = installed_hosts(root, hosts)?;
            ensure!(
                !hosts.is_empty(),
                "no installed skill found; run `chough init --host <host>`"
            );
            print_json(&report(root, &hosts, true)?)
        }
    }
}
