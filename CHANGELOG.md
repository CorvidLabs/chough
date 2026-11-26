# Changelog

All notable changes to Chough are recorded here. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and versions follow
[Semantic Versioning](https://semver.org/).

## [Unreleased]

## [0.6.4] - 2026-11-26

- The chough-workflow skill routes every lifecycle writer through chough
  (REQ-cli-072): SKILL.md's old "run HI, SpecSync and Fledge yourself" line
  sent agents to specsync directly, which left chough session trails unable
  to reach done; reference files now show the `chough change` forms with the
  four unwrapped steps labeled as direct-native-write, and a CI check keeps
  the bundle honest.
- `chough change approve <id> --actor <name> --words "<their words>" --where
  "<citation>" [--recorded-by <agent>] [--note <extra>]` records a relayed human
  approval of a change definition through SpecSync, with the relay's fields —
  `relayed=pi chat`, `decided-by`, `words`, `where` and `recorded-by`, defaulting
  to the running Chough version — in SpecSync's note, and passes SpecSync's output,
  refusals and exit code through unchanged (REQ-cli-071, DRIVE-20). Without the
  approver, their words or the citation, or with one of them blank, it refuses
  before SpecSync runs, so a relay without a citation has no path through Chough.
- A relayed gate close's decision text now names the decided human's
  configured display name (REQ-records-008), so a close through an
  alias renders the display name; the stored relay keeps the raw
  `--decided-by` value as its audit trail, fixing #436.
- The `stage advance --to` not-ahead refusal now names `--reason` for
  `chough stage back` and `chough stage reset`, fixing #440.
- `chough next` now reads the refused-check reason SpecSync recorded in
  `verification-attempts.json` and shows it, instead of a note that keeps no
  reason, fixing #439.
- `chough gate record` validates every named gate before closing any, so a refused
  later close leaves no earlier gate closed, fixing #438.
- Direct-write entries write their `before_state` and `after_state` as `none` when the
  state is absent, in the Timeline entry and its text, fixing #437.
- `chough inbox add` writes its entry whole or not at all without a hard
  link, so it works on volumes without hard links such as exFAT, fixing #435.
- The roadmap records the 0.6.3 release as published, including the clean install
  of `chough` 0.6.3 on macOS arm64.

## [0.6.3] - 2026-11-22

- The workflow skill now ships an optional plan-stage fork and the bounded
  test-stage grouping loop, with tests pinning the wording. The plan reference asks
  one question: can the slice be cut into a left half and a right half with no shared
  file? One worker does a slice that cannot. A slice that can gets one manager and two
  workers, each in its own clone and branch. A worker that finishes stops, and neither
  worker imports the other half. The manager leaves out any path outside that worker's
  list, and then merges the two reports. The test reference names the bounded
  loop, the bug grouping and the validate consent. The stages and build references
  agree with the same wording.
- Optional GitHub stage labels. `integrations/github/` ships a workflow and a
  script a project copies in to label a pull request with the seven main Chough
  stages, in rainbow order. `chough init` does not install them, and the `chough`
  binary does not call GitHub. This repository runs the same two files.
- GitHub stage labels follow the open Chough session when a pull request
  also archives an older one. An archived story sets the label only when no
  open session remains.
- GitHub stage labels leave a pull request's current stage label in place.
  A different `chough-stage:*` label is still removed, and a missing stage
  label is still added.
- GitHub stage labels skip a fork pull request, ignore a symlinked session
  file, and create a label only when it is missing or its color or description
  differs. `chough-stage:implement` and `chough-stage:verify` are deleted only
  when they already exist.
- The documentation is checked against 0.6.2: the README, every page under `docs/`,
  the lifecycle and ownership diagrams, the workflow skill and the exercises. The
  closeout now commits the session folder with the story and the journal post; the
  release branch of the lifecycle diagram stays open at done; `stage advance --to`
  needs `--reason` across an unattended plan or accept; stale versions, test names,
  temp-file paths and leak-scan scope are corrected. The roadmap has the 0.6.2 row,
  and an installed 0.6.2 skill updates to the corrected files.

## [0.6.2] - 2026-11-21

- `chough change ready` counts only the change's own work after a rebase onto the
  default branch or a squash merge of it, so the default branch's files are no longer
  named as outside the declared paths; on the default branch itself every changed file
  still counts, and a base commit the repository no longer has is said plainly instead
  of Git's error (#406).
- A code fence that never closes, such as a session Goal of three backticks, hides
  nothing after it: saving the session no longer repeats its sections, `chough next`
  reads its decisions and open questions, and `chough bug merge` still finds a bug's Log
  (#394).
- The workflow skill's checkpoint reference and `docs/cli.md` say not to run `git pull`,
  `git merge`, `git rebase`, `git checkout`, `git stash` or another Git command that
  rewrites the worktree while another agent works there, since it can replace a record
  another agent's Chough command has just saved; a lone agent may run one between its
  own commands (#387).
- A new record is written whole or not at all: a full disk or a file-size limit no
  longer leaves a 0-byte or cut-off bug, post or entry that stops or misleads every
  reader, and an empty record file is named as empty (#260). `chough init` works on an
  exFAT volume again: `.chough/local/.gitignore` is no longer created by hard link
  (#424).
- Chough moves to SpecSync 6.4.0: install it on every machine that runs change commands,
  and in CI, at once. A machine still on 6.3.0 can be left with a Git-ignored
  `.specsync/hashes.json` after `chough change ready`, which Chough no longer removes. `chough change ready` now names whatever SpecSync's approval dry
  run refuses, in SpecSync's words, in place of its own delta-set rule (#406, #307); it
  no longer names SpecSync's own untracked `.specsync/change.lock` and
  `.specsync/workflow-v2-baseline.json`, or another active change's work, as outside the
  declared paths (#406); and it no longer makes or restores `.specsync/hashes.json`,
  which SpecSync 6.4.0 keeps outside the work tree (GHSA-7q5v-8jcc-rx54). A module's
  spec file needs no merge driver any more, since 6.4.0 records each change in its own
  history fragment (#404).
- The gate summary counts evidence recorded with no case, shows work not committed yet
  as `git diff <base>`, and counts a session started before the first commit from that
  commit (#389). `doctor`'s unfinished-archive fix names the session's journal post
  among the files to commit (#389). Without SpecSync, a move past plan or accept says
  no human decision is recorded, and `stage advance --to` across them needs
  `--reason`, which lands in the Timeline (#390).
- `chough migrate` reads every target, and every entry of the receipt folder, without
  following a link or waiting, and refuses anything in the way before it writes,
  naming the path relative to the project (#337).
- A 0.5.x file that comes back after a migration is judged by the committed receipt
  that lists it: with those bytes it is removed as already migrated; with others the
  preview and the apply refuse, naming it, the receipt and the way on (#370).
- An apply takes the writer lock before it plans, so of several applies of one digest
  only one applies (#378).
- No 0.5.x file is removed before every planned file is on disk; `knowledge/index.md`
  beside `knowledge/INDEX.md` is written by rename, and a regular generated index or
  feed is replaced as the plan says under `replaces` (#338).
- A folder the migration empties, left holding only macOS metadata, goes with it and is
  named (#375).
- `chough doctor --fix` keeps a pointer that names an open session by alias or bare
  slug (#376).
- A closed pipe no longer crashes Chough (#383). A command whose standard output or standard error is closed, as in `chough next | head -1`, stops printing and exits with the code it would have had. It no longer exits 101 with a panic, or 1 with `Error: Broken pipe`. A write command still saves.
- `chough help --help` and `chough help -h` work after the global options `--root`, `--redact` and `--session` (#399). The CLI reference names `--help` and `-h` as the help flags, and `chough -V` and `--version` as the top-level version flags.
- A change's description, `--rationale` and answers pass the leak check (#412, REQ-cli-057, REQ-records-006):
  - `chough change new` and `chough change answer` refuse a description, rationale or answer holding a local path, user or host name, email or credential before SpecSync writes it into the change's committed records. The refusal names the argument and the kind, never the value. `--redact` is the way out: SpecSync then gets `<redacted:<kind>>`, and one line says how many values were replaced.
  - `chough doctor` also scans the open changes under `.specsync/changes/`, but not their `deltas/` or SpecSync's archive. `--fix --redact` skips a finding there, naming `chough change answer` or a send-back.
  - The last places that repeated a value you gave now name what it holds (the rest of #367): `change ship --close-gate`, and an explicit evidence ID.
- `chough bug add` records the active session on the new bug, as its `session`, and no longer writes the session's record (REQ-cli-012, REQ-cli-054, REQ-history-006):
  - Two branches that each file a bug in the same session now merge without a conflict in `session.md` (#382).
  - Filing a bug costs the same however many observations the session holds. At 400 observations it takes well under a second again, not several seconds (#395).
  - A checkpoint holding its 200 bug links no longer refuses a new bug.
  - A session's bugs are the bugs its checkpoint holds followed by the bugs filed in it. `chough checkpoint`, `chough context`, the archived story and the blocking check all read them.
- A session file edited by hand to hold a blank, placeholder or doubled
  `no_native_change` is refused by every reader and save, naming the field and the
  edit, so it can no longer allow the ungated move (#401).
- `chough change ship` checks its whole history, however many commits; in a shallow
  clone without the base commit it refuses, naming `git fetch --unshallow`, and where a
  squash merge removed the base it says so and goes on (#400); its gate close and move
  into done act on the session `--session` or `CHOUGH_SESSION` names (#369).
- `chough inbox list` lists the pinned session (#393).
- `chough session use` and `session new` say on standard error when a pin still names
  another session, and `chough session shelve` takes an alias or bare slug (#372).
- `chough change new`, `answer`, `check` and `status` refuse an argument that holds a
  credential or a control character before SpecSync runs, so SpecSync's own error
  cannot show it (#367).
- Skill and create races (REQ-cli-016, REQ-cli-001, REQ-history-016):
  - Several `chough skill update` runs, or several `chough init` runs, at once each succeed, or refuse only as one on its own would. A file another run wrote first is kept, and no run fails with `File exists` (#386).
  - `chough skill update` refuses a skill file it cannot write, naming the file relative to the project, never with internal assertion text. A file it cannot read is reported as `uninspected` (#373).
  - A skill copy whose only difference is CRLF line breaks reads as the bundle file it equals, and is refreshed with CRLF (#373).
  - With a host's skill directory reached through a symbolic link, such as `.codex/skills` linked to the shared copy, plain `chough skill status` and `chough skill update` skip that host and report it as `uninspected`, with its directory and why, and update the rest. `--host` for that host refuses naming it, and no absolute path is shown (#403).
  - `chough doctor` also reads the records Git neither tracks nor ignores, such as a new session or an evidence entry a killed run left empty. It reports one its reader refuses as `unreadable`, and names a FIFO, socket or folder at a record's name without opening it (#408).
- Leaks and output (REQ-records-006, REQ-records-001, REQ-cli-055, REQ-history-006):
  - `chough checkpoint --resolve-question … --answer` is checked for leaks, redacted under `--redact` and bounded at 4,096 bytes, like every other checkpoint text (#364).
  - A Git conflict marker is named by its seven characters only, in refusals and in doctor's `conflict_markers` and `possible_fenced_conflict` findings, never with the branch name or commit subject on its line (#365).
  - An error shows a value you gave only when it holds no leak and no control character; otherwise it says what the option or variable holds, as in `CHOUGH_SESSION holds token`. This covers the session pin, `chough session use`, bug and evidence references, `--decided-by`, unknown questions, cases and stages, invalid IDs, a gate's text and `doctor --close`, and the argument parser's own usage errors (#367, in part: preferences, skills, stages, validation runs, `correct`, `library merge` and SpecSync's relayed errors follow).
  - The leak guard adds `{"password": …}` keys, `SECRET_KEY` and `AWS_SECRET_ACCESS_KEY` assignments, `xapp-` and `hf_` tokens, `~<user>/` paths, the macOS LocalHostName and each host's `<name>.local` name, and dotted host names inside hyphenated build names. It refuses a leaking `session new --module`, and `chough evidence add --redact` now says how many values it redacted (#368).
  - The user name comes from the account as well as `USER` and `LOGNAME`, so it is checked when they are unset. It counts only inside an absolute path, after `~` or as `user@host`, never in a relative path, a URL or a system root such as `/dev/`, and never as `root` (#379).
  - A save with `--redact` replaces a leak already stored in the session, as a decision an older Chough wrote, instead of adding a redacted copy beside it, and `gate record --redact` passes `--redact` to the closes it runs.
  - `chough gate record` says `Error:` once when a close it runs is refused (#380).
- `chough doctor --fix` no longer destroys work when it finishes an archive: an
  evidence entry the story's receipts don't hold, with its screenshot, and an unfolded
  inbox entry are kept, and the finding names how to move or drop each (#381). macOS
  metadata such as `.DS_Store` and `._*` files no longer blocks the finish (#377).
- `chough next` names finishing an archive while the session folder still holds files:
  commit the folder, story and journal post, then `chough doctor --fix` (#371).
- `chough doctor` finds Git conflict markers in a record's front matter and says to
  resolve the conflict, not to shelve the session. A Timeline entry that
  `doctor --fix --redact` redacted is no longer reported as rewritten (#374).
- The spec walk behind the whole-repository advice follows no link and stops at 4,096
  entries or 32 folders deep, saying so, so a link loop in `specs/` no longer hangs
  `chough doctor` or `chough init` (#402).
- `chough doctor --fix` no longer removes an archived session's folder while its story is
  not committed; it names it to commit first, `git add -f` for an ignored story (#389). A story an earlier
  release wrote still holds its entries, and an unreadable entry is kept and named.
- Control characters and echoes (REQ-records-016, REQ-records-006, REQ-cli-007, REQ-history-015, REQ-reporting-001):
  - **Control characters refused at write (#366).** A value you give holding one is refused as `refused:control-character`, naming the field and the character's code point, never the value, with every file unchanged. This holds under `--redact` too. It covers C0 controls, DEL, C1 controls, and the bidirectional overrides and isolates (U+202A–U+202E, U+2066–U+2069).
    - A line feed is still accepted wherever a field takes a line break, which is every field but `--title` and `--pr`.
    - A Windows line break is stored as a line feed.
    - A tab is refused like any other control character, where it used to be stored as a space.
    - A value a record already holds, written by hand or by an earlier release, never blocks a save.
  - **Control characters printed escaped (#366).** Text Chough prints from a record or from what you gave, in text and JSON alike, shows each control character as `\u` and four hex digits, so no terminal acts on it, and JSON still parses back to the stored value. Errors are included.
    - A line feed on a one-line slot, such as `chough next`'s `Native change: none needed:` line, shows as `\n`.
    - These hold the stored text as printed: the archived story, a journal post written from a story, `knowledge/journal/feed.xml` (which stays well-formed XML) and `knowledge/index.md`.
    - A `cases run` entry stores its command, summary and key failing lines as printed.
  - **YAML values quoted (#366).** A front matter value YAML reads as other than text (`null`, `~`, `true`, `false`, `yes`, `no`, `on`, `off`, in any letter case) is written double-quoted, so it reads back as text.
  - **`chough checkpoint --pr` (#353).**
    - It takes a pull request number, a positive whole number after an optional `#`, which is dropped, or an `http(s)` URL. Any other value is refused naming the two forms. `--pr ""` removes it.
    - It is written as a double-quoted string, and a stored `pr: ""` reads as no pull request.
    - `next` (a `PR:` line), `status`, `context` and the checkpoint view show it.
    - The journal post links only those forms.
  - **No more echoes (#367, in part).** The remaining errors and outputs that repeated a value you gave name the field and what it holds instead. These are the preference and consent keys, `--host`, stage names, validation runs, `correct`'s entry, `library merge` and `bug merge`'s files, `journal add`'s `--kind`, `--link` and `--record`, `stage backfill --at`, and the environment `next` and `status` show and the criteria, requirements and paths `cases select` lists.
  - **`file:` URLs and `USERNAME`.** A local `file:` URL is checked like the path it carries, and `USERNAME` counts as a user name like `USER` and `LOGNAME`.
- After a chough upgrade, `chough doctor` and `chough next` now say when installed
  skill files are an earlier release's: doctor reports each as `skill_outdated`, and
  `next` shows one line, both naming `chough skill update`. A file you edited is not
  reported. `chough status` also carries `no_native_change` (#361, REQ-cli-016).
- When a stage move is refused because the session records no native change, the
  refusal and `chough next` now name both ways on: record a change, or record with
  `chough checkpoint --no-native-change "<reason>"` that the work needs none. The
  workflow skill explains the flag, and a project installed by 0.6.1 gets the new text
  on `chough skill update`. Once the flag is set, `chough next` says the session needs
  no native change, with the reason, and the literal placeholder `<reason>` is refused
  (#351, REQ-cli-021).
- An archived story's What was verified line now shows an entry's limitations beside
  its claim, as `; limits: <limitations>`, so its stated limits are visible without
  reading the receipts. The line is always one line: a claim or a limitation that
  holds line breaks is folded, where a multi-line claim used to add extra lines to the
  story (#349, REQ-history-006).
- An evidence entry takes at most 8 limitations of at most 512 bytes each from
  `--limitation`; `chough evidence add` and `chough cases record` refuse more with
  nothing written. Unbounded limitations could push an archived story past its size
  bound and leave the session stuck at done (#349, REQ-cli-017).
- A new bug's Log now names its evidence as a link relative to the bug's file, as a
  migrated bug's Log does: to the open session's entry, or to the archived story that
  holds it as a receipt. When a session is archived its entries leave with its folder;
  `chough doctor` reports a bug whose link went stale as `stale_bug_links`, and
  `--fix` appends one Log entry linking the story, changing no earlier line (#347,
  REQ-cli-012, REQ-history-012).
- Follow-ups to the conflict-marker check (#339):
  - A conflict quoted inside a closed fenced code block, as an example or tool output, no longer counts as a marker. A fence that never closes hides nothing, and no fence hides a conflict in the `## Timeline` or `## Receipts` sections Chough writes.
  - A lone `=======`, which a half-resolved conflict may leave but a heading underline also is, is never refused. `chough doctor` warns of it instead, as an advisory `possible_conflict_separator` finding that does not change its exit status.
  - A conflict marker inside a code fence, which may be a quoted example or a real conflict Git left there, is never refused either; `chough doctor` warns of each such line as an advisory `possible_fenced_conflict` finding.
  - `chough doctor` reports markers in committed case definitions that Git left unparsable, and `chough cases check` names the file, the line and the marker. A valid definition may still quote a marker in a string.
  - `chough migrate` says in its preview that the apply refuses, without offering the apply command, and its refusal names the 0.5.x file to resolve.
  - The code-fence scan visits each line a bounded number of times, so a record holding many fence openers that never close no longer makes every write and `chough doctor` slow.

  (REQ-records-001, REQ-cli-055, REQ-cli-009)
- With `CHOUGH_SESSION` or `--session` set in a project where no session is open yet, a
  command's refusal now names the variable or option and gives `chough session new
  <key>`, instead of `chough session list`, which listed nothing. It still says when the
  named session is archived, and tells you to drop `--session` or unset `CHOUGH_SESSION`
  (#334, REQ-cli-037).
- `chough bug add` while a session is active now adds the new bug to that session, so a
  blocking bug filed in a session stops it leaving test even when it names nothing else.
  Since 0.6.1 such a bug was related to no session and stopped none. A bug filed with no
  session active stays unlinked, and `chough next` names it as not blocking. When the
  session cannot take another bug, as with its 200 bug links held, `bug add` refuses before
  writing anything (#329, REQ-cli-012, REQ-cli-054).
- `chough evidence add`, `cases run` and `cases record` now take the writer lock
  before they write their entry, so one refused as `refused:checkpoint-busy` after its
  30-second wait leaves nothing behind and a rerun records one entry, not two. Before,
  the entry was already written when the refusal said nothing was saved (#325,
  REQ-cli-040).
- A save refused because the session already holds a leaking ID reference now names
  the way out: `chough session shelve <id>`, then `chough session new <key>`. `chough
  doctor --fix --redact` never rewrites an ID any more, since a redacted ID no longer
  reads: it leaves that line as it is, reports its finding skipped with the same
  recovery, and still redacts the record's other leaks. Before, it rewrote the ID and
  left the session unreadable (#320, REQ-history-012, REQ-cli-055).
- `chough migrate` no longer enters or removes a folder whose name starts with `._`
  beside 0.5.x records, even when it holds only empty folders, so the apply changes
  nothing that its preview leaves out (#321 review, REQ-records-015).
- Chough's readers in `.chough/` now skip the metadata macOS writes beside files: a
  Finder `.DS_Store`, and the AppleDouble `._*` file macOS writes beside every file on
  exFAT and other non-Apple volumes. Before, one beside a bug, an evidence entry or an
  archived story stopped `chough next` and `chough status`, one in `.chough/cases/`
  stopped `chough cases check`, one beside 0.5.x records or knowledge notes stopped
  `chough migrate` or was removed by it, and `chough doctor` reported a committed one
  as unreadable. `chough doctor --leaks` now also reads a tracked binary AppleDouble
  file for leaks, and `--fix --redact` reports such a leak and leaves the file
  unchanged, for a person to remove (#314, REQ-cli-068, REQ-records-015).
- `chough change ship` now refuses before it runs SpecSync's review or ship when a
  commit in the change's history holds a change both open and archived, as a merge of
  main can leave. SpecSync reads that history as corrupt and said so only after
  archiving, so the ship reported an error after it had shipped. The refusal names the
  commit, the change and the recovery. The check is interim until SpecSync's own
  preflight (CorvidLabs/spec-sync#1082) (#298, REQ-cli-059).
- An archived story's receipts now keep each evidence entry's `criteria`,
  `requirements` and `limitations`, so the story says what was verified against what
  and its stated limits stay with it. They used to be dropped at archiving and survived
  only in Git history. A migrated receipt keeps them too (#286, REQ-history-006).
- `chough cases check` no longer refuses a retired case whose `fledge-task` binding names
  a task since removed from `fledge.toml`. A retired case is history, and its binding
  stays as written; an active case naming a missing task is still refused (#283).
- `chough checkpoint --pr <number or URL>` sets a session's pull request, which before
  only a hand edit of `session.md` could. It is one line, passes the same leak guard the
  field always has, and an empty value clears it; like `--objective` and `--title` it
  adds no Timeline entry (#282, REQ-cli-007).
- A session whose work needs no native change can now finish honestly. `chough
  checkpoint --no-native-change "<reason>"` records that and why; its stage moves are
  then allowed as `ungated:no-native-change-required` where they used to refuse as
  `refused:no-native-change`, so it advances to done and is archived with outcome
  `done`. An open human gate still has to be closed by a person, and the reason is
  checked for leaks like any other checkpoint value. Before, such a session
  could only be closed as abandoned (#281, REQ-cli-007).
- A bug that `chough migrate` writes now links each carried observation in its Log to
  the file that holds it: the open session's evidence entry, or the archived story
  that holds its receipt. It used to name them by bare 0.5.x ID, which a reader of the
  Markdown file could not follow (#277, REQ-records-009).
- Each shell can now pin its own session: every command takes `--session <id>`, and
  without it reads `CHOUGH_SESSION`, so several agents in one worktree each write to
  their own session whatever the shared pointer says, and the pointer never moves. A
  name that is not an open session refuses with nothing written. `init --session` and
  `inbox add --session` keep their meanings, and the example hc templates tell each
  agent to export `CHOUGH_SESSION`. Before, a `session use` in one agent's shell
  silently sent another agent's writes into the wrong session (#271, REQ-cli-037).
- A derived journal post's body now opens with the session's current objective, as its
  title is derived from it, so the two agree after a later `checkpoint --objective`.
  When the objective changed after the Goal was filled, the archived story keeps its
  Goal and gains one line, `Objective at closeout: <objective>`; nothing is rewritten
  (#344 review of #269, REQ-records-011, REQ-history-006).
- A session started without an objective no longer keeps the placeholder "Describe
  the current objective before beginning work." in its story. The first `chough
  checkpoint --objective` fills the untouched title and Goal with the objective, so the
  archived story and its journal post carry it. A title or Goal a person wrote is never
  rewritten (#269, REQ-records-004).
- During a git merge or rebase, `chough next`, `chough status` and `chough doctor` now
  say so and say to finish or abort it first. While a path under `.chough/` is
  unmerged, every command that writes a Chough record refuses as `refused:unmerged`,
  naming the paths, and `next` and `status` give only the merge and its next step.
  Before, Chough read and wrote its records mid-conflict with no warning (#266,
  REQ-cli-069).
- A record's body may no longer hold Git conflict markers. A write whose record would
  hold one (a line starting `<<<<<<<` or `>>>>>>>`, or a `=======` line between them)
  is refused as `refused:conflict-markers`, naming the file, the line and the marker,
  with every file unchanged. `chough migrate --apply` checks every file it would write
  before writing any. `chough doctor` reports each marker in a committed record as a
  `conflict_markers` finding, which is resolved by hand. A heading underline outside a
  conflict is not a marker (#265, REQ-records-001, REQ-cli-055).
- A second `chough migrate` on the same day, for a 0.5.x record written into an already
  migrated tree, now names its own receipt, `<date>-from-0-5-2.md`, and applies. It used
  to write the new session folder and then refuse on the first receipt's name, with no
  way forward. The plan names its receipt (`receipt` in the JSON, `Receipt:` in the
  text), the name is part of the plan's digest, and `--apply` checks every target, the
  receipt included, before it writes anything. Every migrate plan's digest changes with
  this release, because the plan now includes its receipt: a preview made with an
  earlier Chough needs a fresh `chough migrate` before `--apply` (#264,
  REQ-records-009).
- On a case-sensitive file system, `chough migrate --apply` now refuses before writing
  when `knowledge/index.md` already exists with other bytes. It used to write the other
  files and the receipt, remove every 0.5.x file, and then fail, leaving a receipt that
  claimed bytes it never wrote (#263, REQ-records-009).
- While SpecSync covers the whole repository, for example with `source_dirs = ["."]`,
  `chough init` and `chough doctor` no longer advise a `verification_ignored_paths` list
  SpecSync refuses as overlapping. They advise narrowing `source_dirs` to the code
  folders, for example `["src"]`, and keeping `.` out of `meaningful_paths`, then
  listing Chough's record folders. They say why: the overlap refusal, and that a
  whole-repository `source_dirs` counts the installed skill bundle as uncovered source
  When the specified code sits at the repository root, with no folder to narrow to, they
  advise adding `.chough` and the skill folder to `exclude_dirs` instead, and say the
  ignored list needs the code in a folder or CorvidLabs/spec-sync#1083 (#251, #279,
  REQ-records-014, REQ-cli-001).
- `chough doctor --fix --reason` now records the reason when it withdraws stale gates:
  each withdrawn gate's `gate-closed` Timeline entry holds it as `withdrawn: <reason>`,
  so the archived story says why, and doctor's JSON echoes it as `decision_reason` on
  each decision it applied. The reason is checked for leaks like any written value:
  refused, or redacted under `--redact`. It used to be written nowhere (#250,
  REQ-history-012).
- The README no longer says that 0.6.0 is not released yet, and the roadmap's
  delivered table has the 0.6.1 release row.
- `chough change ready <id>` lists everything still missing before a change is
  approved and exits 1 when anything is missing. Each finding names the command
  that fixes it, and the command changes no tracked file and no SpecSync hash
  cache (REQ-cli-067, DRIVE-29).

## [0.6.1] - 2026-11-18

- A test now checks the gate summary's change line, `Change: <id> - <title> (<state>)`,
  with a plain dash and no em dash, so the em dash #278 removed can't come back
  unnoticed (#294, REQ-cli-061).
- `chough evidence add`, `cases run` and `cases record` now read the session under the
  project's writer lock before they add their observation reference, so an update
  another writer saves meanwhile is kept. Before, they read `session.md` first and
  saved that stale copy, so parallel writers silently lost decisions and observation
  references. `cases run` still runs its case without holding the lock (#317,
  REQ-cli-040).
- A slug holding `-sk-`, such as the bug ID Chough makes from "Fix flaky sk-learn
  tests in CI pipeline", is no longer read as a model-provider key: `sk-` after a
  hyphen continues a word again, as in 0.6.0. New bugs with such titles are written,
  and 0.6.0 bugs with such IDs update and pass `chough doctor --leaks`. `sk-` at the
  start of a value or after a space is still refused, and the distinctive forms
  (`ghp_` and the other GitHub forms, `glpat-`, `xoxb-`, `AKIA`) stay refused after a
  hyphen (#316, a regression from #267's fix, REQ-records-006).
- The new-record writer added for #260 earlier in this cycle is reverted. It linked a
  temporary file to the record's name, so `chough init`, `bug add`, `journal add` and
  `session new` failed on file systems without hard links such as exFAT (#313), and
  writers running at the same time could lose their temporary files. A new record is
  written as in 0.6.0 again. #260 is open again and is redone in 0.6.2.
- The verify lane's feature-and-bug exercise (`examples/feature-bug-loop.sh`) now stops
  at a refused `chough` or SpecSync command, with that command's error. Before, it
  piped their JSON straight into `jq` without `pipefail`, so a refusal such as a
  refused `bug update --status verified` let it carry on and fail a step later. It now
  captures each command's output before `jq` reads it, since `set -o pipefail` is not
  in every `sh` (#274).
- `chough journal add` now holds the writer lock in a project with no `.chough/`
  directory too. It writes there with default preferences, and the first fix for #272
  took no lock on that path, so parallel adds could still leave the feed missing a
  post (review of #302, REQ-cli-040).
- `chough journal add` now holds the project's writer lock while it writes its post and
  rebuilds the feed and the index, as REQ-cli-040 already said it does. Several
  `journal add` commands run at once no longer leave `knowledge/journal/feed.xml`
  missing posts, so `chough library index --check` passes after them (#272).
- Every reference `chough checkpoint` records now passes the leak check before the
  write, like every other value Chough records. A `--file` or `--design` path holding
  a credential, an email or a user name refuses as `refused:leak` naming the field,
  and `--redact` stores it redacted. A `--criterion`, `--requirement`, `--case`,
  `--observation` or `--native-change` ID holding one refuses even under `--redact`,
  since an ID can't keep its form redacted. In a session that already holds such a
  value, `evidence add`, `cases run` and `cases record` refuse before they write
  anything. Before, such values were committed to `session.md` and only
  `chough doctor` found them afterwards (#268, REQ-records-006).
- The leak check now finds a credential that directly follows a hyphen, such as
  `x-ghp_…` in a session key, a decision or a raw log name, and `chough doctor` reports
  one already committed. Before, a hyphen in front hid the token from both, while the
  same token after a space, colon or dot was refused (#267, REQ-records-006).
- The record reader now opens a file without blocking and checks on the opened file
  that it is a regular one. Before, it checked the path and then opened it, so a file
  replaced with a FIFO in between still blocked the reader. `chough session shelve`
  reads through the same open (review of #305, #262).
- A FIFO or other non-regular file in a record's place no longer blocks a reader.
  `chough status` and `chough next` skip such an entry in `.chough/bugs/`, as
  `chough bug list` already did. A session whose `session.md` is one is listed by
  `chough session list` as not readable, with the reason and the recovery
  `chough session shelve <id>`, instead of vanishing from the list, and the shelve
  moves the folder without reading it (#262, REQ-history-006).
- A blocking bug stops only a session that lists it, holds its evidence, or shares a
  criterion, requirement, or case. Other sessions are warned by name, and a blocking
  bug that names nothing stops no session (#304).
- `chough skill update` replaces a pristine copy of the workflow stages file from main 0fd9fd22, and the architecture note says which session files `session use` reads (FLOW-18).
- `chough help --help` now prints the top-level help and exits 0, and `chough help
  <command>` prints that command's help, so the CLI reference a user reads is the one
  Chough actually lists; `chough checkpoint --help` now prints clap help text for the
  eleven list-valued flags (`--criterion`, `--requirement`, `--case`, `--design`,
  `--file`, `--observation`, `--bug`, `--decision`, `--question`, `--completed`,
  `--blocker`), and `docs/cli.md` documents the `migrate --json`, `doctor --leaks`,
  `stage help` and top-level `chough help` commands the reference missed (#301, REQ-cli-057,
  DRIVE-17).
- This repository routes `specs/**/*.spec.md` and `specs/**/requirements.md` to
  SpecSync's ancestor-aware merge driver, so two branches that each bump a module's
  version and Change Log merge locally without a hand-resolved conflict. `AGENTS.md`
  names the one-time `specsync merge --install` (#291, DRIVE-12).
- The docs, the workflow fast path, and the five host templates now say what current main does, including write-time checks, the daily CI run, journal posts only when the librarian is on, and the short rules a local agent follows (FLOW-18).
- The knowledge index and feed (`knowledge/index.md`, `knowledge/journal/feed.xml`) are
  rebuilt and checked once, at release, not at each move into done or in each pull
  request, so open pull requests no longer conflict on them (#289, REQ-cli-043,
  REQ-cli-047). The move into done writes its journal post and reports the index
  `deferred`; no verify, CI or pull request lane runs `chough library index --check`.
  `fledge run release-check` rebuilds the index on the release preparation branch, fails
  until the rebuild is committed, then runs the strict check. Between releases the index
  on main lags its sources.
- A closed session's archived story writes its What was verified receipt lines with
  ` - ` between the claim and the command or the first fingerprint digits, and a gate
  summary names its change as `Change: <id> - <title>`, so Chough's own records no
  longer carry an em dash that fails the CorvidLabs prose lints (#278, REQ-history-006,
  REQ-cli-061). Stories archived earlier are not rewritten: the journal reads a claim
  from either join (REQ-records-011).
- `chough init` still adds a missing skill file without touching its neighbors, and a
  session whose files were edited since init stays byte-identical on a later init.
  Init never removes or rewrites a file the user or a previous run created
  (REQ-cli-001).
- Chough's change log adds one fragment per change, `changelog.d/<YYYY-MM-DD>-<change-id>.md`, holding
  that change's bullets, instead of editing the `CHANGELOG.md` Unreleased section, and a
  release task that assembles the fragments into the release heading and removes them, so
  two open changes no longer edit the same `CHANGELOG.md` section (#290, LIFE-16).
- A relayed gate close's native approval or review, including the review `chough change
  ship` runs, passes the relay to SpecSync 6.3.0 as `--words`, `--where` and
  `--recorded-by`, so SpecSync's approval and review records hold the relayed decision
  (#249, REQ-records-008). Each value is passed joined to its flag (`--words=<words>`), so
  relay text that starts with `-`, which Chough's own flags accept, is not read as a flag.
  SpecSync 6.3.0 then records such a change at workflow version 3, which SpecSync
  6.2.x refuses, so every checkout and CI job of a project that uses it needs 6.3.0.
- `fledge run flake-window` computes the 1.0 flake window from CI history (REQ-cli-062,
  #217). It reads the CI workflow's runs and jobs, the issues labelled `flake` and
  `.chough/bugs/`, writes nothing, and prints a JSON receipt: `clean`, `running` (with
  the latest restart and when the window can first be clean) or `blocked`. A red or
  partly skipped main run, any rerun, a filed flake and a day without a green main run
  on both macOS and Linux restart the window. It exits 2 when GitHub cannot be read in
  full. Its fixtures run in the `CI gate fixtures` job.
- `integrations/` ships optional, versioned example hc templates for Claude Code,
  Codex, Kimi, Grok and a local agent, the local one unqualified until #220
  (REQ-cli-065, #219). Each has hc's three front-matter keys, resumes with
  `chough next`, loads the skill file its own `chough init` creates and holds no local
  path. `integrations/templates.toml` lists each template's status and an ordered list
  of its versions with their SHA-256, and a README says Chough has no runtime
  dependency on hc, Herdr or Rune.
- The workflow skill's stage contract opens with a fast path for a routine change: a
  bounded first read, the session and native-change commands in order, a delta for every
  affected spec, a stop at each human gate, an early first edit and the checks, so a
  local or small-context agent edits early instead of surveying the repository. SKILL.md
  routes to it, and an installed copy of the previous bundle updates to it (LIFE-22).
- CI also runs once a day on main at 05:23 UTC, so the flake window sees macOS and
  Linux every day even when nothing merges. On a scheduled run the `SpecSync Archive`
  jobs take the run's own commit as their trusted base, since the event has neither a
  pull request base nor a previous commit (REQ-cli-064, #217).
- A host-matrix harness in `tools/host-matrix/` drives one scripted Chough cycle per
  agent host, one fresh host run per turn, in a throwaway copy of the shipping sample.
  It stands in for every human gate only as `SIMULATED-host-matrix-not-a-human` and
  judges the result only from the files, Git and native status. Its self-test runs a
  scripted stand-in host with no model, and hosted CI runs it on Ubuntu and macOS. Real
  host adapters follow (REQ-cli-066, #218).
- `chough migrate`'s preview and the receipt's `## Follow-up` also name what `chough doctor` will report on the migrated tree, from doctor's own session check run on the planned records: the finished sessions `chough doctor --fix` archives, and each session whose open human gates are on an archived native change, with each gate's text and the decision it needs, `chough doctor --fix --reason` (#246, REQ-records-009, REQ-cli-055).
- The verify lane's feature-and-bug exercise (`examples/feature-bug-loop.sh`) waits until the clock is past the failing run's recorded second before rerunning the fixed case, so on a fast runner the bug's `verified` status is no longer refused for a pass in the same second (#247). REQ-cli-012's same-second rule is unchanged.
- Record polish: a bug's actual behaviour is its own `Actual:` paragraph under What
  happened; `chough migrate` names each carried entry or receipt in front matter, its
  0.5.x ID kept as the alias, so `chough next` lists the new names; a fresh project's
  `.chough/README.md` no longer lists `artifacts/`; and the README's init paragraph
  names each host's skill directory in one sentence.
- The intent gate summary in `chough gate list` gives each criterion the session references by ID with its sentence from the `hi/` file that holds it, and no diff line, and each delta at a scope gate says which requirement IDs it adds and which it modifies instead of "adds or modifies" (REQ-cli-061).
- `chough library index` links no rule, skill, decision or policy file that Git ignores, such as a host's skill copy under an ignored `.grok/`, so `knowledge/index.md` is the same in a working tree and a fresh clone, and `library index --check` passes there (#245, REQ-cli-043).
- `chough next` with no open session, the normal state after done, says so, names `chough session new <key>` and exits 0 instead of failing, while `chough status` still refuses; `chough timing` says elapsed wall-clock once, in its header, with no gate-wait, case run or fix-time line repeating it, and its native note no longer reads "read read-only" (REQ-cli-021, REQ-history-006, REQ-reporting-001).
- At the 400-observation cap, `chough cases run`, `cases record`, `evidence add` and `checkpoint --observation` refuse before running or writing anything, naming the bound and `chough session shelve <id>`, and `next` and the leave-test gate take a case's latest run only from the observations the checkpoint references (REQ-cli-032).
- `chough change ship` prints SpecSync's output, then a short text summary of at most 12
  lines naming the review, the ship, the gate close and the move into done, in place of
  the gate close's long JSON report and the move's summary; when the summary is longer,
  its last line names how many lines were dropped and points at `--json`, which prints
  every step's full report (REQ-cli-059, #276).
- The leak check no longer refuses an ordinary word that equals a short all-letter host
  name: such a name is a leak only inside a path or a URL or as `<user>@<host>`, while a
  host name of 8 or more characters, or holding a digit, a hyphen or a dot, is still
  refused anywhere, in writes and in `chough doctor` alike (REQ-records-006).
- The leak check now flags an scp-style remote path, `host:/path`, as the host name and
  the path it carries: a home or system root after the colon refuses as a path leak, and
  the host part counts as the local host name when it matches, in writes and in
  `chough doctor` alike (REQ-records-006, #275).

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
  removed commands and paths (REQ-records-009). At accept, while the latest run of a
  case the checkpoint references is not current, as after the first check materializes
  the requirement text, `chough next` names rerunning it with `chough cases run` before
  the review gate, and the review gate once it passed (REQ-cli-021); the workflow's
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

[0.6.4]: https://github.com/CorvidLabs/chough/releases/tag/v0.6.4
[0.6.3]: https://github.com/CorvidLabs/chough/releases/tag/v0.6.3
[0.6.2]: https://github.com/CorvidLabs/chough/releases/tag/v0.6.2
[0.6.1]: https://github.com/CorvidLabs/chough/releases/tag/v0.6.1
[0.6.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.6.0
[0.5.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.5.0
[0.4.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.4.0
[0.3.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.3.0
[0.2.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.2.0
[0.1.0]: https://github.com/CorvidLabs/chough/releases/tag/v0.1.0
