---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: plan
---

# Plan

## Approach

Correct only both P1s on #65/#197. Existing HI and REQ-history-004 supply the intent. Adopt exact predecessor obligations under cli/history without mutating the old archive. Land the disposable red regressions in the real tree only after native scope approval, then apply the two focused changes described in design.md. Clarify public failure cases through the two declared section deltas, with requirement bodies unchanged.

## Out of scope

Retention eligibility/protections/authorization, successful output, limits, observation or ledger schemas, global locking, archive-stage redesign, the parked workflow proposal, sibling cleanup, new issues/PRs/agents, new HI/REQ IDs, owner configuration, relaxed assertions/timeouts, release/tag/deployment or worker merge. No archived record is rewritten.

## Steps

1. Obtain Leif's scope approval at the committed definition head through orc t19f; worker never writes approval/review.
2. Land the three deterministic red tests and the cfg(test) capture seam unchanged in purpose from the disposable proof. Commit and run their exact filters before the implementation fix; record compact real-tree failures against the new active change.
3. Bind captured bytes to eligible digests and contain unsafe restoration errors per file, continuing recovery and ledger accounting.
4. Add only the new per-change case definitions; materialize CLI Purpose/history Error Cases, update history testing and its verified lesson. Keep requirements and the split-test digest unchanged.
5. Run focused checks and the repository's pinned macOS/Linux verification, strict spec/audit and normal pre-push checks. Finish evidence/checkpoint/index before each final native check; no intervening writes before fresh human implementation review and later native closeout.
6. Stay on #197. Present both P1 fixes and passing evidence for fresh exact-head human implementation acceptance. Archive only after that acceptance; final full hosted CI and thread resolution remain merge gates. Root owns the held merge.

## Estimate

Small, with uncertainty concentrated in adversarial filesystem interleavings and preserving all rollback accounting. `chough timing --cycle current` reported the new stage before its first transition as unknown and excluded archived native history; that gives no scoped duration range. No elapsed history is represented as effort or a deadline.

## Risks

- Capturing replacement bytes can silently grant deletion authority. Compare the captured digest with the selected eligible digest, and prove the injection fired.
- Unsafe destination validation can escape the rollback loop. Catch it per file, use the separate safe recovery path, continue other held files and assert ledger entries for both sets.
- A symlink or private-copy error can overwrite a second writer. Keep path validation and create-only publication; assert symlink inode/target and target bytes remain untouched.
- An incomplete successor edge can stale old provenance. Declare and validate exact path/module/predecessor-entry digests before approval; preserve the predecessor archive byte-for-byte.
- An approval-specific native guard may lack a read-only endpoint in 6.2.0. Run show/status/strict audit/spec checks and explicit owner/delta/entry comparisons; do not substitute an approval probe or claim unrun guards passed.

## Constraints consulted

REQ-history-004, the CLI/history canonical owners, the existing private-staging rollback lesson and the two actual red receipts govern the fix. Both original retention and archive behavior remain scoped as before. `specsync` 6.2.0's documented `depend`/`supersede` interface and emitted acceptance-entry digests supply succession; no guessed lifecycle flags or modified historical acceptance are used.
