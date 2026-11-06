---
change: record-relayed-human-decisions-at-gates-as-claims-not-authentication-123-d7
artifact: docs
---

# Docs

- docs/cli.md: `--close-gate` and the relay flags in the checkpoint row and the human
  gate section; the gate-record line in timing says a relayed record also carries a
  claim of who decided.
- docs/FRD.md: a REQ-cli-050 row and a REQ-history-010 row.
- README.md: approvals and reviews are recorded claims bound to their content, not
  authenticated identity; relayed decisions use `--close-gate`.
- skills/chough-workflow (bundle and the installed `.agents` copy): SKILL.md,
  references/plan.md, accept.md, lifecycle.md and checkpoint.md. KNOWN_PREVIOUS keeps
  main's digests.
- docs/architecture.md and docs/walkthrough.md: the gate-closing step.
- CHANGELOG.md: one Unreleased entry, including the older-binary note.
