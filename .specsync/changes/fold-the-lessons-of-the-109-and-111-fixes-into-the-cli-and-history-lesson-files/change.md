---
id: fold-the-lessons-of-the-109-and-111-fixes-into-the-cli-and-history-lesson-files
state: implementing
type: documentation
base_commit: 4143558d3f99af0be75ed53eeb6dd4e77b6ef395
---

# Fold the lessons of the 109 and 111 fixes into the cli and history lesson files

## Intent

Fold the lessons of the 109 and 111 fixes into the cli and history lesson files

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- The lessons of the #109 and #111 fixes are in specs/cli/context.md and specs/history/context.md in the existing lesson format, each citing a source that resolves, and fledge lanes run verify passes on Linux.

## No-spec Rationale

Lessons only, in the modules context.md files; no contract changes.
