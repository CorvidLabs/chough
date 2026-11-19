---
kind: session
id: 2026-11-18-fix-347
aliases: [fix-347]
objective: A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347)
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: fix-347
pr: ""
criteria: [DRIVE-16]
requirements: [REQ-cli-012]
cases:
  - TC-cli-a-new-bugs-log-links-its-evidence
  - TC-cli-doctor-relinks-a-bugs-evidence-after-archiving
design: []
files: [src/bugs.rs, src/doctor.rs, tests/bugs.rs]
observations:
  - 2026-11-18-tc-cli-a-new-bugs-log-links-its-evidence
  - 2026-11-18-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving
  - 2026-11-18-tc-cli-a-new-bugs-log-links-its-evidence-27a1ae
  - 2026-11-18-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving-136a4e
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of fix-347
    holds: accept
    drives: review
    opened: 2026-11-18T23:14:52Z
run_consents: []
next: Wait for the tester's check and Leif's implementation review of fix-347
---
# A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347)

## Goal

A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347)

## Where it stands

- New bugs link their evidence and doctor relinks after archiving; both tests fail on main and pass with the change

## Decisions and why

- 0xLeif decided "Scope approval of fix-347": "both" (GitHub issue #347, which records 'Leif chose both' on 2026-11-18 for its two follow-ups (new bugs get real links; links follow archiving, by repointing or by doctor reporting and fixing); his planning session (hc t4c) relayed it to fable-t4b as his scope approval and asked for the smaller honest option on the second, which is the doctor fix); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-347 as 0xLeif
- 0xLeif decided "Scope approval of fix-347, rebased onto main (native approval)": "Yes, for today" (Leif's standing rule of 2026-11-18, relayed by his planning session (hc t4c): when an approved PR's delta goes stale only because main changed the same requirement, and the rebase puts the same added sentences on the newer text with nothing else changed, fable-t4b may record his relayed re-approval and report the PR and requirement. Here: PR #360, REQ-cli-012 and REQ-history-012); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve fix-347 as 0xLeif

## What was verified

- [Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-new-bugs-log-links-its-evidence.md)
- [Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving.md)
- [Ran cargo test --locked --test bugs -- --exact a_new_bugs_log_links_its_evidence_relative_to_the_bug for TC-cli-a-new-bugs-log-links-its-evidence: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-a-new-bugs-log-links-its-evidence-27a1ae.md)
- [Ran cargo test --locked --test bugs -- --exact doctor_relinks_a_bugs_evidence_after_its_session_is_archived for TC-cli-doctor-relinks-a-bugs-evidence-after-archiving: passed (exit 0, tests run 1).](evidence/2026-11-18-tc-cli-doctor-relinks-a-bugs-evidence-after-archiving-136a4e.md)

## Open threads

## Timeline

- 2026-11-18T23:00:35Z · start · intent → intent · A new bug's Log links its evidence with relative links, and chough doctor reports and fixes a bug link left stale when its session is archived (#347) ^t-c54c0d
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:00:35Z · advance · intent → design ·  ^t-e82acb
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:00:35Z · skip · design → plan · Record content change; nothing visual or interactive changes ^t-f0ebf6
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:07Z · gate-opened · plan → plan · Scope approval of fix-347 ^t-87ba23
  - native_change: fix-347
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:07Z · gate-closed · plan → plan · Scope approval of fix-347 ^t-1d0cc8
  - native_change: fix-347
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - relayed_by:
    - decided_by: 0xLeif
    - words: both
    - where: GitHub issue #347, which records 'Leif chose both' on 2026-11-18 for its two follow-ups (new bugs get real links; links follow archiving, by repointing or by doctor reporting and fixing); his planning session (hc t4c) relayed it to fable-t4b as his scope approval and asked for the smaller honest option on the second, which is the doctor fix
    - recorded_by: fable-t4b
    - native: specsync change approve fix-347
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:09Z · advance · plan → build · native:approved ^t-e50f6f
  - native_change: fix-347
  - gate: native:approved
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:08:09Z · advance · build → test · native:approved ^t-e49786
  - native_change: fix-347
  - gate: native:approved
  - revision: dbd0b1f7c772ae17e4b0ff59bd92a7024bfffe2b
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:14:52Z · advance · test → accept · native:verifying ^t-d18465
  - native_change: fix-347
  - gate: native:verifying
  - revision: 8072f4dd5c336fae3b3053c464cfde4938520188
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T23:14:52Z · gate-opened · accept → accept · Implementation review of fix-347 ^t-c9e1ca
  - native_change: fix-347
  - revision: 8072f4dd5c336fae3b3053c464cfde4938520188
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T01:52:50Z · gate-opened · accept → accept · Scope approval of fix-347, rebased onto main (native approval) ^t-5a48a4
  - native_change: fix-347
  - revision: 83f87dd373bbdb02efb7554bb44b8d8a96cdb92f
  - holds: accept
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T01:52:50Z · direct-native-write · accept → accept · fix-347's lifecycle records changed outside Chough: state.json changed, approvals.json changed; state verifying → draft ^t-da1e8e
  - native_change: fix-347
  - revision: 83f87dd373bbdb02efb7554bb44b8d8a96cdb92f
  - direct_write:
    - change: fix-347
    - files:
      - state.json: changed
      - approvals.json: changed
    - before_state: verifying
    - after_state: draft
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-19T01:52:50Z · gate-closed · accept → accept · Scope approval of fix-347, rebased onto main (native approval) ^t-e489c3
  - native_change: fix-347
  - revision: 83f87dd373bbdb02efb7554bb44b8d8a96cdb92f
  - relayed_by:
    - decided_by: 0xLeif
    - words: Yes, for today
    - where: Leif's standing rule of 2026-11-18, relayed by his planning session (hc t4c): when an approved PR's delta goes stale only because main changed the same requirement, and the rebase puts the same added sentences on the newer text with nothing else changed, fable-t4b may record his relayed re-approval and report the PR and requirement. Here: PR #360, REQ-cli-012 and REQ-history-012
    - recorded_by: fable-t4b
    - native: specsync change approve fix-347
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
