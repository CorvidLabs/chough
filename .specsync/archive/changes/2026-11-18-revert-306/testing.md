---
change: revert-306
artifact: testing
---

# Testing

`tests/no_hard_links.rs::new_records_are_written_on_a_volume_without_hard_links`
makes and mounts a 64 MB exFAT image with `hdiutil`, checks that the volume
refuses a hard link, and runs `chough init`, `bug add` and `journal add` on it.
It fails on main (be6c9f1 and later) with "Operation not supported (os error
45)" and passes with the revert. It is macOS only and ignored by default,
because `hdiutil` can hang and does not exist on Linux CI; it is run by hand
with `-- --ignored` and that run is recorded as evidence.

The #260 test and unit test that #306 added are removed with the writer they
tested. The parallel temporary-file sweep has no test in this change; the
tester's repro script checks it.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-001 | the exFAT test above, run by hand and recorded with `chough evidence add` |
