---
change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
artifact: testing
---

# Testing

REQ-cli-066 is verified by TC-cli-host-matrix-selftest, bound to
`sh tools/host-matrix/selftest.sh` with `expect_output` `HOST MATRIX SELFTEST PASSED`,
in `.chough/cases/cli.host-matrix.cases.toml`. It runs the harness eight times with the
stand-in host:

- the good run is PASS;
- uncommitted work fails "turn 1: the tree is committed";
- a skipped scope gate fails "turn 2: the scope gate is open";
- the host approving the change itself fails "turn 2: the change waits in draft";
- an extra product file fails "turn 3: only the allowed product files changed";
- a wrong cold resume fails "turn 5: the cold resume states stage done";
- a hung turn fails on its time limit, and its whole process group ends;
- a host that cannot run reads UNAVAILABLE.

Platforms: the case runs locally on macOS arm64, and the hosted job runs it on
`ubuntu-latest` and `macos-latest`. It is run once with `GIT_CONFIG_GLOBAL=/dev/null`,
as in hosted CI. The full `cargo test`, fmt, clippy and the verify lane run at the
final head.
