---
id: fix-394-a-code-fence-that-never-closes-hides-no-section
state: implementing
type: bug_fix
base_commit: 056cbb8029f6535ff74f866a0523893916f682db
---

# Fix 394: a code fence that never closes hides no section

## Intent

Fix 394: a code fence that never closes hides no section

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- A session whose body holds a code fence that never closes saves any number of times with each section once and chough next reads its decisions and questions, a closed fence still hides a heading it quotes, and two branches that each add a Log entry to a bug whose write-up holds an unclosed fence merge with both entries and no hand-resolved conflict.

## No-spec Rationale

Not applicable
