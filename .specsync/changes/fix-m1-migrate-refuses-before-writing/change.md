---
id: fix-m1-migrate-refuses-before-writing
state: approved
type: bug_fix
base_commit: 9d260310bca8553a0203e5ef90bc8372b4e15aae
---

# Fix m1 migrate refuses before writing

## Intent

fix m1 migrate refuses before writing

## Affected Canonical Specs

- `records`
- `cli`

## Acceptance Criteria

- doctor --fix after an in-place migration keeps a pointer that names an open session by alias; any entry at a migration target or receipt name counts as taken, the preview never waits and says the apply would refuse, and refusals name relative paths with nothing written; a returning 0.5.x record whose ID is already held is refused by name, and the generated index and feed are rebuilt in place; of several applies of one digest only one applies and records its consent

## No-spec Rationale

Not applicable
