---
change: list-hi-sub-criterion-ids-in-vault-notes-and-read-trace-lists-whole-and-correct-the-know-14-property-wording-corvid
artifact: context
---

# Context

corvid-agent's review of #107 (5779445909) found two bugs in the vault code:
- `vault::is_criterion` accepted only digits and dots after the family, so HI
  sub-criterion IDs such as `UX-1.a` and `CHAT-15.a`, which Chough accepts everywhere
  else (DRIVE-1), never reached `intent/<file>.md`.
- `vault::traces` cut a `Trace:` list at its first `.`, so `Trace: UX-1.a, UX-2.`
  credited `UX-1` and dropped `UX-2`.

REQ-cli-044 already requires every criterion ID to be listed, so no requirement text
changes. The review also noted that KNOW-14 listed status, dates and commits for every
note, while only archived-change notes carry them. Leif chose "Fix the wording", and
KNOW-14 is corrected before #107 merges.
