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
| grok | chough-grok | grok | openai | supported |
| local | chough-local | chough-local-agent (replace) | openai | unqualified |

Codex, Kimi, Grok and the local agent use `--host openai`, the shared
`.agents/skills` copy that docs/integrations.md records as observed. A project set up
for any other host already has it, and `--host grok` would install only `.grok/skills`
(found in t384's review of #253). The local template's command is a placeholder until #220 picks
and qualifies a setup.

`integrations/templates.toml` (`schema = "chough-integrations/1"`) has one
`[[template]]` per host with `host`, `file`, `name`, `init_host`, `status` and an
ordered `versions` list of `{ version, sha256 }`. The newest entry is the file's digest,
the versions run 1, 2, 3 and no digest repeats. An in-place edit of a listed entry is
left to review. A test also runs a real `chough init` per template and checks the
skill file the template names. The test is `tests/integrations.rs`
(module cli). It reads the files, uses the `toml` and `sha2` crates, and takes
the accepted `--host` values from the built binary's `chough init --help`.
