---
id: 2026-11-10-move-two-192-test-cases-into-their-own-files-196
title: "Move two #192 test cases into their own files (#196)"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-10
time: 2026-11-10T16:03:25Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-09-chough-196-case-files.md
  - .specsync/archive/changes/2026-11-10-move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence/
record: .chough/archive/sessions/2026-11-09-chough-196-case-files.md
actor:
  - agent: claude-code
    terminal: t382
---
# Move two #192 test cases into their own files (#196)

Issue #196 moved two cases from #192 into dedicated per-change case files, keeping their IDs, definitions, bindings and evidence links unchanged, with no runtime, intent, policy or contract change.

All 400 case definitions kept their IDs and digests, with only the two file mappings changed. The case check passed, the unchanged-catalog check accepted the real catalog and rejected a missing and an altered case, and the knowledge index check passed. No Rust bindings, full suite or site build were run.

Acceptance was recorded under the maintainer's standing permission for bug fixes to run the full SpecSync lifecycle and squash-merge once CI is green and review threads are resolved. The story closed as done on 10 November 2026.

Read more: [the story](../../.chough/archive/sessions/2026-11-09-chough-196-case-files.md) · [the archived change](../../.specsync/archive/changes/2026-11-10-move-the-two-192-cases-into-dedicated-196-files-without-changing-definitions-or-evidence/)
