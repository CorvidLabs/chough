---
id: fix-347
state: implementing
type: bug_fix
base_commit: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
---

# Fix 347

## Intent

fix 347

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- A new bug's Log names each piece of linked evidence as a Markdown link relative to the bug's file that opens the open session's entry, bug update and the automatic failure capture do the same, and the front matter keeps the stored form. After that session is archived, chough doctor reports the bug as stale_bug_links, naming the entry and the story, and --fix appends one Log entry linking the story with every earlier Log line unchanged; the bug is then not reported again.

## No-spec Rationale

Not applicable
