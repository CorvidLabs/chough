# Keep observations current across knowledge index rebuilds (issue 136)

## Modified

### REQUIREMENT REQ-cli-004

Bind observations to content and environment. The helper SHALL record with each observation the Git revision, a working content identity and an explicit environment, and SHALL report freshness limitations. Freshness SHALL compare the working content identity, which covers every tracked or untracked non-ignored file except Chough's own records, build output, the generated knowledge directory (the librarian's configured library dir) and SpecSync lifecycle bookkeeping (the state, approval, verification, review, finalization, accepted-state and generated lesson-bundle records of a change and the lifecycle state field of its front matter); a change's deltas and other artifacts remain covered wherever the change directory is. The recorded revision SHALL be kept for display and regression ancestry and SHALL NOT by itself make an observation stale. An observation recorded without a content identity SHALL keep the revision-bound comparison. Trace: LIFE-2, LIFE-6, LIFE-7.

Acceptance Criteria
- Changing observed content, including a native change's delta or artifacts before materialization, makes prior observations stale; another environment is mismatched; no chosen environment is unchecked.
- Recording another observation, editing the session, committing only Chough records, an empty commit, SpecSync lifecycle bookkeeping writes and archiving a change do not self-invalidate the content identity.
- An observation without a content identity remains stale after any new commit.
- Rebuilding the knowledge index or vault after recording an observation does not make that observation stale.

Acceptance Criteria
- An optional retained snapshot.input_inventory is diagnostic metadata only. Reading valid supported inventory SHALL preserve its stored completeness and SHALL NOT change aggregate content/fingerprint comparisons, artifact/case/environment/ancestry rules, historical outcomes or authority. The reader SHALL NOT infer changed paths or completeness from absent, partial or unsupported inventory. New observations SHALL retain existing writer behavior; this reader change SHALL NOT generate inventories.
