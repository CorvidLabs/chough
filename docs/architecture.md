# Architecture

The current agent interprets the request and uses the workflow skill. The helper
handles small deterministic operations: preserve a session, select context, validate
observations, and expose native status. Each existing tool keeps its own records.

![Record ownership](diagrams/ownership.svg)

```mermaid
flowchart TB
    human["Human intent and decisions"] --> agent["Current agent + Chough skill"]
    agent --> hi["HI: durable criterion IDs"]
    agent --> spec["SpecSync: requirements, contracts, changes, lessons"]
    agent --> fledge["Fledge: project tasks and lanes"]
    agent --> helper["Chough helper: focused session and observations"]
    hi --> context["Focused context packet"]
    spec --> context
    helper --> context
    context --> agent
    fledge --> results["Actual test and delivery results"]
    results --> helper
```

## Local records

| Path | Owner and meaning |
|---|---|
| `INTENT.md`, `hi/*.md` | HI: human wants and permanent criterion IDs. |
| `docs/FRD.md` | Workflow synthesis: sized functional requirements linked to intent. |
| `specs/<module>/` | SpecSync: module contracts, requirements, tasks, context, testing, and optional `.3md` lessons. |
| `.specsync/changes/`, `.specsync/archive/` | Native change, approval, verification, review, and archive records. |
| `fledge.toml` | Fledge: task and lane definitions. |
| `.chough/session.json` | Chough: selected objective, scope, modules, decisions, questions, next action. |
| `.chough/evidence/*.json` | Chough: observations tied to revision, worktree, environment, IDs, contracts, and artifacts. |
| `.agents/skills/chough-workflow/` | Installed project skill; existing user changes are preserved. |

`.chough` is plain project-local data. Choose whether to commit shared sessions and
observations according to the project's collaboration and confidentiality needs;
inspect artifacts and context before sharing them. The helper does not upload them.

## A focused packet

![Context assembly](diagrams/context.svg)

```mermaid
flowchart LR
    session["Session objective, decisions, questions, next action"] --> packet["Bounded deterministic packet"]
    instructions["Applicable AGENTS.md instructions"] --> packet
    hi["Scoped native hi export"] --> packet
    modules["Selected contracts and companions"] --> packet
    lessons["Module .3md lesson source when present"] --> packet
    packet --> agent["Current agent resumes"]
    missing["Missing, unreadable, oversized sources"] --> limits["Explicit limitations"]
    limits --> packet
```

The session selects modules rather than loading every contract or chat transcript.
When native `.3md` lessons are present, Chough carries that source and marks its
interpretation boundary; it does not silently replace it with a parallel lesson store.
Deterministic output makes repeated context assembly useful for session resumption
and review. Missing input remains visible so an agent can retrieve what matters.

Context is source material. It may contain user or repository text; it does not
override the host's instructions or authorize external actions. Applicable ancestor
instructions can live outside the project root; managed records and selected project
paths remain constrained to their validated destinations.

## Evidence is scoped, not permanent achievement

A criterion such as LIFE-2 remains durable. An observation says that a particular
check was reported as passed or failed for specific inputs and an environment.
The Git fingerprint includes the revision and relevant worktree inputs; Chough's
own records are excluded to avoid invalidating observations when adding another.
Code edits, a new revision, changed artifacts, or a different environment can make
an observation inapplicable. See the actual status output for the reason.

The observation kind states what was observed: structural validation, a product test,
a human observation, or a delivery observation. It is not a trust level. Recording
`human-observation` does not authenticate a person, and recording a delivery note
does not contact the deployed product. An agent must preserve that distinction.

## Extension boundary

An adapter should consume a native structured result or bounded source, preserve
its status and trust limits, and contribute useful context or evidence references.
It should not allocate a second change identity or translate a tool's degraded or
unknown state into success. [Integrations](integrations.md) documents the current
seams; only HI export and SpecSync status are helper integrations in this slice.
