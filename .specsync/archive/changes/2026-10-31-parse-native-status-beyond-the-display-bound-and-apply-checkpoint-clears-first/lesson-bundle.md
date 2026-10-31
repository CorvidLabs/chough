# Lesson bundle — parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Parse native status beyond the display bound and apply checkpoint clears first
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, src/stage.rs, src/main.rs, src/checkpoint.rs, tests/, .chough/, docs/cli.md
- **Acceptance**: With native status reporting summary.state archived in a JSON response larger than 16384 bytes, chough stage advance leaves accept for done with gate native:archived (BUG-0004, REQ-cli-008).
- **Acceptance**: Native status responses are parsed from the complete output up to a 4 MiB bound; a larger, failed, unavailable or undecodable response still refuses gated transitions as native-unknown.
- **Acceptance**: chough status and next report a large valid native response as observed rather than invalid-output, show the parsed summary state and next action, and keep the displayed content bounded with a visible truncation note.
- **Acceptance**: chough checkpoint applies --clear-awaiting-human and --clear-blockers before additions in the same call, so the newly given entries remain (BUG-0005, REQ-cli-007).
- **Acceptance**: Regression tests cover both defects; the existing suite, the feature-and-bug exercise and native structural checks still pass; the recorded BUG-0004 reproduction, rerun against the real archived evolve-chough-into-a-resumable-stage-workflow change in a disposable copy of the archive tip, now records accept -> done with gate native:archived, and BUG-0004 and BUG-0005 are verified with passing observations.

## Evidence

- Verification commit: `9992363a02c24a00852e0c8764a98a83f9a3b0dc`
- Base commit: `1a108d9844e5bc02e5439cb6b3688803d6435a53`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Two Chough defects surfaced while closing the archived change
`evolve-chough-into-a-resumable-stage-workflow` on PR #2. Both are reproduced with
failed observations and recorded in the local backlog.

- BUG-0004 (`bug-0004-repro-1`): once archived, `specsync change status <id>
  --json` returned 100,133 bytes (262 acceptance entries). `records::native` keeps
  only 16,384 bytes of stdout for display, and the gate parsed that truncated text,
  so `chough stage advance` refused to leave accept as `native-unknown` although
  `summary.state` was `archived`. `chough status` showed the same response as
  `invalid-output`. The failure was safe (a refusal, not a pass). Unit tests stubbed
  small status JSON and the exercise stops at accept, so nothing reached a real
  archive before this.
- BUG-0005 (`bug-0005-repro-1`): `chough checkpoint --clear-awaiting-human
  --awaiting-human X` returned an empty list, because clears run after additions.
  The same ordering applies to `--clear-blockers`.

The human asked for this fix after accepting PR #2. PR #2 is not merged yet, so
this branch starts at its archive tip `1a108d9` and its pull request is stacked on
`feat/resumable-stage-workflow`.

## Interview note

`public_contract` is answered `no`: the canonical contract and requirements do not
change, and SpecSync refuses `no_spec_change` with a public-contract change. The
observable CLI behavior does change, from a defect back to what REQ-cli-007 and
REQ-cli-008 already specify.

## From the change's design.md

# Design

## Native output

`records.rs` gains a parsing path next to the display path. One process run yields
both:

- display: the existing bounded `Section` (16 KiB per stream, truncation marker);
- parsed: the full stdout decoded as JSON when the process succeeded and stdout is
  at most 4 MiB. Larger output is `oversized`; non-JSON output is `invalid-output`.

`stage::native_state` reads `summary.state` from the parsed value. Failed,
unavailable, invalid or oversized responses stay `NativeState::Unknown`, so the
gates still refuse.

`main::native_status` sets the section state from the full parse rather than the
displayed text. The report gains `native_summary` with `state` and `next_action`
when present. When the displayed content is cut, the section says it was
truncated for display only and was parsed in full.

## Checkpoint flags

`checkpoint_command` applies `--clear-blockers` and `--clear-awaiting-human` before
adding entries, so `--clear-awaiting-human --awaiting-human X` leaves `[X]`.

## Not changed

No new commands, record schemas or dependencies. Context assembly keeps its 16 KiB
section bound. Module entry points keep `pub(super)`.

## From the change's testing.md

# Testing

Functional track only; nothing visible or copy-related changes beyond one status
note.

| Case | Test | Verifies |
|---|---|---|
| TC-cli-0038 | `tests/stages.rs::large_native_status_still_gates_by_its_summary_state` | A stubbed archived response of more than 16 KiB lets accept advance to done with `native:archived`; status reports `observed` with `native_summary.state` archived; a response over 4 MiB still refuses as `native-unknown` (REQ-cli-008) |
| TC-cli-0039 | `tests/stages.rs::clear_flags_apply_before_new_entries` | `--clear-awaiting-human` and `--clear-blockers` combined with new entries keep the new entries (REQ-cli-007) |

Regression: the full suite, `fledge lanes run verify` (including the exercise) and
hosted CI at the tip. Real-state check: rerun the BUG-0004 reproduction procedure in
a disposable copy of `1a108d9`, whose checkpoint is at accept for the archived
change, and record a passing observation; record a passing observation for
BUG-0005 from the regression test run; mark both bugs verified.

## Where these lessons go

- `specs/cli/context.md`
