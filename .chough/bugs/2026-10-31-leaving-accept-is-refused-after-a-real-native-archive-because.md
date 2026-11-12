---
kind: bug
id: 2026-10-31-leaving-accept-is-refused-after-a-real-native-archive-because
aliases: [BUG-0004]
title: Leaving accept is refused after a real native archive because status JSON is truncated
status: verified
classification: product-defect
scope: out-of-scope
impact: "Fails safe (refuses rather than passes), but no real change can leave accept through Chough; status and next also show the archived change's output as invalid. Needs a new native change: parse native JSON within a larger bound separately from the 16 KiB display bound, and add a large-output test."
criteria: [FLOW-10, LIFE-7]
requirements: [REQ-cli-008]
cases: []
evidence: [bug-0004-repro-1, bug-0004-fixed-1, tc-cli-0038-1793475511]
receipts:
  - id: 2026-10-31-after-native-archive-chough-stage-advance-refused-leaving
    aliases: bug-0004-repro-1
    claim: "After native archive, chough stage advance refused leaving accept as native-unknown: the 100133-byte status JSON was cut at the 16384-byte display bound before parsing."
    result: failed
    revision: f860c8a1025700a85aae87f94ce3e883b5a09798
    environment: local-macos-arm64
    fingerprint: sha256:3a461d70a465cc1a6d92ad24a124e81b0ae88f091c63dadead262038bda7a2ea
    observed: 2026-10-31T19:23:27Z
  - id: 2026-10-31-tc-cli-0038-0379a8
    aliases: bug-0004-fixed-1
    claim: "Fixed binary from 6118511 on a disposable worktree of 1a108d9: chough stage advance left accept for done with gate native:archived for the real archived change (100133-byte status); status reported observed with native_summary.state archived."
    result: passed
    revision: 61185115f5e0700eb9f7cc6787de951e6beea9a6
    environment: local-macos-arm64
    fingerprint: sha256:de2e1e3801cb0e7b8e992b6e158ec6160bb986fbafc6bb5e0716813dfe150d7d
    case: TC-cli-0038
    case_digest: cfb3b404ffbddc4b14a889f568d6ac41d3577d18ee79dcf87134c1740bfc926c
    observed: 2026-10-31T19:38:34Z
  - id: 2026-10-31-tc-cli-0038
    aliases: tc-cli-0038-1793475511
    claim: "Ran cargo test --locked --test stages -- --exact large_native_status_still_gates_by_its_summary_state for TC-cli-0038: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test stages -- --exact large_native_status_still_gates_by_its_summary_state
    revision: 61185115f5e0700eb9f7cc6787de951e6beea9a6
    environment: local-macos-arm64
    fingerprint: sha256:6638fbde2eb0157ac9ab2757dea5281538965edccff92be22e2ac2f6f2dc7274
    case: TC-cli-0038
    case_digest: cfb3b404ffbddc4b14a889f568d6ac41d3577d18ee79dcf87134c1740bfc926c
    observed: 2026-10-31T19:38:33Z
environment: local-macos-arm64
revision: f860c8a1025700a85aae87f94ce3e883b5a09798
found_by: Native closeout of evolve-chough-into-a-resumable-stage-workflow
related: []
created: 2026-10-31T19:23:35Z
updated: 2026-10-31T19:38:34Z
---
# Leaving accept is refused after a real native archive because status JSON is truncated

**What happened:** chough stage advance from accept reports refused:native-unknown although specsync reports summary.state archived
Actual: native() bounds stdout to 16384 bytes for display, so the gate parses truncated JSON and refuses

**Steps:** An archived change whose status JSON exceeds 16384 bytes (100133 bytes with 262 acceptance entries here)
1. Finalize a native change with many delivery files
2. chough stage advance from accept

**Expected:** The accept gate reads summary.state archived and allows done (REQ-cli-008)

**Why it matters:** Fails safe (refuses rather than passes), but no real change can leave accept through Chough; status and next also show the archived change's output as invalid. Needs a new native change: parse native JSON within a larger bound separately from the 16 KiB display bound, and add a large-output test.

**Fix:** Recorded before 0.6.0; the Log keeps its history.

## Log

- 2026-10-31T19:23:35Z · Reported (Native closeout of evolve-chough-into-a-resumable-stage-workflow); evidence: bug-0004-repro-1
- 2026-10-31T19:38:34Z · status: open -> verified. Fixed in 6118511 (parse native JSON in full up to 4 MiB); regression test and real archived-state rerun pass.; evidence: bug-0004-fixed-1, tc-cli-0038-1793475511
