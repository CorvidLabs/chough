# Lesson bundle: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Make evidence prune --apply recheck protections and restore partial removals
- **Kind**: BugFix
- **Specs**: history, cli
- **Paths**: src/evidence.rs, tests/evidence.rs, specs/history/requirements.md, specs/history/testing.md, specs/history/history.spec.md, specs/cli/cli.spec.md, docs/FRD.md, docs/cli.md, CHANGELOG.md, .chough/cases/history.prune-race.cases.toml, tests/spec_split.rs
- **Acceptance**: A protection appearing between removals stops the apply and restores every earlier removal byte-for-byte without overwriting a recreated file; a later artifact or observation removal failure also restores earlier removals and appends the existing restored ledger event; successful pruning keeps its current eligibility, authorization, ledger and JSON behavior, and existing history remains readable

## Evidence

- Verification commit: `8a9a1a3c030a6a5b18cbe98958b8a413aa5d2a01`
- Base commit: `455cb643cd91a9c804afdf5566e3889abf1bde49`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #65 reports two failure-handling gaps in `chough evidence prune --apply`. On origin/main `455cb643cd91a9c804afdf5566e3889abf1bde49`, `retention()` in `src/evidence.rs` checks `history::protection_fingerprint` and the artifact digest, then calls `remove_if_present`. A protection can appear between that check and the unlink. If a later artifact or observation check or removal fails, files already unlinked stay missing and only the initial ledger line remains.

The existing human intent is LIFE-8: routine reruns do not keep raw logs forever, but proof behind active work and fixed baselines stays, and removed proof is shown as removed rather than passed. LIFE-2 and LIFE-7 also apply. No new HI criterion is proposed.

PR #55 already closed the analogous boundary in `library archive --apply`. Its `remove_verified`, stale-hold refusal, create-new rollback and `restored` ledger event are prior art. Evidence prune differs in one consequential way: its eligible observations need not be committed at HEAD, so rollback must not claim Git can always recover the removed bytes. If a recreated path prevents restoration, this change preserves the original removed bytes under `.chough/removing/` and names that path.

Scope is one bug-fix change and one PR: recheck immediately before every removal, use byte-checked holds, restore earlier removals after a later failure, and keep every existing eligibility, protection, authorization, bound, ledger and successful JSON behavior. The existing `restored` event shape is reused. No lifecycle gate, retention rule, archive implementation, timeout or output schema is weakened or redesigned.

The installed Chough 0.4.0 lacks the `change` command group. Lifecycle commands for this draft use this worktree's Chough 0.5.0 through `cargo run --locked -- ...`; the failed installed-command attempt created no native record. Ownership correction after the first approval attempt: `specs/cli/cli.spec.md` canonically owns `src/evidence.rs`, so the change declares `cli` alongside `history` and includes a minimal Purpose delta plus the CLI contract history in affected paths. A disposable worktree probe with actor `kimi-probe` accepted both deltas and reported `approval_valid: true`; that probe record is not human approval and was removed with the probe worktree. The delivery worktree remains draft pending Leif's approval through the orc.

Role review found no new product or accessibility decision. QA added an earlier-set rollback scenario so a failure after one complete set cannot leave that set pruned. Engineering and architecture confirmed the create-only recovery hold is necessary for uncommitted evidence and that no archive refactor or native read belongs here.

Scope approval was recorded by the orc as 0xLeif at `80fbe1df895f78d8db8fa33daba54914bcad80aa`. Failure-first regressions were committed at `3ad9df48933b880836c812c06369d00b9369e918`, and their failed observations and local bugs at `55cb7fa5`. Implementation is now underway; implementation review remains a human gate through the orc.

## Scope amendment and reapproval

Orc t19f directed a send-back after the declared Linux verify on `7febd53a86d1a007565ff38638b41abd760c123f` failed `moved_requirements_keep_their_wording_under_their_new_ids`. The original approved scope omitted `tests/spec_split.rs`, which pins the exact REQ-history-004 body. The amendment adds only that path; the two approved semantic deltas, requirements, acceptance behavior and existing paths stay unchanged. No HI or REQ ID is added. Native send-back uses the agent actor `sol-ch65` with the honest dependency reason; the original approval remains historical and reapproval is required.

