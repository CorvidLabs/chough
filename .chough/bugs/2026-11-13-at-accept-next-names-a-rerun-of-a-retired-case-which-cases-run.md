---
kind: bug
id: 2026-11-13-at-accept-next-names-a-rerun-of-a-retired-case-which-cases-run
aliases: []
title: At accept, next names a rerun of a retired case, which cases run refuses
status: verified
classification: product-defect
scope: in-scope
impact: next blocks the review on an impossible step (round-4 F4's rerun list)
criteria: []
requirements: [REQ-cli-021]
cases: [TC-cli-accept-next-reruns-no-retired-case]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-13-tc-cli-accept-next-reruns-no-retired-case
  - 2026-11-12-release-0-6-0/2026-11-13-tc-cli-accept-next-reruns-no-retired-case-ba5a0a
receipts: []
environment: local-macos-arm64
revision: fc14a104e2c76a2a133cf3e3ef932f6b1f318509
found_by: "claude-code (hc t382) (Found by t382 preparing Leif's review after round 4: next named reruns of two retired cases)"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T00:29:13Z
updated: 2026-11-13T00:32:57Z
---
# At accept, next names a rerun of a retired case, which cases run refuses

**What happened:** At accept, chough next's rerun list named chough cases run for two referenced cases retired after their runs; chough cases run refuses a retired case, so the step is a dead end before the review gate
Actual: A rerun of the retired case is the first step

**Steps:** Trigger: Reference a case, retire it, then reach accept
1. chough checkpoint --case <case> after a run
2. Set the case's status to retired
3. chough next at accept

**Expected:** No rerun named for a retired case

**Why it matters:** next blocks the review on an impossible step (round-4 F4's rerun list)

**Fix:** 17d60d51: at accept, next names no rerun of a referenced case that is retired or no longer defined (REQ-cli-021)

## Log

- 2026-11-13T00:29:13Z · Reported (Found by t382 preparing Leif's review after round 4: next named reruns of two retired cases); evidence: 2026-11-12-release-0-6-0/2026-11-13-tc-cli-accept-next-reruns-no-retired-case · by claude-code (hc t382)
- 2026-11-13T00:29:13Z · Added despite similar reports: The verified F4 bug made next name stale cases' reruns before the review gate; this one is that list naming retired cases, which cannot be run · by claude-code (hc t382)
- 2026-11-13T00:32:57Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-13-tc-cli-accept-next-reruns-no-retired-case-ba5a0a; fix: 17d60d51: at accept, next names no rerun of a referenced case that is retired or no longer defined (REQ-cli-021). Red at fc14a104, green at 17d60d51 · by claude-code (hc t382)
