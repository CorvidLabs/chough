# Lesson bundle: fix-264

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 264
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/migrate.rs, tests/migrate.rs, docs/cli.md, specs/records/requirements.md, specs/records/records.spec.md, .chough/cases/records.migrate-same-day-receipt.cases.toml, changelog.d/2026-11-18-fix-264.md
- **Acceptance**: A second migration on one day names its receipt <date>-from-0-5-2.md in the preview, as JSON and as text, with the name part of the plan's digest; its apply writes that receipt, leaves the first receipt byte for byte and removes the 0.5.x record. When another plan's receipt takes the previewed name after the preview, the apply refuses with every file unchanged. An interrupted apply still completes under its own receipt name.

## Evidence

- Verification commit: `7e7372e58175ac8f609464017385918298b55171`
- Base commit: `9dd28de28015ce6f873e237044cc9650e74a3712`
- Verified by: `specsync check --spec records`

## From the change's context.md

# Context

GitHub #264, found by a swarm stress test. A project was migrated to 0.6.0; a
0.5.x binary then wrote one more record into the migrated tree. The second
`chough migrate --apply` on the same day wrote the new session folder and then
failed: the receipt name `<date>-from-0-5.md` was already taken by the first
migration's receipt, the error named no recovery, and running it again failed
the same way. The only way out was to rename the first receipt by hand.

Leif's decision, in his comment on #264 on 2026-11-18 from his planning
interview: "suffix and check first. A second same-day migration plan names its
receipt with a deterministic suffix (`…-from-0-5-2.md`), part of the plan and
its digest, and `--apply` checks every target before it writes anything, so a
refusal never leaves a partial write."

REQ-records-009 fixes the receipt's path as one name per day, so this is a
contract change to that requirement. It builds on #312 (#263), which made the
pre-write check cover a case-colliding target.

## From the change's design.md

# Design

Nothing visual changes. The preview gains one line, `Receipt: <path>`, above the
plan digest, and its JSON gains the `receipt` key. The receipt of a second
migration on one day is `<date>-from-0-5-2.md`.

## From the change's testing.md

# Testing

Two tests in `tests/migrate.rs`, both failing on main:

- `a_second_same_day_migration_names_its_own_receipt`: after a migration, a
  stray 0.5.x session record is committed. The preview, as JSON and as text,
  names `<date>-from-0-5-2.md`; the apply writes it with the plan's digest,
  leaves the first receipt byte for byte, removes the stray record, and a third
  run has nothing to do.
- `a_receipt_name_taken_after_the_preview_refuses_before_any_write`: a receipt
  for another plan takes the previewed name after the preview. The apply
  refuses with the tree byte for byte unchanged, and a fresh preview names the
  next receipt and applies.

The existing interrupted-apply test still passes: a receipt this plan's own
apply wrote is used again under the same name and digest.

`.chough/cases/records.migrate-same-day-receipt.cases.toml` binds both tests.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | the two tests above and their cases |

## Where these lessons go

- `specs/records/context.md`
