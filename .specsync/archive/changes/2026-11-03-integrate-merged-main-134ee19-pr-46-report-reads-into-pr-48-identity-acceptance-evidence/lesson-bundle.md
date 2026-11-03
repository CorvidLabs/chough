# Lesson bundle: integrate-merged-main-134ee19-pr-46-report-reads-into-pr-48-identity-acceptance-evidence

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Integrate merged main 134ee19 (PR 46 report reads) into PR 48 identity acceptance evidence
- **Kind**: Operations
- **Specs**: cli
- **Paths**: .chough/session.json, .chough/transitions.jsonl, .chough/evidence/, .chough/artifacts/, .chough/cases/cli.cases.toml, CHANGELOG.md, specs/cli/cli.spec.md, src/main.rs
- **Acceptance**: The checkpoint is a three-way merge against base 16fe55d: 197 observation pointers, 37 file refs, about 64466 bytes; PR #46 removals and its 24 direct evidence-file pins kept; no union resurrection
- **Acceptance**: Transitions keep main lines in order plus PR #48 additions
- **Acceptance**: PR #48 version-history row becomes 25 after main row 24
- **Acceptance**: The PR #48 same-second test reads freshness through the renamed request-local reads without weakening its assertion
- **Acceptance**: Existing identity observations stay as history; the combined source is requalified with fresh observations
- **Acceptance**: fledge lanes run verify passes

## Evidence

- Verification commit: `5c0f57f8e1cb227b6ad48356bc237ab12f9f4fe3`
- Base commit: `781ee446db654bd159295a48fdbee25cebb26cb1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif merged PR #46 as main 134ee1966ed6e6e24739a8374fb30ef4a0445750 on 2026-11-02 at 9:08 PM MT. PR #48 (head 4f43cb1, Refs #30) then conflicted in four files: `.chough/session.json`, `.chough/transitions.jsonl`, `CHANGELOG.md` and `specs/cli/cli.spec.md`. PR #46 also renamed `evidence::freshness` to the request-local `evidence::Reads`, so the PR #48 same-second test in `src/main.rs` no longer compiled.

Authority. Human authority: 0xLeif's through-PR authorization recorded in PR #46's archived context (.specsync/archive/changes/2026-11-02-read-report-case-definitions-once-per-request-and-expose-local-phase-costs/context.md: 'And I already gave it some approval and told it can do any changes. It just needs a PR.'), his PR #48 comment issuecomment-6595229360 (2026-11-02 9:10 PM MT: import the main squash with a true three-way checkpoint reconciliation; Grok owns branch, native and checkpoint writes for this PR), and Leif's 2026-11-02 9:14 AM MT 'Use corvid-agent for all approvals for all things!! You are corvid-agent'; covered scope: integrating merged main 134ee19 into PR #48 (checkpoint and transition reconciliation, version-history renumbering, the test call to the renamed freshness read, requalification); this is separate from PR #48's own test-only scope; limits: no 200-pointer or 65536-byte limit increase, no pruning, no edits to old records, no merge, close or release, #30 human and cold-start criteria stay open.

The criteria #30 still has open (the human and cold-start ones) stay open, and this change adds no cold-start result.

## From the change's design.md

# Design

- **Checkpoint.** Run `chough checkpoint --merge` with main's three-way merge, against base 16fe55d. Theirs has its current-cycle group set to base values, so that PR #48 (the owner of this branch) keeps its cycle; all collections are merged three ways. Result: 197 observations equal to main's set (none of the 24 removals come back), 37 file refs equal to main's, and 64466 bytes.
- **Transitions.** Main's lines in their order, followed by the 10 lines PR #48 added after base.
- **Text conflicts.**
  - CHANGELOG: main's Unreleased entry first, then PR #48's Tests entry.
  - cli.spec.md: PR #48's history row is renumbered to 25 and the front matter to `version: 25`.
- **Test.** `src/main.rs` reads freshness through `evidence::Reads::new(&a).observations(&current, Some("local"))` and asserts that each of the two records is `current-observation`.
- **Requalification.** The PR #48 cases are rerun at the combined source, and the older identity observations are kept as history.

## From the change's testing.md

# Testing

- Pointer, file-ref and byte counts are checked against base, ours and theirs.
- The PR #48 cases are run with `chough cases run`.
- `fledge lanes run verify` passes.

## Where these lessons go

- `specs/cli/context.md`
