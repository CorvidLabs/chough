---
id: no-conflicting-files-per-change-case-files-a-knowledge-rebuild-check-and-a-bug-merge-driver-drive-12
state: implementing
type: feature
base_commit: eb35dd3a1d9ca5ce12dee7fd50e7224432241fee
---

# No conflicting files: per-change case files, a knowledge rebuild check and a bug merge driver (DRIVE-12)

## Intent

No conflicting files: per-change case files, a knowledge rebuild check and a bug merge driver (DRIVE-12)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Parallel pull requests that each add their own case file, edit one bug and rebuild the vault merge without hand-resolved conflicts in files Chough writes: case files per change load together with IDs unique; library index --check fails naming stale or missing generated files and passes after library index (line endings ignored), and the verify and ci lanes run it; chough bug merge merges two edits of one bug and leaves a real disagreement as a conflict with ours unchanged. The new tests fail first, a parallel pull request simulation merges clean, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
