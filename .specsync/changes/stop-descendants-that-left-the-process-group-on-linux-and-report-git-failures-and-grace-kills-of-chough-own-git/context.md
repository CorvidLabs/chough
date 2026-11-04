---
change: stop-descendants-that-left-the-process-group-on-linux-and-report-git-failures-and-grace-kills-of-chough-own-git
artifact: context
---

# Context

Issue https://github.com/CorvidLabs/chough/issues/81 collects follow-ups from the Chough
bot reviews of #79 (FLOW-14):

1. A process that calls `setsid` leaves the process group Chough stops, so it outlives a
   stop. Reproduced on 2026-11-04 (Claude Code session): a case binding whose child ran
   `POSIX::setsid(); sleep 90` kept running after SIGINT and SIGTERM ended Chough in
   about 50 ms, for its full 90 s.
2. `head_revision` turns a failed `git rev-parse HEAD` into `None` without saying so.
3. When the grace kill after a Git command fires (a descendant held Git's output open),
   nothing says so.
4. A background process left by SpecSync's own Git held a native read's pipe about
   120 s in a review harness; the issue asks for a repro first.

FLOW-14 ("everything it started stops too") covers item 1; LIFE-7 covers reporting
items 2 and 3. Item 4 is investigated in this change and reported on the PR; it changes
code only if the repro shows a Chough defect.

Authority: Leif's 2026-11-04 standing approval for bounded Chough bug fixes and his
brief to fix issues 81 to 88 one PR each. Linux only, as his brief says: other Unix
systems keep the process-group stop.
