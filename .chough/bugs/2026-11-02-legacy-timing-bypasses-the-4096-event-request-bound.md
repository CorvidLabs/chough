---
kind: bug
id: 2026-11-02-legacy-timing-bypasses-the-4096-event-request-bound
aliases: [BUG-legacy-timing-bypasses-the-4096-event-request-bound-55a3830ea9566f55e4a6d84a]
title: Legacy timing bypasses the 4096-event request bound
status: verified
classification: product-defect
scope: in-scope
impact: "P2: approved request work bound is not enforced for legacy projects"
criteria: [LIFE-8]
requirements: [REQ-cli-026]
cases: [TC-cli-history-bounds]
evidence: [OBS-product-test-557ba9d0d6f01009fa59fa6c, OBS-product-test-2ae43c5b0d01e1ff9d570f51]
receipts:
  - id: 2026-11-02-independent-bounds-review-reproduced-legacy-timing-exceeding
    aliases: OBS-product-test-557ba9d0d6f01009fa59fa6c
    claim: "Independent bounds review reproduced legacy timing exceeding the approved 4096-event limit: 5000 valid transitions in 714100 bytes produced 5000 records and no partial warning at clean 6fa45db."
    result: failed
    revision: 6fa45db5d78dfcc1a1176d6a37fe7da36cf4bf90
    environment: local-macos-arm64
    content: 0706650c13e025a3e3af738b2b3a63ed9e32dcdf22d03c217ee27d913c34e031
    fingerprint: sha256:a99db52e29c35401c28cf6ae48804ea54b3e24f6a55ae20a60eacb8dee2abcdb
    observed: 2026-11-02T15:22:30Z
  - id: 2026-11-02-both-legacy-timing-regressions-failed-before-the-fix-and
    aliases: OBS-product-test-2ae43c5b0d01e1ff9d570f51
    claim: "Both legacy timing regressions failed before the fix and passed after it: exact 4096 and 4097 input lines, chronological suffix, explicit omission, malformed/unknown input consuming the limit, and byte-identical fixture state."
    result: passed
    revision: 6fa45db5d78dfcc1a1176d6a37fe7da36cf4bf90
    environment: local-macos-arm64
    content: 84b7cd392431a504dc99aca3bb108d89429779d75e6d16c1ef5357001233b7c5
    fingerprint: sha256:86a2cc590e1756d53ad48a8219b27fa1e6402d98e677c406abee5e4a3da797d5
    observed: 2026-11-02T15:25:50Z
environment: local-macos-arm64
revision: 6fa45db5d78dfcc1a1176d6a37fe7da36cf4bf90
found_by: independent history_bounds_review at 6fa45db
related: []
created: 2026-11-02T15:22:42Z
updated: 2026-11-02T15:25:51Z
---
# Legacy timing bypasses the 4096-event request bound

**What happened:** Legacy timing consumes more than 4096 transition records without a partial warning
Actual: 5000 records and segments are processed with partial empty

**Steps:** A checkpoint without history_cursor and more than 4096 valid JSONL transitions below the 16 MiB byte limit

**Expected:** Every timing request processes at most 4096 events and explicitly reports omitted earlier history

**Why it matters:** P2: approved request work bound is not enforced for legacy projects

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-11-02T15:22:42Z · Reported (independent history_bounds_review at 6fa45db); evidence: OBS-product-test-557ba9d0d6f01009fa59fa6c
- 2026-11-02T15:25:51Z · status: open -> verified. Shared history event limit now applies before parsing legacy transitions; independent failing input and exact boundary regressions are retained. Dedicated executable case: TC-cli-history-legacy-timing-bounds.; evidence: OBS-product-test-2ae43c5b0d01e1ff9d570f51
