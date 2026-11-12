---
kind: session
id: 2026-11-06-support-macos-and-linux-with-windows-through-wsl-and-drop-the
aliases:
  - support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
objective: Chough supports macOS and Linux, with Windows through WSL (Ubuntu on WSL2); the native Windows CI job is gone and docs say so (FLOW-17)
intent_scope: FLOW
modules: [cli]
stage: done
stage_status: active
iteration: 1
callers: []
native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
pr: ""
criteria: [FLOW-17]
requirements: [REQ-cli-019]
cases: []
design: []
files: []
observations:
  - OBS-tc-cli-supported-platforms-cf84a4b69599d4e71ffec6b6
  - OBS-tc-cli-supported-platforms-67170b58d041096d0989d3dd
  - OBS-tc-cli-supported-platforms-740fae9392dd55ffb48cb418
  - OBS-tc-cli-supported-platforms-e99a5e875c28321368248744
bugs: []
blockers: []
waiting_on_human: []
run_consents: []
next: "After the relayed human review: specsync change review --reviewer as relayed, ship/finalize, stage advance to done, commit, push, observe CI, merge only on orc's relay"
closed: 2026-11-06
outcome: done
receipts:
  - id: 2026-11-06-tc-cli-supported-platforms
    aliases: OBS-tc-cli-supported-platforms-cf84a4b69599d4e71ffec6b6
    claim: "Ran cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl for TC-cli-supported-platforms: failed (exit 101, tests run 1)."
    result: failed
    command: cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl
    revision: ec830cfb001cc5f71f4f83809ca3294d685f42a4
    environment: local-macos-arm64
    content: 005ebe2bb4278d2a3ccc03ded6004c758d9c37b0fe383d2313adf4343fc59714
    fingerprint: sha256:c5e26a7f43722563170d68ade63749d9823f313bfb175e121bb5639b6ef3f604
    case: TC-cli-supported-platforms
    case_digest: 112ae5cf4fc7779115cab412f367a1e214a5e2e583290f4df406392f277341e5
    observed: 2026-11-06T03:49:59Z
    duration_seconds: 4
  - id: OBS-tc-cli-supported-platforms-67170b58d041096d0989d3dd
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: OBS-tc-cli-supported-platforms-740fae9392dd55ffb48cb418
    removed: .chough/archive/migrations/2026-11-12-from-0-5.md
  - id: 2026-11-06-tc-cli-supported-platforms-f288a9
    aliases: OBS-tc-cli-supported-platforms-e99a5e875c28321368248744
    claim: "Ran cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl for TC-cli-supported-platforms: passed (exit 0, tests run 1)."
    result: passed
    command: cargo test --locked --test package -- --exact supported_platforms_are_macos_and_linux_with_windows_through_wsl
    revision: 2c3834fe817fca1bef538c45c7f38f50feb6d5e0
    environment: local-macos-arm64
    content: f43df51fe83478491cfc84a5685c155a83e9ef81d74dbc9a566fe774589ea972
    fingerprint: sha256:753b3ebe00b44ad8f62ef19398179c3fa307cf84f590732d1b83befae3736afe
    case: TC-cli-supported-platforms
    case_digest: 112ae5cf4fc7779115cab412f367a1e214a5e2e583290f4df406392f277341e5
    observed: 2026-11-06T04:30:25Z
    duration_seconds: 0
---
# Chough supports macOS and Linux, with Windows through WSL (Ubuntu on WSL2); the native Windows CI job is gone and docs say so (FLOW-17)

## Goal

Chough supports macOS and Linux, with Windows through WSL (Ubuntu on WSL2); the native Windows CI job is gone and docs say so (FLOW-17)

## Where it stands

- Captured FLOW-17 with hi (renumbered from the unpushed FLOW-16 draft), FRD section, native change support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17 drafted with artifacts complete
- Failing-first: TC-cli-supported-platforms (FLOW-17, LIFE-9, REQ-cli-019) fails on the base CI and docs with the new test
- Implementation: Product matrix [ubuntu-latest, macos-latest]; README, docs/integrations.md Platforms, docs/ci.md, docs/cli.md, ROADMAP, D-022, CHANGELOG and specs/cli/testing.md say macOS and Linux, Windows through WSL 2 with Ubuntu; TC-cli-supported-platforms passes
- specsync change check passed (verifying); fledge lanes run verify passed on macOS arm64 with Fledge 1.8.0 after the main merge: 10 steps, 459 tests, audit and the exercise green; TC-cli-supported-platforms passes on the merged tree
- Merged main #135 (KNOW-29); rebuilt knowledge; TC-cli-supported-platforms passes on the merged tree
- Finalized and archived the native change (authenticated-history); merged main #134 and #142 (the #141 anchor fix)

