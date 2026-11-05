---
change: adopt-event-history-and-per-change-sessions-in-this-repository-and-union-merge-the-retention-ledger-and-changelog
artifact: docs
---

# Docs

- AGENTS.md: the checkpoint now lives in a per-change session. It covers starting work with `chough session new <change-id>`, the one-time merge driver setup, and what still conflicts (SpecSync spec change logs, #862; case files and knowledge notes until the DRIVE-12 product change) with how to resolve each.
