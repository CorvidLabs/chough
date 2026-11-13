---
id: fix-249-chough-passes-a-relayed-decision-s-words-place-and-recording-agent-to-specsync-s-review-and-approve
state: approved
type: bug_fix
base_commit: 338bbae296e3a7f9b00f57018b46ab8f27b932f8
---

# Fix #249: chough passes a relayed decision's words, place and recording agent to SpecSync's review and approve

## Intent

Fix #249: chough passes a relayed decision's words, place and recording agent to SpecSync's review and approve

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- Closing a gate that drives approval with a human's relay runs specsync change approve with the configured actor, the note, and the relay's words, place and recording agent as --words, --where and --recorded-by (red then green)
- Closing a review gate, through chough checkpoint --close-gate or chough change ship, runs specsync change review --reviewer with the verdict and the relay as --words, --where and --recorded-by, so SpecSync's review record carries the relayed decision (red then green)

## No-spec Rationale

Not applicable
