---
id: 2026-11-19-conflict-marker-follow-ups
title: Conflict-marker follow-ups
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T00:47:01Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-339.md
  - .specsync/archive/changes/2026-11-19-fix-339/
record: .chough/archive/sessions/2026-11-18-fix-339.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# Conflict-marker follow-ups

Conflict-marker follow-ups: markers in fenced code blocks don't count, a stray ======= is caught, case definitions get the same check, and migrate names the 0.5.x source (REQ-records-001, #339)

Why: 0xLeif decided "Scope approval of fix-339": "exempt markers inside fenced code blocks, catch a stray =======, case TOML findings, and the migrate message naming the 0.5.x source … Warn only" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his decisions on the #336 follow-ups filed as CorvidLabs/chough#339, his scope approval for that issue; then for item 2 'Warn only': a lone line of exactly seven '=' with no <<<<<<< or >>>>>>> around it is reported by doctor as 'possible leftover conflict separator at line N', and writes are never refused for it); recorded by…

Verified: Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_conflict_inside_a_code_fence for TC-records-doctor-warns-of-a-conflict-inside-a-code-fence: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-339.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-339/)
