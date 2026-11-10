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

The flake tracker is a GitHub label (`flake`) plus a notes file per flake
under `docs/` while one is open. Known today: #187 (timing test under a full
macOS verify, DRIVE-19) and #132 (fork/fd inheritance: ETXTBSY on stand-in
scripts, held checkpoint lock).

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

For every 0.x tag — v0.1.0, v0.2.0, v0.3.0, v0.4.0, v0.5.0 and each later
0.x — a fixture exercises the upgrade to the current format:

- `tests/fixtures/` holds one fixture per tag: a `.chough` tree captured at
  that tag (an active session, an archived session, transitions, a bug, an
  evidence receipt), small enough to review in a diff. The fixture is
  regenerable by a script checked in next to it, run at that tag.
- A CI case runs the current `chough migrate` (and, once it exists, the
  slug-folder migration) against a copy of each fixture and asserts: the
  migration receipt names every moved file with before and after digests;
  `chough next`, `chough status` and `chough cases check` succeed on the
  migrated tree; no record present before is missing after; a second migrate
  is a no-op.
- A separate case asserts the 1.0 helper fails closed with "run
  `chough migrate`" on an unmigrated 0.x tree and never silently rewrites it.

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
