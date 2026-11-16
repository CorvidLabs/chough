---
id: fix-295-the-init-case-runs-the-real-preservation-test
state: archived
type: bug_fix
base_commit: 82a52e2dd5cb9f131b5c1a33133002fbda83b9f6
---

# Fix #295: the init case runs the real preservation test

## Intent

Fix #295: the init case runs the real preservation test

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- TC-cli-walk-ready-init-preserves runs init_preserves_user_files_and_installs_a_complete_skill. That test keeps edited session and skill bytes and adds a missing bundle file without rewriting the edited neighbor. specsync change audit reports no uncovered meaningful path.

## No-spec Rationale

REQ-cli-001 already states the preservation rule. This change binds the case to the existing init test and adds the missing-bundle assertion.
