## ADDED

### REQUIREMENT REQ-cli-060

Direct lifecycle writes noted. For the active session's native change, Chough SHALL watch the lifecycle records SpecSync writes in `.specsync/changes/<id>/`: `state.json`, `approvals.json`, `review.json`, `review-attempts.json`, `verification.json` and `verification-attempts.json`. It SHALL keep an untracked baseline of their digests and the change's state in `.chough/local/native-writes/<id>.json`.
- Each `chough change new`, `answer`, `check` and `ship` step SHALL first add any difference from the baseline to the baseline's pending findings, and after its SpecSync step SHALL set the baseline to the records as they are. Chough's own steps are therefore never flagged.
- A successful `chough stage` move SHALL record, before its own event, one `direct-native-write` history event (REQ-history-013) for each pending finding and for any difference from the baseline. It SHALL then clear the pending findings and set the baseline.
- Without a baseline, the first writing command SHALL set one and flag nothing.
- `chough next` and `chough status` SHALL show the pending findings and the current difference, as files added, changed or removed, and SHALL write nothing.
- A finding SHALL never refuse a command, and nothing SHALL be recorded outside a stage move, so the window between `change check --commit` and finalize gains no write.

Acceptance Criteria
- A lifecycle record changed outside Chough is recorded once, as a `direct-native-write` event naming the file and the state before and after, with the next stage move; a later move records nothing more.
- After `chough change answer` and `chough change check` change the records, the next stage move records no `direct-native-write` event; a write made before such a step is still recorded.
- A project with no baseline records nothing at its first stage move, and a write after that is recorded.
- `chough next` and `chough status` show a pending write with its file, and change no file under `.chough/`.
- A direct write before `chough change ship` is recorded at the move into done that ship makes.

## MODIFIED

### REQUIREMENT REQ-cli-057

SpecSync change steps through Chough. `chough change new <description> [--kind <kind>] [--spec <spec>]... [--path <path>]...` SHALL run `specsync change new <description>` with the same options and `--json`. `chough change answer <id> <question> <answer>` SHALL run `specsync change answer <id> <question> <answer> --json`. `chough change check <id>` SHALL run `specsync change check <id> --commit`. `chough change status [<id>]` SHALL run `specsync change status [<id>] --json`. Each SHALL run in the project root given by `--root`, with the `specsync` that Chough's native reads use. Each SHALL pass SpecSync's standard output, standard error and exit code through unchanged, and SHALL write nothing under `.chough/` except the untracked direct-write baseline of REQ-cli-060 under `.chough/local/`. `status` is a read and SHALL keep the native read bound (120 s unless `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets it); past the bound it SHALL refuse with nothing recorded and name the retry with a longer bound. `new`, `answer` and `check` write, and SHALL never be stopped by that bound. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14). When `specsync` cannot be started, each SHALL refuse with exit 1 and nothing run, naming the install command from the README's toolchain section and the same step run again.

Acceptance Criteria
- Each of `chough change new`, `answer`, `check` and `status` runs SpecSync with exactly the arguments above, in the `--root` directory, and its standard output, standard error and exit code match SpecSync's byte for byte, including a SpecSync refusal's.
- No `chough change` command changes a tracked file under `.chough/`; under `.chough/local/` it changes only its change's direct-write baseline (REQ-cli-060).
- `chough change check` completes after the native read bound has passed, while `chough change status` past the bound refuses with nothing recorded, and the retry it names succeeds.
- When `specsync` cannot be started, each command refuses with exit 1, naming the install command and the step to run again; with SpecSync installed, that step succeeds.
- A SIGTERM to `chough change check` stops the SpecSync process it started.
