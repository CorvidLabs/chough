---
change: find-the-legacy-history-anchor-in-sessions-rolled-into-the-vault-or-shelved-141-p1
artifact: context
---

# Context

Issue #141 (P1). ch-d7 found it on pristine main `260ecc94`. It is my regression from KNOW-29 (#135).

`adopted_anchor` (src/history.rs) found the session that recorded the legacy history anchor by scanning only `.chough/sessions/` and `.chough/sessions/archive/`. KNOW-29's roll-up moved that session, `adopt-event-history-...`, into `knowledge/attachments/adopt-event-c467f940/sessions/`. Since then, every session without its own anchor refuses gate and stage writes with "legacy history anchor changed; restore the recorded legacy bytes", although `.chough/transitions.jsonl` is byte-identical. That blocks every open Chough session in this repository.

The new test reproduces the refusal exactly, for a session rolled into the vault and for a shelved one.

Shelved sessions (#133) have the same gap.
