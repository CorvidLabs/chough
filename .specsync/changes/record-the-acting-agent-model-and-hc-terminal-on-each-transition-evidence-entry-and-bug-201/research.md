---
change: record-the-acting-agent-model-and-hc-terminal-on-each-transition-evidence-entry-and-bug-201
artifact: research
---

# Research

Observed on macOS, 2026-11-10: Claude Code exports `CLAUDECODE=1` and `CLAUDE_CODE_ENTRYPOINT` to commands it runs; hc exports `HC_TERMINAL_ID` (for example `t37j`), `HC_SCOPE` and `HC_TERMINAL_SOCKET`. No model variable is exported by Claude Code. Kimi Code's shell (its handoff, same day) showed no Kimi-specific variable.
