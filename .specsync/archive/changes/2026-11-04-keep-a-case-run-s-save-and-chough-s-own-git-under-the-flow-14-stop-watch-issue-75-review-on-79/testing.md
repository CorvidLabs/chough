---
change: keep-a-case-run-s-save-and-chough-s-own-git-under-the-flow-14-stop-watch-issue-75-review-on-79
artifact: testing
---

# Testing

- `tests/stop.rs` `stopping_a_case_run_while_it_saves_stops_git_and_records_nothing` fails at e97633e (the Git stand-in survives and an orphan OBS log is left) and passes now.
- `cargo test --locked` and `fledge lanes run verify` pass.
