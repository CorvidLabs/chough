# Chough

**A portable development workflow kit for coding agents: skills, human intent, specs, testing, and local context for resumable work.**

Chough (pronounced **chuff**) is a portable agent skill with a small Rust helper.
The current agent does the work. HI holds durable human criteria, SpecSync owns
contracts and its native change lifecycle, and Fledge runs project tasks. Chough
assembles focused context and records observations tied to a revision and environment.

![Chough and the tools that own its records](docs/diagrams/ownership.svg)

This is the first working foundation in a **private** CorvidLabs repository.
The CLI and starter skill below exist today. The broader [agreed lifecycle](docs/lifecycle.md)
is design for the next evolution, not a claim that stage orchestration, monitoring,
or a librarian has been implemented. Install from source; this bootstrap published
no package, release, or hosted service.

## Start here

```sh
git clone git@github.com:CorvidLabs/chough.git
cd chough
cargo install --path . --locked
chough --help
```

Use an installed `hi`, `specsync`, and `fledge` for their native capabilities.
Check their versions before adopting the workflow; match SpecSync lifecycle writers
and CI. The helper reports missing tools instead of treating their work as complete.

Inside a Git project with existing HI criteria and module contracts:

```sh
chough init --objective "Make the checkout failure reproducible" \
  --intent-scope CHECKOUT --module checkout
chough context
chough status --environment local
chough next --environment local
```

`init` creates a focused session and a discoverable
`.agents/skills/chough-workflow/SKILL.md`, preserving existing files. Invoke
`$chough-workflow` on a host that supports that skill convention, or give the current
agent the [canonical skill](skills/chough-workflow/SKILL.md). See the
[walkthrough](docs/walkthrough.md) for a feature, failed test, bug report, and resumed session.

## What works now

| Surface | First-slice behavior |
|---|---|
| `init` | Preserves existing project files; installs the skill and a plain JSON session. |
| `context` | Selects instructions, scoped HI export, chosen module contracts and companions, decisions, questions, and the next action. |
| `status`, `next` | Reports local evidence freshness and the native SpecSync response; missing information stays explicit. |
| `evidence add` | Records an observation with criterion and requirement IDs, contracts, artifacts, environment, and Git fingerprint. |
| Starter workflow skill | Provides guidance for intent, FRD synthesis, native changes, product checks, bug investigation, delivery observations, and lessons; the agent performs the work. |

Evidence is recorded after work is observed. Chough does not execute a product test
when you add an observation, authenticate a human, or infer that a deployment
succeeded. Structural checks and product tests remain different forms of evidence.

## Agreed direction: planned workflow

Chough stays current-agent-first and agent-neutral: the same skills and records
should work with capable GPT/Codex, Claude, Grok, Kimi, and local-model hosts.
Supported host commands are entry points into that workflow; universal skill/slash
command discovery across those hosts has not been established.

The planned loop moves from HI and an FRD through conditional local HTML/CSS design,
one coherent SpecSync technical scope, implementation, focused tests, local bug
repair, human acceptance/review, and native closeout. Release is optional.
Mandatory checkpoints support cold resume even when the optional project-local
librarian is off. Repetition/monitoring follows explicit project preferences and
persists findings locally. [Lifecycle design](docs/lifecycle.md) explains the
human gates, iteration and optional stages; [roadmap](docs/ROADMAP.md) distinguishes
that work from the current foundation. APS storage and 3md test profiles remain
candidates to qualify, and Rune coordination is optional/later.

## Develop Chough

```sh
fledge run fmt
fledge run test
fledge run lint
fledge lanes run pre-push
```

Read [AGENTS.md](AGENTS.md), [human intent](INTENT.md), the
[derived requirements](docs/FRD.md), and the [CLI contract](specs/cli/cli.spec.md)
before changing behavior. The bootstrap uses real HI, Fledge, and SpecSync structural
checks. It does not claim a human-approved, reviewed, and archived native change.
Subsequent scoped changes should follow the [native lifecycle](docs/lifecycle.md).

- [Architecture and context boundaries](docs/architecture.md)
- [Decisions](docs/DECISIONS.md), [roadmap](docs/ROADMAP.md), and [deferred, unapproved evidence-reuse proposal](docs/NEXT_CHANGE.md)
- [Integration map and precise limits](docs/integrations.md)
- [Diagram rendering and visual verification](docs/verification.md)

MIT licensed. This repository's visibility and its software license are separate facts.
