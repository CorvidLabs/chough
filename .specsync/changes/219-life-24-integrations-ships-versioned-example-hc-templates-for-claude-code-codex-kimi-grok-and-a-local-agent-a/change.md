---
id: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
state: approved
type: feature
base_commit: 63200149608d01550722a2033cadc83103c69c9a
---

# #219 LIFE-24: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, a manifest with each template's version and digest, and a README; Chough never runs hc, Herdr or Rune

## Intent

#219 LIFE-24: integrations/ ships versioned example hc templates for Claude Code, Codex, Kimi, Grok and a local agent, a manifest with each template's version and digest, and a README; Chough never runs hc, Herdr or Rune

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- integrations/ holds a README and one hc template per host (claude-code, codex, kimi, grok, local) whose front matter is exactly name, description and command, whose body runs chough next and names a chough init --host value chough accepts, with no absolute or home path; integrations/templates.toml lists each with its version, init host, status (local unqualified until #220) and SHA-256, and a changed template fails tests/integrations.rs until its digest and version are updated; src/ never runs hc, Herdr or Rune and the skill never names them (red then green in tests/integrations.rs)

## No-spec Rationale

Not applicable
