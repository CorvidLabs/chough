---
change: cli-cleanups-for-issue-49-hyphen-values-in-bug-add-no-repeated-native-next-action-in-the-next-main-line-iso-times-in
artifact: testing
---

# Testing

- `tests/bugs.rs`: a title and symptom starting with `--` are recorded verbatim, and every `bug add --help` option has a description.
- `tests/cases.rs`: every `cases run --help` option has a description.
- `tests/stages.rs`, with a long native next action that names the status command:
  - the advance refusal names `specsync change status <id>` once;
  - the `next` main line is not cut and does not repeat the native next action;
  - the native next action and the gate detail still appear.
- `tests/stages.rs`: the `next` text shows the last failure at `2026-11-02T…Z` for a pinned recorded time, and the JSON keeps the seconds.
- Exercise: run `fledge lanes run verify` (which runs the exercise) and count `chough-exercise.*` directories before and after. Record this as a human-observation with a small log.
- Each new test is checked against the old behavior: it fails on `main`.
