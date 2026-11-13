---
id: 2026-11-06-record-relayed-human-decisions-as-claims-not-authentication
title: Record relayed human decisions as claims, not authentication (#123)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T05:23:00Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-record-relayed-human-decisions-123-d7.md
  - .specsync/archive/changes/2026-11-06-record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7/
record: .chough/archive/sessions/2026-11-06-record-relayed-human-decisions-123-d7.md
actor:
  - agent: claude-code
    terminal: t382
---
# Record relayed human decisions as claims, not authentication (#123)

Chough's approval promise was narrowed to recorded, digest-protected claims rather than authentication. When an agent records a human gate for someone, it now keeps who decided, their words, where, and who recorded it, and a new checkpoint --close-gate option closes one gate with those four values (#123, D7). Authenticating identity or signing inside the tool was rejected; that belongs to attest.

Five relay tests failed first on main. Afterwards the five relay cases passed locally and on Linux, and the full suite passed 456 tests with one flaky timing test filed separately; the verify lane's test steps passed on macOS (463 tests) and Linux.

The maintainer approved the shape and FLOW-16, then accepted the implementation with Record and merge, relayed and recorded by the orchestrating agent. The story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-record-relayed-human-decisions-123-d7.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7/)
