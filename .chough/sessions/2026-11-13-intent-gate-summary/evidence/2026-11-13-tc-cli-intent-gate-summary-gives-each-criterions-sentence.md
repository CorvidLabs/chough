---
kind: evidence
id: 2026-11-13-tc-cli-intent-gate-summary-gives-each-criterions-sentence
aliases: []
claim: "Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line for TC-cli-intent-gate-summary-gives-each-criterions-sentence: failed (exit 101, tests run 1)."
result: failed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-intent-gate-summary-gives-each-criterions-sentence]
case_digest: [ee99a08d2f8ab309c7f29f482e92c61b03157bce9576825f37a1a4f6a3ef0dd8]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - gates
  - "--"
  - "--exact"
  - an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line
exit_code: 101
tests_run: 1
duration_seconds: 1
revision: 0f34dbe2b3c86c306d2f7dc95102c54115053209
content: d24d7aa7431e82f0b13decfced5489c9b5dfe4a80a7080a83460ee90036ba7ee
environment: local-macos-arm64
fingerprint: sha256:10f6261f4a552f605deb6016ecdabf66fa5c81648621aec9d35aee275c017eed
criteria: [FLOW-19, FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-13T02:25:16Z
actor:
  - agent: claude-code
    terminal: t382
---
**Ran cargo test --locked --test gates -- --exact an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line for TC-cli-intent-gate-summary-gives-each-criterions-sentence: failed (exit 101, tests run 1).**

Recorded as failed in local-macos-arm64 at revision 0f34dbe2b3c8, observed 2026-11-13T02:25:16Z. The raw log stays in the machine-local store.

## Key failing lines

- ---- an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line stdout ----
- thread 'an_intent_gate_summary_gives_each_criterions_sentence_and_no_diff_line' (930819295) panicked at tests/gates.rs:847:5:
- assertion `left == right` failed: Gate: Approve the tally intent
- Intent: TALLY-2, TALLY-1, TALLY-99 (the criteria the session references)
-   hi/notes.md: TALLY-2
-   hi/tally.md: TALLY-2, TALLY-1
-   in no hi/ file: TALLY-99
- Tests: 0 passed, 0 failed, 0 not run
- Risks: none recorded
- Diff: unknown, with no merge base and no recorded base commit
-   left: ["  hi/notes.md: TALLY-2", "  hi/tally.md: TALLY-2, TALLY-1", "  in no hi/ file: TALLY-99", "Tests: 0 passed, 0 failed, 0 not run"]
-  right: ["  hi/tally.md", "    TALLY-2  Running tally with --lines prints the number of lines instead of words.", "    TALLY-1  Piping text into tally prints the number of words.", "  in no hi/ file: TALLY-99"]
- note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
- test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.92s
