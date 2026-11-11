# Roadmap

Durable intent is in [HI](../hi/). This is Chough's one roadmap. It covers the path
to 1.0 from Leif's MVP / 1.0 interview of 2026-11-10, his same-day decisions on
issues #14, #165, #167, #201, #234, #235 and #236, and the 2026-11-10 design review
(captured as FLOW-21 to FLOW-24, DRIVE-24 to DRIVE-28, KNOW-30 to KNOW-35 and LIFE-17
to LIFE-26). [Delivered work](#delivered) is listed after it. The roadmap records
proposed sequencing and delivered history; it is not human acceptance or proof of
delivery.

## What 1.0 means

LIFE-20: Chough is solid across every CorvidLabs repo (spec-sync, site, raven,
raven-desktop, jackdaw, peck, podo-*, algochat and fledge): a frozen record
format, safe migrations, no data-loss bugs, and agents that resume reliably.
The date is milestone-driven: 1.0 ships when the bar below is met, not on a
calendar date.

Platforms: macOS and Linux are supported; Windows is best-effort (FLOW-21; see
the note on FLOW-17 below).

## The 1.0 bar

LIFE-21: all four are required.

1. **Dogfood every repo.** Each active CorvidLabs repo completes at least one
   full Chough cycle, intent to done, on the release candidate, with no
   data-loss or stale-record bugs.
2. **Zero known flakes.** CI is green with zero known flaky tests on macOS and
   Linux for two consecutive weeks. Today's known flakes: #187 and #132.
3. **Frozen format + migrations.** The `.chough` record format is frozen at
   1.0, with tested migrations from every 0.x release.
4. **Host-matrix e2e.** Before each release, a scripted end-to-end cycle
   passes on each supported host: Claude Code, Codex, Kimi, Grok and one local
   agent.

How each bar item is proven is in [the 1.0 test plan](test-plan-1.0.md).

## Weekly 0.x releases while 1.0 is pending

LIFE-25: a weekly 0.x release ships from main when CI is green; each release
notes what moved toward the 1.0 bar. An agent prepares the changelog, the
version bump and the green checks; Leif gives a one-line go; then it publishes
to crates.io. The human steps of [releasing](releasing.md) stay human steps.

## Milestones

Leif decided on 2026-11-10 that the human-first redesign (M1 to M3) ships as ONE
change, **Chough 0.6.0**, dogfooded on the chough repository itself before any
other repo updates. The redesign covers slug folders, Markdown records named by date
and slug, the links-never-copies vault and the cleanup. It is the native change
`chough-0-6-0-human-first-records-…` on draft PR #238 (branch
leif/human-first-records).

It folds in these issues:
- #214: slug folders, with #207's side-effect fix;
- #208: external artifacts;
- #210: append-only corrections;
- #233: agent, model and hc terminal on records;
- #237: a gate holds only its own stage;
- #236: a human's gate decision drives the native step as that human;
- #234: no leaks in committed records;
- #12: Chough's records stay out of SpecSync's verification;
- #165: no roll-up of archived sessions.

Only after that dogfood passes on chough do the other repos pick it up (M6 order). It
has not been merged or released yet.

Each milestone names the bar items it serves, what "done" means and how it is
tested. Milestones overlap in time; the dependencies are between the issues,
not the headings.

### M1: Per-session slug folders and the record-format migration

Serves bar 3 (frozen format + migrations) and bar 1 (no data-loss bugs). Ships in
0.6.0.

Scope: DRIVE-24 to DRIVE-28 as settled in the 2026-11-10 design review.
- **Session folders.** Every session gets its own date + slug folder,
  `.chough/sessions/<id>/`. Its `session.md` front matter is the checkpoint, and an
  append-only Timeline in the same file replaces `session.json` and
  `transitions.jsonl`.
- **Finishing.** A finished session becomes one story in
  `.chough/archive/sessions/`, mirroring SpecSync's `changes/` and `archive/`.
- **Human-first records.** Bugs, evidence, sessions and decisions are Markdown with
  YAML front matter, one file per record, and the file is the source of truth.
  Names are a date plus a slug, never sequential numbers, with old IDs kept as
  aliases. Case definitions stay small TOML, and `.chough/README.md` is the one
  reading guide.
