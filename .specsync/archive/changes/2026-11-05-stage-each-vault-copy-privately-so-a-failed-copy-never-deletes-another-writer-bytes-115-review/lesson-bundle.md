# Lesson bundle: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Stage each vault copy privately so a failed copy never deletes another writer bytes (115 review)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs
- **Acceptance**: A vault copy is written and given its mode in a private staging file and then linked into place without replacing anything, so a failed write or mode change removes only the staging file and never another writer's bytes at the destination. The new unit test fails first, the evidence and reduce suites pass, and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `e3f6aa448d7e328547da721023d2f43e1b42978f`
- Base commit: `a6cfd3298d70b8b3ce6e2de92a25c817c4cbb324`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif's agent review of #115 at 628f974b (blocking P1, thread on src/evidence.rs:2700), upheld by corvid-agent's re-review at a6cfd329: after `create_new` made a vault copy, a failing `write_all` or `set_permissions` led to an unconditional `remove_file` on the destination. If another writer had replaced the attachment in between, that removed their bytes. The copy was not yet in `copies`, so the rollback could not recover them. This is the same foreign-byte class as #110. It was reproduced with a syscall fault shim on macOS: the shim rewrote the attachment, then `set_permissions` returned EIO. Leif chose "Record and merge" for #115 (2026-11-05 10:18 MST).

## From the change's testing.md

# Testing

src/evidence.rs unit test `a_failed_copy_never_deletes_another_writers_bytes`:
- the final step writes another writer's bytes to the destination, then fails: the error is returned, the other writer's bytes stay, and no staging file is left;
- a final step that fails with nobody else writing leaves nothing behind;
- a finished copy is `Created`; the same bytes already there are `AlreadyThere`; different bytes are refused and kept.

It fails first, because the destination is deleted (NotFound). tests/evidence.rs and tests/reduce.rs, including the #110 rollback and the create-only cases, still pass. `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
