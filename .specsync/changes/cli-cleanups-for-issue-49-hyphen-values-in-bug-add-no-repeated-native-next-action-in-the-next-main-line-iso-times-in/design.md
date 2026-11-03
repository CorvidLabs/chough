---
change: cli-cleanups-for-issue-49-hyphen-values-in-bug-add-no-repeated-native-next-action-in-the-next-main-line-iso-times-in
artifact: design
---

# Design

1. `src/bugs.rs` `AddArgs`: `allow_hyphen_values = true` on the free-text options, plus a doc comment on every option.
2. `src/stage.rs`:
   - `native_hint(next, status)` returns `Native next action: {next}.`, adding ` Run \`{status}\`.` only when `next` does not contain `status`. It is used in the five native refusals that append both.
   - `assess` and `done_action` build `Gate not open` actions from the gate detail before the native hint (`action_detail`). The full detail stays in `gate.detail` and in refusals.
3. `src/main.rs` `next_text`: `Last failure: … at {timing::utc(seconds)}`. `timing::utc` becomes `pub(super)`.
4. `src/cases.rs` `CasesCommand::Run`: doc comments on `id`, `--environment` and `--observation-id`.
5. `docs/cli.md`:
   - Add a Commands row for `history show|adopt [--apply]|prune [--apply]`.
   - Rewrite the guard intro sentence: SpecSync 6.1.1 and earlier keep `approval_valid` true (BUG-0002), and 6.1.2 flags it. Chough keeps the guard. The later 6.1.1 mentions name that version's digest framing and module-name rule and stay.
6. `examples/feature-bug-loop.sh`: an EXIT trap removes the directory only when the script created it, the mode is `full` and the exit status is 0. Otherwise the directory is kept and its path printed.
