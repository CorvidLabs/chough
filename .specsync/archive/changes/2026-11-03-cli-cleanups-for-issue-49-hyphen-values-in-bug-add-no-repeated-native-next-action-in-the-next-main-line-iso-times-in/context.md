---
change: cli-cleanups-for-issue-49-hyphen-values-in-bug-add-no-repeated-native-next-action-in-the-next-main-line-iso-times-in
artifact: context
---

# Context

Issue #49 collects six cleanups found while dogfooding `main` at a788869 (findings 2, 3, 6, 10, 11 and 12 of the 2026-11-02 status notes). Plan comment: https://github.com/CorvidLabs/chough/issues/49#issuecomment-6593417500.

- `bug add --title "--x"` fails in clap with `unexpected argument`.
- In `next`, the main line cuts mid-word because a `Gate not open` action embeds the gate detail, and that detail already carries the native next action, which the view also prints on its own line. Native refusals append "Run `specsync change status <id>`" even when the native next action names that command.
- The `next` text shows `Last failure: … at <unix seconds>`.
- `bug add` and `cases run` options have no help text.
- The `docs/cli.md` Commands table has no `history` row, and the approved-delta intro treats 6.1.1 behavior as current.
- `examples/feature-bug-loop.sh` leaks one `chough-exercise.*` temp dir per run.

The following are out of scope:

- The approval and actor wording in `stage.rs` refusals and `docs/cli.md` rows 391 to 393, which #44 changes.
- `SKILL.md` and `references/lifecycle.md`.
- `tests/next.rs` and the `next()` regions of `src/main.rs`, which #46 changes.
