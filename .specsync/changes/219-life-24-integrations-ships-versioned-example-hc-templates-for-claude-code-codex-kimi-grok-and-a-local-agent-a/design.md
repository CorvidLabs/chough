---
change: 219-life-24-integrations-ships-versioned-example-hc-templates-for-claude-code-codex-kimi-grok-and-a-local-agent-a
artifact: design
---

# Design

The five template bodies share one text. Only the `--host` value differs, plus one
extra line for Claude Code, which does not discover `.agents/skills` on its own:

| Host | name | command | init host | status |
|---|---|---|---|---|
| claude-code | chough-claude | claude | claude | supported |
| codex | chough-codex | codex --no-daemon | openai | supported |
| kimi | chough-kimi | kimi | openai | supported |
| grok | chough-grok | grok | grok | supported |
| local | chough-local | chough-local-agent (replace) | openai | unqualified |

Codex and Kimi use `--host openai`, the shared `.agents/skills` copy that
docs/integrations.md records as observed. Grok's own value is unexercised until the
#218 harness runs it. The local template's command is a placeholder until #220 picks
and qualifies a setup.

`integrations/templates.toml` (`schema = "chough-integrations/1"`) has one
`[[template]]` per host with `host`, `file`, `name`, `version`,
`init_host`, `status` and `sha256`. The test is `tests/integrations.rs`
(module cli). It reads the files, uses the `toml` and `sha2` crates, and takes
the accepted `--host` values from the built binary's `chough init --help`.
