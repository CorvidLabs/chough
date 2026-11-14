---
id: this-repository-routes-spec-files-to-specsync-s-merge-driver-291
state: draft
type: feature
base_commit: aa1c5a15815d61f281f45e742b45fadc5bf55828
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
