---
change: keep-a-checkpoint-reference-whose-record-exists-but-cannot-be-read-and-refuse-the-closed-cycle-drop-review-of-108
artifact: testing
---

# Testing

tests/reduce.rs `an_unreadable_record_keeps_its_reference_and_refuses_the_drop`: a reset 15 days ago, references r2 and u3, and u3's record replaced with unreadable bytes. Done must be recorded, `checkpoint_references` must be refused naming u3, and both references must stay. It fails first. The earlier drop test, where a missing or old record is dropped, keeps passing. `fledge lanes run verify` runs on Linux.
