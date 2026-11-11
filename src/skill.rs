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
    // Issue #102: exact released source bytes, pinned in the offline release corpus.
    // v0.1.0 commit c03d41956ef1d9f8380f02034f03d42bf52bc174.
    // v0.2.0 commit edbc3c1adecebf6a1378a731965c3bfe8c8217df.
    // v0.3.0 commit 4d65542ff583ee2478f7d98c1dd4434a960e9dc1.
    // v0.4.0 commit 69950c7a9253230aeeee3bac0d0d6b580e291aad.
    (
        "templates/brief.md",
        "fcbc759ab4605a6dd8fffd4af4498783186e308c6d771284bb9a2626f6e0b4e8",
    ),
    // Issue #102: exact released source bytes, pinned in the offline release corpus.
    // v0.2.0 commit edbc3c1adecebf6a1378a731965c3bfe8c8217df.
    (
        "references/accept.md",
        "d2a185514ebe6f7428ed2d31c946126e6201bfd1e0feec5e0dccd1250474aa9c",
    ),
    (
        "references/plan.md",
        "a159d2c4846ca0441337be0716fdd7c7fd96da27e7aea0b0b6e3dfb3b0f9696f",
    ),
    (
        "references/testing.md",
        "ef831afa1094da91e5766c2d9597d6c933abe11b4d1a50913893415bd0d511a4",
    ),
    // Issue #102: exact released source bytes, pinned in the offline release corpus.
    // v0.5.0 commit 455cb643cd91a9c804afdf5566e3889abf1bde49.
    (
        "references/accept.md",
        "18299d9d1adb30fb87a167a1a59ecd8595d1f53ec73522ccee257ea817641701",
    ),
    (
        "references/lifecycle.md",
        "ac787118e30b11a8b5eb1429108bf1ac42911767b1ff33c56d68e925c84f11a3",
    ),
    // Releases 0.1.0 to 0.4.0, unchanged since #22 through main 01f1d023; superseded by
    // the `stage advance --to` row in the transition table (REQ-cli-052, DRIVE-9).
    (
        "references/stages.md",
        "658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f",
    ),
    // Main ec830cf, release 0.4.0 for SKILL.md, plan.md and checkpoint.md; superseded
    // by relayed decisions at gates (#123, D7).
    (
        "SKILL.md",
        "733346e29c7a57461926a6c7d297face74fc291a2e5c7f17d4d57dcf3945ec99",
    ),
    (
        "references/plan.md",
        "1a0a324a83658a1466ee6fe295ccfe9e980275f7abca809ef2927e71fb912663",
    ),
    (
        "references/accept.md",
        "b9609ee86c0fb1c86edba9c76dad00a166be2c0329c7e920c92af551bbfeaa7f",
    ),
    (
        "references/lifecycle.md",
        "432b35393a7be32e7b5228a8afad6f9b40c3cd34c8d8ba889b6b597a17befe60",
    ),
    (
        "references/checkpoint.md",
        "370386488c8e5ff6c974ae9d3bd51998de31183868dfe48af44c0935a3019170",
    ),
    // Release 0.4.0; superseded by the archive-first closeout (#122, D5).
    (
        "references/lifecycle.md",
        "1c9ef93185390ee98aa4b4f2f9fbf2c442a003ad22097d0e74e4676b42404909",
    ),
    (
        "references/accept.md",
        "3da19999ee4830a1fe1e0af63b13efc934f90d05678bbf70c3d79f39103c1150",
    ),
    // Main 9a8b9b1 checkpoint guidance, before immutable history adoption.
    (
        "references/checkpoint.md",
        "c21ddc44653079aaa61553c2a31389b4b942925267d4d9c847b09646e90fcf8b",
    ),
    // Main 3d00e44 includes case ordering; the combined authority references differ.
    (
        "references/plan.md",
        "5fd009510e2ef60d726417a11a2ac3f3a258826ad80d0be5d8b8001c035c0784",
    ),
    (
        "references/accept.md",
        "0d44a367560c8a342c288116e904ecd9d3dcc0a4c0fa702acfbe83ff3b5ceb40",
    ),
    // Main 825a380 (PR #36); superseded by the agent-operated native workflow.
    (
        "SKILL.md",
        "6b81882bc2e8305dc1ae0181b1eb52ad9d586c0a9a6990e03e865d0de4149198",
    ),
    (
        "references/lifecycle.md",
        "14fd2277d5e3f97f24fe6b995b1815eff1cfcd7b49d03669919bdcc7290c2200",
    ),
    (
        "references/plan.md",
        "1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d",
    ),
    (
        "references/accept.md",
        "80f71366ad81e1720bdc5f91d51c5428cd6a60729c9d1675977e284f7b08e398",
    ),
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
    // Main 2756dc2 (PR #9); superseded by the timing report (REQ-reporting-001).
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
    // v0.5.0 (commit 455cb643) and main through 88e53eba; superseded by the Chough 0.6.0
    // session folders and Timeline.
    (
        "references/checkpoint.md",
        "b7371621899a8926447873b1f80db3f64cc51361544251099793a1e1f79760e3",
    ),
    // v0.4.0, v0.5.0 (commit 455cb643) and main through plan step 8's base; superseded
    // by the 0.6.0 vault guidance: index.md, the journal and lessons (REQ-cli-044).
    (
        "references/librarian.md",
        "ba9e89ee55dd51f7ac477a52c27ce1d641272a1fabc9be3fce150fa6321f47b8",
    ),
    // Main 66bef80 (safe checkpoint merges, #29); superseded when the learning handoff
    // change added decision prefixes and the handoff blocks to the same file.
    (
        "references/checkpoint.md",
        "70ec5dee386c4ef178abe76a45236072bea7342aa265192fd0210536fe2639e8",
    ),
    // Identity predecessor bundle digests retained across integration.
    (
        "templates/cases.toml",
        "5f839a81742b9dc3df3b1278e340061a08e95e2a00e203f0f2554eab3aa3fed9",
    ),
    (
        "references/testing.md",
        "8ca2b70f36ee162638cafd63bcc97e8c9ed688b59c2b67b98d61ba3be1d28f37",
    ),
    (
        "references/validate.md",
        "51281ff6c9af4b939105cdf6ecf80d9dd11080dc8d682ecbba9d5b5c17d75c42",
    ),
    // Identity main's librarian reference, superseded by the learning closeout guidance.
    (
        "references/librarian.md",
        "4fa746a83405059803f3589579559dd2ac909d942e1fd8dd709158b488e5dc0d",
    ),
    // Main 118a895 (learning handoffs, #35); superseded by the resume step that asks an
    // agent to state the recorded handoff before its first edit.
    (
        "SKILL.md",
        "367a9873293e79ca40c9634e2b2a12c3223074aaec161c7444690c2d02e12199",
    ),
    // Main 825a380; superseded by the case recording order change (#11, BUG-0017): the
    // scoped change check runs before cases that link a new requirement, and running or
    // recording cases is never a tasks.md checkbox.
    (
        "references/testing.md",
        "426c2df68e8d25bfbdee4bf81b4deaa06eac4525b6dfad3c771311e0d6a4ac99",
    ),
    // Main 0363d4a; superseded when the claude host began sharing `.agents/skills`
    // (LIFE-12) and the separate `.claude/skills` copy became legacy.
    (
        "SKILL.md",
        "0b2c166176af684901005b230da159a4ce4cb926e57a59bf3f6b11d3debfdb3f",
    ),
    // Main 4daf861, unchanged since 4008346; superseded by the composability guidance.
    (
        "references/build.md",
        "e92e5a8fdc2e7b02ac6e01f34e15ee66501f7e3a312970069abb9e409aca167a",
    ),
    // Main edbc3c1 (v0.2.0); superseded by the sessions guidance (REQ-history-006).
    (
        "references/checkpoint.md",
        "00b3c3f4ffaaeeaaca6d829ff0277c9eb354ce0911f66f41a55639d27fc7d35d",
    ),
    // Main edbc3c1 (v0.2.0); superseded by the backfill guidance (REQ-cli-036).
    (
        "references/lifecycle.md",
        "be351d5c3a8c65bd46f4784875b0118f82e5ba4a5b97a7d364d9856636d3b711",
    ),
    // Main 91a1fdd (v0.2.0 bundle, unchanged since); superseded by the native-source
    // guidance (issue #100, LIFE-15): Chough's bounds never justify editing HI intent,
    // specs or SpecSync files.
    (
        "SKILL.md",
        "5d7f85aa72b3bba8e5d83b593556bcdee244939b31038ed3e6000783aac5c4ea",
    ),
    (
        "references/build.md",
        "4d0bf24ddd5e55493658339c76750a639a15997c4290a492cacda3550874f035",
    ),
    // Main f7e5764; superseded by the knowledge-directory guidance (REQ-cli-043): init
    // creates the folder, done refreshes it, and it is committed with the checkpoint.
    (
        "references/accept.md",
        "472396403a4f716f0183ec2c3495ac0ecf0dc412c1b61bb850cee767765e1f6a",
    ),
    (
        "references/librarian.md",
        "772b6900528a84ad0c76638ad91536865ab7d62883d05eb49a8f7841896c2740",
    ),
    // Main 8fbbbe7; superseded by the knowledge-vault guidance (REQ-cli-044).
    (
        "references/librarian.md",
        "452a33f87dfb0c729ac98c790dbac47f811ae474ef693d0792a204bddf5e6772",
    ),
    // #107; superseded by the observation-reduction guidance (REQ-cli-033, REQ-cli-045).
    (
        "references/librarian.md",
        "1a5274fb0e08800f0f11efa530ed2c68422d2cb818ded653714e3e489fd8f762",
    ),
    // #108; superseded by the vault-attachments guidance (REQ-cli-046).
    (
        "references/librarian.md",
        "b9887069d8d37460e96c2fb73da125dedb61bae1f6af4c48fa37becf351349ae",
    ),
    // Main 615b587b; superseded by the docs-accuracy corrections (FLOW-18): where the
    // checkpoint and transitions live with sessions and event history, and SpecSync 6.2.
    (
        "SKILL.md",
        "860cc96169ac2c63de5f60aae1fc7fb417c99a799746a208fcedb968ad6039d5",
    ),
    (
        "references/checkpoint.md",
        "40afaeaf8494d52c4492861f9983fba0babddbe987673ba4981046aea0218016",
    ),
    (
        "references/lifecycle.md",
        "9f989fd4b2a9a9b1edd7288a6385a12e4dcaeba324e14ad5308c68233e2d1644",
    ),
    // Releases 0.1.0 to 0.4.0, unchanged since #2 through main f41e7ab4; superseded by
    // the `[hidden]` guard that keeps the prototype's state panels hidden (#58, FLOW-4).
    (
        "templates/prototype/styles.css",
        "ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239",
    ),
];

