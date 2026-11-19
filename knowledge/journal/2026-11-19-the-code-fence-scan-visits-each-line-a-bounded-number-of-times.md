---
id: 2026-11-19-the-code-fence-scan-visits-each-line-a-bounded-number-of-times
title: The code-fence scan visits each line a bounded number of times
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T01:43:58Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-19-fix-339-fence-pass.md
  - .specsync/archive/changes/2026-11-19-fix-339-fence-pass/
record: .chough/archive/sessions/2026-11-19-fix-339-fence-pass.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# The code-fence scan visits each line a bounded number of times

The code-fence scan visits each line a bounded number of times, so many unclosed fence openers no longer make record writes and doctor quadratic, and an unclosed fence still hides nothing (#356 review P2)

Why: 0xLeif decided "Scope approval of fix-339-fence-pass": "Yes, for today" (Relayed by Leif's planning session (hc t4c) to chough-fixer (hc fixer), 2026-11-18: his standing rule for today, that a real gap his review thread finds in an already-approved PR is fixed inside its approved scope as a small follow-up change, applied to his P2 thread on #356 ('Avoid rescanning the full suffix for every unmatched fence'): compute matching fences in a bounded pass, keep the rule that an unclosed fence hides…

Verified: Ran cargo test --locked --test doctor -- --exact doctor_warns_of_a_conflict_inside_a_code_fence for TC-records-doctor-warns-of-a-conflict-inside-a-code-fence: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-19-fix-339-fence-pass.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-fix-339-fence-pass/)
