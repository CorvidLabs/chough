---
kind: bug
id: 2026-11-13-chough-change-ship-s-native-review-drops-the-relay-review-json
aliases: []
title: "chough change ship's native review drops the relay: review.json names the reviewer with no words, place or recorder"
status: verified
classification: requirement-gap
scope: in-scope
impact: "SpecSync's own review record cannot tell a relayed claim from a direct review; the relay is only in Chough's gate decision. The approve step has the same shape: it carries the relay only inside --note text, not in SpecSync 6.3.0's structured --words, --where and --recorded-by"
criteria: []
requirements: [REQ-records-008, REQ-cli-059]
cases: []
evidence:
  - 2026-11-13-review-relay/2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review
  - 2026-11-13-review-relay/2026-11-13-tc-records-approval-as-the-human
  - 2026-11-13-review-relay/2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review-567cea
  - 2026-11-13-review-relay/2026-11-13-tc-records-approval-as-the-human-29ac02
receipts: []
environment: local-macos-arm64
revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
found_by: "claude-code (hc t382) (t370 (coordinator), from t384's review of chough#242 and #244)"
actor:
  - agent: claude-code
    terminal: t382
related: []
created: 2026-11-13T03:40:50Z
updated: 2026-11-13T07:46:26Z
---
# chough change ship's native review drops the relay: review.json names the reviewer with no words, place or recorder

**What happened:** After a relayed review through chough change ship, SpecSync's review.json names the reviewer (0xLeif) and the verdict, but not the human's words, where they said them, or the agent that recorded them
Actual: src/humans.rs Step::Review runs only specsync change review <id> --reviewer <specsync_actor> --verdict <verdict>, as REQ-records-008 currently words it; the 0.6.0 release's archived review.json has reviewer 0xLeif and no relay fields

**Steps:** Trigger: chough change ship <id> --close-gate <gate> --decided-by Leif --words <words> --where <where> --recorded-by <agent>, where the gate drives review and no scoped review is current
1. Read src/humans.rs Step::arguments for Step::Review: only --reviewer and --verdict are passed
2. Run specsync change review --help (6.3.0): --words, --where and --recorded-by are listed
3. Read any 0.6.x archived review.json, e.g. the 0.6.0 release change's: no relay fields

**Expected:** Chough passes the relay to SpecSync's review as --words, --where and --recorded-by (SpecSync 6.3.0's change review accepts all three), so review.json carries who relayed what

**Why it matters:** SpecSync's own review record cannot tell a relayed claim from a direct review; the relay is only in Chough's gate decision. The approve step has the same shape: it carries the relay only inside --note text, not in SpecSync 6.3.0's structured --words, --where and --recorded-by

**Fix:** fd6efe76 (t38b's ca3b0efd): src/humans.rs's Step::arguments appends --words, --where and --recorded-by from the relay to specsync change approve and change review; gate closes, chough gate record and chough change ship pass the relay they already checked

## Log

- 2026-11-13T03:40:50Z · Reported (t370 (coordinator), from t384's review of chough#242 and #244) · by claude-code (hc t382)
- 2026-11-13T03:40:50Z · Added despite similar reports: The verified 2026-11-12 ship bugs fixed Chough's own close report and gate-closed relay; this one is the relay missing from SpecSync's review.json, which Chough's native call never passes · by claude-code (hc t382)
- 2026-11-13T03:46:38Z · Tracked as GitHub #249; Leif chose reviews and approvals at 2026-11-13T03:44Z (relayed by t370) · by claude-code (hc t382)
- 2026-11-13T07:44:49Z · evidence: 2026-11-13-review-relay/2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review, 2026-11-13-review-relay/2026-11-13-tc-records-approval-as-the-human. Red at da86ddcb (t38b's tests): the ship's review gets only --reviewer and --verdict, and the approve and review closes carry no --words, --where or --recorded-by · by claude-code (hc t382)
- 2026-11-13T07:46:26Z · status: open -> verified; evidence: 2026-11-13-review-relay/2026-11-13-tc-cli-ship-passes-the-relay-to-the-native-review-567cea, 2026-11-13-review-relay/2026-11-13-tc-records-approval-as-the-human-29ac02; fix: fd6efe76 (t38b's ca3b0efd): src/humans.rs's Step::arguments appends --words, --where and --recorded-by from the relay to specsync change approve and change review; gate closes, chough gate record and chough change ship pass the relay they already checked. Green at fd6efe76: the cases pass · by claude-code (hc t382)
