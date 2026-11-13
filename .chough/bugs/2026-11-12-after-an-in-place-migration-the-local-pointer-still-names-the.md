---
kind: bug
id: 2026-11-12-after-an-in-place-migration-the-local-pointer-still-names-the
aliases: []
title: After an in-place migration the local pointer still names the 0.5.x session key, so next and status fail
status: verified
classification: product-defect
scope: in-scope
impact: Not yet assessed.
criteria: []
requirements: [REQ-records-009, REQ-records-003]
cases: [TC-records-a-0-5-pointer-still-names-its-session]
evidence:
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session
  - 2026-11-12-release-0-6-0/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047
receipts: []
environment: local-macos-arm64
revision: 53f118505487a270e1b1f167e316960fe8d83d4f
found_by: "claude-code (hc t382) (t382, dogfood: migrating this repository with the 0.6.0 build)"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-12T14:54:33Z
updated: 2026-11-12T14:57:03Z
---
# After an in-place migration the local pointer still names the 0.5.x session key, so next and status fail

**What happened:** Migrating this repository in place kept .chough/local/active-session with the 0.5.x key; next and status refused 'the active session <key> does not exist' and doctor reported stale_pointer, though the migrated session holds that key as its alias
Actual: The pointer is read as an ID only, never resolved through aliases

**Steps:** chough migrate --apply in a working copy whose 0.5.x pointer names a session key, then chough next

**Expected:** Every record, alias and timeline entry is readable by next and status after the migration (REQ-records-009)

**Why it matters:** Not yet assessed.

**Fix:** Not fixed yet.

## Log

- 2026-11-12T14:54:33Z · Reported (t382, dogfood: migrating this repository with the 0.6.0 build); evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session · by claude-code (hc t382)
- 2026-11-12T14:54:33Z · Added despite similar reports: U4 was session list needing a session folder after an all-archived migration; this is the machine-local pointer still naming a 0.5.x key after an in-place migration, which next and status read · by claude-code (hc t382)
- 2026-11-12T14:57:03Z · status: open -> verified; evidence: 2026-11-12-release-0-6-0/2026-11-12-tc-records-a-0-5-pointer-still-names-its-session-b81047. Green at 8131d511: TC-records-a-0-5-pointer-still-names-its-session passed after the fix; migrate, sessions and doctor tests pass; clippy --all-targets clean. · by claude-code (hc t382)
