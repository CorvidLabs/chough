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
//! The framework itself removes the 0.5.x files no mapper carries
//! ([`retired_files_0_5`]), keeps open bugs' fixtures, and accounts for every file of
//! [`accountable_paths`], so a file a mapper misses is reported as unaccounted.
//!
//! A mapper only reads and returns a [`Carried`]; it writes nothing and redacts
//! nothing. The framework redacts every written file (REQ-records-006), counting each
//! redaction by file, field and kind, builds the plan and its digest, and applies it.

use crate::leaks::{self, Local};
use crate::records::{local_path, print_json};
use anyhow::{Context, Result, bail, ensure};
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

/// One binary 0.6.0 file the migration writes, create-new: a key screenshot whose
/// metadata the mapper already removed (REQ-records-013). Never redacted as text.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Binary {
    /// Project-relative path.
    pub(crate) path: String,
    /// The exact bytes written.
    pub(crate) bytes: Vec<u8>,
}

/// One file the apply writes, with its final bytes.
#[derive(Clone, Debug)]
struct Planned {
    path: String,
    bytes: Vec<u8>,
}

/// What one mapper carries into 0.6.0.
#[derive(Clone, Debug, Default, Serialize)]
pub(crate) struct Carried {
    /// The 0.5.x files this mapper consumed; each is removed after the apply, once its
    /// bytes still match the plan.
    pub(crate) sources: BTreeSet<String>,
    /// The 0.6.0 text files written.
    pub(crate) written: Vec<Written>,
    /// The 0.6.0 binary files written.
    pub(crate) binaries: Vec<Binary>,
    /// Each 0.5.x ID with the project-relative path of the record that now holds it as
    /// an alias (REQ-records-003), or `removed` when it was not carried.
    pub(crate) aliases: BTreeMap<String, String>,
    /// 0.5.x files removed without being carried, each with why.
    pub(crate) removed: BTreeMap<String, String>,
    /// Files kept byte for byte that hold a leak: listed as `leak` findings, never
    /// changed (REQ-records-009), each with the kinds found.
    pub(crate) kept_with_leaks: BTreeMap<String, Vec<String>>,
    /// Each consumed 0.5.x file with the 0.6.0 files it becomes, where the mapper
    /// knows it beyond the alias of the file's own ID (REQ-records-009).
    pub(crate) becomes: BTreeMap<String, BTreeSet<String>>,
    /// What could not be carried and is not a file, such as history a 0.5.x retention
    /// pruned: each said in the plan and the receipt.
    pub(crate) gaps: Vec<String>,
}

impl Carried {
    /// Whether this one already writes `path`, as text or binary.
    fn writes(&self, path: &str) -> bool {
        self.written.iter().any(|known| known.path == path)
            || self.binaries.iter().any(|known| known.path == path)
    }

