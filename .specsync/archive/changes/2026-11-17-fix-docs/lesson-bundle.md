# Lesson bundle: fix-docs

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix docs
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: skills/chough-workflow/references/stages.md, .agents/skills/chough-workflow/references/stages.md, AGENTS.md, docs/lifecycle.md, docs/architecture.md, docs/integrations.md, docs/ci.md, docs/walkthrough.md, docs/verification.md, docs/exercises/feature-bug-loop.md, integrations/templates.toml, integrations/claude-code/hc-template.md, integrations/codex/hc-template.md, integrations/kimi/hc-template.md, integrations/grok/hc-template.md, integrations/local/hc-template.md, changelog.d/2026-11-16-fix-docs.md
- **Acceptance**: The drifted sentences match current main, the fast path keeps its frozen steps inside 32 lines, and each changed host template's newest digest is that file's SHA-256.

## Evidence

- Verification commit: `1a69bc45f8618114597c89a8ea76908d2febe01a`
- Base commit: `0fd9fd229744da039e9d43a5ce1df6dfe5ddc8d7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Fable reviewed HEAD 0fd9fd22 and found twelve documentation sentences that no longer match the helper, plus a fast path that told a small model to answer with raw SpecSync and to commit after `chough change check`. FLOW-18 already requires the docs, skill and diagrams to say what current main does. This change corrects those sentences and puts the same short rules in AGENTS.md and the five host templates. It does not change requirement text.

## From the change's design.md

# Design

No visual or interaction change. The rendered diagrams stay as they are. The preview sizes in docs/verification.md are updated to the committed SVG viewBoxes.

## From the change's testing.md

# Testing

`cargo test --locked --test package the_stage_contract_opens_with_a_short_fast_path_for_a_routine_change` keeps the fast path inside 32 lines and in the frozen order. `cargo test --locked --test integrations` checks each host template's newest digest. `cargo test --locked --test docs` checks the diagram labels and the skill sentences the docs tests name.

## Where these lessons go

- `specs/cli/cli.3md`
