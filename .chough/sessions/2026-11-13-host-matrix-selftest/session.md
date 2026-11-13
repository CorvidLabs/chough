---
kind: session
id: 2026-11-13-host-matrix-selftest
aliases: [host-matrix-selftest]
objective: "Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)"
intent_scope: LIFE
modules: [cli]
stage: accept
stage_status: active
iteration: 1
callers: []
native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
pr: ""
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, LIFE-28, LIFE-29, LIFE-30, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
cases: [TC-cli-host-matrix-selftest]
design: []
files: []
observations:
  - 2026-11-13-tc-cli-host-matrix-selftest
  - 2026-11-13-tc-cli-host-matrix-selftest-5e18df
  - 2026-11-13-tc-cli-host-matrix-selftest-201a83
  - 2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-at-6b7376cf
  - 2026-11-13-full-suite-at-2bce65a2-clean-tree-under-the-heavy-tests-lock
  - 2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2
  - 2026-11-13-tc-cli-host-matrix-selftest-282f87
  - 2026-11-13-tc-cli-host-matrix-selftest-349558
  - 2026-11-13-tc-cli-host-matrix-selftest-b12eb5
  - 2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-in-one-pass-at
  - 2026-11-13-tc-cli-host-matrix-selftest-2b475a
  - 2026-11-13-tc-cli-host-matrix-selftest-33ae9c
  - 2026-11-13-after-merging-main-242-at-30d6405e-clean-tree-fledge-lanes-run
bugs: [2026-11-13-the-host-matrix-turn-deadline-does-not-end-a-hung-turn-under]
blockers: []
waiting_on_human:
  - gate: Implementation review of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
    holds: accept
    drives: review
    opened: 2026-11-13T05:07:45Z
run_consents: []
next: Leif's implementation review through t370; then chough change ship with his relayed words, commit the story, doctor --fix, push the archive tip and watch hosted CI
---
# Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)

## Goal

Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)

## Where it stands

- Red: TC-cli-host-matrix-selftest fails before the harness exists (selftest.sh cannot find run.sh)
- Green: TC-cli-host-matrix-selftest passed on macOS arm64 with GIT_CONFIG_GLOBAL=/dev/null (8 harness runs, all as expected)
- Green at 6b7376cf (after merging main): TC-cli-host-matrix-selftest passed on macOS arm64; fledge verify passed toolchain, fmt, lint, test, intent and spec
- Verify lane passed in two passes (6b7376cf, fdf6561e); draft PR #254 opened and pushed
- Full suite at 2bce65a2: 735 passed in 48 targets (Rust 1.96.0), clippy and fmt clean
- Fixed the dash deadline bug (91b227be); self-test green under dash and bash
- Verify lane passed in one pass at bf753857 (735 tests); t384 passed the review at 96%
- Re-verified after merging main (#242): verify lane 737 tests, self-test green under dash and bash at 30d6405e; t384 re-check PASS

## Decisions and why

- decided: the harness lives in tools/host-matrix/ with adapters at integrations/<host>/adapter.sh beside #219's hc-template.md because t38r's #219 layout and the test plan's bar 4 both put per-host files there
- decided: the self-test runs as its own hosted CI job on Ubuntu and macOS, not as a verify-lane step, because the verify lane's step list is pinned by README wording and tests/docs.rs, and a separate job covers both platforms (FLOW-21)
- decided: each host turn runs with the driver's agent and hc environment cleared and CHOUGH_ACTOR set to the host because chough reads the actor from CLAUDECODE and HC_TERMINAL_ID, which named the harness's own terminal on the stand-in's records
- Leif decided "Approve the intent: the host-matrix harness and its local-agent leg as Leif chose them (LIFE-27 to LIFE-30, under LIFE-21 item 4, LIFE-22 and LIFE-24)": "Plain runs, Herdr optional (Recommended); Docker on the Mac (Recommended); Not sure: keep it generic (Recommended); Next weekly release (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:32Z, his four answers to the design questions for #218 and #220 (run mode, Linux legs, local agent, when a leg blocks a release), relayed by claude-code-t370, who said to quote them at this change's intent and scope gates); recorded by claude-code-t38q; a relayed claim, not authenticated
- Leif decided "Scope approval of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T03:53Z, answering: 'Chough 1.0 harness, PR 1 of 5 (#218): add the test harness that runs one Chough cycle per agent and judges it from the files it leaves. It comes with a fake, no-model agent that tests the harness itself on Mac and Linux CI; no real agents run yet. It lives in tools/host-matrix/, with per-agent adapters next to the #219 templates. Approve this scope?' (option text: 't38q builds it. Its self-test already catches skipped gates, self-approval, wrong resumes and hung turns.'); per t370 his yes also covers REQ-cli-066 as aligned with his four 03:32Z answers and claude-code-t38q's engineering choices (a) to (c) in this change's context.md; relayed by claude-code-t370); recorded by claude-code-t38q; a relayed claim, not authenticated; ran specsync change approve host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in as 0xLeif
- decided: no open session is read from chough next's null stage, or its failure before 0.6.1, because main's #248 made next exit 0 with no open session
- decided: each turn and its timer start as process groups through perl's setpgrp because dash, sh on Ubuntu, turns job control off without a terminal (t384's review); rejected: requiring bash because the harness and its callers are POSIX sh like the rest of tools/

