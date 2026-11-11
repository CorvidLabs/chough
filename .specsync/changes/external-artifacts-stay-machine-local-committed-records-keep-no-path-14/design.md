---
change: external-artifacts-stay-machine-local-committed-records-keep-no-path-14
artifact: design
---

# Design

`bind_external` replaces each absolute artifact with `external:<first 16 hex of its SHA-256>` and stores ID→path in `.chough/local/external-artifacts.json` (bounded at 4096 entries; `.chough/local/.gitignore` is `*`). `artifact_fingerprints` resolves an `external:` ID through that map. Freshness becomes tri-state for artifacts: current, changed (stale-artifact), or not checkable (`external-not-checkable-here`, its own label, after content, artifact-change, case and requested-environment checks). Retention already removes only artifacts under `.chough/artifacts/`, so an external file is never deleted.
