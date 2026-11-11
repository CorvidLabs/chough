# Roadmap

Durable intent is in [HI](../hi/). This roadmap records delivered work and proposed
sequencing; it is not human acceptance or proof of delivery. The path to 1.0 from
the 2026-11-10 MVP interview is in [roadmap-1.0.md](roadmap-1.0.md), with
[its test plan](test-plan-1.0.md).

| Stage | Outcome | State |
|---|---|---|
| First foundation | Skill, focused context, native status, bound observations | Delivered at d38aa19 (bootstrap, no native lifecycle); [hosted CI run](https://github.com/CorvidLabs/chough/actions/runs/41218954725) at that revision only |
| Resumable stage workflow | Stages and gates, checkpoint schema 2, focused cases, local bugs, optional validation, release routing and librarian, per-host skills | Delivered: human implementation review, native archive and merge in PR #2 (4008346, 2026-10-31); BUG-0004 and BUG-0005 fixed in PR #4 (2d782c4) and recorded in PR #6 |
| Native archive CI gate | CI fails while any native change is active or an archive fails to authenticate from history (no check is required to merge yet; see [ci.md](ci.md)) | Delivered in PR #3 (c408390, 2026-10-31) |
| Fail-closed stage gates | Refuse moves when native approval validity, archive integrity or the SDD policy does not hold | Delivered in PR #7 (9cb1c35, 2026-10-31) |
| Bounded external commands | Time and output limits for case bindings and native reads; validation honors a stop after waiting | Delivered: archived natively and merged in PR #8 (090460d, 2026-10-31) |
| First release candidate | Reference-aware evidence retention, current guidance, an installable package without Chough's own records | Delivered: archived natively and merged in PR #9 (2756dc2, 2026-10-31); no tag or release was made |
| Evidence freshness | Observations stay current across commits of Chough records and SpecSync bookkeeping; bug verification needs a pass after the failure | Delivered in PR #17 (c26bc26, 2026-11-01; D-018); fixes BUG-0014, BUG-0015 and BUG-0016 |
| Timing report | `chough timing`: how long stages, tests, bug fixes and human gates took (LIFE-10, REQ-reporting-001, was REQ-cli-020) | Delivered in PR #15 (3ddbcca, 2026-11-01) |
| Reporting local work | Share one root-bound case index per report and expose opt-in local phase/load attribution without caching native authorization | Delivered in merged PR #46 (134ee19, 2026-11-03), archived natively; issue #32 stays open for next/status latency and upstream native authentication latency |
| Archive-first CI | The `Archive` job runs before every other CI job and fails while a native change is open | Delivered in PR #18 (28db18c, 2026-11-01) |
| Bundle hygiene and actionable errors | Bundled templates pass docs gates, case bindings get their environment, colour-safe output matching, requirement and bug errors say how to fix them | Delivered in PR #16 (8b30943, 2026-11-01); fixes BUG-0018, BUG-0019 and the error half of BUG-0017 |
| Focused next and resume guards | A bounded focused `next` view, an approved-delta guard before build, and new-cycle guidance from done | Delivered in PR #23 (99a61ec, 2026-11-01) |
| Kimi new-cycle integration | Align the Kimi records and archived done guidance with main's REQ-cli-021 new-cycle behavior | Delivered in merged PR #24. BUG-0025 is verified by the later linked TC-cli-0105 pass; the original Kimi archived-resume record stays BUG-0024, and no fresh post-fix Kimi run is claimed |
| Fresh-agent handoff | `chough next` shows recent decisions, the last failure and a lesson pointer, names a new cycle after archival, and the skill asks an agent to state the handoff before editing | Implementation delivered and accepted in PRs #35 and #36. The graded cold-start proof for #35 failed; #36's cold-start proof was not run by 0xLeif's decision. No passing cold-start result is claimed |
| Parallel record identities | Curated slug case IDs and random observation, validation and snapshot identities preserve legacy records | Delivered in merged PR #33; same-second two-root and real Git fixture acceptance tests added in merged PR #48 (747a12c, 2026-11-03), archived natively. Issue #30 stays open for its remaining acceptance criteria |
| Bounded timing native reads | Deduplicate timing queries, skip excluded native history and bound total native wait with explicit unknowns | Delivered in merged PR #34, the first slice of issue #32; next/status latency and broader measurement remain open |
| Transition history and retention | Preserve independent branch events and resume state with bounded readers and protected-proof retention | Delivered by merged PR #37 at a788869 for issue #31; no live adoption or cleanup was performed |
| Chough drives native tools | The current agent handles native commands and applicable existing human authorization while preserving native checks and separate acceptance | Delivered in merged PR #44 (16fe55d), archived natively as `let-chough-drive-native-tools-and-reuse-existing-human-authorization`; issue #42 is closed. No fresh-agent cold-start result is claimed |
| Human gates and failed cases | Refuse stage advance and skip while a human gate is open, and refuse leaving test with an unlinked failed case | Delivered in merged PRs #45 (9d02fbf, #38) and #43 (3d00e44, #39) |
| New-project setup | Document the setup path and `specsync change adopt` when SDD is off | Delivered in merged PR #47 (d839fef) |
| CLI cleanups and checkpoint capacity | Hyphen values in `bug add`, ISO times, help and docs drift, exercise temp dir (#49); checkpoint bound 131,072 bytes and 400 observation pointers with atomic refusals (LIFE-11) | Delivered in merged PRs #50 (5568e5e) and #51 (8300b58), 2026-11-03, archived natively |
| Older workflow approval matching | The approved-delta guard must use the approval SpecSync treats as effective for changes approved by older workflow versions | Open as BUG-0020; recorded in PR #25 (47242e9, 2026-11-01) |
| 0.1.0 release | First published version | Published 2026-11-01: tag `v0.1.0` at c03d419 (release prep PR #22, change log PR #27), crates.io `chough` 0.1.0 and the [GitHub release](https://github.com/CorvidLabs/chough/releases/tag/v0.1.0). A clean `cargo install chough --version 0.1.0 --locked` into an empty root with cargo 1.95.0 on macOS arm64 exited 0 and `chough --version` printed `chough 0.1.0` |
| 0.4.0 release | Parallel work without merge conflicts, old proof moved into the knowledge vault, SpecSync 6.2.0 | Published 2026-11-05: tag `v0.4.0` at 69950c7 (release prep PR #116), crates.io `chough` 0.4.0 and the [GitHub release](https://github.com/CorvidLabs/chough/releases/tag/v0.4.0). A clean `cargo install chough --version 0.4.0 --locked` into an empty root with cargo 1.95.0 on macOS arm64 exited 0 and `chough --version` printed `chough 0.4.0` |
| 0.5.0 release | Session lifecycle and gate helpers, current-agent SpecSync steps, rebuildable Linux verification | Published 2026-11-06: tag `v0.5.0` at 455cb643 (release prep PR #185), crates.io `chough` 0.5.0 and the [GitHub release](https://github.com/CorvidLabs/chough/releases/tag/v0.5.0). A clean `cargo install chough --version 0.5.0 --locked` into an empty root with cargo 1.99.0 on macOS arm64 exited 0 and `chough --version` printed `chough 0.5.0` |
| First real project loop | Use the workflow on a real CorvidLabs feature and bug | Used once in real work, on a private CorvidLabs project; it found BUG-0015 to BUG-0019, of which BUG-0015, 0016, 0018 and 0019 are fixed and verified and BUG-0017's error half is fixed and its documented-order half is addressed by merged PR #41 (8fb0ea7, #11). More runs are needed |
| More hosts | Exercise Cursor, Gemini and Grok discovery and cold starts. One Kimi Code CLI 0.41.0 run on macOS arm64 passed `.agents` discovery but could not safely cold-resume because of BUG-0024's stale archived-resume action and BUG-0021's guidance gap. Google Antigravity has a separate unverified narrative report in closed PR #20 | Kimi run observed 2026-11-01, with no fresh post-fix rerun; Cursor, Gemini and Grok remain proposed |
| Observation reuse | Recognize unchanged inputs at a descendant commit | Narrower content-identity form delivered by D-018 in PR #17; the ancestry check and separate reuse label stay unapproved ([NEXT_CHANGE.md](NEXT_CHANGE.md)) |
| Native gate adapter | Reuse Trust where its policy fits | Proposed; needs degraded, failed, unavailable and baseline cases |
| Checkpoint adapter | APS, if it gains file input, compare-and-set and closed schemas (Windows builds are no longer needed, D-022) | Deferred (D-010) |
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
