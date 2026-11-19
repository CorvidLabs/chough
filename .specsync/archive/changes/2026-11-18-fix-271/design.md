---
change: fix-271
artifact: design
---

# Design

- `--session <id>` is a global CLI option. clap leaves a subcommand's own argument of the same name in place, so `chough init --session <key>` keeps its meaning (the first session's key), and `chough inbox add --session` stays as it is. Every other command takes the global option, before or after its subcommand.
- `main` reads the option, or else `CHOUGH_SESSION`, once, before the command runs. It resolves the requested session with `sessions::resolve_open`, by ID, alias or bare slug as `session use` does, and pins it for the command with `sessions::pin`, the per-command resolution `checkpoint_path` already honors. A session that is not open refuses, naming it and `chough session list`, before anything is read or written. The pointer is never touched; archiving a session clears the pointer only when the pointer names that session. `init`, `inbox add` and the `session` commands are exempt, since they set or name the session themselves. `src/sessions.rs` (history) does not change.
- The five hc templates tell the agent to `export CHOUGH_SESSION=<its session's id>` after it opens or picks its session. `integrations/templates.toml` appends a new version for each (REQ-cli-065).
