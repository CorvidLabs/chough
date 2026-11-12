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

D2: the gate summary's base is the change's own `base_commit`, unless the merge base
of HEAD with the default branch is not an ancestor of it (default-branch work merged or
rebased in after the change began, #190), or the change records none; a merge base that
is HEAD itself is never used, and with no usable base the summary says so. One more Git
call, no native read.

The migrate preview: when a native change is open, the plan says up front that
`--apply` will refuse, naming each open change. The preview still writes nothing,
and the apply's refusal is unchanged.

Everything else is configuration, evidence or text: the Cargo version, the CHANGELOG
section, a test helper's name, and case links under `.chough/cases/`.