## Decisions and why

- decided: "<redacted:host-name> + Linux; Windows via WSL" (Leif, orc tab, 2026-11-05 20:17 MST, relayed by orc in hc t18) because Chough should match SpecSync 6.0, which publishes no Windows binary, and other PRs wait on the native Windows job
- decided: "Approve, A on D1-D5" (Leif, relayed by orc in hc t18, 2026-11-05) because the gate recommended A on each: no requirement delta, keep cfg(windows) code untested, Ubuntu jobs cover WSL 2, close no issues, leave ci-windows case labels
- decided: take FLOW-17 and no REQ IDs (orc ID registry: ch-d7 owns FLOW-16, REQ-cli-050, REQ-history-010) because FLOW-17 is free on origin/main and every branch and worktree
- rejected: keeping the first draft numbered FLOW-16 in a commit because the SpecSync Archive walk refuses a change that a PR revision held and later omitted; the unpushed commit 72a26382 was reset locally instead
- decided: native scope approval recorded by orc as actor 0xLeif at 2301ee38 after Leif confirmed "Approve, A on D1-D5" in the orc tab (hc t18) because the definition gate is his; this agent recorded no approval
- decided: "Record and merge" for #139 (Leif, 2026-11-05 22:03 MST, relayed by orc in hc t18) because the implementation review passed; orc recorded the native review as 0xLeif at 1d28c61b and this agent ran ship/finalize (archived 6096f5a3)

## What was verified

- OBS-tc-cli-supported-platforms-cf84a4b69599d4e71ffec6b6
- OBS-tc-cli-supported-platforms-67170b58d041096d0989d3dd
- OBS-tc-cli-supported-platforms-740fae9392dd55ffb48cb418
- OBS-tc-cli-supported-platforms-e99a5e875c28321368248744

## Open threads

## Timeline

- 2026-11-06T03:49:38Z · gate-opened · intent → intent · Native scope approval of support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17: Leif's relayed 'Approve, A on D1-D5', to be recorded after orc's go ^t-275de2
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - revision: ec830cfb001cc5f71f4f83809ca3294d685f42a4
- 2026-11-06T03:58:53Z · gate-closed · intent → intent · Native scope approval of support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17: Leif's relayed 'Approve, A on D1-D5', to be recorded after orc's go ^t-8cab5a
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - revision: 2301ee38ed782b19507ff16e64fbfde4e5f46cb1
- 2026-11-06T03:58:53Z · advance · intent → design ·  ^t-4a91ae
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - revision: 2301ee38ed782b19507ff16e64fbfde4e5f46cb1
- 2026-11-06T03:58:54Z · skip · design → plan · No visuals or interactions change: CI matrix, documentation and a package test ^t-4605dd
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - revision: 2301ee38ed782b19507ff16e64fbfde4e5f46cb1
- 2026-11-06T03:58:57Z · advance · plan → build · native:approved ^t-ca535f
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - gate: native:approved
  - revision: 2301ee38ed782b19507ff16e64fbfde4e5f46cb1
- 2026-11-06T03:59:00Z · advance · build → test · native:approved ^t-01db21
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - gate: native:approved
  - revision: 2301ee38ed782b19507ff16e64fbfde4e5f46cb1
- 2026-11-06T04:25:47Z · advance · test → accept · native:verifying ^t-fd2d06
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - gate: native:verifying
  - revision: 1e076695cedf9aa332799234cb62411be892aff9
- 2026-11-06T04:26:28Z · gate-opened · accept → accept · Human implementation review of support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17, through orc t18 ^t-07582f
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - revision: f1852f824aa5b8da0fdd7721d5386af7145d0525
- 2026-11-06T05:23:08Z · gate-closed · accept → accept · Human implementation review of support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17, through orc t18 ^t-a794fa
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - revision: 6304d4444a9fec7bd73a98be90fd672fa085e1fb
- 2026-11-06T05:23:53Z · advance · accept → done · native:archived ^t-ef6f12
  - native_change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
  - gate: native:archived
  - revision: 6304d4444a9fec7bd73a98be90fd672fa085e1fb


Closed 2026-11-06: done.
