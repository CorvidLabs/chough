---
change: 0-6-1-at-the-400-observation-cap-every-write-that-adds-an-observation-refuses-before-writing-anything-and-names-chough
artifact: context
---

# Context

Preparing Chough 0.6.0's review, the release session reached REQ-cli-032's bound of 400
observation references. From there `chough cases run` executed each case, wrote its
evidence entry, then refused with `too many observation (limit 400)`: 30 passing
entries were left unreferenced, and the message named no recovery, though REQ-cli-032
says such a refusal leaves every record byte-identical and names `chough session shelve
<id>`. `chough next` at accept then counted the unreferenced newer entries as the
cases' latest runs. The bug is recorded as
`.chough/bugs/2026-11-13-at-the-400-observation-cap-cases-run-writes-its-evidence-entry.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)"
for 0.6.1 as six small PRs; this one and the leak check go first.
