---
change: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
artifact: requirements
---

# Requirements

ADDED REQ-cli-042 (see `deltas/cli.md`): Chough's size bounds apply only to its own
`.chough/` records and to what it displays. HI family exports are parsed whole for
criterion validation; archived `finalization.json`, archived `change.md` and
`.specsync/sdd.json` are read within SpecSync's 4 MiB bound. Display cuts say they are
display only and send the reader to the source. The skill says a Chough bound never
justifies editing HI, specs or SpecSync files.

ADDED REQ-reporting-004 (see `deltas/reporting.md`): `chough timing` reads native
lifecycle records within SpecSync's 4 MiB bound, not the 64 KiB record bound.
