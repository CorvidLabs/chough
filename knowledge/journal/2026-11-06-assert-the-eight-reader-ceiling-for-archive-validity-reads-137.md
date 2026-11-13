---
id: 2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-137
title: Assert the eight-reader ceiling for archive validity reads (#137)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T06:38:18Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-on.md
  - .specsync/archive/changes/2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11/
record: .chough/archive/sessions/2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-on.md
actor:
  - agent: claude-code
    terminal: t382
---
# Assert the eight-reader ceiling for archive validity reads (#137)

Audit finding CH-M11 asked that REQ-cli-033's ceiling of eight archive validity reads at once hold on any host. An end-to-end test alone cannot catch a broken ceiling on hosts with 8 or fewer cores, so a pure validity_readers function got its own unit test; the ceiling stays 8 and the floor 2.

The end-to-end test failed against the audit's mutant, which ran 12 reads at once on a 20-core machine, and passed once restored. Both cases passed on macOS.

A leftover empty change directory made the knowledge index list another change as active and failed the Linux knowledge step; it was removed and filed as a separate bug. The orchestrating agent recorded the review of PR #146 as a pass under the maintainer's overnight authorization, and the story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-on.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-assert-the-eight-reader-ceiling-for-archive-validity-reads-on-any-host-ch-m11/)
