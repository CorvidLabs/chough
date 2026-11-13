---
change: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve
artifact: requirements
---

# Requirements

REQ-records-008 (modified): the native approve and review Chough runs for a human's
gate decision pass the relay's words, place and recording agent as `--words`,
`--where` and `--recorded-by`, so SpecSync's approval and review record the relayed
decision; the approval note is unchanged. REQ-cli-059 (modified): the review
`chough change ship` runs carries the same relay.
