---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: tasks
---

# Tasks

Write REQ-cli-071 in `deltas/cli.md`, with the relay note's fields in order, the
refusals before SpecSync runs, the leak checks and the passthrough.

Add `ApproveArgs` and `approve` to `src/change.rs` and dispatch it in `run`,
keeping the change's direct-write baseline current around the SpecSync call.
Update the `Change` doc in `src/main.rs`.

Add the tests in `tests/change.rs` on the stand-in `specsync` pattern: the exact
arguments and the note's shape, the `chough <version>` default, the refusal
without a relay field, the byte-for-byte passthrough and exit code, the leak
refusals before SpecSync runs, and the unchanged tracked `.chough/` files.

Say in `docs/cli.md` and `README.md` that `change approve` records the relayed
approval with its note and passes SpecSync's output through.

Add `changelog.d/2026-11-25-approve-passthrough.md` and
`.chough/cases/cli.approve-passthrough.cases.toml` (drafted at the session's
`agent/` folder, added after `change check` materializes REQ-cli-071, because the
spec-split test reads the canonical requirements).

The definition approval of this change stays the human's; the scope approval,
implementation review, push and hosted CI are separate.
