- `integrations/` ships optional, versioned example hc templates for Claude Code,
  Codex, Kimi, Grok and a local agent, the local one unqualified until #220
  (REQ-cli-065, #219). Each has hc's three front-matter keys, resumes with
  `chough next`, loads the skill file its own `chough init` creates and holds no local
  path. `integrations/templates.toml` lists each template's status and an ordered list
  of its versions with their SHA-256, and a README says Chough has no runtime
  dependency on hc, Herdr or Rune.
