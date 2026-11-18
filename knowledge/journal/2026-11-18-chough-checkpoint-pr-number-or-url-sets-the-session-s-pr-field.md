---
id: 2026-11-18-chough-checkpoint-pr-number-or-url-sets-the-session-s-pr-field
title: chough checkpoint --pr <number or URL> sets the session's pr field…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T23:47:01Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-282.md
  - .specsync/archive/changes/2026-11-18-fix-282/
record: .chough/archive/sessions/2026-11-18-fix-282.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# chough checkpoint --pr \<number or URL\> sets the session's pr field…

chough checkpoint --pr <number or URL> sets the session's pr field with the existing validation, like --objective and --title (REQ-cli-007, #282)

Why: 0xLeif decided "Scope approval of fix-282": "add chough checkpoint --pr <number or URL>, with the existing pr validation and a Timeline entry, like every other checkpoint field.

Verified: Ran cargo test --locked --test checkpoint -- --exact checkpoint_pr_sets_and_clears_the_pull_request for TC-cli-checkpoint-pr-sets-the-pull-request: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-282.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-282/)
