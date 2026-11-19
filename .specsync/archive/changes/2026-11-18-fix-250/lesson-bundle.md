# Lesson bundle: fix-250

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 250
- **Kind**: BugFix
- **Specs**: history, cli, records
- **Paths**: src/stage.rs, src/session_md.rs, src/checkpoint.rs, src/doctor.rs, src/history.rs, tests/doctor.rs, docs/cli.md, specs/history/requirements.md, specs/cli/requirements.md, specs/records/requirements.md, .chough/cases/history.stale-gate-reason.cases.toml, changelog.d/2026-11-18-fix-250.md
- **Acceptance**: After chough doctor --fix --reason withdraws a stale gate, that gate's gate-closed Timeline entry in the archived story holds the reason as its withdrawn field, in the person's words; doctor's JSON gives decision_reason on each decision finding it fixed and on no other finding; withdrawing still records no human decision.

## Evidence

- Verification commit: `9bd520cbf58e0d7ff68582cfc026699eb4da39af`
- Base commit: `e47b4cfaabe2e68fd02246be01fb7e8e256fe00b`
- Verified by: `specsync check --spec cli --spec history --spec records`

## From the change's context.md

# Context

GitHub #250, found on a CorvidLabs/spec-sync clone after its migration.
`chough doctor --fix --reason "<text>"` withdrew five stale gates and archived
their sessions. The reason, which doctor requires before it applies a decision
fix, was written nowhere: not on the `gate-closed` Timeline entries, not in the
stories, and not in doctor's JSON. A reader of the archive could not tell why a
human gate was withdrawn.

Leif's decision, in his comment on #250 on 2026-11-18 from his planning
interview: "record it on each gate. When `doctor --fix --reason` withdraws stale
gates, each withdrawn gate's Timeline entry and the archived story carry the
reason in the person's words, and doctor's JSON echoes it back. REQ-history-012
gets that sentence."

The reason is a new field, `withdrawn`, on the gate's `gate-closed` entry. The
entry's own text stays the gate's text, which other readers match on, and
withdrawing still records no human decision. The story ends with the Timeline,
so it carries the reason through that entry.

## From the change's design.md

# Design

Nothing visual changes. A withdrawn gate's Timeline entry gains one field line:

```text
- 2026-11-18T10:00:00Z · gate-closed · accept → accept · Native approval of ch-alpha ^t-6b37ce
  - native_change: ch-alpha
  - withdrawn: Superseded; ch-alpha is archived
```

Doctor's JSON gives `"decision_reason": "<reason>"` on each decision finding it fixed.

## From the change's testing.md

# Testing

`tests/doctor.rs::doctor_decision_fixes_need_a_reason_and_record_no_human_decision`
is extended. After `chough doctor --fix --close <id> --reason "<text>"` it reads
the archived story of the session whose stale gate was withdrawn and expects the
`gate-closed` entry to hold `  - withdrawn: <text>`, and expects each decision
finding in doctor's JSON to give `decision_reason` and each other finding not
to. It fails on main, where the entry has no such field. The test's existing
checks stay: withdrawing records no human decision.

`tests/doctor.rs::a_stale_gate_reason_that_leaks_is_refused_or_redacted` (added after
the tester's finding on #345): a reason holding a credential is refused as
`refused:leak` with nothing written, and with `--redact` it is written redacted in
the story and in doctor's JSON. It fails without the leak guard.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-history-012 | the test above and the case in `.chough/cases/history.stale-gate-reason.cases.toml` |
| REQ-cli-030, REQ-records-004 | the same test: the entry is written and read back with its `withdrawn` field |

## Where these lessons go

- `specs/history/context.md`
- `specs/cli/cli.3md`
- `specs/records/context.md`
