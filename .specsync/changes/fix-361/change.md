---
id: fix-361
state: implementing
type: bug_fix
base_commit: 66b82f22a2eb775ece9ab823b4f2698137474a68
---

# Fix 361

## Intent

fix 361

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- While an installed skill file is byte-identical to a known earlier bundle, chough doctor reports it as skill_outdated, naming the file and chough skill update, not a decision, and chough next shows the line 'Skill files are outdated: run chough skill update' and skill_outdated as the count; a current, missing or edited file gives neither, and chough skill update clears both. chough status carries no_native_change once the session recorded it.

## No-spec Rationale

Not applicable
