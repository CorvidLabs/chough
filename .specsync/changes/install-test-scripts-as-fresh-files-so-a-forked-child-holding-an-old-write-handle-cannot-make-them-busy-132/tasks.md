---
change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
artifact: tasks
---

# Tasks

- [x] Add the failing test `a_script_installed_while_an_old_write_handle_is_open_still_runs`. It holds a write handle open the way a forked child does, then installs and runs a script. It fails first with "Text file busy (os error 26)".
- [ ] Make `common::stand_in` and `Fixture::executable` write each script to a temporary name from a child `sh`, make it executable there, and rename it into place, so the script is a fresh file that no handle in the test process ever opened for writing. Use `Fixture::executable` for the inline stand-in in tests/cli.rs.
- [ ] Rerun the stress run (60 rounds), and run verify on Linux.
