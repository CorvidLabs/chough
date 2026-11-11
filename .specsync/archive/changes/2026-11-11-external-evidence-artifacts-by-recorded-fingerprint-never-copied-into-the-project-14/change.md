---
id: external-evidence-artifacts-by-recorded-fingerprint-never-copied-into-the-project-14
state: archived
type: feature
base_commit: 9f0a2df721821504c0fdae3d43c911c70a63e661
---

# External evidence artifacts by recorded fingerprint, never copied into the project (#14)

## Intent

External evidence artifacts by recorded fingerprint, never copied into the project (#14)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- evidence add and cases record accept an absolute artifact path: the file is fingerprinted (SHA-256) in place within a 64 MiB bound and never copied into the project; missing, symlinked, directory and oversized external paths are refused by name. Project-relative canonical artifact behavior, freshness re-reads, digests and record shape are unchanged; a later change to the external file flips the observation's artifact freshness.

## No-spec Rationale

Not applicable
