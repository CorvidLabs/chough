---
change: refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71
artifact: testing
---

# Testing

- `tests/stages.rs` `checkpoint_refuses_a_native_reply_that_names_no_change`: a SpecSync
  stand-in answers `empty-object` with `{}`, `empty-array` with `[]` and `no-ids` with a
  summary that has a state but no `id`. Each refuses as `refused:native-change-unknown`,
  naming the ID, with the checkpoint byte-identical alone and with a `--decision` in the
  same call. The shared stub still names the requested ID, so a known ID saves.
- Failing first: the new test fails on the branch before the fix ("unexpectedly
  succeeded").
- `fledge lanes run verify` passes.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-007 | `tests/stages.rs`: `checkpoint_refuses_a_native_reply_that_names_no_change`, `checkpoint_refuses_a_native_reply_that_names_a_different_change` and `checkpoint_refuses_a_native_change_specsync_does_not_know`. |
