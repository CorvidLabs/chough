---
kind: evidence
id: 2026-11-12-tc-cli-context-reads-agents-md-from-the-git-root
aliases: []
claim: "Ran cargo test --locked --test subdirectory -- --exact context_reads_agents_md_from_the_git_root_down_only for TC-cli-context-reads-agents-md-from-the-git-root: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-context-reads-agents-md-from-the-git-root]
case_digest: [4cc31020a8c0670707d9224e0f69aea72b38861fd0319f474ff668f3429896ce]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - subdirectory
  - "--"
  - "--exact"
  - context_reads_agents_md_from_the_git_root_down_only
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: dd3e2ab5df329a8dc68f9f89725310b2b56e029f
content: 6287c845d27db39c417052a4608cf898a2c1017c06b5e7c461b069727288236f
environment: local-macos-arm64
fingerprint: sha256:3b2fa0427483e7871f4ec1443464e6df7f7e478ca965f8b83996397962a28820
criteria: [LIFE-3]
requirements: [REQ-cli-002]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T17:42:36Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test subdirectory -- --exact context_reads_agents_md_from_the_git_root_down_only for TC-cli-context-reads-agents-md-from-the-git-root: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision dd3e2ab5df32, observed 2026-11-12T17:42:36Z. 3 recorded lines were redacted. The raw log stays in the machine-local store.

## Key failing lines

- ---- context_reads_agents_md_from_the_git_root_down_only stdout ----
- thread 'context_reads_agents_md_from_the_git_root_down_only' (921046665) panicked at tests/subdirectory.rs:310:5:
- assertion `left == right` failed: {
-   "schema_version": 1,
-   "sections": [
-     {
-       "content": "SYNTHETIC-ABOVE-THE-REPOSITORY\n",
-       "source": "<redacted:system-path>",
-       "state": "present"
-     },
-     {
-       "content": "SYNTHETIC-REPOSITORY-ROOT\n",
-       "source": "<redacted:system-path>",
-       "state": "present"
-     },
-     {
-       "content": "SYNTHETIC-PROJECT\n",
-       "source": "<redacted:system-path>",
-       "state": "present"
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 3 filtered out; finished in 1.02s
