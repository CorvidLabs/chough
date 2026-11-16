---
id: this-repository-routes-spec-files-to-specsync-s-merge-driver-291
state: archived
type: feature
base_commit: 2b50b058ed10c8e02ec8fcd2223aeea98908785d
---

# This repository routes spec files to SpecSync's merge driver (#291)

## Intent

This repository routes spec files to SpecSync's merge driver (#291)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- This repository's .gitattributes routes specs/**/*.spec.md and specs/**/requirements.md to merge=specsync, AGENTS.md names specsync merge --install, and two branches that each bump one module's version and Change Log merge locally without conflict markers

## No-spec Rationale

Repository merge configuration and its setup note; no requirement text or product behavior changes.
