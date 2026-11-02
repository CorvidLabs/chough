---
change: adopt-merged-main-human-gate-walkthrough-and-example-in-pr-46
artifact: plan
---

# Plan

## Approach
Adopt the final main 9d02 walkthrough and feature-bug loop exactly. Preserve the awaiting-human guard and every earlier main behavior while the first integration reconciles the report branch.

## Out of scope
New policy, behavior, contract semantics, release, live cleanup, archive editing or merge.

## Steps
Merge the final main target; retain these two files byte-for-byte; execute the mandatory lane and Rust 1.88 tests with the report cases; commit and push the existing PR branch. Root performs native review and ship.

## Risks
A stale example can bypass the newly enforced human gate. Exact parent comparison and the required disposable exercise establish integration coverage. This is a small bounded adoption with no claimed deadline.

## Constraints consulted
AGENTS.md; the Chough workflow; LIFE-1, LIFE-2, LIFE-6 and LIFE-7; main's approved awaiting-human guard and existing request-local error-chain lessons.
