---
kind: evidence
id: 2026-11-23-issue-438-timeline-cap-left-one-gate-closed-after-the-grouped
aliases: []
claim: Issue 438 Timeline cap left one gate closed after the grouped command failed at 4096 entries
result: failed
evidence_kind: product-test
revision: 0d5988f804155e7b3c285cace6cc0c52f540e035
content: 7e5b85682efb33171650644809014f8be254f13ed98cdd621719266068a6bd41
environment: macos-local
fingerprint: sha256:0c7726503a325847e0f1d84fc46f18cf8bd2c7abb1351c4402463541e3195d70
external: ext-b1b755
criteria: [FLOW-20]
requirements: [REQ-cli-061]
contracts: [specs/cli/cli.spec.md]
limitations: [Disposable synthetic gates drove no native step.]
observed: 2026-11-23T14:45:36Z
actor:
  - terminal: t14v
---
**Issue 438 Timeline cap left one gate closed after the grouped command failed at 4096 entries.**

Recorded as failed in macos-local at revision 0d5988f80415, observed 2026-11-23T14:45:36Z.

## Key failing lines

- At 4095 entries both gates were open. After exit 1, gate A was closed, gate B was pending, and the Timeline held 4096 entries.
