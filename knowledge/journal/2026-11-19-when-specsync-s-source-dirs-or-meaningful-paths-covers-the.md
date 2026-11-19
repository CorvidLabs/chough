---
id: 2026-11-19-when-specsync-s-source-dirs-or-meaningful-paths-covers-the
title: When SpecSync's source_dirs or meaningful_paths covers the whole…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T00:32:43Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-251.md
  - .specsync/archive/changes/2026-11-19-fix-251/
record: .chough/archive/sessions/2026-11-18-fix-251.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# When SpecSync's source_dirs or meaningful_paths covers the whole…

When SpecSync's source_dirs or meaningful_paths covers the whole repository, chough init and chough doctor advise narrowing to the code folders and say why, instead of a list SpecSync refuses (REQ-records-014, #251, #279)

Why: 0xLeif decided "Scope approval of fix-251": "Chough advises narrowing. When SpecSync's source_dirs is ["."], chough init and chough doctor advise narrowing it to the real code folders (for example ["src"]) and say why, instead of advice SpecSync refuses as overlapping. This fixes #251 and #279 together, without changing SpecSync's overlap rule.

Verified: Ran cargo test --locked --test doctor -- --exact a_whole_repository_specsync_gets_the_narrowing_advice for TC-records-a-whole-repository-specsync-gets-the-narrowing-advice: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-251.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-251/)
