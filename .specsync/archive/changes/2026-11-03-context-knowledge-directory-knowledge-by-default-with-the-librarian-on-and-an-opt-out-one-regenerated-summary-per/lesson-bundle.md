# Lesson bundle: context-knowledge-directory-knowledge-by-default-with-the-librarian-on-and-an-opt-out-one-regenerated-summary-per

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Context knowledge directory: knowledge/ by default with the librarian on and an opt-out, one regenerated summary per archived change with a lessons index, and raw proof outside the newest 3 archived changes leaves the working tree only through a previewed, authorized, validity-checked step
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/library.rs, src/evidence.rs, src/prefs.rs, src/main.rs, tests/, docs/, skills/, .agents/skills/, specs/cli/, hi/knowledge.md, INTENT.md, CHANGELOG.md, README.md, AGENTS.md, .chough/cases/
- **Acceptance**: With no preference set, the librarian is on and its directory is knowledge/; pref set librarian off opts out with a recorded source, and checkpoints and cold resume still work
- **Acceptance**: library index writes knowledge/INDEX.md and one knowledge/archives/<change-id>.md per archived change with its title, criteria, requirements, commits, listed records and artifacts with digests and current location, observation outcomes and environments, linked bugs and lesson bundle; regenerating from the same sources gives identical bytes and no gate reads the directory
- **Acceptance**: The index lists existing library/ snapshots and briefs as they are, byte-identical and unmigrated, and a lessons index that points to each module lesson source and each lesson bundle without copying lesson text; chough next names the knowledge index
- **Acceptance**: library archive previews without writing which observation sets of archives outside the newest 3 would leave the working tree, the bytes and why each other set stays; checkpoint, bug, open-change, hot-archive, validation, newest-run, newest-failure, non-routine and directly named protections all still hold
- **Acceptance**: library archive --apply needs retention consent or preference, is refused while any native change is open, removes only files committed byte-identically at HEAD, writes each removal to the retention ledger first with the knowledge entry and digests, shows removed observations as pruned, and restores everything and refuses when an affected archive's terminal evidence validity changes
- **Acceptance**: fmt, lint, the full tests and fledge lanes run verify pass

## Evidence

- Verification commit: `eae6f44de070763357b3811ca6e771a325fc3ee7`
- Base commit: `0363d4af368bb07cc07f16c2f5fff7645c1c05bc`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Leif (0xLeif) asked on 2026-11-03: "Yes but I want the librarian and archiving and compounding of lessons and archives and all.. artifacts.. we need a context knowledge directory". He then answered the drafted questions in chat, between 8:09 and 8:27 AM MT:

- Q1, "Keep only a summary; raw files stay recoverable from git".
- Q2, "Keep the newest 3 archived changes hot".
- Q3, "Librarian on everywhere with an opt-out".
- Q4, "Use knowledge/ at the project root".

He also said yes to the drafted intent. HI KNOW-1 to KNOW-12 in hi/knowledge.md capture these answers.

At main 0363d4a the librarian is off by default and its folder defaults to `library/`. It only snapshots and indexes, and no step runs after an archive. `.chough/` tracks 1,845 files (about 25 MB), and `chough evidence prune` finds nothing eligible, because every record is younger than 14 days.

Human authority: Leif (0xLeif) in chat, 2026-11-03 8:09–8:27 AM MT. The parent agent relayed the answers above as his intent approval; his next decision is final acceptance.

Covered scope:

- the librarian defaults;
- the `knowledge/` index, archive entries and lessons index;
- a previewed, authorized `chough library archive` step;
- `chough next` naming the index;
- tests, REQ-cli-015, a new REQ-cli-033, and docs.

Limits:

