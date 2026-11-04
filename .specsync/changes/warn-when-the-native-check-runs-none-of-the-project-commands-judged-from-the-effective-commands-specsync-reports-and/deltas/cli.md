## ADDED

### REQUIREMENT REQ-cli-038

Native verification that runs no project command. When SDD is enabled and the native status read that `chough next` or `chough status` makes reports the change's effective `verification_commands`, and every one of them is a SpecSync command (`specsync` followed by its arguments), `chough next` (text and JSON) and `chough status` SHALL say that native verification runs none of the project's commands, naming at most 3 of those commands, each cut to the next view's string bound, with a count for the rest, so a passing `specsync change check` does not show that the project's tests ran; when `.specsync/sdd.json` lists verification commands the effective list does not contain, they SHALL also say how many configured commands are not run. They SHALL decide this from that status read and that file, with no additional native read, and SHALL say nothing when the effective list is absent or includes a project command. At the test stage, when no pause, human decision, blocking bug, unlinked failure or gate stops it, `chough next` SHALL point to recording test evidence before accept, naming `chough cases run`. Neither SHALL allow or refuse any stage move that is not otherwise allowed or refused. Trace: FLOW-15, FLOW-6, LIFE-7.

Acceptance Criteria
- With an effective list of only `specsync check --spec cli`, `next`, `next --json` and `status` carry the warning naming that command.
- With `cargo test --locked` configured in `.specsync/sdd.json` but absent from the effective list, the warning still shows and says 1 configured command is not run.
- With a project command in the effective list, with no effective list in the status reply, with SDD disabled, or with no native configuration, there is no warning.
- At the test stage with nothing else stopping it, `next` names `chough cases run` and says recorded test evidence is needed before accept.
- `stage advance` from test is allowed with and without the warning, and the warning adds no native read.
- A 20,000-character effective command keeps `next` and `next --json` within 16384 bytes with the warning still shown, and 10 commands name 3 with "7 more".
