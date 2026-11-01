# Lesson bundle — align-kimi-integration-with-new-cycle-done-guidance

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Align Kimi integration with new-cycle done guidance
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, tests/stages.rs, tests/next.rs, README.md, docs/ROADMAP.md, docs/exercises/cold-start.md, docs/walkthrough.md, specs/cli/testing.md, specs/cli/tasks.md
- **Acceptance**: Archived done checkpoints with authenticated native evidence follow REQ-cli-021 by recommending a new cycle with a reason-bearing reset. They never repeat stale native review or ship actions. Pause, human gates, blocking bugs, closed-gate precedence and the unchanged checkpoint action retain their existing behavior.
- **Acceptance**: TC-cli-0105 is updated to the newer REQ-cli-021 contract. A failure at the integrated pre-fix tip is linked to BUG-0025 before implementation, followed by a later passing observation. TC-cli-0121 and TC-cli-0122 also pass. Original observations remain retained and historical.
- **Acceptance**: The Kimi records and affected docs distinguish the superseded native-routing implementation from integrated new-cycle behavior. BUG-0024 remains separate from main open BUG-0020 and the new integration defect is BUG-0025. No frozen archive is edited and no fresh post-fix Kimi run is claimed.
- **Acceptance**: Focused Rust tests and fledge lanes run verify and pre-push pass with HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1. The PR remains draft through implementation review. Merge follows only after archive and passing checks under 0xLeif explicit instruction. No release or deployment is claimed.

## Evidence

- Verification commit: `08f0d08029084a3b4a2568d90468cbec0608c1cc`
- Base commit: `b2e2f19d904aec2a5acd73569c8439f5583c01f9`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR #24 archived a native-next-action fix before main advanced. Main then merged PR #23 with REQ-cli-021, which requires a new-cycle action for an archived done checkpoint, and PR #25 with a distinct open BUG-0020.

Merge commit `b2e2f19d904aec2a5acd73569c8439f5583c01f9` contains the mechanical conflict resolution. It preserves both histories and moves the Kimi archived-resume defect to BUG-0024. The integrated code still has the earlier native-routing override, so it contradicts the newer approved REQ-cli-021. BUG-0025 records that integration defect.

0xLeif explicitly selected main's new-cycle behavior. This change implements that decision without a canonical contract delta. The earlier observations and frozen archives remain unchanged. The one coached Kimi run is still the only run; no fresh post-fix run is claimed.

The `.chough/` directory is deliberately excluded from affected paths so record writes do not stale native verification. Slug-based bug identity is a separate change. No new integration, runtime, service or duplicate lifecycle state is in scope.

## From the change's design.md

# Design

## Approach

Remove the archived-done native-next-action override from `assess` in `src/main.rs`. Main's shared `stage::assess` then supplies the REQ-cli-021 new-cycle action. Retain the single native read, focused bounded reports, approved-delta guard and all existing precedence rules.

Revise the Kimi regression tests rather than dropping scenarios. Stale native review or ship text must not become the recommended action. Cover both `next` and `status`, missing or blank native guidance, pause and human-gate precedence, and reset behavior. Main's TC-cli-0121 and TC-cli-0122 remain independent contract coverage.

Update TC-cli-0105 to trace to REQ-cli-021 and the revised regression test. Its definition change makes earlier observations historical; do not edit or delete them. Record a failed observation at the pre-fix integrated tip and link it to BUG-0025 before implementation. A later pass can verify the bug.

Update the affected Kimi records and docs to explain that the original native-routing implementation was superseded during merge by main's new-cycle contract. Preserve the original cold-start verdict and the BUG-0020 and BUG-0024 distinction. Do not change canonical requirements or frozen archives.

## Plan and ownership

1. Update the case and regression expectations, run TC-cli-0105 before the fix, and link the failure to BUG-0025.
2. Remove the override and revise only the affected tests in `src/main.rs`, `tests/stages.rs` and, if needed, `tests/next.rs`.
3. Correct the approved docs and testing companions. One integrator owns code and records to avoid concurrent checkpoint writes.
4. Run focused tests, affected cases and the full local lanes with CI-matching tools. Commit source with `.chough/` records before native `change check --commit`.
5. Push the normal merge and implementation commits, observe hosted checks, and stop for implementation review.

This is a small integration change. The main uncertainty is interaction between the two archived implementations, not new architecture. Timing history is advisory only and is not a deadline.

## Out of scope

Slug-based bug IDs, changes to REQ-cli-021 or REQ-cli-022, a fresh Kimi cold-start run, new hosts, integrations, runtimes, services, releases and deployments.

## From the change's testing.md

# Testing

## Evidence sequence

- TC-cli-0105 proves the integrated BUG-0025 failure before the fix and the REQ-cli-021 behavior afterward. Retain both observations and link them to the bug.
- TC-cli-0121 proves archived authenticated done recommends a reason-bearing reset and new native change.
- TC-cli-0122 proves reset replaces the checkpoint next action and back preserves it.
- `cargo test --test stages` and `cargo test --test next` exercise the focused routing and report behavior.
- `fledge lanes run verify` covers formatting, Rust tests, Clippy, HI, SpecSync, audit, cases, exercise and build. `fledge lanes run pre-push` covers the required push checks.
- Use HI 0.8.0, Fledge 1.7.2 and SpecSync 6.1.1 on `local-macos-arm64`. Report hosted checks separately and do not turn analysis jobs into product-test claims.

A structural check is not a fresh Kimi cold-start run. Any unrun platform or check remains unclaimed.

## Where these lessons go

- `specs/cli/context.md`
