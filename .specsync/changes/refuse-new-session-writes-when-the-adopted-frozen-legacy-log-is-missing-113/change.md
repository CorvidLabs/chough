---
id: refuse-new-session-writes-when-the-adopted-frozen-legacy-log-is-missing-113
state: draft
type: bug_fix
base_commit: 4bbd5a6373d26c0ad069878f8f543842a4564a6f
---

# Refuse new-session writes when the adopted frozen legacy log is missing (113)

## Intent

Refuse new-session writes when the adopted frozen legacy log is missing (113)

## Affected Canonical Specs

- `history`

## Acceptance Criteria

- Find a remembered legacy adoption anchor independently of whether transitions.jsonl currently exists. With a valid adoption holder open, archived, shelved or in its existing supported vault location, deleting that frozen log must refuse a new session append before checkpoint/cursor/active-pointer or event bytes change. Preserve original adopting-session refusal and changed-byte mismatch refusal. Intact/restored logs permit new-session writes, and genuinely new projects with no adoption anchor/log still work with legacy:null. Preserve each session own independent lineage; no anchor copying, immutable record rewrite, new schema, broad archive scan or weakened bounds/path protections. Add meaningful committed regression before the production patch and prove same assertions RED then GREEN on an actually built exact-source artifact only under a separate root lease. Keep old tests/assertions and failed receipts. Current older-runtime private8 controls are qualified evidence only, not current-main execution. Update only the allocated per-change cases/changelog/INDEX and own records, freeze clean exact SHA, and return for different-author review; human acceptance/delivery remain separate.

## No-spec Rationale

Repair existing REQ-history-002 acceptance: unexpected anchor changes refuse new writes, together with independent session history REQ-history-006. The missing frozen log is a mismatch with remembered adoption proof. No canonical wording, HI, CLI/schema, retention authorization or new lifecycle policy is introduced; source/test repair implements the existing contract.
