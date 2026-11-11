---
change: external-artifacts-stay-machine-local-committed-records-keep-no-path-14
artifact: plan
---

# Plan

## Approach

Bind external artifacts to neutral IDs at record time, keep paths machine-local, add the not-checkable freshness label, update docs and tests, and dogfood including `evidence prune --apply`.

## Out of scope

A user-chosen label for the ID (Leif allowed a basename only if the user chooses it; not needed now), migrating records written by the archived change on this unmerged branch (none exist on main), and the general no-leak scan (#234).

## Steps

1. `bind_external` and the machine-local map.
2. Tri-state artifact freshness and the `external-not-checkable-here` label.
3. Tests: no path, file name or home in the record; mapping present and git-ignored; stale on change; not checkable without the mapping.
4. docs/cli.md, CHANGELOG.

## Risks

- A machine-local map lost (fresh clone, cleaned `.chough/local/`) makes the observation unverifiable there by design.
- Two files with identical bytes share one ID; the map keeps the latest path.

## Constraints consulted

Leif's #208 decision; REQ-cli-006; REQ-cli-004 freshness; #234.
