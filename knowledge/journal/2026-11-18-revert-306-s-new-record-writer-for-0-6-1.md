---
id: 2026-11-18-revert-306-s-new-record-writer-for-0-6-1
title: "Revert #306's new-record writer for 0.6.1"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-18
time: 2026-11-18T16:46:28Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-18-revert-306.md
  - .specsync/archive/changes/2026-11-18-revert-306/
record: .chough/archive/sessions/2026-11-18-revert-306.md
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
# Revert #306's new-record writer for 0.6.1

#306's writer linked a temporary file to a new record's name. It failed on file systems without hard links (#313) and a concurrent writer swept its temporary files. The create-new writer is 0.6.0's again; #305's reader and #308's lock are kept. The verify lane passed on macOS after main (f8029cb) was merged in, the tester's parallel repro passed 3 of 3, and one hand run on exFAT passed. Linux was not run locally. #260 is open again for 0.6.2.

Read more: [the story](../../.chough/archive/sessions/2026-11-18-revert-306.md) · [the archived change](../../.specsync/archive/changes/2026-11-18-revert-306/)
