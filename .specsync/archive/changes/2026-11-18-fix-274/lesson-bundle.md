# Lesson bundle: fix-274

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 274
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: examples/feature-bug-loop.sh, tests/cli.rs, .chough/cases/cli.example-stops-at-a-refused-command.cases.toml, changelog.d/2026-11-18-fix-274.md
- **Acceptance**: When a chough or SpecSync command whose JSON examples/feature-bug-loop.sh reads with jq fails or is refused, such as chough bug update --status verified, the example stops at that command with a non-zero exit and that command's error, and runs no later step. No such command's output is piped straight into jq. A run whose commands all succeed passes as before.

## Evidence

- Verification commit: `60e0f1820ad35a4aaffb803886ee7ff3fd3a4535`
- Base commit: `be6c9f126e448bd33945a52261d55db2a9677d3d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #274, found by t37f while fixing #247 (#258). `examples/feature-bug-loop.sh`, the verify lane's feature-and-bug exercise, runs under `set -eu` without `pipefail`, and pipes the JSON of `chough` and `specsync` commands straight into `jq`. A pipeline's exit status is its last command's, so when such a command is refused, `jq` reads nothing, exits 0 and the script carries on. The failure surfaces later, at a step that reads the result. CI run 43357893936 (#247) failed this way: the refused `chough bug update … --status verified` was reported one step late.

`set -o pipefail` is not available in every `sh` (Debian's `dash` lacks it), so the fix captures each such command's output first, where `set -e` stops the script on a failure with the command's own error, and pipes the captured text to `jq`.

Leif approved the scope on 2026-11-18, relayed by his planning session (hc t4c), with the A list (#268, #274, #294). No requirement covers how the example fails, so no requirement text changes.

## From the change's testing.md

# Testing

`cargo test --locked --test cli exercise_stops_at_a_refused_bug_update` runs the script's own helpers and its "rerun after the fix" lines, cut out between their markers, with a stand-in `chough` whose `bug update` is refused. The script must exit non-zero with the refusal and run no later line. On main it carries on and exits 0. `cargo test --locked --test cli exercise_pipes_no_command_output_straight_into_jq` reads the script and requires that no line pipes a `$chough` or `specsync` command's output directly into another command. On main several do. Case `TC-cli-exercise-stops-at-a-refused-command` binds the first. The verify lane's `exercise` step runs the whole script.

## Where these lessons go

- `specs/cli/cli.3md`
