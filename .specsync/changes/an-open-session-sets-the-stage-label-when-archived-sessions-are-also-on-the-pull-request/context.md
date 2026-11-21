---
change: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request
artifact: context
---

# Context

A pull request that archives a finished session and starts a new one currently contributes both stages. The label script treats that as several stages and applies none. The user asked that the new change's stage be the one shown.

LIFE-32 is the criterion. REQ-cli-070 changes so an open session wins, and an archived story is used only when no open session remains. The keep-label change stays open at its own review and is not edited. Archived records are not edited. This change is not merged from here.

## Lessons

- 2026-11-21, verified: counting an archived session and a new open session as one set clears the stage label, so the script chooses the open session first. Source: the first run of `the_script_follows_an_open_session_ahead_of_an_archived_one`, which reported multiple distinct stages `done intent`. Limits: the optional GitHub label script. Two open sessions at different stages still apply no label.
