---
change: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16
artifact: context
---

# Context

Leif chose "Yes, one small PR" (orc tab, relayed by the orc, 2026-11-06): move the #77 sessions entry out of [0.3.0] to the release it shipped in, and add the missing #111 entry. He then captured LIFE-16 with "Capture as written".

## Where each change shipped, from the tags

- #77 merged as a994efa2 on 2026-11-05 at 02:01 UTC. v0.3.0 (4d65542f) was released on 2026-11-04 at 21:13 UTC. v0.4.0 (69950c7a) contains a994efa2, and v0.3.0 does not. The change log at the v0.3.0 tag has no sessions entry. #77's branch predates the release, so its hunk landed under [0.3.0] when it merged, and the v0.4.0 notes never listed sessions.
- #111 merged as ded0e9ef. It is in v0.4.0 and not in v0.3.0, and the change log has no entry for it. Its product change is the bug fix archived as `let-a-session-started-after-legacy-history-adoption-write-events-while-the-adopted-legacy-log-stays-guarded` (REQ-history-006). The rest of #111 adopted event history and sessions in this repository, which is not product behavior.

## The rest of the change log was checked

- Each released section on main was compared with the same section at its tag. The #77 entry is the only text added to a released section after its tag. [0.1.0]'s later differences are its release note and the link footers, and [0.4.0] matches v0.4.0 exactly.
- Every entry that cites a merged pull request sits under the release that first contained it (46 references checked). Entries with no pull-request number can only be checked by comparing with the tag.

## Constraints

- No tags, releases or crates.io publishing.
- `CHANGELOG.md` merges with Git's union driver.
- One pull request for this concern. It merges after #150, and after #151 in this agent's queue.
- The test reads only `CHANGELOG.md`: CI clones carry no tags, and the test needs no network.
