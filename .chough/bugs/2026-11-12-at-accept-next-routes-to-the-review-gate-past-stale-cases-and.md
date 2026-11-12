---
kind: bug
id: 2026-11-12-at-accept-next-routes-to-the-review-gate-past-stale-cases-and
aliases: []
title: At accept, next routes to the review gate past stale cases, and accept.md's order stales the evidence twice
status: verified
classification: product-defect
scope: in-scope
impact: The review is presented on stale evidence, or the cases are rerun needlessly (t38n F4)
criteria: []
requirements: [REQ-cli-021]
cases:
  - TC-cli-accept-next-reruns-cases-not-current
  - TC-cli-accept-reference-lessons-before-case-runs
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-next-reruns-cases-not-current
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-reference-lessons-before-case-runs
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-next-reruns-cases-not-current-d49346
  - 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-reference-lessons-before-case-runs-bf5029
receipts: []
environment: local-macos-arm64
revision: 0c44e742c25af1a4bd717991b53b49f49342f300
found_by: claude-code (hc t382) (t38n's fourth fresh-eyes round report, F4 (relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:54:34Z
updated: 2026-11-12T23:55:31Z
---
# At accept, next routes to the review gate past stale cases, and accept.md's order stales the evidence twice

**What happened:** After chough change check --no-commit materializes specs/, every observation reads historical, yet chough next routes straight to opening the review gate while listing them as not current; and accept.md's Compounding step writes the lesson during closeout, which stales the cases a second time
Actual: next names the review gate; the cases are run three times

**Steps:** Trigger: Follow accept.md: the materializing check, the cases, then the lesson
1. chough change check <id> --no-commit
2. chough next
3. Write the lesson to specs/<module>/context.md after the case runs

**Expected:** next names rerunning the stale cases before the review gate; accept.md writes lessons before the case runs

**Why it matters:** The review is presented on stale evidence, or the cases are rerun needlessly (t38n F4)

**Fix:** 648ebb94 (t38b's 44e9c7b7): at accept next names rerunning a case whose latest run is not current before the review gate, and accept.md has any lesson written before the case runs (REQ-cli-021)

## Log

- 2026-11-12T23:54:34Z · Reported (t38n's fourth fresh-eyes round report, F4 (relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-next-reruns-cases-not-current, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-reference-lessons-before-case-runs · by claude-code (hc t382)
- 2026-11-12T23:54:34Z · Added despite similar reports: The verified accept bugs covered next's review preparation while approved and the review command; this one is next routing past cases whose runs went stale after the materializing check, and accept.md's order that stales them twice · by claude-code (hc t382)
- 2026-11-12T23:55:31Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-next-reruns-cases-not-current-d49346, 2026-11-12-release-0-6-0/2026-11-12-tc-cli-accept-reference-lessons-before-case-runs-bf5029; fix: 648ebb94 (t38b's 44e9c7b7): at accept next names rerunning a case whose latest run is not current before the review gate, and accept.md has any lesson written before the case runs (REQ-cli-021). Red at 0c44e742, green at 648ebb94 · by claude-code (hc t382)
