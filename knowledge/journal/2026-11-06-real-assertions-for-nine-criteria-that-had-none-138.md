---
id: 2026-11-06-real-assertions-for-nine-criteria-that-had-none-138
title: Real assertions for nine criteria that had none (#138)
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-06
time: 2026-11-06T14:09:05Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-06-give-real-assertions-to-the-nine-criteria-the-audit-found-with.md
  - .specsync/archive/changes/2026-11-06-give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138/
record: .chough/archive/sessions/2026-11-06-give-real-assertions-to-the-nine-criteria-the-audit-found-with.md
actor:
  - agent: claude-code
    terminal: t382
---
# Real assertions for nine criteria that had none (#138)

An audit found nine criteria with no selected assertion; this work (#138, plan C6) gave them real ones with a measure-report example script and eleven more assertions. The maintainer chose both recommended options: REQ-cli-029 is asserted in Chough's evidence model and roadmap text, and REQ-reporting-003's measurement criterion through the script plus a test.

Twelve cases passed on macOS, and a mutation round caught 11 of 11 mutations. The measurement test failed first, before the script existed.

Stage moves waited on a KNOW-29 history regression until the chough#141 fix merged into the branch. The maintainer approved the definition and the implementation review, both relayed rather than authenticated, and the story closed done on 2026-11-06.

Read more: [the story](../../.chough/archive/sessions/2026-11-06-give-real-assertions-to-the-nine-criteria-the-audit-found-with.md) · [the archived change](../../.specsync/archive/changes/2026-11-06-give-real-assertions-to-the-nine-criteria-the-audit-found-with-no-selected-assertion-138/)
