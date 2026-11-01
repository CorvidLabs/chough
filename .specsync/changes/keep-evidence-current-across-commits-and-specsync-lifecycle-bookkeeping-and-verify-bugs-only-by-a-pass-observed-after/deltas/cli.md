## MODIFIED

### REQUIREMENT REQ-cli-004

Bind observations to content and environment. The helper SHALL record with each observation the Git revision, a working content identity and an explicit environment, and SHALL report freshness limitations. Freshness SHALL compare the working content identity, which covers every tracked or untracked non-ignored file except Chough's own records, build output and SpecSync lifecycle bookkeeping (state, approval, verification, review, finalization and accepted-state records and the lifecycle state field of a change); a change's deltas and other artifacts remain covered. The recorded revision SHALL be kept for display and regression ancestry and SHALL NOT by itself make an observation stale. An observation recorded without a content identity SHALL keep the revision-bound comparison. Trace: LIFE-2, LIFE-6, LIFE-7.

Acceptance Criteria
- Changing observed content, including a native change's delta or artifacts before materialization, makes prior observations stale; another environment is mismatched; no chosen environment is unchecked.
- Recording another observation, editing the session, committing only Chough records, an empty commit, SpecSync lifecycle bookkeeping writes and archiving a change do not self-invalidate the content identity.
- An observation without a content identity remains stale after any new commit.
