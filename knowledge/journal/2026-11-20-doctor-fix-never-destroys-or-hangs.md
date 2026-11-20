---
id: 2026-11-20-doctor-fix-never-destroys-or-hangs
title: doctor --fix never destroys or hangs
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-20
time: 2026-11-20T01:16:16Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-19-doctor-fix-safety.md
  - .specsync/archive/changes/2026-11-20-doctor-fix-safety/
record: .chough/archive/sessions/2026-11-19-doctor-fix-safety.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# doctor --fix never destroys or hangs

doctor --fix never destroys or hangs: the archive finish keeps evidence its story does not hold (#381) and removes OS metadata (#377), next names the archive finish (#371), doctor names front-matter conflicts and gives no restore advice for a redaction (#374), and the spec walk stays inside the project and is bounded (#402)

Why: 0xLeif decided "Scope approval of doctor-fix-safety": "Approve, with the 3 recommended choices" (Relayed by Leif's planning session (hc t4c) through fable-lead (hc t53) to chough-fixer, 2026-11-18, his scope approval of D1 'doctor --fix must not destroy or hang' (#381, #402, #377, #371, #374) for the deltas at bf9f70c, with the three recommended choices: #381 keeps and names the entry, attaching…

Verified: Ran cargo test --locked --test doctor -- --exact the_spec_walk_follows_no_link_and_is_bounded for TC-records-the-spec-walk-follows-no-link-and-is-bounded: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-19-doctor-fix-safety.md) · [the archived change](../../.specsync/archive/changes/2026-11-20-doctor-fix-safety/)
