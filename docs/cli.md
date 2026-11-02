# CLI reference

Run `chough --help` and each subcommand's `--help` for the installed executable. All
commands accept global `--root PATH` (default: the current directory). Recording
evidence or running cases requires the Git repository root and an existing commit.
Output is JSON unless stated otherwise. Exit codes: 0 on success; 1 when Chough
refuses or fails, with `Error: ...` on stderr; 2 for an unknown command or invalid
flag; 3 when `cases run` or `cases record` recorded an outcome other than `passed`.
`validate run` and `validate resume` exit 0 when attempts fail and report failures in
their JSON.

## Commands

| Command | Behavior |
|---|---|
| `init` | `--objective`, `--intent-scope`, repeated `--module`, repeated `--host openai\|claude\|codex\|cursor\|gemini\|grok` (default openai). Creates a schema 2 checkpoint and the skill bundle; preserves existing files. |
| `context [--json]` | Focused packet: instructions, intent, scoped HI export, module sources, recent transitions, valid transitions, preferences, open bugs. Markdown by default. |
| `status [--environment NAME]` | The full view, always JSON (`--json` is accepted and changes nothing): checkpoint, the native gate for the next main stage in `checkpoint.gate` (`null` when that stage has no prerequisite), valid transitions, `next_valid_action`, native status, snapshot, observations with freshness, open and blocking bugs. |
| `next [--environment NAME] [--json]` | The focused action view, text by default and compact JSON with `--json`; see [The next view](#the-next-view). Earlier builds printed the full status JSON here; scripts that read `next_valid_action` now read `.next_valid_action.action` from `next --json`, or use `status`. |
| `migrate [--stage S]` | Converts a schema 1 session after saving its bytes to `.chough/session.v1.json`; refuses if that backup exists. |
| `checkpoint` | Show, update or merge: `--objective` (clears run consents), `--next-action`, `--native-change`, `--decision`, `--question`, `--resolve-question`, `--completed`, `--blocker`, `--clear-blockers`, `--awaiting-human`, `--clear-awaiting-human`, `--criterion`, `--requirement`, `--case`, `--design`, `--file`, `--observation`, `--bug`, `--merge <base> <ours> <theirs>`. A gate added with `--awaiting-human` that was not already open appends one `gate-opened` transition, and `--clear-awaiting-human` appends one `gate-closed` transition per gate it clears (closed before opened when both are given), after the checkpoint is saved. |
| `stage show\|advance\|skip\|iterate\|back\|reset\|pause\|resume\|enter\|return` | Transitions with reasons and an append-only log. `reset` starts a new cycle and replaces the checkpoint's next action with `New cycle from <stage> after reset: <reason>. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action.`, the reason on one line and cut so the next action stays within 4096 bytes (the transition keeps it whole); `back` keeps it. Every move meets its destination's native prerequisite, whichever command makes it (advance, skip, back, reset, return, release → done): build needs state approved, implementing or verifying (another state is refused as `refused:native-<state>`) with `approval_valid` true (`refused:native-approval-invalid` otherwise) and `correction_valid` not false (`refused:native-correction-invalid` otherwise) and the approved semantic deltas unchanged (see [Approved-delta guard](#approved-delta-guard)); test and accept need that or an archived change whose terminal evidence authenticates; done needs archived (`refused:native-<state>` otherwise) with `terminal_evidence` validity `authenticated-history`, `exact` or `successor-covered` (`refused:native-archive-<validity>` otherwise). With no native change recorded a gated move is refused as `refused:no-native-change`; ungated moves are labelled `ungated:not-configured` or `ungated:sdd-disabled`. `release` is entered only from `done`, and side stages (`bugs`, `validate`, `librarian`) nest at most 4 deep. Intent, design and plan stay reachable. A malformed or ill-typed `.specsync/sdd.json` refuses gated moves as `refused:sdd-policy-invalid`; a valid `"enabled": false` or no policy file leaves them ungated, and a missing `enabled` means enabled, as in SpecSync. Leaving test needs no open blocking bug; entering validate or release needs consent or a standing preference. |
| `cases check\|list\|select\|run\|record` | Validate curated slug and exact legacy numeric definitions; list digests; select focused cases (`--criterion`, `--requirement`, `--path`, `--track copy-localization\|design-qa`); run an automated binding; record a manual or agent-assisted run with `--verdict N=pass\|fail` for every expected outcome. `run` and `record` allocate an OBS ID when `--observation-id` is omitted and exit 3 when the recorded outcome is not passed. |
| `evidence add [ID]`, `evidence prune [--apply]` | `add`: criterion, requirement, contract, kind, outcome, environment, summary; optional artifacts (spelled canonically: relative, `/`-separated), `--case`, `--method`, `--track`, `--limitation`. Omitting ID allocates the new OBS form; an explicit ID must use that form for a new write. Never replaces an ID. `prune`: preview, or with `--apply` remove, eligible observation sets; see [Retention](#retention). |
| `bug add\|update\|list\|search\|show\|report` | Local backlog. `add` takes `--title`, `--symptom`, `--trigger`, `--expected`, `--actual`, `--environment` (required) and refuses likely duplicates unless `--distinct REASON`; regression needs `--regression-baseline`. New manual and automatically captured bugs share the creation-time generator under [Bug identities](#bug-identities). `update ID` takes `--status` (`open`, `investigating`, `fix-pending-verification`, `verified`, `deferred`, `duplicate`, `wont-fix`), `--note`, `--observation`, `--duplicate-of`, `--related`, `--scope`, `--classification`, `--regression-baseline`; updates never recompute or rename the ID. `verified` needs a linked failed observation and a present linked passed observation observed after it (see [freshness](#observation-outcomes-and-freshness)). `list` shows open bugs, `--all` every bug; `search TEXT` scores words in title, symptom and trigger plus shared IDs; `report` prints Markdown and posts nothing. |
| `pref show\|set KEY VALUE\|unset KEY` | Standing preferences, each set with a required `--source`. Values (default first): `repeat-validation` ask, off, always; `monitoring` ask, off, always; `release` on-request, standing; `monitor-initiation` capture-only, propose-change; `librarian` off, on; `library-dir` a path (default `library`); `required-lane` a name (default empty); `retention` off, auto. Only `always`, `standing` and `auto` authorize without a run consent. |
| `consent show\|grant KEY --source S\|revoke KEY` | Run consent for the current objective: repeat-validation, monitoring, release, retention. |
| `validate run\|stop\|resume\|status` | Optional repeated runs of one case with stored limits and local capture of failures. `run CASE --times N --environment E [--interval S]` (N from 1 to 1000; S from 0, the default, to 3600 seconds) needs a case with a binding and a `repeat-validation` run consent or `pref set repeat-validation always`; unlike `stage enter validate`, `monitoring` does not authorize it. A new run receives a VAL ID and every attempt receives a new OBS ID. `resume ID` continues a new or legacy run under its original ID; `stop` and `status` take no argument. A stop request (state `stopped`) or a paused checkpoint stage (state `paused`) is checked immediately before every attempt, including after the interval, which is waited in slices of at most 200 ms; both states resume with `validate resume`. |
| `library init\|snapshot PATH... --reason R\|index` | Optional librarian; requires `pref set librarian on --source ...`. `snapshot` copies each named file (at most 4 MiB) into `<library-dir>/snapshots/SNAP-<slug>-<24-lowercase-hex>/` with a manifest of source path, revision, SHA-256 and creation time; `index` regenerates `INDEX.md` with new and legacy dated snapshots. |
| `skill status\|update [--host H]...` | Compare installed copies with the bundle; update only missing and known-outdated files. Without `--host`, both act on every host whose skill directory is installed; `update` refuses when none is. |
| `timing [--json] [--cycle current\|all] [--stage S] [--case ID] [--bug ID]` | Read-only report of how long stages, tests, bug fixes and human gates took, from existing records; Markdown-style text by default. Every duration is elapsed wall-clock time, not effort. See [Timing](#timing). |

## Records and limits

Managed JSON is bounded to 64 KiB, case files to 256 KiB, captured output to 1 MiB,
context to 16 KiB per section and 128 KiB in total, sessions to 16 modules, each
checkpoint list to 200 items and side-stage nesting to 4 levels.
Native SpecSync status is parsed from its complete output up to 4 MiB and displayed
up to 16 KiB; `status` carries the parsed state and next action in `native_summary`
and `next` in `native`. Larger output is reported as `oversized` and refuses gated
transitions. `status` and `next` each run `specsync change status` at most once and
evaluate the gate on that response.
Unknown fields and unsupported schema versions are rejected. Managed project paths
are project-relative; traversal and symlinks are rejected. The three checkpoint
merge operands may be absolute or outside the project because Git uses temporary
files, but each must be an existing regular file whose final component is not a
symlink. Native tools and bound commands receive direct arguments, never
shell-evaluated text.

| Record | Schema |
|---|---|
| Checkpoint | 2 (schema 1 read-only until migrated) |
| Observation | 2 (schema 1 still read). The optional `duration_seconds` that `cases run` adds is a schema 2 field; because unknown fields are rejected, Chough builds from before it (including the 2756dc2 release candidate) refuse a repository holding such records, so use this build or later there. |
| Case file | `profile = "chough-testing/1"` |
| Bug, preferences, transition (including `gate-opened` and `gate-closed`), validation run, snapshot manifest | 1 |

### Case, observation, validation and snapshot identities

New case definitions use curated `TC-<module>-<title-slug>` IDs. The title slug is
nonempty lowercase ASCII letters and digits in runs separated by single hyphens. The
module is the exact case-file module, and the complete ID stays within 100 bytes,
leaving `96 - module.len()` bytes for the title slug. IDs are globally unique across
all case modules. Deriving an ID from a title uses the same ASCII normalization as
other slugs, but later title edits do not rename a case. Exact legacy
`TC-<module>-NNNN` definitions remain valid with unchanged IDs, positions, serialized
fields and definition digests, without renumbering or migration.

Automatic observations use `OBS-<slug>-<24-lowercase-hex>`, repeated validation runs
use `VAL-<slug>-<24-lowercase-hex>`, and librarian snapshots use
`SNAP-<slug>-<24-lowercase-hex>`. Observation and validation slugs come from the case
ID; direct evidence without a case uses its observation kind, and snapshot slugs come
from the snapshot reason. Normalization lowercases ASCII, replaces each maximal run
of other bytes with one hyphen, trims edge hyphens and supplies a nonempty fallback.
Slugs are bounded to 71 bytes for OBS and VAL and 70 for SNAP so complete IDs stay
within 100 bytes without truncating the final separator or suffix.

Each suffix is one direct getrandom 0.4.3 fill of 12 bytes, encoded as 24 lowercase
hexadecimal digits. One allocation makes one candidate. Random suffixes let identical
repeated runs and independent roots retain distinct history, but do not prove that a
collision is impossible. Existing `observed_at_unix_seconds`,
`started_at_unix_seconds`, `created_at_unix_seconds` and `duration_seconds` fields
keep their meanings. Timestamps and stored durations, not lexical IDs or random
suffixes, determine chronology and timing.

`cases run --observation-id`, `cases record --observation-id` and the optional
positional ID on `evidence add` accept an explicit new-form OBS ID. When omitted,
they allocate one automatically. A legacy timestamp, numeric-suffix or other
old-style ID supplied for a new write is refused before an observation or automated
artifact is created, with a remedy to omit the ID for automatic allocation. No
explicit-ID option is added for validation runs or snapshots.

Every automated artifact remains exactly
`.chough/artifacts/<observation-id>.log`, inheriting the allocated or explicit
observation ID. Observation JSON, validation JSON and log files use atomic
create-new; snapshots use atomic creation of their final directory, create-new
children and a manifest written last. A same-ID different-content creation is
refused and preserves existing bytes. There is no overwrite, timestamp or numeric
fallback, digest retry, second random draw or other retry. Random-source and write
failures preserve pre-existing records. Separate artifact and record writes are not
a crash-proof multi-file transaction, and an interrupted new snapshot without its
manifest is not valid completed history.

Legacy case, observation, validation and snapshot IDs remain readable, linkable and
resumable where supported. Mixed IDs work through bug links, checkpoint references,
status and next, timing, retention and pruning, native manifests, validation status
and resume, and librarian indexes. Reads do not rewrite records. Observation schemas
1 and 2, validation schema 1, snapshot manifest schema 1 and bug schema 1 stay
unchanged, with no bulk migration. Older Chough versions require an upgrade to use
slug case IDs and the other new forms; downgrading is not a migration path.

Bug identity generation and known-answer vectors are unchanged. Transition,
preference, consent, stop-marker and pruning-ledger entry shapes gain no IDs, and HI,
requirement and SpecSync identities stay with their native owners. No runtime,
service, central allocator or persistent random state is added.

### Bug identities

Every new bug uses `BUG-<lowercase-title-slug>-<24-lowercase-hex>`. Manual `bug add`
and automatic failure capture call the same generator when they create a record. The
generator hashes the ASCII domain `chough/bug-id/v1`, one zero byte and a canonical
JSON creation payload containing all initial bug fields other than the ID and update
timestamp. The first 12 SHA-256 digest bytes become the complete 24-character
lowercase hexadecimal suffix.

To form the slug, Chough lowercases ASCII letters, replaces each maximal run of
characters outside ASCII `a` to `z` and `0` to `9` with one hyphen, and trims leading
and trailing hyphens. There is no Unicode transliteration. An empty result becomes
`bug`. The slug is limited to 71 bytes, with any trailing hyphen left by truncation
removed, so the whole ID is at most 100 bytes and the separator and digest are never
truncated.

The ID is generated once and stored in the schema 1 record and its filename. No
canonical payload or new schema field is persisted. Updates never recompute the ID or
rename the record, even when the title or other creation content has changed.
Automatic capture that appends to an existing open bug also preserves its ID.

Exact legacy `BUG-NNNN` IDs and new-form IDs are both accepted for loading, listing,
search, show, report, updates and evidence references. `duplicate_of` and `related`
links resolve either form in both directions. Every legacy record remains schema 1 at
its existing path without migration, renumbering or rewriting old observations and
archives. Older Chough versions that accept only `BUG-NNNN` must be upgraded before
reading, referencing or updating repositories containing new-form IDs; downgrading is
not a migration path.

Creation makes one candidate ID and one atomic create-new attempt. An existing
candidate path is refused without overwrite. A same-ID different-content result
reports a collision and leaves the existing record byte-identical; identical existing
content also remains untouched. Creation never falls back to numeric allocation,
retries the digest or makes a second candidate. Case, observation, validation and
snapshot allocation follows the preceding identity contract. Transition, preference,
consent, stop-marker, pruning-ledger entry, HI, requirement and SpecSync shapes are
unchanged.

## Checkpoint merges

`chough checkpoint --merge <base> <ours> <theirs>` performs a bounded three-way merge
of schema 2 checkpoints. It reads only the three supplied files, not the canonical
session merely because `--root` names a project. Schema 1, mixed or unsupported
versions, malformed JSON, wrong types and unknown fields at any level refuse. Schema
1 live writes still require the explicit backup-preserving `chough migrate` command;
merge never migrates it or creates a schema 1 backup.

Every input and the emitted result stay within the existing 65536-byte checkpoint
bound and 200-item collection bound, with all existing field validation enforced. A
compatible merge atomically replaces only `<ours>` after the complete result passes
validation. `<base>` and `<theirs>` remain byte-identical. Every refusal leaves all
three inputs byte-identical and creates no checkpoint, transition, gate, consent,
approval, review or other Chough record.

For `decisions`, `open_questions`, `completed`, `observations`, `bugs` and every
collection under `references`, strings compare exactly and case-sensitively without
trimming or normalization. A base entry survives only when both sides retain it, so
a one-side removal is not resurrected. An entry absent from base survives when either
side adds it, and an identical addition on both sides appears once. Output walks
surviving base entries in base order, then new ours entries in ours order, then new
theirs entries in theirs order. Reordering alone does not change membership.

The complete values of `objective`, `intent_scope`, `modules`, `stage`,
`native_change`, `next_action`, `run_consents`, `blockers` and `awaiting_human` form
one indivisible current-cycle group. Merge selects base when neither side changes it,
the changed side when only one changes it, or the identical new group when both make
the same change. If both sides change it differently, merge refuses with a
`current-cycle conflict` diagnostic naming the differing members. It never combines
fields, unions consents, blockers or human gates, or infers a lifecycle event.

The repository declares this driver association:

```gitattributes
.chough/session.json merge=chough-session
```

Automatic merging requires this setup in each clone:

```sh
git config --local merge.chough-session.name "Chough schema 2 checkpoint"
git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"
```

Git supplies base, ours and theirs as `%O`, `%A` and `%B`. The driver exits zero only
for a clean merge; a nonzero result leaves the conflict for human resolution.
`chough init` never sets either value or otherwise changes Git configuration. When no
driver is configured, the explicit `chough checkpoint --merge <base> <ours> <theirs>`
command is the fallback. One-owner live writes and the single canonical
`.chough/session.json` remain unchanged, with no second checkpoint store.

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
command and its descendants: on Unix the process group the command leads, on Windows
its process tree with `taskkill /T /F`. It then waits at most 1 s for the output to
close and abandons any reader still blocked, so the run returns on time and its
capture is reported incomplete. On Windows, `taskkill` cannot find descendants whose
parent has already exited; such a run returns on time with an incomplete capture,
and those descendants may keep running.

A case artifact keeps both streams within 1 MiB in total and ends with a note for
each stream that did not fit: `[truncated: stdout wrote N bytes; M discarded]`. Case
limitations name a timeout (`timed out after N s; the command and its descendants
were stopped`), descendants that held output after exit (`... and were stopped` or
`... and could not be stopped`) and `output capture incomplete`. A run whose capture
is incomplete is never `passed`.

Native section states are `observed`; `failed` for a non-zero exit or output that
stayed open and could not be read completely; `unavailable` when the command cannot
start or `CHOUGH_NATIVE_TIMEOUT_SECONDS` is not a whole number from 1 to 3600 (the
command is then not run); `timed-out`; and, for JSON status, `oversized` and
`invalid-output`. Each state other than `observed` leaves lifecycle state unknown,
and gated transitions refuse it as `refused:native-unknown`. Without a SpecSync
configuration, status reports `not-configured` and transitions are not natively gated.
With SpecSync configured and SDD enabled but no native change recorded, gated
transitions run no status command and refuse as `refused:no-native-change`, while
`status` and `next` still show the unscoped `specsync change status --json`. Record
the change with `chough checkpoint --native-change <id>`.

## The next view

`chough next` is what a resuming agent reads first; `chough status` is the full view.
It makes one `specsync change status` read, evaluates the gate for the next main
stage on that response, and shows:

| JSON field | Content |
|---|---|
| `schema_version`, `view` | `1` and `next` |
| `objective`, `stage` | The objective; current stage, status, iteration and side-stage callers |
| `native` | `change`, `read` (the native section state), `state`, `approval_valid`, `correction_valid`, `terminal_evidence`, `next_action` and `approved_deltas` (`unchanged: <modules>`, `no semantic deltas (no_spec_change)`, `changed: ...`, `unverifiable: ...` or `not checked: <why>`); unknown values are `null` |
| `next_valid_action` | `action`, `reason` and, when one applies, `command`; `status` reports the same `action` as its `next_valid_action` |
| `gate` | `to`, `allowed`, `label`, `detail` for the next main stage, or `null` |
| `awaiting_human`, `blockers`, `blocking_bugs` | Each `{"total": N, "items": [...]}`; `other_open_bugs` counts the rest |
| `observations` | `needs_attention_total` and `items` (referenced and `historical`, `pruned` or `unavailable`, each with `id`, `label`, at most 3 `referenced_by` and `referenced_by_total`), `referenced_current` and `unreferenced` counts |
| `decisions` | `{"total": N, "items": [{"kind", "text", "reason"}]}`: the 5 newest checkpoint decisions, newest first; see [Decision prefixes](#decision-prefixes) |
| `last_failure` | `{"id", "observed_at_unix_seconds", "summary", "bugs": {"total", "items": [{"id", "status"}]}}` for the failed observation with the greatest recorded `observed_at_unix_seconds` (ties to the greater ID), with at most 3 bugs that link it by their observations, note observations or regression baseline, by bug ID; `null` when no observation failed. No freshness label is computed; run `chough status` or the case for that |
| `lessons` | `{"sources": {"total", "items": [{"module", "path", "section_line", "note"}]}, "archived_bundles": {"total", "items": [<path>]}}`: for each checkpoint module (at most 10, in checkpoint order), `specs/<module>/<module>.3md` when present, otherwise `specs/<module>/context.md` with the 1-based line of its first line that is exactly `## Lessons` (`note` is `null` when a source is found, otherwise `no Lessons section`, `missing`, `larger than 4194304 bytes, not searched` or `cannot be read: <error>`, for example for a symlinked module directory); and the `.specsync/archive/changes/*/lesson-bundle.md` files, newest first by archive directory name, at most 5 (a symlinked archive directory lists none) |
| `environment`, `full_view`, `truncated` | The requested environment, `chough status`, and whether entries were dropped |

The text form (the default) carries the same fields as short lines. Each form is at
most 16384 bytes: every list shows at most 10 entries beside its total, every string
taken from a record is cut to at most 512 bytes, counted as JSON escapes it (a control
character takes six), ending in ` [cut]`, and while the
output is still over the cap the last entry of the longest list is dropped and
`truncated` becomes true (text adds `Output truncated at 16384 bytes; run chough
status.`). The decision items, linked bugs, lesson sources and archived bundle paths are
lists like the others: they are dropped by the same rule, and their totals are counted
before any cut. It never prints raw native output, observation records, the snapshot,
the full reference list or lesson text. The lesson pointer reads only the heading line
of each source and records nothing: no fold state, ledger or index. Every ordering
comes from recorded fields or names (checkpoint order, recorded observation time and
ID, bug ID, archive directory name), never from file modification times or directory
order, so the same records give byte-identical output in both forms.

### Decision prefixes

Checkpoint decisions stay plain strings (checkpoint schema 2 is unchanged). `next`
reads a prefix convention best-effort: an entry that begins with `decided:`,
`rejected:` or `failed:` is shown with that kind, and when it contains ` because `,
the text after the last ` because ` is its reason and the text before it its text, as
in `rejected: a structured decision schema because it needs a migration`. Any other
entry, such as older free text or `2026-10-31: ...`, is shown whole with `kind` and
`reason` `null`, so the newest entries are always shown. Each text and reason is cut
to 512 bytes. Nothing validates the convention, and an unknown prefix is just text.

The next valid action follows these rules in order: a schema 1 session runs `chough
migrate`; a paused stage checks native state and runs `chough stage resume`; open
awaiting-human gates are named and left to the human; open blocking bugs at test or
accept send work back to build; in stage `done`, an archived change whose terminal
evidence authenticates, or no recorded native change, closes the cycle and the action
is `Start a new cycle: chough stage reset --to intent --reason "<why>" (or --to plan
when the approved intent already covers the new work), then specsync change new
"<outcome>" --spec <module> --path <path>, then chough checkpoint --native-change <id>
--next-action "<next step>".`, with the native next action named as the thing to check
first (without the `specsync change new` step when native SDD is not configured or is
disabled), while an archive that does not authenticate keeps its gate refusal; at
intent, design, plan, build, test or accept, when the recorded native change is
archived with terminal evidence that the done gate accepts, the stage is not continued:
the action is `The recorded native change is archived while the checkpoint is at
<stage>, so this cycle's work is closed.` followed by the same new-cycle steps (fixed
text, so `next` never cuts it), the reason opens with `Native change <id> is archived
while the checkpoint is at <stage>`, its command is the reset, and the native next
action is named as the thing to check first; after the suggested reset the checkpoint
still names the archived change until a new one is recorded, so this rule does not
apply while the latest stage transition is a reset; at accept it
first says `Record done first: chough stage advance (accept to done is allowed). Then:`
and its command is `chough stage advance`, the documented close after finalize (release
and side stages are not affected); a closed gate for the next main stage shows its
label and detail; otherwise `Continue <stage>: <checkpoint next action>`. Every reset
Chough suggests carries `--reason`.

## Approved-delta guard

SpecSync 6.1.1 keeps `approval_valid` true after an approved semantic delta is edited
(BUG-0002). So before a move to build, test or accept (by advance, skip, back, reset
or return) on a change in state approved, implementing or verifying with
`approval_valid` true, Chough reads `.specsync/changes/<id>/approvals.json` read-only
within 4 MiB, SpecSync's change-artifact bound, takes the last `definition` approval,
and compares its `approved_delta_digests` with digests of `deltas/<module>.md` for
every module in the status response's `change.affected_specs` (and every approved
module), computed with SpecSync 6.1.1's `specsync.approved-delta.v1` framing: SHA-256
over the length-framed domain, module name and body, the body with CRLF normalized to
LF.

| Label | When | Next native action |
|---|---|---|
| `refused:native-delta-changed` | A delta differs from its approved digest, is deleted, or belongs to a module not in the approval | Restore the approved delta, or ask the human for a new scope approval (`specsync change approve <id> --actor "<their name>"`) and then run `specsync change check <id>` |
| `refused:native-delta-unverifiable` | The ledger is missing, unreadable, over 4 MiB or malformed; it has no definition approval; status reports no `change.affected_specs`; a module name is not one SpecSync 6.1.1 accepts; a delta cannot be read | Run `specsync change status <id>` |
| `refused:native-delta-unverifiable` | A change with deltas has an approval without `approved_delta_digests` | Ask the human for a new scope approval (`specsync change approve <id> --actor "<their name>"`), which records the digests, and then run `specsync change check <id>`; a status read cannot clear it |

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
stales an observation. Each record still stores the HEAD revision, shown in status,
next and the library index and used for regression-baseline ancestry; freshness
compares a working content identity (`snapshot.content`, a SHA-256 digest). It covers
every tracked or untracked non-ignored file except `.chough/`, `target/` and SpecSync
lifecycle bookkeeping: the top-level `state.json`, `approvals.json`,
`verification.json`, `verification-attempts.json`, `review.json`,
`review-attempts.json`, `finalization.json`, `accepted-state.json` and
`lesson-bundle.md` of each `.specsync/changes/<id>/` and
`.specsync/archive/changes/<date>-<id>/`, and the value of the front-matter `state:`
line of its `change.md`. Change artifacts are keyed by change ID and their path in the
change, so finalize moving a change into the archive keeps observations current. A
deleted tracked file counts as absent. So committing only Chough records, an empty
commit, committing edits that were already present, `specsync change check --commit`
and finalize keep observations current; a change to source, tests, docs, `specs/`,
`.specsync/` configuration or a change's deltas and other artifacts makes them stale.
A record written without a content identity, by an earlier Chough, keeps the
revision-bound comparison and stays stale after any new commit, and so does a schema 1
record, whose schema has no content identity, even if one was added to it by hand; a
malformed one is refused. Older Chough binaries refuse records with a content identity as invalid.

A bug reads `reproduced-and-later-passed`, and accepts `bug update --status verified`,
only when a present linked passed observation was observed strictly after its most
recent present linked failed observation (by `observed_at_unix_seconds`). An older
pass, or one in the same second, leaves it `reproduced`; with no linked failure it
reads `unverified` and `verified` is refused.

Environment comparison uses the label you supply; it does not detect changes in the
real environment. A native status response that decodes as JSON is usable output, not
evidence that a lifecycle gate passed.

Kinds: `product-test`, `structural`, `human-observation`, `delivery-observation`,
`publication-observation`, `installation-observation`, `deployment-observation`,
`live-health-observation`, `dry-run`. Human approval and delivery stay unknown in
helper output; an observation is an unauthenticated claim by whoever recorded it.

## Retention

`chough evidence prune` previews by default and changes no file. An observation set is
one evidence record plus the artifacts under `.chough/artifacts/` that no other record
references; artifacts elsewhere in the project are never removed. Each set is eligible
or protected, and the preview lists eligible sets (cases, environment, age, outcome,
bytes, files), protected sets with every reason that applies, totals, the bytes that
would be reclaimed and a warning when evidence and artifacts exceed 50 MiB.

| Protected because | Reason shown |
|---|---|
| Listed in the checkpoint, as an observation or as a `--design` or `--file` path naming the record or one of its artifacts | `checkpoint` |
| Linked by a bug of any status (observation, note or regression baseline), or named by its fixture path | `<bug-id> <status>` |
| Listed by an active native change's acceptance manifest | `native active <id>` |
| Listed by the most recent archived change's manifest (highest finalization timestamp) | `native archive <id>` |
| The native read failed, or its output had no acceptance entries | `native unavailable` |
| Not a routine automated run: a non-test kind, or a method other than `automated` | the kind, the method, or `unspecified-method` |
| Among the newest 3 routine runs for any case it lists, in its environment | `newest` |
| The newest routine failure for any case it lists, in its environment | `last failure` |
| A failure, or the first or last pass, of a stored validation run | `validation <run>` |
| Younger than 14 days | `recent` |

A record bound to several cases counts in the group of each; one with no case forms its
own group per environment. Paths are compared by one key per file: the spelling rebuilt
from its components joined with `/`, with case folded, so `a//b`, `a/b/` and differently
cased spellings of one file count as one artifact. An artifact is removed only when one
record uses it and no bug fixture or checkpoint path names it.

Manifests are read with `specsync change status <id> --json` for each directory in
`.specsync/changes/` and for the latest archive. Without a SpecSync configuration no
manifest protects a set; when any read is unavailable, fails or lacks
`acceptance_entries`, every set stays protected.

`--apply` removes eligible sets only under a run consent (`chough consent grant
retention --source ...`) or the standing preference `retention` (`off`, the default, or
`auto`, set with `chough pref set retention auto --source ...`). With `auto`, `cases
run`, `validate run` and `validate resume` apply the same rules after recording and
report the result in a `retention` field; a failure there is reported, and the recorded
outcome stands. Before each set is removed, one JSON line is appended to
`.chough/pruned.jsonl`: ID, cases, environment, outcome, kind, revision, observed time,
artifact digests, record digest, removed files, reason, policy and authority, HEAD at
pruning and the pruning time. Files leave the working tree only; committed bytes stay
in Git history, and native archives and ignore rules are untouched. The ledger is read
up to 16 MiB; an unreadable ledger leaves absent observations `unavailable`.

Referenced observations carry a label in `status` (`observation_references` and each
entry in `observations`), `next` (the referenced ones that are not current, in
`observations.items`), `bug show` (`observation_labels`) and `validate status` (each
run's `observation_labels`):

| Label | Meaning |
|---|---|
| `current` | Present and its inputs, artifacts and case definition match now (and the requested environment, when one is given) |
| `historical` | Present but stale |
| `pruned` | Absent and listed in `.chough/pruned.jsonl` |
| `unavailable` | Absent and not in the ledger |

Pruned and unavailable observations never count as current, passed or verifying
evidence: bug verification ignores them, `bug update --status verified` needs a present
passed observation, and validation runs keep their recorded counts.

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
the artifact keeps the raw captured bytes. Removed are CSI sequences (`ESC [`,
parameter bytes 0x30 to 0x3F, intermediate bytes 0x20 to 0x2F, one final byte 0x40 to
0x7E), OSC sequences (`ESC ]` ended by BEL or `ESC \` before any line break) and
two-byte sequences of `ESC` and one byte from 0x40 to 0x5F. A sequence that does not
complete under these rules, including one cut by a line break or the end of the
output, is left unchanged. Removal never removes a line break, so it can never lower
the number of `test result:` lines. When anything was removed, the observation carries
the limitation `terminal escape sequences were removed before matching; the artifact
keeps the raw output`.

Changed from earlier releases: an `expect_output` fragment that contains ESC
(U+001B) is now refused by `cases check`, because it could never match the cleaned
text. A case file that matched raw escape codes must match the visible text instead.

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
  that `specsync change check` has not yet materialized: run that check before running
  or recording cases that link it, as the [accept order](../skills/chough-workflow/references/accept.md)
  does first when the delta adds or modifies requirements.

The hint reads neither the checkpoint nor any native delta, and no unresolved link is
accepted.

`bug add` and `bug update` refuse a scope value given as `--classification` with
``; `<value>` is a scope: use --scope <value>``, and a classification given as
`--scope` with the reverse hint; the exit code and the bug record are unchanged.
`--help` for both commands lists the values: scope `blocking`, `in-scope` or
`out-of-scope`; classification `unclassified`, `product-defect`, `regression`,
`environment`, `requirement-gap` or `evidence-gap`.

## Timing

`chough timing` reads existing records and writes none: every file under `.chough/`
and `.specsync/` is byte-identical afterwards. It runs no case and makes no network
request; its only external commands are bounded `specsync change status <id> --json`
reads (above).

| Section | Derived from |
|---|---|
| Stages | `.chough/transitions.jsonl`, read in full up to 16 MiB. Each `advance`, `iterate`, `back`, `reset`, `skip`, `pause`, `resume`, `enter` and `return` record ends the open segment and starts the next; `gate-opened` and `gate-closed` records start or end none. Every `reset` starts a new cycle and `back` never does; the first cycle starts at the first record. The segment before it has an unknown start, because `init` and `migrate` write no transition. Time in `done` until the next reset is idle between cycles. Iterations, side-stage visits and paused time are shown separately; active time is stage time minus waiting on a human, and paused segments are not stage time. The open segment ends now and is `ongoing`. |
| Human gates | `gate-opened` and `gate-closed` records paired by gate text and reconciled with the checkpoint's `awaiting_human`: opened and still listed is open (elapsed so far); opened, never closed and no longer listed was `cleared without a gate record` (close time unknown); listed with no record, or closed with no opening, was `opened before gate records or record missing`. A second `gate-opened` for a gate already open means it was cleared in between without a record: the earlier opening is closed at an unknown time and a new gate starts. A gate whose close is before its open (or that is open with an open time after now) has one wrong time, so every segment it could overlap under either reading has unknown waiting and active time. Overlapping gates are merged, so waiting time is never counted twice; a gate with an unknown end leaves the affected segments' active time unknown. A gate record says when, never who decided or what. |
| Tests | Observations per case and environment: time, outcome and run time. `cases run` records `duration_seconds` in whole seconds, and a run under one second is `recorded together`, not a measured zero; manual records and earlier runs have no run time, shown unknown. |
| Validation runs | Started, completed, failures, not run, recorded attempts and `elapsed_seconds`, labelled total run wall-clock, including intervals and resumes: it is never a per-test time. |
| Bugs | Created; reproduced is the earliest linked failed observation (a pruned one from its retention-ledger entry, and unknown with reason `unavailable` when any linked observation is neither present nor in the ledger); verified is the first note whose changes prefix records `status: <old> -> verified` (labelled `from note text`, since a free note can look the same); fix time is reproduced to verified. |
| Native | The current change (one status read) and the 10 most recent archived changes, by archive directory date and, on the same date, by `finalization.json` timestamp (one read each; older ones are skipped and reported partial). Created and updated come from status. From the change's record files only `approvals[].gate`, `approvals[].timestamp`, `reopenings[].timestamp` and the `timestamp` of `verification.json`, `review.json` and `finalization.json` are read, through the 64 KiB record bound; actor, note, reviewer and every other field are never read. Validity comes only from status: a definition approval is `current` only when status reports `approval_valid` true; an acceptance or finalization approval that a later native reopening supersedes is `reopened` (a reopening never supersedes the definition approval), and one superseded by a later approval of its gate, or that status reports invalid, is `not current`. |
| Estimates | Per stage (stage time per completed cycle), per bug fix and per closed gate: sample count with minimum, median and maximum, labelled `estimate from N recorded <items>; not a target`. Fewer than 3 samples are `too few to estimate`. Idle time, unknown and same-second values and a current cycle that has not reached `done` are never samples; a current cycle that has reached `done` is complete and is a sample. |

Labels: `elapsed wall-clock`; `recorded together` when start and end share a second
(records written together, not instant work); `ongoing`; and `unknown` with its reason
(missing timestamp, end before start, no transition before the first record, a gate
opened before gate records or cleared without one, a linked observation unavailable,
a native time not exposed, a native read `failed`, `timed-out`, `oversized` or
unavailable, an unrecognised record shape, a record over its bound). Unknown values
are `null` in JSON and `unknown` in text, never 0. Times are UTC ISO 8601, with the raw
Unix seconds in JSON. Anything beyond a bound is listed under `partial` with what was
skipped. `--cycle current` shows only the current cycle; estimates always use every
completed cycle.

Gate records also take slots in the recent-transition windows of `chough context`
(20) and `stage show` (10), so those windows can show fewer stage moves than before.

See the [walkthrough](walkthrough.md) and [exercises](exercises/feature-bug-loop.md).
