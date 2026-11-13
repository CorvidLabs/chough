---
kind: evidence
id: 2026-11-13-tc-cli-host-matrix-selftest-282f87
aliases: []
claim: "Red at 6457d8a8 under dash: HOST_MATRIX_FAULT=hang HOST_TURN_TIMEOUT=3 sh tools/host-matrix/run.sh stand-in, with sh as /bin/dash and stdin not a terminal, judged the run PASS after 1540 s: no turn was ended at its 3 s limit, so each stand-in turn's 300 s hang ran out."
result: failed
evidence_kind: product-test
case: [TC-cli-host-matrix-selftest]
case_digest: [c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2]
revision: 6457d8a880a013d1acb925136b7a9caca34f42fd
content: 724b40ff4a33d8eed5fd3f0c205c5a2ed40e9ae9e25c7f5ac18a6e08745f0326
environment: local-macos-arm64
criteria: [FLOW-21]
requirements: [REQ-cli-066]
contracts: []
limitations: [dash on macOS arm64 stands in for Ubuntu's sh]
observed: 2026-11-13T07:24:36Z
actor:
  - agent: claude-code
    model: claude-opus-5-5
    terminal: t38q
---
**Red at 6457d8a8 under dash: HOST_MATRIX_FAULT=hang HOST_TURN_TIMEOUT=3 sh tools/host-matrix/run.sh stand-in, with sh as /bin/dash and stdin not a terminal, judged the run PASS after 1540 s: no turn was ended at its 3 s limit, so each stand-in turn's 300 s hang ran out.**

Recorded as failed in local-macos-arm64 at revision 6457d8a880a0, observed 2026-11-13T07:24:36Z.

## Key failing lines

- run.sh: 171: set: can't access tty; job control turned off
- {"verdict":"PASS","turns_completed":5,"failed":[]}
- elapsed=1540s at 6457d8a8
