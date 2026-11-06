## ADDED

### REQUIREMENT REQ-cli-052

Several forward moves on one native read, and one read into done. `chough stage advance --to <stage>` SHALL move forward through each main stage, from the current one up to `<stage>`, as consecutive advances. `<stage>` SHALL be a main stage after the current one, and any other value SHALL be refused. Every refusal of `stage advance` SHALL apply, including a stage that is not active, a side stage and an open human gate (REQ-cli-030). Before recording anything, the helper SHALL check every step as `stage advance` alone would check it: its destination's native prerequisite (REQ-cli-008) and, on leaving test, the blocking-bug and unlinked-failure refusals. All steps SHALL share one `specsync change status` read, made only when a step has a native prerequisite. A refused step SHALL refuse the whole command, naming that step with its label and detail. Then no transition SHALL be recorded and the checkpoint SHALL stay byte-identical. Otherwise the helper SHALL record one `advance` transition per step, in order, each with its own gate label and the given note. It SHALL report the steps in order in `applied.steps`, with `applied.from` as the starting stage, `applied.to` as the target and `applied.gate` as the last step's label. `stage advance` without `--to` SHALL be unchanged.

On a move into done by `stage advance`, with or without `--to`, the trim of REQ-cli-041 SHALL take the change's acceptance manifest from the gate's `specsync change status` response for that change instead of reading it again. So the move into done SHALL make one native status read when no other native change is open or archived later and the trim removes nothing. Every other read SHALL stay live and keep its refusals: another change's manifest, and every validity probe around a removal. The reused response SHALL serve only that command and SHALL NOT persist. Trace: DRIVE-9, DRIVE-2, FLOW-2, LIFE-3.

Acceptance Criteria
- Under a per-change session (`chough session new`), with a counting SpecSync stub reporting an approved change at plan, `stage advance --to accept` exits 0 and runs `specsync change status` once. It records three advances (plan to build, build to test, test to accept) as event files in that session's cycle, reports them with their gate labels in `applied.steps`, and leaves the stage at accept. In a single-checkpoint project the same command appends the three to `.chough/transitions.jsonl`.
- From intent, `stage advance --to plan` runs no native read and records two advances.
- Each of these refusals makes at most one native read, records no transition, and leaves the session file and the history directory byte-identical:
  - at build with an open acceptance-blocking bug, `--to accept` is refused naming `refused:blocking-bugs` and the test to accept step;
  - at plan with the stub reporting draft, `--to test` is refused naming `refused:native-draft`;
  - a target at or before the current stage, a side, optional or unknown stage name, and an open human gate are refused.
- With the stub reporting an archived change with authenticated terminal evidence and an acceptance manifest, and the librarian on by default:
  - `stage advance --to done` from test runs `specsync change status` exactly once and reports `librarian_trim`;
  - `stage advance` from accept to done also runs it exactly once, where before this change it ran twice.
- `stage advance` without `--to` records one transition, as before.
- Every test of new behavior above fails against main before the implementation, and passes after it. The unchanged plain advance passes both times. `fledge lanes run verify` passes.
