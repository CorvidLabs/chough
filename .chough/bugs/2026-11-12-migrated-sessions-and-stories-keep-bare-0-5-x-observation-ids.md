---
kind: bug
id: 2026-11-12-migrated-sessions-and-stories-keep-bare-0-5-x-observation-ids
aliases: []
title: Migrated sessions and stories keep bare 0.5.x observation IDs in What was verified
status: verified
classification: product-defect
scope: in-scope
impact: Migrated records read worse than new ones (t38m F10e)
criteria: []
requirements: [REQ-records-009]
cases:
  - TC-records-migrate-evidence-verified-lines-link-entries
  - TC-records-migrate-evidence-story-lines-are-receipt-lines
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-verified-lines-link-entries
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-story-lines-are-receipt-lines
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-verified-lines-link-entries-540c6d
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-story-lines-are-receipt-lines-784592
receipts: []
environment: local-macos-arm64
revision: 4e245263fe4e35b757d9b0531a0992c7cf0f884d
found_by: claude-code (hc t382) (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b)
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T23:05:08Z
updated: 2026-11-12T23:06:39Z
---
# Migrated sessions and stories keep bare 0.5.x observation IDs in What was verified

**What happened:** After chough migrate --apply, an open session's What was verified keeps lines such as '- OBS-product-test-…' instead of linking the carried evidence entry, and a migrated story keeps them instead of receipt lines
Actual: Bare old IDs

**Steps:** Trigger: Migrate a 0.5.x project whose sessions reference observations
1. chough migrate, then chough migrate --apply <digest> with consent
2. Read a migrated session.md and an archived story

**Expected:** Each line links the carried entry, or is a receipt line in a story, the old ID kept as its alias

**Why it matters:** Migrated records read worse than new ones (t38m F10e)

**Fix:** 94f15a7a and 421a7034 (t38b's 1f5b54b8 and 160a9979): a migrated session's verified lines link the carried entries, and a migrated story's are receipt lines, the old IDs kept as aliases

## Log

- 2026-11-12T23:05:08Z · Reported (t38m's third fresh-eyes round report, F10 (relayed by t370); fixed by t38b); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-verified-lines-link-entries, 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-story-lines-are-receipt-lines · by claude-code (hc t382)
- 2026-11-12T23:06:28Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-verified-lines-link-entries-540c6d, 2026-11-12-release-0-6-0/2026-11-12-tc-records-migrate-evidence-story-lines-are-receipt-lines-784592; fix: t38b's 1f5b54b8 and 160a9979: a migrated session's verified lines link the carried entries, and a migrated story's are receipt lines, the old IDs kept as aliases. Red at 4e245263, green at 421a7034 · by claude-code (hc t382)
- 2026-11-12T23:06:39Z · fix: 94f15a7a and 421a7034 (t38b's 1f5b54b8 and 160a9979): a migrated session's verified lines link the carried entries, and a migrated story's are receipt lines, the old IDs kept as aliases. Fix line names the commits on leif/human-first-records · by claude-code (hc t382)
