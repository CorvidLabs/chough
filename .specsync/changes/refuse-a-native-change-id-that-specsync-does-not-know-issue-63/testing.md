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
- Checked against the old behavior: with `src/` at main 4daf861 the first test fails
  ("unexpectedly succeeded"); the second passes on both.
- Real SpecSync 6.1.2 in an `examples/feature-bug-loop.sh` sample: `does-not-exist`
  is refused with SpecSync's own "not found" error and the session hash is unchanged;
  the sample's real change still saves.
- `cargo test --locked` and `fledge lanes run verify` pass.
