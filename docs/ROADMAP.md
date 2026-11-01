# Roadmap

Durable intent is in [HI](../hi/). This roadmap records delivered work and proposed
sequencing; it is not human acceptance or proof of delivery.

| Stage | Outcome | State |
|---|---|---|
| First foundation | Skill, focused context, native status, bound observations | Delivered at d38aa19 (bootstrap, no native lifecycle); [hosted CI run](https://github.com/CorvidLabs/chough/actions/runs/41218954725) at that revision only |
| Resumable stage workflow | Stages and gates, checkpoint schema 2, focused cases, local bugs, optional validation, release routing and librarian, per-host skills | Delivered: human implementation review, native archive and merge in PR #2 (4008346, 2026-10-31); BUG-0004 and BUG-0005 fixed in PR #4 (2d782c4) and recorded in PR #6 |
| Native archive CI gate | CI fails while any native change is active or an archive fails to authenticate from history (no check is required to merge yet; see [ci.md](ci.md)) | Delivered in PR #3 (c408390, 2026-10-31) |
| Fail-closed stage gates | Refuse moves when native approval validity, archive integrity or the SDD policy does not hold | Delivered in PR #7 (9cb1c35, 2026-10-31) |
| Bounded external commands | Time and output limits for case bindings and native reads; validation honors a stop after waiting | Delivered: archived natively and merged in PR #8 (090460d, 2026-10-31) |
| First release candidate | Reference-aware evidence retention, current guidance, an installable package without Chough's own records | Delivered: archived natively and merged in PR #9 (2756dc2, 2026-10-31); no tag or release was made |
| Evidence freshness | Observations stay current across commits of Chough records and SpecSync bookkeeping; bug verification needs a pass after the failure | Delivered in PR #17 (c26bc26, 2026-11-01; D-018); fixes BUG-0014, BUG-0015 and BUG-0016 |
| Timing report | `chough timing`: how long stages, tests, bug fixes and human gates took (LIFE-10, REQ-cli-020) | Delivered in PR #15 (3ddbcca, 2026-11-01) |
| Archive-first CI | The `Archive` job runs before every other CI job and fails while a native change is open | Delivered in PR #18 (28db18c, 2026-11-01) |
| Bundle hygiene and actionable errors | Bundled templates pass docs gates, case bindings get their environment, colour-safe output matching, requirement and bug errors say how to fix them | Delivered in PR #16 (8b30943, 2026-11-01); fixes BUG-0018, BUG-0019 and the error half of BUG-0017 |
| 0.1.0 release | First published version | Preparing; not published |
| First real project loop | Use the workflow on a real CorvidLabs feature and bug | Used once in real work, on a private CorvidLabs project; it found BUG-0015 to BUG-0019, of which BUG-0015, 0016, 0018 and 0019 are fixed and verified and BUG-0017 is half fixed (its error half; the documented-order half is open). More runs are needed |
| More hosts | Exercise Cursor, Gemini, Grok and Kimi discovery and cold starts | Proposed; only Claude Code and Codex were exercised |
| Observation reuse | Recognize unchanged inputs at a descendant commit | Narrower content-identity form delivered by D-018 in PR #17; the ancestry check and separate reuse label stay unapproved ([NEXT_CHANGE.md](NEXT_CHANGE.md)) |
| Native gate adapter | Reuse Trust where its policy fits | Proposed; needs degraded, failed, unavailable and baseline cases |
| Checkpoint adapter | APS, if it gains Windows builds, file input, compare-and-set and closed schemas | Deferred (D-010) |
| Test runner adapter | Magpie for real-command targets with explicit assertions | Deferred until installed and qualified per mode |
| Rune proof of concept | Optional cross-agent session bridge | Deferred; current-agent-first remains the model |
| Monitoring sources | Project-native health and log adapters as cases | Proposed per project; no general health service assumed |

Host-neutral, current-agent-first use is settled. Host discovery still needs real
host-specific exercises; sequential role passes must keep working without subagents.
Focused tests cover the affected criteria and impacted behavior, not every criterion
on every change. More commands should follow friction observed in real use.

## Open choices

Resolved in the approved change: the test-definition format (TOML
`chough-testing/1`, D-011) and APS (qualified and deferred, D-010). Still open:

- Project preferences for optional validation and monitoring, including whether a
  monitor may start an investigation; the default is local capture (D-013), and any
  remediation keeps its scope and gates.
- Project-specific release, trust and signer policies, when those optional paths are
  actually requested or configured.
