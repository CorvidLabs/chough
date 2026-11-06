# Lesson bundle: assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Assert positive track selection and every delivery-observation kind, and that a dry run never stands in for one (152, MA-01, MA-02)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/cases.rs, tests/reduce.rs, CHANGELOG.md
- **Acceptance**: REQ-cli-014 gains a criterion: publication, installation, deployment and live-health observations are each accepted and reported with their own kind and kept as key-item proof (protected by kind, moved into the vault when cold), while a dry run's kind is never a protection and a superseded dry run is trimmed with no vault copy. Tests assert positive copy-localization selection (REQ-cli-010), every delivery kind recorded and reported as itself with a dry run as a dry run (REQ-cli-014, REQ-cli-017), and the retention distinction for each kind. Each new test fails under a recorded targeted mutation that the existing tests let pass; no runtime change. fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `05c19c33da32d7c58d42d9c561bc101433c6b2ad`
- Base commit: `64aa77e01d2763395d8af7566e62743ede37cc32`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #152, from the audit's Chough assertion review (main `700dad96`), items MA-01 and MA-02. REPORT.md's 95% list ("Cases/evidence") puts them first.

- **MA-01 (REQ-cli-010).** `selection_is_focused_on_the_affected_scope_and_tracks` checks that a copy-localization case is omitted by default, then requests only `--track design-qa`. A positive copy-localization selection is never asserted.
- **MA-02 (REQ-cli-014, REQ-cli-017).** `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` writes only a dry run and a legacy product test. It never records the four delivery kinds and never shows a dry run failing to stand in for one.

What the runtime does today, read from `src/evidence.rs`:
- Every kind other than product-test, structural and dry-run is non-routine (`non_routine`).
- `evidence prune` protects a non-routine record with its kind as the reason.
- The trim at done treats a non-routine set as a key item that moves into the vault when its archive is cold (KNOW-19). A dry run is routine: superseded, it is trimmed with no vault copy.

Chough has no other delivery aggregate.

## Assignment

From the orc, deciding overnight for Leif (2026-11-06): tests and contract only, with no runtime change. The issue is filed, the worktree comes from origin/main, failing tests come first, there is a mutation check, and the approval is queued for Leif.

## From the change's testing.md

# Testing

The new tests assert behavior that holds today. Each is shown real by a targeted mutation that makes it fail while the existing tests named in the issue still pass:

| Mutation (src) | Fails | Existing test that lets it pass |
|---|---|---|
| `select` never selects copy-localization, even when requested | `requesting_the_copy_localization_track_selects_its_linked_cases` | `selection_is_focused_on_the_affected_scope_and_tracks` |
| `--kind live-health-observation` no longer accepted (`#[value(skip)]`) | `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run` | `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` |
| Deployment observations counted as routine | `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not` | `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` |
| Dry runs counted as non-routine key items | `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not` | `schema_2_evidence_accepts_delivery_kinds_and_keeps_dry_runs_distinct` |

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-010 | tests/cases.rs `requesting_the_copy_localization_track_selects_its_linked_cases`, `selection_is_focused_on_the_affected_scope_and_tracks` |
| REQ-cli-014 | tests/cases.rs `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run`; tests/reduce.rs `delivery_observations_are_kept_as_proof_while_a_superseded_dry_run_is_not` |
| REQ-cli-017 | tests/cases.rs `every_delivery_kind_is_recorded_as_itself_and_a_dry_run_as_a_dry_run` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
