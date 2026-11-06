---
change: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
artifact: design
---

# Design

- **`src/change.rs`** builds the SpecSync argv for each subcommand. It runs SpecSync with its working directory at `--root`, then copies SpecSync's stdout and stderr bytes to Chough's own and exits with SpecSync's exit code. Chough adds nothing to either stream on success or on a SpecSync refusal.
- **Writes** (`new`, `answer`, `check`) run through `records::run_owned` with no deadline, so a signal to Chough stops the child (FLOW-14) and no time bound does.
- **`status`** runs with the native read deadline (`native_deadline`, 120 s unless `CHOUGH_NATIVE_TIMEOUT_SECONDS`). Past it, Chough stops the child and refuses with exit 1: "Error: specsync change status gave no response within <bound>; nothing was recorded. Run `CHOUGH_NATIVE_TIMEOUT_SECONDS=<longer> chough change status [<id>]` to wait longer."
- **A missing `specsync`** (it cannot be started) refuses with exit 1 before anything runs: "Error: specsync could not be started (<io error>). Install it with `cargo install --locked specsync@6.2.0` (README, Install the toolchain), then run `chough change <step> …` again."
- `--root` defaults to `.`, as for every other command. No file under `.chough/` is read or written, so no session, lock or history is involved.
- Argument order passed to SpecSync: for `new`, the description, then `--kind`, each `--spec`, each `--path`, and `--json`.
