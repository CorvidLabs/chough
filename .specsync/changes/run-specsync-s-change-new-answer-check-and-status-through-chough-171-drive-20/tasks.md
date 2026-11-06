---
change: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
artifact: tasks
---

# Tasks

- [x] Add the REQ-cli-057 tests in `tests/change.rs` with a recording stub `specsync`. They fail first.
- [x] Add `src/change.rs` and the `change` subcommand in `src/main.rs`.
- [x] Move the skill (both copies), `docs/cli.md`, `docs/lifecycle.md` and the CHANGELOG to `chough change` for new, answer, check --commit and status; update `src/skill.rs` and `tests/package.rs` if the skill digests require it.
- [x] Add REQ-cli-057 (materialized from the delta).

After `specsync change check` materializes the delta, not task boxes:
- record the failing-first and passing observations against `.chough/cases/cli.change-steps.cases.toml`;
- run the mutation round and record its logs;
- run `fledge lanes run verify` on Linux.
