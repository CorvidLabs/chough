---
id: fix-269-post
state: archived
type: bug_fix
base_commit: 8f3270abaedf56575c3afcc9999c43e6cf6cbd9b
---

# Fix 269 post

## Intent

fix 269 post

## Affected Canonical Specs

- `records`
- `history`

## Acceptance Criteria

- A derived journal post's body opens with the session's current objective, not the Goal's first paragraph, so after a second checkpoint --objective its title and body agree. At closeout, when the story's Goal paragraph is not the current objective, the archived story gets one line 'Objective at closeout: <current objective>' after the Goal paragraph; when it is, no line is added; no title, Goal or other text is rewritten.

## No-spec Rationale

Not applicable
