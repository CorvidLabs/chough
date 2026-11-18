---
id: 2026-11-18-the-feature-bug-loop-example-stops-at-a-refused-chough-or
title: The feature-bug-loop example stops at a refused chough or SpecSync…
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T16:55:11Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-fix-274.md
  - .specsync/archive/changes/2026-11-18-fix-274/
record: .chough/archive/sessions/2026-11-18-fix-274.md
actor:
  - agent: chough-fixer
    model: claude-opus-5-5
    terminal: t4n
---
# The feature-bug-loop example stops at a refused chough or SpecSync…

The feature-bug-loop example stops at a refused chough or SpecSync command, with its error (#274)

Why: No requirement text changes: no requirement covers how the example fails. Each chough or SpecSync command whose JSON jq reads is captured first, so set -e stops the script at a refusal with its own error; set -o pipefail was rejected because dash lacks it.

Verified: Ran cargo test --locked --test cli -- --exact exercise_stops_at_a_refused_bug_update for TC-cli-exercise-stops-at-a-refused-command: passed (exit 0, tests run 1).

Read more: [the story](../../.chough/archive/sessions/2026-11-18-fix-274.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-fix-274/)
