---
id: 2026-11-24-fix-436-overnight
title: fix-436-overnight
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-24
time: 2026-11-24T14:16:26Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-24-fix-436.md
  - .specsync/archive/changes/2026-11-24-fix-436/
record: .chough/archive/sessions/2026-11-24-fix-436.md
actor:
  - terminal: t14z
---
# fix-436-overnight

fix-436-overnight

Why: 0xLeif decided "Scope approval of fix-436 (display name in decision text)": "I approve" (pi chat t14z (CorvidLabs coordination session), 2026-11-24 about 03:20 UTC, on his standing overnight approval of the six-fix stack (fix-436 is the sixth), relayed by pi-t14z after he confirmed the display-name-only scope in the one-at-a-time interview — flagged as a relayed claim, not authenticated); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-436 as 0xLeif

Verified: Ran cargo test --locked --test humans -- --exact an_exact_name_close_renders_the_display_name_unchanged for TC-records-gate-decision-exact-name-unchanged: passed (exit 0). - c2be898a1c3c; limits: outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected; inherited environment variables were not recorded

Read more: [the story](../../.chough/archive/sessions/2026-11-24-fix-436.md) · [the archived change](../../.specsync/archive/changes/2026-11-24-fix-436/)
