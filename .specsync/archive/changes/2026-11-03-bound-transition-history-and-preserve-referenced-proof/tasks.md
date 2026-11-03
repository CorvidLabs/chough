---
change: bound-transition-history-and-preserve-referenced-proof
artifact: tasks
---

# Tasks

- [x] Retain failure-first regressions and local bug links for the affected history and retention defects.
- [x] Add event storage, explicit adoption and checkpoint cursor with compatible legacy reads and merge refusal.
- [x] Update stage, gate, recent-context, next and timing consumers with bounded lineage-aware reads.
- [x] Add read-only cycle retention preview, authorized apply, immutable receipts and protection revalidation.
- [x] Verify existing evidence retention protection and interrupted/missing/unsafe-input reporting.
- [x] Exercise real Git branch integration, boundary, collision and interrupted-write fixtures.
- [x] Update curated case definitions, docs, package/bundle compatibility and closeout lessons.
- [x] Run focused Rust regression suites and prepare all five bound cases and the required Fledge verification lane.

Human scope approval and implementation acceptance are separate gates, recorded by their native owner after the human decides. They are not task checkboxes. No review or ship occurs during definition preparation.

## Required verification after native materialization

SpecSync requires implementation task completion before it materializes the new requirement IDs. The five executable cases need those IDs for validation. The checklist above therefore records completed implementation, regression testing and verification preparation, not results of commands that have not run.

After native materialization, execute TC-cli-history-adoption, TC-cli-history-branches, TC-cli-history-bounds, TC-cli-history-retention and TC-cli-retention-revalidation; run the required `fledge lanes run verify`; and verify the final implementation on Rust 1.88. These remain mandatory before requesting human implementation acceptance. Record their actual outcomes and limitations in Chough observations and the checkpoint. The approved scope and semantic delta remain unchanged.

- [x] Fix the independently reproduced frozen artifact directory alias protection gap and retain its failure-first preservation regression.

- [x] Measure retention age from the selected closing reset, preserve that timestamp in receipts and prevent older receipts from overriding a recent closure.
- [x] Reproduce post-prune references through real Git integration; report missing referenced history explicitly and refuse cleanup until exact restoration.

- [x] Share normalized reference boundaries and whole-cycle classification across bare and every quoted directory form, preserving exact-file restoration semantics in the real Git regression.
