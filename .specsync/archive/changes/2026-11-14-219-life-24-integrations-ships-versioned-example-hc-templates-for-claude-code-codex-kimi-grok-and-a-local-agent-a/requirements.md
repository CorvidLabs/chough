---
change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
artifact: requirements
---

# Requirements

REQ-cli-065 (added): `integrations/` ships one example hc template for each of
claude-code, codex, kimi, grok and local. Each template has exactly hc's three
front-matter keys, resumes with `chough next`, names an accepted `chough init
--host` value and holds no local path. A manifest lists each template's status and
ordered versions with their SHA-256, so a changed template fails until the next version
with its digest is appended. An in-place edit of a listed entry is left to review. A README
says the folder is optional. Chough never starts hc, Herdr or Rune, and `src/` and
the skill never name Herdr or Rune. The full text is in `deltas/cli.md`.
