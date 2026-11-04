# Lesson bundle: refuse-a-native-change-id-that-specsync-does-not-know-issue-63

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Refuse a native change ID that SpecSync does not know (issue 63)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/checkpoint.rs, tests/stages.rs, docs/cli.md, CHANGELOG.md, tests/common/mod.rs
- **Acceptance**: With SpecSync configured chough checkpoint --native-change for an ID that specsync change status does not report exits non-zero with refused:native-change-unknown naming the ID and the status command and .chough/session.json stays byte-identical even when other updates are in the same call, A failed or timed-out SpecSync read also refuses with nothing saved, An ID that SpecSync reports is saved as before, Without a SpecSync configuration only the existing ID format check applies, docs/cli.md states the refusal and the new test fails on main 4daf861 and fledge lanes run verify passes

## Evidence

- Verification commit: `e3ff647f2fdca1fd345ff459ddab7d491b64a82b`
- Base commit: `4daf861509b70c14282ac16b5d0547d701e385b2`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Kimi's second lab review (2026-11-03, Chough 7bd1b62, SpecSync 6.1.2, round 2 item 2)
ran `chough checkpoint --native-change does-not-exist`: exit 0, `saved: true`, and the
checkpoint held the bad ID. Later reads then failed from that bad state (`next`
refused as `refused:native-unknown`, `status` showed the native read as failed).
Reproduced on main 4daf861 and filed as https://github.com/CorvidLabs/chough/issues/63.

`checkpoint_command` only checked the ID's format (`valid_change_id`). SpecSync 6.1.2
exits 1 with the error "change `<id>` was not found" for an unknown change.

Ruled out: changing what stage gates, `next` or `status` do with native state; clearing
a recorded change (unchanged); projects without SpecSync (nothing to ask, unchanged).
Related round-2 findings filed separately: #62 (concurrent writers) and #64 (orphaned
temp files).

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
