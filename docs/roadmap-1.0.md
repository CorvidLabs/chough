# Roadmap to 1.0

Source: Leif's MVP / 1.0 interview of 2026-11-10 and his same-day decisions on
issues #14, #165, #167, #201, #234, #235 and #236, captured as FLOW-21 to
FLOW-23, DRIVE-24 to DRIVE-27, KNOW-30 to KNOW-32 and LIFE-17 to LIFE-26. This
roadmap records proposed sequencing; it is not human acceptance or proof of
delivery. Durable intent stays in [HI](../hi/).

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

Each milestone names the bar items it serves, what "done" means and how it is
tested. Milestones overlap in time; the dependencies are between the issues,
not the headings.

### M1: Per-session slug folders and the record-format migration

Serves bar 3 (frozen format + migrations) and bar 1 (no data-loss bugs).

Scope: DRIVE-24. Every session gets its own unique slug folder; active and
archived sessions are kept separately, mirroring SpecSync's `changes/` and
`archive/`; sessions stay committed because handoff depends on them; parallel
sessions never rewrite one shared file. Records become human-first
(DRIVE-25): bugs, evidence, sessions and decisions are Markdown with YAML
frontmatter, one file per record, the file itself the source of truth.
Record names are a date plus a slug, never sequential numbers, with old
sequential IDs kept as aliases (DRIVE-27). Includes the corrections command
(LIFE-18, #167), session identity recording (LIFE-17, #201) and the open
session bugs (#181, #113).

Done means: two agents can run parallel sessions in one repository, commit
both, and merge without a hand-resolved conflict in any file Chough writes
(DRIVE-12 stays true); a fresh agent resumes either session from the committed
files alone (FLOW-13); sessions archived under the new layout roll up under
KNOW-29 and KNOW-30.

Tested by: the migration fixtures and parallel-session cases in
[the test plan](test-plan-1.0.md#bar-3-migration-fixtures), plus the existing
same-second and real-Git fixture acceptance runs.

### M2: Honest, small evidence

Serves bar 1 and bar 3.

Scope: LIFE-19 (digest-committed receipts; raw logs and transcripts go to CI
artifacts or are regenerated from seeds, matching LIFE-8), with each evidence
entry a one-line claim plus a receipt in the DRIVE-26 form; the remaining
case-binding fixes (FLOW-22, #14 item 3: external artifacts by recorded
fingerprint); FLOW-23 (#236): a human's Chough gate approval drives the
SpecSync approval as that human, never a hard-coded identity; LIFE-26 (#234):
committed records never leak local paths, usernames, credentials or personal
information; #60 (observation IDs from every recording command), #12 (Chough
records and SpecSync verification making each other stale) and #136
(knowledge index rebuilds marking observations stale).

Done means: no commit to `.chough/` carries a raw transcript; every recorded
observation is a short receipt with a content digest and the exact command;
recording commands return the observation ID they created; evidence-only
commits never make native verification stale (DRIVE-4 stays true).

Tested by: focused cases per fix, plus a repository-size guard in CI that
fails when a committed evidence artifact exceeds the receipt bound.

### M3: Knowledge complete, links never copies

Serves bar 1.

Scope: LIFE-23: all of KNOW is in the 1.0 scope — the librarian, the roll-ups,
the index and observations — re-scoped by the 2026-11-10 interview round 2
(#235): the vault holds only what has no other home (lessons, decisions with
their reasons, bug write-ups and short session stories in the DRIVE-26 form)
and links to hi/, specs/, .specsync/ and .chough/, never copies (KNOW-30,
KNOW-31, KNOW-32). The `knowledge/intent/` and `knowledge/archives/` copies
are removed by the cleanup change (#235 item 4, one cleanup PR per repo);
that removal changes contracted librarian behavior (the "Project librarian"
and "Knowledge directory built and kept current" requirements in
specs/cli/requirements.md), so it is its own native change, not folded into a
docs change. Includes #165 (whose first wording is superseded by KNOW-30),
#211 (parked by #235) and the KNOW criteria still open when this milestone
starts.

Done means: the vault contains no copy of a record that has a home; every
vault note either is original content (lesson, decision, bug write-up,
session story) or links; the index regenerates byte-identical from its
sources (KNOW-4); a fresh agent is pointed at the index at start and resume
(KNOW-1, FLOW-13); `.chough/sessions/archive/` records stay where they are.

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
raven-desktop, jackdaw, peck, podo-*, algochat and fledge, with chough itself
already dogfooding — completes at least one full Chough cycle on the release
candidate.

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

## Session slug folders: migration design

DRIVE-24 changes the session layout from one file per session to one folder
per session, so parallel sessions never share a file and active work is
separated from archives exactly like SpecSync's `changes/` and `archive/`.
DRIVE-25 to DRIVE-27 change the record format itself: sessions, decisions,
bugs and evidence become Markdown with YAML frontmatter, one file per record,
named by date plus slug, with old sequential IDs kept as aliases.

### From (0.x, today)

- `.chough/sessions/<slug>.json` — one JSON file per active session
- `.chough/sessions/archive/<slug>.json` — one JSON file per archived session
- `.chough/session.v1.json` — the frozen legacy checkpoint (the legacy anchor
  of amended KNOW-29)
- `.chough/transitions.jsonl` — one shared append-only transition log
- `.chough/history/cycles/` — shared cycle history
- `.chough/bugs/BUG-NNNN*.json`, `.chough/evidence/OBS-*.json` — sequential
  JSON records

### To (1.0)

- `.chough/sessions/active/<date>-<slug>/` — one folder per active session,
  holding its Markdown session record with YAML frontmatter, its append-only
  transitions and an append-only inbox/ for subagent observations (DRIVE-3)
- `.chough/sessions/archive/<date>-<slug>/` — the same after archival, plus
  the archive receipt; the vault links to these and never moves or copies
  them (KNOW-30)
- `.chough/bugs/<date>-<slug>.md`, `.chough/evidence/<date>-<slug>.md`,
  decisions likewise — one Markdown file per record, the file itself the
  source of truth, no JSON twin (DRIVE-25)

### What migrates

- Each active `sessions/<slug>.json` becomes
  `sessions/active/<date>-<slug>/` with its content carried into the Markdown
  record's frontmatter and body; the format version is bumped by the
  migration receipt.
- Each archived `sessions/archive/<slug>.json` becomes
  `sessions/archive/<date>-<slug>/` the same way.
- The shared `transitions.jsonl` is partitioned: every event that names a
  session is appended to that session folder's transitions; events that name
  no session stay in a committed legacy log that readers still honor.
  Nothing is deleted; the old files are removed only after their replacement
  is committed (the KNOW-7 rule).
- Every `BUG-NNNN` and `OBS-*` record converts to its Markdown form under its
  creation date and slug, with the old ID recorded as an alias in the
  frontmatter so existing links keep resolving (DRIVE-27).
- `session.v1.json`, `history/`, `cases/`, `artifacts/` and
  `preferences.json` do not move.

### How a 0.x checkout upgrades

- The migration is explicit: `chough migrate` performs it, as with the
  schema 1 to 2 migration; it never overwrites existing files, and it writes
  a migration receipt with the before and after digests.
- A 1.0 helper on an unmigrated checkout reads the old layout well enough to
  say "run `chough migrate`" and otherwise fails closed; it never silently
  rewrites records.
- Migrations chain: 0.1.0 → 0.2.0 → … → 1.0, one tested step per released
  format change, so a checkout at any 0.x tag reaches 1.0 by running the
  steps in order.
- Downgrade is not supported; the receipt and Git history are the rollback.

## Notes and flagged conflicts

- FLOW-17 says "macOS and Linux; Windows through WSL"; the interview says
  "macOS + Linux. Windows is best-effort." FLOW-21 captures the interview
  wording; whether WSL stays the documented Windows path is Leif's call, and
  FLOW-17 is unchanged until then.
- KNOW-29 says older archived sessions "roll into the knowledge vault as a
  summary note with attachments", which reads as a move. The 2026-11-10
  interview round 2 (#235) resolved this: KNOW-30 now records that the vault
  links and never moves or copies, with at most a short session story that
  links. KNOW-29's roll-up now reads as link-and-story; the conflict is
  closed.
- The interview describes today's layout as "a single committed
  session.json"; main has had per-session files since 0.4.0 (DRIVE-3,
  DRIVE-13). DRIVE-24 is the folder form of the same direction, not a
  reversal; the conflict-magnet evidence (peck #182) came from a project on
  the older layout.
