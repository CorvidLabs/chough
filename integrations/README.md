# Integrations

This folder is optional. It holds example templates for hc, Herdr's terminal runner,
for running Chough agents as hc terminals, so a person can watch and steer them from
another screen. Chough has no runtime dependency on hc, Herdr or Rune: nothing in
`src/` or the shipped skill starts or calls them, and Chough works the same in any
terminal. Rune is a separate Fledge plugin, not Chough behavior (LIFE-24).

## Templates

| Host | Template | Command | `chough init --host` | Status |
|---|---|---|---|---|
| Claude Code | [claude-code/hc-template.md](claude-code/hc-template.md) | `claude` | `claude` | supported |
| Codex | [codex/hc-template.md](codex/hc-template.md) | `codex --no-daemon` | `openai` | supported |
| Kimi Code | [kimi/hc-template.md](kimi/hc-template.md) | `kimi` | `openai` | supported |
| Grok | [grok/hc-template.md](grok/hc-template.md) | `grok` | `grok` | supported |
| Local agent | [local/hc-template.md](local/hc-template.md) | `chough-local-agent` (a placeholder) | `openai` | unqualified until #220 |

Every template tells the agent to read the project's `AGENTS.md`, load the
chough-workflow skill, resume with `chough next --environment local`, stop at the human
gates and report back with `hc send`. They differ only in the command and the `--host`
value. The `--host` values follow [the host table](../docs/integrations.md#hosts):
Codex and Kimi use the shared `.agents/skills` copy (`openai`). Claude Code's
template also says to open the skill file directly, since Claude Code does not
discover `.agents/skills` on its own. Codex runs with `--no-daemon`, as hc requires.

The local template's command is a placeholder. Replace it with your local agent's
invocation. It counts as a supported host only once a local setup passes the
host-matrix run (#220).

## Using a template

hc reads templates from its own templates folder. Copy one there under its `name`, for
example `chough-codex.md`. Then start an agent on a scoped task:

```sh
hc run --bg --name <lane> --cwd <worktree> chough-codex "<task>"
```

The front matter keeps exactly the three keys hc documents: `name`, `description`
and `command`. The body is the agent's instructions, and hc adds your task after it.

## Versions

[templates.toml](templates.toml) lists each template with its version, its
`chough init --host` value, its status and the SHA-256 of its file. When you change a
template, raise its version and update its `sha256`. `tests/integrations.rs` fails
until both match, so a changed template never ships under an old version.

## Host-matrix runs

The 1.0 host-matrix harness (#218) keeps one `adapter.sh` beside each template. It
reads the adapter, and the template only for the optional hc driver. A harness result
is recorded against the host, not against hc.
