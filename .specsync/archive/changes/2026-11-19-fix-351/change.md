---
id: fix-351
state: archived
type: bug_fix
base_commit: 235c3bfb063e835f67c4aae12f873ceac4074be7
---

# Fix 351

## Intent

fix 351

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With SpecSync configured and no native change recorded, a refused stage move and chough next, as JSON and as text, name both chough checkpoint --native-change <id> and chough checkpoint --no-native-change "<reason>"; next's command stays specsync change new; the workflow skill's lifecycle reference explains the flag in the canonical and the installed copy.

## No-spec Rationale

Not applicable
