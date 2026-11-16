---
change: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag
artifact: context
---

# Context

Issue #249: when a human's relayed decision drives a native step, Chough passed SpecSync no relay, so `review.json` and approvals carried no words, place or recording agent. PR #261 fixed this by passing `--words`, `--where` and `--recorded-by` (SpecSync 6.3.0). corvid-agent's review of #261 then found that relay text starting with `-`, which Chough's own checkpoint and gate flags accept, failed in SpecSync's argument parser.

#261's change was finalized on its branch before main moved (#284 changed REQ-cli-059). SpecSync refuses to redefine an applied change, and #261's archive never reached main, so this change replaces it from current main: #261's code and tests, the `--flag=value` fix, and deltas written against today's REQ-cli-059 and REQ-records-008. #261 will be closed in favor of this PR.