- This change does not run the apply step on this repository's own `.chough` history; that live cleanup needs Leif's separate yes.
- No outside storage (#14 item 3), no release and no deployment.
- Duplicate or contradicting lesson detection, scoped lessons in `next`, and retired-lesson history are deferred.

## From the change's design.md

# Design

## Preferences

- `librarian` defaults to `on`, and `library-dir` defaults to `knowledge`.
- `chough pref set librarian off --source ...` opts out. Every `library` command then refuses with the opt-in hint.
- An explicit `library-dir` keeps working.

## Archives

Each `.specsync/archive/changes/<dir>/` is read from its own files:

- `finalization.json`: change_id, timestamp, implementation_commit;
- `verification.json`: commit, requirement_ids, and acceptance_manifest.entries, which give path and payload_digest;
- `change.md`: the H1 title and the "## Acceptance Criteria" bullets.

Archives are sorted newest first by (timestamp, directory). The first 3 are **hot**; the rest are **cold**.

## Entries

`<dir>/archives/<change-id>.md` is deterministic. It holds:

- the title and the hot or cold state;
- the archive link, finalization timestamp, base, implementation and verification commits, and requirements;
- the acceptance criteria;
- one row per listed `.chough/evidence` or `.chough/artifacts` path, with its digest and location:
  - `working tree` when the file is present with the listed digest;
  - `working tree (changed since archive)` when it is present but differs;
  - `Git <commit>` when it is absent, using the retention ledger's head when the ledger has it, otherwise the verification commit;
- observations, with ID, outcome, environment and cases, read from the record or from the ledger;
- bugs that link those observations;
- the lesson bundle link.

The generated header says that the entry is regenerated by `chough library index` and is not a ledger.

## Index

INDEX.md adds:

- "Archived changes (knowledge entries)", with hot and cold marks;
- "Lessons index", with each module's `<module>.3md` or `context.md` `## Lessons` line, and each archive's `lesson-bundle.md`;
- "Earlier library folder" when `library/` exists and is not the configured directory, listing its snapshots and briefs as they are.

The index stays within 65,536 bytes.

## Archive step

`chough library archive` reuses the retention classification.

**Candidates.** A set is a candidate when its record path, or one of its artifacts, is listed by a cold archive and by no hot archive. It is eligible when its protection reasons, after dropping only `recent` and the latest-archive manifest reason, are empty. The latest archive is itself hot, so hot protection still covers it.

**Protections.** All other reasons stay: checkpoint, bugs, active native manifests, validation, newest, last failure, non-routine and named files. A set listed by a hot archive gains the reason `hot archive <id>`.

**HEAD check.** Each file must be tracked at HEAD with the same blob ID; `git ls-tree` and `git hash-object` each run once. Files that fail the check are listed as `not committed identically at HEAD`.

**Preview.** The preview writes nothing. It returns candidates (files, bytes and archives), protected sets with reasons, the hot and cold archives, and totals.

**`--apply`:**

1. Requires retention authority, as `evidence prune --apply` does.
2. Is refused while `.specsync/changes/` holds any change.
3. Probes `specsync change status <id> --json` `summary.terminal_evidence.validity` for each affected archive.
4. Writes each set's ledger line first, with reason `archived-summary` and a `knowledge` field, then removes the files, keeping their bytes in memory.
5. Probes validity again. On any difference it restores every removed file, appends `restored` ledger lines and fails.
6. Otherwise it regenerates the entries and the index.

Ledger readers skip `restored` lines when reading entries. A restored record is present, so it is never labeled pruned.

## Next

The view gains `knowledge`: the index path when the librarian is on and the index exists, else a hint, or `off`. The text view prints one line, "Knowledge index: ...".

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-015, KNOW-1, KNOW-5, KNOW-6 | `tests/optional.rs::the_librarian_is_on_by_default_in_knowledge_and_can_be_turned_off` |
| REQ-cli-015, KNOW-2, KNOW-3, KNOW-4, KNOW-12 | `tests/evidence.rs::knowledge_entries_summarize_each_archive_deterministically` |
| REQ-cli-033, KNOW-7, KNOW-8, KNOW-11 | `tests/evidence.rs::archive_summary_previews_cold_archive_proof_and_keeps_protections` |
| REQ-cli-033, KNOW-7, KNOW-9, KNOW-10, KNOW-11 | `tests/evidence.rs::archive_summary_apply_is_authorized_ledgered_and_restores_on_validity_change` |
| REQ-cli-033, KNOW-7, KNOW-9 | `tests/evidence.rs::archive_summary_apply_fails_closed_when_a_file_changes_during_the_native_status_check` |
| REQ-cli-033, KNOW-7 | `tests/evidence.rs::archive_summary_lists_a_crlf_file_differing_from_head_as_uncommitted_under_autocrlf` |
| REQ-cli-033, KNOW-8 | `tests/evidence.rs::archive_summary_apply_fails_closed_when_a_protection_appears_during_the_native_status_check` |
| REQ-cli-033, KNOW-7, KNOW-10 | `tests/evidence.rs::archive_summary_restore_keeps_a_file_recreated_during_the_apply` |
| REQ-cli-033, KNOW-9 | `tests/evidence.rs::a_stale_removal_hold_never_breaks_status_and_blocks_the_next_apply` |

All four fail at 0363d4a. The last two were added after review 5756934010 and fail at d5fabf5 (the apply removes bytes written during the validity check; the CRLF file is listed as eligible). The three after them were added after review 5757166239 and fail at 5cd0ead (the apply removes a set the checkpoint started referencing; the rollback overwrites a recreated file; the stale-hold test fails only because `removal_holds` is missing, since a hold in `.chough/removing` never broke a reader there). After review 5757616772 the stale-hold test also runs `next`, `context`, `timing`, `bug list` and the `evidence prune` preview with the hold present; those reader assertions cannot fail first at the test level for the same reason. Unit tests `remove_verified_puts_changed_bytes_back_and_leaves_no_hold` and `put_back_never_replaces_a_file_written_meanwhile` cover the hold itself, and `a_hold_never_sits_in_the_removed_file_directory` pins where it goes: with `hold_path` changed back to 5cd0ead's `.<name>.chough-removing` sibling design it fails (`left: Some(".chough/evidence")`), and it passes with the `.chough/removing` design.

## Cases

- TC-cli-librarian-default-on-knowledge
- TC-cli-knowledge-archive-entries
- TC-cli-archive-summary-preview
- TC-cli-archive-summary-apply
- TC-cli-archive-summary-changed-during-check
- TC-cli-archive-summary-unfiltered-head-compare
- TC-cli-archive-summary-protection-during-check
- TC-cli-archive-summary-protection-before-each-removal (review thread 4459066056: a checkpoint reference to cold-a added on the first `git hash-object --no-filters --stdin`, after the validity check and its re-plan; at 30bb9f0 the apply exited 0 and removed cold-a.json and cold-a.log, and it now refuses with every file kept)
- TC-cli-archive-summary-no-clobber-restore
- TC-cli-archive-summary-stale-holds

## Focused and full checks

`cargo test`, the cases through `chough cases run`, and `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2.

## Dry run

`chough library archive` (preview only) on this repository. The result is reported in the PR and not applied.

## Where these lessons go

- `specs/cli/context.md`
