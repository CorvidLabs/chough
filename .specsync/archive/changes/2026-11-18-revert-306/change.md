---
id: revert-306
state: archived
type: bug_fix
base_commit: 66543c2bf509627ec2a165414acae0bc8ab59c4a
---

# Revert 306

## Intent

revert 306

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- On an exFAT volume chough init, chough bug add and chough journal add succeed and write whole records, as in 0.6.0. The create-new writer in src/records.rs is the one 0.6.0 shipped. #305's non-blocking reader and #308's journal lock are unchanged.

## No-spec Rationale

No requirement text changes. #306 changed how a new record is written (a temporary file linked to the name). That writer fails on file systems without hard links (#313) and leaves its temporary files where a concurrent writer sweeps them. The create-new writer goes back to what 0.6.0 shipped; #260 reopens for 0.6.2.
