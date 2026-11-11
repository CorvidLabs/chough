//! `chough migrate` (REQ-records-009, REQ-records-010): carry a project's 0.5.x records
//! into the 0.6.0 tree. A preview writes nothing and ends with the plan's digest; an
//! apply needs that digest, no open native change, the human's consent and every file
//! it removes committed at HEAD. It writes by create-new only, reads each file back,
//! writes a receipt, and only then removes the 0.5.x files, restoring them if an
//! archive that read valid no longer does.
//!
//! Each record kind has a mapper, run in this order because each needs the one before:
//!
//! 1. sessions (t382, [`sessions_0_5`]): open sessions become session folders and
//!    archived ones (the rolled ones in `knowledge/attachments/` included) archived
//!    stories, each timeline holding its selected history lineage in order, the legacy
//!    log's entries in the session that adopted them; inbox entries move to
//!    `agent/inbox/`; shelved 0.5.x sessions are removed. It also says which
//!    observations each session references ([`SessionHome`]).
//! 2. bug links (t38b, [`bug_links_0_5`]): each 0.5.x bug's linked observation IDs.
//! 3. evidence (t37d, [`evidence_0_5`]): an observation an open session references
//!    becomes one of its evidence entries; one an archived session or a bug references
//!    becomes a receipt; others are removed. A PNG or JPEG artifact of a carried
//!    observation within the bounds of REQ-records-013 is kept beside its session or
//!    story; other artifacts are removed.
//! 4. bugs (t38b, [`bugs_0_5`]): Markdown bugs named by date and slug, every 0.5.x ID an
//!    alias, linked observations as receipts or `removed`, an open bug's fixture kept
//!    byte for byte.
//! 5. knowledge (t37j, [`knowledge_0_5`]): the generated and copied vault content of
//!    REQ-cli-044 removed with why.
//!
//! A mapper only reads and returns a [`Carried`]; it writes nothing and redacts
//! nothing. The framework redacts every written file (REQ-records-006), counting each
//! redaction by file, field and kind, builds the plan and its digest, and applies it.

use crate::leaks::{self, Local};
use crate::records::{local_path, print_json};
use anyhow::{Context, Result, bail};
use serde::Serialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// One 0.6.0 file the migration writes, create-new.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Written {
    /// Project-relative path.
    pub(crate) path: String,
    /// The bytes, before the framework's redaction.
    pub(crate) bytes: String,
}

/// What one mapper carries into 0.6.0.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct Carried {
    /// The 0.5.x files this mapper consumed; each is removed after the apply, once its
    /// bytes still match the plan.
    pub(crate) sources: BTreeSet<String>,
    /// The 0.6.0 files written.
    pub(crate) written: Vec<Written>,
    /// Each 0.5.x ID with the project-relative path of the record that now holds it as
    /// an alias (REQ-records-003), or `removed` when it was not carried.
    pub(crate) aliases: BTreeMap<String, String>,
    /// 0.5.x files removed without being carried, each with why.
    pub(crate) removed: BTreeMap<String, String>,
    /// Files kept byte for byte that hold a leak: listed as `leak` findings, never
    /// changed (REQ-records-009), each with the kinds found.
    pub(crate) kept_with_leaks: BTreeMap<String, Vec<String>>,
}

impl Carried {
    /// Add `other` to this one; a file two mappers write is refused.
    fn absorb(&mut self, other: Carried) -> Result<()> {
        for written in other.written {
            if self.written.iter().any(|known| known.path == written.path) {
                bail!(
                    "two mappers write {}, so the plan is refused; this is a Chough bug",
                    written.path
                );
            }
            self.written.push(written);
        }
        self.sources.extend(other.sources);
        self.aliases.extend(other.aliases);
        self.removed.extend(other.removed);
        self.kept_with_leaks.extend(other.kept_with_leaks);
        Ok(())
    }
}

/// Where a 0.5.x session went, by its 0.5.x key (the file stem, or `legacy` for
/// `.chough/session.json`).
#[derive(Clone, Debug, Serialize)]
pub(crate) struct SessionHome {
    /// The 0.6.0 session ID (`<date>-<slug>`).
    pub(crate) id: String,
    /// Open sessions keep a folder; archived ones become a story.
    pub(crate) archived: bool,
    /// The observation IDs its checkpoint references.
    pub(crate) observations: Vec<String>,
}

/// The receipt map of one carried observation, as a story's or a bug's `receipts:`
/// list holds it (REQ-history-006, with `observed` and `duration_seconds`).
pub(crate) type Receipt = Vec<(String, String)>;

