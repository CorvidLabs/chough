# CLI reference

Run `chough --help` and each subcommand's `--help` for the installed executable. All
commands accept global `--root PATH` (default: the current directory). Recording
evidence or running cases requires the Git repository root and an existing commit.
Output is JSON unless stated otherwise.

## Commands

| Command | Behavior |
|---|---|
| `init` | `--objective`, `--intent-scope`, repeated `--module`, repeated `--host openai\|claude\|codex\|cursor\|gemini\|grok` (default openai). Creates a schema 2 checkpoint and the skill bundle; preserves existing files. |
| `context [--json]` | Focused packet: instructions, intent, scoped HI export, module sources, recent transitions, valid transitions, preferences, open bugs. Markdown by default. |
| `status`, `next` `[--environment NAME]` | Checkpoint, gate for the current stage, valid transitions, `next_valid_action`, native status, snapshot, observations with freshness, open and blocking bugs. |
| `migrate [--stage S]` | Converts a schema 1 session after saving its bytes to `.chough/session.v1.json`; refuses if that backup exists. |
| `checkpoint` | Show or update: `--objective` (clears run consents), `--next-action`, `--native-change`, `--decision`, `--question`, `--resolve-question`, `--completed`, `--blocker`, `--clear-blockers`, `--awaiting-human`, `--clear-awaiting-human`, `--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`. |
| `stage show\|advance\|skip\|iterate\|back\|reset\|pause\|resume\|enter\|return` | Transitions with reasons and an append-only log. Entering build needs native state approved, implementing or verifying; leaving accept needs archived; leaving test needs no open blocking bug; entering validate or release needs consent or a standing preference. |
| `cases check\|list\|select\|run\|record` | Validate definitions; list digests; select focused cases (`--criterion`, `--requirement`, `--path`, `--track copy-localization\|design-qa`); run an automated binding; record a manual or agent-assisted run with `--verdict N=pass\|fail` for every expected outcome. `run` and `record` exit 3 when the recorded outcome is not passed. |
| `evidence add ID` | Criterion, requirement, contract, kind, outcome, environment, summary; optional artifacts, `--case`, `--method`, `--track`, `--limitation`. Never replaces an ID. |
| `bug add\|update\|list\|search\|show\|report` | Local backlog. `add` refuses likely duplicates unless `--distinct`; regression needs `--regression-baseline`; `verified` needs a passed observation; `report` prints Markdown and posts nothing. |
| `pref show\|set\|unset` | Standing preferences with `--source`: repeat-validation, monitoring, release, monitor-initiation, librarian, library-dir, required-lane. |
| `consent show\|grant\|revoke` | Run consent for the current objective: repeat-validation, monitoring, release. |
| `validate run\|stop\|resume\|status` | Optional repeated runs of one case with stored limits, stop requests and local capture of failures. |
| `library init\|snapshot\|index` | Optional librarian; requires `pref set librarian on`. |
| `skill status\|update [--host H]` | Compare installed copies with the bundle; update only missing and known-outdated files. |

## Records and limits

Managed JSON is bounded to 64 KiB, case files to 256 KiB, captured output to 1 MiB,
context to 16 KiB per section and 128 KiB in total, and sessions to 16 modules.
Native SpecSync status is parsed from its complete output up to 4 MiB and displayed
up to 16 KiB; `status` and `next` carry the parsed state and next action in
`native_summary`. Larger output is reported as `oversized` and refuses gated
transitions.
Unknown fields and unsupported schema versions are rejected. Paths are
project-relative; traversal and symlinks are rejected. Native tools and bound
commands receive direct arguments, never shell-evaluated text.

| Record | Schema |
|---|---|
| Checkpoint | 2 (schema 1 read-only until migrated) |
| Observation | 2 (schema 1 still read) |
| Case file | `profile = "chough-testing/1"` |
| Bug, preferences, transition, validation run | 1 |

## Observation outcomes and freshness

| Outcome | Meaning |
|---|---|
| `passed` | The process exited 0 and the binding's assertion held, or every verdict passed |
| `failed` | A reported test or declared assertion failed, a verdict failed, or the run timed out |
| `not-run` | Nothing was asserted: no test matched, the binding was ambiguous, or cargo failed before reporting results |

| Freshness | Meaning |
|---|---|
| `current-observation` | Inputs, artifacts, case definition and environment match now |
| `stale-revision-or-inputs` | The revision or any fingerprinted file changed |
| `stale-artifact` | A bound artifact changed or disappeared |
| `stale-case-definition` | What the case asserts changed since the run |
| `stale-environment` | A different environment was requested |
| `environment-unchecked` | No environment was supplied |
| `unknown-revision` | No Git snapshot could be established |

Environment comparison uses the label you supply; it does not detect changes in the
real environment. A native status response that decodes as JSON is usable output, not
evidence that a lifecycle gate passed.

Kinds: `product-test`, `structural`, `human-observation`, `delivery-observation`,
`publication-observation`, `installation-observation`, `deployment-observation`,
`live-health-observation`, `dry-run`. Human approval and delivery stay unknown in
helper output; an observation is an unauthenticated claim by whoever recorded it.

## Test-case adapters

| Adapter | Runs | Passes only when |
|---|---|---|
| `cargo-test` | `cargo test --locked --test <target> -- --exact <test>` | Exit 0 and exactly one test reported and passed |
| `fledge-task` | `fledge run <task>` | Exit 0 and every `expect_output` fragment appears |
| `command` | The argv as written | Exit 0 and every `expect_output` fragment appears |

See the [walkthrough](walkthrough.md) and [exercises](exercises/feature-bug-loop.md).
