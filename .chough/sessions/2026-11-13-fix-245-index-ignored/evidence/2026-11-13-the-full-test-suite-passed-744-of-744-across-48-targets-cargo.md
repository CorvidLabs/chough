---
kind: evidence
id: 2026-11-13-the-full-test-suite-passed-744-of-744-across-48-targets-cargo
aliases: []
claim: "The full test suite passed 744 of 744 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 722a7502, #245's final head; clippy --all-targets -D warnings and rustfmt --check were clean"
result: passed
evidence_kind: product-test
method: automated
revision: 722a7502ed4ce56819174c5b5fa7d604043cb885
content: 14ae5c7768cc73445778ca916d97ecf4ac15a413a61d40a518e93a3655002911
environment: local-macos-arm64
criteria: [KNOW-14]
requirements: []
contracts: []
limitations: [The raw log is not committed]
observed: 2026-11-13T15:15:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**The full test suite passed 744 of 744 across 48 targets (cargo +1.96.0 test --locked --no-fail-fast, under the heavy-tests lock) at 722a7502, #245's final head; clippy --all-targets -D warnings and rustfmt --check were clean.**

Recorded as passed in local-macos-arm64 at revision 722a7502ed4c, observed 2026-11-13T15:15:33Z.
