# Roadmap

Durable intent is in [HI](../hi/). This roadmap records delivered work and proposed
sequencing; it is not human acceptance or proof of delivery.

| Stage | Outcome | State |
|---|---|---|
| First foundation | Skill, focused context, native status, bound observations | Delivered at d38aa19 (bootstrap, no native lifecycle); [hosted CI run](https://github.com/CorvidLabs/chough/actions/runs/41218954725) at that revision only |
| Resumable stage workflow | Stages and gates, checkpoint schema 2, focused cases, local bugs, optional validation, release routing and librarian, per-host skills | Delivered: human implementation review, native archive and merge in PR #2 (4008346, 2026-10-31); BUG-0004 and BUG-0005 fixed in PR #4 (2d782c4) and recorded in PR #6 |
| Native archive CI gate | A pull request cannot merge while any native change is active or an archive fails to authenticate from history | Delivered in PR #3 (c408390, 2026-10-31) |
| Fail-closed stage gates | Refuse moves when native approval validity, archive integrity or the SDD policy does not hold | Delivered in PR #7 (9cb1c35, 2026-10-31) |
| Bounded external commands | Time and output limits for case bindings and native reads; validation honors a stop after waiting | Delivered in PR #8 (090460d, merged 2026-10-31 UTC) |
| First release candidate | Reference-aware evidence retention, current guidance, an installable package without Chough's own records | Delivered in PR #9 (2756dc2, merged 2026-10-31 UTC); no release is published |
| Timing report | Read-only stage, test, bug-fix and human-gate timing, with elapsed wall-clock labels and history ranges that are estimates, not targets | Delivered in PR #15 (3ddbcca, merged 2026-11-01 UTC) |
| Bundle hygiene and actionable errors | Case-binding environment, colour-safe matching, dash-free bundle hygiene, and actionable requirement and bug errors | Delivered in PR #16 (8b30943, merged 2026-11-01 UTC) |
| Evidence freshness and bug verification | Freshness compares working content identity; the revision is recorded but not compared. Chough-only records, empty commits, committing edits already present when observed and SpecSync lifecycle bookkeeping keep observations current; any fingerprinted content change stales them. Records without content identity stay revision-bound, and bug verification requires a pass after the latest failure. No ancestry check or separate reuse label was added | Delivered in PR #17 (c26bc26, merged 2026-11-01 UTC); broader reuse ideas remain unapproved history ([NEXT_CHANGE.md](NEXT_CHANGE.md)) |
| First real project loop | Use the workflow on a real CorvidLabs feature and bug | Next; the shipping and checkout samples are rehearsals |
| More hosts | Exercise Cursor, Gemini and Grok discovery and cold starts. One Kimi Code CLI 0.41.0 run on macOS arm64 passed `.agents` discovery but exposed BUG-0020's stale archived-resume action and BUG-0021's guidance gap | Kimi run observed 2026-11-01, with no fresh post-fix rerun; Cursor, Gemini and Grok remain proposed |
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
