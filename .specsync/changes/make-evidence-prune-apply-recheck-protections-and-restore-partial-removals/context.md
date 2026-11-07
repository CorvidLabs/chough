---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: context
---

# Context

Issue #65 reports two failure-handling gaps in `chough evidence prune --apply`. On origin/main `455cb643cd91a9c804afdf5566e3889abf1bde49`, `retention()` in `src/evidence.rs` checks `history::protection_fingerprint` and the artifact digest, then calls `remove_if_present`. A protection can appear between that check and the unlink. If a later artifact or observation check or removal fails, files already unlinked stay missing and only the initial ledger line remains.

The existing human intent is LIFE-8: routine reruns do not keep raw logs forever, but proof behind active work and fixed baselines stays, and removed proof is shown as removed rather than passed. LIFE-2 and LIFE-7 also apply. No new HI criterion is proposed.

PR #55 already closed the analogous boundary in `library archive --apply`. Its `remove_verified`, stale-hold refusal, create-new rollback and `restored` ledger event are prior art. Evidence prune differs in one consequential way: its eligible observations need not be committed at HEAD, so rollback must not claim Git can always recover the removed bytes. If a recreated path prevents restoration, this change preserves the original removed bytes under `.chough/removing/` and names that path.

Scope is one bug-fix change and one PR: recheck immediately before every removal, use byte-checked holds, restore earlier removals after a later failure, and keep every existing eligibility, protection, authorization, bound, ledger and successful JSON behavior. The existing `restored` event shape is reused. No lifecycle gate, retention rule, archive implementation, timeout or output schema is weakened or redesigned.

The installed Chough 0.4.0 lacks the `change` command group. Lifecycle commands for this draft use this worktree's Chough 0.5.0 through `cargo run --locked -- ...`; the failed installed-command attempt created no native record. Ownership correction after the first approval attempt: `specs/cli/cli.spec.md` canonically owns `src/evidence.rs`, so the change declares `cli` alongside `history` and includes a minimal Purpose delta plus the CLI contract history in affected paths. A disposable worktree probe with actor `kimi-probe` accepted both deltas and reported `approval_valid: true`; that probe record is not human approval and was removed with the probe worktree. The delivery worktree remains draft pending Leif's approval through the orc.

Role review found no new product or accessibility decision. QA added an earlier-set rollback scenario so a failure after one complete set cannot leave that set pruned. Engineering and architecture confirmed the create-only recovery hold is necessary for uncommitted evidence and that no archive refactor or native read belongs here.

Scope approval was recorded by the orc as 0xLeif at `80fbe1df895f78d8db8fa33daba54914bcad80aa`. Failure-first regressions were committed at `3ad9df48933b880836c812c06369d00b9369e918`, and their failed observations and local bugs at `55cb7fa5`. Implementation is now underway; implementation review remains a human gate through the orc.

## Scope amendment pending human reapproval

Orc t19f directed a send-back after the declared Linux verify on `7febd53a86d1a007565ff38638b41abd760c123f` failed `moved_requirements_keep_their_wording_under_their_new_ids`. The original approved scope omitted `tests/spec_split.rs`, which pins the exact REQ-history-004 body. The amendment adds only that path; the two approved semantic deltas, requirements, acceptance behavior and existing paths stay unchanged. No HI or REQ ID is added. Native send-back uses the agent actor `sol-ch65` with the honest dependency reason; the original approval remains historical and reapproval is required.

After reapproval only, replace the REQ-history-004 pin `128c08a8a763ba404099891f17da8c28e53da365375a44decfd8755c78208fd2` with `d807b65dc985f3d8971b83fcb6c1e1cc4615beb80ca30dee2ca595e847acf0d2`, computed as SHA-256 of the trimmed approved requirement body. Keep the assertion, ID mapping and every other pin unchanged. The implementation pin is intentionally unchanged at this scope gate. Failure receipt: `.chough/artifacts/issue-65-prune-race/finish/linux-verify-before-pin.log`, `OBS-product-test-f87bec1f3802297bc62bebe7`.