/// Where each 0.5.x observation went: an open session's evidence entry (its path), or
/// a receipt (its map), by observation ID. An observation in neither was removed.
#[derive(Clone, Debug, Default)]
pub(crate) struct ObservationHomes {
    pub(crate) entries: BTreeMap<String, String>,
    pub(crate) receipts: BTreeMap<String, Receipt>,
}

/// The 0.5.x record locations whose presence makes a tree unmigrated (REQ-records-010):
/// files, folders whose contents count, and the folders whose `*.json` files count.
const LEGACY_FILES: [&str; 3] = [
    ".chough/session.json",
    ".chough/transitions.jsonl",
    ".chough/pruned.jsonl",
];
const LEGACY_FOLDERS: [&str; 3] = [
    ".chough/sessions/archive",
    ".chough/history",
    ".chough/evidence",
];
const LEGACY_JSON_IN: [&str; 2] = [".chough/sessions", ".chough/bugs"];

/// Every 0.5.x record path in the project, sorted (REQ-records-010).
pub(crate) fn legacy_paths(root: &Path) -> Result<Vec<String>> {
    let mut found = BTreeSet::new();
    for file in LEGACY_FILES {
        if local_path(root, file)?.is_file() {
            found.insert(file.to_string());
        }
    }
    for folder in LEGACY_FOLDERS {
        walk_files(root, folder, &mut found)?;
    }
    for folder in LEGACY_JSON_IN {
        let Ok(entries) = std::fs::read_dir(local_path(root, folder)?) else {
            continue;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.ends_with(".json") && entry.path().is_file() {
                found.insert(format!("{folder}/{name}"));
            }
        }
    }
    Ok(found.into_iter().collect())
}

/// Every file under `relative`, as project-relative paths.
fn walk_files(root: &Path, relative: &str, found: &mut BTreeSet<String>) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(local_path(root, relative)?) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let name = format!("{relative}/{}", entry.file_name().to_string_lossy());
        let kind = entry.file_type()?;
        if kind.is_dir() {
            walk_files(root, &name, found)?;
        } else if kind.is_file() {
            found.insert(name);
        }
    }
    Ok(())
}

#[derive(clap::Args)]
pub(super) struct MigrateArgs {
    /// Apply the plan whose digest a preview printed; without it, the plan is only
    /// printed and nothing is written.
    #[arg(long)]
    apply: Option<String>,
    /// Who consented to the migration, their words and where, recorded in the receipt
    /// as a relayed claim; or a standing `migration` preference gives it.
    #[arg(long, requires = "apply")]
    consent: Option<String>,
}

/// One redaction the plan makes: the file, the field and the kind, never the text.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
struct Redaction {
    file: String,
    field: String,
    kind: String,
}

/// The whole plan: what is read, written, redacted, kept and removed.
struct Plan {
    /// Every 0.5.x file with its SHA-256.
    sources: BTreeMap<String, String>,
    carried: Carried,
    redactions: Vec<Redaction>,
    follow_ups: Vec<String>,
}

impl Plan {
    /// The plan as printed, without its digest.
    fn view(&self) -> Value {
        let written: Vec<Value> = self
            .carried
            .written
            .iter()
            .map(|written| {
                json!({"path": written.path, "sha256": sha256(written.bytes.as_bytes()), "bytes": written.bytes.len()})
            })
            .collect();
        json!({
            "schema_version": 1,
            "sources": self.sources,
            "writes": written,
            "redactions": self.redactions,
            "removed": self.carried.removed,
            "kept_with_leaks": self.carried.kept_with_leaks,
            "aliases": self.carried.aliases,
            "follow_ups": self.follow_ups,
            "git_history": "Git history still holds what earlier commits recorded; this migration does not rewrite it.",
        })
    }

