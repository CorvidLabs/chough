---
change: report-how-long-stages-tests-bug-fixes-and-human-decisions-took-from-existing-records
artifact: context
---

# Context

The human (0xLeif) agreed criterion LIFE-10 in this wording, captured with `hi`:
"I can see how long each stage, test, bug fix and human decision actually took, so I
can estimate future work from real history." The plan reference already asks agents
to estimate increments by relative size without inventing hours; today nothing
shows them what earlier work actually took.

Most of the needed times already exist, spread over records:

- `.chough/transitions.jsonl`: `at_unix_seconds` on every advance, iterate, back,
  reset, pause, resume, skip and side-stage enter or return.
- `.chough/evidence/*.json`: `observed_at_unix_seconds` per observation, with
  case, environment and outcome; `.chough/validation/` run summaries carry
  `started_at_unix_seconds` and `elapsed_seconds`.
- `.chough/bugs/*.json`: `created_at_unix_seconds`, `updated_at_unix_seconds`, and
  notes with `at_unix_seconds` and Chough-written `status: a -> b` text.
- SpecSync: `change status --json` exposes `change.created_at` and
  `change.updated_at`; approval, verification, review and finalization timestamps
  live in the change's native record files (see research).

What is missing is when a human gate opened and closed. `awaiting_human` in
`.chough/session.json` is a list that `--clear-awaiting-human` empties, so the time
a gate waited is lost. That is the one new record this change adds.

Constraints a resuming session needs: draft only until the human approves the scope;
`.chough/` is deliberately not an affected path (Chough writes after `change check`
stale native verification, Raven issues #10 and #12); Chough's own history is thin
(18 transitions on main, several written in the same second when backfilled), so the
report must say when history is too thin rather than invent a range.
