---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: research
---

# Research

- Native status (SpecSync 6.1.1): `specsync change status <id> --json` on the archived
  change bound-external-commands-... returns `change.created_at` and
  `change.updated_at` only. No approval, verification, review or finalization time
  appears anywhere in its 65 KiB output (checked by walking every key; the only other
  numbers that look like times are observation IDs in artifact paths).
- Those times exist in native record files: `approvals.json` (`approvals[].gate`,
  `approvals[].timestamp`; gates `definition` and `finalization`),
  `verification.json` and `verification-attempts.json` (`timestamp`),
  `review.json` and `review-attempts.json` (`timestamp`), `finalization.json`
  (`timestamp`), and `state.json` (`created_at`, `updated_at`), under
  `.specsync/changes/<id>/` while active and `.specsync/archive/changes/<date>-<id>/`
  once archived. They are SpecSync's internal files, not a documented interface, so
  reading them is a choice for the human (open question in the plan).
- Transitions: `stage.rs` `Transition` is `deny_unknown_fields` with string `kind`,
  `from`, `to` and optional `reason`, `gate`, `revision`, `refresh`. New kinds
  `gate-opened` and `gate-closed` fit the existing fields, so released binaries keep
  parsing the log (`chough context` and `next` list them among recent transitions).
- `read_log_tail` keeps only the last 64 KiB; the report needs a full read bounded
  separately (proposed 16 MiB, like the retention ledger, reported partial beyond it).
- Case observations have no run duration today; only validation runs record
  `elapsed_seconds`. A single test's run time is therefore unknown unless a duration
  field is added (open question).
- Bug status history exists only as note text written by `bug update`
  (`status: open -> verified`); the first linked failed observation gives the
  reproduction time.
- Chough's own `.chough/transitions.jsonl` on main: 18 records (14 advance, 3
  reset, 1 skip), with several pairs at the same second, so same-second segments
  must be labelled as recorded together, not as instant work.
