---
kind: evidence
id: 2026-11-12-reproduced-the-jackdaw-finding-t38f-on-a-rehearsal-migrated
aliases: []
claim: "Reproduced the jackdaw finding (t38f, on a rehearsal-migrated jackdaw tree whose case files set XDG_DATA_HOME under the temporary root, 4 findings) with the 0.6.0 build at 8d310373 in a throwaway project: a committed case file whose command binding sets XDG_DATA_HOME to a path under the temporary root (argv env XDG_DATA_HOME=<tmp>/probe-data true) made chough doctor report a leak finding (match system-path, the case file, line 19) with the fix chough doctor --fix --redact, which would rewrite the case's deliberate test setting and break the case"
result: failed
evidence_kind: product-test
method: manual
revision: 8d310373fdb232f3055c710add18018a97dcd3a7
content: 90c8d587800446bb2bb011a5b5dc6c80689d6cd007912ebd26232b94563876c4
environment: local-macos-arm64
criteria: [LIFE-26]
requirements: [REQ-records-006]
contracts: [specs/records/records.spec.md]
limitations: []
observed: 2026-11-12T15:40:46Z
actor:
  - agent: claude-code
    terminal: t382
---
**Reproduced the jackdaw finding (t38f, on a rehearsal-migrated jackdaw tree whose case files set XDG_DATA_HOME under the temporary root, 4 findings) with the 0.6.0 build at 8d310373 in a throwaway project: a committed case file whose command binding sets XDG_DATA_HOME to a path under the temporary root (argv env XDG_DATA_HOME=<tmp>/probe-data true) made chough doctor report a leak finding (match system-path, the case file, line 19) with the fix chough doctor --fix --redact, which would rewrite the case's deliberate test setting and break the case.**

Recorded as failed in local-macos-arm64 at revision 8d310373fdb2, observed 2026-11-12T15:40:46Z.

## Key failing lines

- {"fix": "chough doctor --fix --redact", "kind": "leak", "line": 19, "match": "system-path", "target": ".chough/cases/cli.probe.cases.toml"}
