---
change: external-artifacts-stay-machine-local-committed-records-keep-no-path-14
artifact: context
---

# Context

Follow-up on the same PR (#208) to the archived change `external-evidence-artifacts-by-recorded-fingerprint-never-copied-into-the-project-14`, which committed the external file's absolute path in the observation record. Dogfooding showed the path is machine-specific; Leif decided (2026-11-10, relayed by the coordinator): "mark it machine-local.. also don't leak info or creds or pii.. what if my name was there". Committed records keep only a neutral ID and the fingerprint; the path stays in git-ignored machine-local state; other machines see `external-not-checkable-here`. The general no-leak rule for all committed records is #234.
