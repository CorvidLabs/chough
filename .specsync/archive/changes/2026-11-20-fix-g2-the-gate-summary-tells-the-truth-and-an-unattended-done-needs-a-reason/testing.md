---
change: fix-g2-the-gate-summary-tells-the-truth-and-an-unattended-done-needs-a-reason
artifact: testing
---

# Testing

Each failing on main first, as listed in tasks.md. Then every verify step, the cases on
macOS, and, since #390 adds a refusal (rule 3), the tester's run on the real
repositories: no project that uses SpecSync gets the new refusal or warning (they are
gated); in a copy of each project without SpecSync, `stage advance --to done` from
intent refuses naming `--reason`, and with it moves and records the reason; the gate
summary's Tests line on each real open session counts at least what main counts.
