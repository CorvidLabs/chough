---
change: keep-trimming-when-an-earlier-removal-releases-a-shared-artifact-to-another-eligible-set-and-count-stub-status-calls
artifact: context
---

# Context

#108's closeout ran the move into done on Chough's own repository. It dropped 385 closed-cycle checkpoint references (KNOW-16), and then the trim was refused partway: "protections changed during removal (OBS-product-test-4061...: its files changed)". It restored the 21 files it had removed (KNOW-10 restore).

The cause is in `still_unprotected`, the re-check before every file removal. It required the set's current files to be a subset of the planned files. When two eligible sets share one artifact, neither plan includes that artifact. Removing the first set's record leaves the artifact to the second set alone, so the second set now lists one extra file, and the re-check refused.

A set that gains a file is still safe to trim, because only planned files are removed. The re-check now requires every planned file that is still present to belong to the set alone, with the same record. It still refuses when a planned file becomes shared or the set becomes protected. No requirement text changes.

The same parallel validity reads exposed a race in tests/evidence.rs. The knowledge fixture's stub `specsync` counts status calls in a file to time its hooks. Two concurrent reads could read the same count, so `archive_summary_apply_fails_closed_when_a_file_changes_during_the_native_status_check` and `archive_summary_restore_keeps_a_file_recreated_during_the_apply` fired their hooks at the wrong time, or not at all. This was also the intermittent verify failure seen during #108. The stub now counts under an atomic `mkdir` lock, so both tests time their hooks exactly. Their assertions are unchanged.
