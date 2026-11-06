---
change: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
artifact: plan
---

# Plan

## Approach

Add a `chough change` command with four subcommands, each a thin runner over the matching SpecSync step, and move the skill and docs to it for those four steps.

## Out of scope

- Approve, review, ship and finalize (slices 2 and 5).
- Flagging direct lifecycle writes (slice 3, DRIVE-21).
- Gate summaries and native gate records (slices 4 and 5, FLOW-19 and FLOW-20).
- Project setup (init, scaffold, check, adopt).
- Mapping SpecSync's recovery text to `chough` commands.

## Steps

1. Add the REQ-cli-057 tests in `tests/change.rs` with a stub `specsync` that records its argv and working directory. They fail first: `chough change` does not exist.
2. Add `src/change.rs` and register `change` in `src/main.rs`.
3. Move the skill (both copies), `docs/cli.md`, `docs/lifecycle.md` and the CHANGELOG to `chough change` for these four steps, and update `src/skill.rs` and `tests/package.rs` if the skill digests require it.
4. Materialize the delta, record the cases, run a mutation round, and run `fledge lanes run verify` on Linux.

## Risks

- A hung SpecSync write waits until Ctrl-C. That's deliberate: killing a write mid-commit is worse.
- `chough change` adds a fourth noun next to stage, session and checkpoint.
- A changed skill file makes installed copies report `known-outdated` until `chough skill update`.

## Constraints consulted

- DRIVE-2: one native status read per command; `chough change status` makes exactly one.
- DRIVE-4: the native order is unchanged; Chough only runs SpecSync's own steps.
- FLOW-14: stopping Chough stops its children.
- FLOW-16 as amended and FLOW-10: no gate is recorded by this slice.
- Leif's rule: a refusal names a recovery command, and a test runs it.
