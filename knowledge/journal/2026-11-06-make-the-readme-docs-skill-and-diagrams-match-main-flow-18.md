---
id: 2026-11-06-make-the-readme-docs-skill-and-diagrams-match-main-flow-18
title: Make the README, docs, skill and diagrams match main (FLOW-18)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T20:47:22Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-make-the-readme-docs-workflow-skill-and-diagrams-match-main.md
  - .specsync/archive/changes/2026-11-06-make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18/
record: .chough/archive/sessions/2026-11-06-make-the-readme-docs-workflow-skill-and-diagrams-match-main.md
actor:
  - agent: claude-code
    terminal: t382
---
# Make the README, docs, skill and diagrams match main (FLOW-18)

The README, docs, workflow skill and diagrams had drifted from what main does. One PR, as the maintainer chose, corrected them and added ten FLOW-18 checks that fail when they drift; the stage, lifecycle and ownership diagrams were re-rendered with Mermaid 11.12.0.

Eight of the ten checks failed on the base first. After the fix all ten FLOW-18 cases passed on macOS and again on the tree with main merged in. In the final state all ten cases and cargo test (532 tests) passed.

The maintainer reviewed PR #164 as a pass, then reopened it for a one-word fix: the README helper table gained close and doctor rows, and the CLI reference's session row now names close. The re-review passed and the story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-make-the-readme-docs-workflow-skill-and-diagrams-match-main.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-make-the-readme-docs-workflow-skill-and-diagrams-match-main-with-checks-that-fail-when-they-drift-flow-18/)
