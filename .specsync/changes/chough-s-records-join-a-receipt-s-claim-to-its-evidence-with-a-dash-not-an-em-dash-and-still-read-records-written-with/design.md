---
change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
artifact: design
---

# Design

`receipt_line` (`src/sessions.rs`) and the gate summary's `Change:` line (`src/gates.rs`) join with ` - `. `claim_of` (`src/journal.rs`) cuts a line at the first ` - ` or ` — `, whichever comes first, so a journal post still reads a story archived before this change without rewriting it.

Known limit: a claim that itself holds ` - ` is cut there in the post's `Verified:` line, as a claim holding ` — ` already was. The story itself is unchanged. No visual or interaction change.
