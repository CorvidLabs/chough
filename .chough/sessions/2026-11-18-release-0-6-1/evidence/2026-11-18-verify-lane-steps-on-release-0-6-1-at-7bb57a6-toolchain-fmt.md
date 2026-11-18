---
kind: evidence
id: 2026-11-18-verify-lane-steps-on-release-0-6-1-at-7bb57a6-toolchain-fmt
aliases: []
claim: "Verify lane steps on release/0.6.1 at 7bb57a6: toolchain, fmt, lint, intent, spec, audit, cases, leaks, exercise and build passed. The test step failed once (tests/evidence.rs: parallel_decisions_and_evidence_adds_all_land and evidence_add_rereads_the_session_under_the_writer_lock, from #324, lost updates) and then passed in two full reruns; the two tests also passed 3 times alone and 48 times under eight-way load. The one failure was not reproduced or explained."
result: passed
evidence_kind: product-test
revision: 7bb57a609060fb2b53e672466bb689e431b75f5d
content: 563235bb22691510e88467a7e7779d3fe64f63da178e70c3136a44177b461ec2
environment: local-macos-arm64
criteria: [LIFE-25]
requirements: []
contracts: []
limitations: ["One unexplained failure of two #324 tests under a machine load average near 40"]
observed: 2026-11-18T17:31:01Z
actor:
  - agent: fable
    model: claude-fable-5-2
    terminal: t4b
---
**Verify lane steps on release/0.6.1 at 7bb57a6: toolchain, fmt, lint, intent, spec, audit, cases, leaks, exercise and build passed. The test step failed once (tests/evidence.rs: parallel_decisions_and_evidence_adds_all_land and evidence_add_rereads_the_session_under_the_writer_lock, from #324, lost updates) and then passed in two full reruns; the two tests also passed 3 times alone and 48 times under eight-way load. The one failure was not reproduced or explained.**

Recorded as passed in local-macos-arm64 at revision 7bb57a609060, observed 2026-11-18T17:31:01Z.
