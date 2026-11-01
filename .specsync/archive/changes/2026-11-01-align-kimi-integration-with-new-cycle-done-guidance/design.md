---
change: align-kimi-integration-with-new-cycle-done-guidance
artifact: design
---

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
