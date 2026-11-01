---
change: bundle-hygiene-case-binding-environment-and-colour-safe-matching-and-actionable-requirement-and-bug-errors
artifact: requirements
---

# Requirements

Modified in `deltas/cli.md`; no requirement is added or removed:

- REQ-cli-005 (validate traceability): a refusal names the companions searched and
  a likely cause and fix.
- REQ-cli-009 (validated test definitions): optional bounded `env`, with the names
  that choose what cargo builds or runs refused for cargo-test; fragments with ESC
  refused; an empty `env` leaves the digest unchanged.
- REQ-cli-011 (honest case execution): declared variables are set for the command;
  every executed case states that inherited variables were not recorded; assertions
  match the text with complete terminal escape sequences removed, never removing a
  line break, and the artifact keeps the raw bytes.
- REQ-cli-012 (local bug backlog): a scope given as classification, or the reverse,
  is refused with the right flag named.
- REQ-cli-019 (installable package): the bundle holds no em or en dash, and copies
  from earlier builds are recognized.

Trace: FLOW-7, LIFE-4, LIFE-5, LIFE-9. These HI criteria exist and are only linked;
no HI criterion is added or reworded. REQ-cli-006 (paths inside the project) and
REQ-cli-016 (update only known earlier bytes) are unchanged and constrain the design.
