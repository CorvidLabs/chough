---
change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
artifact: testing
---

# Testing

- tests/reduce.rs `a_script_installed_while_an_old_write_handle_is_open_still_runs` covers both `stand_in` and `Fixture::executable`. It fails first, deterministically, with ETXTBSY.
- Stress: the same 60 rounds at `--test-threads=32` must show no "Text file busy".
- No assertion, gate, bound or timeout changes.
- `fledge lanes run verify` passes on Linux.
