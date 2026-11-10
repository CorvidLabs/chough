---
change: let-automated-case-bindings-record-artifacts-outside-the-project-with-a-recorded-fingerprint-14
artifact: context
---

# Context

Issue #14 item 3 (the only one of its five rough edges still open; items 1, 2, 4 and 5 shipped in #16): case artifacts must be paths inside the project, so browser-run screenshots had to be copied into `.chough/artifacts/` and committed. The proposal: allow an artifact declared outside the project, recording its path, size and SHA-256 in the observation instead of copying it in.

Open questions for the human: whether the observation should copy nothing (fingerprint only) or offer an opt-in copy; how freshness treats an external artifact whose bytes later change; and whether Git LFS should instead be documented as the answer for large artifacts.
