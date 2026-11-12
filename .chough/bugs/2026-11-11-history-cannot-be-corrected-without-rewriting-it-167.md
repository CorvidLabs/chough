---
kind: bug
id: 2026-11-11-history-cannot-be-corrected-without-rewriting-it-167
aliases: [BUG-history-cannot-be-corrected-without-rewriting-it-167-0bd33aabb298af6d0f9d4a83]
title: History cannot be corrected without rewriting it (#167)
status: verified
classification: product-defect
scope: in-scope
impact: "Wrong history stays or gets rewritten by hand (GitHub #167, folded from PR #210)"
criteria: [LIFE-18, LIFE-7]
requirements: [REQ-history-002]
cases: [TC-history-append-only-corrections]
evidence:
  - OBS-tc-history-append-only-corrections-2154744639f0ea1e779ae0b5
  - OBS-tc-history-append-only-corrections-759aeda0263a98f22822c814
receipts:
  - id: 2026-11-11-tc-history-append-only-corrections
    aliases: OBS-tc-history-append-only-corrections-2154744639f0ea1e779ae0b5
    claim: "Ran cargo test --locked --test correct for TC-history-append-only-corrections: failed (exit 101)."
    result: failed
    command: cargo test --locked --test correct
    revision: 9e3eef8ad38c2b6280e5ab337c991d82c7801405
    environment: local-macos-arm64
    content: b12453ad58af8ff7db5a17a14c152fb86dc22cedffbfd336cf3f7bb13d7c14c1
    fingerprint: sha256:a6f26a735e868693fdb92846da129d27e2c03f3f6f6daf0bbb72ef29ad217f71
    case: TC-history-append-only-corrections
    case_digest: 63d947c0746119fc5cc8bfd1fa261342f13d91aa75824dfaa7ae5585c601efb8
    observed: 2026-11-11T19:06:37Z
    duration_seconds: 0
  - id: 2026-11-11-tc-history-append-only-corrections-c13191
    aliases: OBS-tc-history-append-only-corrections-759aeda0263a98f22822c814
    claim: "Ran cargo test --locked --test correct for TC-history-append-only-corrections: passed (exit 0)."
    result: passed
    command: cargo test --locked --test correct
    revision: 82a7392dda5b59925f1dad704d2c6c2778b23b8b
    environment: local-macos-arm64
    content: c0ad4a455cdd87cc042886e343d1ad1a31016c6ee2b83c35f413758823f87006
    fingerprint: sha256:192822405db6ed313be95651961506567c2ea17af31ac9e66127819038f44c42
    case: TC-history-append-only-corrections
    case_digest: 63d947c0746119fc5cc8bfd1fa261342f13d91aa75824dfaa7ae5585c601efb8
    observed: 2026-11-11T19:09:22Z
    duration_seconds: 1
environment: local-macos-arm64
revision: 99b2664f637137a402777a2e023ee66321ffce62
found_by: "GitHub issue #167; red on 0.5.0 at leif/0.6.0-features-red 9e3eef8a"
related: []
created: 2026-11-11T19:07:33Z
updated: 2026-11-11T19:09:40Z
---
# History cannot be corrected without rewriting it (#167)

**What happened:** there is no way to record that an earlier transition's reason, decision or next action was wrong except by editing the record
Actual: chough has no correct command (error: unrecognized subcommand 'correct')

**Steps:** Try to correct an earlier stage move's reason
1. chough init; chough stage advance; chough stage reset --to intent --reason <text>
2. chough correct ^<entry id> --reason <what is true instead>

**Expected:** chough correct appends a correction entry linked to the entry and changes no earlier byte; next and status show the corrected text, marked

**Why it matters:** Wrong history stays or gets rewritten by hand (GitHub #167, folded from PR #210)

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-11T19:07:33Z · Reported (GitHub issue #167; red on 0.5.0 at leif/0.6.0-features-red 9e3eef8a); evidence: OBS-tc-history-append-only-corrections-2154744639f0ea1e779ae0b5
- 2026-11-11T19:09:40Z · status: open -> verified. Fixed by plan step 7 (PR #238, 99b2664f): chough correct appends a linked correction entry and changes no earlier byte; next and status show the corrected text, marked. TC-history-append-only-corrections passed after its 0.5.0 failure.; evidence: OBS-tc-history-append-only-corrections-759aeda0263a98f22822c814
