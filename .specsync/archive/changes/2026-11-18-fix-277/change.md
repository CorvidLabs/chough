---
id: fix-277
state: archived
type: bug_fix
base_commit: 9dd28de28015ce6f873e237044cc9650e74a3712
---

# Fix 277

## Intent

fix 277

## Affected Canonical Specs

- `cli`
- `records`

## Acceptance Criteria

- A migrated bug's Log names each carried observation as a Markdown link, relative to the bug's file, to the open session's evidence entry or to the archived story that holds its receipt; every such link opens a file; a receipt only the bug holds is named as the bug's own; a removed observation keeps its 0.5.x ID. The bug's front matter is unchanged.

## No-spec Rationale

REQ-records-009 already says each bug becomes a Markdown bug and each carried observation becomes an entry or a receipt. The Log lines of a migrated bug named those observations by bare 0.5.x ID; they now link the file that holds each. No requirement text changes.
