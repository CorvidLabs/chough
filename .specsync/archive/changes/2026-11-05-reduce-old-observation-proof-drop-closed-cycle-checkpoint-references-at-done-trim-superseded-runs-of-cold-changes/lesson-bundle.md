# Lesson bundle: reduce-old-observation-proof-drop-closed-cycle-checkpoint-references-at-done-trim-superseded-runs-of-cold-changes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Reduce old observation proof: drop closed-cycle checkpoint references at done, trim superseded runs of cold changes, attribute unlisted proof to its change by cycle time, and read archive validity in parallel
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/evidence.rs, src/library.rs, src/vault.rs, src/stage.rs, src/checkpoint.rs, src/skill.rs, tests/reduce.rs, tests/librarian_trim.rs, tests/evidence.rs, tests/vault.rs, tests/optional.rs, tests/next.rs, tests/stages.rs, hi/knowledge.md, INTENT.md, skills/chough-workflow/references/librarian.md, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: At done with the librarian on, checkpoint observation references from before the current cycle are dropped and reported. The post-archive summary trims superseded routine runs of a case and environment listed by a cold change, keeping the newest run and the newest failure, and trims proof no archive lists that was observed in a cold change cycle, ledgering it with that change and summarizing it in its knowledge entry; proof from a hot change cycle or after the newest archive stays. Archive validity is read in parallel. New tests fail first, the trim on this repository is measured, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `d0da297eec980c7b10df314b61d753603615a77f`
- Base commit: `5326ec48b4ae856543e4d8cabd7c06b29daef0e5`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Leif asked on 2026-11-04 to reduce the artifacts and evidence in `.chough/` and keep the
knowledge in the vault. He chose:
- "Old checkpoint refs" (KNOW-16);
- "Old newest-per-case" (KNOW-17);
- "Loose artifact folders" (KNOW-18);
- "Key items move" (KNOW-19);
- "By cycle time" for proof that no archive lists (KNOW-20).

When asked to confirm KNOW-16 to KNOW-19 explicitly, he answered "Yes, build it", and
then "go ahead and build the reduction PR".

This change covers observation proof: KNOW-16, KNOW-17 and KNOW-20, plus parallel
validity reads so a wide trim stays fast enough. Loose artifacts and attachments
(KNOW-18, KNOW-19) need a new ledger entry kind and file moves, so they follow in the
next change. The safeguards are unchanged: ledger first, the KNOW-10 restore check, no
removal while a native change is open, and every byte recoverable from Git.

## From the change's design.md

# Design

- **`evidence.rs`:**
  - `Classification` gains `newest_run` (the newest routine run per case and
    environment).
  - `ListedArchive` gains `finalized`.
  - `summary_protections`:
    - attributes unlisted sets with `owner_by_cycle` (first archive whose
      `finalized >= observed_at`);
    - removes `newest` when the set is not in `newest_run`;
    - keeps candidate `archives` (owner or listers) for the ledger and knowledge
      pointer, and `listed_by` for validity.
  - `validity` runs on scoped threads, with at most 8 at once.
  - `attributed_sets(root, archives)` lets library.rs list attributed records per
    archive.
- **`library.rs`:** an archive entry's Observations table adds its attributed records,
  marked "by cycle time", with ledger outcomes for pruned ones.
- **`stage.rs`:** on the move into done, with the librarian on,
  `drop_closed_cycle_references` loads each referenced record's `observed_at`, keeps
  those at or after `latest_reset_at`, saves, and reports
  `applied.checkpoint_references`. Then the existing trim and index refresh run.

## From the change's testing.md

# Testing

tests/reduce.rs uses five archives finalized 60 to 20 days ago (two cold), a stub
`specsync`, runs of one case listed by a cold archive, and three unlisted records:
- Superseded runs are trimmed, keeping the newest run and the newest failure.
- Unlisted proof is attributed by cycle time: trimmed and ledgered with its cold owner,
  and named in its note; the hot-cycle and current records stay.
- Done drops closed-cycle checkpoint references, and the trim can then remove them.
- Native validity reads overlap (the stub counts concurrent reads).

Each fails before the change. The existing librarian, evidence and vault tests keep
passing. The trim is run and measured on this repository, and `fledge lanes run verify`
runs on Linux.

## Where these lessons go

- `specs/cli/context.md`