## What was verified

- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: failed (exit 1).](evidence/2026-11-13-tc-cli-host-matrix-selftest.md)
- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).](evidence/2026-11-13-tc-cli-host-matrix-selftest-5e18df.md)
- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).](evidence/2026-11-13-tc-cli-host-matrix-selftest-201a83.md)
- [fledge lanes run verify (Fledge 1.8.0, the CI pin) at 6b7376cf passed toolchain, fmt, lint, test (48 targets), intent and spec, then stopped at audit because verification predated the last edits; after chough change check --commit, audit, cases, knowledge, leaks, exercise and build passed at fdf6561e. Between the two heads only Chough records and SpecSync lifecycle bookkeeping changed.](evidence/2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-at-6b7376cf.md)
- [Full suite at 2bce65a2 (clean tree) under the heavy-tests lock with Rust 1.96.0: cargo +1.96.0 test --locked --no-fail-fast passed 735 tests in 48 targets, 0 failed, 0 ignored; cargo +1.96.0 clippy --all-targets --locked -- -D warnings and cargo +1.96.0 fmt --check exited 0.](evidence/2026-11-13-full-suite-at-2bce65a2-clean-tree-under-the-heavy-tests-lock.md)
- [Reproduced t384's finding: run.sh's with_deadline at 2bce65a2, run by /bin/dash with stdin not a terminal, let 'with_deadline 2 sleep 8' run the full 8 s and return 0; bash ends it at 2 s.](evidence/2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2.md)
- [Red at 6457d8a8 under dash: HOST_MATRIX_FAULT=hang HOST_TURN_TIMEOUT=3 sh tools/host-matrix/run.sh stand-in, with sh as /bin/dash and stdin not a terminal, judged the run PASS after 1540 s: no turn was ended at its 3 s limit, so each stand-in turn's 300 s hang ran out.](evidence/2026-11-13-tc-cli-host-matrix-selftest-282f87.md)
- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).](evidence/2026-11-13-tc-cli-host-matrix-selftest-349558.md)
- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).](evidence/2026-11-13-tc-cli-host-matrix-selftest-b12eb5.md)
- [fledge lanes run verify (Fledge 1.8.0, the CI pin) in one pass at bf753857 with a clean tree under the heavy-tests lock: toolchain, fmt, lint, test (735 passed in 48 targets, 0 failed), intent, spec, audit, cases, knowledge, leaks, exercise and build all passed.](evidence/2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-in-one-pass-at.md)
- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).](evidence/2026-11-13-tc-cli-host-matrix-selftest-2b475a.md)
- [Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0).](evidence/2026-11-13-tc-cli-host-matrix-selftest-33ae9c.md)
- [After merging main (#242) at 30d6405e, clean tree: fledge lanes run verify (Fledge 1.8.0) passed all 12 steps, 737 tests in 48 targets, 0 failed; TC-cli-host-matrix-selftest passed with sh as dash (2b475a) and as bash (33ae9c).](evidence/2026-11-13-after-merging-main-242-at-30d6405e-clean-tree-fledge-lanes-run.md)

## Open threads

## Timeline

- 2026-11-13T03:48:59Z · start · intent → intent · Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR) ^t-65e2dd
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:49:37Z · gate-opened · intent → intent · Approve the intent: the host-matrix harness and its local-agent leg as Leif chose them (LIFE-27 to LIFE-30, under LIFE-21 item 4, LIFE-22 and LIFE-24) ^t-c853c0
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - holds: intent
  - drives: none
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:49:53Z · gate-closed · intent → intent · Approve the intent: the host-matrix harness and its local-agent leg as Leif chose them (LIFE-27 to LIFE-30, under LIFE-21 item 4, LIFE-22 and LIFE-24) ^t-c01cb8
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - relayed_by:
    - decided_by: Leif
    - words: Plain runs, Herdr optional (Recommended); Docker on the Mac (Recommended); Not sure: keep it generic (Recommended); Next weekly release (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:32Z, his four answers to the design questions for #218 and #220 (run mode, Linux legs, local agent, when a leg blocks a release), relayed by claude-code-t370, who said to quote them at this change's intent and scope gates
    - recorded_by: claude-code-t38q
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:50:00Z · advance · intent → design · Leif approved the intent through his four design choices (2026-11-13T03:32Z, relayed by t370) ^t-0a1499
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:50:00Z · skip · design → plan · Scripts, prompts and CI only; no visuals or interactions change ^t-9012bf
  - revision: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:52:59Z · gate-opened · plan → plan · Scope approval of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in ^t-c01633
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - revision: 598c4bc527007f9bebe53f7291e31130b2891561
  - holds: plan
  - drives: approve
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:54:08Z · gate-closed · plan → plan · Scope approval of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in ^t-82f323
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - revision: 7bcdbc8f34bbfcfdc50a34f3797faffa76ef4715
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T03:53Z, answering: 'Chough 1.0 harness, PR 1 of 5 (#218): add the test harness that runs one Chough cycle per agent and judges it from the files it leaves. It comes with a fake, no-model agent that tests the harness itself on Mac and Linux CI; no real agents run yet. It lives in tools/host-matrix/, with per-agent adapters next to the #219 templates. Approve this scope?' (option text: 't38q builds it. Its self-test already catches skipped gates, self-approval, wrong resumes and hung turns.'); per t370 his yes also covers REQ-cli-066 as aligned with his four 03:32Z answers and claude-code-t38q's engineering choices (a) to (c) in this change's context.md; relayed by claude-code-t370
    - recorded_by: claude-code-t38q
    - native: specsync change approve host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
    - actor: 0xLeif
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T03:54:21Z · advance · plan → build · native:approved ^t-81b234
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:approved
  - revision: 7bcdbc8f34bbfcfdc50a34f3797faffa76ef4715
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T05:07:44Z · advance · build → test · native:verifying ^t-8e48fb
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:verifying
  - revision: 6b7376cf61b3b7431180580c989661393f26ff44
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T05:07:45Z · advance · test → accept · native:verifying ^t-d3f596
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:verifying
  - revision: 6b7376cf61b3b7431180580c989661393f26ff44
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T05:07:45Z · gate-opened · accept → accept · Implementation review of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in ^t-eb4251
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - revision: 6b7376cf61b3b7431180580c989661393f26ff44
  - holds: accept
  - drives: review
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T06:57:38Z · back · accept → build · Independent review (t384, PR #254): the turn deadline needs job control, which dash (Ubuntu's sh) turns off without a terminal, so a hung turn is never ended on Linux ^t-9a5953
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:verifying
  - revision: 0886a48aaebf05878bdb15a7be1b6ae73e88abf9
  - refresh: build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, accept: native verification and the human implementation review, checkpoint references: 1 case(s) and 5 observation(s) to re-check with `chough status`
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T07:33:12Z · advance · build → test · native:verifying ^t-9be443
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:verifying
  - revision: 91b227be06a40a5fea5950bcf92c8a812820491b
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T07:33:14Z · advance · test → accept · native:verifying ^t-ff5e35
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:verifying
  - revision: 91b227be06a40a5fea5950bcf92c8a812820491b
  - by: claude-code (claude-opus-5-5, hc t38q)
