# Chough

**Carry human intent through the development loop without losing the reasons, evidence, or next step.**

Chough (pronounced **chuff**) is a portable agent skill with a small Rust helper.
The current agent does the work. HI holds durable human criteria, SpecSync owns
contracts and its native change lifecycle, and Fledge runs project tasks. Chough
assembles focused context and records observations tied to a revision and environment.

![Chough and the tools that own its records](docs/diagrams/ownership.svg)

This is the first working foundation in a **private** CorvidLabs repository.
Install from source; no package, release, or hosted service is published by this bootstrap.

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
| Workflow skill | Guides intent refinement, FRD synthesis, native changes, testing, bug triage, delivery checks, and useful lessons. |

Evidence is recorded after work is observed. Chough does not execute a product test
when you add an observation, authenticate a human, or infer that a deployment
succeeded. Structural checks and product tests remain different forms of evidence.

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
- [Decisions](docs/DECISIONS.md), [roadmap](docs/ROADMAP.md), and [concrete next-change scope](docs/NEXT_CHANGE.md)
- [Integration map and precise limits](docs/integrations.md)
- [Diagram rendering and visual verification](docs/verification.md)

MIT licensed. This repository's visibility and its software license are separate facts.
