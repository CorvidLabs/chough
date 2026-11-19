---
change: fix-339-fence-pass
artifact: context
---

# Context

A follow-up to fix-339 (#356), from Leif's P2 review comment on #356: when a code fence opener has no later matching close, `fenced` walked every remaining line looking for one, and the outer loop repeated that walk for the next opener. Repeated unclosed fence openers therefore made the scan O(n²) with no conflict at all. His measurement: the optimized CLI took 1.61 s to checkpoint a 128,732-byte session with 16,000 openers, and more than 2.0 s at 256,732 bytes with 32,000, far below the 1 MiB session bound. Every record write and doctor's two scans repeat the cost.

The standing rule Leif gave for 2026-11-18, "Yes, for today", covers a real gap his review finds in an already-approved PR: a small follow-up change inside the approved scope, a tester's glance, then hc t4c resolves the thread.
