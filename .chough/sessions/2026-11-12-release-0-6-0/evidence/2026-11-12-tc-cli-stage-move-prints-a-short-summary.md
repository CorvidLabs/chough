---
kind: evidence
id: 2026-11-12-tc-cli-stage-move-prints-a-short-summary
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-stage-move-prints-a-short-summary]
case_digest: [7473a8983adb456754af04271c4b35af8b83d62bd6c38d629f07cf83e76d7f0e]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - a_stage_move_prints_a_short_summary_and_its_full_report_under_json
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: fc6ec27ae1f07c5b83178490af74d6f7e49d9515
content: 38803e6e40cd2e7aa4b518aac802ae53275bba0b549a49abbbbd77f02daa82dc
environment: local-macos-arm64
fingerprint: sha256:532381357b094247f7f6477c5390e5087d30105f21f826374e5b3f1c089ed292
criteria: [FLOW-2]
requirements: [REQ-cli-008]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-12T22:21:15Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test stages -- --exact a_stage_move_prints_a_short_summary_and_its_full_report_under_json for TC-cli-stage-move-prints-a-short-summary: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision fc6ec27ae1f0, observed 2026-11-12T22:21:15Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- a_stage_move_prints_a_short_summary_and_its_full_report_under_json stdout ----
- thread 'a_stage_move_prints_a_short_summary_and_its_full_report_under_json' (925854708) panicked at tests/stages.rs:3263:5:
- the default output is JSON:
- {
-   "applied": {
-     "from": "intent",
-     "gate": null,
-     "kind": "advance",
-     "to": "design"
-   },
-   "authority": "Native SpecSync status decides lifecycle gates. Chough runs a native approval or review only when a human's decision closes a gate that drives one, as that configured human; the relayed decision it keeps is a claim, not authenticated.",
-   "awaiting_human": [],
-   "native_change": null,
-   "recent_transitions": [
-     {
-       "at_unix_seconds": 1794522072,
-       "from": "intent",
-       "gate": null,
-       "id": "t-d49299",
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 49 filtered out; finished in 0.63s
