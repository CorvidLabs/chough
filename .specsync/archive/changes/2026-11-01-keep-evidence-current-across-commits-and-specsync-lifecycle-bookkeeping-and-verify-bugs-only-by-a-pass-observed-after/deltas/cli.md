## MODIFIED

### REQUIREMENT REQ-cli-004

Bind observations to content and environment. The helper SHALL record with each observation the Git revision, a working content identity and an explicit environment, and SHALL report freshness limitations. Freshness SHALL compare the working content identity, which covers every tracked or untracked non-ignored file except Chough's own records, build output and SpecSync lifecycle bookkeeping (the state, approval, verification, review, finalization, accepted-state and generated lesson-bundle records of a change and the lifecycle state field of its front matter); a change's deltas and other artifacts remain covered wherever the change directory is. The recorded revision SHALL be kept for display and regression ancestry and SHALL NOT by itself make an observation stale. An observation recorded without a content identity SHALL keep the revision-bound comparison. Trace: LIFE-2, LIFE-6, LIFE-7.

Acceptance Criteria
- Changing observed content, including a native change's delta or artifacts before materialization, makes prior observations stale; another environment is mismatched; no chosen environment is unchecked.
- Recording another observation, editing the session, committing only Chough records, an empty commit, SpecSync lifecycle bookkeeping writes and archiving a change do not self-invalidate the content identity.
- An observation without a content identity remains stale after any new commit.

### REQUIREMENT REQ-cli-012

Local bug backlog. The helper SHALL persist bug records in `.chough/bugs/` with symptom, trigger, expected and actual behavior, criterion, requirement and case links, environment, revision, evidence references, impact, classification, scope and status. It SHALL list open bugs for a fresh session and SHALL search existing records by symptom, trigger and affected IDs before a new record is added. A bug SHALL remain unverified until a failed observation is linked, and a regression classification SHALL require a passed observation at an ancestor revision. A bug SHALL read as reproduced and later passed, and SHALL accept the verified status, only when a present linked passed observation was observed strictly after its most recent present linked failed observation. Additional evidence SHALL be appended rather than overwritten. The helper SHALL render a report candidate and SHALL NOT publish it. Trace: FLOW-8, LIFE-5.

Acceptance Criteria
- A sample failure becomes a reproduced and classified bug; a second report of the same symptom is found by search and its evidence is appended.
- A regression classification without a passed observation at an ancestor revision is refused.
- An out-of-scope bug stays in the open backlog without changing the checkpoint scope.
- Rendering a report candidate prints Markdown and performs no network action.
- A pass observed before, or in the same second as, the latest linked failure leaves the bug reproduced and the verified status is refused; a bug with no linked failure cannot be marked verified.

### REQUIREMENT REQ-cli-017

Observation schema 2. The helper SHALL record schema 2 observations that may add case IDs with definition digests, method, track, executed argument vector, exit code, tests run, limitations, the not-run outcome and a working content identity, which SHALL be a SHA-256 digest when present, and SHALL continue to read schema 1 observations, and schema 2 observations without a content identity, with unchanged freshness semantics. Trace: LIFE-2, LIFE-7, FLOW-7.

Acceptance Criteria
- Existing schema 1 records keep their freshness results, and schema 2 records add stale-case-definition.
- The kinds publication-observation, installation-observation, deployment-observation, live-health-observation and dry-run are accepted, and the existing kinds remain valid.
- A schema 2 record without a content identity keeps its revision-bound freshness result, and a record with a malformed content identity is refused.
