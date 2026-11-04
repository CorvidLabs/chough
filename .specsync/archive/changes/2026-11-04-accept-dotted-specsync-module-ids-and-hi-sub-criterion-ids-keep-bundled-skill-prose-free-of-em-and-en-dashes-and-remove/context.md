---
change: accept-dotted-specsync-module-ids-and-hi-sub-criterion-ids-keep-bundled-skill-prose-free-of-em-and-en-dashes-and-remove
artifact: context
---

# Context

Leif's 2026-11-03 5:03 PM MST message (t811u) lists what stopped Chough from driving Raven's work that day. Item 1: Raven's checkpoint names SpecSync modules `chat.service`, `room.service` and `linked-wallet.service`, which Chough refused ("invalid session module name"), so `chough next` refused the checkpoint and focused context dropped Raven's core services; `chough checkpoint --criterion UX-12.a` failed with "invalid criterion ID" although hi accepts `CHAT-34.a` and `BOTS-4.g`. Issues 56 and 57 report the same. Item 7 asks for no em dashes in bundled skill templates with a check (Raven BUG-0001, seen with 0.1.0) and a way to correct a `checkpoint --completed` entry without editing session.json by hand.

Chough used one `valid_name` rule (ASCII letters, digits, `-`, `_`) for every ID. The bundled templates on main 8298e5a already contain no U+2013 or U+2014, so the dash part only adds the check that keeps them out.

Intent: hi/drive.md DRIVE-1, DRIVE-7 and DRIVE-8, quoting t811u. Authority: Leif's 7:49 AM MST standing Chough maintainer instruction and his t811u request; corvid-agent is the acting account.
