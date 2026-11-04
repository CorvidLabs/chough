# Changelog

All notable changes to Chough are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

- The cli spec module is split again so its requirements companion stays under the
  65,536-byte record bound: REQ-cli-020, REQ-cli-024 and REQ-cli-028 are now
  REQ-reporting-001 to REQ-reporting-003 in the new `reporting` module, which owns
  src/timing.rs and tests/timing.rs, with wording unchanged. Nine cases moved to
  `.chough/cases/reporting.cases.toml` and six cli cases that also cover requirements
  staying in cli gained `TC-reporting-` twins. The ID mappings are in docs/FRD.md and
  specs/reporting/.
- The hosted SpecSync Archive check reads archived change status with one worker per
  CPU, at least two and at most eight, instead of a fixed two (DRIVE-10). Every
  archive and history snapshot is still authenticated with the same checks, SpecSync
  version and timeouts. On the #71 tree (63 archives) the check took 9 min 56 s with
  two workers, 5 min 4 s with four and 2 min 54 s with eight on the same machine.
- Stopping Chough with Ctrl-C, `kill` (SIGTERM) or a hangup during `chough cases run`
  or a native SpecSync read now stops the command it started, that command's process
  group and Chough's own Git commands, including the Git that `stage backfill` and
  `library snapshot` run, and records nothing partial, including while a case run saves
  its result, and while a descendant of that Git still holds its output after Git
  exits (stopped 2 seconds after Git exits when no signal comes); a library snapshot
  is now written whole or not at all (FLOW-14, #75). A hard `kill -9` is not covered. Unix
  only; Windows is unchanged.
- Cases may link a requirement that the checkpoint's approved native change adds or
  modifies in its delta for the case module before `specsync change check`
  materializes it, so failing-first cases run inside Chough (REQ-cli-009, Raven
  BUG-0004). The change must pass the build gate's native checks; Chough makes one
  native read per command for this, only when a link misses its companion, and lists
  such links under `approved_delta_requirements`. A refused link now says why the
  consulted delta did not resolve it. The testing reference describes how the test
  stage links failing-first evidence.
- `chough checkpoint --native-change <id>` asks SpecSync for the change before saving and
  refuses an ID SpecSync does not report (`refused:native-change-unknown`), leaving the
  checkpoint unchanged (#63, found in Kimi's second lab review). A reply whose
  `summary.id` or `change.id` names a different change is refused too
  (`refused:native-change-mismatch`), and so is a reply that names no change ID
  (`refused:native-change-unknown`).
- Dotted SpecSync module IDs such as `chat.service` and hi sub-criterion IDs such as
  `BOTS-1.a` are accepted by `init`, `checkpoint`, `bugs`, `evidence` and `cases`, and
  a checkpoint holding them is read by every view (#56, #57). Module and requirement
  IDs SpecSync refuses, and dotted criterion IDs hi refuses such as `UX-1.A`, are
  refused. Plain criterion IDs Chough accepted before, such as `UX-012` or `ux-1`,
  are still accepted so existing checkpoints and bugs stay readable, though hi does
  not trace them; re-record one with a valid hi ID, such as `UX-12`, to link it. `checkpoint --remove-completed` removes one mistaken completed
  entry in the same call that adds its correction. A test keeps the bundled workflow
  skill free of em and en dashes (DRIVE-1, DRIVE-7).
- `chough next` now lists the newest 3 archived lesson bundles by recorded
  finalization time, the same three the librarian marks hot, instead of the last 5 by
  directory name, which mixed old and new work on a busy day (#74). It reads them with
  the librarian's own rule, so hidden folders and archives without a readable
  finalization record are skipped, and a hot archive without a bundle is marked
  `(missing)`. The new-cycle guidance after a reset uses the newest reset by time.
- After `chough stage reset` from an archived change, `chough next` keeps the reset's
  new-cycle guidance at every stage and suggests `specsync change new` instead of the
  archived gate refusal; at plan with no native change recorded it also suggests
  `specsync change new` rather than a bare status read (#76, items 2 and 5).
- `chough stage backfill --at <time> --commit <rev> --quote <text>` records a move to
  the next main stage that already happened, at its original time with the commit and
  quote it rests on (REQ-cli-036, REQ-history-005). Every advance refusal applies and
  native state is read now; it records no human approval or review. `chough timing`
  lists backfilled moves separately, marks the segments they bound and keeps those
  stages out of estimates. Older Chough binaries refuse a transition log that holds a
  backfilled record, so upgrade every agent on a project before using it.
- Sessions per objective or native change (REQ-cli-037, REQ-history-006): `chough
  session new <key>` creates `.chough/sessions/<key>.json` with its own history
  cycle, `session adopt` moves an existing checkpoint there byte for byte, `session
  use` and an untracked pointer pick the active session, and switching archives done
  sessions unchanged (a done session with unfolded inbox entries stays open, and
  `session list` shows each session's unfolded count). Subagents append create-only
  entries with `chough inbox add`, and `chough inbox fold` adds them to the active
  session after checking every entry and target first. Branches that record work
  in different sessions merge without conflict; `checkpoint --merge` still merges two
  edits of the same session. Each command saves to the session it resolved when it
  started, even if another process switches sessions meanwhile, and evidence
  retention protects what every open and archived session references, refusing when
  one cannot be read. Without `.chough/sessions/` nothing changes. Older Chough
  binaries do not know sessions mode, so upgrade every agent before adopting.

- Checkpoint writers in parallel in one clone no longer lose updates (#62): commands
  that change `.chough/session.json` take an exclusive lock (an empty file in the
  system temporary directory, named for the project) for their whole
  read-modify-write, waiting up to 30 seconds before refusing as
  `refused:checkpoint-busy` with nothing saved. A `.chough/.session.json.<pid>.tmp`
  left by a writer killed before its rename is now removed by the next writer that
  saves, with a stderr notice (#64); commands that only read remove nothing. New dependency: `fs4` 1.1.0, for a cross-platform file lock
  within MSRV 1.88. The lock file stays in the temporary directory after use and is
  safe to delete when no Chough command is running; the feature-and-bug exercise
  removes its own on exit.

## [0.2.0] - 2026-11-03

- Native SpecSync spec files under `specs/` are read at any size: `chough evidence
  add` no longer refuses a `requirements.md` companion over 65,536 bytes, and `chough
  next` finds the Lessons heading of a `context.md` over 4 MiB (REQ-cli-034).
  Every Chough record keeps its bound. Until a later change, `chough library snapshot`
  keeps its 4 MiB limit. The workflow skill's build reference gains
  advisory "Keep it composable" guidance; size is a hint, never a gate. Leif decided
  this on 2026-11-03.

- The librarian is on by default in `knowledge/` at the project root, with a recorded
  opt-out (`chough pref set librarian off --source ...`; KNOW-1, KNOW-5). `library
  index` now writes one regenerated `knowledge/archives/<change-id>.md` entry per
  archived native change and a lessons index of pointers, and lists an earlier
  `library/` folder as is (KNOW-2, KNOW-4, KNOW-6, KNOW-12). `chough next` names the
  index. Projects that relied on the old `off` default or the `library` folder name
  should set `librarian off` or `library-dir library` explicitly.
- New `chough library archive [--apply]` (REQ-cli-033; KNOW-7 to KNOW-11): a preview,
  or with retention authority and no open native change, removal of observation proof
  only cold archived changes list (outside the newest 3) and that is committed
  byte-identically at HEAD (compared with `git hash-object --no-filters`, so
  `core.autocrlf` cannot hide a difference; the exact bytes being removed are
  checked again after the native validity check, protections are re-evaluated
  then too, and a file or protection that changed refuses the apply with nothing
  removed; files are held in `.chough/removing/`, put back and restored without
  replacing a file written meanwhile, and `chough status` reports any stale hold), after its summary is written and its ledger line appended. A
  removal that changes any affected archive's terminal evidence validity, or that
  fails partway, is restored and refused, and an archive whose validity SpecSync cannot
  report refuses the apply before anything is removed. It never runs on its own after
  finalize. Knowledge entries stay within 64 KiB with cumulative manifests (paths are
  detailed in the entry of the archive that first listed them, at most 100 rows), and
  `library init`, `index` and `snapshot` build everything before writing, so a refusal
  leaves no partial `knowledge/` folder.
- The cli spec module is split so its requirements companion has headroom under the
  65,536-byte record bound: REQ-cli-018 and REQ-cli-025 to REQ-cli-027 are now
  REQ-history-001 to REQ-history-004 in the new `history` module, with wording
  unchanged, and their 22 cases moved to `.chough/cases/history.cases.toml` as
  `TC-history-` cases. The ID mappings are in docs/FRD.md and specs/history/.
- `--host claude` now installs and updates the one shared `.agents/skills` copy instead
  of a second `.claude/skills` copy, and hosts that share a directory get it once
  (LIFE-12). `skill status` and `skill update` report an existing `.claude/skills`
  copy as `legacy` and never write or delete it; the new `chough skill remove-legacy
  --host claude` removes only its unedited files and keeps edits (LIFE-13). Chough's
  own repository drops its `.claude` copy (LIFE-14). Tradeoff: Claude Code 2.1.316
  was observed not to discover `.agents/skills` alone, so it reaches the workflow
  through `CLAUDE.md` and `AGENTS.md`. Leif chose this on 2026-11-03. `init` does not
  write `CLAUDE.md`; add a line pointing at `.agents/skills/chough-workflow/SKILL.md`
  (docs/integrations.md).

- Approved-delta guard refusals that need a new definition approval now name the
  acting agent account and a human authority note, matching docs/cli.md, instead of
  asking the human to approve under their own name.

- The checkpoint `.chough/session.json` is bounded at 128 KiB (131,072 bytes) and
  its `observations` list at 400 pointers (LIFE-11). Leif approved raising these
  limits on 2026-11-02; corvid-agent selected the implementation values under the standing through-PR authority and the later instruction to finish these PRs and merge.
  Every other record keeps 64 KiB and every other checkpoint list 200 items. Older
  Chough releases refuse a checkpoint over 64 KiB, so upgrade before it grows past it.

- Reports lazily share one structural case-definition read per request, preserving
  case digests, freshness checks and native gate reads. No lookup loads no suite;
  later requests start fresh. `CHOUGH_TRACE=1` adds bounded stderr local phases and
  exact suite-load/lookup counts without changing stdout or record schemas.

### Tests

- Issue #30 acceptance evidence: a same-second run of one case in two Git roots
  (pinned by a test-only clock) and a real Git merge of independently added
  observations, artifacts and bugs both keep distinct identities, byte-identical
  records and current observations. No behavior change.

### History and protected retention

- Immutable transition events for new projects, explicit byte-preserving adoption
  for existing projects, and a history cursor in the sole checkpoint/current-cycle
  merge group. Unselected branch/orphan events remain separate.
- Bounded recent and timing reads, per-cycle storage capacity, and explicit
  whole-cycle retention preview/apply with protected proof, immutable receipts and
  conservative interruption handling. Evidence auto-retention stays separate.
- Evidence cleanup revalidates references and artifact identity before removal.

### Added

- Merge-safe checkpoints: `chough checkpoint --merge <base> <ours> <theirs>` does a
  bounded three-way merge of a schema 2 `.chough/session.json`. Compatible history
  and references merge deterministically; current-cycle fields merge only as a whole
  group, and differing changes are refused with a `current-cycle conflict`
  diagnostic, leaving every input untouched. The repository declares
  `.chough/session.json merge=chough-session` in `.gitattributes`; to let Git use
  the driver, each clone runs `git config --local merge.chough-session.name "Chough
  schema 2 checkpoint"` and `git config --local merge.chough-session.driver "chough
  checkpoint --merge %O %A %B"`. `chough init` never changes Git configuration
  ([#29](https://github.com/CorvidLabs/chough/pull/29)).
- Readable slug IDs for new records: new cases use curated `TC-<module>-<title-slug>`
  IDs instead of the next number, and new observations, repeated validation runs and
  librarian snapshots use `OBS-<slug>-<24 hex>`, `VAL-<slug>-<24 hex>` and
  `SNAP-<slug>-<24 hex>` with random suffixes, so parallel runs and separate clones
  keep distinct records. An explicit `evidence add` ID must use the new OBS form for
  a new write. Existing numeric cases, legacy observations, validation runs and dated
  snapshots stay readable with no migration; older Chough versions need an upgrade to
  use the new-form IDs ([#33](https://github.com/CorvidLabs/chough/pull/33)).
- Collision-resistant bug IDs: every new bug, whether from `bug add` or automatic
  failure capture, gets `BUG-<title-slug>-<24 hex>`, derived from a hash of its
  creation fields and never recomputed on update. Legacy `BUG-NNNN` bugs stay valid
  with no renumbering, and links work across both forms
  ([#26](https://github.com/CorvidLabs/chough/pull/26)).
- New-project setup path: `docs/walkthrough.md` now opens with a minimal HI file,
  `specsync init` and `specsync scaffold`, and `specsync change adopt`, and explains what
  `ungated:sdd-disabled` means. README Start here points to it. With SpecSync configured
  but SDD off, `chough next` reports "not checked: native SDD is disabled; run
  `specsync change adopt` to gate stages", and the `ungated:sdd-disabled` gate detail says
  the same. The walkthrough shows the `BUG-<slug>-<24 hex>` ID form, and the exercise
  notes state SpecSync versions only as observed (issue #40).
- Skill resume step: before its first edit, an agent states the recorded handoff in
  its own message (the goal, binding decisions with their reasons, the last failure
  with its bug status, the applicable lessons and the next task) and says what the
  records do not tell it. `SKILL.md` copies from main 118a895 are known-outdated, so
  `chough skill update` replaces them.
- Fresh-agent handoff blocks in `chough next`, read from existing records and within
  the same 16384-byte cap: the 5 newest checkpoint decisions with the kind and reason
  of entries written as `decided:`, `rejected:` or `failed:` `<text> because
  <reason>` (other entries shown whole; no checkpoint schema change), the latest
  failed observation by recorded time with the status of each bug that links it, and
  a pointer to each module's lesson source with the newest archived SpecSync lesson
  bundles. Output is ordered by recorded fields and names only, so the same records
  give byte-identical text and JSON.
- Skill guidance: lessons carry an `observation`, `hypothesis` or `verified` label
  with their source and limits, a project decision always outranks a lesson, a
  failure fixed in the same pull request still gets a bug or lesson, and closeout is
  one quiet pass inside the open native change.


- `CHOUGH_TIMING_NATIVE_BUDGET_SECONDS` (1 to 3600, default 120) bounds the total
  native wait of one timing report; changes left unread are `not-read`
  (`budget-exhausted`) with unknown times. Gates, `next` and `status` are unaffected.
- `CHOUGH_TRACE=1` writes bounded stderr lines for each native read and each timing
  phase, without changing output or exit codes.

### Fixed

- User-facing native writer examples now use the acting agent account and cite
  actual human authority separately, consistently with the workflow bundle.
- Missing referenced history is reported consistently for bare, single-quoted,
  backtick and double-quoted cycle directories after Git integration.
- History retention measures a reset-closed cycle's age from the closing reset,
  preserving recently closed cycles even after long inactivity.
- History inspection reports missing referenced pruned events after branch
  integration, and cleanup refuses until the referenced proof is restored.
- History retention normalizes directory aliases in frozen plaintext artifacts as
  well as structured references, preserving cycles named with Windows separators,
  case differences, repeated separators and dot components.
- `chough stage advance` and `chough stage skip` (and release → done) are refused as
  `refused:awaiting-human` while the checkpoint has an open awaiting-human gate, so an
  agent can no longer leave intent, design or plan before the human answers. The
  message names up to 3 gates and the clearing command; `next` and `status` say the
  moves are refused until the gate is cleared (#38).

- Leaving test for accept, by `stage advance` or `stage skip`, is refused with
  `refused:unlinked-failure` while the latest executed (passed or failed) observation
  of a case referenced in the checkpoint failed and no local bug links it; a later
  `not-run` observation does not clear it. `next` and `status` at test name the same
  refusal. A later pass, or a bug of any status linking the failure, lifts it
  (issue #39).

- CLI cleanups (issue #49): `bug add` accepts text values that begin with `--`
  (for example `--title "--name prints nothing"`); a native gate refusal names
  `specsync change status` once, and the `next` main line no longer repeats the
  native next action (it stays on its own line and in the gate detail), so it is not
  cut mid-word; the `next` text shows the last failure's time as UTC ISO 8601 (JSON
  keeps `observed_at_unix_seconds`); every `bug add` and `cases run` option has help
  text; `docs/cli.md` lists `history` and says SpecSync 6.1.2 flags an edited
  approved delta; `examples/feature-bug-loop.sh` removes the temporary directory it
  created after a passing full run, including under a relative `TMPDIR` (the path
  is made absolute before the script changes directory).

- Skill guidance for case recording order (#11, BUG-0017): the plan, testing and
  accept references say that a requirement an approved change adds is linkable only
  after the scoped `specsync change check` materializes it, so that check runs before
  adding, running or recording cases that link it, and running or recording cases is
  never a `tasks.md` checkbox, because the check refuses while one is unchecked.
  Copies of those three references from main 825a380 are known-outdated, so
  `chough skill update` replaces them.
- `chough next` and `chough status` no longer say `Continue <stage>`, or give the
  build gate refusal, at intent to accept once the recorded native change is
  archived with authenticated terminal evidence; they name the archive and the new
  cycle, recording done first at accept.

### Changed

- The current agent drives HI, SpecSync and Fledge internally and reuses applicable
  explicit prior or standing human scope authorization, recording its source and
  limits instead of asking the operator to repeat a tool-specific approval. Native
  checks and separate implementation acceptance and delivery authority remain.
  Unchanged main 825a380 skill and lifecycle copies can update without replacing edits.
- The skill now says SpecSync 6.1.2 ships `verification_ignored_paths` and why
  Chough's own repository has not adopted it.

- `chough timing` reads each native change at most once per report, and reads
  archived native history only for an unfiltered report (`--cycle all` with no
  `--stage`, `--case` or `--bug`). Filtered reports now emit `native.history: null`
  with `native.history_selection` naming the filters, instead of the archived list
  ([#34](https://github.com/CorvidLabs/chough/pull/34), REQ-cli-024).

## [0.1.0] - 2026-11-01

First release, published on 2026-11-01 as `v0.1.0` and crates.io `chough` 0.1.0.
Release steps are in [docs/releasing.md](docs/releasing.md).

### Added

- Resumable stage workflow: intent, design, plan, build, test, accept and optional
  release, with side stages for bugs, repeated validation and the librarian;
  checkpoint schema 2 in `.chough/session.json`, focused cases (`chough-testing/1`),
  local bugs, preferences and run consents, and per-host skill installation
  ([#2](https://github.com/CorvidLabs/chough/pull/2)).
- Native archive CI checks: CI fails while a native SpecSync change is active or an
  archive fails to authenticate ([#3](https://github.com/CorvidLabs/chough/pull/3)),
  and the archive gate now runs before every other CI job
  ([#18](https://github.com/CorvidLabs/chough/pull/18)). No check is required to
  merge yet; see [docs/ci.md](docs/ci.md).
- Fail-closed stage gates on native approval validity, archive integrity and the SDD
  policy ([#7](https://github.com/CorvidLabs/chough/pull/7)).
- Bounded external commands: time and output limits for case bindings and native
  reads; validation honors a stop request after waiting
  ([#8](https://github.com/CorvidLabs/chough/pull/8)).
- Reference-aware evidence retention (`chough evidence prune`), current guidance and
  an installable package that carries no Chough records
  ([#9](https://github.com/CorvidLabs/chough/pull/9)).
- `chough timing`: how long stages, tests, bug fixes and human decisions took, from
  existing records ([#15](https://github.com/CorvidLabs/chough/pull/15)).
- Case binding `env`, matching on visible text with terminal escape sequences removed
  (an `expect_output` fragment containing ESC is refused), and actionable
  requirement and bug errors ([#16](https://github.com/CorvidLabs/chough/pull/16)).
- A compact `chough next` view, text or `--json`, each form within 16384 bytes with
  bounded lists and stated totals; an approved-delta guard that refuses moves to
  build, test or accept when an affected native delta no longer matches its
  approval digest, since SpecSync 6.1.1 status does not report it (BUG-0002); and done-stage resume that recommends a new
  cycle once the native change is archived
  ([#23](https://github.com/CorvidLabs/chough/pull/23)).
- Release preparation: pinned HI 0.8.0, SpecSync 6.1.2 and Fledge 1.8.0 with one
  install command and a test that keeps the pins in agreement, a minimum Rust
  version of 1.88 checked in CI, skill guidance corrected against the helper, and
  this change log.

### Changed

- Native status is parsed beyond the display bound, and checkpoint clears apply
  before additions ([#4](https://github.com/CorvidLabs/chough/pull/4),
  recorded in [#6](https://github.com/CorvidLabs/chough/pull/6)).
- Evidence stays current across commits that change only Chough records or SpecSync
  bookkeeping, and a bug is verified only by a pass observed after its latest
  failure ([#17](https://github.com/CorvidLabs/chough/pull/17)).
- The skill bundle holds no em or en dashes
  ([#16](https://github.com/CorvidLabs/chough/pull/16)); the README says how to
  pronounce the name ([#19](https://github.com/CorvidLabs/chough/pull/19)).
- Documentation checked against the helper: stage gates, CLI reference, CI, roadmap
  and diagrams ([#21](https://github.com/CorvidLabs/chough/pull/21)).

### Known limitations

See [Known limitations](README.md#known-limitations) in the README.

[0.2.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.2.0
[0.1.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.1.0
