---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: testing
---

# Testing

## Disposable red receipts already executed

See research.md for exact argv, accepted base, private fixture, raw hashes and compact actual failures. Capture: 0 passed / 1 failed, writer removed and `pruned=["target"]`. Unsafe rollback: 0 passed / 2 failed (file and ancestor symlinks), missing recovery hold, missing earlier artifact/record and zero restored events. Each hook asserts it fired; the symlink tests also verify writer identity/target bytes before the failing recovery assertion.

## After scope approval: real-tree failure first

Land the same-purpose tests and cfg(test) seam under src/evidence.rs, commit them first, and execute unchanged commands:

```sh
cargo test --locked --bin chough evidence::tests::pr197_captured_replacement_bytes_survive -- --nocapture
cargo test --locked --bin chough evidence::tests::pr197_unsafe_restore_ -- --nocapture
```

The first filter must execute one test; the second must execute both file/ancestor variants. Capture compact failure observations and link local bugs, then implement. Rerun these identical filters after the fix. No expectation, timeout or scope weakening is permitted.

## Intended per-change cases

Create `.chough/cases/history.pr197-p1.cases.toml` only after scope approval:

- `TC-history-prune-capture-byte-authority`, linked to LIFE-2/7/8 and REQ-history-004: bind the first command; require its named test and `test result: ok. 1 passed`.
- `TC-history-prune-unsafe-restore-continuation`, same intent/requirement: bind the second command; require both named symlink tests and `test result: ok. 2 passed`.

Environment local-macos-arm64 and the declared Linux test environments; method automated, functional track, test-log artifacts. Preserve old case definitions. Validate global ID uniqueness and links with Chough before executing bindings.

## Assertions and regressions

Capture authority: writer's replacement bytes and observation survive; a changed captured candidate cannot appear as pruned. Before the production fix, extend the capture red test with an earlier-completed-set variant so every previous removal is checked for rollback, and retain those assertions after the fix.

Unsafe path: both final-file and ancestor symlinks retain inode and target content; original removed bytes appear in a named safe hold; an earlier completed set's artifact, record and permissions are restored; every ledgered set gets restored accounting with an honest partial outcome. Cleanup errors cannot bypass those attempts/accounting.

Run all evidence unit tests, tests/evidence.rs and tests/history.rs; keep all prior race/rollback/mode assertions. Run spec_split unchanged to prove the requirement-body digest remains the accepted value. All current eligibility, shared/changed artifacts, safe-path and archive rollback regressions remain required.

## Full qualification and human boundaries

Use Fledge 1.8.0, HI 0.8.0 and SpecSync 6.2.0, with the branch-built helper. Put `/Users/leif/.cargo/bin` first in hc terminals. Run project macOS `fledge lanes run verify`, declared `fledge run linux-verify`, strict spec check, `specsync change audit --strict`, normal pre-push and knowledge checks on committed inputs. Record actual failures and passes. All bookkeeping must precede the final native check, and full lanes need current native verification with no intervening input changes. Trust is currently unavailable; do not adopt it under this scope.

Present a fresh exact-head implementation gate for corrected code on #197. No approval/review probe, worker review, self-acceptance or merge. Existing acceptance is historical. Preserve the original archive and all sibling proof. Root records actual human decisions and owns the #994 → #999 → #197 merge queue.


## Executed corrective qualification

Real failure-first commit: `8f7574956f2f8b8789022e68d6a02e9ef0567062`.
The exact capture binding failed exit101 (both first/later scenarios deleted writer
bytes); the unsafe binding failed exit101 in both symlink variants (no recovery
hold, earlier restoration or restored accounting). Raw failed OBS logs are retained.
Correction commit: `6748c3d30a005e46abe468fc00eb43900eda374f`. The same bindings
passed; all four previous #65 cases passed unchanged. Focused evidence unit14,
evidence integration30, history25 and spec_split4 passed. The corrected focused
summary supersedes an earlier retained prose count error; JSON counts were accurate.
Independent disposable mutants removing captured-byte authority or reintroducing
the outer restoration-path escape fail the one/two target tests respectively;
baseline3 passed. Raw mutation failures and a compact executed summary are retained.

Full frozen qualification: `3b9d008d6c48399f8ac4f7c62dee9fcb1b9cc4e9`.
Pinned Linux passed all10steps/600 tests. Sequential macOS passed all10steps/596
tests. The first concurrent macOS attempt failed the existing parallel checkpoint
writer test at its unchanged30s lock bound. Its isolated retry passed1/1 in1.20s
and the full sequential lane passed at identical inputs; cause remains unverified,
tracked locally outside the two P1 behaviors. No timeout/assertion was changed.
See `.chough/artifacts/sol-ch65-corrective-full-summary.json` for exact argv,
exits, SHA256 receipts, limits and the raw initial failure/diagnostic artifact paths.

The preservation check compared4412 existing Chough/archive/policy/split files to
`1fc4c67b59f05097cab4cafa446aa58198f8ee56`: none changed. The approved six
product/contract/case paths are the complete corrective product diff. Final
checkpoint/knowledge/evidence writes precede the last native check; source,
contracts, case definitions and assertions must match this full-tested tip exactly.
Fresh corrected exact-head implementation review, successor archive, final hosted
CI and conversation resolution remain later gates. No old acceptance is reused.
