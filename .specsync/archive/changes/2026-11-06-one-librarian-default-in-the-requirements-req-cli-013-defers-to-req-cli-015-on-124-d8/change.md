---
id: one-librarian-default-in-the-requirements-req-cli-013-defers-to-req-cli-015-on-124-d8
state: archived
type: bug_fix
base_commit: 8e0ab4392279cbbb1a4db08044305c450ae641b7
---

# One librarian default in the requirements: REQ-cli-013 defers to REQ-cli-015, on (124, D8)

## Intent

One librarian default in the requirements: REQ-cli-013 defers to REQ-cli-015, on (124, D8)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- REQ-cli-013 no longer gives the librarian an off default and defers to REQ-cli-015 (on); no requirement sentence names the librarian, a default and off without saying it is on by default; chough pref show still reports librarian on with origin default; the runtime is unchanged. The new test fails first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
