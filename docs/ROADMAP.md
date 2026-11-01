# Roadmap

Durable intent is in [HI](../hi/). This roadmap records delivered work and proposed
sequencing; it is not human acceptance or proof of delivery.

| Stage | Outcome | State |
|---|---|---|
| First foundation | Skill, focused context, native status, bound observations | Delivered at d38aa19 (bootstrap, no native lifecycle); [hosted CI run](https://github.com/CorvidLabs/chough/actions/runs/41218954725) at that revision only |
| Resumable stage workflow | Stages and gates, checkpoint schema 2, focused cases, local bugs, optional validation, release routing and librarian, per-host skills | Delivered: human implementation review, native archive and merge in PR #2 (4008346, 2026-10-31); BUG-0004 and BUG-0005 fixed in PR #4 (2d782c4) and recorded in PR #6 |
| Native archive CI gate | A pull request cannot merge while any native change is active or an archive fails to authenticate from history | Delivered in PR #3 (c408390, 2026-10-31) |
| Fail-closed stage gates | Refuse moves when native approval validity, archive integrity or the SDD policy does not hold | Delivered in PR #7 (9cb1c35, 2026-10-31) |
| Bounded external commands | Time and output limits for case bindings and native reads; validation honors a stop after waiting | Delivered: native archive and merge in PR #8 (090460d, 2026-10-31) |
| First release candidate | Reference-aware evidence retention, current guidance, an installable package without Chough's own records | Delivered: native archive and merge in PR #9 (2756dc2, 2026-10-31); no release is published |
| Stage and test timing report | Record duration for case runs, stage transitions, bug fixes and human decisions (LIFE-10, REQ-cli-020) | Delivered: native archive and merge in PR #15 (3ddbcca, 2026-10-31) |
| Evidence freshness and verification ordering | Keep observations fresh across commits and SpecSync bookkeeping; require pass strictly after failure to verify bugs | Delivered: native archive and merge in PR #17 (c26bc26, 2026-10-31) |
| Bundle hygiene and environment safety | Prohibit em or en dashes, track known earlier digests, validate binding env variables and match terminal output safely | Delivered: native archive and merge in PR #16 (8b30943, 2026-11-01) |
| First real project loop | Use the workflow on a real CorvidLabs feature and bug | Next; the shipping and checkout samples are rehearsals |
| More hosts | Exercise Cursor, Gemini, Grok and Kimi discovery and cold starts | Claude Code and Codex exercised; Gemini host cold start completed on macOS arm64 (Antigravity CLI, 2026-11-01, PR #20); Cursor, Grok and Kimi proposed |
| Observation reuse | Recognize unchanged inputs at a descendant commit | Unapproved and deferred, not rejected ([NEXT_CHANGE.md](NEXT_CHANGE.md)); needs its own human decision |
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
