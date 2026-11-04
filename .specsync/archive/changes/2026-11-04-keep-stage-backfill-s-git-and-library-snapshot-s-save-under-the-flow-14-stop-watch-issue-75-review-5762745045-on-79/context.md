---
change: keep-stage-backfill-s-git-and-library-snapshot-s-save-under-the-flow-14-stop-watch-issue-75-review-5762745045-on-79
artifact: context
---

# Context

The Chough bot's review 5762745045 on #79 found two more FLOW-14 gaps. `stage backfill` ran Git with a bare `Command::output`, so a normal kill left that Git running. `library snapshot` copied its files before it read the Git HEAD for the manifest, so a stop during that read left copied files with no manifest. The requirement and the watch itself are in stop-what-chough-started-when-chough-is-stopped-issue-75 (src/records.rs). This change is the call-site wiring in src/stage.rs and src/library.rs.
