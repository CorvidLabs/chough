---
id: finish-pr50-integration-with-accepted-main-and-preserve-the-bounded-handoff
state: implementing
type: operations
base_commit: 6715944ff6c4d0423f59e38d76a9cba6ea89e708
---

# Finish PR50 integration with accepted main and preserve the bounded handoff

## Intent

Finish PR50 integration with accepted main and preserve the bounded handoff

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Actual main747a and published PR50 parent671 are ancestors; accepted report reads, two-worker archive executor, identity acceptance and CLI fixes all remain. Proper three-way checkpoint collection semantics preserve all old evidence, archives, human/binding text,24direct pins and200observation IDs;10superseded coordination and41completed notes move verbatim into pinned immutable raw/indexed proof. Final checkpoint remains at most65536bytes/200IDs and all other existing bounds. Genuine final-source tests/cases and artifact/input hashes are recorded after native task text settles; new observations use exact record-file pins. Independent review and all exact-head checks pass before authorized merge. No capacity raise or LIFE-11 work here.

## No-spec Rationale

Integrate already accepted main747a with reviewed CLI fixes, preserve every old proof blob and exact retired checkpoint text, and capture genuine final-source evidence. No new product requirement, capacity increase, output-limit change, archive rewrite or release.
