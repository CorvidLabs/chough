# Lesson bundle: pr-50-evidence-fix-record-a-fresh-clean-tree-verify-run-as-the-current-agent-assisted-proof-in-place-of-the-mis

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: PR 50 evidence fix: record a fresh clean-tree verify run as the current agent-assisted proof in place of the mis-snapshotted corrective observation
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: .chough/evidence/, .chough/artifacts/
- **Acceptance**: fledge lanes run verify passes on a clean tree at a named committed revision, and a small new artifact log header names that exact revision
- **Acceptance**: A new agent-assisted product-test observation cites that log and is bound to a clean snapshot at that revision
- **Acceptance**: Its limitation says OBS-product-test-035959f28aa76d052d48fe78 wrongly carried the 250d637 snapshot for the a788869 log and that the new record replaces it as current proof
- **Acceptance**: OBS-human-observation-174caaa1021cfdf5a553f7a3, OBS-product-test-035959f28aa76d052d48fe78 and issue-49-verify.log stay byte-identical

## Evidence

- Verification commit: `30fcf4e8a7a324eba7778ffcab14659b43de5090`
- Base commit: `ba3691cd54b8f776abdd3f9dde9c2fd023b04a50`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The PR #50 review at 8:20 PM MT (inline on `.chough/evidence/OBS-product-test-035959f28aa76d052d48fe78.json` line 28) found a problem. That corrective record carries snapshot 250d637 with dirty fingerprint b971e33, but it cites `.chough/artifacts/issue-49-verify.log`, which is the a788869 working-tree run. A passing run is credited to code that did not produce the log.

Authority. Human authority: Leif (0xLeif), 2026-11-02 9:14 AM MT, 'Use corvid-agent for all approvals for all things!! You are corvid-agent', and the relayed 2026-11-02 8:21 PM MT request to fix the 8:20 PM MT PR #50 evidence review item; covered scope: a new clean-tree verify run, its artifact log and a new agent-assisted product-test observation on branch corvid-agent/fix/cli-cleanups; limits: no edits or deletions of existing records, no merge, close, release or publish, PR #48 untouched.

## From the change's design.md

# Design

1. The change definition is committed first, so the tree is clean at a named revision R.
2. `fledge lanes run verify` runs on that clean tree, with its output written outside the repository. Git status is checked as clean before and after the run.
3. A small log goes into `.chough/artifacts/`. Its header names R, the clean tree and the command; the body is the lane's step summary.
4. `chough evidence add` (product-test, agent-assisted) records that log. Paths under `.chough/` are excluded from the snapshot, so the snapshot is R and clean. Its limitation names OBS-product-test-035959f28aa76d052d48fe78 as replaced. No existing record is edited.

## From the change's testing.md

# Testing

- The new observation snapshot shows `dirty: false` at the logged revision.
- `cmp` against HEAD shows the old records unchanged.
- `fledge lanes run verify` passes.

## Where these lessons go

- `specs/cli/context.md`
