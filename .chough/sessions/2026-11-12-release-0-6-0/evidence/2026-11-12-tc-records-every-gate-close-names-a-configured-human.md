---
kind: evidence
id: 2026-11-12-tc-records-every-gate-close-names-a-configured-human
aliases: []
claim: "Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-records-every-gate-close-names-a-configured-human]
case_digest: [0cc23c38572566605fb9a9bdc0daed35c6d6a3de5e4db2aac8e74afd4945bf42]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - humans
  - "--"
  - "--exact"
  - every_gate_close_names_a_configured_human
exit_code: 101
tests_run: 1
duration_seconds: 2
revision: ab5426a506a54d3ceb2b6594e6a223761200676a
content: 6ede595e9edcc24100106f9d816a4b060c3c173e358522c42397ec0ba84de954
environment: local-macos-arm64
fingerprint: sha256:b93eb6d2e5ce28725a721bc3a23e4dcfe1c807ec588735491dc8c49145ef7fbb
criteria: [FLOW-16, FLOW-23]
requirements: [REQ-records-008]
contracts: [specs/records/records.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T19:57:33Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test humans -- --exact every_gate_close_names_a_configured_human for TC-records-every-gate-close-names-a-configured-human: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision ab5426a506a5, observed 2026-11-12T19:57:33Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- every_gate_close_names_a_configured_human stdout ----
- thread 'every_gate_close_names_a_configured_human' (921965363) panicked at tests/common/mod.rs:165:9:
- ["checkpoint", "--close-gate", "Approve the intent", "--decided-by", "someone not configured", "--words", "yes, go", "--where", "SYNTHETIC chat, 2026-11-11", "--recorded-by", "SYNTHETIC-agent"] unexpectedly succeeded: {
-   "checkpoint": {
-     "awaiting_human": [
-       "Implementation review"
-     ],
-     "blockers": [],
-     "bugs": [],
-     "completed": [],
-     "decisions": [
-       "someone not configured decided \"Approve the intent\": \"yes, go\" (SYNTHETIC chat, 2026-11-11); recorded by SYNTHETIC-agent; a relayed claim, not authenticated"
-     ],
-     "intent_scope": "LIFE",
-     "modules": [
-       "cli"
-     ],
-     "native_change": "human-change",
-     "next_action": "Gate \"Approve the intent\" closed (closed); run `chough next` for the next action.",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 12 filtered out; finished in 1.17s
