---
change: fix-325
artifact: testing
---

# Testing

`cargo test --locked --test evidence a_busy_evidence_add_writes_nothing_and_refuses_with_nothing_saved` holds the project's writer lock from the test and starts `evidence add`.
- After a second, the command must still be waiting with no entry written. On main the entry is already there.
- After the 30-second wait, the command must refuse as `refused:checkpoint-busy` with every file unchanged.
- Once the lock is free, a rerun records exactly one entry.

Case `TC-cli-a-busy-evidence-add-leaves-nothing` binds it.
