---
change: keep-trimming-a-loose-artifact-group-that-gains-a-file-released-by-an-earlier-removal-109-closeout
artifact: testing
---

# Testing

tests/reduce.rs `a_loose_folder_that_gains_a_released_file_is_still_trimmed`: a folder whose a-shared.log is referenced by two eligible runs, and whose z-loose.log is loose and listed by cold a1. The trim must succeed, remove z-loose.log and keep a-shared.log. It fails first with the same refusal. tests/reduce.rs `a_loose_file_protected_during_the_apply_refuses_the_trim` guards the other side: an open bug that names a planned loose file partway through the apply refuses the trim and restores both files. `fledge lanes run verify` runs on Linux, and the trim then runs on this repository at closeout.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-046 | tests/reduce.rs `a_loose_folder_that_gains_a_released_file_is_still_trimmed`, `a_loose_file_protected_during_the_apply_refuses_the_trim` |
