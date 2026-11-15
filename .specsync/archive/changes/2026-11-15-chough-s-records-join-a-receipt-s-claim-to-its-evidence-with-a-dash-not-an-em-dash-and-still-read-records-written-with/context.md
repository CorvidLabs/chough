---
change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
artifact: context
---

# Context

Issue #278: an archived story's What was verified lines joined each claim to its command and fingerprint digits with ` — ` (`src/sessions.rs`, `receipt_line`). raven's Trust lane lints every tracked Markdown file for em dashes in prose, so archiving a session with `chough doctor --fix` failed CorvidLabs/raven#721. It is the same class of problem as #13, but in generated records. The gate summary's `Change:` line used the same join.

The code change was written by Flint 40B (local, Ollama) through Pi in the #278 bake-off, commit `8d1b298` on `bake2/278-flint`; it was the only run that kept old stories readable. Claude Code brought it onto main, opened this session and native change, and adds the legacy-read test the bake-off grading asked for.
