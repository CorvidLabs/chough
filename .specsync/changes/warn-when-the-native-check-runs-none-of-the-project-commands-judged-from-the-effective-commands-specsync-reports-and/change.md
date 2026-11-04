---
id: warn-when-the-native-check-runs-none-of-the-project-commands-judged-from-the-effective-commands-specsync-reports-and
state: approved
type: bug_fix
base_commit: 1ea894c91144947915dcaccf11aecb0f6ab86751
---

# Warn when the native check runs none of the project commands, judged from the effective commands SpecSync reports, and point the test stage to recorded test evidence (issue 85)

## Intent

Warn when the native check runs none of the project commands, judged from the effective commands SpecSync reports, and point the test stage to recorded test evidence (issue 85)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- With an effective native command list of only specsync check the next and status reports carry the warning naming that command. With a configured project command that the effective list omits the warning still shows and counts it. With a project command in the effective list or no list or SDD disabled or no native configuration there is no warning. At the test stage next names chough cases run and says recorded test evidence is needed before accept. Stage moves and native reads are unchanged. Each new test fails on main 91a1fdd and fledge lanes run verify passes on Linux.

## No-spec Rationale

Not applicable
