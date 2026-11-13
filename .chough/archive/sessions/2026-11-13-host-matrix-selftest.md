---
kind: session
id: 2026-11-13-host-matrix-selftest
aliases: [host-matrix-selftest]
objective: "Host-matrix harness: a scripted Chough cycle with a stand-in host and its self-test in hosted CI on macOS and Linux (#218, first PR)"
intent_scope: LIFE
modules: [cli]
stage: done
stage_status: archived
iteration: 1
native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
criteria: [LIFE-21, LIFE-22, LIFE-24, LIFE-27, LIFE-28, LIFE-29, LIFE-30, FLOW-10, FLOW-16, FLOW-21]
requirements: [REQ-cli-066]
cases: [TC-cli-host-matrix-selftest]
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
closed: 2026-11-13
outcome: done
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
- Leif decided "Implementation review of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in": "Approve (Recommended)" (coordinator chat (Claude Code, hc t370), 2026-11-13T14:03Z, answering: 'Chough 1.0, harness PR 1 (#254): built. A fake agent runs the whole Chough cycle through it on Mac and Linux shells, and the harness catches every planted mistake: skipped gate, self-approval, extra file, wrong resume and a hung agent. The reviewer passed it. Approve the implementation?' (option text: 't38q ships it. Then CI runs on Mac and Linux, then merge it.'); asked at head 079a8388; main (#242) was merged afterwards at b5e984bd, changing only the spec change-log row (90) and main's own files, re-verified at 30d6405e and re-checked by t384; relayed by claude-code-t370); recorded by claude-code-t38q; a relayed claim, not authenticated; ran specsync change review host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in as 0xLeif with verdict pass

## What was verified

- 7 earlier runs of the cases below are kept in this story's receipts, not listed here.
- fledge lanes run verify (Fledge 1.8.0, the CI pin) at 6b7376cf passed toolchain, fmt, lint, test (48 targets), intent and spec, then stopped at audit because verification predated the last edits; after chough change check --commit, audit, cases, knowledge, leaks, exercise and build passed at fdf6561e. Between the two heads only Chough records and SpecSync lifecycle bookkeeping changed. ^2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-at-6b7376cf
- Full suite at 2bce65a2 (clean tree) under the heavy-tests lock with Rust 1.96.0: cargo +1.96.0 test --locked --no-fail-fast passed 735 tests in 48 targets, 0 failed, 0 ignored; cargo +1.96.0 clippy --all-targets --locked -- -D warnings and cargo +1.96.0 fmt --check exited 0. ^2026-11-13-full-suite-at-2bce65a2-clean-tree-under-the-heavy-tests-lock
- Reproduced t384's finding: run.sh's with_deadline at 2bce65a2, run by /bin/dash with stdin not a terminal, let 'with_deadline 2 sleep 8' run the full 8 s and return 0; bash ends it at 2 s. ^2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2
- fledge lanes run verify (Fledge 1.8.0, the CI pin) in one pass at bf753857 with a clean tree under the heavy-tests lock: toolchain, fmt, lint, test (735 passed in 48 targets, 0 failed), intent, spec, audit, cases, knowledge, leaks, exercise and build all passed. ^2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-in-one-pass-at
- Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0). — 687b74cf82fc ^2026-11-13-tc-cli-host-matrix-selftest-33ae9c
- After merging main (#242) at 30d6405e, clean tree: fledge lanes run verify (Fledge 1.8.0) passed all 12 steps, 737 tests in 48 targets, 0 failed; TC-cli-host-matrix-selftest passed with sh as dash (2b475a) and as bash (33ae9c). ^2026-11-13-after-merging-main-242-at-30d6405e-clean-tree-fledge-lanes-run

## Bugs

- [The host-matrix turn deadline does not end a hung turn under dash](../../bugs/2026-11-13-the-host-matrix-turn-deadline-does-not-end-a-hung-turn-under.md): verified

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
- 2026-11-13T14:19:17Z · gate-closed · accept → accept · Implementation review of host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in ^t-147acb
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - revision: 30530fa36cc040eaa193f90e362bb1200201d154
  - relayed_by:
    - decided_by: Leif
    - words: Approve (Recommended)
    - where: coordinator chat (Claude Code, hc t370), 2026-11-13T14:03Z, answering: 'Chough 1.0, harness PR 1 (#254): built. A fake agent runs the whole Chough cycle through it on Mac and Linux shells, and the harness catches every planted mistake: skipped gate, self-approval, extra file, wrong resume and a hung agent. The reviewer passed it. Approve the implementation?' (option text: 't38q ships it. Then CI runs on Mac and Linux, then merge it.'); asked at head 079a8388; main (#242) was merged afterwards at b5e984bd, changing only the spec change-log row (90) and main's own files, re-verified at 30d6405e and re-checked by t384; relayed by claude-code-t370
    - recorded_by: claude-code-t38q
    - native: specsync change review host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
    - actor: 0xLeif
  - by: claude-code (claude-opus-5-5, hc t38q)
- 2026-11-13T14:19:19Z · advance · accept → done · native:archived ^t-071ffe
  - native_change: host-matrix-harness-one-scripted-chough-cycle-per-agent-host-judged-from-the-files-with-a-stand-in-host-self-test-in
  - gate: native:archived
  - revision: 30530fa36cc040eaa193f90e362bb1200201d154
  - by: claude-code (claude-opus-5-5, hc t38q)

Closed 2026-11-13: done.

## Receipts

```yaml
receipts:
  - id: 2026-11-13-tc-cli-host-matrix-selftest
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: failed (exit 1)."
    result: failed
    command: sh tools/host-matrix/selftest.sh
    revision: adf4bfe481f22dad28c8e31809c09a5bc072fabc
    environment: local-macos-arm64
    content: e5ab5fab705b055ae79c26426c8d5a5dce6b0d9089fc434b0d090fcb78b3a78d
    fingerprint: sha256:77c89ae68051974383f1ee0ec830421de41740f0ba56027cdd7caffcc3abaa9b
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T03:55:24Z
    duration_seconds: 0
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-5e18df
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
    result: passed
    command: sh tools/host-matrix/selftest.sh
    revision: f2184c94d1988ff3ebe286cf586253250ea05362
    environment: local-macos-arm64
    content: 45f54f7cc5e75b9c9fefff749e94de7eb28e786471d42f738487b79935dc472b
    fingerprint: sha256:a2afb191c3d6660834faac9d5436eac8b86890382bb6436940a8bbe46aaa91c8
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T04:14:42Z
    duration_seconds: 118
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-201a83
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
    result: passed
    command: sh tools/host-matrix/selftest.sh
    revision: 6b7376cf61b3b7431180580c989661393f26ff44
    environment: local-macos-arm64
    content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
    fingerprint: sha256:701726f03a054818714773b57dee3b9e73f79a6fdb0f8436aa1f42e803a3ec72
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T04:31:27Z
    duration_seconds: 149
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-at-6b7376cf
    claim: fledge lanes run verify (Fledge 1.8.0, the CI pin) at 6b7376cf passed toolchain, fmt, lint, test (48 targets), intent and spec, then stopped at audit because verification predated the last edits; after chough change check --commit, audit, cases, knowledge, leaks, exercise and build passed at fdf6561e. Between the two heads only Chough records and SpecSync lifecycle bookkeeping changed.
    result: passed
    revision: fdf6561ee0d1ca48aa806844b60aa3499b30ffe7
    environment: local-macos-arm64
    content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
    observed: 2026-11-13T06:26:16Z
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-full-suite-at-2bce65a2-clean-tree-under-the-heavy-tests-lock
    claim: "Full suite at 2bce65a2 (clean tree) under the heavy-tests lock with Rust 1.96.0: cargo +1.96.0 test --locked --no-fail-fast passed 735 tests in 48 targets, 0 failed, 0 ignored; cargo +1.96.0 clippy --all-targets --locked -- -D warnings and cargo +1.96.0 fmt --check exited 0."
    result: passed
    revision: 2bce65a296608cb0206b3a61761f400e2b4d50cb
    environment: local-macos-arm64
    content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
    observed: 2026-11-13T06:53:09Z
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-reproduced-t384-s-finding-run-sh-s-with-deadline-at-2bce65a2
    claim: "Reproduced t384's finding: run.sh's with_deadline at 2bce65a2, run by /bin/dash with stdin not a terminal, let 'with_deadline 2 sleep 8' run the full 8 s and return 0; bash ends it at 2 s."
    result: failed
    revision: 0886a48aaebf05878bdb15a7be1b6ae73e88abf9
    environment: local-macos-arm64
    content: 8327d299d9048bc9377db6a93dc4cfccc37b3fd42e35d3e1f690ea5fcfaa4a79
    observed: 2026-11-13T06:57:48Z
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-282f87
    claim: "Red at 6457d8a8 under dash: HOST_MATRIX_FAULT=hang HOST_TURN_TIMEOUT=3 sh tools/host-matrix/run.sh stand-in, with sh as /bin/dash and stdin not a terminal, judged the run PASS after 1540 s: no turn was ended at its 3 s limit, so each stand-in turn's 300 s hang ran out."
    result: failed
    revision: 6457d8a880a013d1acb925136b7a9caca34f42fd
    environment: local-macos-arm64
    content: 724b40ff4a33d8eed5fd3f0c205c5a2ed40e9ae9e25c7f5ac18a6e08745f0326
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T07:24:36Z
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-349558
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
    result: passed
    command: sh tools/host-matrix/selftest.sh
    revision: 91b227be06a40a5fea5950bcf92c8a812820491b
    environment: local-macos-arm64
    content: 37601ca808fa35558fbf1b9a6c5612b97deaf5d05c594db7a389cd36732b8592
    fingerprint: sha256:635ea36defd727c787e21fc1d9673fd714c36bea1576cf4127d44e56c3d45b27
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T07:31:25Z
    duration_seconds: 92
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-b12eb5
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
    result: passed
    command: sh tools/host-matrix/selftest.sh
    revision: 91b227be06a40a5fea5950bcf92c8a812820491b
    environment: local-macos-arm64
    content: 37601ca808fa35558fbf1b9a6c5612b97deaf5d05c594db7a389cd36732b8592
    fingerprint: sha256:d54164ba648cb1400beab29792a5b03d3677b5c7d2d6216b66cc5926e40dd3fb
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T07:32:52Z
    duration_seconds: 84
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-fledge-lanes-run-verify-fledge-1-8-0-the-ci-pin-in-one-pass-at
    claim: "fledge lanes run verify (Fledge 1.8.0, the CI pin) in one pass at bf753857 with a clean tree under the heavy-tests lock: toolchain, fmt, lint, test (735 passed in 48 targets, 0 failed), intent, spec, audit, cases, knowledge, leaks, exercise and build all passed."
    result: passed
    revision: bf7538571b47d0fcf1ca2d16c90305bd9ec68e8c
    environment: local-macos-arm64
    content: 37601ca808fa35558fbf1b9a6c5612b97deaf5d05c594db7a389cd36732b8592
    observed: 2026-11-13T07:39:56Z
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-2b475a
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
    result: passed
    command: sh tools/host-matrix/selftest.sh
    revision: 30d6405e58c204b7643531412d31856036dd1895
    environment: local-macos-arm64
    content: 2fa1b777102b16a83cd3d8c9e54b14a723c990f4d6e6d5c44077dce37c2dc37f
    fingerprint: sha256:635ea36defd727c787e21fc1d9673fd714c36bea1576cf4127d44e56c3d45b27
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T14:10:03Z
    duration_seconds: 86
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-tc-cli-host-matrix-selftest-33ae9c
    claim: "Ran sh tools/host-matrix/selftest.sh for TC-cli-host-matrix-selftest: passed (exit 0)."
    result: passed
    command: sh tools/host-matrix/selftest.sh
    revision: 30d6405e58c204b7643531412d31856036dd1895
    environment: local-macos-arm64
    content: 2fa1b777102b16a83cd3d8c9e54b14a723c990f4d6e6d5c44077dce37c2dc37f
    fingerprint: sha256:687b74cf82fce2bf5cda6811dd367aedf3142d82ec229919615a2c39086ef2d6
    case: TC-cli-host-matrix-selftest
    case_digest: c23730a8357c60b400b77c0e416c70fe2356db7251e1c21a9f35dc3498dacba2
    observed: 2026-11-13T14:11:29Z
    duration_seconds: 83
    actor: claude-code (claude-opus-5-5, hc t38q)
  - id: 2026-11-13-after-merging-main-242-at-30d6405e-clean-tree-fledge-lanes-run
    claim: "After merging main (#242) at 30d6405e, clean tree: fledge lanes run verify (Fledge 1.8.0) passed all 12 steps, 737 tests in 48 targets, 0 failed; TC-cli-host-matrix-selftest passed with sh as dash (2b475a) and as bash (33ae9c)."
    result: passed
    revision: 30d6405e58c204b7643531412d31856036dd1895
    environment: local-macos-arm64
    content: 2fa1b777102b16a83cd3d8c9e54b14a723c990f4d6e6d5c44077dce37c2dc37f
    observed: 2026-11-13T14:18:17Z
    actor: claude-code (claude-opus-5-5, hc t38q)
```
