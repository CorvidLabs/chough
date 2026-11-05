## ADDED

### REQUIREMENT REQ-cli-040

Serialized checkpoint writers. Every command that reads the checkpoint to change it, in either mode (`.chough/session.json`, or the active session under `.chough/sessions/`), and `session new`, `session adopt`, `session use` and `inbox fold` SHALL hold one writer lock for the project from before that read until the command exits, so a concurrent writer applies its change to the latest checkpoint and no update is lost. The lock SHALL be the file `.chough/local/checkpoint.lock` inside the project, in a directory Git ignores, so every process working on the project uses the same lock whatever its temporary directory. A writer that cannot take the lock within 30 seconds SHALL refuse as `refused:checkpoint-busy`, naming the checkpoint file it would write, with nothing saved. A writer that read `.chough/session.json` and finds the project adopted into sessions mode before it saves SHALL refuse, naming the adoption, with nothing saved. Read-only commands SHALL take no lock. Trace: DRIVE-11, DRIVE-3.

Acceptance Criteria
- A writer started with a different `TMPDIR` waits while another process holds the project's lock, then saves its update.
- With the lock held, a writer in sessions mode refuses as `refused:checkpoint-busy` naming `.chough/sessions/<key>.json`, with nothing saved.
- Legacy writers racing `session adopt` each either save their decision in the one remaining checkpoint or refuse naming the adoption with nothing saved.
- `.chough/local/checkpoint.lock` is ignored by Git.
