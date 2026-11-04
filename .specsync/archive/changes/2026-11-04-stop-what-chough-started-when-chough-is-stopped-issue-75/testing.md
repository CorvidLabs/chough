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
- `tests/stop.rs` `stopping_a_case_run_while_it_saves_stops_git_and_records_nothing`
  (review on #79): the bound command exits at once, then every `git` call stalls in a
  stand-in that runs the real Git afterwards. For each of SIGINT, SIGTERM and SIGHUP
  sent during that stall, Chough ends by the signal, the stalled Git is gone and
  `.chough` is byte-identical. At e97633e it fails twice over: the Git stand-in
  survives, and a `.chough/artifacts/OBS-*.log` is left with no observation.
- `tests/stop.rs` `a_stop_signal_inherited_as_blocked_stays_blocked`: with SIGTERM
  blocked when Chough starts, a SIGTERM leaves the case running and the run records
  its observation as before.
- Each original test fails with `src/` at main edbc3c1, because the command survives.
- Existing deadline and held-output tests in tests/cases.rs pass unchanged.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-011 | `tests/stop.rs`: `stopping_a_case_run_stops_its_command_and_records_nothing`, `stopping_a_native_read_stops_specsync_and_records_nothing`, `stopping_a_case_run_while_it_saves_stops_git_and_records_nothing`, `a_stop_signal_inherited_as_blocked_stays_blocked`. |
