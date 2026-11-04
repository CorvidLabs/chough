## ADDED

### REQUIREMENT REQ-cli-036

Backfilled stage moves. `chough stage backfill --at <time> --commit <rev> --quote <text>` SHALL record a move to the next main stage that already happened. It SHALL apply every refusal `stage advance` applies, reading the destination's native prerequisite now, and SHALL record an `advance` transition at the given time with a backfill (REQ-history-005). It SHALL refuse with nothing recorded a time after now or before the latest transition, an unreadable latest transition, a commit that is not an ancestor of HEAD, a blank quote and release to done. It SHALL NOT record a human approval, review, consent or gate; the quote is evidence only. `timing` SHALL list backfilled moves separately, mark the segments they bound, report backfilled time per stage and keep those stages out of estimates. Trace: DRIVE-5, LIFE-7.

Acceptance Criteria
- A backfilled plan to build move records the given time, full commit SHA and quote, and `timing --json` lists it under `backfilled` and marks its segments.
- A draft native change, an open awaiting-human gate, a future or out-of-order time, an unknown or unreachable commit, a blank quote and a side stage refuse with the checkpoint and log unchanged.
- No approval, review, consent or gate record is written.
