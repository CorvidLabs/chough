# Lesson bundle: refuse-over-bound-checkpoint-candidates-before-publishing-gate-history

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Refuse over-bound checkpoint candidates before publishing gate history
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/checkpoint.rs, src/history.rs, tests/cli.rs, tests/history.rs, specs/cli/context.md, specs/cli/testing.md
- **Acceptance**: Observation-count and serialized-byte overflow through the checkpoint writer with gate changes refuse before any managed record changes; exact bounds and successful gate history still work; all old evidence and unrelated limits remain byte-identical; focused and required gates pass with current proof.

## Evidence

- Verification commit: `f8b99ff243d42248a3e73fa381030fd12a1f34b0`
- Base commit: `9bd3d506b86cbb312a5a3e195697cd3138920039`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR51 review discussion_r4451099601 reproduces an over-capacity checkpoint writer publishing a gate event before final checkpoint validation. REQ-cli-032 already requires byte-identical refusal. Root owns this isolated followup. Worker owns only the assigned source/test files; root alone writes lifecycle and Chough records.

Acting agent: corvid-agent. Actual human sources: 0xLeif standing through-PR permission relayed in issue11 comment6581702299, explicit chat instruction "Merge approved, green PRs and resynchronize", and latest "Can you finish these PRs up and get them merged and everything?". Covered scope: repair this review defect, run real regressions and native closeout, publish PR51 and conditionally merge the independently reviewed green final head. No new human as-is grade, numeric selection, cold-start claim, release, deployment, pruning, force push or check weakening.

Preserve every existing archived or recorded proof byte. Earlier archived scopes and passing logs remain historical at their actual source. Bound choices131072/400 remain agent choices under the capacity goal; all unrelated bounds remain. Reject weakening the byte-identical refusal contract. Prior numeric diagnostic bug is verified; this is a distinct writer ordering defect.

## From the change's design.md

# Design

Validate the complete candidate checkpoint, including the serialized final history cursor, before publishing gate history. Reuse existing validation and serialization semantics; preserve legacy cursor-free behavior and immutable history format. Handle both observation count and byte capacity and both opening and clearing gates. A dry preparation phase may calculate cursor/event identities in memory if needed, then publish only a valid candidate. Do not replace the lifecycle owner, migrate old history, delete orphan events or broaden bounds. Independent review checks publication order and exact byte-bound edges.

## From the change's testing.md

# Testing

Add writer-level regression snapshots of all managed record paths and bytes. Reproduce failure against unchanged production source with a capacity-overflow candidate and a gate update, capture exact command/source/binary/log identity, then repair and run identical assertions. Cover observation and serialized checkpoint overflow, opened/cleared gates, and successful gate history. Run focused Fledge tests, formatter and lint, required nine-step verify lane and eighteen archive fixtures. Refresh actual macOS report observations at final source. Hosted Linux/macOS/Windows and native archive authentication remain required. No old observation is restamped and no actual live pruning/adoption occurs. Kimi proof is not run.

## Where these lessons go

- `specs/cli/context.md`
