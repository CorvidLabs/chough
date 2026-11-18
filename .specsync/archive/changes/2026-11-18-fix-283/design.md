---
change: fix-283
artifact: design
---

# Design

In `cases::check_links`, the `status == "retired"` early return moves above the `fledge-task` existence check. A retired case's structural checks, run elsewhere when the file loads, are unchanged. Its links and its task are no longer checked. An active case naming a missing task is refused as before.
