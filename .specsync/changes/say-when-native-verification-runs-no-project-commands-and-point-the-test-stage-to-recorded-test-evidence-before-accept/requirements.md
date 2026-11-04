---
change: say-when-native-verification-runs-no-project-commands-and-point-the-test-stage-to-recorded-test-evidence-before-accept
artifact: requirements
---

# Requirements

ADDED REQ-cli-038 (see `deltas/cli.md`). When SDD is enabled and
`.specsync/sdd.json` lists no verification commands, `next`, `status` and the `stage`
report say that native verification runs none of the project's commands; at the test
stage, `next` points to recording test evidence before accept. No gate changes. Trace:
FLOW-15, FLOW-6, LIFE-7.
