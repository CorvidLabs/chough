---
id: done-drops-every-file-and-design-reference-and-archived-sessions-hold-no-proof-by-them-know-21
state: approved
type: feature
base_commit: 9991d09cb153ac409a43c5071ad5371ddd53fe49
---

# Done drops every file and design reference, and archived sessions hold no proof by them (KNOW-21)

## Intent

Done drops every file and design reference, and archived sessions hold no proof by them (KNOW-21)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- stage advance into done drops every file and design reference from the checkpoint and reports them as dropped_files and dropped_design, keeps current-cycle observation references under the KNOW-16 rule, and a cold loose folder a dropped reference held is then trimmed; retention ignores an archived session file and design references without rewriting it. Both new tests fail first and fledge lanes run verify passes on Linux.

## No-spec Rationale

KNOW-21 (Leif chose Drop all at done, 2026-11-05 16:29 MST): REQ-cli-045 widens from observations to file and design references and retention ignores archived sessions references.
