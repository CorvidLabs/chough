# Build loop

`chough stage advance` from plan enters build only when native status shows the
definition approved. Then:

1. Take the next task from the native `tasks.md`. Stay inside the approved paths.
2. Implement with the project's conventions. Keep contracts, requirements
   traceability, tests and documentation in step.
3. Add or update case definitions for the affected requirements (see
   [testing.md](testing.md)).
4. Run focused checks through Fledge (`fledge run test`, `fledge run lint`, project
   lanes) and inspect the output, not just the exit code.
5. Fix failures and repeat. Record a genuine product failure as a bug.
6. Mark the task done in `tasks.md`, then save a checkpoint:
   `chough checkpoint --completed "<increment>" --next-action "<next step>"`.

## Ownership

When subagents help, give each explicit files or modules, the relevant contract and
requirements, and instructions to preserve others' edits. Only one agent edits a
shared file at a time. The checkpoint has one writer; other agents report back
instead of editing it.

## Escalate only what matters

Ordinary choices inside the approved scope are yours. Escalate a genuine blocker or
a decision that would change intent, contract, acceptance criteria or scope. Record
it with `--blocker` or `--awaiting-human`, and go back to plan if scope must change.

A model saying a task is done is not a native transition, a passed test or a review.
Native records and process results are what count.
