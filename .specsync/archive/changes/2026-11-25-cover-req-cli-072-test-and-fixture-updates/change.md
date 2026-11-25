---
id: cover-req-cli-072-test-and-fixture-updates
state: archived
type: bug_fix
base_commit: 9d826c7c660b579ad7e83faf6fac6e31fb29062d
---

# Cover REQ-cli-072 test and fixture updates

## Intent

Cover REQ-cli-072 test and fixture updates

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The archive audit covers tests/docs.rs, tests/fixtures/skill-releases/manifest.json and tests/skill_releases.rs; hosted Archive and verify lanes pass on the branch head; the skill-chough-first step runs in hosted CI.

## No-spec Rationale

README lane-list test row and skill-release manifest/pin follow the REQ-cli-072 bundle edit
