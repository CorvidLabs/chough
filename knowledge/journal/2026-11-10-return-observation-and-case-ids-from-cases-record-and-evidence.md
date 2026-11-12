---
id: 2026-11-10-return-observation-and-case-ids-from-cases-record-and-evidence
title: Return observation and case IDs from cases record and evidence add
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-10
time: 2026-11-10T23:17:30Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-07-return-the-observation-id-from-cases-record-and-evidence-add.md
  - .specsync/archive/changes/2026-11-10-return-the-observation-id-from-cases-record-and-evidence-add-issue-60/
record: .chough/archive/sessions/2026-11-07-return-the-observation-id-from-cases-record-and-evidence-add.md
actor:
  - agent: claude-code
    terminal: t382
---
# Return observation and case IDs from cases record and evidence add

Chough's cases record and evidence add now return the observation ID they record, and cases record also returns the case ID, fixing issue 60. It reused FLOW-7, FLOW-18 and LIFE-4 and, after SpecSync refused a scope with no spec, amended REQ-cli-011 and REQ-cli-017 in one delta; broader output changes were ruled out of scope.

The two new tests, one for a manual case record and one for evidence add, each failed first and then passed twice.

Scope approval and the implementation review came under the maintainer's merge rule for bug fixes, relayed rather than authenticated. The work was PR #204; the native change was archived and the story closed done on 2026-11-10.

Read more: [the story](../../.chough/archive/sessions/2026-11-07-return-the-observation-id-from-cases-record-and-evidence-add.md) · [the archived change](../../.specsync/archive/changes/2026-11-10-return-the-observation-id-from-cases-record-and-evidence-add-issue-60/)
