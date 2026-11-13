---
change: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve
artifact: tasks
---

# Tasks

- [ ] Failing tests: the specsync stub records its arguments, and an approve close, a review close and a ship's review each carry the relay's words, place and recording agent.
- [ ] The fix in `src/humans.rs`, `src/checkpoint.rs` and `src/change.rs`, with `docs/cli.md` and the CHANGELOG.
- [ ] Red then green recorded through Chough, and the full local suite.

Human gates and delivery, as plain text: Leif's review through `chough change ship`, then CI, then his merge.
