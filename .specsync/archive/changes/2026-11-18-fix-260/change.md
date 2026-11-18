---
id: fix-260
state: archived
type: bug_fix
base_commit: 8abb5bb89b5244e72327e1ea6abfc7e66634ebe1
---

# Fix 260

## Intent

fix 260

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- When the write of a new bug record fails part way, as under a file size limit, chough bug add exits non-zero, no file is left in .chough/bugs/, and chough bug list, chough status and chough next still exit 0.

## No-spec Rationale

REQ-records-001 already says every record Chough writes is one Markdown file that starts with front matter; a create-new write that fails part way left a file that is not one. Only the writer and its test change.
