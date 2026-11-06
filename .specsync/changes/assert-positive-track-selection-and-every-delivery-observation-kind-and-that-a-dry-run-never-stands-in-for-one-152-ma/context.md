---
change: assert-positive-track-selection-and-every-delivery-observation-kind-and-that-a-dry-run-never-stands-in-for-one-152-ma
artifact: context
---

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
