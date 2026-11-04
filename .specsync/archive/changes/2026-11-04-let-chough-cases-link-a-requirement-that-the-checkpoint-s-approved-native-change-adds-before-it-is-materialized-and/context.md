---
change: let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and
artifact: context
---

# Context

On Raven on 2026-11-03, `chough cases check` refused a case linking a requirement that existed only in the approved, not yet materialized SpecSync delta (Raven BUG-0004). SpecSync materializes a delta only in `specsync change check`, which refuses while `tasks.md` has an unchecked box, so cases could be linked only after the implementation was finished. Failing-first evidence therefore never went through `chough cases`, and the test stage passed in 3 seconds with no cases recorded (Leif's brief t811u item 6).

Leif's brief offers two options; the parent relayed the 6:03 PM MST decision to make the test stage real: cases accept requirement IDs from the active approved change, and the test stage documents how failing-first evidence is linked. Stage gates do not change.
