---
kind: bug
id: 2026-11-12-a-duplicate-bug-add-prints-a-raw-json-array-and-a-near-miss
aliases: []
title: A duplicate bug add prints a raw JSON array, and a near-miss gate text gets no suggestion
status: verified
classification: product-defect
scope: in-scope
impact: Refusals a person has to decode (t38m F6)
criteria: []
requirements: [REQ-cli-054]
cases:
  - TC-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line
  - TC-cli-near-miss-gate-close-names-the-closest-open-gate
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line-0396f2
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate-660b00
receipts: []
environment: local-macos-arm64
revision: 7fb91195b8f27c97248de42544fd07a89ccc4dbd
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T22:56:49Z
updated: 2026-11-12T23:02:45Z
---
# A duplicate bug add prints a raw JSON array, and a near-miss gate text gets no suggestion

**What happened:** chough bug add for a likely duplicate refuses with 'similar bugs exist: [{"id":…,"score":…}]', a raw JSON array; checkpoint --close-gate with a near-miss gate text refuses as gate-not-open naming no open gate
Actual: A JSON array; no suggestion

**Steps:** Trigger: Add a bug much like an open one; close a gate with one letter wrong
1. chough bug add with a duplicate title
2. chough checkpoint --close-gate 'Release aproval' with the relay

**Expected:** Each similar bug on its own readable line; the refusal names the closest open gate

**Why it matters:** Refusals a person has to decode (t38m F6)

**Fix:** a6c0f346 (t37f's 5ef20a02): a duplicate bug add lists each similar bug as '<id>: <title> (<status>)', and a near-miss --close-gate names the closest open gate

## Log

- 2026-11-12T22:56:49Z · Reported (t38m's third fresh-eyes round report (relayed by t370); fixed by t37f); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate · by claude-code (hc t382)
- 2026-11-12T23:02:45Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-duplicate-bug-add-lists-each-similar-bug-as-a-line-0396f2, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-near-miss-gate-close-names-the-closest-open-gate-660b00; fix: a6c0f346 (t37f's 5ef20a02): a duplicate bug add lists each similar bug as '<id>: <title> (<status>)', and a near-miss --close-gate names the closest open gate. Red at 7fb91195, green at e0131c09 · by claude-code (hc t382)
