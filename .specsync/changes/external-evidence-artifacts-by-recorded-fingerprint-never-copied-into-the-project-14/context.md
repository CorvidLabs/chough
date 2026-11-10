---
change: external-evidence-artifacts-by-recorded-fingerprint-never-copied-into-the-project-14
artifact: context
---

# Context

Issue #14: items 1, 2 and 4 (env on bindings, colour-safe matching, clearer binding/bug-field errors) shipped in #16; the Codex audit on the issue confirms that. Item 3 remains: artifacts had to be copied into the project and committed. Leif's decision (2026-11-10, recorded by the coordinator): allow external evidence artifacts by recorded fingerprint instead of copying them into `.chough/`.

Choice made under that decision: absolute paths only (no `~` expansion), fingerprinted in place with a 64 MiB bound; symlinks, directories and missing files refused.
