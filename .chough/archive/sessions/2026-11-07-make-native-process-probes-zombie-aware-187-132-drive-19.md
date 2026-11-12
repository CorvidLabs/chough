---
kind: session
id: 2026-11-07-make-native-process-probes-zombie-aware-187-132-drive-19
aliases: [make-native-process-probes-zombie-aware-187-132-drive-19]
objective: "Make native process test probes zombie-aware and record the #132 reproduction limit (DRIVE-19)"
intent_scope: DRIVE
modules: [cli, reporting]
stage: test
stage_status: active
iteration: 1
callers: []
native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
pr: ""
criteria: [DRIVE-19]
requirements: [REQ-cli-053, REQ-cli-059, REQ-reporting-002]
cases:
  - TC-cli-killed-unreaped-child-not-alive
  - TC-cli-timed-out-prime-kills-native-process
  - TC-cli-sigterm-stops-native-process
  - TC-reporting-timed-out-native-read-kills-descendants
design: []
files: []
observations:
  - OBS-tc-cli-killed-unreaped-child-not-alive-9af058ce944cce5d05f3feb7
  - OBS-tc-cli-killed-unreaped-child-not-alive-ab310389bde5c4d51a54535d
  - OBS-tc-cli-timed-out-prime-kills-native-process-f812f5bccc709170ad193979
  - OBS-tc-cli-sigterm-stops-native-process-71d69f74f25a3aade7f0057f
  - OBS-tc-reporting-timed-out-native-read-kills-descendants-591504a42688993cdb69e6ee
bugs: [BUG-kill-zero-reports-a-killed-unreaped-zombie-alive-187-fd5176ab33fee38ff313ffc8]
blockers: []
waiting_on_human: []
run_consents: []
next: Run loaded macOS and Linux repetitions and the full Linux verify lane.
closed: 2026-11-07
outcome: closed-without-done
receipts:
  - id: 2026-11-07-tc-cli-killed-unreaped-child-not-alive
    aliases: OBS-tc-cli-killed-unreaped-child-not-alive-9af058ce944cce5d05f3feb7
    claim: "Ran cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is for TC-cli-killed-unreaped-child-not-alive: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is
    revision: 599ff184c3a42d94ce6728eaa0f9bb17cd5b19ea
    environment: local-macos-arm64
    content: f8d14e8410f7a3406713757322acc71fee5f59b73698155987b4fff52cb58d41
    fingerprint: sha256:864b1bfa5a10720a6936231679c2da4a1d24d0c05702a684759003af81bd063b
    case: TC-cli-killed-unreaped-child-not-alive
    case_digest: 396594f0e7fc724973b186bd3d95a7b0bc1ec07c31e7fecc9b3546c1ee069836
    observed: 2026-11-07T03:37:57Z
    duration_seconds: 0
  - id: 2026-11-07-tc-cli-killed-unreaped-child-not-alive-ce9211
    aliases: OBS-tc-cli-killed-unreaped-child-not-alive-ab310389bde5c4d51a54535d
    claim: "Ran cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is for TC-cli-killed-unreaped-child-not-alive: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_killed_unreaped_child_is_not_alive_but_a_running_child_is
    revision: 76ba44981ed1b94b5a2101a52e86ebf1fa9ab3e4
    environment: local-macos-arm64
    content: 551ed13cd453415239abaf7abdcdac9af4eedc5cf060d4705e78736f69e0fb75
    fingerprint: sha256:6b5d51d14e19e807c4f2e3e4958be913857d537e946e2d707215a6a691181ead
    case: TC-cli-killed-unreaped-child-not-alive
    case_digest: 396594f0e7fc724973b186bd3d95a7b0bc1ec07c31e7fecc9b3546c1ee069836
    observed: 2026-11-07T03:40:56Z
    duration_seconds: 2
  - id: 2026-11-07-tc-cli-timed-out-prime-kills-native-process
    aliases: OBS-tc-cli-timed-out-prime-kills-native-process-f812f5bccc709170ad193979
    claim: "Ran cargo test --locked --test native -- --exact a_timed_out_prime_writes_nothing_kills_children_and_retries_cold for TC-cli-timed-out-prime-kills-native-process: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test native -- --exact a_timed_out_prime_writes_nothing_kills_children_and_retries_cold
    revision: 76ba44981ed1b94b5a2101a52e86ebf1fa9ab3e4
    environment: local-macos-arm64
    content: 551ed13cd453415239abaf7abdcdac9af4eedc5cf060d4705e78736f69e0fb75
    fingerprint: sha256:b2f2beb3de97f3fd90b549af909f008d69f20bb709837e8225b0394c2dae7276
    case: TC-cli-timed-out-prime-kills-native-process
    case_digest: cdf2a6b96ef924b73fb7e27816596e4fedaee6d78c9b1a67aaeb26f0d28d2d34
    observed: 2026-11-07T03:41:21Z
    duration_seconds: 11
  - id: 2026-11-07-tc-cli-sigterm-stops-native-process
    aliases: OBS-tc-cli-sigterm-stops-native-process-71d69f74f25a3aade7f0057f
    claim: "Ran cargo test --locked --test change -- --exact sigterm_to_change_check_stops_specsync for TC-cli-sigterm-stops-native-process: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test change -- --exact sigterm_to_change_check_stops_specsync
    revision: 76ba44981ed1b94b5a2101a52e86ebf1fa9ab3e4
    environment: local-macos-arm64
    content: 551ed13cd453415239abaf7abdcdac9af4eedc5cf060d4705e78736f69e0fb75
    fingerprint: sha256:4bbd0f95eee2f04bd8146ce4e689b41cdfa4745711edbe7875011ff40feb05d0
    case: TC-cli-sigterm-stops-native-process
    case_digest: 0caa4107fd0142b4c4a6356f209a46a82307efb6da8a1dea1e91dd46ac90c9d3
    observed: 2026-11-07T03:41:37Z
    duration_seconds: 3
  - id: 2026-11-07-tc-reporting-timed-out-native-read-kills-descendants
    aliases: OBS-tc-reporting-timed-out-native-read-kills-descendants-591504a42688993cdb69e6ee
    claim: "Ran cargo test --locked --test timing -- --exact a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown for TC-reporting-timed-out-native-read-kills-descendants: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test timing -- --exact a_timed_out_or_unconfigured_native_read_leaves_native_times_unknown
    revision: 76ba44981ed1b94b5a2101a52e86ebf1fa9ab3e4
    environment: local-macos-arm64
    content: 551ed13cd453415239abaf7abdcdac9af4eedc5cf060d4705e78736f69e0fb75
    fingerprint: sha256:fc0699ee50cf8851a66a1342c75c7bdcd3bcffe7940c49013a85621a36a73694
    case: TC-reporting-timed-out-native-read-kills-descendants
    case_digest: d875646a2350a849e5b0c3a34e8740fdea349031e7cab27878dceea1fe540f8c
    observed: 2026-11-07T03:42:05Z
    duration_seconds: 11
