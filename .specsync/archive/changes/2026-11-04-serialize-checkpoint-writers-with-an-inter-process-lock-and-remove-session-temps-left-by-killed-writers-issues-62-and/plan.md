---
change: serialize-checkpoint-writers-with-an-inter-process-lock-and-remove-session-temps-left-by-killed-writers-issues-62-and
artifact: plan
---

# Plan

## Approach

Hold one inter-process lock across every checkpoint read-modify-write, and clean up
session temps only while holding it.

## Out of scope

Temps of other managed records (preferences, bugs), merge-driver temps, record identity
across parallel agents (#30), an MSRV change, and Chough binaries older than this change
writing at the same time (they do not take the lock).

## Steps

1. Failing-first tests in tests/cli.rs (parallel writers, killed before and after the
   rename, orphan-only cleanup) with a lock-holder helper process.
2. Lock and cleanup in src/checkpoint.rs; fs4 dependency; unlocked reads in
   src/validate.rs.
3. docs/cli.md and CHANGELOG.md; fledge verify.

## Risks

A command that holds the lock for a long native read (up to the native timeout) makes a
concurrent writer wait, then refuse after 30 seconds rather than lose an update. One new
dependency package. The lock file sits in the system temp directory, so two processes
with different `TMPDIR` values, or in different containers sharing one clone, do not
exclude each other. One empty file per project accumulates in the temp directory.

## Constraints consulted

MSRV 1.88 (D-019); lifecycle records stay with their owners; refusals save nothing
(REQ-cli-032 writer regressions keep passing unchanged).
