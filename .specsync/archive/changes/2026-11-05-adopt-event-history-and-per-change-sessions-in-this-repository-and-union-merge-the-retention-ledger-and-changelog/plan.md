---
change: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
artifact: plan
---

# Plan

## Approach

Use Chough's own adoption commands on this repository; no product code changes. `history adopt --apply` freezes the legacy transition log behind a digest anchor and sends new transitions to event files. `session adopt` moves the checkpoint into `.chough/sessions/`. `.gitattributes` gives the append-only ledger and the changelog Git's union driver. AGENTS.md teaches the per-change routine.

## Out of scope

- One case file per change, and rebuilding knowledge notes after a merge: the DRIVE-12 product change.
- Spec change logs and `requirements.md`: SpecSync, CorvidLabs/spec-sync#862.
- Pruning history.

## Steps

1. Approve this definition, then move to build.
2. Preview and apply `history adopt`, then `session adopt`.
3. Edit `.gitattributes` and AGENTS.md.
4. Run the merge simulation before and after, then `fledge lanes run verify`.
5. Close out, push, and merge before other branches diverge. Note on corvid-agent's #69 that it will conflict once on the moved checkpoint.

## Risks

- **An older helper rejects the adopted checkpoint.** CI and local helpers build Chough from this tree, and the docs say to use a compatible helper and never remove the cursor.
- **A branch still edits `.chough/session.json`.** It conflicts once with the move; resolve by keeping the move and redoing its checkpoint updates in its own session. Only #69 is open now.
- **Union keeps both sides of a changed line.** The ledger is append-only. For CHANGELOG.md, both bullets are kept; a reviewer edits wording if two branches touched the same bullet.

## Constraints consulted

DRIVE-3, DRIVE-11, DRIVE-12, REQ-history-006, REQ-history-007, and the checkpoint reference in the skill (adopt on the main line before branches diverge).