- **Also included:** the corrections command (LIFE-18; #167, scoped in #210),
  session identity on records (LIFE-17; #201, scoped in #233), and a gate that holds
  only its own stage (#237).

Done means:
- two agents can run parallel sessions in one repository, commit both, and merge
  without a hand-resolved conflict in any file Chough writes (DRIVE-12 stays true);
- a fresh agent resumes either session from the committed files alone (FLOW-13);
- every finished session stays one archived story, never rolled up or trimmed
  (KNOW-34).

Tested by: the migration fixtures and parallel-session cases in
[the test plan](test-plan-1.0.md#bar-3-migration-fixtures), plus the existing
same-second and real-Git fixture acceptance runs.

### M2: Honest, small evidence

Serves bar 1 and bar 3. Ships in 0.6.0.

Scope:
- **LIFE-19.** Raw logs are never committed: they stay in `.chough/local/logs/` and
  in CI. Each evidence entry is a one-line claim plus a receipt: the exact command, a
  SHA-256 fingerprint and, for a failure, the key failing lines.
- **FLOW-22** (#14 item 3): external artifacts are fingerprinted in place.
- **FLOW-23** (#236): a human's Chough gate decision drives the SpecSync approval or
  review as that human, from a configured identity and never a built-in one.
- **FLOW-24:** an approval on permission given in advance is recorded in the agent's
  name, quoting the earlier permission.
- **LIFE-26** (#234): committed records never leak local paths, user names,
  credentials or personal information.
- **#12:** Chough's records and SpecSync verification no longer make each other
  stale.

#60 and #136 are already fixed on main (#204, #209).

Done means:
- no commit to `.chough/` carries a raw transcript;
- every recorded observation is a short receipt with a content digest and the exact
  command;
- recording commands return the observation ID they created;
- evidence-only commits never make native verification stale (DRIVE-4 stays true).

Tested by: focused cases per fix, plus a repository-size guard in CI that
fails when a committed evidence artifact exceeds the receipt bound.

### M3: Knowledge complete, links never copies

Serves bar 1. Ships in 0.6.0.

Scope: LIFE-23, all of KNOW, re-scoped by the 2026-11-10 interview round 2 (#235)
and the design review.
- **Links, not copies.** The vault holds only what has no other home: cross-project
  lessons and journal posts. It links to `hi/`, `specs/`, `.specsync/` and
  `.chough/`, including the bug files and session stories in `.chough/`, and never
  copies or holds them (KNOW-30, KNOW-31, KNOW-32).
- **Nothing trimmed.** Every finished session's story stays in the archive (KNOW-34).
- **Proof splits** (KNOW-35): logs go to CI; the record keeps the key failing lines,
  the exact command and a fingerprint; and a few small key screenshots sit beside the
  story under a size cap.
- **Lessons.** A module's own lessons stay with its spec, and only cross-project
  lessons live in `knowledge/lessons/` (KNOW-31).
- **Journal.** A post is short and links the archived session story, so nothing is
  written twice (KNOW-33). `knowledge/index.md` is the vault's reading guide
  (DRIVE-28).

The old `knowledge/intent/`, `archives/`, `modules/`, `bugs/`, `sessions/`,
`attachments/` and timeline pages are removed by `chough migrate`. Removing them
changes the contracted librarian behavior, so they go with the 0.6.0 native change
and its requirements, not with a docs change. This milestone includes #165 (whose
first wording is superseded by KNOW-30), #211 (dropped: copying session records into
the vault is duplication under #235), and the KNOW criteria still open when it
starts.

Done means:
- the vault contains no copy of a record that has a home;
- every vault note is either original content (a cross-project lesson or a journal
  post) or a link;
- the index and feed regenerate byte-identical from their sources (KNOW-4);
- a fresh agent is pointed at the index at start and resume (KNOW-1, FLOW-13);
- archived session stories stay at `.chough/archive/sessions/`.

Tested by: the vault regeneration and link-not-copy cases, exercised during
the per-repo dogfooding runs.

### M4: Zero known flakes

Serves bar 2.

Scope: DRIVE-19; the known flakes #187 and #132, and any flake found later.

Done means: the flake tracker lists zero open flakes, and CI has been green
on macOS and Linux for two consecutive weeks with no flake rerun or skip.

Tested by: the flake tracker and clean-window rules in
[the test plan](test-plan-1.0.md#bar-2-zero-known-flakes).

### M5: Host-matrix end-to-end harness

Serves bar 4.

Scope: LIFE-22 (Claude Code, Codex and Kimi Code fully supported with skills,
templates and resume tested on each; Grok and one local agent added), LIFE-24
(the optional `integrations/` folder with versioned example hc templates and
the multi-agent coordination guide; Rune stays a separate Fledge plugin), and
FLOW-21 platform coverage. Includes #76 (next guidance gaps for a fresh
agent) and #190 (gate list over-counting) where the harness exposes them.

Done means: one scripted end-to-end cycle exists and passes on Claude Code,
Codex, Kimi, Grok and one local agent, on macOS and Linux; the skills,
templates and cold resume are exercised per host, not only installed; the
harness runs before each release.

Tested by: the harness design in
[the test plan](test-plan-1.0.md#bar-4-host-matrix-e2e-harness).

### M6: Dogfood every repo

Serves bar 1.

Scope: LIFE-20. Each active CorvidLabs repo — spec-sync, site, raven,
raven-desktop, jackdaw, peck, podo-*, algochat and fledge — completes at
least one full Chough cycle on the release candidate. Order: the 0.6.0
redesign is dogfooded on the chough repository first; the other repos update
only after that dogfood passes (Leif, 2026-11-10).

Done means: the per-repo checklist in
[the test plan](test-plan-1.0.md#bar-1-dogfood-checklist-per-repo) is complete for every
repo, each with its cycle recorded and no data-loss or stale-record bug left
open.

Tested by: the dogfood checklist and its recorded evidence.

### M7: Freeze the format and ship 1.0

Serves bar 3 and closes the release.

Scope: freeze the `.chough` record format, confirm the migration chain from
every 0.x tag passes, run the full host-matrix and confirm the flake window,
then prepare the 1.0 release for Leif's approval (LIFE-25 process).

Done means: the format version is declared frozen in the docs and the helper
refuses unknown newer versions; every migration fixture from v0.1.0 to the
last 0.x passes; all four bar items are proven; Leif approves the release.

Tested by: [the test plan](test-plan-1.0.md) in full.

## Upgrading a 0.x project to 0.6.0

0.6.0 replaces the 0.x layout with the human-first tree:
- open sessions in `.chough/sessions/<id>/`, each with `session.md`, `evidence/` and
  `agent/`;
- finished sessions as `.chough/archive/sessions/<id>.md`;
- bugs as `.chough/bugs/<id>.md`;
- machine-only files in the Git-ignored `.chough/local/`.

The [README](../README.md#what-a-project-holds) shows the tree, and `.chough/README.md`
in each project explains it.

The upgrade is one explicit, previewed step.
- **Preview.** `chough migrate` reads the 0.x records, writes nothing, and prints the
  plan with its digest.
- **Apply.** `chough migrate --apply <digest> --consent "<who, their words, where>"`
  applies exactly that plan. It applies only with the human's consent, never while a
  native change is open, and only when Git holds every byte it removes.
- **What it writes.** Every file is written by create-new and read back, followed by
  a receipt in `.chough/archive/migrations/`. Old IDs stay as aliases, and an
  archived SpecSync change is never left weaker than before.

Fixtures from every released version, 0.1.0 to 0.5.0, migrate to the same tree. Until
a project migrates, 0.6.0 refuses every other command with `refused:unmigrated`.
Downgrade is not supported: Chough 0.5.x cannot read 0.6.0 records, so upgrade every
agent before a project migrates. The receipt and Git history are the rollback.

## Delivered

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
| Human-first records (0.6.0) | Markdown records in session folders, gates that hold their own stage, approval or review as the human, no leaks, links-never-copies vault, `chough migrate` | In progress as one native change on draft PR #238; not merged, released or dogfooded yet |
| First real project loop | Use the workflow on a real CorvidLabs feature and bug | Used once in real work, on a private CorvidLabs project; it found BUG-0015 to BUG-0019, of which BUG-0015, 0016, 0018 and 0019 are fixed and verified and BUG-0017's error half is fixed and its documented-order half is addressed by merged PR #41 (8fb0ea7, #11). More runs are needed |
| More hosts | Exercise Cursor, Gemini and Grok discovery and cold starts. One Kimi Code CLI 0.41.0 run on macOS arm64 passed `.agents` discovery but could not safely cold-resume because of BUG-0024's stale archived-resume action and BUG-0021's guidance gap. Google Antigravity has a separate unverified narrative report in closed PR #20 | Kimi run observed 2026-11-01, with no fresh post-fix rerun; Cursor, Gemini and Grok remain proposed |
| Observation reuse | Recognize unchanged inputs at a descendant commit | Narrower content-identity form delivered by D-018 in PR #17; the ancestry check and separate reuse label stay unapproved (the deferred proposal is in Git history and D-018) |
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

Resolved in earlier approved changes: the test-definition format (TOML
`chough-testing/1`, D-011) and APS (qualified and deferred, D-010). Still open:

- Project preferences for optional validation and monitoring, including whether a
  monitor may start an investigation; the default is local capture (D-013), and any
  remediation keeps its scope and gates.
- Project-specific release, trust and signer policies, when those optional paths are
  actually requested or configured.

## Notes and flagged conflicts

- FLOW-17 says "macOS and Linux; Windows through WSL"; the interview says
  "macOS + Linux. Windows is best-effort." FLOW-21 captures the interview
  wording; whether WSL stays the documented Windows path is Leif's call, and
  FLOW-17 is unchanged until then.
- KNOW-29 says older archived sessions "roll into the knowledge vault as a
  summary note with attachments", which reads as a move. The 2026-11-10
  interview round 2 (#235) and Leif's "Keep all" answer (KNOW-34) resolved
  this: the vault links and never moves or copies, and every finished
  session's story stays. KNOW-29 and the other cleanup-era criteria carry a
  supersession note and retire with the 0.6.0 change, which removes the
  behavior, requirements and cases together; KNOW-10's never-weaken and
  KNOW-11's preview-first safeguards carry forward to the one-time upgrade.
- The interview describes the layout before 0.6.0 as "a single committed
  session.json"; main has had per-session files since 0.4.0 (DRIVE-3,
  DRIVE-13). DRIVE-24 is the folder form of the same direction, not a
  reversal; the conflict-magnet evidence (peck #182) came from a project on
  the older layout.
