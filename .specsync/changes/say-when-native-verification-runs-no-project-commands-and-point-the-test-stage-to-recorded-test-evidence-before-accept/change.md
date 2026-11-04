---
id: say-when-native-verification-runs-no-project-commands-and-point-the-test-stage-to-recorded-test-evidence-before-accept
state: approved
type: bug_fix
base_commit: 9d5d65b5acb306dde3af1f4166ce5c1b2c068f74
---

# Say when native verification runs no project commands, and point the test stage to recorded test evidence before accept (issue 85)

## Intent

Say when native verification runs no project commands, and point the test stage to recorded test evidence before accept (issue 85)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With SDD enabled and no verification_commands in .specsync/sdd.json chough next and chough status and chough stage reports say native verification runs none of the project commands and name verification_commands as the place to list them,With at least one verification command or with SDD disabled or with no native configuration none of them shows that warning,At the test stage with no gate or human decision or blocking bug stopping it chough next points to chough cases run and says recorded test evidence is needed before accept,The same stage moves are allowed and refused with and without the warning and the warning adds no native read,Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux

## No-spec Rationale

Not applicable
