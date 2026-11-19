---
id: 2026-11-19-chough-change-ready-a-read-only-report-before-approval
title: "chough change ready: a read-only report before approval"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-19
time: 2026-11-19T15:39:11Z
kind: shipped
links:
  - 311
  - .chough/archive/sessions/2026-11-14-readiness-report-before-change-approval.md
  - .specsync/archive/changes/2026-11-19-readiness-report-before-change-approval/
record: .chough/archive/sessions/2026-11-14-readiness-report-before-change-approval.md
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t4b
---
# chough change ready: a read-only report before approval

Adds chough change ready <id> (REQ-cli-067, DRIVE-29): one read-only report of what is still missing before a change is approved, each finding naming the command that fixes it; it exits 1 when anything is missing and writes nothing.

Verified: all eleven verify steps on macOS and Linux and its ten cases on macOS at 75b6bf6e; 890 tests in corvid-agent's run; tester2's harness against real SpecSync 6.3.0, identical on d3ab8590 and 75b6bf6e; the hash cache left byte-identical or absent, a replacement by another process left alone.

Known gaps go to #406: SpecSync's dry run of the approval (spec-sync #1091) will replace this report's own copy of the owner rule; #307 is folded there.

Read more: the pull request 311 · [the story](../../.chough/archive/sessions/2026-11-14-readiness-report-before-change-approval.md) · [the archived change](../../.specsync/archive/changes/2026-11-19-readiness-report-before-change-approval/)