    /// The SHA-256 of the printed plan, which `--apply` must name.
    fn digest(&self) -> String {
        sha256(self.view().to_string().as_bytes())
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn run(root: &Path, args: MigrateArgs) -> Result<()> {
    let plan = plan(root)?;
    let digest = plan.digest();
    match args.apply {
        None => {
            let mut view = plan.view();
            view["plan_digest"] = json!(digest);
            view["apply"] = json!(format!(
                "chough migrate --apply {digest} --consent \"<who consented, their words and where>\""
            ));
            print_json(&view)
        }
        Some(_) => bail!(
            "`chough migrate --apply` is being built in plan step 10; nothing was written. Run `chough migrate` to preview the plan."
        ),
    }
}

/// Build the plan: run every mapper in order, redact what they write and list every
/// 0.5.x file read.
fn plan(root: &Path) -> Result<Plan> {
    let mut carried = Carried::default();
    let (sessions, homes) = sessions_0_5(root)?;
    carried.absorb(sessions)?;
    let links = bug_links_0_5(root)?;
    let (evidence, observations) = evidence_0_5(root, &homes, &links)?;
    carried.absorb(evidence)?;
    carried.absorb(bugs_0_5(root, &observations)?)?;
    carried.absorb(knowledge_0_5(root)?)?;
    carried
        .written
        .sort_by(|left, right| left.path.cmp(&right.path));
    let local = Local::current();
    let mut redactions = vec![];
    for written in &mut carried.written {
        redactions.extend(redact(local, written)?);
    }
    redactions.sort();
    let mut sources = BTreeMap::new();
    for path in legacy_paths(root)?
        .into_iter()
        .chain(carried.sources.iter().cloned())
    {
        let bytes = std::fs::read(local_path(root, &path)?)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        sources.insert(path, sha256(&bytes));
    }
    // REQ-records-014: the plan lists the missing verification exclusions as a
    // follow-up; Chough never writes SpecSync's policy.
    let follow_ups = crate::doctor::exclusions(root)
        .advice()
        .into_iter()
        .collect::<Vec<_>>();
    Ok(Plan {
        sources,
        carried,
        redactions,
        follow_ups,
    })
}

/// Redact every leak in a written file (REQ-records-006), returning each redaction by
/// field and kind. A Markdown record is redacted field by field and in its body; any
/// other file as a whole.
fn redact(local: &Local, written: &mut Written) -> Result<Vec<Redaction>> {
    let mut found = vec![];
    let mut note = |field: &str, text: &str| {
        for matched in leaks::find(local, text) {
            found.push(Redaction {
                file: written.path.clone(),
                field: field.to_string(),
                kind: matched.kind.name().to_string(),
            });
        }
    };
    if written.path.ends_with(".md")
        && let Ok(mut record) = crate::markdown::parse(&written.path, &written.bytes)
    {
        for (field, value) in record.fields() {
            note(&field, &value);
        }
        if !found.is_empty() {
            record.redact_leaks(local);
            written.bytes = record.render();
        }
    } else {
        note("content", &written.bytes.clone());
        if !found.is_empty() {
            written.bytes = leaks::redact(local, &written.bytes).0;
        }
    }
    Ok(found)
}

/// Mapper 1, sessions (t382). Being built; it carries nothing yet.
fn sessions_0_5(_root: &Path) -> Result<(Carried, BTreeMap<String, SessionHome>)> {
    Ok((Carried::default(), BTreeMap::new()))
}

/// Mapper 2, each 0.5.x bug's linked observation IDs (t38b).
fn bug_links_0_5(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    crate::bugs::links_0_5(root)
}

/// Mapper 3, evidence (t37d): replace this body with a call to
/// `crate::evidence::migrate_0_5(root, sessions, bug_links)`.
fn evidence_0_5(
    _root: &Path,
    _sessions: &BTreeMap<String, SessionHome>,
    _bug_links: &BTreeMap<String, Vec<String>>,
) -> Result<(Carried, ObservationHomes)> {
    Ok((Carried::default(), ObservationHomes::default()))
}

/// Mapper 4, bugs (t38b).
fn bugs_0_5(root: &Path, observations: &ObservationHomes) -> Result<Carried> {
    crate::bugs::migrate_0_5(root, observations)
}

/// Mapper 5, knowledge (t37j): replace this body with a call to
/// `crate::library::migrate_0_5(root)`.
fn knowledge_0_5(_root: &Path) -> Result<Carried> {
    Ok(Carried::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_written_record_is_redacted_field_by_field_and_counted() {
        let local = Local::new(Some("SYNTHETIC-user"), Some("SYNTHETIC-host"));
        let mut written = Written {
            path: ".chough/bugs/2026-11-11-x.md".into(),
            bytes: "---\nkind: bug\nid: 2026-11-11-x\ntitle: Broken at /Users/SYNTHETIC-user/p\n---\n# x\n".into(),
        };
        let found = redact(&local, &mut written).unwrap();
        assert_eq!(
            found,
            vec![Redaction {
                file: ".chough/bugs/2026-11-11-x.md".into(),
                field: "title".into(),
                kind: "home-path".into(),
            }]
        );
        assert!(
            written.bytes.contains("<redacted:home-path>"),
            "{}",
            written.bytes
        );
        assert!(!written.bytes.contains("/Users/"));
    }

    #[test]
    fn two_mappers_writing_one_file_are_refused() {
        let mut one = Carried::default();
        let file = Written {
            path: "a.md".into(),
            bytes: String::new(),
        };
        one.written.push(file.clone());
        let mut two = Carried::default();
        two.written.push(file);
        assert!(one.absorb(two).is_err());
    }
}
