---
id: doctor-fix-safety
state: draft
type: bug_fix
base_commit: 6936f88f47a2c69f9febf15822678f1d804ff027
---

# Doctor fix safety

## Intent

doctor fix safety

## Affected Canonical Specs

- `history`
- `cli`
- `records`

## Acceptance Criteria

- doctor --fix never deletes an evidence entry its archived story does not hold (by ID and receipt) or an unfolded inbox entry, and names each kept entry with the way out by hand (#381); macOS metadata never blocks the archive finish (#377); next names committing the session folder, story and journal post, then doctor --fix, while an archive is unfinished (#371); a front-matter conflict is reported as conflict_markers and unreadable names resolving it, and a Timeline entry that only redaction changed is not reported as rewritten while a hand edit still is (#374); the spec walk follows no symlink, is bounded at 4,096 entries and 32 folders, and says when it stops at the bound (#402).

## No-spec Rationale

Not applicable