After reapproval only, replace the REQ-history-004 pin `128c08a8a763ba404099891f17da8c28e53da365375a44decfd8755c78208fd2` with `d807b65dc985f3d8971b83fcb6c1e1cc4615beb80ca30dee2ca595e847acf0d2`, computed as SHA-256 of the trimmed approved requirement body. Keep the assertion, ID mapping and every other pin unchanged. The implementation pin is intentionally unchanged at this scope gate. Failure receipt: `.chough/artifacts/issue-65-prune-race/finish/linux-verify-before-pin.log`, `OBS-product-test-f87bec1f3802297bc62bebe7`.

Leif reapproved this one-path amendment at `4ef6b8e29913077a6f557d1371a930540e02884b`, replying "Yes" to the listed three-gate batch on 2026-11-07. Orc t19f recorded the native approval as 0xLeif; the worker verified approval_valid and unchanged deltas, closed only the named Chough reapproval gate with the exact relay, and changed only the intended test digest literal. Implementation review, archive and merge remain separate human gates.

## From the change's design.md

# Design

## Prepared removal

At apply start, call `stale_holds` and refuse if any prior recovery file needs inspection, before writing a new ledger line. For each eligible set, the existing fresh classification and artifact fingerprint checks still run. Read the observation and removable artifact bytes and permissions before appending the set's existing ledger entry. The ledger entry is unchanged and remains first.

Artifacts are removed first and the observation last so the set stays classifiable. Immediately before each file, recompute `history::protection_fingerprint` and require equality with the set's captured value. Call `remove_verified(root, path, expected_bytes)` instead of digest-checking plus `remove_if_present`. Its rename into `.chough/removing/` and exact byte comparison close the check/unlink window without replacing a writer's file. Record each successful removal's path, bytes and permissions.

## Rollback and recovery

Any protection mismatch, changed file or removal error after a ledger line triggers rollback for all earlier removals of that set and any earlier ledgered sets. Restore each with `create_new(true)`, write the exact saved bytes and restore permissions. Never overwrite a path recreated meanwhile. Continue best-effort restoration so one conflict does not strand other files.

If a recreated path or another restoration failure prevents recovery, write the original bytes create-only to the existing digest-derived `.chough/removing/<digest>.hold` path and name it in the refusal. Unlike archive-summary proof, an evidence-prune candidate need not be committed, so the error never substitutes "recover from HEAD" for an actual retained copy. Existing holds are never replaced and cause the next apply to refuse with an inspection remedy.

Append one `restored` event per ledgered set using the current schema: `schema_version`, `event`, `id`, `reason` and `restored_at_unix_seconds`. The reason names the original failure and every unrestored path or recovery hold. Existing ledger readers therefore undo the removal label, while the refusal makes partial recovery explicit. Return an error; never include the failed set in `pruned`.

Successful applies keep their existing `eligible`, `protected`, `totals`, `pruned`, `skipped`, `ledger`, `note` and policy output. Preview remains read-only.

## Deterministic fault injection

A narrow `#[cfg(test)]` hook in the evidence module runs at the boundary after a successful removal and before the next protection check. Tests use it to add a checkpoint reference, fail a later set after an earlier set completed, and recreate a path during rollback. It is not compiled into release binaries and has no environment variable or CLI setting. The partial-removal integration test instead makes the evidence directory read-only, exercising the real CLI without a seam.

## Ownership and compatibility

Only `src/evidence.rs` changes production behavior. `library archive --apply` and `src/history.rs` are untouched. Existing remove/hold and ledger reader functions are reused, with no new persisted schema, native read, lock, service or coordinator.

## From the change's testing.md

# Testing

This is the verification plan, not a claim that these checks have run.

## Failure first

Before changing the retention implementation, add these cases in `.chough/cases/history.prune-race.cases.toml` and their exact bindings:

