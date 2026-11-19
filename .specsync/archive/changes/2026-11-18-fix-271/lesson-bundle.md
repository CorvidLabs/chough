# Lesson bundle: fix-271

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 271
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/main.rs, tests/session_pin.rs, docs/cli.md, integrations/claude-code/hc-template.md, integrations/codex/hc-template.md, integrations/grok/hc-template.md, integrations/kimi/hc-template.md, integrations/local/hc-template.md, integrations/templates.toml, .chough/cases/cli.session-pin.cases.toml, changelog.d/2026-11-18-fix-271.md
- **Acceptance**: With two open sessions and the pointer on one, chough evidence add --session <other>, and the same command with CHOUGH_SESSION=<other>, record into the other session and leave the pointer unchanged, and chough next --session <other> shows that session. A --session or CHOUGH_SESSION naming an archived or unknown session refuses with every file unchanged. chough init --session and chough inbox add --session keep their meaning, and session new and session use ignore CHOUGH_SESSION. The five versioned hc templates in integrations/ tell the agent to export CHOUGH_SESSION for its session.

## Evidence

- Verification commit: `764c420437fd806e352512a184fccb78355014e4`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #271, from the 0.6.0 swarm report (#273, s3). Every session-scoped write resolves the one shared pointer, `.chough/local/active-session`. So when two agents work in one worktree and one runs `chough session use` for its own session, the other agent's next `evidence add`, `checkpoint` or `bug add` lands in the wrong session, silently and with exit 0. Only `chough inbox add` took a `--session`.

Leif's decision (2026-11-18, planning interview, recorded on #271): pin per shell. A `CHOUGH_SESSION` environment variable and a `--session <id>` flag on every write command make each agent write to its own session, whatever the shared pointer says, and hc agent launches set the variable. He then approved the requirement wording in REQ-cli-037's delta ("Approve") and chose "Yes, same PR" for the hc templates: the five versioned templates in `integrations/` tell the agent to export `CHOUGH_SESSION` for its session. Both were relayed by hc t4c.

## From the change's design.md

# Design

- `--session <id>` is a global CLI option. clap leaves a subcommand's own argument of the same name in place, so `chough init --session <key>` keeps its meaning (the first session's key), and `chough inbox add --session` stays as it is. Every other command takes the global option, before or after its subcommand.
- `main` reads the option, or else `CHOUGH_SESSION`, once, before the command runs. It resolves the requested session with `sessions::resolve_open`, by ID, alias or bare slug as `session use` does, and pins it for the command with `sessions::pin`, the per-command resolution `checkpoint_path` already honors. A session that is not open refuses, naming it and `chough session list`, before anything is read or written. The pointer is never touched; archiving a session clears the pointer only when the pointer names that session. `init`, `inbox add` and the `session` commands are exempt, since they set or name the session themselves. `src/sessions.rs` (history) does not change.
- The five hc templates tell the agent to `export CHOUGH_SESSION=<its session's id>` after it opens or picks its session. `integrations/templates.toml` appends a new version for each (REQ-cli-065).

## From the change's testing.md

# Testing

`tests/session_pin.rs`:
- `a_pinned_shell_writes_its_own_session_whatever_the_pointer_says`: two open sessions, with the pointer on one. `evidence add --session <other>` and `evidence add` with `CHOUGH_SESSION=<other>` each record into the other session, the pointer is unchanged, and `next --session <other>` shows that session. Fails on main.
- `a_pin_to_a_session_that_is_not_open_refuses_with_nothing_written`: `--session` and `CHOUGH_SESSION` naming an unknown or archived session refuse with every file unchanged. Fails on main.
- `init_and_inbox_add_keep_their_session_and_session_use_ignores_the_variable`.

`tests/integrations.rs` keeps checking the templates' versions in `integrations/templates.toml`. Case `TC-cli-a-pinned-shell-writes-its-own-session` binds the first test.

## Where these lessons go

- `specs/cli/cli.3md`
