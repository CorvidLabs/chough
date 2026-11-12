---
kind: session
id: 2026-11-06-install-test-scripts-as-fresh-files-so-a-forked-child-holding
aliases:
  - install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
objective: Install test scripts as fresh files so a forked child cannot make them busy (#132)
intent_scope: DRIVE
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
pr: ""
criteria: [DRIVE-10]
requirements: []
cases: []
design: []
files: []
observations: []
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: Wait for Leif to approve, then install scripts as fresh files
closed: 2026-11-06
outcome: done
receipts: []
---
# Install test scripts as fresh files so a forked child cannot make them busy (#132)

## Goal

Install test scripts as fresh files so a forked child cannot make them busy (#132)

## Where it stands

## Decisions and why

- PR #145 (#132) accepted: native review (pass) recorded as 0xLeif at 74a6133d by the orc under Leif authorization to decide as him overnight (2026-11-05 23:3x MST).

## What was verified

## Open threads

## Timeline

- 2026-11-06T05:22:10Z · advance · intent → design · Plan approved natively (0xLeif, Approve, 2026-11-05 via the orc) ^t-e500b3
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - revision: d519ddad0b395509e400ec2a3c8c1aa3bab6fdb4
- 2026-11-06T05:22:11Z · advance · design → plan · Plan approved natively (0xLeif, Approve, 2026-11-05 via the orc) ^t-388d21
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - revision: d519ddad0b395509e400ec2a3c8c1aa3bab6fdb4
- 2026-11-06T05:22:11Z · advance · plan → build · Plan approved natively (0xLeif, Approve, 2026-11-05 via the orc) ^t-4bc1e9
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - gate: native:approved
  - revision: d519ddad0b395509e400ec2a3c8c1aa3bab6fdb4
- 2026-11-06T05:22:19Z · advance · build → test · Native change verified (specsync change check --commit) ^t-788a8a
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - gate: native:verifying
  - revision: c39cc5f01433d270781caa7bb5d262f16e15bb0c
- 2026-11-06T05:22:21Z · advance · test → accept · a_script_installed_while_an_old_write_handle_is_open_still_runs passed on Linux (local-linux-aarch64), failed first with ETXTBSY ^t-bfa67a
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - gate: native:verifying
  - revision: c39cc5f01433d270781caa7bb5d262f16e15bb0c
- 2026-11-06T05:22:21Z · gate-opened · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-25f2ba
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - revision: c39cc5f01433d270781caa7bb5d262f16e15bb0c
- 2026-11-06T06:36:03Z · gate-closed · accept → accept · Native implementation review of this change by Leif (FLOW-10) ^t-83a012
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - revision: 70d936739330ffc339b6ce9945edbf4cf9dd4080
- 2026-11-06T06:36:06Z · advance · accept → done · Native change archived; review recorded ^t-610581
  - native_change: install-test-scripts-as-fresh-files-so-a-forked-child-holding-an-old-write-handle-cannot-make-them-busy-132
  - gate: native:archived
  - revision: 70d936739330ffc339b6ce9945edbf4cf9dd4080


Closed 2026-11-06: done.
