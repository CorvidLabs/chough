# Lesson bundle: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Claude host installs the shared .agents skill copy, legacy .claude copies are reported and removed only on an explicit command, and this repository drops its .claude copy
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/skill.rs, src/main.rs, tests/, specs/cli/, skills/, .agents/, .claude/, CLAUDE.md, README.md, docs/, examples/, CHANGELOG.md, hi/, INTENT.md, .chough/
- **Acceptance**: `chough init --host claude` and `chough skill update --host claude` write only the shared `.agents/skills/chough-workflow/` copy, and a request naming both claude and openai installs one copy, not two
- **Acceptance**: `chough skill status` and `chough skill update` report an installed `.claude/skills/chough-workflow/` copy as legacy and never delete, update or rewrite it; `chough init` never deletes it
- **Acceptance**: `chough skill remove-legacy --host claude` removes only legacy files byte-identical to the current or a known earlier bundle, keeps edited or unknown files, and refuses with nothing removed when the shared `.agents` copy is missing
- **Acceptance**: This repository has no `.claude` skill copy, `CLAUDE.md` routes Claude Code to `AGENTS.md` and the shared copy, and `docs/integrations.md` states that Claude Code 2.1.316 did not discover `.agents/skills` alone
- **Acceptance**: codex, cursor, gemini and grok keep their existing directories; fmt, lint, the full test suite and `fledge lanes run verify` pass

## Evidence

- Verification commit: `d036d9234b5926d26d0a8dcd2caeb03171138fe1`
- Base commit: `0363d4af368bb07cc07f16c2f5fff7645c1c05bc`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif (0xLeif) answered a multiple-choice interview on 2026-11-03: "Which AI tools should get a copy of the instructions? Only the shared .agents copy", and at 8:27 AM MT, choosing among a pointer skill, removal and keeping both copies, "Remove .claude entirely". He chose removal knowing that docs/integrations.md records Claude Code 2.1.316 discovering `.claude/skills` and not `.agents/skills` alone (answer: NONE). HI LIFE-12, LIFE-13 and LIFE-14 capture this.

At main 0363d4a `chough init --host claude` writes a full bundle to `.claude/skills/chough-workflow/`, and this repository commits byte-identical full copies under both `.agents/` and `.claude/`.

Human authority: Leif (0xLeif) in chat 2026-11-03 8:09-8:27 AM MT, interview answers "Only the shared .agents copy" and "Remove .claude entirely", relayed by the parent agent as his intent approval; his next decision is final acceptance ("Ask me only at intent and final acceptance"). Covered scope: the claude host mapping, legacy-copy reporting and an explicit removal command in src/skill.rs and src/main.rs, their tests, REQ-cli-016, removal of this repository's own `.claude/` copy, and docs. Limits: other hosts keep their directories; no other project's files are touched; no release or deployment; edited legacy files are never removed.

## From the change's design.md

# Design

`HOSTS` maps `claude` to `.agents/skills`. Host lists are deduplicated by directory before planning or reporting, so `--host claude --host openai` plans one bundle; reports keep the first requested host name and list every host that shares the directory. A new `LEGACY` table names `(claude, .claude/skills)`. `skill status` and `skill update` add a `legacy` array: each installed legacy directory with every file's state (current, known-outdated, edited, unknown) and the explicit removal command. Neither ever writes there.

`chough skill remove-legacy --host claude` (no host means every installed legacy directory) refuses with nothing removed unless the shared `.agents/skills/chough-workflow/SKILL.md` exists. It removes only bundle files whose bytes are the current bundle's or a known earlier bundle's (KNOWN_PREVIOUS), keeps edited and unknown files, then removes directories left empty, and reports removed and kept files.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-016, LIFE-12 | `tests/optional.rs::claude_host_installs_the_shared_agents_copy_once`: `init --host claude --host openai` creates only `.agents/skills/chough-workflow/` and no `.claude/`; `skill update --host claude` acts on the shared copy. Fails at 0363d4a. |
| REQ-cli-016, LIFE-13 | `tests/optional.rs::legacy_claude_copy_is_reported_and_removed_only_explicitly`: a legacy `.claude` copy with one edited file and one known-outdated file is reported as legacy by status and update and stays byte-identical; `remove-legacy` refuses without the shared copy; after install it removes current and known-outdated files and keeps the edited file. Fails at 0363d4a. |
| REQ-cli-016, LIFE-14 | `tests/workflow_authority.rs` checks the `.agents` installed copy and that this repository has no `.claude/`; `tests/package.rs` keeps `.claude/` forbidden in the package. `tests/optional.rs::skills_install_per_host_and_update_only_unmodified_files` keeps its two-directory coverage with codex instead of claude. |

## Cases

- TC-cli-claude-host-shares-agents-copy (LIFE-12) binds `claude_host_installs_the_shared_agents_copy_once`; recorded failed before the fix.
- TC-cli-legacy-claude-copy-removed-only-explicitly (LIFE-13) binds `legacy_claude_copy_is_reported_and_removed_only_explicitly`; recorded failed before the fix.

## Focused and full checks

`cargo test --test optional`, the focused case through `chough cases run`, and `fledge lanes run verify` with hi 0.8.0, fledge 1.8.0 and specsync 6.1.2.

## Where these lessons go

- `specs/cli/context.md`
