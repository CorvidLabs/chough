---
change: stop-what-chough-started-when-chough-is-stopped-issue-75
artifact: testing
---

# Testing

- `tests/stop.rs` `stopping_a_case_run_stops_its_command_and_records_nothing`: for each of
  SIGINT, SIGTERM and SIGHUP, a `command` case starts `sh` with a background `sleep`;
  once it is running, the signal is sent to `chough cases run`. Chough ends by that
  signal, no process of the case survives, and the `.chough` directory (session,
  evidence, artifacts) is byte-identical to before. Then, with SIGHUP ignored by the
  parent (`trap '' HUP`), SIGHUP leaves the case running and the run records its
  observation as before, so an inherited ignored signal stays ignored.
- `tests/stop.rs` `stopping_a_native_read_stops_specsync_and_records_nothing`: the same
  for a stub `specsync` started by `chough next --json`.
- Each new test fails with `src/` at main edbc3c1, because the command survives.
- Existing deadline and held-output tests in tests/cases.rs pass unchanged.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-011 | `tests/stop.rs`: `stopping_a_case_run_stops_its_command_and_records_nothing`, `stopping_a_native_read_stops_specsync_and_records_nothing`. |
