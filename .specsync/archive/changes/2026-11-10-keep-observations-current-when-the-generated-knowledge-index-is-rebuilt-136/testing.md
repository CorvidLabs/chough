---
change: keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136
artifact: testing
---

# Testing

| Requirement | Coverage |
|---|---|
| REQ-cli-004 | `TC-cli-knowledge-rebuild-keeps-observation-current` binds the regression `rebuilding_the_knowledge_index_keeps_the_observation_current` in tests/evidence.rs: record a passed case observation, confirm it reads current, rebuild knowledge/INDEX.md listing it, confirm it still reads current |

Failing-first: the regression is committed before the fix and run there through the case (failed observation linked to the #136 bug); the same case passes after the fix. The full `fledge run test` suite and the existing freshness and retention suites run unchanged.
