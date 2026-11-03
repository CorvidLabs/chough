---
id: claude-host-installs-the-shared-agents-skill-copy-legacy-claude-copies-are-reported-and-removed-only-on-an-explicit
state: archived
type: feature
base_commit: 0363d4af368bb07cc07f16c2f5fff7645c1c05bc
---

# Claude host installs the shared .agents skill copy, legacy .claude copies are reported and removed only on an explicit command, and this repository drops its .claude copy

## Intent

Claude host installs the shared .agents skill copy, legacy .claude copies are reported and removed only on an explicit command, and this repository drops its .claude copy

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- `chough init --host claude` and `chough skill update --host claude` write only the shared `.agents/skills/chough-workflow/` copy, and a request naming both claude and openai installs one copy, not two
- `chough skill status` and `chough skill update` report an installed `.claude/skills/chough-workflow/` copy as legacy and never delete, update or rewrite it; `chough init` never deletes it
- `chough skill remove-legacy --host claude` removes only legacy files byte-identical to the current or a known earlier bundle, keeps edited or unknown files, and refuses with nothing removed when the shared `.agents` copy is missing
- This repository has no `.claude` skill copy, `CLAUDE.md` routes Claude Code to `AGENTS.md` and the shared copy, and `docs/integrations.md` states that Claude Code 2.1.316 did not discover `.agents/skills` alone
- codex, cursor, gemini and grok keep their existing directories; fmt, lint, the full test suite and `fledge lanes run verify` pass

## No-spec Rationale

Not applicable
