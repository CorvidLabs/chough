# CLI reference

Run `chough --help` and each subcommand's `--help` for the installed executable. All
commands accept the global `--root PATH` (default: the current directory) and
`--redact` (see [No leaks in committed records](#no-leaks-in-committed-records)).
Recording evidence or running cases needs a Git repository with an existing commit.
The project may be the repository root or a subdirectory of it. In a subdirectory the
evidence snapshot covers only the project's files, with the repository's `HEAD` as its
revision, and changes elsewhere in the repository do not affect it (REQ-cli-056,
DRIVE-18).

Output is JSON unless stated otherwise. Exit codes: 0 on success; 1 when Chough
refuses or fails, with `Error: ...` on stderr; 2 for an unknown command or invalid
flag; 3 when `cases run` or `cases record` recorded an outcome other than `passed`.
When a gate a human closes drives a native approval or review and SpecSync refuses it,
`checkpoint --close-gate` passes SpecSync's output through and exits with SpecSync's
own exit code, and `change ship` does the same for SpecSync's ship or review.
`validate run` and `validate resume` exit 0 when attempts fail and report failures in
their JSON.

## The records tree

Chough keeps each record as one Markdown file with YAML front matter (REQ-records-001,
REQ-records-002). The front matter carries the IDs and fields agents read; the body is
the record for a person.

```text
.chough/
  README.md                        a one-screen guide to this tree
  preferences.json                 standing preferences (chough pref)
  cases/*.cases.toml               case definitions
  sessions/<id>/session.md         an open session: checkpoint, story and Timeline
  sessions/<id>/evidence/          the session's evidence entries
  sessions/<id>/agent/             agent working files, inbox/ included
  archive/sessions/<id>.md         one story per finished session, kept for good
  archive/shelved/<id>/            an unreadable session, moved byte for byte
  archive/migrations/<id>.md       a migration receipt
  bugs/<id>.md                     one file per bug, with an append-only Log
  local/                           ignored by Git, never committed
```

Each kind has one rule. A session folder lives while its work is open and leaves the
tree once its story is written and its bytes are committed. An archived story and a
bug file stay for good. Case definitions and preferences are configuration a person
edits. Machine-only files live in `.chough/local/` and are never committed. Intent
lives in `hi/`, contracts in `specs/` and change history in `.specsync/archive/`;
Chough links them and never copies them.

`chough init` and `chough migrate --apply` write `.chough/README.md` create-only, and
an existing one stays byte-identical.

`.chough/local/` holds only this worktree's state, with a `.gitignore` of `*`
(REQ-history-007): the active-session pointer `active-session`, the writer lock
`checkpoint.lock`, gate lists in `gate-lists/`, direct-write baselines in
`native-writes/` and an optional `identity.toml` (see [Human identity](#human-identity)).
Every machine-specific fact Chough keeps is there and nowhere else.

It also holds the raw logs of case runs in `logs/`, the paths of external artifacts in
`external-artifacts.toml` (see [Evidence entries](#evidence-entries)) and a pending
`validate stop` request, `validation.stop`.

A project that still holds 0.5.x records (`.chough/session.json`,
`.chough/sessions/*.json`, `.chough/sessions/archive/`, `.chough/history/`,
`.chough/transitions.jsonl`, `.chough/evidence/`, a `.chough/bugs/*.json` bug or
`.chough/pruned.jsonl`) is refused by every command other than `chough migrate`,
`chough doctor`, `chough skill`, `chough pref set|unset migration`, `--help` and
`--version`, as `refused:unmigrated` (REQ-records-010). The migration preference runs
there because it writes no committed file other than `.chough/preferences.json`, which
the migration keeps byte for byte, so a standing consent can be given before the first
migration. The message names up to 5 such paths with the count of the rest, says the
project uses the Chough 0.5.x layout, and names `chough migrate` to preview and
`chough migrate --apply` to convert. A 0.5.x file written into a migrated tree, for
example by an older Chough, is refused the same way. See [Migration](#migration).

## Commands

| Command | Behavior |
|---|---|
| `init` | `--objective` (creates the first session, keyed by `--session KEY` or the objective's slug; without it no session is created), `--intent-scope`, repeated `--module`, repeated `--host openai\|claude\|codex\|cursor\|gemini\|grok` (default openai; claude shares the openai `.agents/skills` copy, installed once). Installs the skill bundle; preserves existing files and never touches a legacy `.claude/skills` copy. With the librarian on (the default) it also creates the knowledge directory. It sets up Chough's merge drivers and reports them as `merge`: it appends the `chough-bug` and `chough-generated` lines `.gitattributes` lacks, creating it when absent and changing no line, and in a Git work tree sets each driver Git has no command for in the clone's local config (see [Merging branches](#merging-branches)). Prints the `records_in_verification` advice when `.specsync/sdd.json` leaves Chough's record folders in verification (see [Doctor](#doctor)). With `--host claude` it prints `claude_md`: the `line` to add to `CLAUDE.md`, since Claude Code 2.1.316 does not discover `.agents/skills` alone, whether `CLAUDE.md` already names the skill file (`present`) and a `note`; it writes nothing to `CLAUDE.md`. Refuses a 0.5.x project as unmigrated, and refuses `--objective` while a session is open. |
| `context [--json]` | Focused packet: instructions (each `AGENTS.md` from the project's Git root down to the project, named relative to it, never one above the repository), intent, scoped HI export, module sources, recent Timeline entries, valid transitions, preferences, open bugs. Markdown by default. |
| `status [--environment NAME]` | The full view, always JSON (`--json` is accepted and changes nothing). It starts with a `summary` object (see [The status view](#the-status-view)), then the checkpoint, the native gate for the next main stage in `checkpoint.gate` (`null` when that stage has no prerequisite), valid transitions, `next_valid_action`, `corrections`, native status, the snapshot, observations with freshness, and open and blocking bugs. |
| `next [--environment NAME] [--json]` | The focused action view, text by default and compact JSON with `--json`; see [The next view](#the-next-view). |
| `migrate [--apply DIGEST] [--consent TEXT]` | Without `--apply`, previews the migration of a 0.5.x project's records to 0.6.0 and writes nothing; with `--apply` and the preview's digest, applies that plan with the human's consent. See [Migration](#migration). |
| `checkpoint` | Show or update: `--objective` (clears run consents), `--next-action`, `--native-change`, `--decision`, `--question`, `--resolve-question` (with `--answer` when it resolves exactly one), `--completed`, `--remove-completed` (marks one exact completed entry withdrawn before any `--completed` additions, so a mistaken entry is corrected in one call; an entry that is not present is refused), `--blocker`, `--clear-blockers`, `--awaiting-human` with `--holds` and `--drives` (see [Human gates](#human-gates)), `--clear-awaiting-human`, `--close-gate` with `--decided-by`, `--words`, `--where` and `--recorded-by`, and `--verdict` or `--no-native` for a gate that drives a native step (see [Relayed decisions at gates](#relayed-decisions-at-gates)), `--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug` (a bug's name or alias; an unknown one, or an alias two bugs hold, is refused), and `--title` and `--summary`, the journal post's short human title (one line, at most 72 bytes) and its summary (at most 900 bytes, one paragraph per line), which the move into done uses (see [Moving into done](#moving-into-done)); an empty value removes either. A gate added with `--awaiting-human` that was not already open appends one `gate-opened` Timeline entry with its `holds` and `drives`, and `--clear-awaiting-human` appends one `gate-closed` entry per gate it withdraws, in the same save as the checkpoint change; `--clear-awaiting-human` records no decision. With SpecSync configured, `--native-change <id>` first runs `specsync change status <id> --json` and refuses as `refused:native-change-unknown`, saving nothing from the call, unless that read succeeds with JSON (a missing change, a failed or timed-out read, or non-JSON output refuses), and as `refused:native-change-mismatch` when the reply's `summary.id` or `change.id` names a different change; a JSON reply that names no change ID, such as `[]` or `{}`, also refuses as `refused:native-change-unknown`; without a SpecSync configuration only the ID format is checked. That read happens on every `--native-change` call: under a second for an open change, and seconds longer for an archived one, since SpecSync re-reads its acceptance records. |
| `stage show\|advance\|skip\|iterate\|back\|reset\|pause\|resume\|enter\|return\|backfill` | Moves between stages with reasons, each one Timeline entry. A move prints a short text summary by default, at most 10 lines: `Applied:` (the kind, with its gate, steps, backfill commit or authority), `Move:` (from and to), `Stage:` (the stage reached, its iteration and status), `Refresh:` after a back or reset, `Closeout:` after the move into done, `Next action:` (the checkpoint's) and a line naming `chough next` and `--json`. `--json` prints the full JSON report instead, with every field it gave before: `applied`, `stage`, `native_change`, `awaiting_human`, `valid_transitions`, `stages`, `recent_transitions`, `transitions_partial` and `authority`; `stage show` prints that report either way. `skip` skips the current stage, never the next one: it moves to the next main stage as `advance` does, under the same checks, and records `skip · <current> → <next>` with its required reason. To skip design, advance from intent into design with `chough stage advance`, then run `chough stage skip --reason "<why>"` at design, which records `skip · design → plan`; a skip at intent records `skip · intent → design` and skips intent. `reset` starts a new cycle and replaces the checkpoint's next action with `New cycle from <stage> after reset: <reason>. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action.`, the reason on one line and cut so the next action stays within 4096 bytes (the entry keeps it whole); `back` keeps it. Every move meets its destination's native prerequisite, whichever command makes it (advance, skip, back, reset, return, release → done): build needs state approved, implementing or verifying (another state is refused as `refused:native-<state>`) with `approval_valid` true (`refused:native-approval-invalid` otherwise) and `correction_valid` not false (`refused:native-correction-invalid` otherwise) and the approved semantic deltas unchanged (see [Approved-delta guard](#approved-delta-guard)); test and accept need that or an archived change whose terminal evidence authenticates; done needs archived (`refused:native-<state>` otherwise) with `terminal_evidence` validity `authenticated-history`, `exact` or `successor-covered` (`refused:native-archive-<validity>` otherwise). With no native change recorded a gated move is refused as `refused:no-native-change`; ungated moves are labelled `ungated:not-configured` or `ungated:sdd-disabled`. `release` is entered only from `done`, and side stages (`bugs`, `validate`, `librarian`) nest at most 4 deep. Intent, design and plan stay reachable. A malformed or ill-typed `.specsync/sdd.json` refuses gated moves as `refused:sdd-policy-invalid`; a valid `"enabled": false` or no policy file leaves them ungated, and a missing `enabled` means enabled, as in SpecSync. Leaving test for accept (advance or skip) needs no open blocking bug (`refused:blocking-bugs`) and, for each case referenced in the checkpoint, a latest executed (passed or failed) observation (by recorded time, then ID; a later `not-run` does not replace it) that passed or that a local bug of any status links (`refused:unlinked-failure` otherwise, naming the case and observation; rerun the case, or link it with `chough bug add ... --observation <id>` or `chough bug update <bug> --observation <id>`); `next` and `status` at test name the same refusal; entering validate or release needs consent or a standing preference. While an open human gate holds a stage the move would leave, `advance` and `skip` are refused as `refused:awaiting-human` before any native read (see [Human gates](#human-gates)); `back`, `iterate`, `reset`, `pause`, `resume`, `enter` and `return` are never refused by a gate. `backfill` records an advance that already happened; see [Backfilled moves](#backfilled-moves). `advance --to <stage>` moves through each main stage up to `<stage>` on one native read; see [Batched advance](#batched-advance). The move into done also archives the session; see [Moving into done](#moving-into-done). |
| `cases check\|list\|select\|run\|record` | Validate curated slug and exact legacy numeric definitions; list digests; select focused cases (`--criterion`, `--requirement`, `--path`, `--track copy-localization\|design-qa`); run an automated binding with `--environment`; record a manual or agent-assisted run with `--environment`, `--method manual\|agent-assisted`, `--summary`, optional `--artifact` and `--limitation`, and `--verdict N=pass\|fail` for every expected outcome. `run` and `record` name the evidence when `--observation-id` is omitted (see [Record names](#record-names)) and exit 3 when the recorded outcome is not passed. `record` returns the evidence as `observation` and the case ID as `case` beside `recorded`; `run` returns `case`, `observation`, `outcome` and `summary`. |
| `evidence add [ID]` | `add`: at least one `--criterion`, and kind, outcome, environment and summary; optional `--requirement` and `--contract`, each validated when given (a requirement the active session's native change adds or modifies in its approved delta counts, see [Approved delta links](#approved-delta-links)), `--artifact` (a project file spelled canonically: relative, `/`-separated; or one absolute path outside the project, fingerprinted in place), `--case`, `--method`, `--track`, `--limitation`, `--failing-lines` (a manual failure's key failing lines), `--screenshot` (one key screenshot committed beside the entry) and `--raw-log` (the CI run that holds the raw log); see [Evidence entries](#evidence-entries). Omitting ID names the evidence for you; an explicit ID must be a new-form name for a new write. Returns the stored ID as `observation` beside `recorded`. Never replaces an ID. There is no evidence retention in 0.6.0; see [Retention](#retention). |
| `bug add\|update\|list\|search\|show\|report\|merge` | Local backlog, one Markdown file per bug; see [Bugs](#bugs). `add` takes `--title`, `--symptom`, `--trigger`, `--expected`, `--actual` and `--environment` (required), and optionally `--impact`, `--criterion`, `--requirement`, `--case`, `--step` (in order), `--fixture`, `--observation`, `--classification`, `--scope`, `--regression-baseline` and `--source`; it refuses likely duplicates unless `--distinct REASON`, listing each similar bug, most similar first, as a line `<id>: <title> (<status>)` and naming `chough bug update <id> --observation <id> --note <text>` to append evidence instead, and regression needs `--regression-baseline`. `update ID` (the bug's name or any alias) appends one Log entry and takes `--fix TEXT` (sets the Fix paragraph and logs it), `--status` (`open`, `investigating`, `fix-pending-verification`, `verified`, `deferred`, `duplicate`, `wont-fix` or `fixed-upstream`, which needs `--note`), `--note`, `--observation`, `--duplicate-of`, `--related`, `--scope`, `--classification` and `--regression-baseline`; an update never renames the bug. `verified` needs a linked failed observation and a present linked passed observation observed after it (see [freshness](#observation-outcomes-and-freshness)); without them the refusal names `chough bug update ID --observation OBS`. `--duplicate-of` and `--related` must name another existing bug: a link to the bug itself or to an unknown bug, and an update or report of an unknown bug, are refused with nothing written and name `chough bug list --all`. Changing a duplicate's status clears its `duplicate_of` and notes it, so `--status open` reopens a bug marked duplicate by mistake (REQ-cli-054). `list` shows open bugs, `--all` every bug; `search TEXT` scores words in title, symptom and trigger plus shared `--criterion`, `--requirement` and `--case` IDs; `report` prints Markdown and posts nothing. `merge BASE OURS THEIRS` is the Git merge driver for `.chough/bugs/*.md` that `.gitattributes` declares as `merge=chough-bug` (see [Merging branches](#merging-branches)). |
| `pref show\|set KEY VALUE\|unset KEY` | Standing preferences, each set with a required `--source`. Values (default first): `repeat-validation` ask, off, always; `monitoring` ask, off, always; `release` on-request, standing; `migration` on-request, standing (a standing `migration` preference gives the human's consent to `chough migrate --apply`); `monitor-initiation` capture-only, propose-change; `librarian` on, off (on by default; `off` is the recorded opt-out); `library-dir` a path (default `knowledge`); `required-lane` a name (default empty). Only `always` and `standing` authorize without a run consent. |
| `consent show\|grant KEY --source S\|revoke KEY` | Run consent for the current objective: repeat-validation, monitoring and release. |
| `validate run\|stop\|resume\|status` | Optional repeated runs of one case with stored limits and local capture of failures. `run CASE --times N --environment E [--interval S]` (N from 1 to 1000; S from 0, the default, to 3600 seconds) needs a case with a binding and a `repeat-validation` run consent or `pref set repeat-validation always`; unlike `stage enter validate`, `monitoring` does not authorize it. `resume ID` continues a stored run under its original ID or alias; `stop` and `status` take no argument. A stop request (state `stopped`) or a paused checkpoint stage (state `paused`) is checked immediately before every attempt, including after the interval, which is waited in slices of at most 200 ms; both states resume with `validate resume`. |
| `library init\|index [--check]\|merge BASE OURS THEIRS` | The librarian, on by default in `knowledge/` (opt out with `pref set librarian off --source ...`); see [Knowledge vault](#knowledge-vault). `init` builds `index.md`, `journal/` and `lessons/`. `index` regenerates `index.md` and the feed, and removes a `README.md` an earlier release generated once its bytes are committed; `index --check` writes nothing and fails naming each generated file that is missing or differs (see [Merging branches](#merging-branches)). `merge` is the Git merge driver for those two files: it keeps ours, and the next `library index` rebuilds them. No gate reads the knowledge directory. |
| `journal add --kind K --title T --body B [--link L]...`, `journal add --record STORY --title T --body B [--link L]...` | Write one post, `knowledge/journal/<date>-<slug>.md`, by create-new, then regenerate the index and feed (REQ-records-011). With `--record`, the post is an archived story's, for a backfill: its kind is `shipped` for a done story and `closed` otherwise, its date the story's `closed`, its `time` the story's latest Timeline time on that date (none otherwise), its `record` the story, and it links what the post at the move into done links, then each `--link`; it refuses a story that does not exist, and one a post's `record` already names, naming that post. `--kind` is `shipped`, `closed`, `intent`, `spec`, `broke` or `learned`; the body is at most 1,200 bytes; each `--link` is a project path Git tracks (a file, or a folder holding one; never under `.git/` or ignored) or an http(s) URL; a leading `./` is dropped, and a `#fragment` is kept on the link while the path before it is checked. The move into done writes a `shipped` post from the archived story and `session close` a `closed` one. Chough never rewrites or removes a post. |
| `skill status\|update [--host H]...` | Compare installed copies with the bundle; update only missing and known-outdated files. Without `--host`, both act on every host whose skill directory is installed; `update` refuses when none is. Hosts that share a directory get one entry. Both report a `legacy` list for an older `.claude/skills` copy, with each file's state, and never write it. |
| `skill remove-legacy [--host claude]` | Remove a legacy `.claude/skills` copy's files that are byte-identical to the current or a known earlier bundle, then empty directories; keeps edited and unknown files and reports `removed` and `kept`. Refuses, removing nothing, while the shared `.agents/skills` copy is missing. |
| `timing [--json] [--cycle current\|all] [--stage S] [--case ID] [--bug ID]` | Read-only report of how long stages, tests, bug fixes and human gates took, from existing records; Markdown-style text by default. Every duration is elapsed wall-clock time, not effort. See [Timing](#timing). |
| `history show` | The active session's Timeline, oldest first, each entry with its `id`, with what could not be read; an entry that is not a valid transition is refused. With no open session it shows the newest archived story's Timeline, marked `archived`. See [Transition history](#transition-history). |
| `correct <entry> --reason <text>` | Appends a `correction` entry linked to `<entry>` (`^t-xxxxxx` in the active session, or `<session>#^t-xxxxxx` for any open session or archived story) and changes no earlier byte; `next` and `status` show the corrected text, marked. See [Corrections](#corrections). |
| `session new\|use\|list\|close\|shelve`, `inbox add\|list\|fold` | Sessions in their own folders, setting aside an unreadable one, and the subagent inbox. Starting or switching work never touches another session. See [Sessions](#sessions). |
| `doctor [--dry-run\|--fix] [--reason R] [--close KEY]...` | Finds and fixes the Chough debt that stops a fresh worktree from entering Chough. See [Doctor](#doctor). |
| `gate list\|record` | Numbers the open human gates, each with a one-screen summary and its digest, and records one relayed reply for listed gates only, with `--verdict` or `--no-native` for a gate that drives a native step. See [Gate lists](#gate-lists). |
| `change new\|answer\|check\|status\|ship` | Runs SpecSync's change new, answer, check (with `--commit`) or status step in `--root` and passes its output, refusals and exit code through unchanged; `ship` checks first, runs the human's review when the gate drives one, ships, then closes the review gate with its relay and moves to done. See [SpecSync change steps](#specsync-change-steps). |

## Backfilled moves

`chough stage backfill --at <time> --commit <rev> --quote <text> [--note <text>]`
records a move to the next main stage that already happened, for example one made in a
subagent without Chough (REQ-cli-036). `--at` is the original time, as
`YYYY-MM-DDTHH:MM:SSZ` or Unix seconds; `--commit` is the commit the move rests on;
`--quote` is the human's or the record's words, kept verbatim.

- Every `advance` refusal applies, and the destination's native prerequisite is read
  now. A gate native state does not support now is refused, whatever the quote says.
- It refuses, recording nothing, a time after now or before the latest recorded
  Timeline entry (history stays in time order), an unreadable latest entry, a commit
  that does not resolve or is not an ancestor of HEAD, a blank quote, and release to
  done.
- It appends one `advance` entry with the given time, carrying the structured fields
  `recorded_at` (when it was written), `commit` (the full SHA) and `quote`
  (REQ-history-005). An entry without a backfill carries none of the three.
- It records no human approval, review, consent or awaiting-human gate. The quote is
  evidence only, and the output says so. Human gates are not backfilled: open and close
  them with `chough checkpoint` when they happen.
- `chough timing` lists every backfilled move under `backfilled` (and a "Backfilled
  moves" section in text) with both times, the commit and the quote, marks each segment
  a backfilled move starts or ends `"backfilled": true`, reports `backfilled` stage time
  per stage, and leaves a stage with backfilled time out of estimate samples.

## Batched advance

`chough stage advance --to <stage> [--note <text>]` moves forward through each main stage
from the current one up to `<stage>`, as consecutive advances (REQ-cli-052, DRIVE-9).

- `<stage>` is design, plan, build, test, accept or done, after the current stage. A
  target that is not ahead, a side or optional stage, or an unknown name is refused, and
  so is every case a plain `advance` refuses: a stage that is not active, a side stage,
  an open human gate that holds a stage the move would leave.
- Every step is checked before anything is recorded, as a plain `advance` would check it:
  the destination's native prerequisite and, on leaving test, open blocking bugs and an
  unlinked failure. All steps share one `specsync change status` read, made only when a
  step has a native prerequisite; intent to plan reads nothing.
- A refused step refuses the whole command, naming the step (`advance test -> accept
  refused (refused:blocking-bugs): ...; nothing was recorded`). No entry is recorded and
  the session file is byte-identical.
- Otherwise it appends one `advance` entry per step, each with its gate label and the
  note, all in one save, and reports them in `applied.steps`. `applied.from` is the
  starting stage, `applied.to` the target and `applied.gate` the last step's label.
- `--to done` runs the same steps at done as a plain advance into done (see
  [Moving into done](#moving-into-done)), on the same one read.

`stage advance` without `--to` is unchanged.

## Transition history

Every transition of a session is one entry of its `session.md`'s `## Timeline`
(REQ-history-002): `- <time> · <kind> · <from> → <to> · <text> ^t-xxxxxx`. `<time>` is
UTC to the second in ISO 8601; `<kind>` is `start`, `advance`, `iterate`, `back`,
`reset`, `pause`, `resume`, `skip`, `enter`, `return`, `gate-opened`, `gate-closed`,
`direct-native-write` or `correction`; `<text>` is the reason, gate or note; and
`^t-xxxxxx` is the entry ID, `t-` and 6 lowercase hex digits from fresh random bits,
unique in its file. `chough init --objective` and `chough session new` each write one
`start` entry first, from and to the stage the session starts in, with its objective,
or its key when none was given; `chough timing` reads it as the start of that stage's
first segment. The structured fields follow as indented `- key: value` lines
(`native_change`, `gate`, `revision`, `refresh`, a backfill's `recorded_at`, `commit`
and `quote`, a gate-opened entry's `holds` and `drives`, a closed gate's
`relayed_by`, the direct-write fields, a correction's `corrects`, and `by`). A field
with parts is a `- key:` line with one `- part: value` line per part under it, never
JSON; a list of parts goes one level further, as a direct write's files, each
`- <path>: <change>`:

```markdown
- 2026-11-12T16:39:05Z · gate-closed · plan → plan · Approve the scope of c1 ^t-92acd1
  - relayed_by:
    - decided_by: Leif
    - words: Yes, go ahead
    - where: coordinator chat, 2026-11-12T15:24Z
    - recorded_by: claude-code-t382
    - native: specsync change approve c1
    - actor: 0xLeif
  - by: claude-code (hc t382)
```

A value that holds a line break, starts or ends with a space, or starts with `"` is
written as a JSON string, so it reads back exactly. Entries written before 0.6.0 hold
these fields as one JSON value, and Chough still reads them (REQ-history-002). The `^t-`
suffix is an Obsidian block ID, so `session.md#^t-xxxxxx` links one entry.

The entry is written in the same atomic save as the checkpoint change it goes with, so
a refused save leaves neither. Chough only appends entries and never edits, reorders
or removes one; an entry ID is never reused in its file. `history show` lists the
active session's entries, or the newest archived story's when no session is open.
Gate entries also take slots in the recent-transition windows of `chough context` (20)
and `stage show` and a move's `--json` report (10).

An entry also records the acting agent, when known, as its `by` field:
`<agent> (<model>, hc <terminal>)`, leaving out what is not known
(REQ-records-007). `CHOUGH_ACTOR`,
`CHOUGH_ACTOR_MODEL` and `CHOUGH_HC_TERMINAL` set them; otherwise the agent is
`claude-code` when `CLAUDECODE` is set, and the terminal is `HC_TERMINAL_ID`. A value
that fails the leak check, or a terminal that is not a plain hc name, is left out with
a warning, and an unknown actor writes no field; neither fails the command. `chough
next` and `chough status` show the last actor. The actor is a claim an agent makes
about itself, never a human decision.

### Corrections

History is never rewritten (REQ-history-015). `chough correct ^t-xxxxxx --reason
"<what is true instead>"` appends one `correction` entry, `- <time> · correction ·
<stage> → <stage> · <statement> ^t-yyyyyy`, with `- corrects: ^t-xxxxxx`, to the
Timeline that holds the entry; every earlier byte of the file stays as it was. Name an
entry of another open session or an archived story as `<session>#^t-xxxxxx`, by
session ID or alias; a story gets the entry appended to its Timeline and nothing else.
A blank statement or one over 4,096 bytes, an unknown session or entry, and a
correction of a correction are refused with nothing written. `chough next` and
`chough status` show a corrected next action or decision as its newest statement,
marked `(corrected, N corrections)`, and `next` lists the 10 newest corrections, newest
first, with their total.

| Timeline limit | Bound |
|---|---|
| One entry with its fields | 65,536 bytes |
| One session's Timeline | 4,096 entries; the next append is refused naming `chough session close` and `chough session new` |
| Recent guidance (`next`) | The newest 128 entries |
| Timing | 4,096 entries and 16 MiB in one request |

An entry that is not a valid transition (an unknown stage or kind, a direct-write
field on another kind, or a relay with only one of `summary_digest` and `list`, or of
`native` and `actor`) is
refused by every reader, never shown as history. A line in the Timeline that is not an
entry, such as the closing line of an archived story, is prose and is skipped.

## Records and limits

Every Chough record is one UTF-8 Markdown file that starts with YAML front matter
between two `---` lines (REQ-records-001). That file is the record's only source of
truth: no JSON twin is written beside it, and no index, cache or generated note is read
in its place. Front matter uses a strict subset of YAML: `key: value` scalars, plain or
double-quoted with `\"`, `\\` and `\n` escapes; flow lists `[a, b]`; block lists of
scalars; and block lists of flat maps of scalars. Front matter outside that subset, a
duplicate key or a missing closing `---` is refused naming the file, the line and the
rule, and the command writes nothing.

Reading is tolerant: an unknown front matter key is kept byte for byte, in place, when
Chough rewrites the front matter; a missing optional key takes its default; CRLF reads
as LF. Chough owns only the front matter and the sections it appends to. A write
replaces the front matter, appends list items to the end of a named section, or creates
a missing named section at the end of the body, and does nothing else to an existing
body, so a person's edit to a Goal, a bug's Steps or an entry's body survives. Files
are written with LF line endings by an atomic replace under the writer lock, or by
create-new for a new record.

| Record | Bound |
|---|---|
| A record | 65,536 bytes |
| A session record or an archived story | 1,048,576 bytes |
| A migration receipt | 4,194,304 bytes |

A record over its bound is refused naming the bound, with every file byte-identical; a
session record over its bound names the recovery `chough session shelve <id>`
(REQ-cli-032, DRIVE-14). The session cap comes from measured use: at least 10 times the
largest per-change session and 4 times the largest single checkpoint seen on
2026-11-05. At the cap, `chough next` and `chough status` answer within 250 ms in a
release build on macOS and Linux, and the focused view stays within 16 KiB; a test
builds a session at exactly the cap and checks these budgets. Case files are bounded
to 256 KiB, captured output to 1 MiB, context to 16 KiB per section and 128 KiB in
total, sessions to 16 modules, the checkpoint `observations` list to 400 items, every
other checkpoint list to 200 items and side-stage nesting to 4 levels.

Native SpecSync spec files under `specs/` are the project's contracts, not Chough
records, and are read at any size; size is a composability hint, never a gate.
`next` searches a module's `context.md` for its Lessons heading at any size
(REQ-cli-034). These bounds apply only to Chough's own records under `.chough/` and to
what Chough displays (REQ-cli-042, LIFE-15). Native sources are never cut for a
decision: criteria are validated against the complete `hi export -- <FAMILY>` output
(up to 4 MiB), and SpecSync's own files, such as an archived `finalization.json` or
`change.md`, `.specsync/sdd.json` and the change records `timing` reads, are read
within SpecSync's 4 MiB change-artifact bound. A display section cut at its budget says
so and sends you to the complete source. Never shorten HI intent or criteria, specs or
SpecSync files to fit a Chough bound; a native source that exceeds one is a Chough bug
to report.

Native SpecSync status is parsed from its complete output up to 4 MiB and displayed
up to 16 KiB; `status` carries the parsed state and next action in `native_summary`
and `next` in `native`. Larger output is reported as `oversized` and refuses gated
transitions. `status` and `next` each run `specsync change status` at most once and
evaluate the gate on that response. Managed project paths are project-relative;
traversal and symlinks are rejected. Native tools and bound commands receive direct
arguments, never shell-evaluated text.

### No leaks in committed records

Before it writes a committed record, Chough checks every value it writes, in front
matter and body, for (REQ-records-006):

- an absolute path under a home, temporary or system root (`/Users/`, `/home/`,
  `/root/`, `/private/`, `/tmp/`, `/var/`, `/Volumes/`, `/mnt/`) or a Windows drive path,
  and a `~/` path;
- the local host name, and the local user name inside a path or as `<user>@<host>`;
- an email address;
- credentials: GitHub, GitLab, Slack, AWS and model-provider token forms, private key
  blocks, bearer and basic authorization values, JSON web tokens, and `password`,
  `secret`, `token` or `api_key` assignments with a value of 8 or more characters.

A match refuses the write as `refused:leak`, naming the field and the kind of match
(`home-path`, `system-path`, `windows-path`, `host-name`, `user-name`, `email`, `token`,
`private-key`, `authorization`, `jwt` or `credential`) and never the matched text, with
every file byte-identical. With the global `--redact`, each match is stored as
`<redacted:<kind>>` instead, and the write proceeds, saying how many values were
redacted. A configured human's display name (see [Human identity](#human-identity))
does not count as a user name. This build checks session values and Timeline entries,
relay values, corrections, bugs and journal posts, and `chough migrate` always redacts
what it writes.

`chough doctor` scans every file Git tracks under `.chough/` and the knowledge
directory the same way, except that a path under `/tmp/` or `/private/tmp/` in a case
definition (`.chough/cases/*.cases.toml`) is a deliberate test setting, such as a
case's own `XDG_DATA_HOME`, and is not reported; a home path, the local user or host
name, an email or a credential there still is (REQ-records-006).

Evidence entries, case runs and validation runs get the same check. The key failing
lines a case run keeps are redacted, never refused, and the entry says how many were;
the lines `evidence add --failing-lines` gives are checked like any other value (see
[Evidence entries](#evidence-entries)).

### Concurrent record writers

Every command that writes a Chough record holds one writer lock for the project while
it reads what it changes and saves it, so a concurrent writer applies its change to the
latest records and no update is lost (REQ-cli-040, DRIVE-11). That covers the commands
that change the checkpoint or the stage, `consent`, `pref`, `correct`, `session new`,
`use`, `close` and `shelve`, `inbox add` and `fold`, `bug add` and `update`,
`evidence add`, `cases record`, `journal add` and `doctor --fix`. A command that first
runs a case or a native process takes the lock only for its save and re-reads what it
changes under it. Read-only commands, such as `status`, `next`, `cases check` and
`timing`, take no lock.

The lock is the file `.chough/local/checkpoint.lock` inside the project, so every
process working on the project shares it whatever its temporary directory (`TMPDIR`),
one lock per worktree (#83). The operating system releases it when a command exits or
is killed, so a killed writer leaves no stale lock. A writer that takes the lock writes
its process ID into the file. A writer waits up to 30 seconds, then refuses as
`refused:checkpoint-busy` with nothing saved, naming the record it would write and the
holder: `held by process N`, or, when the named process is not running, that a process
it forked may still hold the lock through an inherited descriptor (#132). Rerun it when
the other command finishes. The lock file stays after use and is safe to delete while
no Chough command runs for the project. `CHOUGH_TRACE=1` adds a
`chough-trace: checkpoint-lock waited_ms=<n>` line to stderr for each writer.

A session write goes to `.session.md.<pid>.tmp` in the session folder and is renamed
over `session.md`; an inbox add writes a temporary file in the inbox folder and links
it into place. `.chough/sessions/.gitignore` ignores those temps. A writer holding the
lock removes a temp a killed writer left, just before it saves, and says so on stderr,
for example `chough: removed .chough/sessions/<id>/<temp> (<bytes> bytes), left by a
session write that was stopped before it saved; the session was not changed by it`.
The killed command's update was never saved. Other file names are left alone.

### Module, criterion and reference identities

A session, bug, evidence or case module is any module ID SpecSync accepts, so a dotted
module such as `chat.service` works: one path segment of at most 247 bytes (so
`<module>.spec.md` fits in 255), with no path separator, `<>:"|?*` or control
character, not `.` or `..`, no leading or trailing space, no trailing `.`, and no
Windows reserved device basename such as `CON` or `COM1`.
A criterion is any ID hi accepts, including sub-criteria such as `BOTS-1.a`: an
uppercase family, a hyphen, then levels that alternate a number without a leading
zero and lowercase letters, with no length limit beyond the record bounds. A plain
ID Chough accepted before, such as `UX-012`, is still accepted so existing records
stay readable, though hi does not trace it; a dotted ID hi rejects, such as `UX-1.A`,
is refused. Requirement and case references, such as `REQ-chat.service-1`, are
ASCII letters, digits, `-`, `_` and `.` up to 100 bytes with at least one letter or
digit, and are never used as paths. A requirement reference matches its heading as a
whole token, and a sentence-ending `.` after it still matches.

### Record names

A new session, evidence entry, bug, inbox entry, journal post and migration receipt is
named `<YYYY-MM-DD>-<slug>` (REQ-records-003). The date is the UTC date of its creation.
The slug is the lowercase ASCII letters and digits of its source (a session key, a case
ID or claim, a bug title, a post title), each run of other characters replaced by one
hyphen and the edge hyphens trimmed; with nothing left, the slug is the record kind. A
whole name is at most 80 bytes, the slug cut back at a hyphen. A name is never a
sequence number, a counter or a high-water mark, and it never changes after creation.

When a file of that name already exists in the same folder, the new record gets `-` and
6 lowercase hex digits from 24 fresh random bits, written by create-new. If that name
exists too, the write is refused with nothing written, and there is no second draw.
Random bits, for a clash suffix, a Timeline entry ID or a neutral external ID, come
from direct use of getrandom 0.4.3; no runtime, service or allocator is added.
Timestamps stay in their own fields and decide chronology, never identity.

A record's front matter gives its name as `id`. Every ID it had before 0.6.0 (a
`BUG-NNNN` or `BUG-<slug>-<hex>` bug, an `OBS-`, `VAL-` or `SNAP-` record, a session
key) is kept in its `aliases` list, and every command and link that takes a record ID
resolves it by `id` or by alias. An alias two records hold is refused by the command
that resolves it. SpecSync requirement IDs, case IDs, lesson numbering and spec
versions keep their own schemes.

New case definitions use curated `TC-<module>-<title-slug>` IDs. The title slug is
nonempty lowercase ASCII letters and digits in runs separated by single hyphens. The
module is the exact case-file module, and the complete ID stays within 100 bytes,
leaving `96 - module.len()` bytes for the title slug. IDs are globally unique across
all case modules. Deriving an ID from a title uses the same ASCII normalization as
other slugs, but later title edits do not rename a case. Exact legacy
`TC-<module>-NNNN` definitions remain valid with unchanged IDs, positions, serialized
fields and definition digests, without renumbering or migration.

An automated case run's evidence entry is named `<date>-<case slug>`, and
`evidence add` names its entry from its first `--case`, or from its summary when it
has none. A repeated validation run is one entry, `<date>-<case slug>-run`, that holds
every attempt, and each attempt is also its own case-run entry. Identical runs on one
day in one session get clash suffixes (REQ-cli-011, REQ-cli-023).
`cases run --observation-id`, `cases record --observation-id` and the positional ID of
`evidence add` stay optional. A supplied ID must be a date and slug name not in use for
a new write: a name already in use is refused with nothing written and no clash
suffix, and an old-style ID is refused with a remedy to omit it. 0.5.x observation and
validation IDs stay readable and resumable as aliases.

## Bugs

Each bug is one Markdown record, `.chough/bugs/<id>.md` (REQ-cli-012). Its front matter
gives `kind: bug`, `id`, `aliases`, `title`, `status`, `classification`, `scope`,
`impact`, `criteria`, `requirements`, `cases`, `evidence` (the linked evidence IDs),
`receipts`, `regression_baseline`, `fixture`, `environment`, `revision`, `found_by`
(the acting agent), `fixed_in`, `duplicate_of`, `related`, `created` and `updated`.
Its body gives `# <title>` and the paragraphs What happened, Steps (led by
`Trigger:` and the trigger, then each step), Expected, Why it matters and Fix, then
`## Log`, an append-only list with one dated entry per report,
note, evidence link and status change. A person's edits to the paragraphs stay, and
every earlier Log line stays byte-identical. Fix says `Not fixed yet.` until `bug
update --fix <text>` sets it, which the Log records too; marking a bug verified while
its Fix still says `Not fixed yet.` replaces that with a line naming the passing
observation and its revision.

`bug add` and `bug update` given `--observation` with an evidence entry of the active
session also add the bug to that session's checkpoint, as `checkpoint --bug` does, once,
so the session's story names the bugs found in it. An entry of another session, or a
receipt, leaves the active checkpoint unchanged.

Manual `bug add` and automatic failure capture both name a new bug `<date>-<title
slug>` by create-new (see [Record names](#record-names)). An update never renames the
record, even when its title has changed. A bug resolves by its name or any alias, so a
migrated bug answers to its `BUG-NNNN` or `BUG-<slug>-<hex>` alias in list, search,
show, update and report, and `duplicate_of` and `related` links resolve from alias to
name and back. A `.json` bug left in `.chough/bugs/` is refused as `refused:unmigrated`.
Chough 0.5.x cannot read 0.6.0 bug records or the date and slug names of any 0.6.0
record, so upgrade every agent before a project migrates; downgrading is not a
migration path.

Statuses are `open`, `investigating`, `fix-pending-verification`, `verified`,
`deferred`, `duplicate`, `wont-fix` and `fixed-upstream`. Four are closed: `verified`,
`duplicate`, `wont-fix` and `fixed-upstream`, and a closed bug never blocks.
`fixed-upstream` is for a bug in another tool's behavior, closed with `bug update
--status fixed-upstream --note "<the upstream fix, and who checked it where>"`; without
`--note` it is refused. It needs no Chough observation and never reads as verified by
Chough.

`bug add` and `bug update` refuse a scope value given as `--classification` with
``; `<value>` is a scope: use --scope <value>``, and a classification given as
`--scope` with the reverse hint; the exit code and the bug record are unchanged.
Scope is `blocking`, `in-scope` or `out-of-scope`; classification is `unclassified`,
`product-defect`, `regression`, `environment`, `requirement-gap` or `evidence-gap`.

## Human gates

`chough checkpoint --awaiting-human "<gate>" [--holds <stage>] [--drives
approve|review|none]` records a real gate still waiting for a person (REQ-cli-030).
`--holds` names the stage it holds: a main stage, release or a side stage, by default
the current stage. `--drives` names the native step a human's decision at it runs: by
default `approve` for a gate held at plan, `review` for one held at accept and `none`
otherwise. A gate that was not already open appends one `gate-opened` Timeline entry
with its `holds` and `drives`.

While an open gate holds the stage a move would leave, `chough stage advance` and
`chough stage skip` are refused as `refused:awaiting-human` before any native read; for
`stage advance --to`, every stage the move would leave counts. The refusal names at most
3 such gates, in recorded order, with the count of the rest, and says to record the
human's decision with `chough checkpoint --close-gate` or to withdraw a gate no human
decision closes with `chough checkpoint --clear-awaiting-human`. No flag or reason
bypasses it. A gate that holds a later stage does not refuse a move out of an earlier
one, so a pending release or publication decision does not stop build, test or accept
(#237). Iterate, back, reset, pause, resume and side-stage enter and return are never
refused by a gate. `chough next`, `chough status` and `chough gate list` list every
open gate with the stage it holds and the step it drives; while a gate holds the
current stage, the next action says not to decide it for the human and names
`--close-gate`.

`--clear-awaiting-human` withdraws every open gate and records no decision: use it for
gates no human decision closes. To withdraw some and keep the others, pass the gates
still waiting with `--awaiting-human` in the same call; they are logged closed and
opened again, so their waiting time restarts there.

### Relayed decisions at gates

When a human decides a gate, the agent records their decision on their behalf
(REQ-cli-050, FLOW-16):

```sh
chough checkpoint --close-gate "<exact gate text>" --decided-by "<who>" \
  --words "<their exact words>" --where "<where and when, and who relayed it>" \
  --recorded-by "<agent>" [--verdict pass|block] [--no-native]
```

It closes only that gate. Other open gates stay open and still refuse the moves they
hold.

When the gate drives a native step, Chough first runs that step as the decided human,
whose SpecSync actor comes from the identity file (REQ-records-008, see
[Human identity](#human-identity)):

- for a gate that drives approval, SpecSync's change approval of the session's native
  change, with the human's `specsync_actor` as the actor and a note that reads
  `Approved by <name> at the Chough gate "<gate>": "<words>" (<where>). Relayed and
  recorded by <agent> at <time> through chough checkpoint --close-gate; a relayed claim,
  not authenticated.`, naming `chough gate record <list>` instead when the decision came
  through a gate list. `gate record` reports, for each gate it closes, the native step
  that close ran as `native_step` (the command, the human, the SpecSync actor and the
  exit code, or why nothing ran);
- for a gate that drives review, SpecSync's change review, with the human's
  `specsync_actor` as the reviewer and the verdict given as `--verdict pass` or
  `--verdict block`, which is required there and is never read from the words.

Every check the save makes runs before the native step. When SpecSync refuses, Chough
passes its output and exit code through, the gate stays open and nothing is written.
When it succeeds, Chough closes the gate and adds the decision, which names the step
it ran. A gate that drives a native step needs the session's native change recorded:
otherwise it is refused as `refused:no-native-change`, naming
`chough checkpoint --native-change <id>`. Chough's own native step updates the
direct-write baseline, so it is not reported as a direct write.

`--no-native` closes a gate that drives a native step with the relay and runs nothing,
for a human decision that is neither an approval nor a review, such as a request for
changes. Chough never decides which case applies from the words. A gate that drives no
native step runs nothing. Every close, whatever its gate drives and with or without
`--no-native`, still names a configured human: `--decided-by` is checked against the
identity file before anything runs (see [Human identity](#human-identity)).

A next action written while a gate held the stage describes waiting on it. So when a
close (`--close-gate`, `gate record` or `change ship --close-gate`) leaves no open gate
holding the current stage, and the same call gives no `--next-action`, the checkpoint's
next action becomes ``Gate "<gate>" closed (<decision>); <step>.``, naming the step that
decision allows, never `chough next` (REQ-cli-021):

| Close | Next action |
|---|---|
| Native approval ran | ``Gate "<gate>" closed (approved); run `chough stage advance` to move on.`` |
| Gate drives no native step | ``Gate "<gate>" closed; run `chough stage advance` to move on.`` |
| Native review, `--verdict pass` | ``Gate "<gate>" closed (review pass); run `chough change ship <id>` to ship.`` |
| Native review, `--verdict block` | ``Gate "<gate>" closed (review block); run `chough stage back --to build --reason "<why>"` to address the review.`` |
| `--no-native` | ``Gate "<gate>" closed; record the next action the decision calls for with `chough checkpoint --next-action "<action>"`.`` |
| `change ship --close-gate`, after the ship | ``Gate "<gate>" closed (review pass); the change shipped, so run `chough stage advance --to done`.``, without "(review pass)" for a gate that drives approval |

A close that decided nothing beyond closing says "closed" once. A `--next-action` given
in the same call is kept, and a close that leaves another gate holding the stage changes
nothing. The decision itself stays in the Timeline.

Only a decision made at that gate is in the human's name. When an agent approves or
reviews on authority the human gave in advance, without the human answering a gate, it
records the native step as the acting agent account, `<agent>-<terminal>`, quoting the
human's earlier words and where they said them (REQ-cli-029, FLOW-24); the
[Approved-delta guard](#approved-delta-guard) shows that approval. If SpecSync or the
project's configuration refuses that actor, the agent stops and reports it, and never
records the step in the human's name instead.

The gate's `gate-closed` entry carries `relayed_by` with the four values under
`decided_by`, `words`, `where` and `recorded_by` (REQ-history-010), the shape agreed
with SpecSync's side for its approval and review records (spec-sync#633). When the
close drove a SpecSync step, `relayed_by` also gives `native`, the command it ran
(`specsync change approve` or `specsync change review`, then the change ID), and
`actor`, the SpecSync actor it ran as, both or neither. One decision
is added to Decisions and why for the handoff, ending with the native step when one
ran:

```text
<who> decided "<gate>": "<their words>" (<where>); recorded by <agent>; a relayed claim, not authenticated
```

The output carries `relayed_decision` with the gate, `relayed_by`, `provenance`: "a
relayed claim, not authenticated: it records who is said to have decided, not proof
that they did", and `native_step`, what ran or why nothing did: when nothing ran, `ran`
is null and `why` reads `no native step ran (the gate drives none)` or, for
`--no-native`, `no native approval ran (--no-native)` (or `review`). Chough does not
authenticate any of the four values. A relayed decision writes a SpecSync record only
through the native step above, and satisfies no stage gate: build, test, accept and
done still read native state. Every value passes the leak check before anything runs.

Each refusal runs nothing, saves nothing, writes no entry and names its recovery:

| Refusal | When | Recovery |
|---|---|---|
| `refused:relay-incomplete` | A value is missing or blank | Re-run with all four values; the message prints the command |
| `refused:relay-without-gate` | A relay value is given without `--close-gate` | `chough next` lists the open gates; add `--close-gate` with one |
| `refused:gate-not-open` | No open gate has that exact text | `chough next` lists the open gates; use one's exact text. When an open gate is close to the text given (at most a quarter of its characters differ once case and spacing are ignored), the refusal asks `Did you mean "<gate>"?` |
| `refused:relay-too-long` | The decision would exceed 4096 bytes | Shorten `--words` or `--where`, keeping the full text in a committed file named in `--where` |
| `refused:verdict-required` | The gate drives review and neither `--verdict` nor `--no-native` is given | Add `--verdict pass` or `--verdict block`, or `--no-native` |
| `refused:verdict-not-applicable` | `--verdict` is given for a gate that does not drive review | Leave `--verdict` out |
| `refused:no-native-change` | The gate drives a native step and the session records no native change | `chough checkpoint --native-change <id>`, or `--no-native` |
| `refused:human-identity-unknown` | `--decided-by` names no configured human, or two | Add them, or make the name unique, in the identity file the message names |
| `refused:identity-file-invalid` | The identity file cannot be parsed, or a human has a blank `name` or `specsync_actor` | Fix the file the message names |
| `refused:leak` | A value holds a local path, user or host name, email or credential | Remove it, or pass `--redact` |

`--close-gate` does not combine with `--clear-awaiting-human` or `--awaiting-human`.

### Human identity

Chough reads human identities from configuration, never from its source
(REQ-records-008). It reads `.chough/local/identity.toml` when that file exists, and
otherwise the user file `$XDG_CONFIG_HOME/chough/identity.toml` (by default
`~/.config/chough/identity.toml`). Each person is one `[[human]]`:

```toml
[[human]]
name = "<display name>"
aliases = ["<another name they go by>"]
specsync_actor = "<their SpecSync actor>"
```

`name` is how records show the person, `aliases` is optional, and `specsync_actor` is
the actor SpecSync records. At every gate close, through `checkpoint --close-gate`,
`gate record` or `change ship --close-gate`, whatever the gate drives and with or
without `--no-native`, `--decided-by` must name exactly one configured human, by
name or alias, ignoring case and surrounding space; otherwise the close is refused as
`refused:human-identity-unknown` before anything runs, naming both files and the keys
to add. Chough never guesses an identity and never falls back to an agent account. No
name is built in: a project or a person configures who decides. The relay stays an
unauthenticated claim (FLOW-16).

## Merging branches

Two branches that each record Chough work merge without a hand-resolved conflict in any
file Chough writes (REQ-cli-047, DRIVE-12):

| Files | How they merge |
|---|---|
| `.chough/sessions/<id>/` | One session folder per piece of work ([Sessions](#sessions)); its Timeline is append-only |
| `.chough/archive/sessions/<id>.md`, evidence and inbox entries, migration receipts | Files with date and slug names, written once by create-new |
| `.chough/cases/<module>.<part>.cases.toml` | Each change adds its cases in a file of its own |
| `CHANGELOG.md` | Git's `union` driver: both sides' lines are kept |
| `.chough/bugs/<id>.md` | `chough bug merge`, below |
| `knowledge/journal/<date>-<slug>.md` | Posts with unique names, written once |
| `knowledge/index.md`, `knowledge/journal/feed.xml` | `chough library merge` keeps ours, then `chough library index`; `library index --check` fails CI until it is rebuilt |

Two branches that create a record with the same date and slug in the same folder, the
same title on the same day, do conflict, and the merge shows it plainly.

A cases file is `<module>.cases.toml`, or `<module>.<part>.cases.toml`. Its `module`
field names the module, and `<part>` uses letters, digits, `-` and `_`. Case IDs stay
unique across all files. Add a change's cases in a file named after it, for example
`cli.no-conflicting-files.cases.toml`.

`chough bug merge <base> <ours> <theirs>` merges two edits of one bug into `ours`:
- a front matter field only one side changed takes that change;
- the list fields (`aliases`, `criteria`, `requirements`, `cases`, `evidence`,
  `receipts`, `related`) keep base's entries, then each side's additions, without
  duplicates;
- `updated` is the later one;
- the Log keeps base's lines, then each side's added lines in order, without
  duplicates;
- a body paragraph only one side changed takes that change.

The result must be a valid bug record, and it is written as bug commands write one. A
field or paragraph both sides changed differently, such as `status`, is left for a
human: `ours` is unchanged and the command exits nonzero.

`chough library index --check` writes nothing. It fails, naming each generated knowledge
file that is missing or differs from a rebuild, with CRLF read as LF, and it is a step of
the verify and ci lanes here.

`chough library merge <base> <ours> <theirs>` is the driver for `knowledge/index.md` and
`knowledge/journal/feed.xml`. It keeps ours byte for byte and exits 0, so a merge commit
never holds a union of two feeds that is not valid XML; the next `chough library index`
rebuilds both from the merged posts and sources.

Declare the drivers in `.gitattributes`:

```gitattributes
CHANGELOG.md merge=union
.chough/bugs/*.md merge=chough-bug
knowledge/index.md merge=chough-generated
knowledge/journal/feed.xml merge=chough-generated
```

`chough init` declares Chough's three lines for you. It appends each one that no line
declares after the last line, creates the file when it is absent, and changes no line
already there; the knowledge lines name `library-dir` when it is set.
In the knowledge directory, only the two generated files, which `library index --check`
compares, use the driver. Posts, lessons, snapshots and briefs are not generated, so
a conflicting edit to one still stops for a human.
`CHANGELOG.md` is this repository's own; a project declares it only if it keeps one.
Sessions need no merge driver: each is its own folder, and there is no checkpoint
merge.

Git does not clone config, so each clone needs the drivers too. `chough init` sets each
one in the clone's local config while Git has no command for it, so a command you set is
kept. In a fresh clone, `chough doctor` reports them missing as `merge_drivers`, and
`chough doctor --fix` sets them. By hand:

```sh
git config --local merge.chough-bug.driver "chough bug merge %O %A %B"
git config --local merge.chough-generated.driver "chough library merge %O %A %B"
```

Without them, Git falls back to its normal text merge for those files. Spec change logs
and `requirements.md` belong to SpecSync (CorvidLabs/spec-sync#862).

## Sessions

Each piece of work is a session folder (REQ-history-006, REQ-records-004):
`.chough/sessions/<id>/session.md` holds the checkpoint in its front matter and the
story in its body, with `evidence/` and `agent/` beside it. `<id>` is the UTC date and
the slug of the session's key; the key is kept as an alias.

The front matter gives `kind: session`, `id`, `aliases`, `objective`, `intent_scope`,
`modules`, `stage`, `stage_status`, `iteration`, `callers`, `native_change`, `pr`,
`criteria`, `requirements`, `cases`, `design`, `files`, `observations`, `bugs`,
`blockers`, `waiting_on_human` (each gate with `gate`, `holds`, `drives` and
`opened`), `run_consents` and `next`. The body is the story, in this order: `#
<objective>`, `## Goal`, `## Where it stands`, `## Decisions and why`, `## What was
verified`, `## Open threads` and `## Timeline`. Chough writes the Goal from the
objective once and never rewrites the Goal or Where it stands paragraphs. A decision
from `checkpoint --decision` is one item in Decisions and why, completed work one item
in Where it stands, and an open question one item in Open threads.
`--resolve-question` keeps the question's line and marks it ` (resolved)`; with
`--answer <text>` and exactly one `--resolve-question`, the line becomes
`<question> (resolved: <answer>)`. `--remove-completed` marks an item withdrawn on its
own line. `next` and `status` read decisions, completed work and open questions from
these sections, so no field repeats them.

Each observation the session references is one What was verified item that links its
entry:

```text
- [<claim>](evidence/<id>.md)
```

An entry another open session holds is linked as `../<session>/evidence/<id>.md`, and
an observation held only as a receipt, or by no record, stays a plain `- <id>` item. A
person's edit to an item's claim keeps the item.

- `chough session new <key> [--objective T] [--intent-scope S] [--module M]...`
  creates the folder by create-new and makes it active. A key another session holds as
  an alias is refused. It touches no other session: starting or switching work never
  archives, closes or changes another one, even a finished one (Q3, #207).
- `chough session use <id, alias or slug>` makes an open session active, by ID, alias
  or a bare slug exactly one open session has; an archived one is refused. It touches
  no other session either.
- `chough session list` reports each session's ID, aliases, stage, status (`archived`
  for a story), whether it is active, open or archived, and whether it is finished (an
  archived story always is), and names `chough doctor --fix` for each finished open
  session. A session that cannot be read, because it is
  over its bound or invalid, is listed with the reason and the recovery `chough session
  shelve <id>`, and does not stop `session list`, `session new`, `session use` or another
  session's `next`.
- `chough session close <key> --reason <text>` closes an open session for abandoned
  work, without a native read: it withdraws each open human gate of that session as
  `chough checkpoint --clear-awaiting-human` does, one `gate-closed` entry each and no
  decision, and names them under `withdrawn_gates`; it adds the decision `Closed
  without done: <reason>`, archives the session with outcome `closed-without-done` and
  writes a `closed` journal post. A blank reason, an archived or missing session and one
  with unfolded inbox entries are refused with nothing changed.
- `chough session shelve <id>` moves an unreadable or oversized session folder,
  unparsed and byte for byte, to `.chough/archive/shelved/<id>/`, with `shelved.md`
  inside it giving the folder's size, each file's size and SHA-256, the time and the
  reason, and clears the active pointer when it named it. Shelving never replaces a file, and
  no reader scans `.chough/archive/shelved/`.

A session is finished when it is at stage done, or when its native change is archived
on HEAD at any stage, as the archive's `finalization.json` records, read without a
native read (DRIVE-16). A session only ever closes itself: at its own move into done
(see [Moving into done](#moving-into-done)) or with `chough session close`. Nothing
reads, moves or rewrites another session's folder or story, so the archive keeps every
earlier story (KNOW-34, #165).

The active session is named by `.chough/local/active-session`. With no pointer, the
only open session is active, or else the only open session that is not finished; a
finished session and a folder whose story exists do not count toward several open
sessions. With none open, several and no pointer, or a pointer to a missing or archived
session, every command that needs a session refuses with nothing written and names the
command to run: `no open session; run chough session new <key>` when none is open.
Each command resolves its session once, so a `session use` in another process partway
through a command does not redirect its save. `chough init --objective <text>` creates
the first session as `session new` would, keyed by `--session <key>` or the
objective's slug, and `chough init` alone creates none.

### Doctor

`chough doctor` finds the Chough debt that stops a fresh worktree from entering Chough
and fixes it with Chough's own steps (REQ-cli-055, REQ-history-012, DRIVE-17). It
prints JSON: `mode` (`report`, `dry-run` or `fix`) and `findings`. Each finding has a
`kind`, a `target`, the `fix` command, whether the fix is a `decision`, and a `status`
of `found`, `fixed` or `skipped` (a skipped finding also has a `reason`). It makes no
native read, and never records a human decision, approval, review or consent.

| Kind | What it finds | Fix |
|---|---|---|
| `unmigrated` | 0.5.x records, such as `.chough/session.json` (REQ-records-010); the finding lists up to 20 of their `paths` and the `total` | Names `chough migrate`; doctor never migrates. The session findings below wait until the tree is migrated |
| `leak` | A file Git tracks under `.chough/` or the knowledge directory that matches the leak patterns of REQ-records-006, one finding per `line` and `match` (the kind), never the text | `chough doctor --fix --redact` replaces each match with `<redacted:<kind>>` in place; `--fix` alone skips it. Not a decision |
| `stale_pointer` | The active pointer names a missing or archived session | Remove the pointer |
| `finished_session` | An open session at done, or with its native change archived on HEAD | Archive it into its story; one with unfolded inbox entries is skipped, naming `chough inbox fold` |
| `unfinished_archive` | A session folder whose story exists and that still holds files | Remove each file whose bytes are committed at HEAD; otherwise skipped, naming the commit to make |
| `stale_gates` | A finished session with open awaiting-human gates, whose native change is archived | A decision: withdraw the gates as `checkpoint --clear-awaiting-human` does, recording no human decision, then archive the session |
| `unfinished_session` | An open, unfinished session named with `--close <key>` | A decision: `session close <key> --reason` |
| `timeline_rewritten` | A session or story whose Timeline at HEAD is not an exact prefix of its working Timeline (REQ-history-002); `entries` names the entries that differ | None: restore them from `git show HEAD:<file>` by hand |
| `alias_conflict` | An ID that is the `id` or an alias of two records of one kind (REQ-records-003); `records` names both | None: decide which record keeps it |
| `oversize` | A committed record, evidence entry or screenshot over its bound, or a session's screenshots over 2,097,152 bytes in all (REQ-records-001, -005, -013), with its `bytes` and `bound` | None |
| `unreadable` | A committed Chough record under `.chough/` that its reader refuses, such as front matter outside the subset, so `status` and `next` refuse the project (REQ-history-012); `error` names the file, the line and the rule. A record over its bound is `oversize` instead | None: fix the line by hand in a reviewed change, or set an open session aside with `chough session shelve <id>` |
| `records_in_verification` | `.specsync/sdd.json` exists and its `verification_ignored_paths` lacks `.chough/sessions/`, `.chough/archive/` or `.chough/bugs/` (REQ-records-014) | Never fixed: Chough never writes the policy. The finding adds `missing` (the entries to add), `specsync` (`6.1.2 and later`, the releases that honour the list) and `advice`: adding them is an ordinary reviewed change to `.specsync/sdd.json`. A broader entry such as `.chough/` gets the advice to list the three folders instead. `chough init` prints the same advice as `records_in_verification` |
| `merge_drivers` | In a Git work tree, a clone whose Git config has no command for `merge.chough-bug.driver` or `merge.chough-generated.driver`, as a fresh clone has none, or a `.gitattributes` that lacks one of Chough's merge lines (REQ-cli-047); one finding, with `missing_drivers` and `missing_lines` | `chough doctor --fix` sets the missing drivers in the clone's local config. It never writes `.gitattributes`, so a missing line keeps the finding `skipped`, naming the lines: `chough init` appends exactly those, then commit the file |
| `merge_driver_outdated` | The merge drivers are set, and the `chough` they run, found through `PATH` as Git finds it (Git runs `<program> --version` in its shell), is missing or reports a version older than the running Chough, so a merge would run older code on these records; one finding naming the drivers, `running_version`, `path_version` (null when missing) and `install`. A driver set to another program is not checked | None: Chough never installs itself. Install the running version where Git finds it, `cargo install chough --locked` as in the README's Install section, then run `chough doctor` again |

- **No flag or `--dry-run`:** reports every finding as `found` and changes nothing.
- **`--fix`:** applies the fixes in the order of the table. It applies a decision fix
  only with `--reason <text>`, and otherwise reports it as `skipped`. It leaves every
  session and story it does not fix byte-identical.
- **`--leaks`:** reports only the `leak` and `oversize` findings, changes nothing and
  exits 1 on any; it is the verify lane's `leaks` step, so CI fails on a leak or an
  oversized record.
- **Exit code:** 0 when nothing is left `found` or `skipped`, 1 otherwise, so CI can
  run `chough doctor` as a check. `records_in_verification` is never fixed, so it keeps
  the exit at 1 until the project lists the three folders. A missing merge line keeps
  `merge_drivers` at 1 the same way until `.gitattributes` declares it.

Run it when a fresh worktree refuses to enter Chough (for example "several open
sessions"), after merging main, and after the move into done to finish removing the
session folder.

### Subagent inbox

A subagent reports to a session without editing it:

```sh
chough inbox add --session <key> --from <name> --kind observation|decision|completed|question --text <text>
```

Each call creates one Markdown record,
`.chough/sessions/<id>/agent/inbox/<date>-<slug>.md`, giving `kind`, `id`, `session`,
`from`, `value`, `added` and, for an observation, `recorded`, and edits no session file
(REQ-history-009). It takes the writer lock from before it checks that the session is
open until it exits, so it never leaves an entry beside a session a concurrent switch
archives (#98). The entry is written whole to an ignored temporary file and then linked
into place, so an add stopped partway leaves no partial entry; the next save removes
the temporary file. For an observation, the text is its evidence ID (record the
evidence first). An ID with no recorded evidence is still added, because a subagent may
report before this tree has the record, but `inbox add` warns on stderr naming it, and
`inbox list` marks each observation entry `recorded: true` or `false` (#84). An entry
carries no approval, review, consent, gate or stage field, and one holding such a field
is refused, so a subagent cannot close a gate or move a stage through it; stage gates
still come from native SpecSync state. A session holds at most 200 unfolded entries.

`chough inbox list` shows the active session's unfolded entries. `chough inbox fold`
adds them, in ID order, to the active session's observations, Decisions and why, Where
it stands or Open threads, skipping values already there, saves the session within its
bounds, then moves each entry to `agent/inbox/folded/`. Before it saves, it checks
every entry the way `inbox add` does (a valid `from`, text that is not blank and within
4096 bytes, an evidence ID for an observation) and that no `folded/` target exists; any
failure refuses with the session and the inbox unchanged. A rerun after an interrupted
fold adds nothing twice.

## Moving into done

When `chough stage advance` (with or without `--to done`) moves the active session into
done and its save succeeds, the same command goes on, with no further native read
(REQ-history-006, REQ-records-011, REQ-cli-043, REQ-cli-052):

1. It archives the session. It writes `.chough/archive/sessions/<id>.md` by create-new:
   the session's front matter with `stage: done`, `stage_status: archived` and no
   `next`, since the session's next action no longer applies, `closed` (the UTC date),
   `outcome` (`done`) and `receipts`, then the story as written, the Timeline and a
   closing line `Closed <date>: <outcome>.` It reads the story back, then removes each
   file of the session folder whose exact bytes are committed at HEAD
   (REQ-records-012); any other file stays, and the folder counts as archived because
   its story exists. It clears the active pointer. The move's `--json` report gives it
   as `applied.session_archive`, and its text summary's `Closeout:` line names the
   status of this and the next two steps: `archived` with the `id`, the `story` and the files
   `kept`. A session with unfolded inbox entries stays open as `kept_open` with
   `inbox_unfolded`, and one for which release is authorized stays open as `kept_open`
   with that `reason`, to archive itself at the move from release into done.
2. When the session was archived, it writes the journal's `shipped` post by create-new,
   reported as `applied.journal_post`, with links to the PR, the archived story and the
   change's archive folder. Its title is the session's `title` and its body the
   session's `summary`, one paragraph per line, which the agent sets before the move
   with `chough checkpoint --title "<short human title>" --summary "<what changed and
   why, what was verified with its final numbers, what is next>"`. Without them, the
   title is the objective's first clause (cut at a word within 72 bytes) and the body
   the story's Goal, then `Why:` with its first decision, `Verified:` with its newest
   verified entry and `Next:` with its first open thread, within 1,200 bytes.
3. It regenerates `index.md` and `journal/feed.xml`, reported as
   `applied.librarian_index` with status `refreshed` and the index path, `skipped`
   with the librarian off, or `refused` with its reason, so `chough library index
   --check` passes right after done (KNOW-4).

None of these undoes the move into done. Nothing is trimmed and no other session or
story is read, moved or rewritten. Then `chough next` and `chough status` refuse with
"no open session" and name `chough session new <key>` until a session is opened
(DRIVE-13).

The finish takes two commits. First commit the story, the journal post and the
regenerated index and feed. The session's own `session.md` changed in the move, so its
bytes were not yet committed and it stayed. Then run `chough doctor --fix`, which
removes the session folder now that its bytes are committed, and commit that.

Each evidence entry the session holds becomes one map in the story's `receipts`, in
observation-time order, oldest first, with its `id`, `aliases` (one value, comma and
space separated), `claim`, `result`, `observed`, `command`, `revision`, `environment`,
`content`, `fingerprint`, `raw_log`, `case`, `case_digest`, `duration_seconds`,
`screenshot` and `actor` when present. What was verified gives the latest receipt of
each case and every receipt bound to no case, one line each:

```text
- <claim> — `<command>` <first 12 fingerprint digits> [screenshot](<session>/<file>) ^<evidence id>
```

The command appears only when the claim does not already hold it, as a case run's
claim does, and the digits and the screenshot link only when the receipt has them; a
receipt without a claim shows its ID. The line replaces the item that linked the entry
while the session was open, since that folder leaves with the session; an item a
person wrote that names the entry is kept, and that entry gets no second line. A
case's earlier runs get no line, and the item that linked one goes; instead one line,
first in the section, counts them, as in `2 earlier runs of the cases below are kept
in this story's receipts, not listed here.`, and `receipts` keeps every run.
Screenshots move beside the story. Evidence and bug links into an archived session
resolve to its story's receipts.

## Migration

`chough migrate` reads a project's 0.5.x records, writes nothing, and prints the plan
(REQ-records-009). A person consents to it, so it prints the plan as text by default:
each 0.5.x file with its SHA-256 and what it becomes, each file it writes, the
redactions, removals and kept files, what is not carried, the aliases and follow-ups,
any refusal the apply would give now, then the plan digest and the apply command.
`chough migrate --json` prints the same plan and digest as JSON, with these keys:

- `sources`: each 0.5.x file with its SHA-256;
- `becomes`: each 0.5.x file with the 0.6.0 files it becomes;
- `writes`: each 0.6.0 file it would write;
- `redactions`: every value it would redact, by file, field and kind;
- `removed`: every file it would remove without carrying it, with why;
- `kept`: each open bug's fixture under `.chough/artifacts/`, kept byte for byte, with
  the bug that keeps it;
- `kept_with_leaks`: files it keeps byte for byte (an open bug's fixture, a `library/`
  snapshot or brief) that hold a leak, listed for a person to fix and never changed;
- `gaps`: what carried records held that their entries or receipts do not, one counted
  line per field, which the receipt lists under Not carried;
- `unaccounted`, `aliases` (old ID to new), `follow_ups` and `git_history`;
- `plan_digest`, the plan's SHA-256, and `apply`, the exact apply command;
- `apply_refused`, only while a native change in the project is not archived: the
  refusal `refused:change-open` that `--apply` would give, each open change by name
  and why. It is not part of the digest.

On a tree with no 0.5.x record, migrated already or never 0.5.x, the preview says
there is nothing to migrate (only `nothing_to_do` under `--json`), with no plan digest
and no apply command, and writes nothing. The apply prints JSON either way.

`chough migrate --apply <digest> --consent "<who consented, their words and where>"`
applies exactly that plan. Run the preview and the apply on the same machine and
account. The plan's redactions depend on the local host and user name, so its digest
differs between machines and accounts: a plan previewed in CI or by another user does
not apply here and is refused as `refused:plan-digest`. The consent is recorded in the
receipt as a relayed claim;
a standing `migration` preference (`chough pref set migration standing --source
"<their words>"`) gives it instead. Chough never infers consent from the digest or
another record. Before writing anything, `--apply` refuses:

| Refusal | When |
|---|---|
| `refused:plan-digest` | The plan it builds now has another digest; preview again |
| `refused:unaccounted` | A 0.5.x file has no place in the plan |
| `refused:change-open` | A native change in the project is not archived, named; the upgrade never runs while a change is open |
| `refused:no-consent` | Neither `--consent` nor a standing `migration` preference gives the human's consent |
| `refused:uncommitted` | A 0.5.x file it would remove differs from its bytes at HEAD or is untracked, named, so Git holds every byte it removes |
| `refused:target-exists` | A file it would write already exists with other bytes |

It then writes the 0.6.0 tree by create-new only, reads each file back and compares
its digest, and writes the receipt `.chough/archive/migrations/<date>-from-0-5.md`:
every file before and after with its path, size and SHA-256, the redactions by file,
field and kind, every removed file with why and the commit that holds it, and the
old-to-new ID map, with `none` under any section that has nothing to list. Every
`session.md` it writes ends with exactly one newline. Only then does it remove the 0.5.x files, each only while its bytes
still match the plan. It never weakens a SpecSync archive: before removing anything it
reads the native validity of every archived change whose acceptance manifest lists a
file it would remove, at most 8 reads at once, reads them again after the removal, and
if any archive that read valid no longer does, restores every removed file byte for
byte and refuses as `refused:archive-weakened`, naming those archives. A file an
interrupted apply wrote with the planned bytes counts as done on the next apply, and a
second apply on a migrated tree reports nothing to do.

The apply prints `applied`, `plan_digest`, the `receipt` path, `written`, `removed`,
`archives_checked` (the archives whose validity it read), `unstaged` and `git_history`.
`unstaged` names each 0.5.x file whose name differs from a file it writes only by case,
such as `knowledge/INDEX.md` and `knowledge/index.md`: on a case-insensitive file system
those are one file, so it drops the old name from Git's index (never from the work
tree), and the next `git add` records the rename.

What becomes what:

- each open session, legacy `.chough/session.json` included, becomes a session folder
  whose Timeline holds, in order, every transition of its selected history lineage, the
  legacy log's entries going to the session that adopted them;
- each archived session, and each session rolled into `knowledge/attachments/`, becomes
  an archived story;
- each bug becomes a Markdown bug, its old ID kept as an alias;
- inbox entries move to their session's `agent/inbox/`;
- an open gate holds the stage its session was at;
- case definitions and `.chough/preferences.json` stay byte-identical.

Each observation an open session references becomes an evidence entry of that session;
when several open sessions reference it, the one whose new ID sorts first holds it. Each
one only an archived session or a bug references becomes a receipt of that story or
bug, and others are removed. A repeated validation run becomes one entry. Entries are
named as `evidence add` names them, from the observed date and the first case or the
claim, or the case and `run`; a clash takes the first 6 hexadecimal digits of the
SHA-256 of the old ID, so the preview and the apply name everything alike, and every old
ID is an alias. A raw log under `.chough/artifacts/` is removed and its recorded digest
becomes the fingerprint. A PNG or JPEG artifact of a carried observation is stripped of
its metadata and kept beside its session or story within the bounds of
[key screenshots](#evidence-entries); an artifact elsewhere in the project stays where
it is, its path and digest on the entry. The generated and copied
vault content of earlier releases is removed after it is listed, and so are
`.chough/transitions.jsonl`, `.chough/history/`, `.chough/pruned.jsonl`,
`.chough/session.v1.json`, `.chough/artifacts/` other than files an open bug names, and
shelved 0.5.x sessions. An ID whose record was removed without being carried resolves
as `removed`, naming the receipt, never as current or passed.

Every file the migration writes passes the leak check with redaction, so no home
folder path, user name or credential survives in a rewritten record. Git history is not
rewritten, and the receipt says so: earlier commits keep what they held. Upgrade every
agent first: Chough 0.5.x finds no checkpoint in a migrated tree. The migration plan and
receipt also list the `records_in_verification` advice as a follow-up. The receipt holds
every follow-up under one `## Follow-up` heading, one paragraph each.

## SpecSync change steps

`chough change` runs SpecSync's agent steps for you, so an agent can drive a change
through `chough` alone (REQ-cli-057, REQ-cli-059, DRIVE-20):

| Command | Runs |
|---|---|
| `chough change new <description> [--kind K] [--spec S]... [--path P]... [--no-spec-change --rationale R]` | `specsync change new <description> [--kind K] [--spec S]... [--path P]... [--no-spec-change --rationale R] --json` |
| `chough change answer <id> <question> <answer>` | `specsync change answer <id> <question> <answer> --json` |
| `chough change check <id>` | `specsync change check <id> --commit` |
| `chough change check <id> --no-commit` | `specsync change check <id>` |
| `chough change status [<id>]` | `specsync change status [<id>] --json` |

Each runs in the `--root` directory with the `specsync` on `PATH`, the one Chough's
native reads use. SpecSync's standard output, standard error and exit code pass through
byte for byte, including its refusals and the recovery commands they name. Chough adds
nothing and writes nothing under `.chough/`.

- `status` is a read and keeps the native read bound (120 s;
  `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets 1 to 3600). Past the bound, SpecSync is stopped,
  nothing is recorded, and the refusal names the same command with a longer bound. An
  invalid bound is refused before SpecSync runs, naming the command with the default.
- `new`, `answer` and `check` write, so the read bound never stops them. A hung write
  waits until you stop Chough. Stopping Chough with SIGINT, SIGTERM or SIGHUP stops
  SpecSync and its descendants (FLOW-14).
- When `specsync` cannot be started, the step refuses with exit 1 and runs nothing,
  naming `cargo install --locked specsync@6.3.0` and the step to run again.

A human's approval and review run through their gate (see
[Relayed decisions at gates](#relayed-decisions-at-gates)).
`chough change check <id> --no-commit` runs SpecSync's check without `--commit`, the
check the workflow's accept step runs first to materialize requirement text; like
every `chough change` step, it is never recorded as a direct lifecycle write.

### A new project and its first change

1. Write the intent as an HI file and check it with `hi check`. A minimal one, such as
   `hi/checkout.md`, names its family, says what people want, and lists each criterion
   with an ID that never moves:

   ```markdown
   ---
   hi: 1
   families: [CHECKOUT]
   ---

   # Checkout

   ## Intent

   Shoppers should not pay shipping on larger orders.

   ## Criteria

   - **CHECKOUT-1**  An order of fifty dollars or more ships free.
   ```

2. Create each module's contract: `specsync init`, then `specsync scaffold <module>`,
   and fill in its requirements and testing. A baseline
   `specs/<module>/requirements.md` gives each requirement a `### REQ-<module>-<n>`
   heading, the sentence that says what SHALL hold, and its acceptance criteria. That
   is the ID a case's `requirements` names and the block a delta's `## MODIFIED`
   replaces:

   ```markdown
   ---
   spec: shipping.spec.md
   ---

   ### REQ-shipping-001

   Shipping cost. An order under fifty dollars SHALL cost five dollars to ship.

   Acceptance Criteria
   - An order of 49.99 costs five dollars to ship.
   ```

3. Turn SpecSync's change gates on with `specsync change adopt`. `specsync init` leaves
   them off, and until you adopt, stage moves pass as `ungated:sdd-disabled` and no
   native lifecycle gates them.
4. `chough init --objective "<outcome>" --intent-scope <FAMILY> --module <module>`, then
   follow `chough next`. Add the human to the identity file before the first gate,
   since every gate they decide names them (see [Human identity](#human-identity)).
5. At plan, `chough change new "<description>" --spec <module>`, then `chough change
   answer` for each question SpecSync asks, and one delta file per affected spec.

A delta, `.specsync/changes/<id>/deltas/<module>.md`, says how the change's contract
differs: each requirement it adds under `## ADDED`, and each one it changes, with its
whole new text and not a diff, under `## MODIFIED`:

```markdown
## ADDED

### REQUIREMENT REQ-shipping-004

Free shipping. An order of fifty dollars or more SHALL ship free.

Acceptance Criteria
- An order of exactly fifty dollars ships free; one of 49.99 does not.

## MODIFIED

### REQUIREMENT REQ-shipping-002

<the requirement as it will read once the change is archived>

Acceptance Criteria
- <a check that shows it holds>
```

The deltas must name exactly the specs the change affects, and a spec is affected when
it owns a file the change touches. SpecSync 6.3.0 can call a draft ready for approval
before it has any delta, so check `deltas/` before asking the human to approve the
scope.

The change's own `.specsync/changes/<id>/testing.md` must name every requirement its
deltas add or modify before `specsync change check` (and so `chough change check`)
verifies it; until then it refuses with "no requirement evidence for <ID>; add a row
for it to the `## Requirement evidence` table". SpecSync 6.3.0 looks only for the ID in
that file or in a test file, so give each requirement a row naming the test or check
that shows it:

```markdown
## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-shipping-004 | `tests/shipping.rs`: an order of exactly fifty dollars ships free |
```

### Ship

`chough change ship <id> [--close-gate <gate> --decided-by <who> --words <words>
--where <where> --recorded-by <agent>]` is the closeout after the human's review
(REQ-cli-059). It runs in these steps:

1. Before anything runs, with nothing written, it refuses unless the active session's
   native change is `<id>` and its stage is accept, and unless every open human gate is
   the one `--close-gate` names. The relay values are checked as `chough checkpoint
   --close-gate` checks them. Each refusal names the command that recovers: the full
   ship command with the relay, `chough stage advance --to accept`, `chough checkpoint
   --native-change <id>`, or `chough checkpoint --close-gate` for another open gate.
2. When the named gate drives review and the change has no current scoped review, it
   first runs SpecSync's change review as the decided human with verdict `pass`,
   since a ship is run only on acceptance (REQ-records-008). The identity is resolved
   before SpecSync runs (`refused:human-identity-unknown` otherwise), and a refused
   review stops here with SpecSync's output and exit code and nothing written.
3. It runs `specsync change ship <id>` in `--root` with no time bound, and passes
   SpecSync's output through. A SpecSync refusal ends here with its exit code, and
   nothing is written.
4. After a successful ship, it closes the gate as `chough checkpoint --close-gate` would,
   with the relay (a relayed claim), and then moves the session as `chough stage advance
   --to done` would, printing each report after SpecSync's output. If either is
   refused, it exits 1, says the change is shipped and names the remaining commands.

The gate closes after the ship, so even a project that keeps Chough's records in
SpecSync's verification writes nothing under `.chough/` between `change check
--commit` and finalize (see `chough_writes` in [The next view](#the-next-view)).

`--list <list> --gate <n>` may stand in for `--close-gate <gate>`. The gate is then
number `<n>` of a `chough gate list` list, and its summary must still have its listed
digest, or the ship refuses before anything runs, naming `chough gate list`. The closed
gate's relay keeps that digest and the list (REQ-history-014).

## Gate lists

`chough gate list [--json]` numbers every open human gate of every open session from 1,
with the stage it holds and the native step it drives (REQ-cli-061, FLOW-19, FLOW-20).
Each gate gets a one-screen summary of at most 24 lines and 4,096 bytes, with:

- what changed: the files, insertions and deletions since the change's own
  `base_commit` (from its `state.json`), so an earlier change on the same branch is not
  counted again, leaving out `.chough/`, the knowledge directory and SpecSync's
  lifecycle bookkeeping, and naming the five files with the most changed lines among the
  rest (#190). At a gate that drives approval, the change's own Markdown files under
  `.specsync/changes/<id>/` (its `change.md`, deltas, plan and other artifacts) are what
  the human approves, so they count and are named first; its `state.json` and
  `approvals.json` stay left out. It counts from the merge base of HEAD with the default branch
  (`origin/HEAD`, else `main`) instead when that merge base is not an ancestor of the
  `base_commit`, meaning default-branch work came in after the change began, and when
  the change records no `base_commit`. A merge base that is HEAD itself, as on the
  default branch, is not used; with no usable base, the summary says the base is
  unknown;
- the tests: how many cases passed, failed and were not run, by the latest observation
  of each case that links a requirement the change's deltas add or modify (or that the
  session's observations name), with the failing case IDs;
- the risks: the first 8 lines of the change's `plan.md` section "Risks", or "none
  recorded";
- a diff link: a GitHub compare URL from the base to HEAD when `origin` is on GitHub,
  otherwise `git diff <base>..<HEAD>`.

Each summary's digest is the SHA-256 of its text. The list is saved untracked as
`.chough/local/gate-lists/<list>.json`, where `<list>` is the first 12 hex digits of its
SHA-256. Listing makes no native read and writes no tracked file. The text list ends
with how to reply: one `chough gate record <list> --gates <n>[,<n>…]` reply, with the
relay, for the gates whose change is not verifying, and for each gate whose change's
`state.json` state is `verifying`, `chough change ship <id> --list <list> --gate <n>`
with the relay, since such a gate closes through the ship.

`chough gate record <list> --gates <n>[,<n>…] --decided-by <who> --words <words>
--where <where> --recorded-by <agent> [--verdict pass|block] [--no-native]` is one reply
for several listed gates. It closes each one as `chough checkpoint --close-gate` would,
and each relay keeps the same words, its own `summary_digest` and the `list`. A named
gate that drives a native step runs it as the decided human before any gate is closed.
It closes all of them or none. Before writing anything, it refuses and names the
recovery for:

- an unknown list (`refused:unknown-list`), a number not on it (`refused:not-listed`),
  a gate no longer open (`refused:gate-not-open`), or a summary that changed since it
  was listed (`refused:summary-changed`): run `chough gate list`;
- a gate of another open session (`refused:other-session`): `chough session use <key>`;
- a gate whose change is verifying (`refused:verifying`): such a gate closes through
  the ship, which runs the review as the human and closes the gate after the native
  ship, so the refusal names `chough change ship <id> --list <list> --gate <n>` with
  the relay. Chough's records may still be written meanwhile where
  `verification_ignored_paths` lists them;
- more than one named gate that drives a native step (`refused:several-native-steps`):
  record them one at a time.

The oldest Chough that reads a relay holding `summary_digest` and `list` is 0.6.0.

## Direct lifecycle writes

Agents may still run `specsync` lifecycle commands directly, and they still work. Chough
notes each such write in the session's Timeline, so the need for it can be seen and
removed (DRIVE-21, REQ-cli-060, REQ-history-013).

- **What is watched:** for the active session's native change, the lifecycle records
  SpecSync writes in `.specsync/changes/<id>/`. These are `state.json`,
  `approvals.json`, `review.json`, `review-attempts.json`, `verification.json` and
  `verification-attempts.json`. Authored artifacts such as `context.md`, the plan and
  the deltas are not watched.
- **The baseline:** `.chough/local/native-writes/<id>.json` holds each record's SHA-256
  and the change's state. It is untracked, and written only once `.chough/local/` is
  ignored. `chough change new`, `answer`, `check` and `ship`, and a native step a
  human's gate decision runs, update it after their SpecSync step, so Chough's own steps
  are never flagged. A difference they find before their step stays pending.
- **Recording:** the next successful `chough stage` move, or gate close with a human's
  decision (`checkpoint --close-gate`, `gate record` or `change ship --close-gate`),
  records each pending write and any current difference as a `direct-native-write`
  entry before its own entry, so the story reads in the order the writes happened. The
  entry has `from` and `to` equal to the current stage and the fields `change`, one
  `file` line per changed file (its path under `.specsync/changes/<id>/` and `added`,
  `changed` or `removed`), `before_state` and `after_state` (the change's state from
  `state.json`, or `none` when it is absent). With no baseline, the first move or close
  only sets one. Nothing is recorded outside a stage move or a gate close, and
  withdrawing a gate with `--clear-awaiting-human` records nothing.
- **Views:** `chough next` prints `Direct lifecycle writes: N not yet recorded (…)` and
  carries `direct_writes`, and `chough status` adds `direct_writes` (`baseline`,
  `pending`). Neither writes anything.

## Execution bounds

Case bindings and native reads (`specsync change status`, `hi export`) run through
one bounded runner. Both streams are read concurrently; each keeps at most its cap
while reading and keeps draining beyond it, counting every byte.

| Run | Stdout kept | Stderr kept | Deadline |
|---|---|---|---|
| Case binding | 1 MiB | 1 MiB | `timeout_seconds` (default 600 s) |
| Native read | 4 MiB + 1 byte (the parse bound) | 16 KiB | 120 s; `CHOUGH_NATIVE_TIMEOUT_SECONDS` sets 1 to 3600 |

At the deadline, or when output stays open more than 2 s after the command exits
(cut short at the deadline, but at least 200 ms after the exit), Chough stops the
command and its descendants: on Unix the process group the command leads. It then
waits at most 1 s for the output to close and abandons any reader still blocked, so
the run returns on time and its capture is reported incomplete. Native Windows is not
supported (FLOW-17); the code that stops a process tree there with `taskkill /T /F`
remains but is no longer built or tested.

Stopping Chough stops what it started (FLOW-14). On Unix, when Chough gets SIGINT
(Ctrl-C), SIGTERM (a plain `kill`) or SIGHUP while a case binding, a native read or
one of its own Git commands runs, it stops that command's process group and ends by
the same signal. On Linux it also stops every process below that command by parent
links, including one that left the group with `setsid`, and, because the `chough`
binary is the child subreaper of what it starts, one whose parent exited after starting
it (a double fork) (#81). Because Chough is that subreaper for the whole run, a stop
for a deadline or a held pipe also stops an orphan re-parented to Chough earlier in the
same run, for example a detached `git gc --auto` left by one of its Git commands. It
keeps watching until the command's output closes: when a descendant still holds the
output of one of Chough's own Git commands after Git exits, a signal still stops the
group, and without one Chough stops the group 2 seconds after Git exits, says so on
stderr (`chough: stopped a process that kept Git's output open 2 s after Git exited`),
and goes on with Git's output. When `git rev-parse HEAD` fails for a reason other than
no repository or a HEAD with no commit, Chough says so on stderr (`chough: could not
read the Git revision (...)`) and saves the record without a revision. During `cases
run` the stop handling lasts through the save: a stop before the save begins records
nothing (no evidence, log or checkpoint change), and a stop once it has begun lets the
log and the evidence both be written first, so a run is never half recorded. A signal
Chough inherited as ignored (for example under `nohup`) or as blocked keeps that
disposition. A hard `kill -9` cannot be caught, so a command may outlive it.

A case run's captured output keeps both streams within 1 MiB in total and ends with a
note for each stream that did not fit: `[truncated: stdout wrote N bytes; M
discarded]`. Case limitations name a timeout (`timed out after N s; the command and
its descendants were stopped`), descendants that held output after exit (`... and were
stopped` or `... and could not be stopped`) and `output capture incomplete`. A run
whose capture is incomplete is never `passed`.

Native section states are `observed`; `cached` for a reused archived report as described
below; `failed` for a non-zero exit or output that stayed open and could not be read
completely; `unavailable` when the command cannot start or
`CHOUGH_NATIVE_TIMEOUT_SECONDS` is not a whole number from 1 to 3600 (the command is
then not run); `timed-out`; and, for JSON status, `oversized` and `invalid-output`. A
`failed` section's content starts with how the command ended, in plain words: `exit
<code>`, `killed by signal <number>`, `timed out` or `no exit status`, then its bounded
output (#87). Each state other than `observed` or `cached` leaves lifecycle state
unknown, and gated transitions refuse it as `refused:native-unknown`. Without a SpecSync
configuration, status reports `not-configured` and transitions are not natively gated.
With SpecSync configured and SDD enabled but no native change recorded, gated
transitions run no status command and refuse as `refused:no-native-change`, while
`status` and `next` still show the unscoped `specsync change status --json`. Record
the change with `chough checkpoint --native-change <id>`.

### Authenticated archived report reuse

For a checkpoint whose native change is archived, `next` and `status` can reuse a
previous `specsync change status <id> --json` response only when SpecSync reported the
same archived change with `authenticated-history`. The report's native read state is
`cached`. Chough does not parse an archive and decide validity itself, and active-change
responses are never cached.

Entries live at `<git common dir>/chough/archive-auth/<sha256 of change id>.json`,
outside tracked inputs and Chough's records. This is trusted local state like the rest
of `.git`, not human approval or a second lifecycle ledger. Delete that directory to
force live reads; doing so never changes native state.

Reuse requires the same archive bytes and Git tree/carrying history, resolved SpecSync
binary bytes and recorded version, native policy/configuration/baselines, and a default
full-history setup. Any shallow boundary, replace ref, graft or alternate-object
configuration disables reuse and priming. Missing, malformed, oversized, symlinked,
mismatched or unproven entries fall back to one bounded live status read. A live response
cannot supply the cached marker. Cache identity is checked again after a slow live read
before anything is recorded.

Only `next` and `status` use an entry. Stage transitions, timing, native verification
and every other lifecycle operation always call SpecSync live, even after a warm
report, with the same gate predicates, deadlines and output bounds.

Only a completed authenticated response primes an entry. A timed-out prime writes no
entry, keeps the existing 120-second deadline and descendant kill, and produces no warm
result: the next invocation retries a bounded cold read. Under sufficient load, cold
SpecSync status can exceed that deadline repeatedly; this change does not hide or extend
it. The approved warm budget is measured only after a successful prime: on the
121-archive macOS arm64 Chough repository, ten release runs of each command must have
p95 at or below 2000 ms and every run at or below 3000 ms (DRIVE-15).

Snapshot hashing keeps its exact legacy and content digest streams but computes the two
independent ordered passes concurrently. There is no persistent evidence-fingerprint
cache or metadata-only freshness shortcut.

## The next view

`chough next` is what a resuming agent reads first; `chough status` is the full view.
It makes at most one `specsync change status` read, or reuses an authenticated archived
response as above, and evaluates the gate for the next main
stage on that response, and shows:

| JSON field | Content |
|---|---|
| `schema_version`, `view` | `1` and `next` |
| `objective`, `stage` | The objective; current stage, status, iteration and side-stage callers |
| `native` | `change`, `read` (the native section state), `state`, `approval_valid`, `correction_valid`, `terminal_evidence`, `next_action` and `approved_deltas` (`unchanged: <modules>`, `no semantic deltas (no_spec_change)`, `changed: ...`, `unverifiable: ...` or `not checked: <why>`; with SpecSync configured but SDD off, as `specsync init` leaves it, the reason is ``native SDD is disabled; run `specsync change adopt` to gate stages``, and the `ungated:sdd-disabled` gate detail says the same); unknown values are `null` |
| `native_check` | For a recorded native change with a workspace under `.specsync/changes/`, otherwise `null`: `last_attempt` (the latest attempt in its `verification-attempts.json` by timestamp: `passed`, `at_unix_seconds`, a 12-character `commit` and `failed_commands` with at most 3 `command` and `exit_code` entries and their `total`, or `null`), `not_passed_for_tree` (native state `approved` or `implementing`, or `verifying` while SpecSync's `handoff.readiness` from the same read is not `safe`, which it reports when the tree moved after the last check; a `verifying` reply with no readiness gives an "unknown" note instead), `note` and `unknown` (why the record could not be read; never shown as no failure). SpecSync 6.1.2 records an attempt only when a verification command runs; a refusal before verifying, such as incomplete tasks or missing requirement evidence, leaves no record or reason, so the note names `specsync change check <id>` (#86). It reads that one local file and no native command |
| `direct_writes` | Pending direct lifecycle writes; see [Direct lifecycle writes](#direct-lifecycle-writes) |
| `native_verification_warning` | A sentence when the native check runs none of the project's commands, judged from the change's effective commands in the status read, otherwise `null`; `status` carries the same key. See [Native verification that runs no project command](#native-verification-that-runs-no-project-command) |
| `next_valid_action` | `action`, `reason` and, when one applies, `command`; `status` reports the same `action` as its `next_valid_action` |
| `gate` | `to`, `allowed`, `label`, `detail` for the next main stage, or `null` |
| `awaiting_human`, `blockers`, `open_questions`, `blocking_bugs` | Each `{"total": N, "items": [...]}`; `other_open_bugs` counts the rest. `open_questions` are the session's Open threads, from `checkpoint --question` or a folded inbox question; `status` reports all of them in its `checkpoint` section |
| `gates` | Each open gate as `{"gate", "holds", "drives"}`, at most 10 |
| `observations` | `needs_attention_total` and `items` (referenced, each entry once however it is referenced, and `historical`, `removed`, `ambiguous` or `unavailable`, each with `id`, `label`, at most 3 `referenced_by` and `referenced_by_total`; a removed one's `receipt`; an ambiguous one's at most 3 `places` and `places_total`; and `resolved`, the `<session>/<id>` form a reference through its own session resolved as), `referenced_current`, `superseded` (runs not current left out because the checkpoint references a later run of the same case; the text shows it only when not 0) and `unreferenced` counts |
| `decisions` | `{"total": N, "items": [{"kind", "text", "reason"}]}`: the 5 newest items of the session's Decisions and why, newest first, a corrected one as its newest statement; see [Decision prefixes](#decision-prefixes) |
| `corrections` | The 10 newest corrections, newest first, each cut to 512 bytes, with their total; see [Corrections](#corrections) |
| `last_actor` | The `by` of the newest Timeline entry that has one, or `null` |
| `last_failure` | `{"id", "observed_at_unix_seconds", "summary", "bugs": {"total", "items": [{"id", "status"}]}}` for the failed observation with the greatest recorded `observed_at_unix_seconds` (ties to the greater ID), with at most 3 bugs that link it by their observations, Log or regression baseline, by bug ID; `null` when no observation failed. No freshness label is computed; run `chough status` or the case for that |
| `lessons` | `{"sources": {"total", "items": [{"module", "path", "section_line", "note"}]}, "archived_bundles": {"total", "items": [<path>], "note"?}}`: for each checkpoint module (at most 10, in checkpoint order), `specs/<module>/<module>.3md` when present, otherwise `specs/<module>/context.md` with the 1-based line of its first line that is exactly `## Lessons` (`note` is `null` when a source is found, otherwise `no Lessons section`, `missing` or `cannot be read: <error>`, for example for a symlinked module directory); and, for the archives the librarian counts (not hidden, a UTF-8 name, and a readable `finalization.json` whose `change_id` the directory name ends with), each one's `lesson-bundle.md` path, newest first by recorded finalization timestamp then directory name, at most 3, the same three `knowledge/index.md` links; an archive without a bundle is shown as `<path> (missing)`, `total` counts those archives, and an unreadable timestamp sorts oldest; when the archives cannot be read, for example because an archive directory is a symlink, no bundle is listed, `total` is 0 and `note` is `cannot be read: <error>`, the same error `chough library index` refuses with (`note` is absent otherwise) |
| `knowledge` | `{"librarian", "dir", "index"}`: the librarian preference, the knowledge directory, and `<dir>/index.md` when the librarian is on and the index exists, otherwise `null` |
| `chough_writes` | `{"state", "text", "missing", "policy"}`: when Chough's own records are safe to write (REQ-records-014). `any-point` while `.specsync/sdd.json` leaves `.chough/sessions/`, `.chough/archive/` and `.chough/bugs/` out of verification (or there is no policy): they may be written and committed at any point of the native order. `commit-first` otherwise: commit them before `specsync change check --commit` and write nothing under `.chough/` until finalize; `missing` names the entries. The text form prints it as a `Chough writes:` line |
| `environment`, `full_view`, `truncated` | The requested environment, `chough status`, and whether entries were dropped |

The text form (the default) carries the same fields as short lines. SpecSync's own next
action, often a later step of its lifecycle than the one to take now, is shown under the
native change as `SpecSync's own next step, for reference (the step to take now is
Next): <text>`, so `Next:` alone is the step to take now; the JSON keeps it as
`native.next_action`. Each form is at
most 16384 bytes: every list shows at most 10 entries beside its total, every string
taken from a record is cut to at most 512 bytes, counted as JSON escapes it (a control
character takes six), ending in ` [cut]`. A command is never cut, whatever its length
(REQ-cli-021): in the next valid action and the gate, each command in backticks stays
whole and only the text around it counts toward the 512 bytes, so the commands after a
cut follow ` [cut]` in order, and the `command` field is never cut. While the
output is still over the cap the last entry of the longest list is dropped and
`truncated` becomes true (text adds `Output truncated at 16384 bytes; run chough
status.`). The decision items, linked bugs, lesson sources and archived bundle paths are
lists like the others: they are dropped by the same rule, and their totals are counted
before any cut. It never prints raw native output, evidence records, the snapshot,
the full reference list or lesson text. The lesson pointer reads only the heading line
of each source and records nothing: no fold state, ledger or index. Every ordering
comes from recorded fields or names (checkpoint order, recorded observation time and
ID, bug ID, archive directory name), never from file modification times or directory
order, so the same records give byte-identical output in both forms.

### The status view

`chough status` starts with the same focused summary `chough next` gives, so the next
action is never buried (BUG-0023). Its JSON starts with a `summary` object: `stage`
(`current`, `status` and `iteration`), `native_change`, `next_valid_action` (`action`,
`reason` and `command`, the same action `next --json` gives, but whole: status is the
full report, so it never cuts the action as `next`'s bounded view cuts a long one),
`gates` (each with `holds` and `drives`) and `blockers`. Its full report follows, including `corrections`
and the session's decisions. In text, the first 40 lines give the stage, the next action
and its command, the open gates and the blockers.

### Native verification that runs no project command

`specsync change status --json` reports the commands a change's native check runs as
`summary.verification_commands`. When SDD is enabled and every one of them is
SpecSync's own (`specsync check --spec <module>` and the like), the check runs none of
the project's commands, so its "verified" does not show that the project's tests ran
(#85, FLOW-15). `chough next` then prints a `Warning:` line under the native change, and
`next --json` and `chough status` carry the sentence as `native_verification_warning`
(otherwise `null`). It is judged from that list, not from `.specsync/sdd.json`:
SpecSync 6.1.2's `change check` does not run the `verification_commands` configured
there (CorvidLabs/spec-sync#848), so configured commands the list leaves out are counted
in the warning instead of hiding it. It uses the status read already made, adds none,
says nothing when the list is absent, and is advice only: every stage move is allowed
or refused exactly as without it.

### Decision prefixes

Decisions are list items in the session's Decisions and why. `next` reads a prefix
convention best-effort: an entry that begins with `decided:`, `rejected:` or `failed:`
is shown with that kind, and when it contains ` because `, the text after the last
` because ` is its reason and the text before it its text, as in `rejected: a
structured decision schema because it needs a migration`. Any other entry, such as
older free text or `2026-10-31: ...`, is shown whole with `kind` and `reason` `null`,
so the newest entries are always shown. Each text and reason is cut to 512 bytes.
Nothing validates the convention, and an unknown prefix is just text.

### The next valid action

The next valid action follows these rules in order:

1. A paused stage checks native state and runs `chough stage resume`.
2. An open gate that holds the current stage is named and left to the human: `Waiting
   for a human decision: <gates>. Do not decide it for them; continue only independent
   work within scope.`, naming `chough checkpoint --close-gate` to record their decision
   and saying that `chough stage advance` and `chough stage skip` are refused until it
   is closed.
3. Open blocking bugs at test or accept send work back to build.
4. At test, a referenced case whose latest executed run failed with no linked bug names
   the `refused:unlinked-failure` refusal and `chough bug add`.
5. Right after `chough init`, at intent with no criteria referenced, the action is to
   capture or read the intent: run `hi check`, then `chough context`, and reference the
   criteria the work covers with `chough checkpoint --criterion <id>` (#76).
6. At accept while the native change is verifying, the human reviews next. The action
   names `chough gate list`; for a decision to ship, `chough change ship <id>
   --close-gate "<gate>"` with the relay, which runs the review as them, ships, and
   closes the gate only after the native ship, so nothing under `.chough/` changes
   between `change check` and finalize; and for a decision not to ship, `chough
   checkpoint --close-gate "<gate>"` with the relay and `--verdict block`. With no
   review gate open, its command opens one with `chough checkpoint --awaiting-human
   "Implementation review of <id>" --holds accept --drives review`; while SpecSync's
   verification counts Chough's records (`chough_writes` is `commit-first`), it also
   says to commit that gate and run `chough change check <id>` again before the ship.
   With the review gate open, the waiting action names the same ship. It never suggests
   an agent review: where `next` and `status` show SpecSync's next action at accept,
   its review command, which names a `--reviewer`, reads as the human's review at the
   review gate, in the native summary and the gate to done alike (REQ-cli-021).
   While the native change is still approved or implementing, the action is the review
   preparation of the workflow's accept step, in its order: `chough change check <id>
   --no-commit` first when a delta of the change adds or modifies requirements, so the
   requirement text the cases link is materialized; then the affected cases with
   `chough cases run`; the review gate with `chough checkpoint --awaiting-human
   "Implementation review of <id>" --holds accept --drives review`; the commit of the
   source with Chough's records; and `chough change check <id>`. Its command is the
   first of those steps. It never shows the refused move into done or a bare native
   status read.
6a. At plan, while the native change is a complete draft that waits for its definition
   approval (SpecSync's next action names `specsync change approve`), the human
   approves the scope next. The action opens the scope gate with `chough checkpoint
   --awaiting-human "Approve the scope of <id>" --drives approve`, says to show them
   `chough gate list`, and names `chough checkpoint --close-gate` to record their
   decision, which runs the approval as them. Neither `next` nor `status` passes on
   SpecSync's own approve command, which names an `--actor`; where they show
   SpecSync's next action, it reads as the human's scope approval (REQ-cli-021). An
   approval on authority the human gave in advance stays as
   [Approved-delta guard](#approved-delta-guard) describes.
7. In stage done, an archived change whose terminal evidence authenticates, or no
   recorded native change, closes the cycle. The move into done archives the session,
   so `next` then refuses with "no open session" and names `chough session new <key>`
   (see [Moving into done](#moving-into-done)). For a session the move kept open, the
   action names what closes it:
   - while an authorized release is still ahead, even with inbox entries unfolded,
     `Release is authorized for this cycle: enter it.`, with the command
     `chough stage enter release --reason "<request>"`;
   - otherwise, with unfolded inbox entries, folding them with `chough inbox fold` and
     then archiving the session with `chough doctor --fix`, with the command
     `chough inbox fold`. Entries still unfolded after the move from release into done
     keep the session open the same way;
   - with neither, `Start the next piece of work in a new session: chough session new
     <key> --objective "<outcome>", then specsync change new "<outcome>" --spec <module>
     --path <path>, then chough checkpoint --native-change <id> --next-action "<next
     step>".`, without the `specsync change new` step when native SDD is not configured
     or is disabled.

   The native next action is named as the thing to check first. No stage reset is
   named at done. An archive that does not authenticate keeps its gate refusal.
8. At intent, design, plan, build, test or accept, when the recorded native change is
   archived with terminal evidence that the done gate accepts, the stage is not
   continued: the action is `The recorded native change is archived while the checkpoint
   is at <stage>, so this cycle's work is closed.` followed by the same new-cycle steps
   (fixed text, so `next` never cuts it), the reason opens with `Native change <id> is
   archived while the checkpoint is at <stage>`, its command is the reset, and the
   native next action is named as the thing to check first. At accept it first says
   `Record done first: chough stage advance (accept to done is allowed), which archives
   this session. Then:`, followed by rule 7's new-session steps instead of the reset,
   and its command is `chough stage advance`, the documented close after finalize
   (release and side stages are not affected).
9. After the suggested reset the checkpoint still names the archived change until a new
   one is recorded, so rule 8 does not apply while the archived change belongs to the
   previous cycle (the latest stage entry is a reset, or the change was finalized no
   later than the latest reset). The session's whole Timeline is read for that reset;
   when it cannot be read whole, the action says `Unknown whether native change <id>
   belongs to this cycle` with the reason and the command `chough history show`, rather
   than guessing either way. The reset's next action stays, and a refused gate for the
   next main stage keeps it, names this cycle's missing native change and suggests
   `specsync change new`, never another reset.
10. A closed gate for the next main stage shows its label and detail (with no native
    change recorded, `refused:no-native-change` suggests `specsync change new` rather
    than a status read).
11. At test, when nothing else stops it, the action is `Continue test: <next action>`,
    the checkpoint's next action ended as a sentence of its own, and names `chough cases
    select` for the cases this change affects, `chough cases run <case-id> --environment
    <env>` for each, and `chough bug add` for a failed run. Once every case the
    checkpoint references passed on its latest executed run, and no blocking bug is
    open, it names `chough stage advance` to accept instead, with that command.
12. Otherwise `Continue <stage>: <checkpoint next action>`.

Every reset Chough suggests carries `--reason`.

## Approved-delta guard

SpecSync 6.1.1 and earlier keep `approval_valid` true after an approved semantic delta is
edited (BUG-0002); SpecSync 6.1.2 flags it. Chough keeps its own check with either, so
before a move to build, test or accept (by advance, skip, back, reset or return) on a change in state approved, implementing or verifying with
`approval_valid` true, Chough reads `.specsync/changes/<id>/approvals.json` read-only
within 4 MiB, SpecSync's change-artifact bound, takes the effective `definition`
approval (the one whose definition digest matches the current definition, as SpecSync
treats it for every workflow version, BUG-0020), and compares its
`approved_delta_digests` with digests of `deltas/<module>.md` for every module in the
status response's `change.affected_specs` (and every approved module), computed with
SpecSync 6.1.1's `specsync.approved-delta.v1` framing: SHA-256 over the length-framed
domain, module name and body, the body with CRLF normalized to LF.

The current agent performs the recovery operations below. A new definition approval
comes either from the human's decision at the scope gate, which runs the approval as
them, or, on authority they gave in advance, from the agent, which records its own
account as the native actor and the human's words, where they said them, covered scope
and limits in the note. Apply existing authority when it covers the repair; ask only
for an actual missing or exceeded scope decision, and never ask the human to run a
command. Scope approval does not supply implementation acceptance or delivery
authority.

| Label | When | Next native action |
|---|---|---|
| `refused:native-delta-changed` | A delta of one of the change's affected specs or approved modules differs from its approved digest or is deleted. A delta for a module in neither is not compared here; SpecSync's `change check` refuses it. SpecSync 6.3.0 itself reports an approval with a changed delta invalid, so there the same edit is refused first as `refused:native-approval-invalid` | Restore the approved delta, or have the changed scope approved: open a scope gate with `chough checkpoint --awaiting-human "<gate>" --drives approve` and record the human's decision with `chough checkpoint --close-gate`, which runs the approval as them; on authority they gave in advance, record `specsync change approve <id> --actor "<agent>-<terminal>" --note "Approved under <the human's words> (<source>); covered scope: <scope>; limits: <limits>. Recorded by <agent> (<model>, hc <terminal>)."`. Then run `specsync change check <id>` |
| `refused:native-delta-unverifiable` | The ledger is missing, unreadable, over 4 MiB or malformed; it has no definition approval; status reports no `change.affected_specs`; a module name is not one SpecSync 6.1.1 accepts; a delta cannot be read | Run `specsync change status <id>` |
| `refused:native-delta-unverifiable` | A change with deltas has an approval without `approved_delta_digests` | An approval records the digests, the same two ways as for `refused:native-delta-changed`; then run `specsync change check <id>`; a status read cannot clear it |

A no-spec-change approval with no deltas and an empty or absent digest map passes. The
guard starts no process and copies nothing; no gate runs `specsync change check`,
which writes canonical specs and native records and runs verification commands.
Archived changes keep the terminal-evidence rule.

## Observation outcomes and freshness

| Outcome | Meaning |
|---|---|
| `passed` | The process exited 0, its output was captured completely and the binding's assertion held, or every verdict passed |
| `failed` | A reported test or declared assertion failed, a verdict failed, the run timed out, or its output capture was incomplete |
| `not-run` | Nothing was asserted: no test matched, the binding was ambiguous, or cargo failed before reporting results |

| Freshness | Meaning |
|---|---|
| `current-observation` | Inputs, artifacts, case definition and environment match now |
| `stale-revision-or-inputs` | A fingerprinted file changed; for a record without a content identity, also any new revision |
| `stale-artifact` | A bound artifact changed or disappeared |
| `stale-case-definition` | What the case asserts changed since the run |
| `stale-environment` | A different environment was requested |
| `environment-unchecked` | No environment was supplied |
| `unknown-revision` | No Git snapshot could be established |

The label keeps its name for JSON compatibility, but the revision alone no longer
stales an observation. Each record still stores the HEAD revision, shown in status and
next and used for regression-baseline ancestry; freshness compares a working content
identity (`content`, a SHA-256 digest). It covers every tracked or untracked
non-ignored file except `.chough/`, `target/`, the generated knowledge directory (the
configured `library-dir`, `knowledge/` by default) and SpecSync lifecycle bookkeeping:
the top-level `state.json`, `approvals.json`, `verification.json`,
`verification-attempts.json`, `review.json`, `review-attempts.json`,
`finalization.json`, `accepted-state.json` and `lesson-bundle.md` of each
`.specsync/changes/<id>/` and `.specsync/archive/changes/<date>-<id>/`, and the value of
the front-matter `state:` line of its `change.md`. Change artifacts are keyed by change
ID and their path in the change, so finalize moving a change into the archive keeps
observations current. A deleted tracked file counts as absent. So committing only
Chough records, an empty commit, committing edits that were already present,
`specsync change check --commit` and finalize keep observations current; a change to
source, tests, docs, `specs/`, `.specsync/` configuration or a change's deltas and
other artifacts makes them stale. A record written without a content identity, by an
earlier Chough, keeps the revision-bound comparison and stays stale after any new
commit; a malformed one is refused.

Freshness reads an evidence entry, and a receipt in an archived story or bug, the same
way, and a receipt migrated from an observation with no content identity keeps the
revision-bound result.

Referenced observations carry a label in `status` (`observation_references` and each
entry in `observations`), `next` (the referenced ones that are not current, in
`observations.items`), `bug show` (`observation_labels`) and `validate status` (each
run's `observation_labels`): `current` (present, and its inputs, artifacts and case
definition match now, and the requested environment when one is given), `historical`
(present but stale), `ambiguous` (a merge left two records with that name, and the
row names each place as `<place>/<id>`) and `unavailable` (absent). An ID whose record
`chough migrate` removed without carrying it resolves as `removed`, naming the receipt.
None of these but `current` counts as current, passed or verifying evidence.

A bug reads `reproduced-and-later-passed`, and accepts `bug update --status verified`,
only when a present linked passed observation was observed strictly after its most
recent present linked failed observation (by recorded observation time). A receipt
counts as present, and an observation shown as removed does not. An older pass, or one
in the same second, leaves it `reproduced`; with no linked failure it reads
`unverified` and `verified` is refused.

Environment comparison uses the label you supply; it does not detect changes in the
real environment. A native status response that decodes as JSON is usable output, not
evidence that a lifecycle gate passed.

Kinds: `product-test`, `structural`, `human-observation`, `delivery-observation`,
`publication-observation`, `installation-observation`, `deployment-observation`,
`live-health-observation`, `dry-run`. A dry run never satisfies a delivery kind. Human
approval and delivery stay unknown in helper output; an observation is an
unauthenticated claim by whoever recorded it.

## Evidence entries

Each observation is an evidence entry of the active session,
`.chough/sessions/<session>/evidence/<id>.md` (REQ-cli-017). Its front matter gives
`kind: evidence`, `id`, `aliases`, `claim` (the summary), `result` (`passed`, `failed`
or `not-run`), `evidence_kind`, `method`, `track`, `case` and `case_digest` when it is
bound to a case, `command` (the executed argument vector), `exit_code`, `tests_run`,
`revision`, `content` (the working content identity), `environment`, `fingerprint`,
`raw_log`, `external` or `screenshot` when present, `artifacts` and
`artifact_fingerprints` for project files given with `--artifact`, `criteria`,
`requirements`, `contracts`, `limitations`, `observed` (the UTC time),
`duration_seconds` when known and `actor`. Its body is the claim as one bold sentence
and one plain sentence saying what the receipt shows, followed by the key failing lines
when it failed.

Every view of an entry, and what `cases run`, `cases record` and `evidence add` print,
carries `provenance`, what the entry is and is not. A case run Chough executed, one with
a `command`, says "unauthenticated observation; not proof of human approval or
deployment"; a manual entry says "unauthenticated observation; not proof of human
approval, test execution, or deployment".

A repeated validation run's single entry gives `validation_run: true`, `run_case` (its
case, in place of `case`), `authority`, `times`, `interval_seconds`, `state`,
`started`, `completed`, `failures`, `not_run`, `elapsed_seconds`, `bugs` and
`attempts`, one flat map per attempt with `started`, `observed`, `result`,
`duration_seconds` and `observation`, the case-run entry that attempt recorded. Its
`result` is `not-run` and its `evidence_kind` is `structural`: the run is a summary, its
attempts are the observations, and it never counts as a run of its case. Chough rewrites
it in place as each attempt finishes.

A raw log is never committed (REQ-records-005). The raw output of an executed case is
written to `.chough/local/logs/<evidence id>.log`, and the entry gives the exact
command, the SHA-256 of the captured output as its `fingerprint` and, when a CI run
holds the log, `raw_log` naming that run. For a failed or not-run run, the entry's body
keeps the key failing lines: at most 20 lines and 2,048 bytes of the captured output,
with complete terminal escape sequences removed, taken for the `cargo-test` adapter
from the first failure block and the `test result:` line, and otherwise from the last
non-blank lines of standard error and then standard output. A failing line that holds a
local path or credential is redacted, never refused, and the entry says how many were.
`evidence add --failing-lines <text>` keeps a manual failure's own key failing lines
within the same bounds.

An artifact given to `evidence add --artifact` or `cases record --artifact` as an
absolute path outside the project, one per entry, is fingerprinted in place with
SHA-256, within a 64 MiB bound, and never copied into the repository; a missing path, a
directory, a symlink and a file over 64 MiB are refused, naming the path on standard
error only. Its path and fingerprint go to `.chough/local/external-artifacts.toml`
under a neutral ID, `ext-<6 hex>`, and the committed entry gives only `external:
ext-<hex>` and `fingerprint: sha256:<64 hex>`. On the machine that holds the ID,
freshness re-reads the file and reports the artifact `current` or `changed`; elsewhere
it reads `external, not checkable here`, which is neither a pass nor a staleness of the
entry's own content.

`evidence add --screenshot <path>` commits one key screenshot with its entry
(REQ-records-013): a PNG or JPEG of at most 524,288 bytes, at most 5 per session and
2,097,152 bytes per session in all.
Chough removes its metadata (PNG `tEXt`, `zTXt`, `iTXt`, `tIME` and `eXIf` chunks; JPEG
`APP1` to `APP15` and `COM` segments) and writes it by create-new as
`.chough/sessions/<session>/evidence/<evidence id>.png` (or `.jpg`) beside the entry,
which gives `screenshot` and the `fingerprint` of the committed bytes. Another format,
a file over a bound, a symlink, and a sixth screenshot or one past the session total
are refused with nothing written, saying to crop or compress it, or to record it by
fingerprint with `--artifact`. At finish each screenshot moves beside the story,
to `.chough/archive/sessions/<session>/<evidence id>.<ext>`. Chough cannot see what an
image shows: check a screenshot for personal information before adding it.

Chough creates each entry, raw log, screenshot and validation run entry by atomic
create-new: a same-name different-content creation is refused and the existing bytes
stay. A validation run's entry is the one entry rewritten afterwards, as its attempts
finish.

## Retention

Retention is retired in 0.6.0 (REQ-history-001, REQ-history-004): evidence lives in its
session and folds into the archived story at finish, raw logs are never committed, and
files leave the tree only when their bytes are committed. There is no prune command and
no `.chough/pruned.jsonl`. `chough pref set retention` and
`chough consent grant retention` are refused, naming why, with nothing written; a value
an earlier release recorded still reads, and nothing acts on it. Never delete evidence
by hand, and never report a `removed`, `ambiguous` or `unavailable` observation as
passed.

## Knowledge vault

The knowledge directory (`knowledge/`, or the `library-dir` preference) holds only what
has no other home (REQ-cli-044, KNOW-30, KNOW-31). It links to `hi/`, `specs/`,
`.specsync/` and `.chough/` and never copies their content:

| Path | What it is |
|---|---|
| `index.md` | The reading guide, generated (`type: guide`): where to start; the journal and its feed as the timeline of archived changes, with `.specsync/archive/changes/` linked for changes archived before the journal began, and the newest 10 posts in the feed's order, a post without a `time` marked "(no time)"; where things live (`hi/`, `specs/`, `.specsync/archive/`, `.chough/README.md`, `.chough/archive/sessions/`, `fledge.toml`); at most 20 open bugs, blocking first and then the newest by `created`, each linked, with their total and, when more are open, `.chough/bugs/` linked and `chough bug list` named for the rest; the rule, skill, decision and policy files that exist; each module's lesson source with its count of lessons, every file in `lessons/` and the newest 3 archived lesson bundles; and existing snapshots and briefs |
| `journal/<date>-<slug>.md` | Short posts (`type: journal`), written once by create-new and never rewritten by Chough (REQ-records-011) |
| `journal/feed.xml` | An Atom feed of the newest 20 posts, generated: newest first by `time`, and a post without one by its date and then name, after that date's timed posts. An entry's `updated` is its `time`, or its date at midnight UTC without one |
| `lessons/<date>-<slug>.md` | Lessons that span projects (`type: lesson`, with `applies_to` and `source`) |
| `snapshots/`, `briefs/` | Content from earlier releases, kept as it is and linked from the index (a snapshot by its `manifest.json`). The index also links the repository-root `library/` of earlier releases, outside this directory, unless `library-dir` makes it the vault itself |

Every note Chough writes starts with YAML front matter giving at least `id`, `title`,
`type` and `tags`, with ISO 8601 dates, and links with standard relative Markdown links
that resolve inside the repository, to a file or a folder, so the directory reads the
same in Obsidian and on GitHub. A link to a folder, such as `../specs/`, opens on
GitHub; Obsidian opens only links to files. Every note stays within 65,536 bytes. The generated `index.md` and `feed.xml`
are byte-identical for the same sources; they copy no criterion, requirement, lesson
or record text. A module's own lessons stay in its spec's lesson source. No gate reads
the vault.

A module's lesson source is `specs/<module>/<module>.3md` when it exists, where each
plane after the first is one lesson, and otherwise the `## Lessons` section of
`specs/<module>/context.md`. There `index.md` counts a lesson only when it starts with
its date, `YYYY-MM-DD` followed by a space, comma or colon, as a list item or a
paragraph, or as a `### YYYY-MM-DD` heading; an undated lesson stays in the file but
counts as none. A lesson also gives its certainty label, its source and its limits
(see the skill's `references/accept.md`):

```markdown
## Lessons

- 2026-11-12, verified: an order of exactly fifty dollars ships free, so the
  threshold check is inclusive. Source: bug 2026-11-12-shipping-charged-at-fifty.
  Limits: checkout totals only; refunds were not checked.
```

A journal post's front matter gives `id`, `title`, `type: journal`, `tags`, `date`,
`time` (the UTC time it was written, to the second; the file name keeps only the date),
`kind` (`shipped`, `closed`, `intent`, `spec`, `broke` or `learned`), `links` and, for
a finished session, `record`, the path of its archived story. The move into done
writes a `shipped` post and `session close` a `closed` one; `chough journal add` writes
any other.

Earlier releases also generated `INDEX.md`, `README.md`, `rules.md`, `timeline.md`,
`timeline/`, `intent/`, `archives/`, `modules/`, `bugs/`, `sessions/` and
`attachments/` here. Chough no longer writes any of them, and `chough library index`
removes a `README.md` an earlier release generated once its bytes are committed.

`chough migrate` removes the rest of that generated and copied content after listing
it.

## Test-case adapters

| Adapter | Runs | Passes only when |
|---|---|---|
| `cargo-test` | `cargo test --locked --test <target> -- --exact <test>` | Exit 0 and exactly one test reported and passed |
| `fledge-task` | `fledge run <task>` | Exit 0 and every `expect_output` fragment appears |
| `command` | The argv as written | Exit 0 and every `expect_output` fragment appears |

### Binding environment

Any binding may declare `env`, a table of variable names and string values that are
set for the executed command on top of the inherited environment. Chough never clears
the inherited environment and sets no variable of its own, not even `NO_COLOR`; add
`NO_COLOR = "1"` yourself when a tool should print plain text. Native reads are
unaffected.

```toml
[case.bind]
adapter = "command"
argv = ["npx", "vitest", "run"]
expect_output = ["Test Files  1 passed (1)"]
env = { NO_COLOR = "1", PORT = "4173" }
```

`cases check` refuses, with the case location: a name that does not match
`[A-Za-z_][A-Za-z0-9_]*` or is longer than 128 bytes; two names equal ignoring ASCII
case (Windows treats them as one variable); a value containing NUL or longer than
4 KiB; and more than 32 entries. A `cargo-test` binding also cannot declare, in any
ASCII case, a name that chooses what cargo builds or runs: `PATH`, `CARGO`,
`CARGO_HOME`, `RUSTC`, `RUSTDOC`, `RUSTC_WRAPPER`, `RUSTC_WORKSPACE_WRAPPER`,
`RUSTFLAGS`, `CARGO_ENCODED_RUSTFLAGS`, `RUSTDOCFLAGS`,
`CARGO_ENCODED_RUSTDOCFLAGS`, `RUSTC_BOOTSTRAP`, and any name starting with
`RUSTUP_` (the toolchain), `CARGO_BUILD_`, `CARGO_TARGET_` (a per-target runner,
linker or flags, which can carry `-C linker=...`), `CARGO_HOST_`, `CARGO_PROFILE_`
or `CARGO_UNSTABLE_`. `CARGO_TARGET_DIR`, which only chooses where cargo writes, is
allowed. For `command` and `fledge-task`, which name their
program in argv, a declared `PATH` can change which program that name resolves to.

`env` lives in the committed case file, so it is not a place for secrets. Its names
and values are part of the case digest: adding or editing `env` makes earlier
observations `stale-case-definition`, while an omitted or empty `env` leaves the
digest of an existing case unchanged. An observation names the declared variables,
`declared environment set: NAME, ...`, never their values. Every observation of an
executed case, with or without `env`, carries the limitation `inherited environment
variables were not recorded`: the inherited environment can still change what a
binding does, and the record cannot show it.

### Output matching

`expect_output` fragments and the `cargo-test` count of `test result:` lines are
matched against the captured text with complete terminal escape sequences removed;
the log keeps the raw captured bytes. Removed are CSI sequences (`ESC [`,
parameter bytes 0x30 to 0x3F, intermediate bytes 0x20 to 0x2F, one final byte 0x40 to
0x7E), OSC sequences (`ESC ]` ended by BEL or `ESC \` before any line break) and
two-byte sequences of `ESC` and one byte from 0x40 to 0x5F. A sequence that does not
complete under these rules, including one cut by a line break or the end of the
output, is left unchanged. Removal never removes a line break, so it can never lower
the number of `test result:` lines. When anything was removed, the observation carries
the limitation `terminal escape sequences were removed before matching; the artifact
keeps the raw output`.

An `expect_output` fragment that contains ESC (U+001B) is refused by `cases check`,
because it could never match the cleaned text. A case file that matched raw escape
codes must match the visible text instead.

### Error hints

A requirement link that no searched companion holds is refused as before, with the
same exit code, and the message names the companions searched,
`requirement <ID> not found in specs/<module>/requirements.md`, then a likely cause:

- When the ID has the form `REQ-<module>-NNN` (the module is what remains after
  removing `REQ-` and the last `-NNN`, so `REQ-checkout-ui-004` names `checkout-ui`)
  and that module's `requirements.md` holds it, the message names the module and the
  fix. A cases file binds one module's contract, so `cases check`, `run` and `record`
  say to move the case to `.chough/cases/<module>.cases.toml`; `evidence add` says to
  add `--contract specs/<module>/<module>.spec.md`.
- Otherwise the ID may be a typo, or a requirement an approved native change adds
  that `specsync change check` has not yet materialized: record that change in the
  checkpoint (see below), or run that check before running or recording cases that
  link it. With a native change recorded, the refusal names the deltas it searched too.

### Approved delta links

`cases check`, `cases run`, `cases record` and `cases select` accept a case
requirement that the module's `requirements.md` does not hold yet when the
checkpoint's native change adds or modifies it in its delta for the case file's
module (`.specsync/changes/<id>/deltas/<module>.md`, under `## ADDED` or
`## MODIFIED`), so failing-first cases can be linked and run before
`specsync change check` materializes the delta (Raven BUG-0004). The change must pass
the same native checks as the move to build: state approved, implementing or
verifying, `approval_valid` true, `correction_valid` not false and the approved
deltas unchanged (see [Approved-delta guard](#approved-delta-guard)). Chough makes
this one native status read per command, only when a link misses its companion, and
decides no approval itself. `cases check` lists each such link under
`approved_delta_requirements` with its case, requirement, change and module. When
the change was consulted and did not resolve a link, the refusal adds
`the checkpoint's native change <id> was consulted and did not resolve it: <reason>`,
such as `its cli delta does not add or modify REQ-cli-091` or the build gate refusal.
`evidence add` resolves a `--requirement` the same way, from the delta of each module
it searches: the module of each `--contract` given or, with none, the module the ID
names. A requirement in neither is refused naming both, `requirement <ID> not found in
specs/<module>/requirements.md or in .specsync/changes/<id>/deltas/<module>.md`, with why
the change did not resolve it.

## Timing

`chough timing` reads existing records and writes none: every file under `.chough/`
and `.specsync/` is byte-identical afterwards. It runs no case and makes no network
request; its only external commands are bounded `specsync change status <id> --json`
reads (above).

| Section | Derived from |
|---|---|
| Stages | The Timeline of every open session and every archived story, within 4,096 entries and 16 MiB in one request; a session or story that cannot be read is named and the report marked partial. Each `advance`, `iterate`, `back`, `reset`, `skip`, `pause`, `resume`, `enter` and `return` entry ends the open segment and starts the next; `gate-opened`, `gate-closed`, `direct-native-write` and `correction` entries start or end none. Every `reset` starts a new cycle and `back` never does. The first cycle starts at the session's `start` entry and holds the stage the session started in; a timeline without one, such as a migrated 0.5.x one, starts the first cycle at its first stage move and shows the segment before it as `Before the first record`, its start unknown. Time in `done` until the next reset is idle between cycles. Iterations, side-stage visits and paused time are shown separately; active time is stage time minus waiting on a human, and paused segments are not stage time. The open segment ends now and is `ongoing`. |
| Human gates | `gate-opened` and `gate-closed` entries paired by gate text and reconciled with the checkpoint's awaiting gates: opened and still listed is open (elapsed so far); opened, never closed and no longer listed was `cleared without a gate record` (close time unknown); listed with no entry, or closed with no opening, was `opened before gate records or record missing`. A second `gate-opened` for a gate already open means it was cleared in between without a record: the earlier opening is closed at an unknown time and a new gate starts. A gate whose close is before its open (or that is open with an open time after now) has one wrong time, so every segment it could overlap under either reading has unknown waiting and active time. Overlapping gates are merged, so waiting time is never counted twice; a gate with an unknown end leaves the affected segments' active time unknown. A gate entry says when; a relayed one also carries a claim of who decided (REQ-cli-050), which timing does not report. |
| Tests | Case runs per case and environment: time, outcome and run time, from evidence entries and the receipts of archived stories and bugs. `cases run` records `duration_seconds` in whole seconds, and a run under one second is `recorded together`, not a measured zero; manual records and earlier runs have no run time, shown unknown. |
| Validation runs | Started, completed, failures, not run, recorded attempts and `elapsed_seconds`, labelled total run wall-clock, including intervals and resumes: it is never a per-test time. |
| Bugs | Created; reproduced is the earliest linked failed observation (unknown with reason `unavailable` when a linked observation is not present); verified is the first Log entry that records `status: <old> -> verified` (labelled `from the bug Log`); fix time is reproduced to verified. |
| Native | The current change (one status read) and, only for an unfiltered report, the 10 most recent archived changes, by archive directory date and, on the same date, by `finalization.json` timestamp (one read each, reusing the current change's read when it is also archived; older ones are skipped and reported partial). See [Timing native reads](#timing-native-reads). Created and updated come from status. From the change's record files only `approvals[].gate`, `approvals[].timestamp`, `reopenings[].timestamp` and the `timestamp` of `verification.json`, `review.json` and `finalization.json` are read, within SpecSync's change-artifact bound; actor, note, reviewer and every other field are never read. Validity comes only from status: a definition approval is `current` only when status reports `approval_valid` true; an acceptance or finalization approval that a later native reopening supersedes is `reopened` (a reopening never supersedes the definition approval), and one superseded by a later approval of its gate, or that status reports invalid, is `not current`. |
| Estimates | Per stage (stage time per completed cycle, except a stage with time a backfilled move bounds), per bug fix and per closed gate: sample count with minimum, median and maximum, labelled `estimate from N recorded <items>; not a target`. Fewer than 3 samples are `too few to estimate`. Idle time, unknown and same-second values and a current cycle that has not reached `done` are never samples; a current cycle that has reached `done` is complete and is a sample. |

Labels: `elapsed wall-clock`, which the text says once, in its header, rather than
after every number, while the JSON keeps it on each duration; `recorded together` when
start and end share a second (records written together, not instant work); `ongoing`;
and `unknown` with its reason
(missing timestamp, end before start, no transition before the first entry, a gate
opened before gate records or cleared without one, a linked observation unavailable,
a native time not exposed, a native read `failed`, `timed-out`, `oversized` or
unavailable, an unrecognised record shape, a record over its bound). Unknown values
are `null` in JSON and `unknown` in text, never 0. Times are UTC ISO 8601, with the raw
Unix seconds in JSON. Anything beyond a bound is listed under `partial` with what was
skipped. `--cycle current` shows only the current cycle; estimates always use every
completed cycle.

### Timing native reads

These rules (REQ-reporting-002, was REQ-cli-024) apply only to `chough timing`. Stage
moves always make their live native read with the per-read deadline above, and gates
still refuse every unknown native state. `next` and `status` can instead reuse an
authenticated archived response only as described above; timing never uses that entry.

- **One read per change.** Each distinct change ID is passed to `specsync change
  status` at most once per report. The response is reused wherever the report shows
  that change, then discarded: nothing is cached between commands.
- **History selection.** Archived history is read only for `--cycle all` with no
  `--stage`, `--case` or `--bug` filter. This changes the output: in 0.1.0 a
  filtered report still listed archived history. A filtered report now reads only
  the checkpoint change and emits `native.history: null`, `native.skipped_archived:
  null` and `native.history_selection: {"state": "skipped-by-filter", "filters":
  [...], "reason": ...}`, with a `partial` entry and a text line saying archived
  history was not read. An unfiltered report emits `history_selection.state`
  `selected` with `read`, `reused`, `skipped_beyond_bound`, `not_read_budget` and
  `budget_seconds`. Stage estimates come only from Timeline entries and are the same
  either way.
- **Whole-report budget.** `CHOUGH_TIMING_NATIVE_BUDGET_SECONDS` (whole seconds,
  1 to 3600, default 120) bounds the total time the report waits on native reads. An
  invalid value is refused before any record or native read. Each read runs with the
  smaller of the per-read deadline and the remaining budget; a read cut by the budget
  is `timed-out` and names the budget. A change the budget leaves no time for has
  `status_read` `not-read` and `not_read_reason` `budget-exhausted`. Its record files
  are not read either, so every time (created, updated, approvals, reopenings,
  verification, review, finalization) is unknown with the budget as the reason. It is
  counted in `not_read_budget` and listed under `partial`.

Timing's request-local deduplication does not reuse the archived-report entry. A cold or
invalid archived report still takes as long as SpecSync takes; only a completed
authenticated response can make later `next` or `status` reports cached (see
[issue #32](https://github.com/CorvidLabs/chough/issues/32)).

## Report read diagnostics

A `next` or `status` request lazily loads the structural case suite once and shares
its exact digests across observation and reference checks. No bound-case lookup
means no suite load. The context belongs to one root and one request; the next
request reads again. Artifact bytes, environment and source identities retain their
existing checks. This does not cache native authorization or change lifecycle gates.

### Trace output

`CHOUGH_TRACE=1` writes local attribution to stderr only. Lines are at most 1024 bytes;
without it, stdout and stderr are unchanged, and a trace never changes a report, a
record or the exit code.

- One `chough-trace: native state=<state> elapsed_ms=<n> command=<program args>` line
  per bounded native read, in every command; the state and time come first, and only
  a command too long for the bound is shortened, ending `...`.
- For timing, one `chough-trace: phase <name> elapsed_ms=<n>` line per phase
  (`transitions`, `tests`, `validation`, `native`, `render`).
- For each writer, one `chough-trace: checkpoint-lock waited_ms=<n>` line (#132).
- For `next` and `status`: `snapshot.git-enumeration`, `snapshot.hash` and
  `snapshot.git-status` separate Git and source work; `evidence.load` marks observation
  parsing; `case-suite` marks a structural suite load; `freshness.artifacts`
  aggregates artifact-check time; and `next.assess`, `next.references`, `next.render`,
  `status.assess`, `status.observations`, `status.references` and `status.render`
  identify report work. A line such as `case-suite loads=1 lookups=48 unique_ids=2`
  counts suite load attempts, digest lookups and distinct resolved IDs, not
  operating-system file opens. A no-case request reports zero loads. Nested intervals
  overlap, so do not sum them as independent wall time.

Profile old/new freshly built binaries against identical frozen inputs. Record the
revision, source patch if dirty, binary hash, build profile, input sizes, flags and
native outcomes. `sh examples/measure-report.sh OLD NEW ROOT [RUNS]` does this for
`next`. It runs both binaries against one root and prints that root with its
revision and dirty state, each binary's SHA-256 and version, and each run as first
process or repeated. Native wait (completed native reads in the trace) is reported
apart from local time. Any native read that did not complete makes the native side
incomplete, with no native comparison, and no line combines local and native time. A
first process does not imply a purged OS cache. A native timeout is incomplete
qualification even when the reporting command exits zero; a sum of zero completed
native calls is not zero native work. Request-local case reuse alone does not fix
expensive native authentication, and authenticated archived reuse never proves that a
later live gate or native check passed.

See the [walkthrough](walkthrough.md) and [exercises](exercises/feature-bug-loop.md).
