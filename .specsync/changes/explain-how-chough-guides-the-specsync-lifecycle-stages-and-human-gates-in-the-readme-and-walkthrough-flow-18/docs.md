---
change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
artifact: docs
---

# Docs

- README.md: a short "How Chough works" section near the top that answers Leif's questions directly — Chough does not auto-drive all of SpecSync; the current agent runs HI, SpecSync and Fledge itself while following the skill; stages advance only when the agent runs `chough stage`, gated by native status; the human gates that remain are intent approval, design approval (visual changes only), scope approval when authority is missing, implementation acceptance/review, merge, and separately-authorized release/deploy. State precisely that `chough change new/answer/check/status` are pass-through wrappers and only `change ship` also closes the review gate and moves the session to done, and that `chough next` is routing guidance, not an unattended runner. Rework "The loop" into the short goal → agent work → human decision → tested PR loop with copyable human start/resume examples and a focused agent resume recipe; keep the command catalog in "What the helper does" and docs/cli.md.
- docs/walkthrough.md: name, at each stage, who acts (agent) and who decides (human), matching the README; show that routine in-scope build/test/stage movement is agent work under the recorded authorization, not a new permission question per tool call.
- docs/lifecycle.md: state the SpecSync lifecycle ownership and integration boundaries as they are — native approvals and reviews are recorded claims with a real human source, archive-first CI blocks other jobs while a change is open, review precedes ship and a later commit makes the review stale; Chough reads native state for gates and never authenticates people or approves silently.
- Automation gaps are listed as open product decisions (issue #76 items 1/3/4, no auto-driver), not papered over by wording.
