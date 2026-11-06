## ADDED

### REQUIREMENT REQ-history-012

Doctor findings and fixes. `chough doctor` (REQ-cli-055) SHALL find this debt without a native read, and its fixes SHALL use Chough's own steps:
- `legacy_checkpoint`: `.chough/session.json` exists. The fix SHALL adopt history first when the checkpoint has no history cursor, and then adopt it as a session (REQ-history-006) under its native change ID or under `--adopt-as <key>`. With neither key, the finding SHALL be `skipped`, with a fix that names `--adopt-as <key>`.
- `stale_pointer`: the active pointer names a session that is missing or archived. The fix SHALL remove the pointer.
- `finished_session`: an open session that is finished (REQ-history-006) and has no open awaiting-human gate. The fix SHALL archive it with its bytes unchanged. A session with unfolded inbox entries SHALL instead be reported as `skipped`, with a fix that names `chough inbox fold`.
- `stale_gates`: an open session whose native change is archived on HEAD and that has open awaiting-human gates. This fix is a decision. With `--reason`, it SHALL withdraw those gates as `chough checkpoint --clear-awaiting-human` does, recording no human decision, and SHALL then archive the session as the `finished_session` fix does.
- `unfinished_session`: an open, unfinished session named with `--close <key>`. This fix is a decision. With `--reason`, it SHALL close the session as `chough session close <key> --reason` does.

The fixes SHALL run in that order. They SHALL never replace a file, and SHALL leave the bytes of every session they do not fix unchanged. Trace: DRIVE-17, DRIVE-16, DRIVE-13, DRIVE-3.

Acceptance Criteria
- A finished session at stage intent and one at done are archived byte for byte, a stale pointer is removed, and open work stays open.
- A legacy checkpoint with a native change and no history cursor is adopted under that change's ID, with a cursor. One with no native change is skipped, naming `--adopt-as <key>`, and `--adopt-as <key>` adopts it.
- A finished session with unfolded inbox entries stays open and is reported as skipped, naming `chough inbox fold`.
- Without `--reason`, a stale gate and a session named by `--close` are skipped with their files unchanged. With `--reason`, the gate is withdrawn with no human decision recorded and the session is archived, and the named session is closed with the reason.
