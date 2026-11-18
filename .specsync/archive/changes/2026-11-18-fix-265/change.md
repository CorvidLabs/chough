---
id: fix-265
state: archived
type: bug_fix
base_commit: 0b7ac77e7acf916d40a464035b1fa22c0b503d29
---

# Fix 265

## Intent

fix 265

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- A record write whose body holds a Git conflict marker, a line starting <<<<<<<, a line starting >>>>>>>, or a ======= line between them, is refused as refused:conflict-markers naming the file, the line and the marker, with every file unchanged; a ======= line outside such a block, as a Markdown heading underline, is not a marker. chough doctor reports each committed record (not other tracked files) whose body holds one as a conflict_markers finding naming the file, the line and the marker, with no fix, and exits 1. chough migrate checks every file it would write before writing any and refuses, writing nothing, when one would hold a conflict marker.

## No-spec Rationale

Not applicable
