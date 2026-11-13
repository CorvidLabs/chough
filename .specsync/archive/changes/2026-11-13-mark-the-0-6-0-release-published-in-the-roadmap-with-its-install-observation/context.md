---
change: mark-the-0-6-0-release-published-in-the-roadmap-with-its-install-observation
artifact: context
---

# Context

Chough 0.6.0 was released on Leif's go: the release prep merged in #238 as 443c63ab
(after #239), the coordinator tagged `v0.6.0` there, published crates.io `chough` 0.6.0
and created the GitHub release. releasing.md step 9 marks the release published in the
roadmap's delivered table. On 2026-11-13T01:43Z Leif chose "Yes (Recommended)" for one
small docs PR doing that with the install check result, his merge counting as the
review (relayed by t370).

The clean install (step 8) was run by the coordinator, claude-code-t370, and relayed:
`cargo install chough --version 0.6.0 --locked` into an empty root with a fresh target
directory on macOS arm64 succeeded, and the installed `chough --version` printed
`chough 0.6.0`. It is recorded as an installation observation that t370 made.

The scope is `docs/roadmap-1.0.md`, the observation and native lifecycle records only.
No HI criterion, requirement, contract, tag, publication, GitHub release or deployment
is part of this change.
