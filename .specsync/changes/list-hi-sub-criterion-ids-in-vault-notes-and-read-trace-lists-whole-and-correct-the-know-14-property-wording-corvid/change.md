---
id: list-hi-sub-criterion-ids-in-vault-notes-and-read-trace-lists-whole-and-correct-the-know-14-property-wording-corvid
state: implementing
type: bug_fix
base_commit: 16dc3294fc8e91a3f6a116914412fe8714a28ddd
---

# List HI sub-criterion IDs in vault notes and read Trace lists whole, and correct the KNOW-14 property wording (corvid-agent review of 107)

## Intent

List HI sub-criterion IDs in vault notes and read Trace lists whole, and correct the KNOW-14 property wording (corvid-agent review of 107)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Vault intent notes list HI sub-criterion IDs such as UX-1.a, a Trace list such as UX-1.a, UX-2. credits both IDs and never UX-1, the archive note front matter lists both, and KNOW-14 says every note has at least id, title, type and tags while an archived-change note also gives status, dates, modules, criteria, requirements and commits. The new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-044: the vault must list each criterion ID, including HI sub-criterion IDs; no requirement text changes.
