# Lesson bundle: keep-vault-copies-another-writer-changed-when-a-refused-trim-rolls-back-110

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep vault copies another writer changed when a refused trim rolls back (110)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/evidence.rs, tests/evidence.rs
- **Acceptance**: A refused trim apply removes only the vault copies it created that still hold the bytes and mode it wrote; a copy another writer changed is kept as written and named in the refusal; the source is restored either way. The new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `217cdbd5fc97a2d7ed58d355217e9d74ac540838`
- Base commit: `99c02a57e1279237e6e2f80099d8bab1d6074dc4`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

Issue #110 (P1, reproduced by Leif's review agent on merged main 2587fe0): when a trim's apply is refused, its rollback deletes every vault copy the apply created, by path only (`copies` holds path strings). If another writer changed one of those attachments after the apply created it, the rollback deletes that writer's bytes. Restoring the original `.chough/` source doesn't recover them. Leif's decision through the orc (2026-11-05 09:43 MST): Chough 0.4.0 is released after #112, #114 and #110, and this issue comes next in order.

## From the change's testing.md

# Testing

tests/evidence.rs `a_refused_apply_keeps_vault_copies_another_writer_changed`, built on `knowledge_fixture()`. The manual record of cold a2 moves into the vault, and a SpecSync stand-in makes validity change once it is gone, so the apply is refused. During the post-removal validity read (`.stub-hook-after` 3), a hook does one of three things:
1. rewrites the attachment: the other writer's bytes are kept and named in the refusal;
2. changes the attachment's mode to 0600: the copy is kept with that mode and named;
3. does nothing: the unchanged copy is removed.

In every case the source record is restored. The test fails first, because the copy changed in case 1 is deleted. The identical-attachment and create-only cases in tests/reduce.rs still pass. `fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
