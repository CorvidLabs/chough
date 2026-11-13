---
kind: evidence
id: 2026-11-13-clean-cargo-install-chough-version-0-6-0-locked-from-crates-io
aliases: []
claim: Clean cargo install chough --version 0.6.0 --locked from crates.io into an empty root with a fresh target directory on macOS arm64 succeeded; the installed chough --version printed chough 0.6.0
result: passed
evidence_kind: installation-observation
method: manual
revision: 443c63ab65b747f47248ad2166f8fb299da94a41
content: b056982fd067aff750b535081ae04bbaf4da1742935833f4a5a8559c6a739214
environment: macos-arm64-crates-io
criteria: [LIFE-25]
requirements: []
contracts: []
limitations:
  - Run by the coordinator, claude-code-t370, at about 2026-11-13T01:45Z and relayed to t382; t382 did not run it
  - The cargo version used was not reported
observed: 2026-11-13T01:47:22Z
actor:
  - agent: claude-code
    terminal: t382
---
**Clean cargo install chough --version 0.6.0 --locked from crates.io into an empty root with a fresh target directory on macOS arm64 succeeded; the installed chough --version printed chough 0.6.0.**

Recorded as passed in macos-arm64-crates-io at revision 443c63ab65b7, observed 2026-11-13T01:47:22Z.
