# TC-cli-0113 reviewer note (agent-assisted, Claude Code, f6416a5)

Each correction read against the cited source at this revision.

1. SKILL.md: without `--host`, init installs for `openai`; any `--host` replaces the
   default (src/main.rs `init`: `if hosts.is_empty() { vec!["openai"] } else { hosts }`).
   Matches.
2. references/stages.md: `stage resume` only logs the transition and sets the stage
   active, returning a check telling the agent to run `chough next` (src/stage.rs
   StageCommand::Resume). Matches.
3. references/validate.md stop: `validate stop` writes `.chough/validation.stop`
   (src/validate.rs STOP_REQUEST, ValidateCommand::Stop); `halt` returns stopped while
   it exists, checked before every attempt of any run; only `validate resume` removes
   it. Matches.
4. references/validate.md pause: `halt` returns paused when the checkpoint stage
   status is paused. Matches.
5. references/validate.md monitoring: entering validate accepts repeat-validation or
   monitoring (src/stage.rs StageCommand::Enter), while `validate run` and `resume`
   call `authority`, which accepts only repeat-validation (src/validate.rs). Matches.
6. references/testing.md timeout: `timeout_seconds` must be 1 to 3600 (src/cases.rs
   validation), default 600 (`unwrap_or(600)`), and a timed out run is
   Outcome::Failed. fledge-task and command refuse an empty expect_output. Matches.
7. references/testing.md record: `cases record` returns NOT_PASSED_EXIT (3) when the
   outcome is not passed (src/cases.rs CasesCommand::Record). Matches.
8. templates/cases.toml: the commented fledge-task (`task`), command (`argv`),
   `timeout_seconds` and `env` lines name fields the binding accepts (src/cases.rs
   binding validation). Matches.
9. references/checkpoint.md: `--clear-awaiting-human` takes every open gate
   (`std::mem::take`); gates passed again in the same call are re-added after the
   clear, logged gate-closed and then gate-opened (src/checkpoint.rs). Matches.
10. references/lifecycle.md: refused:no-native-change, ungated:not-configured and
    ungated:sdd-disabled (no config, no sdd.json or enabled false), and
    refused:native-correction-invalid (src/stage.rs native_state and gate). Matches.

(Items 3 to 5 are the validate.md findings; the design counts nine corrections by
file and finding.)

Host copies:
- .agents/skills/chough-workflow: byte-identical to skills/chough-workflow (diff -r)
- .claude/skills/chough-workflow: byte-identical to skills/chough-workflow (diff -r)

chough skill status --host openai --host claude:
- openai: current (24 files)
- claude: current (24 files)
