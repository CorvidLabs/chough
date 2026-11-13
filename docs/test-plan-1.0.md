# 1.0 test plan

How each item of the 1.0 bar (LIFE-21) is proven. Companion to
[the 1.0 roadmap](roadmap-1.0.md). Evidence rules apply throughout: a
structural pass, an exit code or an actor label is never behavioral proof, and
each run is recorded as a short receipt with a content digest and the exact
command (LIFE-19), with raw logs in CI artifacts.

## Bar 1: dogfood checklist, per repo

Each active CorvidLabs repo — spec-sync, site, raven, raven-desktop, jackdaw,
peck, podo-*, algochat and fledge — completes the checklist on the release
candidate. Chough itself dogfoods continuously and needs no separate run.

Per repo:

1. Install the release candidate from the tag (`cargo install --locked`) and
   run `chough init` in the repo without overwriting anything.
2. Carry one real request through a full cycle: intent, plan (one native
   scope approval by the human), build, test with focused cases, accept
   (human review recorded natively), done.
3. Include at least one real bug found, reproduced, fixed and verified in the
   cycle where the repo's work surfaces one; do not invent one where the work
   is clean.
4. Hand off: a fresh agent with none of the conversation resumes from the
   committed project files and states the stage, gates and next action
   correctly (FLOW-13).
5. Record every run as evidence; confirm no data-loss or stale-record bug
   stayed open at the end of the cycle.

Done is a full row per repo: cycle change ID, resume observation, open-bug
list at close, and the receipts. A row with an unrecorded step does not count.

## Bar 2: zero known flakes

The flake tracker is a GitHub label (`flake`) plus one local bug record per flake in
`.chough/bugs/`, which holds its failing runs, diagnostics and stress results in its
Log (#217); there is no notes file under `docs/`. Each record names DRIVE-19 and its
issue as `GitHub #<n>` in a Log note. The label is the current list. It was seeded on
2026-11-13 with three closed flakes and one open one:
- #187: a timing test under a full macOS verify;
- #132: fork and fd inheritance (ETXTBSY on stand-in scripts, a held checkpoint lock);
- #169: a timing trace test straddling a second;
- #247: the exercise's same-second race, still open.

Rules:

- Any intermittent CI or local failure is filed as a flake within a day, with
  the failing run, seed or commit, and environment. A test that "passed on
  retry" without a filed flake resets nothing but is itself recorded.
- A flake leaves the tracker only when its fix is merged and the test has
  run clean in CI on macOS and Linux since the merge.
- The clean window: two consecutive weeks with CI green on macOS and Linux,
  no flake open, and no retried-then-passed job. Any new flake or retry
  restarts the window.
- The window is computed from the CI run history, not from memory; the
  computation is a recorded observation at release time.

## Bar 3: migration fixtures

Chough 0.6.0's `chough migrate` is the upgrade from every earlier release (Q7, #213,
REQ-records-009). Fixtures exercise it:

- `tests/fixtures/` holds one fixture per released tag, v0.1.0 to v0.5.0, each a
  `.chough` tree captured from a project built from that tag, small enough to review
  in a diff and regenerable by a script checked in next to it, run at that tag.
- Synthetic fixtures add what a captured project may lack:
  - a legacy single checkpoint with adopted history;
  - open and archived sessions;
  - sessions the 0.5.0 roll-up moved into the vault;
  - a bug of each ID form;
  - inbox entries;
  - external and committed artifacts;
  - home-folder paths in records;
  - old proof.
- A CI case runs `chough migrate` against a copy of each fixture and asserts:
  - the plan and its digest, and the written tree;
  - the receipt, with before and after digests for every file;
  - every old ID resolving as an alias, and a removed ID reading `removed`;
  - each redaction;
  - `chough next`, `chough status`, `chough bug show`, `chough session list` and
    `chough timing` working on the result;
  - a second apply reporting nothing to do;
  - an interrupted apply finishing on the next run;
  - an archive that would read invalid making it restore every removed file and
    refuse.
- A separate case asserts that 0.6.0 refuses an unmigrated or mixed tree with
  `refused:unmigrated`, naming `chough migrate`, and never silently rewrites it.
- Later format changes before 1.0 add one tested step each, so a checkout at any 0.x
  tag reaches the frozen format.

## Bar 4: host-matrix e2e harness

One scripted end-to-end cycle that passes on each supported host — Claude
Code, Codex, Kimi, Grok and one local agent — before each release (LIFE-21
item 4, LIFE-22).

Design:

- A host-neutral scenario script drives a disposable sample project through:
  install the release candidate, `chough init --host <host>` (or the host's
  documented skill path), intent capture of a fixed small request, plan and a
  recorded stand-in scope approval, a one-file build, focused tests, accept
  with a recorded stand-in review, done and archive, then a cold resume by a
  fresh invocation of the same host that must state the stage and next action
  from the project files alone.
- Stand-in gates are marked as stand-ins; the harness never records a human
  decision (FLOW-10, FLOW-16).
- Per host, a thin adapter under `integrations/<host>/` (LIFE-24) holds the
  host's invocation, its skill-discovery check and its expected discovery
  path. The scenario asserts the same committed records on every host: same
  session folder, same transition kinds, same case results.
- The local-agent leg uses one qualified local model setup (for example on
  the Mac Studio); qualification means the full scenario passes once on it,
  recorded, before it counts as a supported host.
- Platforms: the matrix runs on macOS and Linux (FLOW-21). Windows remains
  best-effort and is not part of the gate.
- The harness has no runtime dependency on Herdr/hc or Rune (LIFE-24); where
  a host is driven through hc in practice, the versioned example template
  lives in `integrations/` and the harness result is recorded against the
  host, not the driver.
- Before each release, the matrix run's receipts are linked from the release
  notes; a missing or failed host leg blocks the release.

How it is built, after Leif's choices of 2026-11-13 (LIFE-27 to LIFE-30):

- The harness is `tools/host-matrix/` (REQ-cli-066). Each host runs as plain
  command-line runs, one per turn, with no Herdr; an hc template for watching a run is
  optional (LIFE-27). Its self-test drives a scripted stand-in host with no model. It
  must pass the good run and fail each known-bad run on the check that names it, and it
  runs in hosted CI on Ubuntu and macOS. How it judges is in
  [its README](../tools/host-matrix/README.md).
- Per-host adapters follow in later changes at `integrations/<host>/adapter.sh`, beside
  each host's hc template (#219).
- The Linux legs run in a Docker container on Leif's own computer, with the agents'
  logins passed in only for the run (LIFE-28).
- The local-agent leg is generic: any local command-line agent, with one settings file
  naming its command and model, counts once one full run passes and is recorded
  (LIFE-29, #220).
- From the next weekly release, a failed or missing leg blocks the release for every
  host that already passes; the local agent joins once it is qualified (LIFE-30).
