---
id: recognize-a-crlf-checkout-of-the-earlier-knowledge-readme-and-treat-an-unmatched-backtick-as-literal-when-reading-trace
state: implementing
type: bug_fix
base_commit: 6b2dfb557fe939b6b2d98e3fd50006d1dae4442e
---

# Recognize a CRLF checkout of the earlier knowledge README and treat an unmatched backtick as literal when reading Trace lists (Windows CI and review of 107)

## Intent

Recognize a CRLF checkout of the earlier knowledge README and treat an unmatched backtick as literal when reading Trace lists (Windows CI and review of 107)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- An untouched earlier knowledge README checked out with CRLF line endings is upgraded like the LF one, so the README test passes on Windows; an edited README is still kept; a requirement line with one unmatched backtick before its Trace list still credits that list. The new test fails first and fledge lanes run verify passes on Linux; hosted Windows passes.

## No-spec Rationale

Bug fix within REQ-cli-044: the README upgrade must hold on every supported platform and every traced criterion is read; no requirement text changes.
