---
change: keep-observations-current-when-the-generated-knowledge-index-is-rebuilt-136
artifact: context
---

# Context

Issue #136 (filed at Leif's request, 2026-11-05): `knowledge/INDEX.md` lists the most recent observations and sits inside REQ-cli-004's working content identity, so rebuilding the index after recording an observation makes that observation stale. Reproduced by the regression test in tests/evidence.rs: without the fix the observation reads `historical` after `chough library index`.

Constraint: the legacy fingerprint keeps its exact byte stream for older records, so the exclusion applies only to the content identity and the dirty flag. The library dir name is the configured `library-dir` pref (default `knowledge`). KNOW-3: the knowledge directory points at records, it is not one.
