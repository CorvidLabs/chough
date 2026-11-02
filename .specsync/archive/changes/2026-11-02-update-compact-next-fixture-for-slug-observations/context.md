---
change: update-compact-next-fixture-for-slug-observations
artifact: context
---

# Context

The approved identity implementation rejects old-style IDs on new observation writes. The compact-next large-record fixture in tests/next.rs still supplies "current-one", so it fails before exercising the output bounds. The failure is retained in the takeover's initial-tests.log.

The file is absent from the identity approval's frozen affected_paths. Preserve that approval and delta. This native dependent change covers tests/next.rs only; no production behavior or canonical contract changes.
