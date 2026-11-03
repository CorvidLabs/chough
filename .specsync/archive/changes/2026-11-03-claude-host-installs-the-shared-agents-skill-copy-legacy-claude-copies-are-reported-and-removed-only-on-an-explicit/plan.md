---
change: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
artifact: plan
---

# Plan

## Approach

Remap claude to `.agents/skills`, deduplicate shared directories, report legacy copies, and add one explicit removal command. Remove this repository's `.claude/` copy with that command and update docs to state the Claude Code tradeoff honestly.

## Out of scope

Remapping codex, cursor, gemini or grok; touching any other project; a pointer skill; release.

## Steps

1. Failure-first regressions in tests/optional.rs and a case.
2. Implement in src/skill.rs (and the subcommand wiring).
3. Update existing tests that asserted a `.claude` install, changing only what LIFE-12 changes.
4. Remove `.claude/` here; update CLAUDE.md, README.md, SKILL.md host text, docs and examples.
5. Verify through Fledge.

## Risks

Claude Code may not list the skill from `.agents/skills`; CLAUDE.md routes it through AGENTS.md. A user with an edited legacy copy keeps it.

## Constraints consulted

AGENTS.md (preserve existing files and others' edits); specs/cli/context.md host discovery lesson (2026-10-31).
