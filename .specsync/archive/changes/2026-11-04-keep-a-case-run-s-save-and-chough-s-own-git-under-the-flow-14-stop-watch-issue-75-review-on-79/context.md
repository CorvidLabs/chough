---
change: keep-a-case-run-s-save-and-chough-s-own-git-under-the-flow-14-stop-watch-issue-75-review-on-79
artifact: context
---

# Context

The Chough bot's review 5762369485 on #79 found that `cases run` wrote its artifact and took its Git snapshot after the stop watch ended, so a SIGTERM in that window left an OBS log with no observation (23 of 121 trials), and Chough's own Git commands survived a normal kill. The requirement and the watch itself are in stop-what-chough-started-when-chough-is-stopped-issue-75 (src/records.rs). This change is the call-site wiring in src/cases.rs and src/evidence.rs.
