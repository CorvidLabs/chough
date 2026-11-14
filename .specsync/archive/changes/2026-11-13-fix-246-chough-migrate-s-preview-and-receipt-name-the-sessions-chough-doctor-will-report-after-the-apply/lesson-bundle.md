# Lesson bundle: fix-246-chough-migrate-s-preview-and-receipt-name-the-sessions-chough-doctor-will-report-after-the-apply

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix #246: chough migrate's preview and receipt name the sessions chough doctor will report after the apply
- **Kind**: BugFix
- **Specs**: records, cli
- **Paths**: src/doctor.rs, src/migrate.rs, tests/migrate.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: chough migrate's preview and its receipt's Follow-up name each session chough doctor reports right after the apply: finished_session ones to archive with chough doctor --fix, and stale_gates ones with each open gate's text and chough doctor --fix --reason, computed by doctor's own session check on the planned records (red then green)

## Evidence

- Verification commit: `a3479d697016981e7e3507c3be1876e67a0d4303`
- Base commit: `25af1398b2627c339a5ce0a0b3f3cd78a79e261c`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

corvid-agent, adopting Chough 0.6.0 in CorvidLabs/spec-sync and CorvidLabs/raven, found
that `chough migrate`'s preview listed three follow-ups (`records_in_verification`,
the merge-driver lines and `chough skill update`) and nothing else. Right after the
apply, though, `chough doctor` reported `finished_session` and `stale_gates`
findings. A stale gate's withdrawal is a decision, so the person consenting saw it
only after the apply, and the adoption PR had to go back to the human. Filed as GitHub
#246. t38b reproduced it with a 0.5.x fixture holding a finished session and an
archived-change session waiting on two gates (red at 129a4dbb).

On 2026-11-13T04:37Z Leif approved the scope ("Chough #246 migrate preview", relayed
by t370): the migration preview and receipt also list the sessions doctor will flag
right after the apply, as a small migration wording change. A docs-only line was the
fallback, and it isn't needed.

## From the change's design.md

# Design

Doctor's per-session check in `src/doctor.rs` becomes one function that takes a
parsed session and its unfolded inbox count. Doctor calls it on disk, with no change
in what it reports. `chough migrate`'s plan in `src/migrate.rs` calls it on each
planned `.chough/sessions/<key>/session.md`, parsed from the planned bytes, with the
planned inbox count, and turns each finding into a follow-up. The archived native
changes it reads are already present before the apply. So the preview, the receipt
and doctor share one computation. `docs/cli.md`'s Migration section and the
CHANGELOG say so.

## From the change's testing.md

# Testing

`tests/migrate.rs`: the adopted 0.5.x fixture plus two sessions with archived native
changes, one waiting on two human gates and one with none. After `chough migrate
--apply`, `chough doctor` reports `stale_gates` and `finished_session`. The
preview's follow-ups and the receipt's `## Follow-up` each name both sessions, and the
stale one with both gates' text and `chough doctor --fix --reason`. Red at the test
commit, green at the fix. Doctor's existing tests still pass unchanged. The full local
suite runs at the final head.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-009 | `tests/migrate.rs`: the preview and the receipt name the sessions doctor reports after the apply |
| REQ-cli-055 | `tests/migrate.rs`: after the apply, doctor reports exactly the sessions the follow-ups named |
| REQ-history-012 | doctor's existing tests: its findings are unchanged by the shared check |

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
