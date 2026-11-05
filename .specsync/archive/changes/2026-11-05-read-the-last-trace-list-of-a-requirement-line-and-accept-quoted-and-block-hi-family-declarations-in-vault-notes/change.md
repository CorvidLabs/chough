---
id: read-the-last-trace-list-of-a-requirement-line-and-accept-quoted-and-block-hi-family-declarations-in-vault-notes
state: archived
type: bug_fix
base_commit: 94ebefc04e6910f066800cc34698a6c2b3a101bd
---

# Read the last Trace list of a requirement line and accept quoted and block HI family declarations in vault notes (reviews of 107)

## Intent

Read the last Trace list of a requirement line and accept quoted and block HI family declarations in vault notes (reviews of 107)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A requirement that mentions Trace: inline before its real Trace list is credited from the real list, so this repository REQ-cli-044 credits KNOW-14 and KNOW-15; HI files declaring families quoted or as a YAML block list link to their intent notes; the new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-044: the vault must credit the criteria each requirement traces and link every HI file; no requirement text changes.
