---
id: hosted-timing-proof-for-105-on-the-long-99-history-do-not-merge
state: archived
type: bug_fix
base_commit: d0480d65f9420068d17f74488a5c74bf0118450e
---

# Hosted timing proof for 105 on the long 99 history (do not merge)

## Intent

Hosted timing proof for 105 on the long 99 history (do not merge)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The three proof paths are covered by this change and match #118 head e18e0941; with nothing relaxed the four hosted SpecSync Archive shards run on the 361-revision #99 history and their durations and counts are posted on #105.

## No-spec Rationale

Draft proof branch only, never merged: carries the #118 archive inspector, fixtures and CI wiring onto #99 final head so the hosted shards walk its 361 revisions; no requirement text changes.
