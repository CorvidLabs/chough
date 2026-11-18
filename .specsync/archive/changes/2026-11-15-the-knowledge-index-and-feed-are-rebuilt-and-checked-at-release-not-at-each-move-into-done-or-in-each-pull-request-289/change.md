---
id: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
state: archived
type: feature
base_commit: 4d3c1a5cfddc9e6710be2dad1c39162816adacf0
---

# The knowledge index and feed are rebuilt and checked at release, not at each move into done or in each pull request (#289)

## Intent

The knowledge index and feed are rebuilt and checked at release, not at each move into done or in each pull request (#289)

## Affected Canonical Specs

- `cli`
- `records`
- `history`

## Acceptance Criteria

- A change's move into done and its pull request leave knowledge/index.md and feed.xml unchanged, no PR or CI lane runs library index --check, and fledge run release-check rebuilds both and passes the strict check before the release PR merges

## No-spec Rationale

Not applicable
