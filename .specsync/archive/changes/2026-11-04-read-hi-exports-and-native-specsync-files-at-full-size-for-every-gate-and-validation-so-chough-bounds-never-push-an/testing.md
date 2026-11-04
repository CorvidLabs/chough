---
change: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
artifact: testing
---

# Testing

- `tests/cli.rs` `criteria_validate_against_a_hi_family_export_over_16_kib`: a HI export
  over 64 KiB; `evidence add` and `cases check` accept `LIFE-40`, found only past the
  first 16 KiB, and refuse retired `LIFE-2`; the refusal never names 16384.
- `tests/cli.rs` `an_archived_finalization_over_the_record_bound_is_read`: `evidence prune`
  reports native state observed with a 70 KB `finalization.json`.
- `tests/cli.rs` `an_archived_change_md_over_the_record_bound_is_read`: `library index`
  writes the title and criterion of a 70 KB archived `change.md`.
- `tests/cli.rs` `an_sdd_policy_over_the_record_bound_is_read`: a padded disabled policy
  gives `ungated:sdd-disabled`.
- `tests/cli.rs` `display_truncation_points_to_the_complete_source` and the extended
  `aggregate_context_budget_is_explicit`: markers say display only and never shorten.
- `tests/cli.rs` `the_skill_says_chough_bounds_never_justify_editing_native_sources`.
- `tests/timing.rs` `native_approval_ledgers_over_the_record_bound_still_give_their_times`.
- `tests/cli.rs` `records_other_than_the_checkpoint_keep_the_65536_byte_bound` still
  passes: Chough records keep their bounds.
- Every new test failed with `src/` at main 91a1fdd
  (`.chough/artifacts/issue-100/failure-first-on-main-91a1fdd.log`).
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-042 | `tests/cli.rs`: the six tests above and `aggregate_context_budget_is_explicit` |
| REQ-reporting-004 | `tests/timing.rs`: `native_approval_ledgers_over_the_record_bound_still_give_their_times` |
