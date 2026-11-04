---
change: read-hi-exports-and-native-specsync-files-at-full-size-for-every-gate-and-validation-so-chough-bounds-never-push-an
artifact: design
---

# Design

- `records::read_native_text` reads a native file within `NATIVE_RECORD_LIMIT`
  (4 MiB, SpecSync's own change-artifact bound) and names that bound when exceeded.
  The four SpecSync readers above use it.
- `context::hi_export_json` runs `hi export` through `records::native_json`, which parses
  the complete stdout up to the 4 MiB parse bound and keeps the display section bounded;
  `validate_criteria` uses the parsed value. `chough context` keeps `hi_export` for display.
- The test-only `with_native_stdout` stand-in now also answers `native_json`, so unit
  tests that validate criteria keep running without a host `hi`.
- Display markers (`records::DISPLAY_TRUNCATED` and the context aggregate markers) say
  "for display only", send the reader to the complete source and say never to shorten it.
  Section state labels are unchanged.
- Skill: one Working rules bullet in SKILL.md (kept under its 6,144-byte entry bound by
  tightening the Host use wording without dropping a fact) and one build.md bullet; the
  previous digests join `KNOWN_PREVIOUS` so installed copies update.
