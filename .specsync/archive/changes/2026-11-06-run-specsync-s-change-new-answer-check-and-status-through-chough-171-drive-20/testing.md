---
change: run-specsync-s-change-new-answer-check-and-status-through-chough-171-drive-20
artifact: testing
---

# Testing

New tests in `tests/change.rs`, one or more per REQ-cli-057 criterion. Each uses a stub `specsync` on PATH that records its argv and working directory, then prints known bytes to stdout and stderr and exits with a chosen code:

- `change_new_answer_check_and_status_run_specsync_with_exact_arguments`: exact argv and working directory for all four.
- `specsync_output_and_exit_code_pass_through_byte_for_byte`: stdout, stderr and exit code, including a SpecSync refusal (exit 1) and a nonstandard exit (3).
- `no_change_command_writes_under_dot_chough`: the bytes of every file under `.chough/` are unchanged.
- `check_outlives_the_native_bound_while_status_refuses_past_it`: with `CHOUGH_NATIVE_TIMEOUT_SECONDS=1` and a stub that sleeps 2 s, check exits 0 and status refuses; the retry it names, with a longer bound, exits 0.
- `a_missing_specsync_refuses_naming_install_and_the_step_to_rerun`: the refusal names `cargo install --locked specsync@6.2.0` and the step; putting the stub on PATH and running that step exits 0.
- `sigterm_to_change_check_stops_specsync`: the stub's process is gone after Chough is stopped.

They fail first because `chough change` does not exist.

Mutation round, each mutant failing at least one new test: drop `--commit`; drop `--json` from status; swap the stdout and stderr copies; return 0 instead of SpecSync's exit code; apply the native bound to check; skip the bound for status; omit the working directory.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-057 | tests/change.rs, the six tests above; cases in `.chough/cases/cli.change-steps.cases.toml` |

## Lanes

`fledge lanes run verify` passes on Linux; the change's tests pass on macOS.
