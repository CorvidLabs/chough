---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: plan
---

# Plan

## Approach

Add `ship` to `chough change`. It checks everything first and then runs `specsync change ship`. After success it runs Chough's own `checkpoint --close-gate` and `stage advance --to done` for the session. The walkthrough, the skill's closeout and the docs move to it.

## Out of scope

- Approve and review (slice 5).
- Flagging direct lifecycle writes (slice 3, DRIVE-21).
- Gate summaries (slice 4).
- `library index` and commits.
- A `chough change finalize`.

## Steps

1. Add the REQ-cli-059 tests to `tests/change.rs` with a stand-in `specsync` that ships and reports archived status. They fail first: `chough change ship` does not exist.
2. Extract the relay check `checkpoint --close-gate` runs into a shared function in `src/checkpoint.rs`, with no behavior change. Add `ship` to `src/change.rs`.
3. Move `docs/walkthrough.md`, `docs/lifecycle.md`, `docs/cli.md`, the README row, the CHANGELOG and the skill's accept, lifecycle and stages references (both copies) to `chough change new`, `check` and `ship`. Update `src/skill.rs`, `tests/package.rs` and `tests/docs.rs` if their checks require it.
4. Materialize the delta, record the cases, run a mutation round, and run `fledge lanes run verify` on Linux.

## Risks

- Ship cannot be undone. A refusal after it leaves the change shipped and the session at accept, and the output says so and names the remaining commands.
- Running Chough's own binary for the two follow-up steps costs two process starts.
- The relay values pass through a command line, as they already do for `checkpoint --close-gate`.

## Constraints consulted

- DRIVE-2: the move into done keeps one native status read; ship reads none itself.
- DRIVE-4: nothing writes to `.chough/` between check and finalize.
- FLOW-14: a stop stops SpecSync.
- FLOW-16 as amended and FLOW-10: Chough records only the relayed decision it is given.
- Leif's rule: each refusal names a recovery and a test runs it.
