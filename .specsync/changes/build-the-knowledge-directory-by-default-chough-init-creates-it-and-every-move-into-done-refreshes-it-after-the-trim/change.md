---
id: build-the-knowledge-directory-by-default-chough-init-creates-it-and-every-move-into-done-refreshes-it-after-the-trim
state: implementing
type: bug_fix
base_commit: f7e576410539c2922330afd773b64fa5df4d9636
---

# Build the knowledge directory by default: chough init creates it and every move into done refreshes it after the trim; fold the 2026-11-04 lessons into the module lesson sources

## Intent

Build the knowledge directory by default: chough init creates it and every move into done refreshes it after the trim; fold the 2026-11-04 lessons into the module lesson sources

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With the librarian on, chough init creates knowledge/ with README.md, INDEX.md, briefs/ and one entry per archived change, preserves an existing folder, and still succeeds writing no knowledge file when the index cannot be bounded. stage advance into done regenerates the index and entries after the trim whether or not anything was trimmed, reporting applied.librarian_index refreshed, skipped with the librarian off, or refused with its reason, and never undoes the move. Chough own repository commits its knowledge/ folder, and the cli and history lesson sources gain the supported 2026-11-04 lessons with certainty labels. The new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
