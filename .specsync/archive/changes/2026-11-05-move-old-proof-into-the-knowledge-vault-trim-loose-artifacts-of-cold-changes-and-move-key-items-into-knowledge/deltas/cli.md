## ADDED

### REQUIREMENT REQ-cli-046

Loose artifacts and vault attachments. The post-archive summary of REQ-cli-033 SHALL also cover loose artifacts: each top-level entry under `.chough/artifacts/` whose files no observation record references. Such an entry SHALL belong to the cold archive whose manifest lists one of its files, or else to the archive whose cycle its first Git commit falls in, by cycle time (KNOW-20). It SHALL stay while any of its files is named by the checkpoint's references or by the fixture of a bug that is not closed (verified, duplicate or wont-fix), is listed by a hot archive, is not committed identically at HEAD, or was first committed after the newest archive. An eligible entry SHALL be ledgered first as `kind: loose-artifact` with ID `artifact:<entry>`, with its files, and SHALL then be removed (KNOW-18). Key items SHALL move into the vault instead of being removed:
- the record and artifacts of a non-routine observation (manual, agent-assisted or a human observation) whose only remaining protection is that it is non-routine;
- the artifacts of a failed run;
- screenshots (png, jpg, jpeg, gif, webp, svg) and failure-first logs among artifacts.

Each key item SHALL move to `knowledge/attachments/<archive>/<path under .chough/>` before its original is removed. The ledger entry SHALL map each moved file to its attachment, a restore SHALL delete the copies it made, and the archive's knowledge note SHALL list its loose artifacts and link its attachments (KNOW-19). Trace: KNOW-18, KNOW-19, KNOW-20, KNOW-9, KNOW-10.

Acceptance Criteria
- A loose artifact entry listed only by a cold archive, and one first committed in a cold archive's cycle, are trimmed and ledgered as loose artifacts with that archive, and the archive's note lists their files. One first committed after the newest archive, and one named by an open bug's fixture, stay.
- The loose fixture of a verified bug is trimmed.
- A manual observation with a screenshot, and the log of an older failed run, owned by a cold archive, move to `knowledge/attachments/<archive>/`. They are gone from `.chough/`, mapped in the ledger and linked from the archive's note. A routine record is removed, not moved.
