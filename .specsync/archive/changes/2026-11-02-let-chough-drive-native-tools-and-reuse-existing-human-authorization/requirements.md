---
change: let-chough-drive-native-tools-and-reuse-existing-human-authorization
artifact: requirements
---

# Requirements

Add REQ-cli-029, traced to FLOW-1, FLOW-2, FLOW-5, FLOW-13, LIFE-1, LIFE-6 and LIFE-7.
The agent handles native commands inside the Chough conversation, honors applicable
explicit prior or standing human authorization, and records the source and limits
when it writes a concrete native scope approval. It does not repeat authorization
merely because the next operation belongs to an internal tool.

A plain keep-working instruction without applicable authority and an agent review
label are insufficient. Fresh native checks, frozen approved scope, honest product
evidence and separate implementation acceptance remain required. No new CLI writer
or policy engine is added; this is the current-agent skill contract and safe bundle
upgrade behavior.

The native actor is the acting agent account; actual human authorization is the
separately cited source, scope and limits. The human name is not used as the actor
merely because the human authorized agent work. A renewed event preserves earlier
records and covers the refined wording; it does not retroactively rewrite them.
