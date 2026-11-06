# Lesson bundle: bug-classification-deduplication-and-report-mappings-and-their-recovery-branches-153

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Bug classification, deduplication and report mappings, and their recovery branches (153)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/bugs.rs, tests/bugs.rs, docs/cli.md, docs/FRD.md, CHANGELOG.md
- **Acceptance**: REQ-cli-054 (new) states the bug value, deduplication and report mappings and their recovery branches; six tests in tests/bugs.rs assert its six criteria. Two fail first on defects found at 64aa77e0: a mistaken duplicate cannot be reopened, and a bug can relate to itself, with refusals naming no recovery. A small src/bugs.rs change fixes them: reopening clears duplicate_of, self-related links are refused, and refusals name chough bug list --all or chough bug update <id> --observation <id>. The four mapping tests each fail under a targeted mutation that the existing tests let pass. Every refusal's recovery command is run by a test. fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `d204d97ff893db23cfcfbc437500d11fda770fc7`
- Base commit: `615b587b33a1ef3a569f9557e2782a6be3df2f0f`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #153. The audit (main `700dad96`) rated "Chough: bugs" at 90%: "the entire classification/dedup/reporting surface is not independently exercised". Its 95% list asks to "complete classification, deduplication and report mappings; then exercise their negative/recovery branches with retained failure/fix proof".

Probing those branches on main `64aa77e0` (macOS) found two defects and two refusals that name no recovery:
1. After `chough bug update B --duplicate-of A`, `chough bug update B --status open` is refused: "duplicate_of is only set for duplicate bugs". No flag clears `duplicate_of`, so a mistaken duplicate cannot be reopened except by hand-editing the record.
2. `chough bug update A --related A` is accepted, while `--duplicate-of A` on A is refused.
3. "unknown bug: BUG-0099" (from update and report) and "a bug cannot duplicate itself" name no recovery command.
4. "verified needs a linked failed observation …" names no command either.

## Assignment and IDs

From the orc, deciding overnight for Leif (2026-11-06): own issue, a worktree from origin/main, failing tests first, a mutation check, and the approval queued for Leif. REQ-cli-054 is taken from the Chough ID registry: main's highest is 052, 053 is on a branch, and 054 was found nowhere.

## From the change's design.md

# Design

All runtime changes are in `src/bugs.rs`. No record schema changes; schema 1 bug records stay as they are.

- `update`: after the status, classification, scope and duplicate handling, a bug whose status is not `duplicate` loses its `duplicate_of`. The change "no longer a duplicate of <id>" goes into the same note as the status change.
- `update --related <id>` refuses the bug's own ID before any write: "a bug cannot relate to itself; name another bug with --related <id> (run `chough bug list --all` to see the bug IDs)".
- The self-duplicate refusal gains the same recovery. `load` of an unknown ID says "unknown bug: <id>; run `chough bug list --all` to see the bug IDs". That covers update, report, show and links.
- The two verified refusals append "link one with `chough bug update <id> --observation <id>`" and "link a later pass with `chough bug update <id> --observation <id>`".
- Search, scoring, thresholds, value lists and the report format are unchanged; the new tests pin them.

## From the change's testing.md

# Testing

Six tests in tests/bugs.rs, one per REQ-cli-054 criterion:
- `every_classification_scope_and_status_is_accepted_and_others_are_refused`
- `only_an_open_blocking_bug_blocks_leaving_test`
- `search_scores_words_and_shared_ids_and_refuses_only_likely_duplicates`
- `links_to_the_bug_itself_or_to_an_unknown_bug_are_refused_with_their_recovery`: fails first, because the self-duplicate refusal names no recovery, a self-related link is accepted, and the unknown-bug refusal names no recovery.
- `a_mistaken_duplicate_is_reopened_and_an_unverifiable_fix_names_its_recovery`: fails first, because "duplicate_of is only set for duplicate bugs".
- `the_report_shows_every_stored_field_and_none_recorded_for_empty_ones`

Each test runs the recovery its refusal names: `chough bug list --all`, `chough bug update <id> --status open`, and `chough bug update <id> --observation <id>` followed by verified.

Mutation check of the four mapping tests. Each mutant fails the new test, while the existing test named for it passes:

| Mutation | Existing test that passes it |
|---|---|
| `evidence-gap` recorded as `requirement-gap` | `a_scope_given_as_a_classification_and_the_reverse_name_the_right_flag` |
| `wont-fix` counted as open | `an_open_blocking_bug_sends_work_back_to_build` |
| shared-ID bonus dropped | `a_reproduced_bug_is_deduplicated_and_keeps_added_evidence` |
| duplicate threshold lowered to 0.25 | `a_reproduced_bug_is_deduplicated_and_keeps_added_evidence` |
| report hides the fixture | `unverified_out_of_scope_bugs_stay_visible_without_changing_scope` |

Retained failure and fix proof: the two failing-first tests are recorded failing before the fix and linked to a local bug, then recorded passing after it, and the bug is verified.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-054 | tests/bugs.rs, the six tests above |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
