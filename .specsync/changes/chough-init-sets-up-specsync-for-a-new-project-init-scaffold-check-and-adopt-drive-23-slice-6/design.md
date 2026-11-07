---
change: chough-init-sets-up-specsync-for-a-new-project-init-scaffold-check-and-adopt-drive-23-slice-6
artifact: design
---

# Design

- **`setup::run(root, modules)`** returns `Ok(Setup)` with `{status, steps: [{command, exit_code}]}`, or refuses.
- **Eligibility.** No module named gives `status: "no-module"`. An existing `.specsync/` or `specs/` gives `status: "existing-setup-left-as-is"`. Otherwise it runs the four steps in order, each `specsync …` through `records::run_owned` in `--root`, with no deadline, under the stop watch.
- **A step that cannot start** (`NotFound`) refuses: "specsync could not be started (…); nothing was written. Install it with `cargo install --locked specsync@6.2.0` (README, Install the toolchain), then run `<the same chough init>` again."
- **A non-zero step** refuses: "`<step>` exited <code>, so SpecSync setup stopped and nothing of Chough's was written. Run `<step>`, then `<steps left>`, then `<the same chough init>` again."
- **`init`** calls `setup::run` first and adds `specsync_setup` to its JSON output. Its existing create-only writes are unchanged.
- **The same `chough init`** is the command line as given, rebuilt with `--objective`, `--intent-scope`, each `--module` and each `--host`.
