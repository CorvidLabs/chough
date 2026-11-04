---
change: scale-independent-archive-authentication-workers-with-the-runner-s-cpus-so-the-archive-check-fits-its-unchanged-budget
artifact: context
---

# Context

## Human authority and limits

Acting agent: corvid-agent. Intent: Leif in chat, 2026-11-03 6:30 PM MST (t813u), "I've had reports that sometimes SpecSync archive takes like three minutes to run" and "make sure that the whole suite runs fast and is good", recorded as DRIVE-10. The parent agent relayed the decision for this PR at 9:31 PM MST: make the check itself faster without weakening it, and raise no timeout. This does not authorize lowering a gate, a longer timeout, rewriting recorded proof, release or deployment. Closeout and merge need tests, independent review and green exact-head checks.

## Problem

The hosted SpecSync Archive job on #71 (head f646bc5, 63 archives) was cancelled at its unchanged 10-minute limit twice. On main, #68's run took 8 min 39 s. Locally the same script passes on the #71 head in 9 min 56 s with two workers. Each native 6.1.2 `change status <archived id>` takes about 19 s (user 9.5 s, sys 5.7 s), and the check reads every archive with a fixed two workers (from #46), so the wall time grows with the archive count. ubuntu-latest runners for public repositories have 4 vCPUs.

## Approach

Use one worker per CPU that `os.cpu_count()` reports, at least 2 and at most 8, for the independent read-only status reads of the fixed current checkout. Everything else is unchanged: history replay stays serial, every obligation and predicate stays, and every submitted read is joined before the final fingerprint.

## Ruled out

- A SpecSync build with spec-sync#840's fast archived reads: #840 is still open and 6.1.2 is the latest release; building from an unmerged PR is not allowed.
- Raising the job timeout.
- Skipping archives already authenticated on the base: deferred, since it changes what this run authenticates.
