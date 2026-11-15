---
change: chough-s-records-join-a-receipt-s-claim-to-its-evidence-with-a-dash-not-an-em-dash-and-still-read-records-written-with
artifact: design
---

# Design

`receipt_line` (`src/sessions.rs`) and the gate summary's `Change:` line (`src/gates.rs`) join with ` - `. `claim_of` (`src/journal.rs`) cuts a line at its join: the first ` - ` or ` — ` that is followed by the backticked command or the 12 fingerprint digits `receipt_line` writes after it. A journal post therefore reads a story archived before this change without rewriting it, and a ` - ` inside a claim stays in the claim. No visual or interaction change.
