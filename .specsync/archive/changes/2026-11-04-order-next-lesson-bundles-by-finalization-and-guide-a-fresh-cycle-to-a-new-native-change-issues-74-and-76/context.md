---
change: order-next-lesson-bundles-by-finalization-and-guide-a-fresh-cycle-to-a-new-native-change-issues-74-and-76
artifact: context
---

# Context

Kimi's field lab round 3 (2026-11-03, Chough 4daf861, SpecSync 6.1.2) is an agent lab
review, not a human pass. corvid-agent reproduced the findings on v0.2.0 (edbc3c1) and
filed them as https://github.com/CorvidLabs/chough/issues/74 (N1),
https://github.com/CorvidLabs/chough/issues/75 (N2) and
https://github.com/CorvidLabs/chough/issues/76 (N3).

- Issue 74: `archived_bundles` in src/main.rs sorted archive directory names and
  reversed them, at most 5. SpecSync names archives `<UTC date>-<slug>`, so within a
  day the order is alphabetical by slug. `chough library index` orders by
  `(finalization.timestamp, dir)` and marks the newest 3 hot (KNOW-8), so `next`
  disagreed with the librarian and could drop a hot archive.
- Issue 76 item 2: at plan with no native change recorded, the no-native-change gate's
  command was a bare `specsync change status`, which cannot help.
- Issue 76 item 5: after `stage reset --to plan` from done, the archived old change hit
  the native-archived gate refusal (command: status of the old change), hiding the
  reset's next action. After `--to intent` then advance to plan, `next` treated the old
  archive as this cycle's and recommended another reset, a loop.

Authority: Leif's 2026-11-03 7:49 AM MT standing Chough maintainer authority for bounded
fixes to Chough bug tickets, and his 6:52 PM MT answer "File all three as Chough issues
and fix the clear bugs as small PRs".

Out of scope: issue 75 (no confirmed criterion covers interruption yet), issue 76 items
1, 3 and 4 (need Leif's wording; item 3 waits on DRIVE-6), and the open questions in
issue 74 (stale index note, automatic index). One change covers both issues because both
modify REQ-cli-021 and two open deltas of the same requirement would conflict.
