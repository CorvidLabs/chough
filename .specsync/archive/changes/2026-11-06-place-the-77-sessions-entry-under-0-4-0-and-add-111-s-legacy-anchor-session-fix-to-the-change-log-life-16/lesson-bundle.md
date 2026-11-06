# Lesson bundle: place-the-77-sessions-entry-under-0-4-0-and-add-111-s-legacy-anchor-session-fix-to-the-change-log-life-16

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Place the #77 sessions entry under 0.4.0 and add #111's legacy-anchor session fix to the change log (LIFE-16)
- **Kind**: Documentation
- **Paths**: CHANGELOG.md, tests/changelog.rs
- **Acceptance**: In CHANGELOG.md the #77 sessions entry ('Sessions per objective or native change') sits under [0.4.0], not [0.3.0], with its text unchanged except that it now names #77; [0.4.0] gains one entry for #111's fix, so a session started with chough session new after a project adopted its legacy history writes events instead of refusing with 'legacy history anchor changed'; no other entry in a released section changes. The new tests in tests/changelog.rs fail first on main, and fledge lanes run verify passes.

## Evidence

- Verification commit: `c8c1b6b7ddb63ce78173255bf4370b5b43595c26`
- Base commit: `b535b92fb0e7cf5679449c18860e136bb27397e6`
- Verified by: `specsync check (no spec in scope)`

## From the change's context.md

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

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
