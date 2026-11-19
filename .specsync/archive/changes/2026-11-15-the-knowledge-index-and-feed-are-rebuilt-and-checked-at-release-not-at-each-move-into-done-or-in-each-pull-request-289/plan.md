---
change: the-knowledge-index-and-feed-are-rebuilt-and-checked-at-release-not-at-each-move-into-done-or-in-each-pull-request-289
artifact: plan
---

# Plan

## Approach

Stop regenerating the knowledge index at the move into done, and stop checking it in pull requests. Rebuild and strictly check it once, at release.

## Out of scope

- A CI job that pushes to main.
- Any change to how the index is generated, or to the merge drivers.

## Steps

1. Defer the refresh at the move into done.
2. Keep `--check` strict, and remove it from the verify and CI lanes.
3. Add the `release-check` task and the release step.
4. Fix every doc and skill line that said otherwise, and re-pin the skill bundle.
5. Tests for the deferred report, the lanes and the strict check.

## Risks

- Between releases, `knowledge/index.md` on main lags its sources. An agent reading it sees the archive as of the last release. The journal posts and the archive folders themselves are current.
- A hand edit to the index in a pull request is not caught until the release rebuild, which overwrites it.

## Constraints consulted

- KNOW-4: the index stays regenerable from its sources, byte for byte.
- Leif's choice of design B.
