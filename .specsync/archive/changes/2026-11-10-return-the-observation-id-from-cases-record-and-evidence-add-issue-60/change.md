---
id: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
state: archived
type: bug_fix
base_commit: 5ff54dd726367321b4e7ede3f931d38657b9b81e
---

# Return the observation ID from cases record and evidence add (issue 60)

## Intent

Return the observation ID from cases record and evidence add (issue 60)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- On base 5ff54dd7, new focused tests fail because cases record omits case and observation and evidence add omits observation. After the fix, cases record returns case equal to the requested case ID and observation equal to both the recorded file stem and stored record id; evidence add returns observation with the same equality. Existing keys, record bytes apart from ordinary ID allocation, exit codes and stored schemas remain unchanged; docs/cli.md names the returned keys, the focused cases and full Fledge verify lane pass locally.

## No-spec Rationale

Not applicable
