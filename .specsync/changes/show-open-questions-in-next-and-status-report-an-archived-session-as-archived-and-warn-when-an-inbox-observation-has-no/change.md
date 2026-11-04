---
id: show-open-questions-in-next-and-status-report-an-archived-session-as-archived-and-warn-when-an-inbox-observation-has-no
state: verifying
type: bug_fix
base_commit: 290cce3a024425cfd35600a43b8e458beaa89436
---

# Show open questions in next and status, report an archived session as archived, and warn when an inbox observation has no record (issue 84)

## Intent

Show open questions in next and status, report an archived session as archived, and warn when an inbox observation has no record (issue 84)

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- chough next shows the checkpoint open questions in text and JSON bounded to 10 with their total and chough status reports them,A question folded from a session inbox appears in next,session list reports an archived session with status archived,inbox add of an observation entry whose ID has no evidence record still adds it and warns on stderr naming the ID and inbox list marks it recorded false,Each new test fails on 290cce3 and fledge lanes run verify passes on Linux

## No-spec Rationale

Not applicable
