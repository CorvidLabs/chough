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

The tester's check of #354 passed the arcron shape and blocked on NFTMix's. There, every specified file sits at the repository root, so there is no folder to narrow `source_dirs` to. The literal example `["src"]` leaves coverage at 0/0, and no SpecSync setting satisfies both the coverage gate and the ignored list. Leif chose "Both: better advice + SpecSync" (relayed by hc t4c, 2026-11-18). When the specified code sits at the root, init and doctor advise adding the skill bundle's and Chough's folders to `exclude_dirs`, which fixes #279's coverage. They also say the ignored list needs the code moved into a folder, or SpecSync's fix, filed as CorvidLabs/spec-sync#1083. The folder-repo advice stays as it is.
