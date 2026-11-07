---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: context
---

# Context

Issue #65 reports two failure-handling gaps in `chough evidence prune --apply`. On origin/main `455cb643cd91a9c804afdf5566e3889abf1bde49`, `retention()` in `src/evidence.rs` checks `history::protection_fingerprint` and the artifact digest, then calls `remove_if_present`. A protection can appear between that check and the unlink. If a later artifact or observation check or removal fails, files already unlinked stay missing and only the initial ledger line remains.

The existing human intent is LIFE-8: routine reruns do not keep raw logs forever, but proof behind active work and fixed baselines stays, and removed proof is shown as removed rather than passed. LIFE-2 and LIFE-7 also apply. No new HI criterion is proposed.

PR #55 already closed the analogous boundary in `library archive --apply`. Its `remove_verified`, stale-hold refusal, create-new rollback and `restored` ledger event are prior art. Evidence prune differs in one consequential way: its eligible observations need not be committed at HEAD, so rollback must not claim Git can always recover the removed bytes. If a recreated path prevents restoration, this change preserves the original removed bytes under `.chough/removing/` and names that path.

Scope is one bug-fix change and one PR: recheck immediately before every removal, use byte-checked holds, restore earlier removals after a later failure, and keep every existing eligibility, protection, authorization, bound, ledger and successful JSON behavior. The existing `restored` event shape is reused. No lifecycle gate, retention rule, archive implementation, timeout or output schema is weakened or redesigned.

The installed Chough 0.4.0 lacks the `change` command group. Lifecycle commands for this draft use this worktree's Chough 0.5.0 through `cargo run --locked -- ...`; the failed installed-command attempt created no native record. Role review found no new product or accessibility decision. QA added an earlier-set rollback scenario so a failure after one complete set cannot leave that set pruned. Engineering and architecture confirmed the create-only recovery hold is necessary for uncommitted evidence and that no archive refactor or native read belongs here.

Scope approval and implementation review remain human gates through the orc. No implementation has been approved or performed.
