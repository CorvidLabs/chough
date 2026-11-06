---
change: chough-doctor-finds-and-fixes-the-chough-debt-that-stops-a-fresh-worktree-from-entering-chough-drive-17
artifact: context
---

# Context

Leif asked for a `chough doctor` on 2026-11-06, through the orc. It is one command that finds and fixes repo debt, the debt spec-sync #941 cleaned up by hand: twelve leftover sessions on main, a legacy `session.json` that needed adopting, and stale approval gates on archived changes. A fresh worktree could not enter Chough until that was done. He captured DRIVE-17 with "Capture + both defaults": with no flag the doctor only reports, and `--fix` without `--reason` applies only the safe fixes. #163's finished rule and `session close` (#166) are reused here, and #166 is not held for this change.
