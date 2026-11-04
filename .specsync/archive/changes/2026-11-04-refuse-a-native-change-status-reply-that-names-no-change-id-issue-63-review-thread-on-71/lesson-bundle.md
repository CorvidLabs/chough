# Lesson bundle: refuse-a-native-change-status-reply-that-names-no-change-id-issue-63-review-thread-on-71

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Refuse a native change status reply that names no change ID (issue 63, review thread on #71)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: CHANGELOG.md, docs/cli.md, src/checkpoint.rs, tests/stages.rs
- **Acceptance**: With SpecSync configured chough checkpoint --native-change refuses as refused:native-change-unknown when the status reply is JSON that names neither summary.id nor change.id (for example [] or {}) and .chough/session.json stays byte-identical even with other updates in the same call; a reply that names the requested ID still saves; a reply naming another ID still refuses as refused:native-change-mismatch; docs/cli.md states it; the new test fails on the branch before the fix and fledge lanes run verify passes

## Evidence

- Verification commit: `94fe794a7d2998a5681aeb89e5d2dc863ed0865e`
- Base commit: `efa397da2f4c196d5e5120e87b7125dd25a106c1`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Review thread on #71 (P2, src/checkpoint.rs, 2026-11-04 01:17 UTC): a successful
`specsync change status` reply only proves exit 0 with parseable JSON. Stand-ins that
returned `[]`, `{}` or another change's summary all saved `made-up`. e3ff647 refuses a
reply naming another change (`refused:native-change-mismatch`), but a reply that names
no change ID still saved; the Chough bot's review 5762088554 noted that as well. The
thread asks for the reported change ID to be required, with missing-ID and wrong-ID
regression tests.

The earlier change for #63 is archived on this branch, so this follow-up is its own
native change on the same pull request. SpecSync 6.1.2 always names `summary.id` and
`change.id`, so real replies are unaffected.

Ruled out: changing stage gates, `next` or `status` native reads.

## From the change's testing.md

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

## Where these lessons go

- `specs/cli/context.md`
