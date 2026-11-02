# Chough

**A portable development workflow kit for coding agents: skills, human intent, specs, testing, and local context for resumable work.**

[![crates.io](https://img.shields.io/crates/v/chough.svg)](https://crates.io/crates/chough)
[![CI](https://github.com/CorvidLabs/chough/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/CorvidLabs/chough/actions/workflows/ci.yml)
[![MSRV 1.88](https://img.shields.io/badge/MSRV-1.88-blue.svg)](Cargo.toml)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

Chough (pronounced **chuff**, as in *chuffed*: how you feel when it merges) is a
portable agent skill with a small Rust helper. It is current-agent-first and
agent-neutral: the same skill and records are meant for
capable GPT/Codex, Claude, Grok, Kimi and local-model hosts. You keep working with
the agent you already use. Chough gives it a shared stage workflow
and a checkpoint in plain project files. HI keeps the human's wording and criterion
IDs, SpecSync owns contracts and the native change lifecycle, and Fledge runs tasks.
Chough records what stage work is in, what was decided, what was actually checked
(with the revision and environment), which bugs are open, and the next valid action.

![Chough and the tools that own its records](docs/diagrams/ownership.svg)

Chough is a public CorvidLabs repository. Release history is in
[CHANGELOG.md](CHANGELOG.md). Nothing is hosted or deployed.

## Install

Chough needs Rust 1.88 or later. Install a released version from crates.io (the
releases are listed in [CHANGELOG.md](CHANGELOG.md)):

```sh
cargo install chough --locked
chough --help
```

To run unreleased work from main, install from the repository:

```sh
cargo install --git https://github.com/CorvidLabs/chough --locked
```

or from a clone:

```sh
git clone https://github.com/CorvidLabs/chough.git
cd chough
cargo install --path . --locked
```

The skill bundle is embedded in the binary; `chough init` copies it into your
project.

## Install the toolchain

Chough works with HI, SpecSync and Fledge. One command installs the versions CI
uses:

```sh
cargo install --locked human-intent@0.8.0 fledge@1.8.0 specsync@6.1.2
```

This command needs Rust 1.89 or later, because SpecSync 6.1.2 declares it. Cargo
skips a tool already installed from crates.io at that version and replaces a
different version (`--force` reinstalls). The binaries are `hi`, `fledge` and
`specsync`. Install Chough itself as shown in [Install](#install).

## Start here

In a Git project with HI criteria and SpecSync module contracts:

```sh
chough init --objective "Free shipping from fifty dollars" \
  --intent-scope CHECKOUT --module shipping --host claude
chough next --environment local
```

`init` never overwrites existing files. It writes the checkpoint
`.chough/session.json` and copies the skill bundle into each requested host's
project skill directory (`openai`, the default, → `.agents/skills`, a shared skills
directory; `claude` → `.claude/skills`; `gemini` → `.gemini/skills`, which no run has
exercised; also `codex`, `cursor`, `grok`). Kimi Code CLI uses the existing `openai`
installer value and discovered `.agents/skills/chough-workflow/SKILL.md` in one
observed run. Google Antigravity read the skill from `.agents/skills` in one run
reported narratively in closed PR #20 and not independently verified. `chough next`
prints the focused next action as text (`--json` for compact JSON); `chough status`
is the full JSON report. Then ask your agent in plain language, for example "Use the
Chough workflow to add free shipping at fifty dollars", or give it
[the skill](skills/chough-workflow/SKILL.md) directly.

## The loop

![Stages, backtracking and side stages](docs/diagrams/stages.svg)

| Stage | What happens | Who decides |
|---|---|---|
| intent | Capture wants in HI, derive an FRD, synthesize role reviews | The human approves the intent as a whole |
| design | Only for visual changes: one local HTML/CSS prototype and its companion | The human approves the render and notes together |
| plan | SpecSync change with design, plan and tasks | One native scope approval |
| build | Implement the approved tasks through Fledge | Agent, within scope |
| test | Focused cases for the affected criteria; three tracks | Evidence, not opinion |
| accept | Present evidence; native review, then same-PR archive | The human reviews the implementation |
| release | Optional, only on request or a standing setting | The human |

An active stage can iterate or pause. An active main stage after intent can go back,
and one before done, or release, can be skipped with a reason. A paused stage resumes,
and reset works from any stage, including a paused or side stage.
Side stages for bugs, optional repeated validation and the optional librarian return
to their caller. Stage gates read native SpecSync state, never a saved label:
reaching build needs a valid scope approval, reaching test or accept needs a valid
approval or an authenticated archive, and reaching done needs an authenticated native
archive. A gate that cannot read native state refuses, and so does one with SpecSync
configured but no native change recorded (`refused:no-native-change`); a project
without SpecSync configured, or with SDD disabled, is not gated.

## What the helper does

| Command | Purpose |
|---|---|
| `init`, `skill status/update` | Install the skill per host without overwriting edits |
| `next`, `status`, `context` | Resume: stage, gates, freshness, open bugs, focused sources |
| `checkpoint`, `migrate` | Keep the single checkpoint; migrate schema 1 explicitly |
| `stage …` | Advance, skip, iterate, back, reset, pause, resume, enter and return |
| `cases check/list/select/run/record` | Validate and list `chough-testing/1` definitions, select focused cases, record honest outcomes |
| `evidence add` | Record an observation with its revision, inputs, environment and artifacts; freshness compares content, not the revision |
| `evidence prune` | Preview, then on consent remove, routine runs nothing references ([retention](docs/architecture.md#records-and-owners)) |
| `bug add/update/search/list/show/report` | Local-first backlog; report candidates are never posted |
| `pref`, `consent`, `validate` | Standing preferences versus run-only consent; optional repeated runs |
| `library init/snapshot/index` | Optional project-local librarian |
| `timing` | Read-only report of how long stages, tests, bug fixes and human gates took, as elapsed wall-clock time with history ranges that are estimates, not targets |

## Merge parallel checkpoint edits

The repository declares `.chough/session.json merge=chough-session` in
`.gitattributes`. To enable the merge driver, each clone runs:

```sh
git config --local merge.chough-session.name "Chough schema 2 checkpoint"
git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"
```

Git supplies base, ours and theirs as `%O`, `%A` and `%B`. A nonzero driver result
leaves the conflict for human resolution. `chough init` never sets either Git
configuration value.

Without the driver, use the explicit fallback:

```sh
chough checkpoint --merge <base> <ours> <theirs>
```

The command merges schema 2 checkpoints only. Success replaces only ours, while a
current-cycle conflict stays unresolved instead of combining fields. One agent still
owns live checkpoint writes, and merging adds no second store. See
[Checkpoint merges](docs/cli.md#checkpoint-merges) for the collection and conflict
rules.

Chough does not authenticate people, record approvals or reviews, publish anything,
or deploy. A structural pass, an exit code or an actor label is never treated as
behavioral proof or human acceptance.

## Tested platforms

| Where | What runs |
|---|---|
| Hosted CI: Ubuntu, macOS, Windows | `cargo test` and a release build |
| Hosted CI: Ubuntu, Rust 1.88.0 | `cargo test` with the minimum Rust version |
| Hosted CI: Ubuntu | An archive gate that runs first and blocks every other job while a SpecSync change is open; the Fledge `verify` lane (format, lint, tests, HI and SpecSync checks, change audit, case definitions, the feature and bug exercise, build); the native archive check and its fixtures |
| Local: macOS arm64 | Development, the exercises and lifecycle records |

One 2026-11-01 Kimi Code CLI 0.41.0 run on macOS arm64 auto-discovered the `.agents`
skill, built successfully and attempted a cold resume. It could not safely resume the
archived work because `next` and `status` repeated stale review and ship guidance,
recorded as BUG-0024; it also exposed BUG-0021's missing mapping guidance. The earlier
native-routing fix for BUG-0024 was superseded during merge by main's newer REQ-cli-021
new-cycle behavior. BUG-0025 records the distinct integration defect and is verified by
the later linked TC-cli-0105 pass under the approved alignment change. TC-cli-0105
covers the new-cycle regression and TC-cli-0106 retrospectively reviews the
documentation. Their evidence is not a fresh
post-fix Kimi rerun. Other platforms may work but have not been exercised.

## Known limitations

- On Windows, a bounded command's descendants whose parent has already exited cannot
  be stopped; the run returns on time with an incomplete capture, never `passed`.
- Calls to Git (fingerprints and revisions) are not yet bounded in time or output.
- SpecSync 6.1.1 and earlier do not flag an approved delta edited after approval;
  SpecSync 6.1.2 does
  ([CorvidLabs/spec-sync#823](https://github.com/CorvidLabs/spec-sync/pull/823)), and
  Chough's own guard still refuses such a delta with older versions.
- In a repository with no commits yet, `chough next` reports the raw `git rev-parse
  HEAD` error as the snapshot; make a first commit before relying on freshness.
- Cursor, Gemini and Grok have not had real discovery or cold-start exercises. Kimi
  remains limited to the single run above, with no fresh post-fix run. Google
  Antigravity has only the unverified narrative report linked below.

See [integrations and limits](docs/integrations.md) for the rest.

## Try it

- [Feature and bug exercise](docs/exercises/feature-bug-loop.md):
  `fledge run exercise` runs a real loop in a disposable sample.
- [Visual design exercise](docs/exercises/visual-design.md) with rendered review.
- [Cold start by an independent agent](docs/exercises/cold-start.md).
- [Walkthrough](docs/walkthrough.md): install, one feature and bug, and resuming.

## Develop Chough

```sh
fledge run fmt
fledge run test
fledge run lint
fledge lanes run verify
```

Read [AGENTS.md](AGENTS.md), [intent](INTENT.md), the [FRD](docs/FRD.md) and the
[CLI contract](specs/cli/cli.spec.md) before changing behavior. Product changes go
through the native SpecSync lifecycle; the policy requires a change for meaningful
files.

- [Architecture](docs/architecture.md), [lifecycle](docs/lifecycle.md), [CLI reference](docs/cli.md)
- [Decisions](docs/DECISIONS.md), [roadmap](docs/ROADMAP.md), [integrations and limits](docs/integrations.md)
- [Evidence-reuse proposal](docs/NEXT_CHANGE.md): addressed for content-identical
  commits by D-018; its ancestry check and reuse label remain unapproved
- [Diagram rendering and visual verification](docs/verification.md)
- [Release steps](docs/releasing.md) and the [change log](CHANGELOG.md)

MIT licensed. This repository's visibility and its software license are separate facts.