    /// Add `other` to this one; a file two mappers write is refused.
    fn absorb(&mut self, other: Carried) -> Result<()> {
        for path in other
            .written
            .iter()
            .map(|written| &written.path)
            .chain(other.binaries.iter().map(|binary| &binary.path))
        {
            if self.writes(path) {
                bail!("two mappers write {path}, so the plan is refused; this is a Chough bug");
            }
        }
        self.written.extend(other.written);
        self.binaries.extend(other.binaries);
        self.sources.extend(other.sources);
        // An ID has one home: a real home wins over `removed`, and two different homes
        // are refused rather than decided by mapper order (REQ-records-003).
        for (id, home) in other.aliases {
            match self.aliases.get(&id) {
                Some(known) if *known == home || home == "removed" => {}
                Some(known) if known != "removed" => bail!(
                    "two mappers alias {id}, to {known} and to {home}, so the plan is refused; this is a Chough bug"
                ),
                _ => {
                    self.aliases.insert(id, home);
                }
            }
        }
        self.removed.extend(other.removed);
        self.kept_with_leaks.extend(other.kept_with_leaks);
        self.gaps.extend(other.gaps);
        for (source, targets) in other.becomes {
            self.becomes.entry(source).or_default().extend(targets);
        }
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
/// `.chough/sessions/shelved/` held 0.5.x shelved sessions as `<key>.json`; left alone,
/// 0.6.0 would read the folder as a session's.
const LEGACY_FOLDERS: [&str; 4] = [
    ".chough/sessions/archive",
    ".chough/sessions/shelved",
    ".chough/history",
    ".chough/evidence",
];
const LEGACY_JSON_IN: [&str; 2] = [".chough/sessions", ".chough/bugs"];
/// How many 0.5.x paths an unmigrated refusal names before it counts the rest.
const NAMED_LEGACY_PATHS: usize = 5;

/// Refuse while any 0.5.x record exists, before anything is read or written
/// (REQ-records-010). Every command but `migrate`, `doctor` and `skill` calls this once,
/// so a mixed tree, 0.6.0 records with a 0.5.x file written later, refuses the same way.
/// Whether this process runs a command REQ-records-010 lets read a 0.5.x tree
/// (migrate, doctor, skill), so the guard is not repeated under the writer lock.
static GUARD_EXEMPT: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Mark this process as running a command the unmigrated guard exempts.
pub(crate) fn exempt_from_guard() {
    GUARD_EXEMPT.store(true, std::sync::atomic::Ordering::SeqCst);
}

/// The guard again, once a writer holds the checkpoint lock: a writer that passed the
/// guard in main while `migrate --apply` held the lock, and then restored the 0.5.x
/// files after an archive read invalid, refuses instead of writing into a mixed tree.
pub(crate) fn ensure_migrated_under_lock(root: &Path) -> Result<()> {
    if GUARD_EXEMPT.load(std::sync::atomic::Ordering::SeqCst) {
        return Ok(());
    }
    ensure_migrated(root)
}

pub(crate) fn ensure_migrated(root: &Path) -> Result<()> {
    let legacy = legacy_paths(root)?;
    if legacy.is_empty() {
        return Ok(());
    }
    let named = legacy
        .iter()
        .take(NAMED_LEGACY_PATHS)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(", ");
    let rest = match legacy.len().saturating_sub(NAMED_LEGACY_PATHS) {
        0 => String::new(),
        more => format!(" and {more} more"),
    };
    bail!(
        "refused:unmigrated: this project uses the Chough 0.5.x layout ({named}{rest}), so nothing was written. Run `chough migrate` to preview the conversion, then `chough migrate --apply <digest>` to convert."
    )
}

/// 0.5.x files REQ-records-009 removes or carries that REQ-records-010 does not count
/// as making a tree unmigrated: the schema 1 checkpoint backup, every file under
/// `.chough/artifacts/`, where an open bug's fixture rightly stays, and the validation
/// runs under `.chough/validation/`, whose `VAL-` IDs stay aliases (REQ-records-003).
const ACCOUNTED_FILES: [&str; 1] = [".chough/session.v1.json"];
const ACCOUNTED_FOLDERS: [&str; 2] = [".chough/artifacts", ".chough/validation"];

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

/// Every 0.5.x file the plan must carry, keep or remove (REQ-records-009): the paths of
/// [`legacy_paths`] and, while there are any, the files only the migration removes. A
/// migrated tree has none, so its kept fixtures are never planned again.
fn accountable_paths(root: &Path) -> Result<Vec<String>> {
    let mut found: BTreeSet<String> = legacy_paths(root)?.into_iter().collect();
    if found.is_empty() {
        return Ok(vec![]);
    }
    for file in ACCOUNTED_FILES {
        if local_path(root, file)?.is_file() {
            found.insert(file.to_string());
        }
    }
    for folder in ACCOUNTED_FOLDERS {
        walk_files(root, folder, &mut found)?;
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
    /// Every 0.5.x file the plan must account for ([`accountable_paths`]).
    legacy: Vec<String>,
    carried: Carried,
    /// Open bugs' fixture files, kept byte for byte, each with why.
    kept: BTreeMap<String, String>,
    /// Every file the apply writes, text (redacted) and binary, by path.
    files: Vec<Planned>,
    redactions: Vec<Redaction>,
    follow_ups: Vec<String>,
}

impl Plan {
    /// What each carried 0.5.x file becomes: the files its mapper names, else the home of
    /// the 0.5.x ID its file is named by (REQ-records-009). A removed file says why in
    /// `removed` instead.
    fn becomes(&self) -> BTreeMap<String, Vec<String>> {
        let mut out = BTreeMap::new();
        for source in &self.carried.sources {
            let mut targets: BTreeSet<String> = self
                .carried
                .becomes
                .get(source)
                .cloned()
                .unwrap_or_default();
            if targets.is_empty() {
                let name = source.rsplit('/').next().unwrap_or(source);
                let stem = name.rsplit_once('.').map_or(name, |(stem, _)| stem);
                if let Some(home) = self.carried.aliases.get(stem)
                    && home != "removed"
                {
                    targets.insert(home.clone());
                }
            }
            out.insert(source.clone(), targets.into_iter().collect());
        }
        out
    }

    /// The plan as printed, without its digest.
    fn view(&self) -> Value {
        let written: Vec<Value> = self
            .files
            .iter()
            .map(|written| {
                json!({"path": written.path, "sha256": sha256(&written.bytes), "bytes": written.bytes.len()})
            })
            .collect();
        json!({
            "schema_version": 1,
            "sources": self.sources,
            "becomes": self.becomes(),
            "writes": written,
            "redactions": self.redactions,
            "removed": self.carried.removed,
            "kept": self.kept,
            "kept_with_leaks": self.carried.kept_with_leaks,
            "gaps": self.carried.gaps,
            "unaccounted": self.unaccounted(),
            "aliases": self.carried.aliases,
            "follow_ups": self.follow_ups,
            "git_history": "Git history still holds what earlier commits recorded; this migration does not rewrite it.",
        })
    }

    /// The 0.5.x files the plan neither carries, removes nor keeps; an apply refuses
    /// while there is any, so it never leaves a mixed tree.
    fn unaccounted(&self) -> Vec<String> {
        self.legacy
            .iter()
            .filter(|path| {
                !self.carried.sources.contains(*path)
                    && !self.carried.removed.contains_key(*path)
                    && !self.kept.contains_key(*path)
            })
            .cloned()
            .collect()
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
            // REQ-records-009: say up front that the apply will refuse while a native
            // change is open, naming each. It stays outside the digest, which names only
            // the conversion.
            let open = open_native_changes(root)?;
            if !open.is_empty() {
                view["apply_refused"] = json!({
                    "refusal": "refused:change-open",
                    "changes": open,
                    "why": format!(
                        "native change {} is not archived, and `--apply` refuses while a change is open; archive it (or finish it) first, then run `chough migrate` again",
                        open.join(", ")
                    ),
                });
            }
            print_json(&view)
        }
        Some(wanted) => apply(root, &plan, &digest, &wanted, args.consent.as_deref()),
    }
}

/// At most this many archive validity reads run at once (REQ-records-009).
const VALIDITY_READERS: usize = 8;

/// Apply the plan whose digest is `wanted` (REQ-records-009): every refusal writes
/// nothing; then write by create-new and read back, write the receipt, and remove the
/// 0.5.x files, restoring them if an archive that read valid no longer does.
fn apply(
    root: &Path,
    plan: &Plan,
    digest: &str,
    wanted: &str,
    consent: Option<&str>,
) -> Result<()> {
    if plan.sources.is_empty() {
        return print_json(
            &json!({"schema_version": 1, "applied": false, "nothing_to_do": "no 0.5.x record is left; the project is migrated"}),
        );
    }
    ensure!(
        wanted == digest,
        "refused:plan-digest: the plan Chough builds now has digest {digest}, not {wanted}, so nothing was written; nothing is migrated that was not previewed first.\nRun `chough migrate` to preview the current plan, then apply its digest."
    );
    let unaccounted = plan.unaccounted();
    ensure!(
        unaccounted.is_empty(),
        "refused:unaccounted: the plan neither carries nor removes {}, so nothing was written; applying it would leave a mixed tree. This is a Chough bug: report it with the plan.",
        unaccounted.join(", ")
    );
    let open = open_native_changes(root)?;
    ensure!(
        open.is_empty(),
        "refused:change-open: native change {} is not archived, and the migration never runs while a change is open, so nothing was written.\nArchive it (or finish it) first, then run `chough migrate` again.",
        open.join(", ")
    );
    let consent = match consent {
        Some(consent) if !consent.trim().is_empty() => {
            format!("--consent (a relayed claim): {}", consent.trim())
        }
        _ => match crate::prefs::effective(root, "migration")? {
            (value, origin) if value == "standing" => origin,
            _ => bail!(
                "refused:no-consent: the migration needs the human's consent, as --consent \"<who consented, their words and where>\" or a standing `migration` preference, so nothing was written; Chough never infers consent from the digest or any record."
            ),
        },
    };
    let removals: BTreeSet<String> = plan
        .carried
        .sources
        .iter()
        .chain(plan.carried.removed.keys())
        .cloned()
        .collect();
    let uncommitted: Vec<&String> = removals
        .iter()
        .filter(|path| !crate::sessions::committed_at_head(root, path))
        .collect();
    ensure!(
        uncommitted.is_empty(),
        "refused:uncommitted: the migration removes only bytes Git holds, and {} differ from HEAD or are untracked, so nothing was written.\nCommit them, then run `chough migrate` again.",
        uncommitted
            .iter()
            .map(|path| path.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
    for written in &plan.files {
        let path = local_path(root, &written.path)?;
        if let Ok(existing) = std::fs::read(&path)
            && existing != written.bytes
            && !collides_by_case(plan, &written.path)
        {
            bail!(
                "refused:target-exists: {} already exists with other bytes, and the migration writes by create-new only, so nothing more was written.",
                written.path
            );
        }
    }
    // The receipt is known before anything is written: one over its own bound refuses
    // here, writing nothing (REQ-records-001, REQ-records-009).
    let receipt = receipt_path()?;
    let receipt_text = receipt_record(plan, digest, &consent, &removals, root).map_err(|error| {
        anyhow::anyhow!(
            "refused:receipt-bound: the migration receipt would exceed its {} byte bound, so nothing was written: {error:#}",
            crate::markdown::RECEIPT_BOUND
        )
    })?;
    // Every refusal above wrote nothing, not even a lock file. From here the writer lock
    // is held through the writes, the removals, the archive reads and any restore, so
    // no other writer saves in between (REQ-cli-040).
    crate::checkpoint::lock_checkpoint(root)?;
    // Write: create-new, or count a file an interrupted apply wrote with the planned
    // bytes as done. A path that differs only in case from a planned removal (on a
    // case-insensitive file system, one file) waits until the removals, so an
    // interruption or a restore never meets it half replaced.
    let mut written_now = vec![];
    for written in plan
        .files
        .iter()
        .filter(|written| !collides_by_case(plan, &written.path))
    {
        write_planned(root, written, &mut written_now)?;
    }
    let receipt_local = local_path(root, &receipt)?;
    match std::fs::read_to_string(&receipt_local) {
        Ok(existing) => ensure!(
            existing.contains(&format!("plan_digest: {digest}")),
            "refused: {receipt} already exists for another plan, so nothing was removed"
        ),
        Err(_) => {
            if let Some(parent) = receipt_local.parent() {
                std::fs::create_dir_all(parent)?;
            }
            ensure!(
                crate::records::write_new_bounded(
                    root,
                    &receipt,
                    &receipt_text,
                    crate::markdown::RECEIPT_BOUND
                )?,
                "{receipt} appeared while migrating; nothing was removed"
            );
        }
    }
    // Never weaken an archive: read the validity of every archived change whose
    // acceptance manifest lists a file to remove, remove, read again, and restore
    // everything if any that read valid no longer does.
    let affected = affected_archives(root, &removals)?;
    let before = validity(root, &affected)?;
    let mut kept_bytes: Vec<(String, Vec<u8>)> = vec![];
    for path in &removals {
        let local = local_path(root, path)?;
        let Ok(bytes) = std::fs::read(&local) else {
            continue;
        };
        if plan
            .sources
            .get(path)
            .is_some_and(|planned| *planned != sha256(&bytes))
        {
            continue;
        }
        std::fs::remove_file(&local)?;
        kept_bytes.push((path.clone(), bytes));
    }
    let after = validity(root, &affected)?;
    let weakened: Vec<&String> = affected
        .iter()
        .filter(|id| {
            before.get(*id).is_some_and(|state| is_valid(state))
                && !after.get(*id).is_some_and(|state| is_valid(state))
        })
        .collect();
    if !weakened.is_empty() {
        for (path, bytes) in &kept_bytes {
            let local = local_path(root, path)?;
            if let Some(parent) = local.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::write(local, bytes)?;
        }
        bail!(
            "refused:archive-weakened: removing the 0.5.x files made archive {} read invalid, so every removed file was restored byte for byte; the written 0.6.0 files and the receipt stay for review.",
            weakened
                .iter()
                .map(|id| id.as_str())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    // On a case-insensitive file system a case-only rename, such as knowledge/INDEX.md
    // to knowledge/index.md, is one file, and Git keeps its index entry under the old
    // name, so the commit would hold the new bytes under the 0.5.x name. Drop that
    // entry from Git's index (never the work tree) before writing the new name, so the
    // next `git add` records the rename and the receipt matches the commit
    // (REQ-records-009).
    let mut unstaged = vec![];
    for (path, _) in &kept_bytes {
        if plan
            .files
            .iter()
            .any(|written| written.path != *path && written.path.eq_ignore_ascii_case(path))
        {
            unstage(root, path)?;
            unstaged.push(path.clone());
        }
    }
    for written in plan
        .files
        .iter()
        .filter(|written| collides_by_case(plan, &written.path))
    {
        write_planned(root, written, &mut written_now)?;
    }
    prune_empty_folders(root)?;
    prune_emptied(root, kept_bytes.iter().map(|(path, _)| path.as_str()));
    print_json(&json!({
        "schema_version": 1,
        "applied": true,
        "plan_digest": digest,
        "receipt": receipt,
        "written": written_now,
        "removed": kept_bytes.iter().map(|(path, _)| path).collect::<Vec<_>>(),
        "archives_checked": affected,
        "unstaged": unstaged,
        "git_history": "Git history still holds what earlier commits recorded; this migration does not rewrite it.",
    }))
}

/// Drop `path` from Git's index only, as `git rm --cached` does, leaving the work tree.
fn unstage(root: &Path, path: &str) -> Result<()> {
    let output = crate::records::run_owned(
        std::process::Command::new("git").arg("-C").arg(root).args([
            "rm",
            "--cached",
            "--quiet",
            "--ignore-unmatch",
            "--",
            path,
        ]),
        None,
    )?;
    ensure!(
        output.status.success(),
        "git rm --cached {path} failed: {}; the written files and the receipt stay for review",
        String::from_utf8_lossy(&output.stderr).trim()
    );
    Ok(())
}

/// Whether `path` differs only in case from a planned removal, such as
/// `knowledge/index.md` for `knowledge/INDEX.md`: one file on a case-insensitive file
/// system, so it is written after the removals.
fn collides_by_case(plan: &Plan, path: &str) -> bool {
    plan.carried
        .removed
        .keys()
        .chain(plan.carried.sources.iter())
        .any(|removed| removed != path && removed.eq_ignore_ascii_case(path))
}

/// Write one planned file by create-new, or count it done when it already holds the
/// planned bytes, then read it back.
fn write_planned(root: &Path, written: &Planned, written_now: &mut Vec<String>) -> Result<()> {
    let path = local_path(root, &written.path)?;
    let done = std::fs::read(&path).is_ok_and(|bytes| bytes == written.bytes);
    if !done {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        ensure!(
            crate::records::write_new_bytes(root, &written.path, &written.bytes)?,
            "{} appeared while migrating; nothing more was written",
            written.path
        );
        written_now.push(written.path.clone());
    }
    let back = std::fs::read(&path)?;
    ensure!(
        sha256(&back) == sha256(&written.bytes),
        "{} did not read back as planned",
        written.path
    );
    Ok(())
}

/// Every native change with a workspace under `.specsync/changes/`: not archived.
fn open_native_changes(root: &Path) -> Result<Vec<String>> {
    let mut open = vec![];
    if let Ok(entries) = std::fs::read_dir(local_path(root, ".specsync/changes")?) {
        for entry in entries.flatten() {
            if entry.path().join("state.json").is_file() {
                open.push(entry.file_name().to_string_lossy().to_string());
            }
        }
    }
    open.sort();
    Ok(open)
}

/// The archived changes whose acceptance manifest lists a file in `removals`.
fn affected_archives(root: &Path, removals: &BTreeSet<String>) -> Result<Vec<String>> {
    let mut affected = vec![];
    let Ok(entries) = std::fs::read_dir(local_path(root, ".specsync/archive/changes")?) else {
        return Ok(affected);
    };
    for entry in entries.flatten() {
        let Ok(bytes) = std::fs::read(entry.path().join("verification.json")) else {
            continue;
        };
        let Ok(value) = serde_json::from_slice::<Value>(&bytes) else {
            continue;
        };
        let lists = value["acceptance_manifest"]["entries"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|entry| entry["path"].as_str())
            .any(|listed| {
                removals.iter().any(|removed| {
                    removed == listed
                        || removed.starts_with(&format!("{}/", listed.trim_end_matches('/')))
                })
            });
        if lists {
            let finalization: Value = std::fs::read(entry.path().join("finalization.json"))
                .ok()
                .and_then(|bytes| serde_json::from_slice(&bytes).ok())
                .unwrap_or(Value::Null);
            let id = finalization["change_id"]
                .as_str()
                .map(String::from)
                .unwrap_or_else(|| entry.file_name().to_string_lossy().to_string());
            affected.push(id);
        }
    }
    affected.sort();
    affected.dedup();
    Ok(affected)
}

/// The terminal evidence validity of each change, at most eight reads at once.
fn validity(root: &Path, ids: &[String]) -> Result<BTreeMap<String, String>> {
    let mut found = BTreeMap::new();
    for chunk in ids.chunks(VALIDITY_READERS) {
        let read: Vec<(String, String)> = std::thread::scope(|scope| {
            let handles: Vec<_> = chunk
                .iter()
                .map(|id| {
                    scope.spawn(move || {
                        let response = crate::records::native_json(
                            root,
                            "specsync",
                            &["change", "status", id, "--json"],
                        );
                        let state = response
                            .value
                            .as_ref()
                            .and_then(|value| {
                                value["summary"]["terminal_evidence"]["validity"].as_str()
                            })
                            .unwrap_or("unknown")
                            .to_string();
                        (id.clone(), state)
                    })
                })
                .collect();
            handles
                .into_iter()
                .filter_map(|handle| handle.join().ok())
                .collect()
        });
        found.extend(read);
    }
    Ok(found)
}

/// Whether a terminal evidence validity reads as valid.
fn is_valid(state: &str) -> bool {
    matches!(state, "authenticated-history" | "exact" | "valid")
}

/// Remove each folder a removal left empty, from the removed file's own folder up to,
/// never including, the project root: a session rolled into the vault leaves no empty
/// `knowledge/attachments/<folder>/sessions/` behind. A folder that still holds anything
/// stays.
fn prune_emptied<'a>(root: &Path, removed: impl Iterator<Item = &'a str>) {
    for path in removed {
        let mut folder = Path::new(path).parent();
        while let Some(relative) = folder.filter(|folder| !folder.as_os_str().is_empty()) {
            let Ok(local) = local_path(root, &relative.to_string_lossy()) else {
                break;
            };
            if std::fs::remove_dir(local).is_err() {
                break;
            }
            folder = relative.parent();
        }
    }
}

/// Remove the 0.5.x folders the migration emptied.
fn prune_empty_folders(root: &Path) -> Result<()> {
    fn prune(path: &std::path::Path) {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    prune(&entry.path());
                }
            }
        }
        let _ = std::fs::remove_dir(path);
    }
    for folder in [
        ".chough/history",
        ".chough/evidence",
        ".chough/sessions/archive",
        ".chough/sessions/shelved",
        ".chough/artifacts",
    ] {
        prune(&local_path(root, folder)?);
    }
    Ok(())
}

/// The migration receipt: every file before and after with its size and SHA-256, the
/// redactions, every removed file with why and the commit that holds it, the alias map
/// and the consent (REQ-records-009).
fn receipt_record(
    plan: &Plan,
    digest: &str,
    consent: &str,
    removals: &BTreeSet<String>,
    root: &Path,
) -> Result<String> {
    let head = std::process::Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(root)
        .output()
        .ok()
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
        .unwrap_or_default();
    let mut record = crate::markdown::Record::new("");
    record.set_scalar("kind", "receipt")?;
    record.set_scalar(
        "id",
        receipt_path()?
            .trim_start_matches(".chough/archive/migrations/")
            .trim_end_matches(".md"),
    )?;
    record.set_scalar("plan_digest", digest)?;
    record.set_scalar(
        "applied",
        &crate::session_md::iso_time(crate::records::now()?),
    )?;
    record.set_scalar("consent", consent)?;
    record.set_scalar("removed_at_commit", &head)?;
    let mut body = String::from(
        "# Migration from Chough 0.5.x\n\nGit history still holds what earlier commits recorded; this migration does not rewrite it.\n\n## Before\n\n",
    );
    for (path, sha) in &plan.sources {
        body.push_str(&format!("- `{path}` {sha}\n"));
    }
    body.push_str("\n## After\n\n");
    for written in &plan.files {
        body.push_str(&format!(
            "- `{}` {} bytes {}\n",
            written.path,
            written.bytes.len(),
            sha256(&written.bytes)
        ));
    }
    body.push_str("\n## Redactions\n\n");
    for redaction in &plan.redactions {
        body.push_str(&format!(
            "- `{}` {}: {}\n",
            redaction.file, redaction.field, redaction.kind
        ));
    }
    body.push_str("\n## Removed\n\n");
    let becomes = plan.becomes();
    for path in removals {
        let why = match plan.carried.removed.get(path) {
            Some(why) => why.clone(),
            None => match becomes.get(path).filter(|targets| !targets.is_empty()) {
                Some(targets) => format!(
                    "carried into {}",
                    targets
                        .iter()
                        .map(|target| format!("`{target}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                None => "carried into the 0.6.0 records above".to_string(),
            },
        };
        body.push_str(&format!("- `{path}`: {why}; held by commit {head}\n"));
    }
    body.push_str("\n## Kept with leaks\n\n");
    for (path, kinds) in &plan.carried.kept_with_leaks {
        body.push_str(&format!("- `{path}`: {} (fix by hand)\n", kinds.join(", ")));
    }
    body.push_str("\n## Not carried\n\n");
    for gap in &plan.carried.gaps {
        body.push_str(&format!("- {gap}\n"));
    }
    body.push_str("\n## Aliases\n\n");
    for (old, new) in &plan.carried.aliases {
        body.push_str(&format!("- `{old}` → `{new}`\n"));
    }
    for follow_up in &plan.follow_ups {
        body.push_str(&format!("\n## Follow-up\n\n{follow_up}\n"));
    }
    let mut record = crate::markdown::parse("receipt", &format!("{}{body}", record.render()))?;
    record.redact_leaks(Local::current());
    record.render_within(crate::markdown::Kind::Receipt)
}

/// Build the plan: run every mapper in order, redact what they write and list every
/// 0.5.x file read.
fn plan(root: &Path) -> Result<Plan> {
    let mut carried = Carried::default();
    let (sessions, homes) = sessions_0_5(root)?;
    carried.absorb(sessions)?;
    carried.absorb(retired_files_0_5(root)?)?;
    // REQ-records-002: the reading guide, written only when there is none; one an
    // interrupted apply already wrote with these bytes stays planned, so the plan the
    // resumed apply builds has the same digest.
    let guide = local_path(root, ".chough/README.md")?;
    if !guide.exists()
        || std::fs::read(&guide).is_ok_and(|bytes| bytes == crate::sessions::README.as_bytes())
    {
        carried.written.push(Written {
            path: ".chough/README.md".into(),
            bytes: crate::sessions::README.into(),
        });
    }
    let links = bug_links_0_5(root)?;
    let (evidence, observations) = evidence_0_5(root, &homes, &links)?;
    carried.absorb(evidence)?;
    story_receipts(&mut carried, &homes, &observations)?;
    carried.absorb(bugs_0_5(root, &observations)?)?;
    // Knowledge runs last: its index.md is the one the tree has once every other
    // mapper's plan applies.
    let knowledge = knowledge_0_5(root, &carried)?;
    carried.absorb(knowledge)?;
    // A file a mapper carried is never `removed without carrying`, whatever another
    // mapper listed.
    let carried_sources = carried.sources.clone();
    carried
        .removed
        .retain(|path, _| !carried_sources.contains(path));
    // An open bug's fixture is kept byte for byte, whatever a mapper listed.
    let kept = crate::bugs::open_fixture_owners_0_5(root)?;
    carried.removed.retain(|path, _| !kept.contains_key(path));
    carried
        .written
        .sort_by(|left, right| left.path.cmp(&right.path));
    let local = Local::current();
    let mut redactions = vec![];
    for written in &mut carried.written {
        redactions.extend(redact(local, written)?);
    }
    redactions.sort();
    let mut files: Vec<Planned> = carried
        .written
        .iter()
        .map(|written| Planned {
            path: written.path.clone(),
            bytes: written.bytes.clone().into_bytes(),
        })
        .chain(carried.binaries.iter().map(|binary| Planned {
            path: binary.path.clone(),
            bytes: binary.bytes.clone(),
        }))
        .collect();
    files.sort_by(|left, right| left.path.cmp(&right.path));
    let legacy = accountable_paths(root)?;
    let mut sources = BTreeMap::new();
    for path in legacy
        .clone()
        .into_iter()
        .chain(carried.sources.iter().cloned())
        .chain(carried.removed.keys().cloned())
    {
        let bytes = std::fs::read(local_path(root, &path)?)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        sources.insert(path, sha256(&bytes));
    }
    // REQ-records-014: the plan lists the missing verification exclusions as a
    // follow-up; Chough never writes SpecSync's policy.
    let mut follow_ups = crate::doctor::exclusions(root)
        .advice()
        .into_iter()
        .collect::<Vec<_>>();
    // REQ-cli-047: a migrated project declares Chough's merge lines and sets its merge
    // drivers as one `chough init` sets up. The migration writes create-new only, so it
    // names what to add; the lines come from the committed `.gitattributes`, so the plan
    // is the same in every clone.
    let lines = crate::library::missing_merge_lines(root)?;
    if !lines.is_empty() {
        follow_ups.push(format!(
            "Declare Chough's merge drivers so generated files and bugs merge without a hand-resolved conflict (REQ-cli-047): add {} to `.gitattributes` and commit it, then run `chough doctor --fix` in each clone, which sets `merge.chough-bug.driver` and `merge.chough-generated.driver`.",
            lines
                .iter()
                .map(|line| format!("`{line}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }
    Ok(Plan {
        sources,
        legacy,
        carried,
        kept,
        files,
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

/// The migration receipt's path, fixed when the plan is built: an apply on another UTC
/// day rebuilds a plan with another digest and asks for a fresh preview.
pub(crate) fn receipt_path() -> Result<String> {
    Ok(format!(
        ".chough/archive/migrations/{}-from-0-5.md",
        crate::identity::utc_date(crate::records::now()?)
    ))
}

/// A deterministic six-hex-digit suffix or entry ID from `seed`, so the plan an apply
/// rebuilds has the previewed digest (REQ-records-009).
fn hex6(seed: &str) -> String {
    sha256(seed.as_bytes())[..6].to_string()
}

/// The 0.5.x history cursor of a session (`history_cursor`).
#[derive(serde::Deserialize, Clone, Default)]
struct Cursor {
    cycle: String,
    head: Option<String>,
    #[serde(default)]
    legacy: Option<Value>,
}

/// One 0.5.x history event.
#[derive(serde::Deserialize)]
struct Event {
    parent: Option<String>,
    previous_cycle: Option<Cursor>,
    transition: crate::stage::Transition,
}

/// A session's selected history lineage, oldest first, as 0.5.x selected it: from the
/// cursor's head through each parent and previous cycle, then the legacy log when the
/// walk reaches its anchor. Returns the transitions, the files read, and what could not
/// be read (a pruned or missing event ends the walk there).
fn lineage(
    root: &Path,
    cursor: &Cursor,
) -> Result<(Vec<crate::stage::Transition>, Vec<String>, Vec<String>)> {
    let mut transitions = vec![];
    let mut files = vec![];
    let mut missing = vec![];
    let mut current = cursor.clone();
    let mut seen = BTreeSet::new();
    while let Some(id) = current.head.clone() {
        if !seen.insert((current.cycle.clone(), id.clone())) {
            missing.push(format!("a parent loop at {id}"));
            break;
        }
        let path = format!(".chough/history/cycles/{}/{id}.json", current.cycle);
        let Ok(bytes) = std::fs::read(local_path(root, &path)?) else {
            missing.push(format!("{path} (pruned or missing)"));
            break;
        };
        let event: Event = serde_json::from_slice(&bytes)
            .with_context(|| format!("{path} is not a 0.5.x history event"))?;
        files.push(path);
        transitions.push(event.transition);
        current.head = event.parent;
        if current.head.is_none()
            && let Some(previous) = event.previous_cycle
        {
            current = previous;
        }
    }
    if current.legacy.is_some() && missing.is_empty() {
        let legacy = ".chough/transitions.jsonl";
        if let Ok(text) = std::fs::read_to_string(local_path(root, legacy)?) {
            for line in text.lines().rev().filter(|line| !line.trim().is_empty()) {
                transitions.push(
                    serde_json::from_str(line).with_context(|| {
                        format!("{legacy} holds a line that is not a transition")
                    })?,
                );
            }
            files.push(legacy.to_string());
        } else {
            missing.push(format!("{legacy} (the adopted legacy log)"));
        }
    }
    transitions.reverse();
    Ok((transitions, files, missing))
}

/// The newest commit time of `relative`, when Git has one.
fn committed_at(root: &Path, relative: &str) -> Option<u64> {
    let output = std::process::Command::new("git")
        .args(["log", "-1", "--format=%ct", "--", relative])
        .current_dir(root)
        .output()
        .ok()?;
    String::from_utf8_lossy(&output.stdout).trim().parse().ok()
}

/// Mapper 1, sessions (t382): each 0.5.x session becomes a session folder (open) or an
/// archived story, its timeline the selected lineage; unfolded inbox entries move to
/// `agent/inbox/`; shelved sessions are removed.
fn sessions_0_5(root: &Path) -> Result<(Carried, BTreeMap<String, SessionHome>)> {
    let mut carried = Carried::default();
    let mut homes = BTreeMap::new();
    let mut found: Vec<(String, String, bool)> = vec![];
    if local_path(root, ".chough/session.json")?.is_file() {
        found.push(("legacy".into(), ".chough/session.json".into(), false));
    }
    // Sessions the 0.5.x roll-up moved into the vault are archived stories too
    // (`<library-dir>/attachments/<folder>/sessions/<key>.json`).
    let library = crate::prefs::effective(root, "library-dir")
        .map(|(dir, _)| dir)
        .unwrap_or_else(|_| "knowledge".into());
    let mut folders: Vec<(String, bool)> = vec![
        (".chough/sessions".into(), false),
        (".chough/sessions/archive".into(), true),
    ];
    let attachments = format!("{}/attachments", library.trim_end_matches('/'));
    if let Ok(entries) = std::fs::read_dir(local_path(root, &attachments)?) {
        let mut rolled: Vec<String> = entries
            .flatten()
            .filter(|entry| entry.path().join("sessions").is_dir())
            .filter_map(|entry| entry.file_name().into_string().ok())
            .map(|folder| format!("{attachments}/{folder}/sessions"))
            .collect();
        rolled.sort();
        folders.extend(rolled.into_iter().map(|folder| (folder, true)));
    }
    for (folder, archived) in folders {
        let folder = folder.as_str();
        let mut names: Vec<String> = std::fs::read_dir(local_path(root, folder)?)
            .map(|entries| {
                entries
                    .flatten()
                    .filter(|entry| entry.path().is_file())
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .filter(|name| name.ends_with(".json"))
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        for name in names {
            let key = name.trim_end_matches(".json").to_string();
            found.push((key, format!("{folder}/{name}"), archived));
        }
    }
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut lineage_files: BTreeSet<String> = BTreeSet::new();
    for (key, path, archived) in found {
        let text = std::fs::read_to_string(local_path(root, &path)?)
            .with_context(|| format!("{path} cannot be read, so no plan was made"))?;
        let mut value: Value = serde_json::from_str(&text)
            .with_context(|| format!("{path} is not a 0.5.x session record"))?;
        ensure!(
            value["schema_version"].as_u64() != Some(1),
            "refused:schema-1: {path} is a schema 1 checkpoint, older than 0.5.x, so no plan was made. Convert it to schema 2 with Chough 0.5.x first (`chough migrate --stage <its stage>`), then run this `chough migrate` again."
        );
        let cursor = value
            .as_object_mut()
            .and_then(|map| map.remove("history_cursor"))
            .filter(|cursor| !cursor.is_null());
        let mut session: crate::checkpoint::Checkpoint = serde_json::from_value(value)
            .with_context(|| format!("{path} is not a 0.5.x session record"))?;
        let (transitions, files, missing) = match cursor {
            Some(cursor) => lineage(root, &serde_json::from_value(cursor)?)?,
            // A legacy single checkpoint before history events: its log is its history.
            None if key == "legacy" => lineage(
                root,
                &Cursor {
                    legacy: Some(Value::Bool(true)),
                    ..Cursor::default()
                },
            )?,
            None => (vec![], vec![], vec![]),
        };
        lineage_files.extend(files.iter().cloned());
        let at = transitions
            .iter()
            .map(|transition| transition.at_unix_seconds)
            .filter(|at| *at > 0)
            .min()
            .or_else(|| committed_at(root, &path))
            .unwrap_or(crate::records::now()?);
        let last = transitions
            .iter()
            .map(|transition| transition.at_unix_seconds)
            .max()
            .unwrap_or(at);
        let source = if key == "legacy" {
            session.objective.clone()
        } else {
            key.clone()
        };
        let mut id =
            crate::identity::dated_name(&crate::identity::utc_date(at), &source, "session");
        if !taken.insert(id.clone()) {
            id = format!("{id}-{}", hex6(&path));
            taken.insert(id.clone());
        }
        session.meta = crate::session_md::Meta {
            id: id.clone(),
            aliases: if key == "legacy" {
                vec![]
            } else {
                vec![key.clone()]
            },
            pr: String::new(),
            gates: vec![],
        };
        session.pending = transitions;
        let record = crate::session_md::render_with(&session, last, &mut |index, attempt| {
            Ok(format!("t-{}", hex6(&format!("{id}#{index}#{attempt}"))))
        })?;
        let mut record = record;
        let (target, text) = if archived {
            let date = crate::identity::utc_date(last);
            let outcome = if session.stage.current == "done" {
                "done"
            } else {
                "closed-without-done"
            };
            record.set_scalar("closed", &date)?;
            record.set_scalar("outcome", outcome)?;
            record.set("receipts", crate::markdown::Value::Maps(vec![]))?;
            let mut text = record.render();
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&format!("\nClosed {date}: {outcome}.\n"));
            (format!(".chough/archive/sessions/{id}.md"), text)
        } else {
            (format!(".chough/sessions/{id}/session.md"), record.render())
        };
        for gap in missing {
            carried
                .gaps
                .push(format!("{path}: history not carried into {target}: {gap}"));
        }
        carried.written.push(Written {
            path: target.clone(),
            bytes: text,
        });
        // The session file and its lineage's history files all become this session's
        // folder or story.
        for source in std::iter::once(&path).chain(files.iter()) {
            carried
                .becomes
                .entry(source.clone())
                .or_default()
                .insert(target.clone());
        }
        carried.sources.insert(path.clone());
        carried.sources.extend(files);
        if key != "legacy" {
            carried.aliases.insert(key.clone(), target.clone());
        }
        // Unfolded inbox entries of an open session move to its agent/inbox/.
        let inbox = format!(".chough/sessions/{key}.inbox");
        let mut entries: Vec<String> = std::fs::read_dir(local_path(root, &inbox)?)
            .map(|entries| {
                entries
                    .flatten()
                    .filter_map(|entry| entry.file_name().into_string().ok())
                    .filter(|name| name.ends_with(".json"))
                    .collect()
            })
            .unwrap_or_default();
        entries.sort();
        for name in entries {
            let entry_path = format!("{inbox}/{name}");
            if archived {
                carried
                    .removed
                    .insert(entry_path, "an inbox entry of an archived session".into());
                continue;
            }
            let entry: Value =
                serde_json::from_str(&std::fs::read_to_string(local_path(root, &entry_path)?)?)
                    .with_context(|| format!("{entry_path} is not a 0.5.x inbox entry"))?;
            let text = |field: &str| entry[field].as_str().unwrap_or_default().to_string();
            let kind = <crate::sessions::Kind as clap::ValueEnum>::from_str(&text("kind"), true)
                .map_err(|_| anyhow::anyhow!("{entry_path} has an unknown inbox kind"))?;
            let added = entry["recorded_at_unix_seconds"].as_u64().unwrap_or(at);
            let name = crate::identity::dated_name(
                &crate::identity::utc_date(added),
                &text("text"),
                "entry",
            );
            let rendered = crate::sessions::render_entry(
                &name,
                &id,
                &text("from"),
                kind,
                &text("text"),
                None,
                &crate::session_md::iso_time(added),
            )?;
            // The 0.5.x INB- ID stays the entry's alias (REQ-records-003), and the
            // receipt maps it to the new file (REQ-records-009).
            let new_path = format!(".chough/sessions/{id}/agent/inbox/{name}.md");
            let rendered = if text("id").is_empty() {
                rendered
            } else {
                let mut entry = crate::markdown::parse(&new_path, &rendered)?;
                entry.set("aliases", crate::markdown::Value::List(vec![text("id")]))?;
                entry.render_within(crate::markdown::Kind::Inbox)?
            };
            carried.written.push(Written {
                path: new_path.clone(),
                bytes: rendered,
            });
            carried
                .becomes
                .entry(entry_path.clone())
                .or_default()
                .insert(new_path.clone());
            carried.sources.insert(entry_path.clone());
            carried.aliases.insert(text("id"), new_path);
        }
        homes.insert(
            key,
            SessionHome {
                id,
                archived,
                observations: session.observations.clone(),
            },
        );
    }
    // What no session's lineage selected, shelved sessions and 0.5.x-only files.
    let mut leftovers = BTreeSet::new();
    walk_files(root, ".chough/history", &mut leftovers)?;
    walk_files(root, ".chough/sessions/shelved", &mut leftovers)?;
    for path in [".chough/transitions.jsonl", ".chough/sessions/.gitignore"] {
        if local_path(root, path)?.is_file() {
            leftovers.insert(path.to_string());
        }
    }
    for path in leftovers {
        if lineage_files.contains(&path) || carried.sources.contains(&path) {
            continue;
        }
        let why = if path.starts_with(".chough/sessions/shelved") {
            "a shelved 0.5.x session is not carried"
        } else if path.ends_with(".gitignore") {
            "0.5.x temporary-file ignore; 0.6.0 writes temporaries in .chough/local/"
        } else if path.contains("/pruned/") {
            "a 0.5.x retention receipt; there is no retention in 0.6.0"
        } else {
            "history no session's selected lineage holds"
        };
        carried.removed.insert(path, why.into());
    }
    Ok((carried, homes))
}

/// The 0.5.x files no mapper carries and nothing in 0.6.0 replaces, each removed with
/// why (REQ-records-009).
fn retired_files_0_5(root: &Path) -> Result<Carried> {
    let mut carried = Carried::default();
    for (path, why) in [
        (
            ".chough/session.v1.json",
            "the schema 1 checkpoint backup an earlier Chough kept when it moved the checkpoint to schema 2; 0.6.0 reads neither",
        ),
        (
            ".chough/pruned.jsonl",
            "the 0.5.x retention ledger; 0.6.0 has no retention (REQ-cli-049), and Git keeps its bytes",
        ),
    ] {
        if local_path(root, path)?.is_file() {
            carried.removed.insert(path.to_string(), why.to_string());
        }
    }
    Ok(carried)
}

/// Each archived story's `receipts` (REQ-history-006): the receipt the evidence mapper
/// made of each observation its 0.5.x session referenced, or, for one carried nowhere, a
/// tombstone naming the migration receipt, which never counts, its ID aliased `removed`
/// unless a mapper already placed it. The sessions mapper writes every story with no
/// receipts, since evidence runs after it.
fn story_receipts(
    carried: &mut Carried,
    sessions: &BTreeMap<String, SessionHome>,
    homes: &ObservationHomes,
) -> Result<()> {
    let receipt = receipt_path()?;
    for home in sessions.values().filter(|home| home.archived) {
        let path = format!(".chough/archive/sessions/{}.md", home.id);
        let Some(written) = carried
            .written
            .iter_mut()
            .find(|written| written.path == path)
        else {
            continue;
        };
        let mut receipts: Vec<Vec<(String, String)>> = vec![];
        for id in &home.observations {
            let map = match homes.receipts.get(id) {
                Some(map) => map.clone(),
                None if homes.entries.contains_key(id) => continue,
                None => {
                    carried
                        .aliases
                        .entry(id.clone())
                        .or_insert_with(|| "removed".to_string());
                    vec![
                        ("id".to_string(), id.clone()),
                        ("removed".to_string(), receipt.clone()),
                    ]
                }
            };
            if !receipts.contains(&map) {
                receipts.push(map);
            }
        }
        if receipts.is_empty() {
            continue;
        }
        let mut record = crate::markdown::parse(&path, &written.bytes)?;
        record.set("receipts", crate::markdown::Value::Maps(receipts))?;
        written.bytes = record
            .render_within(crate::markdown::Kind::Story)
            .with_context(|| format!("{path} with its receipts"))?;
    }
    Ok(())
}

/// Mapper 2, each 0.5.x bug's linked observation IDs (t38b).
fn bug_links_0_5(root: &Path) -> Result<BTreeMap<String, Vec<String>>> {
    crate::bugs::links_0_5(root)
}

/// Mapper 3, evidence (t38b).
fn evidence_0_5(
    root: &Path,
    sessions: &BTreeMap<String, SessionHome>,
    bug_links: &BTreeMap<String, Vec<String>>,
) -> Result<(Carried, ObservationHomes)> {
    crate::evidence_migrate::migrate_0_5(root, sessions, bug_links)
}

/// Mapper 4, bugs (t38b).
fn bugs_0_5(root: &Path, observations: &ObservationHomes) -> Result<Carried> {
    crate::bugs::migrate_0_5(root, observations)
}

/// Mapper 5, knowledge (t37j), given what the earlier mappers carry.
fn knowledge_0_5(root: &Path, earlier: &Carried) -> Result<Carried> {
    crate::library::migrate_0_5(root, earlier)
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

    /// Every release fixture's sessions map to 0.6.0 records that read back strictly,
    /// with every selected transition on the timeline, and the same plan twice.
    #[test]
    fn every_release_fixture_maps_its_sessions_to_readable_records() {
        let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases");
        for tag in ["v0.1.0", "v0.2.0", "v0.3.0", "v0.4.0", "v0.5.0"] {
            let root = fixtures.join(tag);
            let (carried, homes) = sessions_0_5(&root).unwrap();
            let (again, _) = sessions_0_5(&root).unwrap();
            assert_eq!(carried.written, again.written, "{tag}: not deterministic");
            assert!(!carried.written.is_empty(), "{tag}: no session carried");
            let events = {
                let mut found = BTreeSet::new();
                walk_files(&root, ".chough/history", &mut found).unwrap();
                found.len()
            };
            let legacy = std::fs::read_to_string(root.join(".chough/transitions.jsonl"))
                .map(|text| text.lines().filter(|line| !line.trim().is_empty()).count())
                .unwrap_or(0);
            let mut entries = 0;
            for written in &carried.written {
                if written.path.ends_with("/session.md")
                    || written.path.starts_with(".chough/archive/sessions/")
                {
                    let session = crate::session_md::read(&written.path, &written.bytes)
                        .unwrap_or_else(|error| panic!("{tag} {}: {error:#}", written.path));
                    let record = session.source.as_ref().unwrap();
                    entries += crate::session_md::timeline(record).unwrap().len();
                    assert!(
                        homes.values().any(|home| written.path.contains(&home.id)),
                        "{tag}: {} has no home",
                        written.path
                    );
                }
            }
            assert_eq!(
                entries,
                events + legacy,
                "{tag}: transitions lost or duplicated"
            );
        }
    }

    /// A session the 0.5.x roll-up moved into the vault becomes an archived story, its
    /// history read through its cursor as for any archived session.
    #[test]
    fn a_rolled_session_in_the_vault_becomes_an_archived_story() {
        fn copy(from: &Path, to: &Path) {
            std::fs::create_dir_all(to).unwrap();
            for entry in std::fs::read_dir(from).unwrap().flatten() {
                let target = to.join(entry.file_name());
                if entry.path().is_dir() {
                    copy(&entry.path(), &target);
                } else {
                    std::fs::copy(entry.path(), target).unwrap();
                }
            }
        }
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases/v0.5.0");
        let root = tempfile::tempdir().unwrap();
        copy(&fixture, root.path());
        let rolled = root
            .path()
            .join("knowledge/attachments/fixture-second/sessions");
        std::fs::create_dir_all(&rolled).unwrap();
        std::fs::rename(
            root.path()
                .join(".chough/sessions/archive/fixture-second.json"),
            rolled.join("fixture-second.json"),
        )
        .unwrap();
        let (carried, homes) = sessions_0_5(root.path()).unwrap();
        assert!(homes["fixture-second"].archived);
        let story = &carried.aliases["fixture-second"];
        assert!(story.starts_with(".chough/archive/sessions/"), "{story}");
        assert!(
            carried
                .sources
                .contains("knowledge/attachments/fixture-second/sessions/fixture-second.json")
        );
        let written = carried
            .written
            .iter()
            .find(|written| &written.path == story)
            .unwrap();
        crate::session_md::read(&written.path, &written.bytes).unwrap();
    }

    /// Story receipts: a carried observation's receipt once, an open session's entry
    /// left to that session, and one carried nowhere as a tombstone aliased `removed`.
    #[test]
    fn a_story_takes_its_receipts_and_tombstones_after_the_evidence_mapper() {
        fn copy(from: &Path, to: &Path) {
            std::fs::create_dir_all(to).unwrap();
            for entry in std::fs::read_dir(from).unwrap().flatten() {
                let target = to.join(entry.file_name());
                if entry.path().is_dir() {
                    copy(&entry.path(), &target);
                } else {
                    std::fs::copy(entry.path(), target).unwrap();
                }
            }
        }
        let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/releases/v0.5.0");
        let root = tempfile::tempdir().unwrap();
        copy(&fixture, root.path());
        let (mut carried, mut sessions) = sessions_0_5(root.path()).unwrap();
        let key = sessions
            .iter()
            .find(|(_, home)| home.archived)
            .map(|(key, _)| key.clone())
            .unwrap();
        let home = sessions.get_mut(&key).unwrap();
        home.observations = vec![
            "OBS-carried".into(),
            "OBS-carried".into(),
            "OBS-entry".into(),
            "OBS-gone".into(),
        ];
        let story = format!(".chough/archive/sessions/{}.md", home.id);
        let carried_map = vec![
            ("id".to_string(), "OBS-carried".to_string()),
            ("result".to_string(), "passed".to_string()),
        ];
        let homes = ObservationHomes {
            entries: BTreeMap::from([("OBS-entry".into(), "elsewhere".into())]),
            receipts: BTreeMap::from([("OBS-carried".into(), carried_map.clone())]),
        };
        story_receipts(&mut carried, &sessions, &homes).unwrap();
        let written = carried
            .written
            .iter()
            .find(|written| written.path == story)
            .unwrap();
        let record = crate::markdown::parse(&story, &written.bytes).unwrap();
        let tombstone = vec![
            ("id".to_string(), "OBS-gone".to_string()),
            ("removed".to_string(), receipt_path().unwrap()),
        ];
        assert_eq!(record.maps("receipts"), vec![carried_map, tombstone]);
        assert_eq!(carried.aliases["OBS-gone"], "removed");
        assert!(!carried.aliases.contains_key("OBS-entry"));
        crate::session_md::read(&written.path, &written.bytes).unwrap();
    }

    /// A binary write (a stripped screenshot) is planned byte for byte, never as text,
    /// shares the one-writer rule with text writes, and writes and reads back exactly.
    #[test]
    fn a_binary_write_is_planned_and_written_byte_for_byte() {
        let png = vec![
            0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0xff, 0x00, 0xfe,
        ];
        let mut text = Carried::default();
        text.written.push(Written {
            path: ".chough/archive/sessions/a/shot.png".into(),
            bytes: "text".into(),
        });
        let mut binary = Carried::default();
        binary.binaries.push(Binary {
            path: ".chough/archive/sessions/a/shot.png".into(),
            bytes: png.clone(),
        });
        let refused = text.absorb(binary.clone()).unwrap_err().to_string();
        assert!(refused.contains("two mappers write"), "{refused}");
        let mut empty = Carried::default();
        empty.absorb(binary).unwrap();
        assert_eq!(empty.binaries[0].bytes, png);

        let root = tempfile::tempdir().unwrap();
        let planned = Planned {
            path: ".chough/archive/sessions/a/shot.png".into(),
            bytes: png.clone(),
        };
        let mut now = vec![];
        write_planned(root.path(), &planned, &mut now).unwrap();
        assert_eq!(now, vec![planned.path.clone()]);
        assert_eq!(std::fs::read(root.path().join(&planned.path)).unwrap(), png);
        // An interrupted apply's file with the planned bytes counts as done.
        let mut again = vec![];
        write_planned(root.path(), &planned, &mut again).unwrap();
        assert!(again.is_empty());
    }

    /// An alias has one home: a real home wins over `removed` in either order, and two
    /// different homes are refused.
    #[test]
    fn an_alias_keeps_its_one_real_home() {
        let carry = |id: &str, home: &str| {
            let mut carried = Carried::default();
            carried.aliases.insert(id.into(), home.into());
            carried
        };
        let mut plan = carry("OBS-a", "removed");
        plan.absorb(carry("OBS-a", ".chough/bugs/x.md")).unwrap();
        plan.absorb(carry("OBS-a", "removed")).unwrap();
        plan.absorb(carry("OBS-a", ".chough/bugs/x.md")).unwrap();
        assert_eq!(plan.aliases["OBS-a"], ".chough/bugs/x.md");
        let refused = plan
            .absorb(carry("OBS-a", ".chough/archive/sessions/y.md"))
            .unwrap_err()
            .to_string();
        assert!(refused.contains("two mappers alias OBS-a"), "{refused}");
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
