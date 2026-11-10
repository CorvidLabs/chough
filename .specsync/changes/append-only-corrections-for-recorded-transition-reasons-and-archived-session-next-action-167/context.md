---
change: append-only-corrections-for-recorded-transition-reasons-and-archived-session-next-action-167
artifact: context
---

# Context

Issue #167 (Codex's P2 on CorvidLabs/spec-sync#941, deferred here by Leif): a wrong `stage reset --reason` was recorded verbatim, and once the session archived, nothing in `.chough/` could say so without hand-editing. History events are immutable (REQ-history-002), so the correction must be a new append-only record that points at the event it corrects.

Open questions for the human: the command shape (for example `chough history correct <event-id> --reason ... --statement ...`), whether corrections also apply to decisions/open questions in open sessions or only to history events and archived sessions, and whether readers show corrections by default or behind a flag.
