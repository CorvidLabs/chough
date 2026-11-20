---
id: bug-add-leaves-the-session
state: archived
type: bug_fix
base_commit: 056cbb8029f6535ff74f866a0523893916f682db
---

# Bug add leaves the session

## Intent

bug add leaves the session

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- A bug added while a session is active, with or without --observation, records that session as its session key and leaves the session's record byte-identical; chough checkpoint and chough context list it after the bugs the checkpoint holds, the archived story names it, and a blocking one stops the session (#382, #395). Two branches that each add a bug in the same session merge without a conflict and the merged checkpoint lists both (#382). bug add takes about the same time at 400 observations as at 3 (#395), and with 200 bug links held it still files the bug in the session. A bug updated with --observation naming an entry of the active session joins its checkpoint unless it was filed in that session. A bug whose session key names an archived session or one that no longer exists is one of no open session's bugs: no checkpoint, context or blocking check lists it, and no command reports an error for it.

## No-spec Rationale

Not applicable
