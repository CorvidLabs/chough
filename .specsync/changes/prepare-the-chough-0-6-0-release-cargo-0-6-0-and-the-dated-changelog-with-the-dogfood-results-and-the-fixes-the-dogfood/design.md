---
change: prepare-the-chough-0-6-0-release-cargo-0-6-0-and-the-dated-changelog-with-the-dogfood-results-and-the-fixes-the-dogfood
artifact: design
---

# Design

D1: the active-session pointer resolves through aliases. `sessions::pointed` returns
the pointer's session ID: the pointer itself when a session folder or story has that
ID, otherwise the one open session whose alias or bare key it is. `active()` and
doctor's `stale_pointer` check both use it, so a pointer an older layout or a
`session use <alias>` left keeps naming its session, and a pointer naming nothing
is still `stale_pointer`. The pointer file is machine-local and is not rewritten.

Everything else is configuration, evidence or text: the Cargo version, the CHANGELOG
section, a test helper's name, and case links under `.chough/cases/`.
