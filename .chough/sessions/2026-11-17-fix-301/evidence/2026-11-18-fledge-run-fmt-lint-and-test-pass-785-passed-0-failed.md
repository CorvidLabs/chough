---
kind: evidence
id: 2026-11-18-fledge-run-fmt-lint-and-test-pass-785-passed-0-failed
aliases: []
claim: "fledge run fmt, lint and test pass (785 passed, 0 failed), including the two new tests in tests/cli.rs: chough help --help exits 0 with the top-level help, and chough checkpoint --help prints a description under each of the eleven list-valued flags (fails on the old checkpoint.rs); docs/cli.md has no change ready text, per Leif's decision"
result: passed
evidence_kind: product-test
revision: a265791f12cfe2dfeea8ff44d4f1ae37d8ca3640
content: 7995adc76a17798018c53e50db951b1a222e4af722f14c3d303ed9c48641a5be
environment: local Mac, cargo via fledge, branch merged with main at a265791
criteria: [DRIVE-17]
requirements: []
contracts: []
limitations: []
observed: 2026-11-18T03:35:36Z
actor:
  - agent: claude-code
    terminal: t28
---
**fledge run fmt, lint and test pass (785 passed, 0 failed), including the two new tests in tests/cli.rs: chough help --help exits 0 with the top-level help, and chough checkpoint --help prints a description under each of the eleven list-valued flags (fails on the old checkpoint.rs); docs/cli.md has no change ready text, per Leif's decision.**

Recorded as passed in local Mac, cargo via fledge, branch merged with main at a265791 at revision a265791f12cf, observed 2026-11-18T03:35:36Z.
