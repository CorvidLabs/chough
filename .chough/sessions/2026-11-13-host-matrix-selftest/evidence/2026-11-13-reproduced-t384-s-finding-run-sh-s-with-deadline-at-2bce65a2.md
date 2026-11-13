---
kind: evidence
id: 2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2
aliases: []
claim: "Reproduced t384's finding: run.sh's with_deadline at 2bce65a2, run by /bin/dash with stdin not a terminal, let 'with_deadline 2 sleep 8' run the full 8 s and return 0; bash ends it at 2 s."
result: failed
evidence_kind: product-test
revision: 0886a48aaebf05878bdb15a7be1b6ae73e88abf9
content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
environment: local-macos-arm64
criteria: [FLOW-21]
requirements: [REQ-cli-066]
contracts: []
limitations:
  - a minimal reproduction of the function, not a full harness run; dash on macOS arm64 stands in for Ubuntu's sh
observed: 2026-11-13T06:57:48Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Reproduced t384's finding: run.sh's with_deadline at 2bce65a2, run by /bin/dash with stdin not a terminal, let 'with_deadline 2 sleep 8' run the full 8 s and return 0; bash ends it at 2 s.**

Recorded as failed in local-macos-arm64 at revision 0886a48aaebf, observed 2026-11-13T06:57:48Z.

## Key failing lines

- set: can't access tty; job control turned off
- status=0 elapsed=8s
