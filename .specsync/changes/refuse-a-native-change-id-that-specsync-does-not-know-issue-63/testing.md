---
change: refuse-a-native-change-id-that-specsync-does-not-know-issue-63
artifact: testing
---

# Testing

- `tests/stages.rs` `checkpoint_refuses_a_native_change_specsync_does_not_know`: a
  SpecSync stand-in exits 1 with "not found" for `does-not-exist`. The refusal names
  `refused:native-change-unknown`, the ID and `specsync change status does-not-exist`.
  The checkpoint stays byte-identical alone and with a `--decision` in the same call.
  A failing status read (`.stub-fail`) also refuses with nothing saved, and a known ID
  then saves.
- `tests/stages.rs` `checkpoint_native_change_without_specsync_keeps_the_format_check`:
  with no SpecSync configuration any well-formed ID saves and a malformed one still fails.
- `tests/stages.rs` `checkpoint_refuses_a_native_reply_that_names_a_different_change`:
  a stand-in answers `alias-of-add-thing` with `summary.id` and `change.id`
  `add-thing`, and `split-reply` with `summary.id` matching but `change.id`
  `add-thing`. Both refuse as `refused:native-change-mismatch`, naming both IDs, with
  the checkpoint byte-identical. A reply naming no change is left to the stage gates,
  as before. The shared SpecSync stub now echoes `summary.id`, as SpecSync 6.1.2 does,
  so every other native test exercises the matching path. It failed before the check
  ("unexpectedly succeeded").
- Checked against the old behavior: with `src/` at main 4daf861 the first test fails
  ("unexpectedly succeeded"); the second passes on both.
- Real SpecSync 6.1.2 in an `examples/feature-bug-loop.sh` sample: `does-not-exist`
  is refused with SpecSync's own "not found" error and the session hash is unchanged;
  the sample's real change still saves.
- `cargo test --locked` and `fledge lanes run verify` pass.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-007 | `tests/stages.rs`: `checkpoint_refuses_a_native_change_specsync_does_not_know`, `checkpoint_refuses_a_native_reply_that_names_a_different_change` and `checkpoint_native_change_without_specsync_keeps_the_format_check`. |
