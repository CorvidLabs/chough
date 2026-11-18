---
kind: evidence
id: 2026-11-18-ran-the-tester-s-parallel-repro-60-checkpoint-and-60-evidence
aliases: []
claim: "Ran the tester's parallel repro (60 checkpoint and 60 evidence add writers at once, chough-swarm-0611/runs/s3/repro/f2.sh) three times against this branch's debug build: all 120 writers exited 0 in each run, with no swept temporary file."
result: passed
evidence_kind: product-test
revision: 0cd19f98cde9dd17435a28021ebc7e35cc274c17
content: b45755596385ee5f8bff5c2085b75705c28dd9e69d31452c41c26fcb2723a4f0
environment: local-macos-arm64
criteria: [DRIVE-25]
requirements: [REQ-records-001]
contracts: []
limitations: []
observed: 2026-11-18T16:29:54Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Ran the tester's parallel repro (60 checkpoint and 60 evidence add writers at once, chough-swarm-0611/runs/s3/repro/f2.sh) three times against this branch's debug build: all 120 writers exited 0 in each run, with no swept temporary file.**

Recorded as passed in local-macos-arm64 at revision 0cd19f98cde9, observed 2026-11-18T16:29:54Z.
