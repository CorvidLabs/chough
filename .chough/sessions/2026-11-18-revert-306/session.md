---
kind: session
id: 2026-11-18-revert-306
aliases: [revert-306]
objective: "Revert #306's new-record writer for 0.6.1: it cannot create records on a file system without hard links (#313) and its temporary files are swept by a concurrent writer; #260 is redone in 0.6.2"
intent_scope: ""
modules: []
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: revert-306
pr: ""
criteria: [DRIVE-25]
requirements: [REQ-records-001]
cases: [TC-records-a-volume-without-hard-links-takes-new-records]
design: []
files: [src/records.rs, tests/no_hard_links.rs]
observations:
  - 2026-11-18-ran-tests-no-hard-links-rs-new-records-are-written-on-a-volume
  - 2026-11-18-tc-records-a-volume-without-hard-links-takes-new-records
  - 2026-11-18-ran-the-tester-s-parallel-repro-60-checkpoint-and-60-evidence
bugs: []
blockers: []
waiting_on_human:
  - gate: Implementation review of revert-306
    holds: accept
    drives: review
    opened: 2026-11-18T16:29:56Z
run_consents: []
next: Wait for Leif's implementation review of revert-306
---
# Revert #306's new-record writer for 0.6.1: it cannot create records on a file system without hard links (#313) and its temporary files are swept by a concurrent writer; #260 is redone in 0.6.2

## Goal

Revert #306's new-record writer for 0.6.1: it cannot create records on a file system without hard links (#313) and its temporary files are swept by a concurrent writer; #260 is redone in 0.6.2

## Where it stands

- Reverted #306's writer, test, case and changelog fragment; the exFAT test fails on main and passes with the revert

## Decisions and why

- 0xLeif decided "Scope approval of revert-306": "Revert #306" (Relayed by Leif's planning session (hc t4c) to fable-t4b on 2026-11-18, as his answer to: revert #306 for 0.6.1, or fix both regressions forward); recorded by fable-t4b; a relayed claim, not authenticated; ran specsync change approve revert-306 as 0xLeif

## What was verified

- [Ran tests/no_hard_links.rs::new_records_are_written_on_a_volume_without_hard_links by hand on a 64 MB exFAT image at about 08:50 on 2026-11-18, before it was marked ignored: with main's writer chough init failed with 'Operation not supported (os error 45)'; with the revert init, bug add and journal add passed (1 passed, 3.08s). A second run at 09:21 hung in hdiutil attach and was stopped, so it gave no result.](evidence/2026-11-18-ran-tests-no-hard-links-rs-new-records-are-written-on-a-volume.md)
- [Verdicts 1=pass. Ran the test by hand on a 64 MB exFAT image at about 08:50 on 2026-11-18, before it was marked ignored: with main's writer chough init failed with 'Operation not supported (os error 45)'; with the revert init, bug add and journal add passed (1 passed, 3.08s).](evidence/2026-11-18-tc-records-a-volume-without-hard-links-takes-new-records.md)
- [Ran the tester's parallel repro (60 checkpoint and 60 evidence add writers at once, chough-swarm-0611/runs/s3/repro/f2.sh) three times against this branch's debug build: all 120 writers exited 0 in each run, with no swept temporary file.](evidence/2026-11-18-ran-the-tester-s-parallel-repro-60-checkpoint-and-60-evidence.md)

## Open threads

## Timeline

- 2026-11-18T15:46:01Z · start · intent → intent · Revert #306's new-record writer for 0.6.1: it cannot create records on a file system without hard links (#313) and its temporary files are swept by a concurrent writer; #260 is redone in 0.6.2 ^t-f5e641
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:46:02Z · advance · intent → design ·  ^t-1d5ddc
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:46:02Z · skip · design → plan · Record writing change; nothing visual or interactive changes ^t-42369d
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:47:45Z · gate-opened · plan → plan · Scope approval of revert-306 ^t-e88889
  - native_change: revert-306
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - holds: plan
  - drives: approve
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:47:45Z · gate-closed · plan → plan · Scope approval of revert-306 ^t-47d225
  - native_change: revert-306
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - relayed_by:
    - decided_by: 0xLeif
    - words: Revert #306
    - where: Relayed by Leif's planning session (hc t4c) to fable-t4b on 2026-11-18, as his answer to: revert #306 for 0.6.1, or fix both regressions forward
    - recorded_by: fable-t4b
    - native: specsync change approve revert-306
    - actor: 0xLeif
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:47:46Z · advance · plan → build · native:approved ^t-7bb464
  - native_change: revert-306
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T15:47:47Z · advance · build → test · native:approved ^t-9d15b5
  - native_change: revert-306
  - gate: native:approved
  - revision: 66543c2bf509627ec2a165414acae0bc8ab59c4a
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T16:29:56Z · advance · test → accept · native:verifying ^t-e89a47
  - native_change: revert-306
  - gate: native:verifying
  - revision: 0cd19f98cde9dd17435a28021ebc7e35cc274c17
  - by: fable (claude-fable-5-2, hc t4b)
- 2026-11-18T16:29:56Z · gate-opened · accept → accept · Implementation review of revert-306 ^t-3af31f
  - native_change: revert-306
  - revision: 0cd19f98cde9dd17435a28021ebc7e35cc274c17
  - holds: accept
  - drives: review
  - by: fable (claude-fable-5-2, hc t4b)
