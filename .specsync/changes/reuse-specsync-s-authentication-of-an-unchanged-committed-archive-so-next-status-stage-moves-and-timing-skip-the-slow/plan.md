---
change: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
artifact: plan
---

# Plan

## Approach

Replay SpecSync's own authenticated-history verdict for byte-identical archive inputs, stored outside the worktree, and add a batched advance that reuses one native read.

## Out of scope

Porting SpecSync's integrity check; caching active changes; changing retention reads; SpecSync itself.

## Steps

1. Spec delta and approval.
2. Failing tests with the counting stub.
3. `src/native.rs`, wiring in main.rs, stage.rs and timing.rs, `stage advance --to`.
4. Docs, CHANGELOG, cli.spec.md files list.
5. Verify, then time on Raven before and after.

## Risks

A SpecSync verdict that depends on inputs outside the digest. Mitigated by recording only self-contained archives and keying on the binary, policy, config and baseline; any doubt reads live.

## Constraints consulted

Native SpecSync state decides gates; no recording of human approval or review; REQ-cli-024 and REQ-cli-028 amended explicitly; no new dependency.