---
# Make native process test probes zombie-aware and record the #132 reproduction limit (DRIVE-19)

## Goal

Make native process test probes zombie-aware and record the #132 reproduction limit (DRIVE-19)

## Where it stands

- Failing tests: 599ff184c3a42d94ce6728eaa0f9bb17cd5b19ea
- Zombie-aware helper and all three migrated probes pass focused Chough cases on macOS arm64.

## Decisions and why

- Agent scope from Leif's handoff: use one test-only zombie-aware liveness helper for #187; reject product stop_tree changes because the descendant was already killed. Leave #132 open without code changes because it did not reproduce.

## What was verified

- OBS-tc-cli-killed-unreaped-child-not-alive-9af058ce944cce5d05f3feb7
- OBS-tc-cli-killed-unreaped-child-not-alive-ab310389bde5c4d51a54535d
- OBS-tc-cli-timed-out-prime-kills-native-process-f812f5bccc709170ad193979
- OBS-tc-cli-sigterm-stops-native-process-71d69f74f25a3aade7f0057f
- OBS-tc-reporting-timed-out-native-read-kills-descendants-591504a42688993cdb69e6ee

## Open threads

## Timeline

- 2026-11-07T03:35:18Z · direct-native-write · intent → intent ·  ^t-a3bd1c
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - revision: cc7c18ad0af953017b5edbead96ca12819480007
  - direct_write: {"change":"make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit","files":[{"path":"state.json","change":"changed"},{"path":"approvals.json","change":"changed"}],"before_state":"draft","after_state":"approved"}
- 2026-11-07T03:35:18Z · advance · intent → design ·  ^t-e719e1
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - revision: cc7c18ad0af953017b5edbead96ca12819480007
- 2026-11-07T03:35:19Z · skip · design → plan · Test-only process probe correction; no visuals or interactions change. ^t-d49cab
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - revision: cc7c18ad0af953017b5edbead96ca12819480007
- 2026-11-07T03:35:27Z · advance · plan → build · native:approved ^t-d13cae
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - gate: native:approved
  - revision: cc7c18ad0af953017b5edbead96ca12819480007
- 2026-11-07T03:37:33Z · advance · build → test · native:approved ^t-12ac89
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - gate: native:approved
  - revision: 599ff184c3a42d94ce6728eaa0f9bb17cd5b19ea
- 2026-11-07T03:38:27Z · back · test → build · The deterministic zombie case failed; fix the blocking probe defect. ^t-639218
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - gate: native:approved
  - revision: 599ff184c3a42d94ce6728eaa0f9bb17cd5b19ea
  - refresh: build: implementation against the current definition, test: rerun affected cases; earlier observations may be stale, checkpoint references: 4 case(s) and 1 observation(s) to re-check with `chough status`
- 2026-11-07T03:40:39Z · advance · build → test · native:approved ^t-b1ddeb
  - native_change: make-native-process-test-probes-zombie-aware-for-187-and-preserve-the-132-reproduction-limit
  - gate: native:approved
  - revision: 76ba44981ed1b94b5a2101a52e86ebf1fa9ab3e4


Closed 2026-11-07: closed-without-done.
