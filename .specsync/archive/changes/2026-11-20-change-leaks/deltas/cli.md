# cli delta

## ADDED

(none)

## MODIFIED

### REQUIREMENT REQ-cli-057

SpecSync change steps through Chough. `chough change new <description> [--kind <kind>] [--spec <spec>]... [--path <path>]... [--no-spec-change --rationale <why>]` SHALL run `specsync change new <description>` with the same options and `--json`. `chough change answer <id> <question> <answer>` SHALL run `specsync change answer <id> <question> <answer> --json`. `chough change check <id>` SHALL run `specsync change check <id> --commit`, and `chough change check <id> --no-commit` SHALL run `specsync change check <id>` without it, the check the workflow's accept step runs first to materialize requirement text. `chough change status [<id>]` SHALL run `specsync change status [<id>] --json`. Each SHALL run in the project root given by `--root`, with the `specsync` that Chough's native reads use. Before SpecSync runs, each SHALL refuse with exit 1 an id, kind, spec, path or question argument that holds what the leak guard refuses in a written value or a control character (REQ-records-006), naming the argument and what it holds and not showing the value, so SpecSync's own error cannot show it; `--redact` SHALL NOT change this, since SpecSync, not Chough, would write the value. A description, a `--rationale` and an answer, which SpecSync writes into the change's committed records, SHALL be checked before SpecSync runs as REQ-records-006 checks a written value: one holding a leak SHALL refuse with exit 1 as `refused:leak`, naming the argument and the kind and never the value, or under `--redact` SHALL reach SpecSync with each match replaced by `<redacted:<kind>>`, with one line on standard error, before SpecSync's output, saying how many were replaced; one holding a control character SHALL refuse as `refused:control-character` (REQ-records-016), under `--redact` too (#412). Each SHALL pass SpecSync's standard output, standard error and exit code through unchanged, apart from that line, and SHALL write nothing under `.chough/` except the untracked direct-write baseline of REQ-cli-060 under `.chough/local/`. `status` is a read and SHALL keep the native read bound (120 s unless `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets it); past the bound it SHALL refuse with nothing recorded and name the retry with a longer bound. `new`, `answer` and `check` write, and SHALL never be stopped by that bound. Stopping Chough with SIGINT, SIGTERM or SIGHUP SHALL stop the SpecSync process it started (FLOW-14). When `specsync` cannot be started, each SHALL refuse with exit 1 and nothing run, naming the install command from the README's toolchain section and the same step run again.

Acceptance Criteria
- `chough change new --no-spec-change --rationale <why>` passes both options to SpecSync, and `--no-spec-change` without `--rationale` is refused before SpecSync runs, naming the command with `--rationale`.
- Each of `chough change new`, `answer`, `check` and `status` runs SpecSync with exactly the arguments above, in the `--root` directory, and its standard output, standard error and exit code match SpecSync's byte for byte, including a SpecSync refusal's, apart from the `--redact` count line.
- No `chough change` command changes a tracked file under `.chough/`; under `.chough/local/` it changes only its change's direct-write baseline (REQ-cli-060).
- `chough change check` completes after the native read bound has passed, while `chough change status` past the bound refuses with nothing recorded, and the retry it names succeeds.
- When `specsync` cannot be started, each command refuses with exit 1, naming the install command and the step to run again; with SpecSync installed, that step succeeds.
- A SIGTERM to `chough change check` stops the SpecSync process it started.
- `chough change check <id> --no-commit` runs `specsync change check <id>` without `--commit`, passes its output and exit code through, and the next stage move records no `direct-native-write` for what it wrote.
- `chough change new` with a credential sample as its `--path`, `--spec` or `--kind`, and `chough change status` with one as its id, each refuse before SpecSync runs, with and without `--redact`, name the argument and what it holds, and show no part of the sample; an id holding a control character is refused the same way.
- With each leak kind in a description, a `--rationale` and an answer, `chough change new` and `chough change answer` refuse with exit 1 naming the argument and the kind, SpecSync does not run, and nothing under `.specsync/` changes; with `--redact`, the change's records hold `<redacted:<kind>>` and no matched text, and one line says how many were replaced; a control character in any of them is refused under `--redact` too.

## REMOVED

(none)
