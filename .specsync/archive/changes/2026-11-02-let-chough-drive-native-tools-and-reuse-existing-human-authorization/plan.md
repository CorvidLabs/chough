---
change: let-chough-drive-native-tools-and-reuse-existing-human-authorization
artifact: plan
---

# Plan

## Approach

Use the existing current-agent skill to orchestrate native tools and consume applicable human authorization. Preserve the Rust helper and native ownership.

## Steps

1. Freeze this concrete definition and record native scope approval from the actual
   standing human instruction of 2026-11-02, with source and limits. This is agent
   execution of an existing human decision, not a new decision by an agent.
2. Update SKILL.md, lifecycle.md and README so the agent drives internal tools and
   reuses applicable explicit authorization. Preserve fresh checks and real human
   acceptance. Avoid edits to Grok's plan/testing/accept references.
3. Register exact prior hashes and add one meaningful install/update regression with
   frozen independently hashed predecessor fixtures and edited-copy preservation.
4. Update FRD, roadmap and changelog plus one executable case after native
   materialization; run focused package checks, required Fledge verification and MSRV.
5. Record actual observations and limits, commit source with Chough records, perform
   committed native verification, push one PR and coordinate review with Grok.

Ownership: root owns skill/lifecycle/README, definition/contracts and records.
workflow_bundle_upgrade owns only additive src/skill.rs entries, its separate test
and frozen predecessor fixtures. Grok's PR41 owns its separate ordering files;
report_read_context owns #32 in another checkout. Preserve all existing edits.

Relative size: small, with the main uncertainty being agent compliance in a new
session. Existing historical timing is elapsed wall-clock, not effort, and no
reliable effort deadline is inferred. Cold-start execution is not part of this scope.
No merge, release, deployment, live adoption or cleanup follows from this work.

## Risks

Overbroad interpretation of authority could invent a human decision. Require the
actual instruction, its covered scope and limits, and preserve separate acceptance
and delivery authority. Changed bundle bytes could be misclassified as user edits;
register exact predecessors and test real upgrades plus edited-copy preservation.

## Out of scope

New CLI writers, a model client, persistent authority cache, runtime or lifecycle
ledger; changing native gates; Grok41 ordering files; performance/storage changes;
merge, release, deployment, live cleanup and cold-start grading.

## Constraints consulted

AGENTS.md, INTENT.md, existing HI/FRD, specs/cli contract and companions, native
lessons about exact predecessor registration and verification ordering, the actual
human authorization of 2026-11-02 and the shared-file ownership comments on issues
11, 32 and 42. Existing human authority outranks a lesson or a generic skill default.
