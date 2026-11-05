---
change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
artifact: tasks
---

# Tasks

- [x] Extract the copy step into `place_copy`, with its final step injectable, and add the failing unit test: another writer replaces the destination while setting the mode fails.
- [x] Write and finish the copy in a private staging file beside the destination, then link it into place without replacing anything. On any failure, remove only the staging file.
- [x] Run the evidence and reduce suites and the verify lane on Linux.
