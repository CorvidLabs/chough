---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: requirements
---

# Requirements

- ADDED REQ-cli-061 (`deltas/cli.md`): `chough gate list` gives numbered one-screen summaries with digests, saved untracked. `chough gate record` closes only listed gates with one relay. Every refusal names its recovery.
- MODIFIED REQ-cli-059: `chough change ship` accepts `--list` and `--gate` in place of `--close-gate`.
- ADDED REQ-history-014 (`deltas/history.md`): a relay may carry `summary_digest` and `list`, both or neither.

Trace: FLOW-19, FLOW-20, FLOW-16 (as amended), FLOW-10.
