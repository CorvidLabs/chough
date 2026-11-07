---
change: make-evidence-prune-apply-recheck-protections-and-restore-partial-removals
artifact: testing
---

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

With HI 0.8.0 and SpecSync 6.2.0, using Fledge 1.8.0 where available (the current local binary is 1.7.2 and any difference is reported):

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
