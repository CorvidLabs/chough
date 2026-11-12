# Changelog

All notable changes to Chough are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.6.0] - 2026-11-12

### Human-first records (breaking)

Chough 0.6.0 changes the record format. Chough 0.5.x cannot read 0.6.0 records, and
0.5.0 already cannot read main's evidence inventory field, so upgrade Chough for every
agent that works in a project before that project migrates. Then, with no native
change open, run `chough migrate` to preview the conversion and, with the human's
consent, `chough migrate --apply <digest> --consent "<who, their words, where>"` to
apply it once. Run both on the same machine and account: the plan's redactions depend
on the host and user name, so a plan previewed in CI or by another user has another
digest and does not apply.

Dogfood: this repository migrated itself with the 0.6.0 build on 2026-11-12, with no
native change open and the human's consent (3,238 files carried or removed with why,
215 written, 10 values redacted). Every digest in the receipt matched the commits
before and after, and CI's archive authentication passed over the migrated tree. The
release preparation then ran as one native change on the 0.6.0 build only: its scope
approval and its review ran through Chough's gates as the human, the move into done
wrote the story, the journal post and the index while every earlier story stayed
byte-identical, and `chough doctor` was clean after the two-step closeout. The
dogfood found three bugs, each fixed before the tag with a red then green case: an
in-place migration kept the machine-local active-session pointer holding a 0.5.x
session key, which now resolves through the session's alias (REQ-history-006); a gate
summary's "What changed" counted a second change on one branch from the merge base
with main, including the first change's work, and now counts from the change's own
`base_commit`, using the merge base only when default-branch work came in after that
base or no base is recorded (REQ-cli-061); and `chough migrate`'s preview, tried on a
project with open native changes, printed a digest without saying the apply would
refuse, and now names each open change under `apply_refused`, outside the digest
(REQ-records-009).
A 0.5.x build on a migrated tree sees no open session and refuses `bug list` at the
first Markdown bug; a 0.5.x write, such as `session new`, adds a 0.5.x file, after
which 0.6.0 refuses the tree as `refused:unmigrated` naming that file. So upgrade every
agent before a project migrates.

- **Every record is one Markdown file** with YAML front matter, and the file is the
  record: no JSON twin, cache or index is read in its place (REQ-records-001). Chough
  keeps front matter keys it does not know and every line a person wrote outside the
  sections it appends to, and refuses front matter outside its strict YAML subset with
  the file, line and rule.
- **Names are a date and a slug**, `<YYYY-MM-DD>-<slug>`, never a counter
  (REQ-records-003). A same-day clash gets six random hex digits, a name never changes,
  and every earlier ID (`BUG-0012`, `OBS-…`, a session key) stays as an alias that still
  resolves.
