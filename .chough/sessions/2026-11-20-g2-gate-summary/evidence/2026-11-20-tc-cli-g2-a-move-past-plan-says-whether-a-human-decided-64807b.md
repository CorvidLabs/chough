---
kind: evidence
id: 2026-11-20-tc-cli-g2-a-move-past-plan-says-whether-a-human-decided-64807b
aliases: []
claim: "Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1)."
result: passed
evidence_kind: product-test
method: automated
track: functional
case: [TC-cli-g2-a-move-past-plan-says-whether-a-human-decided]
case_digest: [2c7ef7f4a08553661e5c362f7169727fe426dffbae8c100f2ea9821c97f150c4]
command:
  - cargo
  - test
  - "--locked"
  - "--test"
  - stages
  - "--"
  - "--exact"
  - without_specsync_a_move_past_plan_says_whether_a_human_decided
exit_code: 0
tests_run: 1
duration_seconds: 0
revision: 35859e1516e1fa57a436c12188c3b3aa7263b420
content: db21fe4862a1a7deb857a290ea4afff285aad1eecddf04ee171bcd6e10f6fcdb
environment: local-macos-arm64
fingerprint: sha256:0051d90e19884695a6d6fe6386c1aa9b797052eec24b1468a3e66e580ed348ec
criteria: [DRIVE-3]
requirements: [REQ-cli-031]
contracts: [specs/cli/cli.spec.md]
limitations: [inherited environment variables were not recorded]
observed: 2026-11-20T18:43:51Z
actor:
  - agent: fable
    model: claude-opus-5-5
    terminal: t85
---
**Ran cargo test --locked --test stages -- --exact without_specsync_a_move_past_plan_says_whether_a_human_decided for TC-cli-g2-a-move-past-plan-says-whether-a-human-decided: passed (exit 0, tests run 1).**

Recorded as passed in local-macos-arm64 at revision 35859e1516e1, observed 2026-11-20T18:43:51Z. The raw log stays in the machine-local store.
