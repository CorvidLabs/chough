---
id: let-automated-case-bindings-record-artifacts-outside-the-project-with-a-recorded-fingerprint-14
state: draft
type: feature
base_commit: 4e7bd5cbba1175f17e69752f317def5a49ac001f
---

# Let automated case bindings record artifacts outside the project with a recorded fingerprint (#14)

## Intent

Let automated case bindings record artifacts outside the project with a recorded fingerprint (#14)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A command or agent-assisted case MAY declare an artifact as an absolute or home-relative path outside the project; the run copies or links nothing into the repository, records the artifact's path, size and SHA-256 fingerprint in the observation, and refuses when the path is missing or unreadable. Project-relative artifact behavior, artifact caps and digests are unchanged. An external artifact's later change is detectable from the recorded fingerprint.

## No-spec Rationale

Not applicable
