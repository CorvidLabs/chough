---
change: fix-251
artifact: context
---

# Context

GitHub #251 and #279, from CorvidLabs/arcron and NFTMix adopting Chough 0.6.0. Many CorvidLabs repos set SpecSync's `source_dirs = ["."]`.

- **#251:** `chough doctor`'s `records_in_verification` advised listing Chough's record folders in `verification_ignored_paths`. SpecSync refuses any entry that overlaps `source_dirs`, so the advice could not be followed and the finding stayed forever. The arcron follow-up found that SpecSync refuses the list over `meaningful_paths` holding `.` too.
- **#279:** with `source_dirs = ["."]`, the skill bundle `chough init` installs counts as uncovered source, and a 100% coverage gate goes red on adoption.

Leif's decision (2026-11-18, planning interview, on both issues, relayed by hc t4c as the scope approval) is that Chough advises narrowing. When SpecSync's `source_dirs` is `["."]`, `chough init` and `chough doctor` advise narrowing it to the real code folders, for example `["src"]`, and say why, instead of advice SpecSync refuses as overlapping. That fixes both issues together without changing SpecSync's overlap rule.

Asked whether the advice also covers `.` in `meaningful_paths`, he answered "Yes, both" (relayed by hc t4c, 2026-11-18).