/// Host names and project skill directories, matching the CorvidLabs skills installer.
pub(super) const HOSTS: &[(&str, &str)] = &[
    ("openai", ".agents/skills"),
    // Claude shares the one `.agents/skills` copy (LIFE-12). Claude Code 2.1.316 was
    // observed not to discover `.agents/skills` alone; CLAUDE.md routes it instead.
    ("claude", ".agents/skills"),
    ("codex", ".codex/skills"),
    ("cursor", ".cursor/skills"),
    ("gemini", ".gemini/skills"),
    ("grok", ".grok/skills"),
];

/// Host directories that earlier helpers wrote and this one no longer installs. A copy
/// found there is reported as legacy and removed only by `skill remove-legacy` (LIFE-13).
const LEGACY: &[(&str, &str)] = &[("claude", ".claude/skills")];

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
    /// Remove unedited files of a legacy host copy, such as `.claude/skills`, keeping edits.
    RemoveLegacy {
        #[arg(long = "host")]
        hosts: Vec<String>,
    },
}

/// Requested hosts with duplicates of an already listed directory dropped, so hosts
/// that share a directory get one copy.
fn distinct(hosts: &[String]) -> Result<Vec<String>> {
    let mut seen = vec![];
    let mut kept = vec![];
    for host in hosts {
        let base = directory(host)?;
        if !seen.contains(&base) {
            seen.push(base);
            kept.push(host.clone());
        }
    }
    Ok(kept)
}

