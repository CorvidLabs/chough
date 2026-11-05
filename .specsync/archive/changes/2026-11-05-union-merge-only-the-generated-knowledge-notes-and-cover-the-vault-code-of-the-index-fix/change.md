---
id: union-merge-only-the-generated-knowledge-notes-and-cover-the-vault-code-of-the-index-fix
state: archived
type: bug_fix
base_commit: 4ab0b8ad33a937a7945f7ac5b898d7b9145ca5f7
---

# Union-merge only the generated knowledge notes, and cover the vault code of the index fix

## Intent

Union-merge only the generated knowledge notes, and cover the vault code of the index fix

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- Only the knowledge notes library index generates and library index --check compares (INDEX.md, rules.md, timeline.md and the timeline, archives, modules and intent notes) merge with Git union; attachments, snapshots, briefs and README.md do not. src/vault.rs, changed by the index-bound fix, is covered. The new test fails first and fledge lanes run verify passes on Linux with the audit green.

## No-spec Rationale

Bug fix within REQ-cli-047 (the union driver must cover only files library index --check compares) and coverage of src/vault.rs, which the index-bound fix changed outside its declared paths; no requirement text changes.
