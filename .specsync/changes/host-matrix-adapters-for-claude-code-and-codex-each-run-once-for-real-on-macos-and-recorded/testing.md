---
change: host-matrix-adapters-for-claude-code-and-codex-each-run-once-for-real-on-macos-and-recorded
artifact: testing
---

# Testing

The cases are in `.chough/cases/cli.host-matrix-hosts.cases.toml`:

- **TC-cli-host-matrix-unavailable-turn** is bound to `sh tools/host-matrix/selftest.sh`.
  It expects the new line `ok: usage-limit -> UNAVAILABLE` and the final
  `HOST MATRIX SELFTEST PASSED`. It runs under dash and under bash.
- **TC-cli-host-matrix-claude-code** and **TC-cli-host-matrix-codex** are agent-assisted
  cases. Each is one real run, `sh tools/host-matrix/run.sh <host>`, on macOS arm64. Its
  verdict is recorded with `chough cases record`: pass only on a PASS result. The receipt
  goes with it as the summary and an artifact: host version, chough commit, log SHA-256
  and the failed checks.

The full suite runs at the final head.
