## ADDED

### REQUIREMENT REQ-cli-038

Empty native verification. When a native SpecSync configuration exists and `.specsync/sdd.json` enables SDD with no `verification_commands`, `chough next` (text and JSON), `chough status` and the `chough stage` report SHALL say that native verification runs none of the project's commands, so a passing `specsync change check` does not show that the project's tests ran, and SHALL name `verification_commands` in `.specsync/sdd.json` as the place to list them. The helper SHALL decide this from that file alone, with no native read. At the test stage, when no pause, human decision, blocking bug, unlinked failure or gate stops it, `chough next` SHALL point to recording test evidence before accept, naming `chough cases run`. Neither SHALL allow or refuse any stage move that is not otherwise allowed or refused. Trace: FLOW-15, FLOW-6, LIFE-7.

Acceptance Criteria
- With SDD enabled and no `verification_commands`, `next`, `next --json`, `status` and `stage show` carry the warning naming `verification_commands`.
- With at least one verification command, with SDD disabled, or with no native configuration, none of them carries it.
- At the test stage with nothing else stopping it, `next` names `chough cases run` and says recorded test evidence is needed before accept.
- `stage advance` from test is allowed with and without the warning, and the warning adds no native read.
