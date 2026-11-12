---
id: 2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-125
title: "Size the checkpoint from measured use: a 1 MiB cap (#125)"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T03:34:19Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap.md
  - .specsync/archive/changes/2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each/
record: .chough/archive/sessions/2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap.md
actor:
  - agent: claude-code
    terminal: t382
---
# Size the checkpoint from measured use: a 1 MiB cap (#125)

For DRIVE-14 and issue #125, the checkpoint was sized from measured use: a 1 MiB cap with budgets at the cap, a way to shelve a session, and sessions that never break each other.

The three cap cases passed on Linux: a checkpoint at the cap stays fast and one byte more is refused, handoff blocks stay within the cap and are byte-identical, and one oversized or unreadable session does not break the others. The cap, next and sessions tests also passed on macOS, and all new tests failed first.

The maintainer accepted PR #133 on 2026-11-05, choosing to record and merge. The change was archived and the story closed as done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-size-the-checkpoint-from-measured-use-a-1-mib-cap-with-at-cap-budgets-session-shelve-and-sessions-that-never-break-each/)
