---
change: parse-native-status-beyond-the-display-bound-and-apply-checkpoint-clears-first
artifact: tasks
---

# Tasks

Human gates (plain text, not tasks): scope approval of this change; implementation
review at acceptance. Post-verification closeout (push, CI, review, ship) is not a
task here.

- [ ] Parse native JSON from the complete output within 4 MiB in `records.rs`; use it in `stage::native_state` and `main::native_status`, adding `native_summary`.
- [ ] Apply checkpoint clears before additions in `checkpoint.rs`.
- [ ] Add the two regression tests and cases TC-cli-0038 and TC-cli-0039.
- [ ] Note the display and parse bounds in `docs/cli.md`.
- [ ] Rerun the BUG-0004 reproduction against a disposable copy of the archive tip; record observations; mark BUG-0004 and BUG-0005 verified.
- [ ] Run `fledge lanes run verify`.
