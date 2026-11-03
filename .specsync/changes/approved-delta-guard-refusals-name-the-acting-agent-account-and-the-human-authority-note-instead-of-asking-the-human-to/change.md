---
id: approved-delta-guard-refusals-name-the-acting-agent-account-and-the-human-authority-note-instead-of-asking-the-human-to
state: implementing
type: bug_fix
base_commit: 8300b584e6e76c676d3da2f21aab9eb7e89b311a
---

# Approved-delta guard refusals name the acting agent account and the human authority note instead of asking the human to approve under their name

## Intent

Approved-delta guard refusals name the acting agent account and the human authority note instead of asking the human to approve under their name

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Each approved-delta guard refusal that names a native approval says to record it as the acting agent account with the human authority source, covered scope and limits in the note, matching docs/cli.md, and no longer says to ask the human or to use their name as the actor
- The refusal labels, refused moves leaving records byte-identical, and the status-read guidance for other unverifiable comparisons are unchanged

## No-spec Rationale

Not applicable
