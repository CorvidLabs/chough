# Diagnostic inventory reader compatibility

## Modified

### REQUIREMENT REQ-cli-004

Bind observations to content and environment. The helper SHALL record with each observation the Git revision, a working content identity and an explicit environment, and SHALL report freshness limitations. Freshness SHALL compare the working content identity, which covers every tracked or untracked non-ignored file except Chough's own records, build output and SpecSync lifecycle bookkeeping (the state, approval, verification, review, finalization, accepted-state and generated lesson-bundle records of a change and the lifecycle state field of its front matter); a change's deltas and other artifacts remain covered wherever the change directory is. The recorded revision SHALL be kept for display and regression ancestry and SHALL NOT by itself make an observation stale. An observation recorded without a content identity SHALL keep the revision-bound comparison. Trace: LIFE-2, LIFE-6, LIFE-7.

Acceptance Criteria
- Changing observed content, including a native change's delta or artifacts before materialization, makes prior observations stale; another environment is mismatched; no chosen environment is unchecked.
- Recording another observation, editing the session, committing only Chough records, an empty commit, SpecSync lifecycle bookkeeping writes and archiving a change do not self-invalidate the content identity.
- An observation without a content identity remains stale after any new commit.

Acceptance Criteria
- An optional retained snapshot.input_inventory is diagnostic metadata only. Reading valid supported inventory SHALL preserve its stored completeness and SHALL NOT change aggregate content/fingerprint comparisons, artifact/case/environment/ancestry rules, historical outcomes or authority. The reader SHALL NOT infer changed paths or completeness from absent, partial or unsupported inventory. New observations SHALL retain existing writer behavior; this reader change SHALL NOT generate inventories.

### REQUIREMENT REQ-cli-006

The helper SHALL bound assembled context and reject unsupported records or paths escaping the project. Trace: LIFE-3, LIFE-4.

Acceptance: traversal, invalid module/record IDs and symlink escape attempts fail; large context is visibly bounded. Local artifacts remain reviewable plain files.

Acceptance Criteria
- The reader SHALL accept optional diagnostic inventory v1 with strict inventory fields version:u32, complete:bool, entries and strict entry fields domain:u8, key, path, digest. Inventory SHALL contain at most128 entries and at most32768 compact serialized JSON bytes. v1 entries SHALL have unique safe nonempty relative normal paths, valid SHA256 digests and domain/key matching the existing canonical fingerprint_key classification. Unknown fields/types, invalid semantic entries, exceeded limits and unsupported versions SHALL refuse explicitly without mutating observations. Absent/null inventory SHALL preserve existing legacy reading behavior and existing whole-record bounds.