| Case | Regression and required failing behavior on the base |
|---|---|
| `TC-history-prune-protection-race` | In-crate deterministic test: after one artifact is removed, a checkpoint protection appears before the next removal. The base leaves the first artifact missing and has no restored event. The fixed test requires every original file restored byte-for-byte, the new checkpoint reference preserved, an actionable refusal and the existing restored event. |
| `TC-history-prune-restores-partial-removal` | `tests/evidence.rs`, real CLI: the evidence directory rejects observation deletion after artifacts were removed. The base leaves artifacts missing. The fixed test requires record and artifacts restored with original bytes and permissions, a restored ledger event and no stale hold. |
| `TC-history-prune-restores-earlier-sets` | In-crate deterministic test with two eligible sets: the first set is completely removed before a later set fails. The base leaves the first set pruned. The fixed test restores both sets, appends a restored event for each and keeps every later protection or changed byte intact. |
| `TC-history-prune-restore-preserves-recreated-file` | In-crate deterministic test: a writer recreates one removed path during rollback. The fixed test keeps the recreated bytes, preserves the original uncommitted bytes in a named create-only hold, records the restore outcome and never claims Git HEAD contains those bytes. The base has no rollback or recovery hold. |

Run each exact test against the unfixed tree and save its command, exit status and assertion output under `.chough/artifacts/issue-65-prune-race/before/`. A compile error or a filter that runs zero tests is not a behavioral failure-first result. Record failed observations with actual generated IDs and link local product-defect bugs before implementing the fix. Do not edit or restamp those failures later.

## Focused and compatibility checks

After implementation, rerun the identical commands unchanged, then run:

```sh
cargo test --locked --bin chough evidence::tests
cargo test --locked --test evidence
cargo test --locked --test history
cargo run --locked -- cases check
```

Run the four new cases with `cargo run --locked -- cases run <case-id> --environment local-macos-arm64`, recording their actual returned observation IDs. Link the strictly later passes to the same bugs. Also exercise a successful `evidence prune --apply` in an isolated Git fixture and verify that its output and ledger entry retain their existing shape, while a refused apply reads as restored rather than completed pruning.

The full existing evidence, history, retention and archive suites must pass without changed expectations. In particular, shared artifacts, newly referenced records, missing or changed files, symlink refusal, unreadable sessions, ledger rotation and archive rollback remain covered.

## Full verification

With HI 0.8.0 and SpecSync 6.2.0, using Fledge 1.8.0 (the finishing worker verified all three installed pins):

```sh
fledge run fmt
fledge run lint
fledge run test
fledge lanes run verify
fledge run linux-verify
```

The Linux command uses the repository-defined Docker images and pinned toolchain. Hosted macOS, Ubuntu, MSRV, archive and other required CI results are claimed only after observed at the final PR head. No real repository retention apply, release or deployment is part of verification.

## Evidence boundaries

A structural pass, an exit code, an actor label or a restored event alone is not behavioral proof or human acceptance. Each race test must prove its hook ran. Every rollback assertion checks exact bytes and permissions, non-replacement of recreated files, recovery holds and ledger meaning. Scope approval, native verification, implementation review, archive and merge remain separate records.

## Finish regression: restore publication identity

The existing recreated-file case also runs `prune_restore_permissions_preserve_replacement_identity` on Unix. The deterministic hook replaces the public destination after rollback prepares bytes and before it sets the mode. It asserts the second writer keeps exact bytes, mode 0600 and inode, and the original uncommitted bytes survive in the named recovery hold. On the unfixed implementation, the test ran once and failed because rollback changed mode 0600 to 0640; see `.chough/artifacts/issue-65-prune-race/finish/restore-identity-before.log` and `OBS-product-test-972af92fbd4cb64f5dd3094a`. The failing test commit is `bf57eb259b17a2ed5634b5ef62f4b1520f8bdd93`. The fix reuses private staging and create-only hard-link publication; it never chmods or deletes a shared public destination during restoration.

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/context.md`
