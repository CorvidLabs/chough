---
kind: bug
id: 2026-11-12-a-failed-case-run-s-body-does-not-say-why-it-failed
aliases: []
title: A failed case run's body does not say why it failed
status: verified
classification: product-defect
scope: in-scope
impact: A reader cannot tell why the run failed (t38m F10c)
criteria: []
requirements: [REQ-cli-017]
cases: [TC-cli-failed-run-body-says-why]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-failed-run-body-says-why
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-failed-run-body-says-why-ed1722
receipts: []
environment: local-macos-arm64
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:05:01Z
updated: 2026-11-12T23:06:39Z
---
# A failed case run's body does not say why it failed

**What happened:** A failed case's evidence body says 'failed (exit 0)' and 'Key failing lines: - 2', while the reason, expected output missing: "3", is only in its front-matter limitations
Actual: Only 'failed (exit 0)' and unexplained failing lines

**Steps:** Trigger: Run a case whose expected output is missing from its output
1. chough cases run <case> where the expected text does not appear
2. Read the evidence entry's body

**Expected:** The body says the expected output was missing, quoting it, before the key failing lines

**Why it matters:** A reader cannot tell why the run failed (t38m F10c)

**Fix:** 9f67dffe (t38b's f62cd586): a failed entry's body says why it failed, such as the missing expected output, before its key failing lines (REQ-cli-017)

## Log

- 2026-11-12T23:05:01Z · Reported (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-failed-run-body-says-why · by claude-code (hc t382)
- 2026-11-12T23:06:28Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-failed-run-body-says-why-ed1722; fix: t38b's f62cd586: a failed entry's body says why it failed, such as the missing expected output, before its key failing lines (REQ-cli-017). Red at 4e245263, green at 421a7034 · by claude-code (hc t382)
- 2026-11-12T23:06:39Z · fix: 9f67dffe (t38b's f62cd586): a failed entry's body says why it failed, such as the missing expected output, before its key failing lines (REQ-cli-017). Fix line names the commits on leif/human-first-records · by claude-code (hc t382)