fn legacy_directory(host: &str) -> Result<String> {
    LEGACY
        .iter()
        .find(|(name, _)| *name == host)
        .map(|(_, path)| format!("{path}/chough-workflow"))
        .ok_or_else(|| {
            let names: Vec<&str> = LEGACY.iter().map(|(name, _)| *name).collect();
            anyhow::anyhow!(
                "no legacy copy is known for host {host}; legacy hosts: {}",
                names.join(", ")
            )
        })
}

/// Relative paths of every file under a directory, `/`-separated and sorted.
fn files_under(root: &Path, base: &str) -> Result<Vec<String>> {
    fn walk(path: &Path, prefix: &str, out: &mut Vec<String>) -> Result<()> {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let relative = if prefix.is_empty() {
                name
            } else {
                format!("{prefix}/{name}")
            };
            let kind = entry.file_type()?;
            if kind.is_dir() {
                walk(&entry.path(), &relative, out)?;
            } else {
                out.push(relative);
            }
        }
        Ok(())
    }
    let mut out = vec![];
    walk(&local_path(root, base)?, "", &mut out)?;
    out.sort();
    Ok(out)
}

/// The state of each file present in a legacy copy: current, known-outdated, edited
/// or unknown (not a bundle file).
fn legacy_files(root: &Path, base: &str) -> Result<Vec<(String, &'static str)>> {
    let mut states = vec![];
    for name in files_under(root, base)? {
        let state = match BUNDLE.iter().find(|(bundled, _)| *bundled == name) {
            Some((_, content)) => file_state(root, base, &name, content)?,
            None => "unknown",
        };
        states.push((name, state));
    }
    Ok(states)
}

