---
change: explain-how-chough-guides-the-specsync-lifecycle-stages-and-human-gates-in-the-readme-and-walkthrough-flow-18
artifact: context
---

# Context

## The request

Leif, 2026-11-07, after the SpecSync #893 review: "let's talk about chough now.. does it auto drive all of spec-sync? it still asks for the stages? can we document or update readme and all stuff for how chough works?" This change is the documentation part: explain, in the README and walkthrough, how Chough actually drives the workflow. It invents no new automation and relaxes no human authority.

## What current main does (verified on e2d20dad)

- `chough change new/answer/check/status` are pass-through wrappers: they run the matching `specsync change` step and pass its output, refusals and exit code through unchanged; nothing under `.chough/` is read or written (src/change.rs:1-7, 88-141). Installed Chough 0.5.0's `change new` lacks the `--no-spec-change`/`--rationale` flags that main's source has; the native `specsync change new` was used for this definition.
- `chough change ship` is the only wrapper that touches Chough records: it checks session, stage, open gates and the review relay, runs `specsync change ship`, then closes the review gate with its relay and moves the session to done (src/change.rs:53-55).
- `chough next` is routing and guidance only: it assesses the session, native SpecSync status, evidence freshness, gates and bugs, and prints the focused next valid action; it executes nothing (src/main.rs:744-838). It is not an unattended pipeline runner.
- The agent advances stages by running `chough stage …`; src/stage.rs records transitions and refuses ones native SpecSync state does not allow (src/stage.rs:1-2, 299-317). No stage advances itself.
- Native `specsync change approve` and `change review` are never wrapped: they stay human decisions recorded from real human authority. `chough checkpoint --close-gate` / `chough gate record` keep a relayed claim (who decided, exact words, where, recording agent), never a native approval (REQ-history-010, FLOW-16).
- The workflow skill (skills/chough-workflow/SKILL.md) already says the current agent runs HI/SpecSync/Fledge itself, uses applicable prior or standing human authorization for a concrete scope, and asks when authority is missing or exceeded. That is existing product policy; this change documents it, it does not extend it.

## Gaps that stay product decisions, not docs

- There is no auto-driver: nothing runs the whole pipeline unattended. Documenting `next` as guidance is correct; building more automation is a separate proposal.
- Issue #76 items 1, 3 and 4 (`next` guidance wording right after init, at test, and at accept) still wait on Leif's wording; item 4 depends on DRIVE-6. This change does not pre-answer them.
- The archive-stage proposal (handoff ch-archive-stage.md / ch-archive-stage-proposal.md) is separate and unapproved; the docs must not describe it as current behavior.

## Constraints

- Docs only: README.md, docs/walkthrough.md, docs/lifecycle.md. docs/cli.md stays the command catalog; the authoritative skill text is already accurate and is not rewritten. No helper behavior, stage semantics, retention or SpecSync changes; no new HI or REQ IDs (FLOW-18, FLOW-13, LIFE-3 cover this).
- tests/docs.rs (FLOW-18, FR-docs-1 to FR-docs-6) must keep passing: the README helper table names every command `chough --help` lists, documented commands/flags exist, links and anchors resolve, and no page states a tool version other than the CI pins (HI 0.8.0, Fledge 1.8.0, SpecSync 6.2.0, D-021).
- Do not universalize one orc session's exact-head-only recording rule into the public docs; the general skill allows the authorized current agent to record applicable human authority.
- Before every push: `specsync change audit --strict`; `fledge lanes run verify` before the native check.
