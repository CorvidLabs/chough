---
id: fix-301
state: draft
type: feature
base_commit: 0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7
---

# Fix 301

## Intent

fix 301

## Affected Canonical Specs

- None

## Acceptance Criteria

- docs/cli.md documents the checkpoint command's list-valued flags (--criterion, --requirement, --case, --design, --file, --observation, --bug) and chough checkpoint --help prints help text for each of them

## No-spec Rationale

Docs and help-text only: adds clap help text to the list-valued checkpoint flags and updates docs/cli.md; no canonical spec, requirement, or behavior changes.