fn legacy_hosts(root: &Path, requested: &[String]) -> Result<Vec<String>> {
    if !requested.is_empty() {
        for host in requested {
            legacy_directory(host)?;
        }
        return Ok(requested.to_vec());
    }
    let mut hosts = vec![];
    for (host, _) in LEGACY {
        if local_path(root, &legacy_directory(host)?)?.is_dir() {
            hosts.push((*host).to_string());
        }
    }
    Ok(hosts)
}

/// Installed legacy copies, reported and never written.
fn legacy_report(root: &Path) -> Result<Vec<Value>> {
    let mut reports = vec![];
    for host in legacy_hosts(root, &[])? {
        let base = legacy_directory(&host)?;
        let files: Vec<Value> = legacy_files(root, &base)?
            .into_iter()
            .map(|(file, state)| json!({"file": file, "state": state}))
            .collect();
        reports.push(json!({
            "host": host,
            "directory": base,
            "shared": directory(&host)?,
            "files": files,
            "remove": format!("chough skill remove-legacy --host {host} removes current and known-outdated files and keeps edited or unknown ones"),
        }));
    }
    Ok(reports)
}

/// Remove a legacy copy's unedited bundle files, then directories left empty.
fn remove_legacy(root: &Path, requested: &[String]) -> Result<Value> {
    let hosts = legacy_hosts(root, requested)?;
    ensure!(!hosts.is_empty(), "no legacy skill copy found");
    for host in &hosts {
        let shared = format!("{}/SKILL.md", directory(host)?);
        ensure!(
            local_path(root, &shared)?.is_file(),
            "the shared copy {} is missing; install it with `chough skill update --host {host}` (or `chough init --host {host}`) before removing the legacy copy; nothing was removed",
            directory(host)?
        );
    }
    let mut reports = vec![];
    for host in &hosts {
        let base = legacy_directory(host)?;
        if !local_path(root, &base)?.is_dir() {
            reports.push(json!({"host": host, "directory": base, "removed": [], "kept": []}));
            continue;
        }
        let mut removed = vec![];
        let mut kept = vec![];
        for (file, state) in legacy_files(root, &base)? {
            if matches!(state, "current" | "known-outdated") {
                fs::remove_file(local_path(root, &format!("{base}/{file}"))?)?;
                removed.push(file);
            } else {
                kept.push(file);
            }
        }
        // Remove directories left empty, deepest first, up to the host directory.
        let top = base.split('/').next().unwrap_or_default().to_string();
        let mut directories: Vec<String> = vec![];
        for file in &removed {
            let mut parts: Vec<&str> = file.split('/').collect();
            parts.pop();
            while !parts.is_empty() {
                directories.push(format!("{base}/{}", parts.join("/")));
                parts.pop();
            }
        }
        let mut parent = base.clone();
        while parent != top {
            directories.push(parent.clone());
            match parent.rfind('/') {
                Some(index) => parent.truncate(index),
                None => break,
            }
        }
        directories.push(top);
        directories.sort_by_key(|path| std::cmp::Reverse(path.matches('/').count()));
        directories.dedup();
        for path in directories {
            let full = local_path(root, &path)?;
            if full.is_dir() && fs::read_dir(&full)?.next().is_none() {
                fs::remove_dir(&full)?;
            }
        }
        reports.push(json!({"host": host, "directory": base, "removed": removed, "kept": kept}));
    }
    Ok(json!({
        "schema_version": 1,
        "legacy": reports,
        "note": "Only files byte-identical to the current or a known earlier bundle were removed; edited and unknown files were kept.",
    }))
}

/// Paths the installation would create, validated before any file is written.
pub(super) fn planned_files(hosts: &[String]) -> Result<Vec<(String, &'static str)>> {
    let mut files = vec![];
    for host in &distinct(hosts)? {
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
        return distinct(&requested);
    }
    let mut hosts = vec![];
    for (host, _) in HOSTS {
        if local_path(root, &directory(host)?)?.is_dir() {
            hosts.push((*host).to_string());
        }
    }
    distinct(&hosts)
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
        let sharing: Vec<&str> = HOSTS
            .iter()
            .filter(|(_, path)| format!("{path}/chough-workflow") == base)
            .map(|(name, _)| *name)
            .collect();
        reports
            .push(json!({"host": host, "directory": base, "shared_by": sharing, "files": files}));
    }
    Ok(json!({
        "schema_version": 1,
        "bundle_files": BUNDLE.len(),
        "hosts": reports,
        "legacy": legacy_report(root)?,
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
        SkillCommand::RemoveLegacy { hosts } => print_json(&remove_legacy(root, &hosts)?),
    }
}
