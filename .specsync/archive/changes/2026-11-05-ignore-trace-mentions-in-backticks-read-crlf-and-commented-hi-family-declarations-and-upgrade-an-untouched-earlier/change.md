---
id: ignore-trace-mentions-in-backticks-read-crlf-and-commented-hi-family-declarations-and-upgrade-an-untouched-earlier
state: archived
type: bug_fix
base_commit: 5326ec48b4ae856543e4d8cabd7c06b29daef0e5
---

# Ignore Trace mentions in backticks, read CRLF and commented HI family declarations, and upgrade an untouched earlier knowledge README (review of 107 at 5326ec4)

## Intent

Ignore Trace mentions in backticks, read CRLF and commented HI family declarations, and upgrade an untouched earlier knowledge README (review of 107 at 5326ec4)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A Trace list is read whether an inline backticked Trace mention comes before or after it; HI files with CRLF line endings or a YAML comment after the families list give their family; a knowledge README byte-identical to the template generated at 8fbbbe7 is upgraded to the current template by library index and init, and an edited README is left byte-identical. The new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Bug fix within REQ-cli-044: every generated note has front matter and every traced criterion and HI family is read; no requirement text changes.
