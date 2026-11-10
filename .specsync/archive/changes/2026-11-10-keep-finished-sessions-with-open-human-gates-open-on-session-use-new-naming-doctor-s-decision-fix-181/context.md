---
change: keep-finished-sessions-with-open-human-gates-open-on-session-use-new-naming-doctor-s-decision-fix-181
artifact: context
---

# Context

Issue #181 (found in CorvidLabs/spec-sync PR #993): `chough session use`/`session new` archived finished sessions that still held an open awaiting-human gate, silently dropping the gate; `chough doctor` treats the same sessions as `stale_gates`, a decision fix that needs `--reason`.

Constraint: REQ-history-012 already defines doctor's `finished_session` as a finished session with NO open gate, so the switch behavior contradicted the existing doctor contract; this change aligns REQ-history-006 with it instead of adding new policy. No new criterion IDs; FLOW-16/DRIVE-17 keep applying: no human decision is recorded by the switch.
