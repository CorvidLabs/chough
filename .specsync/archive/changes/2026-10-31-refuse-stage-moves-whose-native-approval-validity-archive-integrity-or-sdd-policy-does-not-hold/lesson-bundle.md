# Lesson bundle — refuse-stage-moves-whose-native-approval-validity-archive-integrity-or-sdd-policy-does-not-hold

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Refuse stage moves whose native approval validity, archive integrity or SDD policy does not hold
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/stage.rs, tests/, .chough/, docs/cli.md, skills/chough-workflow/, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/
- **Acceptance**: Entering build, test or accept by advance, skip, back or reset is refused unless native status reports state approved, implementing or verifying with summary.approval_valid true and summary.correction_valid not false; the refusal names the failed condition and the native next action. With approval_valid false and an eligible state, plan -> build is refused (BUG-0006, REQ-cli-008).
- **Acceptance**: Reaching done by advance, skip, reset or release -> done is refused unless native state is archived and summary.terminal_evidence.validity is authenticated-history, exact or successor-covered; corrupt-history, a missing validity or any other state refuses. Reset and back to intent, design and plan stay allowed, and test and accept are also reachable for an archived change with valid terminal evidence (BUG-0007, REQ-cli-008).
- **Acceptance**: The SDD policy is read with SpecSync semantics: no .specsync/sdd.json, or a valid policy with enabled false, stays ungated with the existing labels; a missing enabled key means enabled; malformed JSON, a non-object, or an ill-typed known field refuses gated moves as refused:sdd-policy-invalid and names the file (BUG-0008, REQ-cli-008, REQ-cli-003).
- **Acceptance**: Every existing transition, side-stage and release behavior that these rules do not name is unchanged; the full suite, the feature-and-bug exercise and native structural checks pass; regression tests reproduce BUG-0006, BUG-0007 and BUG-0008 with real SpecSync in disposable repositories using clearly synthetic approvals, and the three bugs are verified by passing observations.

## Evidence

- Verification commit: `fccb78c9d4ee83c93feba1ab831fced2890bbd7c`
- Base commit: `ef6695fe9a753112dc77b156d11d9441ea81e529`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A review of Chough after PR #2 found three gate defects, reproduced on main `f19823d`
against SpecSync 6.1.1 in disposable copies of `examples/shipping` (synthetic
approvals only). They are BUG-0006, BUG-0007 and BUG-0008, each with a failed
observation and a reproduction log under `.chough/artifacts/`.

- BUG-0006: `stage.rs` gates build on `summary.state` alone. With state `approved` and
  `approval_valid: false` (scope changed after approval), plan -> build was allowed as
  `native:approved`.
- BUG-0007: `reset` accepts any main stage and gates only `build`, so `reset --to done`
  and `--to accept` succeeded while the change was draft.
- BUG-0008: `sdd_enabled` turns a parse failure into `Value::Null`, so truncated JSON,
  `"enabled": "yes"` and a missing `enabled` all became `ungated:sdd-disabled`.
  SpecSync itself rejects the first two as an invalid policy and reads a missing
  `enabled` as true (`#[serde(default)]`, default enabled).

Native interface, investigated in spec-sync `f240d37`: `change status <id> --json`
recomputes `approval_valid` from the scope digest on every call, but does not compare
approved delta bodies (BUG-0002, upstream; a separate SpecSync change). For archived
changes, `summary.terminal_evidence.validity` is `authenticated-history` (or `exact`)
when the archive authenticates and `corrupt-history` when it does not; `stale` applies
only to accepted changes. Frozen archives are not re-tested against later HEADs.

Out of scope: process bounds, validation stop timing and native read bounds (BUG-0009
to BUG-0011, a separate change), and SpecSync's own behavior.

## From the change's design.md

# Design

One destination check replaces the two purpose-specific gates in `src/stage.rs`.

| Destination | Prerequisite (SDD enabled, change recorded) |
|---|---|
| intent, design, plan | none: backtracking and reset stay free |
| build | state approved, implementing or verifying; `approval_valid` true; `correction_valid` not false |
| test, accept | the build prerequisite, or the done prerequisite |
| done | state archived; `terminal_evidence.validity` authenticated-history, exact or successor-covered |

- The check runs for advance, skip, back, reset and release -> done, so every route to
  a stage meets the same prerequisite.
- `approval_valid` must be present and true; a missing field is unknown and refuses.
  `correction_valid` refuses only when present and false.
- Refusal labels: `refused:native-<state>` (unchanged), `refused:native-approval-invalid`,
  `refused:native-correction-invalid`, `refused:native-archive-<validity>`,
  `refused:sdd-policy-invalid`, `refused:no-native-change` and `refused:native-unknown`
  (unchanged). Each detail names the native next action and status command.
- SDD policy, mirroring SpecSync: no file is `NotAdopted` (label `ungated:sdd-disabled`,
  unchanged); a JSON object whose known fields have SpecSync's types is read with
  `enabled` defaulting to true; anything else is `Invalid(reason)` and refuses. Unknown
  fields are accepted, as SpecSync accepts them.
- No new dependency, no native write, no change to how native status is read.
- The test stub in `tests/common/mod.rs` reports `approval_valid` (and, for archived,
  `terminal_evidence.validity`) the way SpecSync 6.1.1 does, so stubbed tests exercise
  the same fields as real ones; a stub mode omits them to test the fail-closed path.

## From the change's testing.md

# Testing

| Requirement | Evidence |
|---|---|
| REQ-cli-008 | tests/stages.rs regression tests; cases for BUG-0006, BUG-0007, BUG-0008 |
| REQ-cli-003 | tests/stages.rs policy tests (invalid policy is unknown, not disabled) |

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-008 | `tests/stages.rs`: approval-invalid build refusal; reset/back/skip/release destination checks; corrupt and missing archive validity |
| REQ-cli-003 | `tests/stages.rs`: malformed, non-object, ill-typed and missing-enabled policies |

Real SpecSync runs in disposable repositories (local Git identity); approvals in them
are labelled SYNTHETIC. Stubbed native status covers values that real SpecSync cannot
be driven to cheaply (`corrupt-history`, missing fields). Regression: full `cargo test`,
`fledge lanes run verify` (includes the feature-and-bug exercise), and a rerun of the
recorded reproductions, which must now refuse. Platforms: hosted CI on Ubuntu, macOS
and Windows; no platform-specific code.

## Where these lessons go

- `specs/cli/context.md`
