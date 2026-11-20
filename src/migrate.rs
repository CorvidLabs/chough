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
use std::sync::atomic::{AtomicU32, Ordering};

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
    /// The archived story (its path) that holds each observation carried as a receipt,
    /// when a story does; a receipt only a bug holds has none (#277).
    pub(crate) stories: BTreeMap<String, String>,
}

impl ObservationHomes {
    /// The name a migrated record's front matter gives a 0.5.x observation (t38n's
    /// round-4 F9, REQ-records-003): its entry's file name, or its receipt's `id`, each
    /// of which keeps the 0.5.x ID as its alias. One carried as neither keeps its ID,
    /// which resolves as `removed`.
    pub(crate) fn name(&self, id: &str) -> String {
        if let Some(name) = self
            .entries
            .get(id)
            .and_then(|path| path.rsplit('/').next())
            .and_then(|file| file.strip_suffix(".md"))
        {
            return name.to_string();
        }
        self.receipts
            .get(id)
            .and_then(|map| map.iter().find(|(key, _)| key == "id"))
            .map_or_else(|| id.to_string(), |(_, name)| name.clone())
    }

    /// Each of `ids` by [`ObservationHomes::name`], once, in order.
    pub(crate) fn names(&self, ids: &[String]) -> Vec<String> {
        let mut names: Vec<String> = vec![];
        for id in ids {
            let name = self.name(id);
            if !names.contains(&name) {
                names.push(name);
            }
        }
        names
    }
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

/// The 0.5.x files the plan being built removes as already migrated, because a committed
/// receipt of an earlier migration lists them with these bytes (REQ-records-009, #370).
/// The mappers leave them out, so none is carried a second time. It is process-wide
/// state keyed by project-relative path and set by each plan build; a test that builds
/// plans of two projects in one process would share it.
static ALREADY_MIGRATED: std::sync::Mutex<BTreeSet<String>> =
    std::sync::Mutex::new(BTreeSet::new());

/// Whether the plan being built removes `path` as already migrated, so no mapper reads it.
pub(crate) fn already_migrated(path: &str) -> bool {
    ALREADY_MIGRATED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
        .contains(path)
}

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
            // macOS metadata beside a 0.5.x record is not one (REQ-records-015).
            if name.ends_with(".json")
                && !crate::evidence::is_os_metadata(&name)
                && entry.path().is_file()
            {
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

/// Every file under `relative`, as project-relative paths, leaving out macOS metadata
/// files and folders, which the migration neither carries nor removes (REQ-records-015).
fn walk_files(root: &Path, relative: &str, found: &mut BTreeSet<String>) -> Result<()> {
    let Ok(entries) = std::fs::read_dir(local_path(root, relative)?) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        if crate::evidence::is_os_metadata(&entry.file_name().to_string_lossy()) {
            continue;
        }
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
    /// Print the preview's plan as JSON instead of text; the apply prints JSON either way.
    #[arg(long)]
    json: bool,
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
    /// The receipt the apply writes, named when the plan is built (REQ-records-009).
    receipt: String,
    /// The generated files (`knowledge/index.md`, `knowledge/journal/feed.xml`) that are
    /// regular files with other bytes when the plan is built, which the apply replaces
    /// with the rebuilt ones as `chough library index` does; in the digest, so a file
    /// that appears after the preview changes the plan (REQ-records-009).
    replaces: Vec<String>,
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
            "receipt": self.receipt,
            "replaces": self.replaces,
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

/// What a preview says on a tree with no 0.5.x record (REQ-records-009).
const NOTHING_TO_MIGRATE: &str =
    "no 0.5.x record is left, so there is nothing to migrate and no plan digest to apply";

pub(super) fn run(root: &Path, args: MigrateArgs) -> Result<()> {
    // REQ-records-009 (#378): an apply holds the writer lock from before it reads the
    // records it plans from until it has finished, so of two applies of one digest the
    // second finds nothing left to migrate. The Git-ignored lock file is not a write of
    // the migration.
    if args.apply.is_some() {
        crate::checkpoint::lock_checkpoint(root)?;
    }
    let (plan, digest) = plan_with_a_free_receipt(root)?;
    match args.apply {
        // REQ-records-009: on a tree with no 0.5.x record the preview says there is
        // nothing to migrate and gives no plan digest, as the apply there reports
        // nothing to do.
        None if plan.sources.is_empty() => {
            if args.json {
                print_json(&json!({
                    "schema_version": 1,
                    "nothing_to_do": NOTHING_TO_MIGRATE,
                }))
            } else {
                println!("Nothing to migrate: {NOTHING_TO_MIGRATE}.");
                Ok(())
            }
        }
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
            // #337: a target the apply would refuse is named up front, read without
            // waiting.
            if open.is_empty()
                && let Some(refusal) = target_refusal(root, &plan, &digest)?
            {
                view["apply_refused"] = json!({
                    "refusal": "refused:target-exists",
                    "why": refusal,
                });
            }
            if open.is_empty()
                && !view["apply_refused"].is_object()
                && let Some(refusal) = conflict_refusal(root, &plan)
            {
                view["apply_refused"] = json!({
                    "refusal": "refused:conflict-markers",
                    "why": refusal,
                });
            }
            if !open.is_empty() {
                view["apply_refused"] = json!({
                    "refusal": "refused:change-open",
                    "changes": open,
                    "why": change_open_why(&open, "`--apply` refuses while a change is open"),
                });
            }
            // #339: a plan the apply would refuse is not offered to apply, in JSON either.
            if view["apply_refused"].is_object()
                && let Some(fields) = view.as_object_mut()
            {
                fields.remove("apply");
            }
            // REQ-records-009 (F12b): a person consents to the plan, so it is text by
            // default, and the same plan and digest as JSON under --json.
            if args.json {
                print_json(&view)
            } else {
                print!("{}", preview_text(&view));
                Ok(())
            }
        }
        Some(wanted) => apply(root, &plan, &digest, &wanted, args.consent.as_deref()),
    }
}

/// Each planned record that would hold a Git conflict marker, with the 0.5.x file it
/// comes from, its line in the record and the marker (REQ-records-001, #339).
fn planned_conflicts(root: &Path, plan: &Plan) -> Vec<(String, String, usize, String)> {
    let mut found = vec![];
    for written in &plan.files {
        let Ok(text) = std::str::from_utf8(&written.bytes) else {
            continue;
        };
        if !crate::records::holds_a_record(root, &written.path) {
            continue;
        }
        if let Some((line, marker)) = crate::records::conflict_markers(text).into_iter().next() {
            let source = plan
                .carried
                .becomes
                .iter()
                .find(|(_, targets)| targets.contains(&written.path))
                .map_or_else(|| written.path.clone(), |(source, _)| source.clone());
            found.push((source, written.path.clone(), line, marker));
        }
    }
    found
}

/// The refusal `--apply` gives, and the preview names up front, while a planned record
/// would hold a Git conflict marker: the 0.5.x file to resolve, not only the 0.6 target.
fn conflict_refusal(root: &Path, plan: &Plan) -> Option<String> {
    let conflicts = planned_conflicts(root, plan);
    let (source, target, line, marker) = conflicts.first()?;
    let more = match conflicts.len() {
        1 => String::new(),
        count => format!(" ({} more files hold one too)", count - 1),
    };
    Some(format!(
        "refused:conflict-markers: {source} holds a Git conflict marker, which would migrate into {target} at line {line} (`{marker}`){more}; resolve the conflict in {source} by hand and commit it, then run `chough migrate` again; nothing was written"
    ))
}

/// Build the plan under the first receipt name that is free for it (REQ-records-009,
/// #264): `<date>-from-0-5.md`, then `<date>-from-0-5-2.md` and so on. A name is free
/// when no file holds it, or when the receipt there is this plan's own, left by an
/// interrupted apply. The name is part of the plan, so each candidate has its own
/// digest, and the same tree always gives the same name and digest.
fn plan_with_a_free_receipt(root: &Path) -> Result<(Plan, String)> {
    for ordinal in 1..=RECEIPT_ORDINALS {
        RECEIPT_ORDINAL.store(ordinal, Ordering::Relaxed);
        let plan = plan(root)?;
        let digest = plan.digest();
        let taken = match at_target(root, &plan.receipt)? {
            AtTarget::Absent => false,
            AtTarget::Bytes(existing) => !is_own_receipt(&existing, &digest),
            AtTarget::Other(_) => true,
        };
        if !taken || plan.sources.is_empty() {
            return Ok((plan, digest));
        }
    }
    bail!(
        "refused:receipt-names: {RECEIPT_ORDINALS} migration receipts already exist for today, so nothing was written. Run `chough migrate` again tomorrow (UTC)."
    )
}

/// The preview as text, for the person who consents to it (REQ-records-009, F12b): the
/// plan the JSON gives, section by section with `none` under an empty one, then any
/// refusal the apply would give now, the plan's digest and the apply command.
fn preview_text(view: &Value) -> String {
    fn section(out: &mut String, heading: &str, items: Vec<String>) {
        out.push_str(&format!("\n{heading} ({}):\n", items.len()));
        if items.is_empty() {
            out.push_str("none\n");
        }
        for item in items {
            out.push_str(&format!("- {item}\n"));
        }
    }
    let text = |value: &Value| value.as_str().unwrap_or_default().to_string();
    let map = |key: &str| view[key].as_object().cloned().unwrap_or_default();
    let list = |key: &str| view[key].as_array().cloned().unwrap_or_default();
    let mut out =
        String::from("Migration plan from Chough 0.5.x to 0.6.0. This preview writes nothing.\n");
    let becomes = map("becomes");
    let removed = map("removed");
    section(
        &mut out,
        "Reads",
        map("sources")
            .iter()
            .map(|(path, sha)| {
                let targets: Vec<String> = becomes
                    .get(path)
                    .and_then(Value::as_array)
                    .into_iter()
                    .flatten()
                    .map(|target| format!("`{}`", text(target)))
                    .collect();
                let fate = if !targets.is_empty() {
                    format!("becomes {}", targets.join(", "))
                } else if removed.contains_key(path) {
                    "is removed without carrying (below)".to_string()
                } else {
                    "becomes no file".to_string()
                };
                format!("`{path}` {} {fate}", text(sha))
            })
            .collect(),
    );
    section(
        &mut out,
        "Writes",
        list("writes")
            .iter()
            .map(|write| {
                format!(
                    "`{}` {} bytes {}",
                    text(&write["path"]),
                    write["bytes"],
                    text(&write["sha256"])
                )
            })
            .collect(),
    );
    section(
        &mut out,
        "Replaces, as `chough library index` does",
        list("replaces")
            .iter()
            .map(|path| format!("`{}`: rebuilt from its sources", text(path)))
            .collect(),
    );
    section(
        &mut out,
        "Redactions",
        list("redactions")
            .iter()
            .map(|redaction| {
                format!(
                    "`{}` {}: {}",
                    text(&redaction["file"]),
                    text(&redaction["field"]),
                    text(&redaction["kind"])
                )
            })
            .collect(),
    );
    section(
        &mut out,
        "Removes without carrying",
        removed
            .iter()
            .map(|(path, why)| format!("`{path}`: {}", text(why)))
            .collect(),
    );
    section(
        &mut out,
        "Keeps byte for byte",
        map("kept")
            .iter()
            .map(|(path, why)| format!("`{path}`: {}", text(why)))
            .collect(),
    );
    section(
        &mut out,
        "Kept with leaks, to fix by hand",
        map("kept_with_leaks")
            .iter()
            .map(|(path, kinds)| {
                let kinds: Vec<String> = kinds.as_array().into_iter().flatten().map(text).collect();
                format!("`{path}`: {}", kinds.join(", "))
            })
            .collect(),
    );
    section(
        &mut out,
        "Not carried",
        list("gaps").iter().map(text).collect(),
    );
    section(
        &mut out,
        "Unaccounted, which the apply refuses",
        list("unaccounted")
            .iter()
            .map(|path| format!("`{}`", text(path)))
            .collect(),
    );
    section(
        &mut out,
        "Aliases",
        map("aliases")
            .iter()
            .map(|(old, new)| format!("`{old}` -> `{}`", text(new)))
            .collect(),
    );
    section(
        &mut out,
        "Follow-ups",
        list("follow_ups").iter().map(text).collect(),
    );
    out.push_str(&format!("\n{}\n", text(&view["git_history"])));
    if view["apply_refused"].is_object() {
        out.push_str(&format!(
            "\nThe apply would refuse now ({}): {}\n",
            text(&view["apply_refused"]["refusal"]),
            text(&view["apply_refused"]["why"])
        ));
    }
    out.push_str(&format!(
        "\nReceipt: {}\nPlan digest: {}\n",
        text(&view["receipt"]),
        text(&view["plan_digest"])
    ));
    // #339: a plan the apply would refuse is not offered to apply.
    if !view["apply_refused"].is_object() {
        out.push_str(&format!(
            "Apply this plan with the human's consent:\n  {}\n",
            text(&view["apply"])
        ));
    }
    out
}

/// What is at a path the migration would write, read without following a link and
/// without waiting (REQ-records-009, #337).
enum AtTarget {
    Absent,
    /// A regular file, with its bytes.
    Bytes(Vec<u8>),
    /// Anything else, which counts as existing with other bytes: what it is.
    Other(&'static str),
}

/// Read what is at `relative`: a directory, link, FIFO or other special file is named,
/// never opened, and a regular file is opened without waiting (#337).
fn at_target(root: &Path, relative: &str) -> Result<AtTarget> {
    let path = local_path(root, relative)?;
    let kind = match std::fs::symlink_metadata(&path) {
        Ok(meta) => meta.file_type(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(AtTarget::Absent),
        Err(_) => return Ok(AtTarget::Other("a path it cannot read")),
    };
    if kind.is_dir() {
        return Ok(AtTarget::Other("a directory"));
    }
    if kind.is_symlink() {
        return Ok(AtTarget::Other("a symbolic link"));
    }
    if !kind.is_file() {
        return Ok(AtTarget::Other("a special file, such as a FIFO"));
    }
    let read = crate::records::open_regular(&path).and_then(|mut file| {
        let mut bytes = vec![];
        std::io::Read::read_to_end(&mut file, &mut bytes)?;
        Ok(bytes)
    });
    Ok(match read {
        Ok(bytes) => AtTarget::Bytes(bytes),
        Err(_) => AtTarget::Other("a file it cannot read"),
    })
}

/// Whether a receipt's bytes are this plan's own, left by an interrupted apply.
fn is_own_receipt(bytes: &[u8], digest: &str) -> bool {
    String::from_utf8_lossy(bytes).contains(&format!("plan_digest: {digest}"))
}

/// The refusal the apply gives, and the preview names up front, while anything at a
/// path the apply would write, the receipt included, is not the planned bytes, a
/// generated file the plan replaces, a planned removal under another letter case, or
/// this plan's own receipt (REQ-records-009, #337, #264). Paths are project-relative.
fn target_refusal(root: &Path, plan: &Plan, digest: &str) -> Result<Option<String>> {
    let mut problems = vec![];
    for written in &plan.files {
        match at_target(root, &written.path)? {
            AtTarget::Absent => {}
            AtTarget::Bytes(bytes) if bytes == written.bytes => {}
            AtTarget::Bytes(_) if plan.replaces.contains(&written.path) => {}
            AtTarget::Bytes(_) if is_a_planned_removal_by_case(root, plan, &written.path)? => {}
            AtTarget::Bytes(_) => {
                problems.push(format!("{} already exists with other bytes", written.path));
            }
            AtTarget::Other(what) => problems.push(format!("{} is {what}", written.path)),
        }
    }
    match at_target(root, &plan.receipt)? {
        AtTarget::Absent => {}
        AtTarget::Bytes(bytes) if is_own_receipt(&bytes, digest) => {}
        AtTarget::Bytes(_) => {
            problems.push(format!("{} already exists for another plan", plan.receipt));
        }
        AtTarget::Other(what) => problems.push(format!("{} is {what}", plan.receipt)),
    }
    if problems.is_empty() {
        return Ok(None);
    }
    Ok(Some(format!(
        "{}, and the migration writes only where nothing is, so nothing was written. Move or remove it, then run `chough migrate` again.",
        problems.join("; ")
    )))
}

/// The suffix of the temporary file a planned file is written to before its rename.
pub(super) const MIGRATING: &str = ".migrating";

/// Write one planned file under a temporary name in its folder and give it its name by
/// rename, replacing what is there (#338): a file whose name differs only in case from
/// a planned removal, or a generated file the plan replaces. Then read it back.
fn write_by_rename(root: &Path, written: &Planned, written_now: &mut Vec<String>) -> Result<()> {
    let path = local_path(root, &written.path)?;
    let listed = path
        .parent()
        .zip(path.file_name())
        .is_some_and(|(folder, name)| {
            std::fs::read_dir(folder)
                .is_ok_and(|entries| entries.flatten().any(|entry| entry.file_name() == name))
        });
    let done = listed
        && matches!(at_target(root, &written.path)?, AtTarget::Bytes(bytes) if bytes == written.bytes);
    if !done {
        let folder = path.parent().context("a planned file has no folder")?;
        std::fs::create_dir_all(folder)?;
        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        // A fixed name, which the next plan skips (REQ-records-009); one a stopped apply
        // left is removed first. The writer lock keeps a second apply out meanwhile.
        let temporary = folder.join(format!(".{name}{MIGRATING}"));
        if std::fs::symlink_metadata(&temporary).is_ok_and(|meta| meta.is_file()) {
            std::fs::remove_file(&temporary)?;
        }
        {
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&temporary)?;
            std::io::Write::write_all(&mut file, &written.bytes)?;
            file.sync_all()?;
        }
        if let Err(error) = std::fs::rename(&temporary, &path) {
            let _ = std::fs::remove_file(&temporary);
            bail!("{} could not take its name ({error})", written.path);
        }
        // On a file system that folds case the rename replaces the file's bytes but can
        // keep its old spelling, as `knowledge/INDEX.md`; a case-only rename then gives it
        // the planned one (#338).
        if let Some(folder) = path.parent()
            && let Some(name) = path.file_name()
            && let Ok(entries) = std::fs::read_dir(folder)
        {
            let names: Vec<std::ffi::OsString> =
                entries.flatten().map(|entry| entry.file_name()).collect();
            if !names.iter().any(|listed| listed == name)
                && let Some(listed) = names.iter().find(|listed| {
                    listed
                        .to_string_lossy()
                        .eq_ignore_ascii_case(&name.to_string_lossy())
                })
            {
                // Through a name of its own, since a file system that folds case may take
                // a case-only rename as no change at all, as a Docker Desktop mount of a
                // macOS folder does (tester2's finding).
                let between = folder.join(format!(".{}{MIGRATING}", name.to_string_lossy()));
                std::fs::rename(folder.join(listed), &between)?;
                std::fs::rename(&between, &path)?;
            }
            ensure!(
                listed_exactly(&path),
                "{} could not be given its name in that letter case",
                written.path
            );
        }
        written_now.push(written.path.clone());
    }
    match at_target(root, &written.path)? {
        AtTarget::Bytes(back) if back == written.bytes => Ok(()),
        _ => bail!("{} did not read back as planned", written.path),
    }
}

/// Whether the folder of `path` lists its exact name, letter case included.
fn listed_exactly(path: &Path) -> bool {
    path.parent()
        .zip(path.file_name())
        .is_some_and(|(folder, name)| {
            std::fs::read_dir(folder)
                .is_ok_and(|entries| entries.flatten().any(|entry| entry.file_name() == name))
        })
}

/// How many files a folder holds, all of them, for a macOS metadata folder.
fn files_under(path: &Path) -> u64 {
    std::fs::read_dir(path).map_or(0, |entries| {
        entries
            .flatten()
            .map(|entry| match entry.file_type() {
                Ok(kind) if kind.is_dir() => files_under(&entry.path()),
                _ => 1,
            })
            .sum()
    })
}

/// The number of macOS metadata files a folder holds when it holds nothing else, in it
/// or in any folder under it (REQ-records-015); `None` when it holds anything else.
fn only_metadata(path: &Path) -> Option<u64> {
    if !std::fs::symlink_metadata(path).ok()?.is_dir() {
        return None;
    }
    let mut count = 0;
    for entry in std::fs::read_dir(path).ok()?.flatten() {
        let kind = entry.file_type().ok()?;
        if crate::evidence::is_os_metadata(&entry.file_name().to_string_lossy()) {
            count += if kind.is_dir() {
                files_under(&entry.path())
            } else {
                1
            };
        } else if kind.is_dir() {
            count += only_metadata(&entry.path())?;
        } else {
            return None;
        }
    }
    Some(count)
}

/// #375 (REQ-records-015): once the apply has succeeded, remove each folder it emptied of
/// its 0.5.x records that holds nothing but macOS metadata, with that metadata, highest
/// such folder first, and return each with the number of metadata files it held. A
/// folder that holds anything else, and `.chough` itself, stays.
fn remove_emptied_folders<'a>(
    root: &Path,
    removed: impl Iterator<Item = &'a str>,
) -> Result<Vec<(String, u64)>> {
    let mut folders = BTreeSet::new();
    for path in removed {
        let mut folder = Path::new(path).parent();
        while let Some(relative) = folder.filter(|folder| !folder.as_os_str().is_empty()) {
            folders.insert(relative.to_string_lossy().into_owned());
            folder = relative.parent();
        }
    }
    let mut ordered: Vec<String> = folders.into_iter().collect();
    ordered.sort_by_key(|folder| folder.matches('/').count());
    let mut emptied: Vec<(String, u64)> = vec![];
    for folder in ordered {
        if folder == ".chough"
            || emptied
                .iter()
                .any(|(gone, _)| folder.starts_with(&format!("{gone}/")))
        {
            continue;
        }
        let local = local_path(root, &folder)?;
        if let Some(count) = only_metadata(&local).filter(|count| *count > 0) {
            std::fs::remove_dir_all(&local)?;
            emptied.push((folder, count));
        }
    }
    Ok(emptied)
}

/// Where earlier migrations' receipts are, the only place a receipt is (REQ-records-009).
const RECEIPTS: &str = ".chough/archive/migrations";

/// A receipt of an earlier migration, one Git holds committed with the bytes on disk:
/// the files it read with their SHA-256, and what each removed file was carried into.
struct EarlierReceipt {
    path: String,
    read: BTreeMap<String, String>,
    carried: BTreeMap<String, Vec<String>>,
}

/// The `- `path` <sha256>` lines of a receipt's `## Before` section: the files it read,
/// or `None` when it has no such section.
fn files_read(text: &str) -> Option<BTreeMap<String, String>> {
    // A receipt checked out with CRLF line ends reads the same.
    let text = &text.replace("\r\n", "\n");
    let start = text.find("\n## Before\n")? + "\n## Before\n".len();
    let section = &text[start..];
    let section = &section[..section.find("\n## ").unwrap_or(section.len())];
    Some(
        section
            .lines()
            .filter_map(|line| {
                let rest = line.strip_prefix("- `")?;
                let (path, sha) = rest.split_once("` ")?;
                Some((path.to_string(), sha.trim().to_string()))
            })
            .collect(),
    )
}

/// What each removed file of a receipt's `## Removed` section was carried into.
fn carried_into(text: &str) -> BTreeMap<String, Vec<String>> {
    let text = &text.replace("\r\n", "\n");
    let Some(start) = text.find("\n## Removed\n") else {
        return BTreeMap::new();
    };
    let section = &text[start + "\n## Removed\n".len()..];
    let section = &section[..section.find("\n## ").unwrap_or(section.len())];
    section
        .lines()
        .filter_map(|line| {
            let rest = line.strip_prefix("- `")?;
            let (path, why) = rest.split_once("`: ")?;
            let into = why.strip_prefix("carried into ")?;
            let into = into.split("; held by commit").next().unwrap_or(into);
            let targets = into
                .split(", ")
                .filter_map(|target| target.strip_prefix('`')?.strip_suffix('`'))
                .map(str::to_string)
                .collect();
            Some((path.to_string(), targets))
        })
        .collect()
}

/// Whether Git holds `relative` at HEAD, whatever is on disk.
fn held_by_git(root: &Path, relative: &str) -> bool {
    crate::records::run_owned(
        std::process::Command::new("git").arg("-C").arg(root).args([
            "cat-file",
            "-e",
            &format!("HEAD:{relative}"),
        ]),
        None,
    )
    .is_ok_and(|output| output.status.success())
}

/// Every receipt of an earlier migration (REQ-records-009, #370, #337): each entry of
/// the receipt folder whose name ends in `.md`, leaving out macOS metadata, read without
/// waiting. An entry that is not a receipt Chough can read refuses, naming it; a receipt
/// Git has never held is this migration's own and judges nothing; one Git holds whose
/// bytes on disk differ from the last commit refuses, naming it.
fn earlier_receipts(root: &Path) -> Result<Vec<EarlierReceipt>> {
    let Ok(entries) = std::fs::read_dir(local_path(root, RECEIPTS)?) else {
        return Ok(vec![]);
    };
    let mut names: Vec<String> = entries
        .flatten()
        .filter_map(|entry| entry.file_name().into_string().ok())
        .filter(|name| name.ends_with(".md") && !crate::evidence::is_os_metadata(name))
        .collect();
    names.sort();
    let (mut unreadable, mut edited, mut judged) = (vec![], vec![], vec![]);
    for name in names {
        let path = format!("{RECEIPTS}/{name}");
        // The bound is checked from the file's length before anything is read.
        if std::fs::symlink_metadata(local_path(root, &path)?)
            .is_ok_and(|meta| meta.is_file() && meta.len() > crate::markdown::RECEIPT_BOUND)
        {
            unreadable.push(format!(
                "{path} is over the {}-byte receipt bound",
                crate::markdown::RECEIPT_BOUND
            ));
            continue;
        }
        let bytes = match at_target(root, &path)? {
            AtTarget::Absent => continue,
            AtTarget::Other(what) => {
                unreadable.push(format!("{path} is {what}"));
                continue;
            }
            AtTarget::Bytes(bytes) => bytes,
        };
        if bytes.len() as u64 > crate::markdown::RECEIPT_BOUND {
            unreadable.push(format!(
                "{path} is over the {}-byte receipt bound",
                crate::markdown::RECEIPT_BOUND
            ));
            continue;
        }
        let Ok(text) = String::from_utf8(bytes) else {
            unreadable.push(format!("{path} is not UTF-8"));
            continue;
        };
        let Some(read) = files_read(&text) else {
            unreadable.push(format!("{path} has no list of files read"));
            continue;
        };
        if !held_by_git(root, &path) {
            continue;
        }
        if !crate::sessions::committed_at_head(root, &path) {
            edited.push(path);
            continue;
        }
        judged.push(EarlierReceipt {
            path,
            read,
            carried: carried_into(&text),
        });
    }
    ensure!(
        unreadable.is_empty(),
        "refused:migration-receipt: {}, so it is not a migration receipt Chough can read and the returning 0.5.x files cannot be judged by it; nothing was written, and the apply would refuse too. Every `.md` entry of {RECEIPTS}/ is read as a receipt, so move or remove it, then run `chough migrate` again.",
        unreadable.join("; ")
    );
    ensure!(
        edited.is_empty(),
        "refused:migration-receipt: {} is held by Git but differs on disk from its last commit, as a receipt edited, half merged or redacted and not committed is, so the returning 0.5.x files cannot be judged by it; nothing was written, and the apply would refuse too. Commit it, or restore it from the last commit with `git restore -- {}`, then run `chough migrate` again.",
        edited.join(", "),
        edited
            .iter()
            .map(|path| crate::sessions::shell_quoted(path))
            .collect::<Vec<_>>()
            .join(" ")
    );
    Ok(judged)
}

/// #370 (REQ-records-009): the 0.5.x files a committed receipt of an earlier migration
/// lists with these bytes, each with why it is removed as already migrated. While any
/// 0.5.x file present is listed with other bytes, refuse, naming each with its receipt
/// and the way on, so an edited or different record never becomes a second one and is
/// never dropped as if it had no place.
fn already_migrated_files(root: &Path, legacy: &[String]) -> Result<BTreeMap<String, String>> {
    let mut already = BTreeMap::new();
    if legacy.is_empty() {
        return Ok(already);
    }
    let receipts = earlier_receipts(root)?;
    let mut different = vec![];
    for path in legacy {
        let listing: Vec<&EarlierReceipt> = receipts
            .iter()
            .filter(|receipt| receipt.read.contains_key(path))
            .collect();
        let Some(first) = listing.first() else {
            continue;
        };
        let AtTarget::Bytes(bytes) = at_target(root, path)? else {
            continue;
        };
        let sha = sha256(&bytes);
        if let Some(receipt) = listing.iter().find(|receipt| receipt.read[path] == sha) {
            already.insert(
                path.clone(),
                format!(
                    "already migrated: {} lists it with these bytes, so it is not carried again, whether or not the record it was carried into is still there",
                    receipt.path
                ),
            );
            continue;
        }
        let way = if path == ".chough/session.json" {
            "it is the fixed path of whichever session was current, so start what you want of the session it holds as a new one with `chough session new <key>`".to_string()
        } else {
            match first
                .carried
                .get(path)
                .filter(|targets| !targets.is_empty())
            {
                Some(targets) => format!(
                    "carry what you want from it into {} by hand",
                    targets
                        .iter()
                        .map(|target| format!("`{target}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                None => "carry what you want from it into the 0.6.0 records by hand".to_string(),
            }
        };
        different.push(format!(
            "- `{path}`, which {} lists with other bytes: {way}",
            first.path
        ));
    }
    ensure!(
        different.is_empty(),
        "refused:already-migrated: a 0.5.x file an earlier migration read came back with other bytes, so nothing was written, and the apply would refuse too; it never becomes a second record and is never dropped as if it had no place:\n{}\nThen remove each such file, commit, and run `chough migrate` again.",
        different.join("\n")
    );
    Ok(already)
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
        "refused:change-open: {}",
        change_open_why(
            &open,
            "the migration never runs while a change is open, so nothing was written"
        )
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
    // REQ-records-001 (#265, #339): no record is written with a Git conflict marker in
    // its body; every planned file is checked before any is written, so nothing is, and
    // the refusal names the 0.5.x file to resolve.
    if let Some(refusal) = conflict_refusal(root, plan) {
        bail!("{refusal}");
    }
    // REQ-records-009 (#337, #264): every file the apply would write, the receipt
    // included, is checked before any is written, read without following a link and
    // without waiting; anything at such a path but the planned bytes, a generated file
    // the plan replaces, or this plan's own receipt refuses here, writing nothing.
    if let Some(refusal) = target_refusal(root, plan, digest)? {
        bail!("refused:target-exists: {refusal}");
    }
    // The receipt is known before anything is written: one over its own bound refuses
    // here, writing nothing (REQ-records-001, REQ-records-009).
    let receipt = plan.receipt.clone();
    let receipt_text = receipt_record(plan, digest, &consent, &removals, root).map_err(|error| {
        anyhow::anyhow!(
            "refused:receipt-bound: the migration receipt would exceed its {} byte bound, so nothing was written: {error:#}",
            crate::markdown::RECEIPT_BOUND
        )
    })?;
    // Every refusal above wrote nothing; the writer lock, taken before the plan was
    // built, is held through the writes, the removals, the archive reads and any restore,
    // so no other writer saves in between (REQ-cli-040).
    //
    // Write every planned file, the receipt included, before any 0.5.x file is removed
    // (#338): create-new, or count a file an interrupted apply wrote with the planned
    // bytes as done. A file whose name differs only in case from a planned removal, and
    // a generated file the plan replaces, is written under a temporary name in its folder
    // and takes its name by rename, so a file system that folds case cannot leave the
    // sources removed and the planned file missing. The 0.5.x file such a rename replaces
    // on a file system that folds case is kept in memory, to be listed as removed and
    // restored with the others.
    // First every planned file written where nothing is, by create-new. The receipt
    // follows. Every write that replaces a file already there comes last (below): one
    // whose name differs only in case from a 0.5.x file the plan removes, and a generated
    // file the plan replaces. So a failure before them leaves every file that was there
    // byte for byte, and the plan, which `replaces` is part of, unchanged.
    let replaces_one = |path: &str| {
        collides_by_case(plan, path) || plan.replaces.iter().any(|replaced| replaced == path)
    };
    let mut written_now = vec![];
    for written in plan
        .files
        .iter()
        .filter(|written| !replaces_one(&written.path))
    {
        write_planned(root, written, &mut written_now)?;
    }
    let receipt_local = local_path(root, &receipt)?;
    match at_target(root, &receipt)? {
        AtTarget::Bytes(existing) => ensure!(
            is_own_receipt(&existing, digest),
            "refused:target-exists: {receipt} appeared while migrating, for another plan, so nothing was removed"
        ),
        AtTarget::Other(what) => bail!(
            "refused:target-exists: {receipt} appeared while migrating as {what}, so nothing was removed"
        ),
        AtTarget::Absent => {
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
    // Last, by rename, every write that replaces a file already there: a generated file
    // under `replaces`, and a file whose name differs only in case from a 0.5.x file the
    // plan removes (knowledge/index.md beside knowledge/INDEX.md). On a file system that
    // folds case those two are one file, so that rename is the 0.5.x file's removal: it
    // comes only once every other planned file and the receipt hold their bytes (#338),
    // and the 0.5.x bytes it replaces are kept in memory to be listed as removed and
    // restored.
    let mut replaced_by_case: Vec<(String, Vec<u8>)> = vec![];
    for written in plan
        .files
        .iter()
        .filter(|written| replaces_one(&written.path))
    {
        // The 0.5.x file of another letter case, with its planned bytes, read before the
        // rename. Whether the rename replaced it is told afterwards, from what the folder
        // lists: before the planned file exists, two names in one folder cannot be told
        // apart from one file (#338, tester2's finding on Linux).
        let mut before: Vec<(String, Vec<u8>)> = vec![];
        for removed in plan
            .carried
            .removed
            .keys()
            .chain(plan.carried.sources.iter())
            .filter(|removed| {
                **removed != written.path && removed.eq_ignore_ascii_case(&written.path)
            })
        {
            if let AtTarget::Bytes(bytes) = at_target(root, removed)?
                && plan
                    .sources
                    .get(removed)
                    .is_some_and(|sha| *sha == sha256(&bytes))
            {
                before.push((removed.clone(), bytes));
            }
        }
        // A failure after the rename has happened, on a file system that folds case, has
        // already replaced the 0.5.x file of another letter case: say so, and how to get
        // it back from the last commit, since no other 0.5.x file was removed (F2).
        if let Err(error) = write_by_rename(root, written, &mut written_now) {
            let replaced: Vec<&String> = before
                .iter()
                .map(|(removed, _)| removed)
                .filter(|removed| {
                    local_path(root, removed).is_ok_and(|path| !listed_exactly(&path))
                })
                .collect();
            if replaced.is_empty() {
                return Err(error.context("no 0.5.x file was removed"));
            }
            return Err(error.context(format!(
                "on this file system, which folds case, that write replaced {}; no other 0.5.x file was removed. Get it back from the last commit with `git restore -- {}`, then run `chough migrate` again",
                replaced.iter().map(|path| path.as_str()).collect::<Vec<_>>().join(", "),
                replaced
                    .iter()
                    .map(|path| crate::sessions::shell_quoted(path))
                    .collect::<Vec<_>>()
                    .join(" ")
            )));
        }
        for (removed, bytes) in before {
            if !listed_exactly(&local_path(root, &removed)?) {
                replaced_by_case.push((removed, bytes));
            }
        }
    }
    // Never weaken an archive: read the validity of every archived change whose
    // acceptance manifest lists a file to remove, remove, read again, and restore
    // everything if any that read valid no longer does.
    let affected = affected_archives(root, &removals)?;
    let before = validity(root, &affected)?;
    let mut kept_bytes: Vec<(String, Vec<u8>)> = replaced_by_case;
    for path in &removals {
        if kept_bytes.iter().any(|(kept, _)| kept == path) {
            continue;
        }
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
    let emptied = remove_emptied_folders(root, kept_bytes.iter().map(|(path, _)| path.as_str()))?;
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
        "emptied_folders": emptied
            .iter()
            .map(|(folder, count)| json!({"folder": folder, "metadata_files": count}))
            .collect::<Vec<_>>(),
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

/// Whether the file at the planned `path` is one file with a 0.5.x source of another
/// letter case, as `knowledge/index.md` is `knowledge/INDEX.md` on a file system that
/// folds case: its other bytes are the source's, not a file to replace.
fn one_file_by_case_with_a_source(root: &Path, carried: &Carried, path: &str) -> Result<bool> {
    let target = local_path(root, path)?;
    for source in carried
        .removed
        .keys()
        .chain(carried.sources.iter())
        .filter(|source| *source != path && source.eq_ignore_ascii_case(path))
    {
        if one_file_by_case(&target, &local_path(root, source)?) {
            return Ok(true);
        }
    }
    Ok(false)
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

/// Whether the file read at the planned `path` is a planned removal under its other
/// spelling, so its other bytes are the 0.5.x file's and not a target in the way
/// (REQ-records-009, #263). That holds only where the two names are one file: a
/// removal that differs only in case still opens, and the folder does not list both
/// names. On a case-sensitive file system `knowledge/index.md` beside
/// `knowledge/INDEX.md` is a file of its own, and the apply refuses it before writing.
fn is_a_planned_removal_by_case(root: &Path, plan: &Plan, path: &str) -> Result<bool> {
    let target = local_path(root, path)?;
    for removed in plan
        .carried
        .removed
        .keys()
        .chain(plan.carried.sources.iter())
        .filter(|removed| *removed != path && removed.eq_ignore_ascii_case(path))
    {
        if one_file_by_case(&target, &local_path(root, removed)?) {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Whether `target` and `removal`, two paths in one folder that differ only in case,
/// name one file: `removal` opens, and the folder does not list both names.
fn one_file_by_case(target: &Path, removal: &Path) -> bool {
    let listed = |path: &Path| {
        path.parent()
            .zip(path.file_name())
            .is_some_and(|(folder, name)| {
                std::fs::read_dir(folder)
                    .is_ok_and(|entries| entries.flatten().any(|entry| entry.file_name() == name))
            })
    };
    removal.exists() && !(listed(target) && listed(removal))
}

/// Write one planned file by create-new, or count it done when it already holds the
/// planned bytes, then read it back.
fn write_planned(root: &Path, written: &Planned, written_now: &mut Vec<String>) -> Result<()> {
    let path = local_path(root, &written.path)?;
    let done =
        matches!(at_target(root, &written.path)?, AtTarget::Bytes(bytes) if bytes == written.bytes);
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

/// Why the apply refuses while native changes are open: each by name, one per line, and
/// the command that finishes and archives one.
fn change_open_why(open: &[String], refuses: &str) -> String {
    let (these, are) = if open.len() == 1 {
        ("this native change", "is")
    } else {
        ("these native changes", "are")
    };
    format!(
        "{these} {are} not archived, and {refuses}:\n{}\nFinish each and archive it with `chough change ship <id>` (or archive it with SpecSync), then run `chough migrate` again.",
        open.iter()
            .map(|id| format!("- {id}"))
            .collect::<Vec<_>>()
            .join("\n")
    )
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

/// Whether the folder at `path` holds nothing at all. macOS's rmdir deletes orphaned
/// AppleDouble `._*` files in a folder it removes, so a folder is removed only when this
/// holds, and the migration never removes macOS metadata (REQ-records-015).
fn holds_nothing(path: &Path) -> bool {
    std::fs::read_dir(path).is_ok_and(|mut entries| entries.next().is_none())
}

/// Remove each folder a removal left empty, from the removed file's own folder up to,
/// never including, the project root: a session rolled into the vault leaves no empty
/// `knowledge/attachments/<folder>/sessions/` behind. A folder that still holds anything,
/// macOS metadata included, stays.
fn prune_emptied<'a>(root: &Path, removed: impl Iterator<Item = &'a str>) {
    for path in removed {
        let mut folder = Path::new(path).parent();
        while let Some(relative) = folder.filter(|folder| !folder.as_os_str().is_empty()) {
            let Ok(local) = local_path(root, &relative.to_string_lossy()) else {
                break;
            };
            if !holds_nothing(&local) || std::fs::remove_dir(local).is_err() {
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
                // A macOS metadata folder is neither entered nor removed, so the folder
                // that holds it stays too (REQ-records-015, #321 review).
                if crate::evidence::is_os_metadata(&entry.file_name().to_string_lossy()) {
                    continue;
                }
                if entry.path().is_dir() {
                    prune(&entry.path());
                }
            }
        }
        if holds_nothing(path) {
            let _ = std::fs::remove_dir(path);
        }
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

/// What a receipt section with nothing to list says (REQ-records-009).
const NOTHING_LISTED: &str = "none\n";

/// The migration receipt: every file before and after with its size and SHA-256, the
/// redactions, every removed file with why and the commit that holds it, the alias map
/// and the consent (REQ-records-009), with `none` under a section with nothing to list.
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
        "# Migration from Chough 0.5.x\n\nGit history still holds what earlier commits recorded; this migration does not rewrite it.\n",
    );
    let section = |body: &mut String, heading: &str, items: Vec<String>| {
        body.push_str(&format!("\n## {heading}\n\n"));
        if items.is_empty() {
            body.push_str(NOTHING_LISTED);
        }
        for item in items {
            body.push_str(&item);
        }
    };
    section(
        &mut body,
        "Before",
        plan.sources
            .iter()
            .map(|(path, sha)| format!("- `{path}` {sha}\n"))
            .collect(),
    );
    section(
        &mut body,
        "After",
        plan.files
            .iter()
            .map(|written| {
                format!(
                    "- `{}` {} bytes {}\n",
                    written.path,
                    written.bytes.len(),
                    sha256(&written.bytes)
                )
            })
            .collect(),
    );
    section(
        &mut body,
        "Redactions",
        plan.redactions
            .iter()
            .map(|redaction| {
                format!(
                    "- `{}` {}: {}\n",
                    redaction.file, redaction.field, redaction.kind
                )
            })
            .collect(),
    );
    let becomes = plan.becomes();
    let mut removed = vec![];
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
        removed.push(format!("- `{path}`: {why}; held by commit {head}\n"));
    }
    section(&mut body, "Removed", removed);
    section(
        &mut body,
        "Kept with leaks",
        plan.carried
            .kept_with_leaks
            .iter()
            .map(|(path, kinds)| format!("- `{path}`: {} (fix by hand)\n", kinds.join(", ")))
            .collect(),
    );
    section(
        &mut body,
        "Not carried",
        plan.carried
            .gaps
            .iter()
            .map(|gap| format!("- {gap}\n"))
            .collect(),
    );
    section(
        &mut body,
        "Aliases",
        plan.carried
            .aliases
            .iter()
            .map(|(old, new)| format!("- `{old}` → `{new}`\n"))
            .collect(),
    );
    // One Follow-up section holds every follow-up, one paragraph each, so a receipt
    // never repeats the heading; with none, it has no such section.
    if !plan.follow_ups.is_empty() {
        body.push_str(&format!(
            "\n## Follow-up\n\n{}\n",
            plan.follow_ups.join("\n\n")
        ));
    }
    let mut record = crate::markdown::parse("receipt", &format!("{}{body}", record.render()))?;
    record.redact_leaks(Local::current());
    record.render_within(crate::markdown::Kind::Receipt)
}

/// Build the plan: run every mapper in order, redact what they write and list every
/// 0.5.x file read.
fn plan(root: &Path) -> Result<Plan> {
    // #370: while any 0.5.x file is present, every receipt of an earlier migration is
    // read first, and a file one lists with these bytes is left out of every mapper.
    let already = already_migrated_files(root, &accountable_paths(root)?)?;
    *ALREADY_MIGRATED
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner) = already.keys().cloned().collect();
    let mut carried = Carried::default();
    let (sessions, homes) = sessions_0_5(root)?;
    carried.absorb(sessions)?;
    carried.absorb(retired_files_0_5(root)?)?;
    // REQ-records-002: the reading guide, written only when there is none; one an
    // interrupted apply already wrote with these bytes stays planned, so the plan the
    // resumed apply builds has the same digest. A migrated guide names `artifacts/`.
    let readme = crate::sessions::migrated_readme();
    let planned_guide = match at_target(root, ".chough/README.md")? {
        AtTarget::Absent => true,
        AtTarget::Bytes(bytes) => bytes == readme.as_bytes(),
        AtTarget::Other(_) => true,
    };
    if planned_guide {
        carried.written.push(Written {
            path: ".chough/README.md".into(),
            bytes: readme,
        });
    }
    let links = bug_links_0_5(root)?;
    let (evidence, observations) = evidence_0_5(root, &homes, &links)?;
    carried.absorb(evidence)?;
    story_receipts(&mut carried, &homes, &observations)?;
    session_verified_links(&mut carried, &homes, &observations)?;
    observation_names(&mut carried, &homes, &observations)?;
    // #277: a bug's body links a receipt to the archived story that holds it, the
    // first in session order when several do.
    let mut observations = observations;
    for home in homes.values().filter(|home| home.archived) {
        for id in &home.observations {
            if observations.receipts.contains_key(id) {
                observations
                    .stories
                    .entry(id.clone())
                    .or_insert_with(|| format!(".chough/archive/sessions/{}.md", home.id));
            }
        }
    }
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
    // #370: a file removed as already migrated is never carried, whatever a mapper read.
    for (path, why) in &already {
        carried.sources.remove(path);
        carried.becomes.remove(path);
        carried.removed.insert(path.clone(), why.clone());
    }
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
    // REQ-records-009, REQ-cli-016 (F3): the installed skill copy is not migrated, so
    // while it is not current, as `chough skill status` reports it, the plan names
    // `chough skill update`; the copy still names 0.5.x commands and paths until then.
    follow_ups.extend(crate::skill::update_advice(root)?);
    // REQ-records-009, REQ-cli-055 (#246): doctor's own session check, run on the session
    // records this plan writes, names what `chough doctor` reports right after the apply,
    // so the stale gates' decisions are asked in the same round as the consent.
    follow_ups.extend(crate::doctor::session_follow_ups(
        root,
        &planned_sessions(&carried)?,
    )?);
    // REQ-records-009: a generated file that is a regular file with other bytes is
    // replaced by the rebuilt one, as `chough library index` replaces it, and the plan
    // says so; anything else there is refused before any write.
    let mut replaces = vec![];
    for written in &files {
        if crate::library::is_generated(root, &written.path)?
            && let AtTarget::Bytes(bytes) = at_target(root, &written.path)?
            && bytes != written.bytes
            && !one_file_by_case_with_a_source(root, &carried, &written.path)?
        {
            replaces.push(written.path.clone());
        }
    }
    // REQ-records-003: a 0.5.x record whose ID a 0.6.0 record already holds as an alias
    // is refused, naming that record, so no second record takes the alias.
    let planned: BTreeSet<&str> = files.iter().map(|file| file.path.as_str()).collect();
    let holders = crate::doctor::alias_holders(root);
    let mut held = vec![];
    for (id, home) in &carried.aliases {
        if home == "removed" {
            continue;
        }
        // A record this plan writes holds it only when an interrupted apply wrote it,
        // which Git has never held.
        for holder in holders.get(id).into_iter().flatten() {
            if !planned.contains(holder.as_str()) || held_by_git(root, holder) {
                held.push(format!("- `{id}` is already an alias of `{holder}`"));
            }
        }
    }
    ensure!(
        held.is_empty(),
        "refused:alias-held: a 0.5.x record holds an ID a 0.6.0 record already holds as an alias, so nothing was written, and the apply would refuse too; a second record never takes an alias:\n{}\nCarry what you want from the 0.5.x record into that record by hand, remove the 0.5.x file, commit, and run `chough migrate` again.",
        held.join("\n")
    );
    Ok(Plan {
        sources,
        legacy,
        carried,
        kept,
        files,
        receipt: receipt_path()?,
        replaces,
        redactions,
        follow_ups,
    })
}

/// Each open session record the plan writes, `.chough/sessions/<key>/session.md` as
/// redacted, parsed, with the number of inbox entries it writes directly under that
/// session's `agent/inbox/`, counted as `chough doctor` counts unfolded entries there.
fn planned_sessions(
    carried: &Carried,
) -> Result<Vec<(String, crate::checkpoint::Checkpoint, usize)>> {
    let dir = format!("{}/", crate::sessions::DIR);
    let mut found = vec![];
    for written in &carried.written {
        let Some(key) = written
            .path
            .strip_prefix(&dir)
            .and_then(|rest| rest.strip_suffix("/session.md"))
            .filter(|key| !key.contains('/'))
        else {
            continue;
        };
        let crate::checkpoint::Session::V2(session) =
            crate::checkpoint::parse(&written.path, &written.bytes)?;
        let inbox = format!("{dir}{key}/agent/inbox/");
        let unfolded = carried
            .written
            .iter()
            .filter(|entry| {
                entry
                    .path
                    .strip_prefix(&inbox)
                    .and_then(|name| name.strip_suffix(".md"))
                    .is_some_and(|name| {
                        !name.contains('/')
                            && !name.starts_with('.')
                            && crate::records::valid_change_id(name)
                    })
            })
            .count();
        found.push((key.to_string(), *session, unfolded));
    }
    Ok(found)
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
    let suffix = match RECEIPT_ORDINAL.load(Ordering::Relaxed) {
        0 | 1 => String::new(),
        ordinal => format!("-{ordinal}"),
    };
    Ok(format!(
        ".chough/archive/migrations/{}-from-0-5{suffix}.md",
        crate::identity::utc_date(crate::records::now()?)
    ))
}

/// Which same-day receipt name the plan being built uses: 1 is `<date>-from-0-5.md`,
/// and 2 and later add `-<n>` (REQ-records-009, #264). Set by
/// [`plan_with_a_free_receipt`] before each build, and read by everything that names
/// the receipt, so the plan, its digest and the receipt agree.
static RECEIPT_ORDINAL: AtomicU32 = AtomicU32::new(1);

/// How many receipts one UTC day can hold.
const RECEIPT_ORDINALS: u32 = 99;

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
            .filter(|folder| !crate::evidence::is_os_metadata(folder))
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
                    .filter(|name| !crate::evidence::is_os_metadata(name))
                    .collect()
            })
            .unwrap_or_default();
        names.sort();
        for name in names {
            let key = name.trim_end_matches(".json").to_string();
            found.push((key, format!("{folder}/{name}"), archived));
        }
    }
    // #370: a session file removed as already migrated is not carried again.
    found.retain(|(_, path, _)| !already_migrated(path));
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
            title: String::new(),
            summary: vec![],
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
            // REQ-history-006 (F10a): the story first, then its closing line, then
            // `## Receipts`, which the evidence mapper's receipts fill in later.
            let mut text = record.render();
            if !text.ends_with('\n') {
                text.push('\n');
            }
            text.push_str(&format!("\nClosed {date}: {outcome}.\n"));
            text.push_str(&crate::markdown::receipts_section(vec![])?);
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
                    .filter(|name| !crate::evidence::is_os_metadata(name))
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
        // F10e: the bare 0.5.x ID line of each observation carried as a receipt becomes
        // the line a 0.6.0 story writes for it; a line a person wrote is kept.
        let mut record = crate::markdown::parse(&path, &written.bytes)?;
        let listed = crate::session_md::items(&record, crate::session_md::VERIFIED);
        for id in &home.observations {
            let (Some(map), true) = (homes.receipts.get(id), listed.contains(id)) else {
                continue;
            };
            if let Some(line) = crate::sessions::receipt_line(map, None) {
                record.replace_item(crate::session_md::VERIFIED, id, &line)?;
            }
        }
        // REQ-history-006 (F10a): the receipts go in the story's last section.
        let text = crate::markdown::with_receipts_section(&record.render(), receipts)?;
        let bound = crate::markdown::Kind::Story.bound();
        ensure!(
            text.len() as u64 <= bound,
            "refused: {path} with its receipts would be {} bytes, over its bound of {bound} bytes; nothing was written",
            text.len()
        );
        written.bytes = text;
    }
    Ok(())
}

/// Each open session's What was verified (REQ-records-009, F10e): the sessions mapper
/// writes an observation's line as its bare 0.5.x ID, since evidence runs after it, so
/// the line of each observation carried as an evidence entry becomes the line a 0.6.0
/// save writes, its claim linking the entry's file; the old ID stays the entry's alias.
/// A line a person wrote, or one for an observation carried as no entry, is kept.
fn session_verified_links(
    carried: &mut Carried,
    sessions: &BTreeMap<String, SessionHome>,
    homes: &ObservationHomes,
) -> Result<()> {
    use crate::session_md::VERIFIED;
    // Each carried entry's claim, by its path.
    let mut claims = BTreeMap::new();
    for written in &carried.written {
        if homes.entries.values().any(|entry| *entry == written.path) {
            let record = crate::markdown::parse(&written.path, &written.bytes)?;
            claims.insert(
                written.path.clone(),
                record.scalar("claim").unwrap_or_default().to_string(),
            );
        }
    }
    for home in sessions.values().filter(|home| !home.archived) {
        let path = format!(".chough/sessions/{}/session.md", home.id);
        let Some(written) = carried
            .written
            .iter_mut()
            .find(|written| written.path == path)
        else {
            continue;
        };
        let mut record = crate::markdown::parse(&path, &written.bytes)?;
        let listed = crate::session_md::items(&record, VERIFIED);
        let mut changed = false;
        for id in &home.observations {
            let (Some(entry), true) = (homes.entries.get(id), listed.contains(id)) else {
                continue;
            };
            // `.chough/sessions/<owner>/evidence/<name>`, linked from this session.
            let Some((owner, name)) = entry
                .strip_prefix(".chough/sessions/")
                .and_then(|rest| rest.split_once("/evidence/"))
            else {
                continue;
            };
            let link = if owner == home.id {
                format!("evidence/{name}")
            } else {
                format!("../{owner}/evidence/{name}")
            };
            let claim = claims.get(entry).map_or(id.as_str(), String::as_str);
            record.replace_item(
                VERIFIED,
                id,
                &crate::session_md::verified_link(claim, &link),
            )?;
            changed = true;
        }
        if changed {
            written.bytes = record.render();
        }
    }
    Ok(())
}

/// t38n's round-4 F9 (REQ-records-009): a migrated session's and story's front matter
/// `observations` name each carried entry or receipt, as the body links it, never the
/// 0.5.x ID, which stays the entry's or receipt's alias; so `chough next` and `chough
/// status` list the names a reader finds the files by.
fn observation_names(
    carried: &mut Carried,
    sessions: &BTreeMap<String, SessionHome>,
    homes: &ObservationHomes,
) -> Result<()> {
    for home in sessions.values() {
        let path = if home.archived {
            format!(".chough/archive/sessions/{}.md", home.id)
        } else {
            format!(".chough/sessions/{}/session.md", home.id)
        };
        let Some(written) = carried
            .written
            .iter_mut()
            .find(|written| written.path == path)
        else {
            continue;
        };
        let mut record = crate::markdown::parse(&path, &written.bytes)?;
        let listed = record.list("observations");
        let named = homes.names(&listed);
        if named != listed {
            record.set("observations", crate::markdown::Value::List(named))?;
            written.bytes = record.render();
        }
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

    /// REQ-records-009 (#370): the receipts released builds wrote read by the rule:
    /// Chough's own, from 0.6.0 (#239), and one the v0.6.1 build wrote on a synthetic
    /// project. Each lists the files read with their SHA-256 under `## Before`.
    #[test]
    fn receipts_from_released_builds_list_the_files_read() {
        for (build, text) in [
            (
                "0.6.0",
                include_str!("../.chough/archive/migrations/2026-11-12-from-0-5.md"),
            ),
            (
                "0.6.1",
                include_str!("../tests/fixtures/migrate/receipts/from-0-6-1.md"),
            ),
        ] {
            let read = files_read(text).unwrap_or_else(|| panic!("{build}: no files read"));
            assert!(!read.is_empty(), "{build}: lists no file read");
            for (path, sha) in &read {
                assert!(
                    path.starts_with(".chough/") || path.starts_with("knowledge/"),
                    "{build}: {path}"
                );
                assert!(
                    sha.len() == 64 && sha.bytes().all(|byte| byte.is_ascii_hexdigit()),
                    "{build}: {path} {sha}"
                );
            }
            assert!(
                !carried_into(text).is_empty(),
                "{build}: no removed file carried"
            );
        }
    }

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
            stories: BTreeMap::new(),
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
        assert_eq!(
            record.story_receipts(&story).unwrap(),
            vec![carried_map, tombstone]
        );
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

    /// REQ-records-009 (#263): two names that differ only in case are one file only
    /// where the file system says so. A folder that lists both holds two files, and a
    /// target whose other spelling no longer opens stands alone.
    #[test]
    fn two_names_differing_only_in_case_are_one_file_only_when_the_folder_lists_one() {
        let root = tempfile::tempdir().unwrap();
        let old = root.path().join("INDEX.md");
        let new = root.path().join("index.md");
        std::fs::write(&old, "old").unwrap();
        if new.exists() {
            // A case-insensitive file system: one file under either spelling.
            assert!(one_file_by_case(&new, &old));
            return;
        }
        std::fs::write(&new, "other bytes").unwrap();
        assert!(!one_file_by_case(&new, &old), "the folder lists both names");
        std::fs::remove_file(&old).unwrap();
        assert!(!one_file_by_case(&new, &old), "the other spelling is gone");
    }
}
