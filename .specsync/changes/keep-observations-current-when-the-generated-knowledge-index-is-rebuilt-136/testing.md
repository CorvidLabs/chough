---
change: keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-004 | New regression in tests/evidence.rs: record passed case observation, confirm current, rebuild knowledge/INDEX.md listing it, confirm still current; existing freshness and retention suites unchanged |

RED observed with the src fix stashed: the observation reads non-current after the index rebuild. GREEN with the fix.