- **Sessions are folders**, `.chough/sessions/<id>/`, whose `session.md` front matter is
  the checkpoint, with the story (Goal, Where it stands, Decisions and why, What was
  verified, Open threads) and an append-only `## Timeline` in the same file
  (REQ-records-004, REQ-history-002, REQ-history-006). Each Timeline entry names the
  acting agent, model and hc terminal when known (#233, REQ-records-007). There are no
  event files, cycle folders or shared transition log, and no checkpoint merge: each
  session is its own folder.
- **A finished session becomes one story**, `.chough/archive/sessions/<id>.md`, kept for
  good and never moved, copied or trimmed (#165, REQ-history-006, REQ-records-012). The
  session folder leaves the tree only once its bytes are committed: commit the story,
  then run `chough doctor --fix` and commit that.
- **A gate holds the stage it names** (#237, REQ-cli-030).
  `chough checkpoint --awaiting-human "<gate>"` takes `--holds <stage>`, the current
  stage by default, so a pending release or publication decision no longer stops build,
  test or accept. It takes `--drives approve|review|none`, which defaults to approve at
  plan and review at accept.
- **A human's decision at a gate drives SpecSync as that human** (#236, REQ-records-008).
  Closing a driving gate with `--close-gate` runs `specsync change approve` or
  `specsync change review` first, as the human configured in
  `chough/identity.toml` under the user configuration directory or in
  `.chough/local/identity.toml`.
  - A review gate needs `--verdict pass|block`.
  - `--no-native` records a decision that is neither an approval nor a review.
  - Every gate close names a configured human, whatever the gate drives and with or
    without `--no-native`, so an intent or design approval is in a configured name
    too: an unknown `--decided-by` is refused as `refused:human-identity-unknown`
    before anything runs, through `--close-gate`, `gate record` and `change ship`.
  - On authority given in advance, the agent records the step as
    `<agent>-<terminal>` and quotes the human (FLOW-24).
- **No leaks in committed records** (#234, REQ-records-006). A local path, user or host
  name, email address or credential in a value Chough would commit refuses the write as
  `refused:leak`, naming the field and the kind, never the text. The global `--redact`
  stores `<redacted:<kind>>` instead.
- **`chough correct <entry> --reason <text>`** appends a correction to a Timeline entry
  and never rewrites history (#210, REQ-history-015).
- **Bugs are Markdown**, `.chough/bugs/<id>.md`, with an append-only `## Log`
  (REQ-cli-012). `bug update --status fixed-upstream --note <text>` closes a bug fixed in
  another tool. The `chough bug merge` driver now merges Markdown bugs field by field and
  the Log entry by entry; declare it for `.chough/bugs/*.md`.
- **The knowledge vault links and never copies** (REQ-cli-043, REQ-cli-044,
  REQ-records-011). It holds `knowledge/index.md` (the generated reading guide), the
  journal in `knowledge/journal/` with its Atom feed, and cross-project lessons. The
  move into done writes a short post that links the archived story. `chough journal
  add` writes any other post. `chough library merge` is the Git merge driver that keeps
  the generated index and feed buildable.
- **Chough's records stay out of SpecSync's verification** (#12, REQ-records-014). With
  `.chough/sessions/`, `.chough/archive/` and `.chough/bugs/` in
  `verification_ignored_paths` (SpecSync 6.1.2 and later), Chough's records can be
  written and committed at any point of the native order. `chough next` says when that
  holds, and `chough doctor` reports `records_in_verification` while an entry is
  missing.
- **`chough timing` reads every session's Timeline** and every archived story
  (REQ-reporting-001).
- **Tool versions and the lock.** A `toolchain` Fledge task runs first in the verify
  lane and names each local tool that differs from the CI pins (BUG-0022). A busy lock
  refusal names the process that holds the lock (#132).
- **SpecSync 6.3.0.** Chough 0.6.0 is built and checked with SpecSync 6.3.0. Every
  SpecSync Action step in CI uses the v6.3.0 release commit with `version: '6.3.0'`,
  and the install command, `chough change`'s install remedy, AGENTS.md, the archive
  scripts and the CI, CLI and integration docs name 6.3.0, on Leif's decision.
- **Evidence lives in its session** (REQ-cli-017, REQ-records-005, REQ-records-013). Each
  observation is one entry in `.chough/sessions/<id>/evidence/` with the exact command,
  a SHA-256 fingerprint and, for a failure, why when Chough knows it, such as an
  expected output that was missing, then at most 20 key failing lines. A repeated
  validation run is one entry that holds every attempt. Raw logs stay in
  `.chough/local/logs/` and in CI and are never committed. External artifacts are
  fingerprinted in place, and up to 5 small key screenshots per session are committed
  without metadata. When the session finishes, each entry becomes a receipt in its
  story.

Removed: `chough checkpoint --merge` and the `chough-session` merge driver;
`chough session adopt`, `chough history adopt` and `chough history prune`;
`chough library snapshot` and `chough library archive`; `chough session shelve
--legacy`; the schema 1 meaning of `chough migrate`; the knowledge vault's copied
notes, attachments and session roll-up; and, with evidence moving into sessions,
`chough evidence prune` and the retention ledger.

New: `chough migrate` (preview, `--apply`, `--consent`), `chough correct`,
`chough journal add`, `chough library merge`, `--holds` and `--drives` on
`--awaiting-human`, `--verdict` and `--no-native` on `--close-gate` and `gate record`,
the global `--redact`, `--failing-lines`, `--screenshot` and `--raw-log` on
`chough evidence add`, and the human identity file.

### Also in 0.6.0

- A journal post has a short human title and a real summary (REQ-records-011). Before
  the move into done the agent sets them with `chough checkpoint --title "<title>"
  --summary "<what changed and why; what was verified; what is next>"`; without them,
  Chough derives a title from the objective's first clause and a body from the story's
  Goal, first decision, newest verified entry and first open thread. `chough journal
  add --record <story> --title T --body B` writes the post of an archived story that has
  none, dated by the story's close, for a backfill. This repository's 69 archived
  stories each got one.
- At plan, while the change waits for its approval, `chough next` names the human scope
  gate and never passes on SpecSync's own `specsync change approve <id> --actor
  <name>`, so no agent is routed to approve its own scope (REQ-cli-021). At a scope
  gate, `chough gate list`'s summary counts and names the change's own deltas first,
  each with the requirement IDs it adds or modifies, then its plan and other Markdown
  files, since they are what the human approves; at a gate held at intent with no
  native change, it names the criteria the session references and the HI files that
  hold them (REQ-cli-061).
- Timeline fields with parts, such as a relayed decision's `relayed_by` and a direct
  write's `direct_write`, are labelled lines instead of JSON; entries written with JSON
  still read (REQ-history-002). A story's receipts are in observation-time order, its
  What was verified gives each case's latest run once, with its command once, and one
  line counting the earlier runs, which stay in its receipts, and its front matter says
  `stage_status: archived` with no stale `next` (REQ-history-006). The story comes
  first: its receipts sit last, in a `## Receipts` section, a `## Bugs` section names
  each bug of the session with its title and status, and a story written with its
  receipts in the front matter still reads.
- `chough init` and `chough session new` each write a `start` entry first, from and to
  the stage the session starts in, with its objective or key, so `chough timing` right
  after reports when the first stage started, in Cycle 1, which starts at that entry,
  instead of an unknown start before the first record; a timeline without one still
  reads (REQ-history-002). The timing text says "elapsed wall-clock" once, in its
  header, rather than after every number.
- `chough change check <id> --no-commit` runs SpecSync's check without `--commit`, the
  check the accept step runs first to materialize requirement text, so following the
  workflow records no direct lifecycle write; and a gate close with a human's decision
  records any direct lifecycle write before its own entry, so the story reads in the
  order the writes happened (REQ-cli-057, REQ-cli-060).
- `chough doctor` no longer reports a path under `/tmp/` or `/private/tmp/` in a case
  definition as a leak, since a case may set its own temporary data folder; a home path,
  user or host name, email or credential there still counts (REQ-records-006).
- `knowledge/lessons/` holds the first cross-project lessons, each from recorded
  evidence.
- From a fresh-eyes test of 0.6.0: `chough cases check` accepts a link to a
  requirement only the approved delta adds when the module has no `requirements.md`
  yet, instead of a raw file error (REQ-cli-009); `chough context` reads `AGENTS.md`
  from the Git root down only, named relative to the project; `chough gate record`
  reports the native step each close ran, and the approval note names the command the
  decision came through; a stage move's authority line says when Chough runs a native
  approval or review; a direct lifecycle write's Timeline entry says which files
  changed and how the state moved; `chough status` gives its next action whole, and
  closing the last gate holding a stage replaces a next action left stale by it; a
  bug's Steps start with `Trigger:`, `chough bug update --fix` sets its Fix, and
  verifying a bug with no Fix names the passing run (REQ-cli-012); a migration receipt
  says `none` under an empty section (REQ-records-009); and every `session.md` ends
  with exactly one newline, so `git diff --check` passes on a migration commit.
- From a second fresh-eyes round: closing a gate sets the next action to the step its
  decision allows, `chough stage advance`, `chough change ship <id>`, `chough stage back
  --to build` or, after `--no-native`, `chough checkpoint --next-action`, so `chough
  next` never names itself; at accept while the change is approved, `next` names the
  review preparation in order; and `next` and `status` keep every command whole,
  cutting only the text around it (REQ-cli-021). `chough gate list` names `chough
  change ship <id> --list <list> --gate <n>` for a gate whose change is verifying, and
  `gate record`'s refusal there says why (REQ-cli-061). `chough evidence add` needs only
  `--criterion`, takes `--requirement` and `--contract` as optional, and accepts a
  requirement the session's change adds in its delta (REQ-cli-005). `chough doctor`
  reports `merge_driver_outdated` when the `chough` the merge drivers run through
  `PATH` is missing or older than the running one (REQ-history-012). `chough migrate`
  on a migrated tree says there is nothing to migrate and gives no digest, and a
  receipt has one Follow-up heading (REQ-records-009). A close that runs no native step
  says why in its `native_step` and names its decision once; `session list` shows an
  archived story as finished; an executed case run's provenance no longer denies that
  the test ran; and without a native change `next` says only that none is recorded,
  with no trace IDs in its reasons. `docs/cli.md` shows a minimal HI file, a baseline
  `requirements.md` and SpecSync's Requirement evidence table for a new project, and
  says that `chough stage skip` skips the current stage.
- From a third fresh-eyes round: the workflow skill and `docs/cli.md` say that a lesson
  in `context.md` starts with its `YYYY-MM-DD` date, the only form the knowledge index
  counts, with an example (REQ-cli-015); `chough init --host claude` prints the line
  to add to `CLAUDE.md` as `claude_md`, writing nothing to it (REQ-cli-016); every
  `chough stage` move prints a short text summary, at most 10 lines naming what it
  applied, the move, the stage reached and the next action, with its full JSON report,
  every field as before, under `--json` (REQ-cli-008); and `chough migrate` previews the
  plan as text, since a person consents to it, with the same plan and digest as JSON
  under `--json` (REQ-records-009). At accept, `chough next` and `status` show
  SpecSync's review command as the human's review at the review gate, never as a step
  for the agent; at test, once every case's latest run passed, `next` names `chough
  stage advance` and counts superseded runs apart; and SpecSync's own next step is
  labelled for reference (REQ-cli-021). `chough session close` withdraws each open human
  gate with a `gate-closed` entry and names it under `withdrawn_gates`, never dropping
  it silently (REQ-history-006). A scope gate's summary names every delta first with its
  requirement IDs, and an intent gate's names the criteria and their HI files
  (REQ-cli-061). A duplicate `bug add` lists each similar bug as a line, and a near-miss
  `--close-gate` names the closest open gate. A failed run's body says why it failed
  before its key lines (REQ-cli-017); `chough context` gives the session, its Timeline
  and its open bugs as Markdown; a migrated session's verified lines link their entries
  and a migrated story's are receipt lines; and `chough timing` starts Cycle 1 at the
  start entry.
- From a fourth fresh-eyes round: a move to another stage replaces a gate close's next
  action with `Continue <stage>`, so build never says to run `chough stage advance` with
  nothing built (REQ-cli-021); `chough gate list` counts what changed in the working
  tree, so a change's delta, plan and other files count, the delta named first, before
  they are committed, and an untracked file counts its lines as insertions
  (REQ-cli-061); and while the project's installed skill copy is not current, as
  `chough skill status` reports it, the `chough migrate` plan's and receipt's follow-ups
  name `chough skill update`, since migration leaves a 0.5.x copy that still names
  removed commands and paths (REQ-records-009).
- From a fourth fresh-eyes round: at accept, while the latest run of a case the
  checkpoint references is not current, as after the first check materializes the
  requirement text, `chough next` names rerunning it with `chough cases run` before the
  review gate, and names the review gate once it passed (REQ-cli-021). The workflow's
  accept reference has any lesson written before the case runs, so one rerun covers it.
- README, `docs/cli.md` and the workflow skill give a new project's path, with
  `specsync change adopt` and the format of a change's delta.
- Stopping a command on macOS no longer leaves behind a child the command forked as the
  stop arrived: Chough signals the command's process group until no live member is
  left, for at most 500 ms, where one kill could miss a member the group gained while
  it was signalled (FLOW-14). The test stand-ins are also warmed at setup, since macOS
  checks each newly written executable on its first run, which takes seconds under
  load; three flaky tests are fixed by these two changes and #172.
- `chough cases record` returns `case` and `observation`, and `chough evidence add` returns
  `observation`, beside their existing keys, so automation no longer parses the ID out
  of `recorded` (#60, REQ-cli-011, REQ-cli-017).
- `chough session new` and `session use` keep a finished session that still holds an open
  human gate open, reporting it under `kept_open` with its gate count and recovery,
  instead of archiving the gate silently (#181, REQ-history-006).
- Rebuilding the knowledge index no longer makes current evidence stale: the content a
  run is checked against leaves out the generated knowledge directory (#136, REQ-cli-004).
- An evidence entry may list the inputs its run read (its input inventory). Chough reads
  that list strictly and within its bounds and refuses a path in it that is not a plain
  project path; freshness is still decided from the whole working content, never from
  the list.
- `chough skill update` recognizes the unedited skill files of every release from 0.1.0
  to 0.5.0, so it updates them, while a file a person edited is kept as it is (#102,
  REQ-cli-016).
- `chough timing` skips a direct lifecycle write in the Timeline (a SpecSync file
  changed outside SpecSync's commands) as part of no stage, as it skips gate records,
  instead of marking the report partial (#189, REQ-history-013, REQ-reporting-001). An
  entry of a kind it really does not know still marks the report partial.
- `chough change new` takes `--no-spec-change` and `--rationale` and passes them to
  `specsync change new` (#189, REQ-cli-057). `--no-spec-change` without `--rationale`
  is refused before SpecSync runs.
- The prototype template keeps hidden state panels hidden whatever display rules a
  project adds (#58, FLOW-4). Its `styles.css` gains `[hidden] { display: none
  !important; }`, so a rule such as `.surface section { display: grid }` no longer shows
  every state at once. An unedited `styles.css` from 0.1.0 to 0.4.0 updates as
  known-outdated.
- Right after `chough stage advance` into done, `chough library index --check` passes
  with no command in between (#191, KNOW-4, REQ-cli-043): the move writes the story, the
  journal post, the index and the feed in one step.
- `chough gate list` and `chough gate record` show the human what each gate decides
  and take one reply for listed gates only (#184, FLOW-19, FLOW-20, REQ-cli-061,
  REQ-history-014). `gate list` numbers every open human gate, each with a one-screen
  summary and the SHA-256 digest of that summary. The summary covers what changed since
  the change's own base commit, the tests, the plan's risks and a diff link; the tests
  are the latest observation of each case that links a requirement the change's deltas
  add or modify. It reads files and Git only, and saves the list untracked in
  `.chough/local/gate-lists/`. `gate record <list> --gates 1,3` with the four relay
  values closes exactly those gates, each relay keeping the same words, its own
  `summary_digest` and the `list`; it closes all of them or none. It refuses an unknown
  list, an unlisted number, a gate no longer open, a summary changed since listing, a
  gate of another open session, or a change that is verifying, and each refusal names
  its recovery. `chough change ship` takes `--list` and `--gate` in place of
  `--close-gate`.

## [0.5.0] - 2026-11-06

- Direct SpecSync lifecycle writes are noted in history (#183, DRIVE-21, REQ-cli-060,
  REQ-history-013). For the active session's native change, Chough watches the lifecycle
  records SpecSync writes: `state.json`, `approvals.json`, `review.json`,
  `review-attempts.json`, `verification.json` and `verification-attempts.json`. It
  compares them with an untracked baseline in `.chough/local/native-writes/`, which
  `chough change new`, `answer`, `check` and `ship` keep current, so Chough's own steps
  are never flagged. Any other change is recorded once, as a `direct-native-write` event
  naming each file added, changed or removed and the state before and after, with the
  next successful stage move. Nothing is recorded between `change check --commit` and
  finalize. `chough next` and `chough status` show pending writes and write nothing. A
  finding never refuses a command. Direct `specsync` calls still work; until approve and
  review move into Chough, they are what this shows. REQ-cli-057 now lets the change
  steps write that one baseline file. Older builds refuse a history holding a
  `direct-native-write` event ("unknown transition kind"), so
  the oldest Chough that reads such a history is 0.5.0, the first release with this
  change. Chough 0.4.0 does not read it.
- `chough change ship <id>` is the closeout after the human's review (#179, DRIVE-20,
  REQ-cli-059). Given the review gate's relay (`--close-gate`, `--decided-by`, `--words`,
  `--where`, `--recorded-by`), it checks the session, the stage, the open gates and the
  relay before anything runs, then runs `specsync change ship <id>` unbounded with its
  output passed through. After the ship it closes the review gate as a relayed claim and
  moves the session to done. Each refusal names the command that recovers, and a step
  refused after the ship says the change is shipped and names what is left. The
  walkthrough and the skill's closeout now run `chough change new`, `check` and `ship`,
  with review still `specsync change review`. The relay check `checkpoint --close-gate`
  runs is shared, with its refusals unchanged.
- Archived `next` and `status` reports can reuse SpecSync's authenticated verdict for
  identical archive/native/Git inputs instead of repeating the slow native read
  (REQ-cli-053, DRIVE-15, #32). The read state is `cached`; only those reports reuse it,
  and stage moves, timing, retention and verification stay live. Shallow/replaced/
  grafted/alternate history, active changes and every mismatch go live. A timed-out
  prime writes no entry and keeps the existing deadline and descendant kill. The two
  exact snapshot digest passes now run concurrently without changing freshness.
- The Linux test images are rebuilt from the repository with one command (#177,
  DRIVE-22). `fledge run linux-image` builds `chough-linux:1` to `:3` from
  `tools/linux/`, taking the specsync, fledge and human-intent versions from the CI
  workflow's pins. `fledge run linux-verify` runs the verify lane on the current
  commit inside `chough-linux:3`, and refuses with exit 3 when a tool is not pinned.
- `chough change new`, `answer`, `check` and `status` run SpecSync's own change steps
  (#171, DRIVE-20, REQ-cli-057), so an agent can drive a change through `chough` for
  them. `check` runs `specsync change check <id> --commit`, and the other three add
  `--json`. SpecSync's output, refusals and exit code pass through byte for byte, and
  nothing under `.chough/` is written. `status` keeps the native read bound, and a
  refusal past it names a retry with a longer bound. The writes are never stopped by
  that bound. A missing `specsync` refuses naming the install command and the step to
  run again. The workflow skill and docs now name `chough change` for these steps. This
  is slice 1 of "Chough drives SpecSync"; DRIVE-21, FLOW-19 and FLOW-20 are captured
  for later slices, and FLOW-16 and the Drive intent carry "amended by FLOW-19" notes.
- A project in a subdirectory of its Git repository can run cases and record evidence
  there (#170, DRIVE-18). The evidence snapshot then covers only the project's files,
  with the repository's `HEAD`. Before, every such command refused with "--root must be
  the Git repository root for evidence", and that advice did not work there. Nothing
  changes for a project at the repository root.
- The timing trace test no longer fails when its two runs straddle a second (#169,
  DRIVE-19). It runs each report once and requires them to be equal except for the
  exact clock shift between the runs, instead of retrying for a byte-equal pair.
- The README, docs, workflow skill and diagrams say what main does again, and
  `tests/docs.rs` fails when they drift (FLOW-18). It checks the stage diagram
  against the helper's stages and moves, every inline Mermaid block against its
  source and each SVG against its labels, the README helper table against
  `chough --help`, the README verify list against `fledge.toml`, every documented
  `chough` flag and relative link, the CLI reference's `.gitattributes` lines against
  the repository's, and stated SpecSync, Fledge and HI versions against CI's pins.
  Corrected: the skill's no-helper resume and checkpoint reference name sessions and
  event history; the stage diagram skips from design to plan and names the unlinked
  failure; the closeout closes the review gate with `--close-gate`; SpecSync's
  `verification_ignored_paths` ships from 6.1.2; the archive summary keeps the newest
  run (KNOW-17); releasing.md works for any version. Installed copies of the earlier
  `SKILL.md`, `checkpoint.md` and `lifecycle.md` update as known-outdated.
  The CLI reference's Commands table is also checked against `chough --help`, and its
  bug row names `merge` (#173).
- `chough doctor` finds and fixes the Chough debt that stops a fresh worktree from
  entering Chough (DRIVE-17): finished sessions left open, a legacy
  `.chough/session.json` to adopt, a stale active pointer, and awaiting-human gates on
  sessions whose native change is archived. With no flag or `--dry-run` it only
  reports. `--fix` applies the safe fixes. Withdrawing a gate and closing unfinished
  work (`--close <key>`) are decisions that also need `--reason`; without it they are
  reported as skipped. It makes no native read and records no human decision.
- A session whose native change is archived on HEAD is finished even short of done
  (#163, DRIVE-16), for example after a move into done timed out once SpecSync had
  finalized the change. `session new` and `session use` archive it, and with no
  active pointer it no longer makes other worktrees refuse with "several open
  sessions". `chough session close <key> --reason <text>` closes abandoned work with
  the reason recorded. Neither reads native state.
- Bug links name their way out, and a mistaken duplicate can be reopened (#153, audit
  Bugs row, REQ-cli-054). `chough bug update <id> --status open` on a bug marked
  duplicate used to fail with "duplicate_of is only set for duplicate bugs"; changing a
  duplicate's status now clears `duplicate_of` and notes it. A bug can no longer relate
  to itself. Self or unknown links, and unknown bugs, are refused naming
  `chough bug list --all`, and `verified` without linked proof names
  `chough bug update <id> --observation <id>`. New tests also hold every
  classification, scope and status, the blocking rule, the search scores and the
  report's fields. REQ-cli-054 states these mappings.
- Tests now hold positive track selection and every delivery-observation kind (#152,
  audit MA-01, MA-02). Requesting `--track copy-localization` selects its linked case
  while design-QA stays omitted, and both tracks select all three. Publication,
  installation, deployment, live-health and the legacy `delivery-observation` kind are
  each recorded and reported as themselves, and a dry run as a dry run. Each delivery kind is
  key-item proof: `evidence prune` protects it with its kind as the reason and the done
  trim moves it into the vault, while a superseded dry run is never protected and is
  removed with no vault copy. REQ-cli-014 gains the matching criterion; no behavior
  changes.
- The nine acceptance criteria the audit found with no assertion now have tests (#138).
  REQ-cli-002 (lesson labels, no lesson store), REQ-cli-007 (one checkpoint store, no
  APS), REQ-cli-012 (the bug-ID upgrade note), REQ-cli-029 (recorded authority, labels
  never decide, a skipped cold start is never a pass), REQ-cli-031 (the walkthrough
  opening), REQ-cli-045 (librarian off, no reset time) and REQ-reporting-003. For the
  last, `sh examples/measure-report.sh OLD NEW ROOT [RUNS]` pairs two binaries on one
  root and keeps native wait apart from local time. Each new assertion was shown to fail
  under a targeted mutation. Behavior is unchanged.
- A session archives itself at its own move into done and clears the active pointer
  (#155, REQ-history-006, DRIVE-13), so a done session merged to main is already
  archived and no other worktree's `session new` moves it into an unrelated change. It
  stays open while its inbox has unfolded entries or release is authorized.
- The KNOW-29 roll-up never rolls the archived session that holds the legacy history
  anchor (#143, KNOW-29 amended by Leif). It stays in `.chough/sessions/archive/` in
  addition to the five, so the anchor never depends on the knowledge directory.
- The chough-workflow skill's transition table now names `chough stage advance --to
  <stage>` (REQ-cli-052, DRIVE-9). Installed copies of `references/stages.md` from 0.1.0
  to 0.4.0 report `known-outdated`, and `chough skill update` refreshes them.
- An empty change directory left by a branch switch no longer counts as an open native
  change (#148). The knowledge index no longer lists it as active, which had made
  `library index --check` fail on a fresh clone, and the trim no longer refuses because
  of it. A directory with a change file still counts as open.

- A test now holds REQ-cli-033's ceiling of 8 archive validity reads at once on any
  host (#137, audit CH-M11). The reader count is a pure function, `validity_readers`,
  with no behavior change. A unit test checks a host that reports 128 cores, and a
  12-archive test sees no more than 8 reads overlap. Raising the ceiling to 128 fails
  both; the earlier overlap test passed it. REQ-cli-033 gains the matching criterion.
- `chough stage advance --to <stage>` moves through each main stage up to `<stage>` on
  one `specsync change status` read. Every step is checked first, as a plain advance
  would check it, and a refused step refuses the whole batch with nothing recorded
  (REQ-cli-052, DRIVE-9). The move into done, plain or batched, now reads native status
  once: the librarian trim takes the change's own manifest from the gate's read instead
  of reading it again (DRIVE-2). Other changes' manifests and the validity probes around
  a removal stay live, and nothing is cached past the command. This is the part of
  PR #69 that main still needed; its archive cache goes to #32.
- Chough supports macOS and Linux, and Windows through WSL 2 with Ubuntu (FLOW-17).
  Hosted CI no longer runs the native Windows product job; the Ubuntu jobs cover WSL 2
  as Linux, and the docs say what they do not cover (WSL 1, a checkout on `/mnt/c`,
  Git for Windows). Native Windows is not supported, matching SpecSync from 6.0.
  Requirements are unchanged, and the CRLF-safe tests still run on Linux and macOS.
- Relayed decisions at human gates (#123, audit D7, FLOW-16, REQ-cli-050,
  REQ-history-010). When a human decides a gate, the agent records it on their behalf
  with `chough checkpoint --close-gate "<gate>" --decided-by --words --where
  --recorded-by`. This closes only that gate, puts `relayed_by` on its `gate-closed`
  record and adds one handoff decision marked "a relayed claim, not authenticated".
  It writes no SpecSync record and satisfies no native gate. Missing values, values
  without a gate, a gate that is not open and an over-long decision each refuse with
  nothing saved and name their recovery. `--clear-awaiting-human` still withdraws gates
  and records no decision.
  - The awaiting-human refusal and waiting action name `--close-gate`.
  - README, the skill, the docs and the cli spec now say that approvals and reviews
    are recorded claims bound to their content, not authenticated identity.
  - Installed copies of the changed skill files update.
  - A binary without this change refuses a history event that carries `relayed_by`.
- History writes no longer refuse once the session that adopted the legacy history is
  rolled into the vault (KNOW-29) or shelved (#141). The legacy anchor is now also
  found in `.chough/sessions/shelved/` and in the vault's
  `<library-dir>/attachments/*/sessions/`, with the same exact match, and a changed
  legacy log is still refused.

- REQ-cli-039 AC2 now says that a `verifying` change shows no native-check note only
  with handoff readiness `safe`; any other or missing readiness keeps a note, as the
  requirement's body and last criterion already said (#126, audit D10). Behavior is
  unchanged.
- Only the five most recent archived sessions stay in `.chough/sessions/archive/`
  (KNOW-29, REQ-history-011). At each move into done, older ones move byte for byte into
  `knowledge/attachments/<key>/sessions/<key>.json`, and each gets a generated summary
  note at `knowledge/sessions/<key>.md` (REQ-cli-051). Nothing leaves Git history.

- The checkpoint cap is now 1 MiB (1,048,576 bytes), up from 128 KiB, chosen from
  measured use (DRIVE-14, REQ-cli-032):
  - It is at least 10 times the largest per-change session and 4 times the largest
    single checkpoint observed.
  - At the cap, `next` and `status` measured 69 ms on macOS and 19 ms on Linux in
    release.
  - A test builds a checkpoint at exactly the cap and checks the budgets, and one byte
    more is refused.
  - REQ-cli-021's full 40/40/50 x 4 KiB stress shape now fits and is tested as written
    (#125, audit D9).
- New `chough session shelve <key>` (and `--legacy`) sets an oversized or unreadable
  checkpoint aside, byte for byte, so work continues. One unreadable session no longer
  breaks `session list`, `session new`, `session use` or another session's `next`;
  `session list` reports it and names `shelve` (REQ-history-006).

- REQ-cli-013 no longer gives the librarian an `off` default. It defers to REQ-cli-015,
  so the requirements state one librarian default, `on`, which is what the runtime
  reports (#124, audit D8). Behavior is unchanged.
- The workflow skill and docs now teach an archive-first closeout (#122, audit D5).
  The order is: verify locally, the human's implementation review, the native review,
  finalize and the done move, and only then push and observe CI. The hosted Archive
  gate refuses while a change is open. The no-commit rule between check and finalize
  is unchanged. Installed 0.4.0 copies of the lifecycle and accept references update.
  The feature-and-bug exercise now runs this closeout against the real archive
  inspector, with a simulated reviewer.

- The retention ledger stays small (KNOW-23, REQ-cli-049). At done,
  `.chough/pruned.jsonl` keeps its newest 100 entries. Older entries that an archive's
  note summarizes move, with their restores, into
  `knowledge/attachments/<archive>/pruned.jsonl`. Every ledger reader reads those shards
  too, so labels, bug views and notes are unchanged. The step is reported as
  `applied.ledger_rotation`.
- Generated bug notes (`knowledge/bugs/*.md`) now union-merge and are rebuilt like the
  other generated knowledge notes, so parallel branches no longer conflict on them
  (#128, REQ-cli-047, DRIVE-12).

- Closed bugs release proof (KNOW-22, REQ-cli-048). In the librarian trim, a verified,
  duplicate or wont-fix bug no longer protects the observations it links. Its key items
  move into `knowledge/attachments/<bug>/` when they would otherwise be trimmed:
  - the earliest linked failure, with its artifacts;
  - for a verified bug, the first pass after it;
  - its fixture under `.chough/`.

  The other linked runs are trimmed when cold. `knowledge/bugs/<id>.md` links what
  moved. Open bugs, and `chough evidence prune`, keep every link.
- The move into done now drops every file and design reference from the checkpoint, and
  archived sessions' file and design references no longer protect old proof (KNOW-21,
  REQ-cli-045). `applied.checkpoint_references` reports them as `dropped_files` and
  `dropped_design`. Observation references keep the KNOW-16 rule. Cold loose folders
  that only those references named are now trimmed, and the native archive and the
  vault keep what the change touched.

- The `SpecSync Archive` shards now split the PR history walk as well as archive
  authentication (#105, DRIVE-10). Shard K inventories every fourth PR revision from
  position K and refuses a change omitted from any of them, so a long pull request no
  longer leaves shard 1 walking every revision alone. On the 361-revision #99
  candidate, shard 1 went from 268.6 s to 72.7 s on 4 CPUs, and
  the slowest shard takes 88.8 s. Every native check and
  the 10-minute limit are unchanged, and each shard reports the revisions it walked.

## [0.4.0] - 2026-11-05

- A refused trim no longer deletes vault attachments another writer changed (#110,
  REQ-cli-046, KNOW-10). Each vault copy is written and given its mode in a private
  staging file, then hard-linked into place, so a failed copy removes only its staging
  file. A rollback removes a copy only while it still holds the bytes and mode written,
  and names any copy it kept.
- Sessions per objective or native change (#77, REQ-cli-037, REQ-history-006): `chough
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
  binaries do not know sessions mode, so upgrade every agent before adopting. `session new
  --module` accepts the same SpecSync module IDs as `init`, such as `chat.service`.
- A session started with `chough session new` in a project that adopted its legacy
  transition log now writes events (#111, REQ-history-006). Its first transition used to
  refuse with "legacy history anchor changed", so a new session in an adopted project
  could not record a transition. The new session's history stays its own, the legacy log
  keeps its bytes, and a change to that log still refuses writes in every session.
- Parallel pull requests no longer conflict in files Chough writes (REQ-cli-047,
  DRIVE-12). Each change can add its cases in its own
  `.chough/cases/<module>.<part>.cases.toml`. `chough library index --check` fails
  naming stale generated knowledge, so the vault can union-merge and be rebuilt; this
  repository's verify and ci lanes run it. `chough bug merge` is a Git merge driver for
  bug records. It keeps both sides' notes and observations and leaves a real
  disagreement, such as two different statuses, to a human.
- The pinned SpecSync toolchain is now 6.2.0. Every SpecSync Action step in CI uses the
  v6.2.0 release commit with `version: '6.2.0'`, and the README install command,
  AGENTS.md, the archive scripts and the CI and integration docs name 6.2.0 (D-021).
- Old proof now moves into the knowledge vault (REQ-cli-046; KNOW-18, KNOW-19). Loose
  artifacts that no record references, such as folders kept from earlier pull requests,
  are summarized and trimmed once their change is cold, and an open bug keeps its
  fixture. Key items move into `knowledge/attachments/<change>/` and are linked from the
  change's note instead of being deleted: manual and agent-assisted observations, failed
  runs' logs, screenshots and failure-first logs. On this repository the preview found
  42 loose groups (259 files, 6.1 MB) and 36 observation sets, with 165 files moving to
  the vault. An archive ID over 24 characters gets a short, stable attachments folder (a
  readable prefix and 8 hex digits), and a key item whose attachment path would pass 220
  characters stays where it is, so the vault checks out on Windows. Moved and restored
  files keep their permissions. A move never overwrites an attachment that is already
  there. A folder named by the checkpoint or an open bug protects what it contains, and a
  loose folder with any current file stays whole.
- Old observation proof now leaves `.chough/` as changes go cold (REQ-cli-033,
  REQ-cli-045; KNOW-16, KNOW-17, KNOW-20). At done, the checkpoint drops references
  from closed cycles. The post-archive summary trims routine runs that a newer run of
  the same case and environment supersedes, keeping the newest run and the newest
  failure. Proof no archive lists is attributed to the change whose cycle it was
  observed in, and its knowledge note lists it. Archive validity is read in parallel.
- The knowledge directory is now an Obsidian-style vault (REQ-cli-044, KNOW-14,
  KNOW-15). Every note starts with YAML front matter (`id`, `title`, `type`, `tags`, and
  for an archived change its status, ISO date, modules, requirements, traced HI criteria
  and commits) and links with standard Markdown links that resolve. New notes connect
  everything: `modules/<module>.md`, `intent/<file>.md` (each criterion ID with the
  changes that trace it) and `rules.md` (rules, skills, decisions, policy). The index
  links only files that exist. KNOW-14 to KNOW-19 capture Leif's vault and reduction
  choices; the reduction (KNOW-16 to KNOW-19) comes in a later change.
- With the librarian on (the default), the knowledge directory now exists and stays
  current (REQ-cli-043, KNOW-1, KNOW-5). `chough init` creates `knowledge/`
  create-only, and every `chough stage advance` into done regenerates it after the trim,
  whether or not anything was trimmed, reporting `applied.librarian_index`. Before
  this, only `library init` and `library index` built it, and the trim at done wrote
  entries only when it removed something. Chough's own repository had no `knowledge/`
  while its trims left ledger entries pointing to summaries that were never committed;
  it now commits the folder. The workflow skill says to commit the folder with the
  checkpoint, and to regenerate it after a merge conflict. The cli and history lesson
  sources gain the lessons from 2026-11-04.
- `inbox add` now takes the checkpoint writer lock, so a concurrent `session use` can no
  longer archive the session between its check and its write and strand the entry, and
  evidence retention protects observations named by unfolded inbox entries (#98,
  REQ-history-009). Each entry is published whole, so an add stopped while writing leaves
  no partial entry behind.

- A command whose save is refused because its session file was archived, removed or
  adopted while it ran now also removes the history events it had written, so no orphan
  `EVT-` file is left (#82, REQ-history-008). A regression test shows `session use` waits
  for a running `stage advance`.

- The checkpoint writer lock now lives in the project, at `.chough/local/checkpoint.lock`
  (Git-ignored), instead of the system temporary directory, so writers with different
  `TMPDIR` values, or a temp cleaner, can no longer split it and lose updates (#83,
  DRIVE-11, REQ-cli-040). A busy refusal names the file it would write, the active
  session's in sessions mode. Chough releases before this still lock in the temporary
  directory and do not serialize with this one.

- `chough next` (text and JSON) now shows the checkpoint's open questions, at most 10
  with their total like blockers, and `chough status` reports all of them, so a
  question from `checkpoint --question` or a folded subagent inbox entry is no longer
  silently parked. `session list` reports an archived session's status as
  `archived`. `inbox add` of an observation ID with no evidence record still adds the
  entry but warns, and `inbox list` marks observation entries `recorded: true|false`
  (#84, LIFE-3, DRIVE-3, REQ-cli-021, REQ-history-006).

- With the librarian on (the default), `chough stage advance` into done now trims cold
  archived proof right after the move, under the same post-archive summary rules (ledger
  first, validity restore, nothing removed while a native change is open), and reports it
  as `applied.librarian_trim`. `chough pref set retention manual` opts out (#88, KNOW-13,
  REQ-cli-041).

- Stopping Chough on Linux now also stops what its command started outside the process
  group: a descendant that called `setsid`, and one whose parent exited (the `chough`
  binary is now the child subreaper of what it starts). Chough says on stderr when it
  stops a process that kept its own Git's output open, and when `git rev-parse HEAD`
  fails for a reason other than no repository or no commit (#81, FLOW-14, REQ-cli-011),
  including when `git` is not installed, which was silent before.

- A failed native command is now shown as `exit 2` or `killed by signal 15` instead of
  Rust's debug form `exit=Some(2)` or `exit=None`, in `status`, `next` and stage refusals
  (#87, REQ-cli-003).

- CI: the hosted SpecSync Archive check runs as four parallel jobs,
  `SpecSync Archive (1/4)` to `(4/4)`. Each authenticates a quarter of the native
  archives and keeps the 10-minute limit, and shard 1 also walks every PR revision.
  Every archive is still authenticated with every check unchanged (DRIVE-10). The
  single job had reached 7 to 10 minutes and timed out on a 91-commit PR, because each
  archive status read costs tens of seconds inside SpecSync.
  `check-specsync-archive.py --shard K/N` defaults to 1/1, which behaves as before.
- `chough next` now shows the native check: the latest attempt SpecSync recorded for the
  open change, with its time, commit and failing commands with exit codes, and, while
  the change is approved or implementing, that the check has not passed for this tree.
  SpecSync keeps no reason when it refuses before verifying (for example incomplete
  tasks), so `next` names `specsync change check <id>` instead of implying no failure
  (#86, REQ-cli-039). A `verifying` change whose tree moved after its last check, which
  SpecSync reports through `handoff.readiness`, is also shown as not passed for this tree.
  It reads one local file and adds no native read.

- `chough next` and `chough status` now warn when the native check runs none of the
  project's commands, judged from the effective `verification_commands` SpecSync reports
  in the status read they already make (only `specsync check …`), and count commands
  configured in `.specsync/sdd.json` that SpecSync 6.1.2 does not run
  (CorvidLabs/spec-sync#848). At the test stage `next` points to recording test
  evidence with `chough cases run` before accept (#85, FLOW-15, REQ-cli-038). No gate
  changes and no native read is added; the warning names at most 3 commands, each cut,
  so `next` stays within its bound. The `.specsync/sdd.json` reader moved from
  src/stage.rs to a small src/policy.rs. Replaces the closed #89.

## [0.3.0] - 2026-11-04

- Chough's size bounds now apply only to its own records under `.chough/` and to what
  it displays (#100, LIFE-15, REQ-cli-042, REQ-reporting-004). `chough evidence add`,
  `chough cases check` and `chough cases run` validate criteria against the complete
  `hi export -- <FAMILY>` output (up to 4 MiB) instead of its 16 KiB display section,
  so a HI family over 16 KiB no longer fails with "native HI family export is invalid
  or exceeds 16384 bytes". An archived `finalization.json` (`evidence prune`, history
  retention), an archived `change.md` (`library index`, which silently dropped its
  title and criteria), `.specsync/sdd.json` (stage gates) and the native records
  `timing` reads are read within SpecSync's 4 MiB bound instead of the 64 KiB record
  bound. Display cuts in `chough context` and native output now say they are for
  display only and send you to the complete source. The workflow skill says Chough's
  bounds never justify editing HI intent or criteria, specs or SpecSync files; report
  a native source over a Chough bound as a Chough bug. Every Chough record keeps its
  bound. Leif reported this after an agent shortened his HI intent in
  CorvidLabs/jackdaw to fit the 16 KiB bound: "Chough should only reduce and do all
  that for its own files in .chough and not the specs, not the human intent."
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

[0.6.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.6.0
[0.5.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.5.0
[0.4.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.4.0
[0.3.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.3.0
[0.2.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.2.0
[0.1.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.1.0
