---
change: release-0-6-1
artifact: context
---

# Context

0.6.1 is a fixes-only patch release, decided by Leif on 2026-11-18 and relayed
by his planning session (hc t4c): "focus on getting 0.6.1 ready and up and
released and all so we can unblock Kyn". Jackdaw PR 184 needs the blocking-bug
fix (#304) in a released Chough.

`chough change ready` (#311) and its fix (#310) are not in it; they go to 0.7.0.
#306 was reverted (#323), so #260 is not fixed in 0.6.1.

This change is the release preparation of `docs/releasing.md`: the version, the
change log and the rebuilt knowledge index, on a branch, merged before the tag.
Tagging, publishing and the GitHub release are Leif's separate decisions and are
not part of this change.
