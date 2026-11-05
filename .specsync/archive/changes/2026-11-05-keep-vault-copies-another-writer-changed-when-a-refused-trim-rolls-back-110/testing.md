---
change: keep-vault-copies-another-writer-changed-when-a-refused-trim-rolls-back-110
artifact: testing
---

# Testing

tests/evidence.rs `a_refused_apply_keeps_vault_copies_another_writer_changed`, built on `knowledge_fixture()`. The manual record of cold a2 moves into the vault, and a SpecSync stand-in makes validity change once it is gone, so the apply is refused. During the post-removal validity read (`.stub-hook-after` 3), a hook does one of three things:
1. rewrites the attachment: the other writer's bytes are kept and named in the refusal;
2. changes the attachment's mode to 0600: the copy is kept with that mode and named;
3. does nothing: the unchanged copy is removed.

In every case the source record is restored. The test fails first, because the copy changed in case 1 is deleted. The identical-attachment and create-only cases in tests/reduce.rs still pass. `fledge lanes run verify` passes on Linux.
