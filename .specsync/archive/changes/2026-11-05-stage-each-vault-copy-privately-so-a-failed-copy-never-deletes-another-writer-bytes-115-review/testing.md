---
change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
artifact: testing
---

# Testing

src/evidence.rs unit test `a_failed_copy_never_deletes_another_writers_bytes`:
- the final step writes another writer's bytes to the destination, then fails: the error is returned, the other writer's bytes stay, and no staging file is left;
- a final step that fails with nobody else writing leaves nothing behind;
- a finished copy is `Created`; the same bytes already there are `AlreadyThere`; different bytes are refused and kept.

It fails first, because the destination is deleted (NotFound). tests/evidence.rs and tests/reduce.rs, including the #110 rollback and the create-only cases, still pass. `fledge lanes run verify` passes on Linux.
