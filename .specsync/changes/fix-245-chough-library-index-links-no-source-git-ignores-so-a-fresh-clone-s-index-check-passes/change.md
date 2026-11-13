---
id: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
state: approved
type: bug_fix
base_commit: 85304969e053e7477dff11d2ee53b6dddcb6a070
---

# Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes

## Intent

Fix #245: chough library index links no source Git ignores, so a fresh clone's index check passes

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough library index links no rule, skill, decision or policy file that Git ignores, so after chough init --host grok in a repository that ignores .grok, the index links nothing under .grok/ and library index --check passes in the working tree and in a fresh clone (red then green)

## No-spec Rationale

Not applicable
