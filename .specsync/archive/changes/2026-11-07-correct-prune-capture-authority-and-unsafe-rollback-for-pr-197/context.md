---
change: correct-prune-capture-authority-and-unsafe-rollback-for-pr-197
artifact: context
---

# Context

## Corrective scope, not historical reacceptance

Issue: https://github.com/CorvidLabs/chough/issues/65
Existing PR: https://github.com/CorvidLabs/chough/pull/197
Capture P1: https://github.com/CorvidLabs/chough/pull/197#discussion_r4502629883
Unsafe-restore P1: https://github.com/CorvidLabs/chough/pull/197#discussion_r4502629890

The predecessor `make-evidence-prune-apply-recheck-protections-and-restore-partial-removals` is archived under `.specsync/archive/changes/2026-11-07-make-evidence-prune-apply-recheck-protections-and-restore-partial-removals/`. Leif's implementation acceptance at `8a9a1a3c030a6a5b18cbe98958b8a413aa5d2a01` and its archive are immutable historical evidence. Current accepted archive/bookkeeping head is `1fc4c67b59f05097cab4cafa446aa58198f8ee56`. Its 593/597 local tests and 14 hosted checks passed, but the final complete thread scan found these two uncovered P1s. Neither old acceptance nor green CI accepts a correction that has not been reviewed.

Orc t19f directed a new active successor in this same branch, worktree, issue and PR. No new PR, worker or issue is authorized. Before this definition gate, executable regressions were added only to a disposable copy of the accepted head, with a private target directory. Production and test files in the real tree remain unchanged until Leif approves this exact successor scope. No approval/review probe is authorized or run.

## Existing intent and ownership

LIFE-2, LIFE-7, LIFE-8 and REQ-history-004 already require honest proof retention, changed/unsafe candidate preservation and complete rollback accounting. No HI or requirement ID is added. The requirement body and `tests/spec_split.rs` digest stay byte-identical. This is a public-contract clarification (`public_contract = yes`, `no_spec_change = false`): the CLI Purpose and history Error Cases explicitly describe the capture-boundary refusal and per-file unsafe-restore outcome.

`cli` canonically owns `src/evidence.rs` and its unit tests; `history` owns its contract and context/testing companions. Native `depend` and five `supersede` obligations link the exact predecessor entries for every existing owned path being corrected. Digests came from the pinned tool's predecessor `acceptance_entries`, not guessed hashes. The new per-change case file has no predecessor entry. No owner-policy or reserved exact-owner workaround is needed.

## Boundaries

Keep eligibility/protections, authorization, limits, schemas, successful output, all existing assertions, old case definitions, archives, sibling sessions and earlier failure evidence. No source edit, actual case-file edit, native approval or review precedes the scope gate. After implementation, obtain a fresh exact-head implementation acceptance; the old review is historical only.
