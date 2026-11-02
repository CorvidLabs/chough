---
change: agents-state-the-recorded-handoff-before-their-first-edit-the-skill-s-resume-steps-ask-a-fresh-agent-to-say-the-goal
artifact: context
---

# Context

The learning handoff change (PR #35, merged as 118a895) added the decisions, last
failure and lessons blocks to `chough next`. Its cold-start proof (criterion 9)
failed. On the frozen seeded repository, Kimi Code CLI 0.41.0 ran `chough next`, saw
the decisions, the failure with BUG-0001's status and the lesson pointer, and read
`context.md` and `session.json` before its first edit, but none of its own messages
before that edit stated those facts. 0xLeif graded it Fail (P1 to P3 not met, P4 to
P6 met; observation handoff-cold-start-change-kimi-1-grade). The baseline on main
fb6e487 failed P2 and P3 the same way.

The lesson recorded in specs/cli/context.md names the untested hypothesis this
change tests: the helper puts the facts in front of the agent, but the skill never
asks it to state them before acting. 0xLeif accepted PR #35 as is, with criterion 9
failed, and asked for this follow-up as its own change.

For a session picking this up:

- The seed and rubric in docs/exercises/handoff-cold-start/ are frozen and hashed
  (observation handoff-cold-start-frozen-pair-1). Do not edit them; a change voids
  every run.
- Kimi needs rune to drive it, shows a folder trust dialog in a new directory, and
  has a 5-hour usage quota that stopped the earlier real-repository run after 8 tool
  calls. 0xLeif starts Kimi runs himself; agents prepare the scripts.
- Two sessions closed out PR #35 in parallel on one branch, and the second archive
  made SpecSync report corrupt history until the branch was repaired. Only one
  session closes out this change.
