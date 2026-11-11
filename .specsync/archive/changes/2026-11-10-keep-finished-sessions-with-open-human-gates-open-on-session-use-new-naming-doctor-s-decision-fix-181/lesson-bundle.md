# Lesson bundle: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep finished sessions with open human gates open on session use/new, naming doctor's decision fix (#181)
- **Kind**: BugFix
- **Specs**: history
- **Paths**: src/sessions.rs, tests/sessions.rs, docs/cli.md, CHANGELOG.md, .chough/cases/history.gated-switch-181.cases.toml, knowledge/
- **Acceptance**: A finished session (stage done, or native change finalized on HEAD) with a non-empty awaiting_human list stays open on session new and session use and is reported in kept_open with its open-gate count and a recovery: 'chough doctor --fix --reason <text>' when its native change is finalized, otherwise the checkpoint --close-gate command; a finished session without open gates is archived unchanged as before; the existing inbox-unfolded kept-open behavior is unchanged. Regression: a switch with one gated finalized session, one gated done session without a finalized change and one ungated finalized session archives only the ungated one, leaving the two gated sessions byte-identical.

## Evidence

- Verification commit: `2f21c1f353efda8748ffda42ded58238d91de8c0`
- Base commit: `2981a59cd4a545c7ce38421c72e53469993194cd`
- Verified by: `specsync check --spec history`

## From the change's context.md

# Context

Issue #181 (found in CorvidLabs/spec-sync PR #993): `chough session use`/`session new` archived finished sessions that still held an open awaiting-human gate, silently dropping the gate; `chough doctor` treats the same sessions as `stale_gates`, a decision fix that needs `--reason`.

Constraint: REQ-history-012 already defines doctor's `finished_session` as a finished session with NO open gate, so the switch behavior contradicted the existing doctor contract; this change aligns REQ-history-006 with it instead of adding new policy. No new criterion IDs; FLOW-16/DRIVE-17 keep applying: no human decision is recorded by the switch.

## From the change's testing.md

# Testing

| Requirement | Coverage |
|---|---|
| REQ-history-006 | `TC-history-switch-keeps-gated-finished-sessions-open` binds `a_finished_session_with_an_open_gate_stays_open_on_switch` in tests/sessions.rs: a gated finalized session and a gated done session stay open byte for byte with their gate counts and recoveries, and an ungated finalized session is archived unchanged |

Failing-first: the regression is committed before the fix and run there through the case (failed observation linked to the #181 bug); the same case passes after the fix. Existing switch, inbox and doctor suites run unchanged in the full `fledge run test` suite.

## Where these lessons go

- `specs/history/context.md`
