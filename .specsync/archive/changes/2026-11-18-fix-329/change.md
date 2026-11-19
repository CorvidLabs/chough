---
id: fix-329
state: archived
type: bug_fix
base_commit: 275fe4dae6d7a083751186bee89627af9f8140fe
---

# Fix 329

## Intent

fix 329

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough bug add while a session is active puts the new bug on that session's bugs list, with or without --observation, so a blocking bug filed with nothing linked stops that session leaving test and its story names it. A bug added with no session active is on no session's bugs list and stops no session, and chough next names it as not blocking that session. chough bug update with --observation joins the active session only when the entry it names is that session's, as before.

## No-spec Rationale

Not applicable
