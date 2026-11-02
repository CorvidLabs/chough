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
    // Main 2756dc2 (PR #9); superseded by the timing report (REQ-cli-020).
    (
        "references/checkpoint.md",
        "943d2b5dd1cf092b618c154a20f11ad30ea5a5ecbada1b0f1bb2ab480f047fa2",
    ),
    (
        "references/plan.md",
        "8ecec7edffa34daa0082a026d6e07d4acc1245d6fc3dcba9e87d7296c226b8ab",
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
    // Main 2756dc2 (PR #9); superseded by the bundle hygiene change (#13): the two
    // templates held em dashes, and references/testing.md gained env, escape removal
    // and artifact guidance.
    (
        "templates/frd.md",
        "5647b56e21f6667d1ed97381bcbd67a7baf2f60dd8332abcb269e72d4759fd23",
    ),
    (
        "templates/cases.toml",
        "6c7ebaa10b099829bc0db4529e28286ded721b0dfa9b5635a3ae0f37d9d91d4a",
    ),
    (
        "references/testing.md",
        "eac153b213914bf71a84cb08cda5f0810730a324b112e3caf96ee3be2672b3e9",
    ),
    // Main 5af3f78 (PR #19) and main ee6e3d2 (0.1.0 docs sweep); superseded by the 0.1.0
    // release prep, the focused next change and Kimi host guidance (BUG-0021).
    (
        "SKILL.md",
        "d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c",
    ),
    // Main ee6e3d2; superseded by the focused next change's done-stage resume steps.
    (
        "references/stages.md",
        "5127182e148521e9c9df2490eb45055d0c5edb3302f245bc4f7aab114b591a24",
    ),
    (
        "references/validate.md",
        "5b7ea4ec1a12c745e7d5a14625f4f98ab2731bf13b588986e4f0a6b7686b4aed",
    ),
    (
        "references/testing.md",
        "a056931078182d3bd887cfa96c6bc4bd68a83b955e479bca95266bf1d1c1ee7c",
    ),
    (
        "references/checkpoint.md",
        "20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0",
    ),
    (
        "references/lifecycle.md",
        "a776c2f85d8803d78d24bd124773b73be1b09dd190c5cc9b6dd5a11fe4cc09c3",
    ),
    (
        "templates/cases.toml",
        "8a07e72f5f72668f6b440a14d457a8352850a94413ba7e1b5962abc5cdae234a",
    ),
    // Main 99a61ec (PR #23, focused next); superseded by the 0.1.0 release prep.
    (
        "SKILL.md",
        "78656f0d9ad32c55ee95f24f1abd42a3339ecff817dd5bf4782bab0d58182183",
    ),
    (
        "references/checkpoint.md",
        "33f4266b10dea45b674164c73ed0dc556857541f790c1db5c56390e7138f2c14",
    ),
    // Main 7357d41 (Kimi host cold start, #24); superseded by the 0.1.0 release prep.
    (
        "SKILL.md",
        "f479854d764ce83917a15889c16b9bf447c3c2e8e208224515f784ca1c57ad40",
    ),
    // Main fb6e487; superseded by the learning handoff change (lesson certainty labels,
    // decision prefixes, closeout and the shipped `verification_ignored_paths`).
    (
        "references/accept.md",
        "4e085b88eaa66a59fd6c8ce7d3acb1201b46292603a2cf13ab5333fc5ea85d86",
    ),
    (
        "references/bugs.md",
        "df42d472ae25b0ed07ad3beb1257ce77c3fe4e159feefe8e70e7c18ec03a6945",
    ),
    (
        "references/checkpoint.md",
        "28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf",
    ),
    (
        "references/librarian.md",
        "df33dfa9aa3db1355b47ef47a653e75ff4ae42a8c1de096bcd1718f6a4287c17",
    ),
    (
        "references/lifecycle.md",
        "e232af339e12254fcc735ffa87b72c36d0e80a8b3efc585cb7be627973d42235",
    ),
    // Main 66bef80 (safe checkpoint merges, #29); superseded when the learning handoff
    // change added decision prefixes and the handoff blocks to the same file.
    (
        "references/checkpoint.md",
        "70ec5dee386c4ef178abe76a45236072bea7342aa265192fd0210536fe2639e8",
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
