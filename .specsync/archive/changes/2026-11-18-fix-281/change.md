---
id: fix-281
state: archived
type: bug_fix
base_commit: 66efea4dd777077ef9bbd412cf0fac8f7ea93b41
---

# Fix 281

## Intent

fix 281

## Affected Canonical Specs

- `cli`
- `records`

## Acceptance Criteria

- With SpecSync configured and no native change recorded, chough checkpoint --no-native-change <reason> records the reason in the session; its gated stage moves are then allowed and labelled ungated:no-native-change-required, so it reaches done and is archived with outcome done, the story keeping the reason; an open human gate still refuses the move; a blank reason is refused; the flag is refused while a native change is recorded, and recording one removes the reason.

## No-spec Rationale

Not applicable
