---
change: 0-6-1-chough-change-ship-prints-a-short-text-summary-after-specsync-s-output-with-its-full-report-under-json
artifact: context
---

# Context

In Chough 0.6.0's fourth fresh-eyes round (t38n, F6), `chough change ship` printed
SpecSync's text, then a long JSON checkpoint and relayed decision, then the move
summary, all interleaved. Stage moves already print a short summary with their full
report under `--json` (REQ-cli-008). The bug is recorded as
`.chough/bugs/2026-11-12-chough-change-ship-prints-specsync-s-text-a-long-json.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)" for
0.6.1 as six small PRs; this is one.
