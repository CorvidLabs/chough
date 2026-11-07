## MODIFIED

### REQUIREMENT REQ-cli-057

SpecSync change steps through Chough. `chough change new <description> [--kind <kind>] [--spec <spec>]... [--path <path>]... [--no-spec-change --rationale <why>]` SHALL run `specsync change new <description>` with the same options and `--json`. `chough change answer <id> <question> <answer>` SHALL run `specsync change answer <id> <question> <answer> --json`. `chough change check <id>` SHALL run `specsync change check <id> --commit`. `chough change status [<id>]` SHALL run `specsync change status [<id>] --json`. Each SHALL run in the project root given by `--root`, with the `specsync` that Chough's native reads use. Each SHALL pass SpecSync's standard output, standard error and exit code through unchanged, and SHALL write nothing under `.chough/` except the untracked direct-write baseline of REQ-cli-060 under `.chough/local/`. `status` is a read and SHALL keep the native read bound (120 s unless `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets it); past the bound it SHALL refuse with nothing recorded and name the retry with a longer bound. `new`, `answer` and `check` write, and SHALL never be stopped by that bound. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14). When `specsync` cannot be started, each SHALL refuse with exit 1 and nothing run, naming the install command from the README's toolchain section and the same step run again.

Acceptance Criteria
- `chough change new --no-spec-change --rationale <why>` passes both options to SpecSync, and `--no-spec-change` without `--rationale` is refused before SpecSync runs, naming the command with `--rationale`.
- Each of `chough change new`, `answer`, `check` and `status` runs SpecSync with exactly the arguments above, in the `--root` directory, and its standard output, standard error and exit code match SpecSync's byte for byte, including a SpecSync refusal's.
- No `chough change` command changes a tracked file under `.chough/`; under `.chough/local/` it changes only its change's direct-write baseline (REQ-cli-060).
- `chough change check` completes after the native read bound has passed, while `chough change status` past the bound refuses with nothing recorded, and the retry it names succeeds.
- When `specsync` cannot be started, each command refuses with exit 1, naming the install command and the step to run again; with SpecSync installed, that step succeeds.
- A SIGTERM to `chough change check` stops the SpecSync process it started.
