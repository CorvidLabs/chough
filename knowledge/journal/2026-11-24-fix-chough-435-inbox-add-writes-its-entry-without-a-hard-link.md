---
id: 2026-11-24-fix-chough-435-inbox-add-writes-its-entry-without-a-hard-link
title: "Fix chough#435: inbox add writes its entry without a hard link"
type: journal
tags: [chough/journal, chough/shipped]
date: 2026-11-24
time: 2026-11-24T14:12:32Z
kind: shipped
links:
  - .chough/archive/sessions/2026-11-23-fix-435.md
  - .specsync/archive/changes/2026-11-24-fix-435/
record: .chough/archive/sessions/2026-11-23-fix-435.md
actor:
  - terminal: t14z
---
# Fix chough#435: inbox add writes its entry without a hard link

Fix chough#435: inbox add writes its entry without a hard link, whole or not at all, like records::write_new_bytes

Why: 0xLeif decided "Approve the scope of fix-435": "I approve" (pi chat t14z (CorvidLabs orchestrator session), 2026-11-23 about 03:45 UTC, after the one-screen stack summary for the six-change overnight branch); recorded by pi-t14z; a relayed claim, not authenticated; ran specsync change approve fix-435 as 0xLeif

Verified: Ran cargo test --locked --test no_hard_links -- --ignored --exact new_records_are_written_on_a_volume_without_hard_links for TC-records-inbox-add-without-hard-links: passed (exit 0). - 368460f1101f; limits: outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected; inherited environment variables were not recorded

Read more: [the story](../../.chough/archive/sessions/2026-11-23-fix-435.md) · [the archived change](../../.specsync/archive/changes/2026-11-24-fix-435/)
