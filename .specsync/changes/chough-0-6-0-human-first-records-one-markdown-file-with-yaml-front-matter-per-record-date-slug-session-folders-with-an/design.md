---
change: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an
artifact: design
---

# Design

The approved prototype (`design/prototype/index.html` with `COMPANION.md`) is the
source for the tree and the look of each record. This page says how Chough reads
and writes it. The deltas are the contract; where this page and a delta differ,
the delta wins.

## The tree

```text
.chough/
  README.md                                  one-screen guide (create-only)
  preferences.json                           unchanged (standing preferences)
  cases/*.cases.toml                         unchanged (case definitions)
  sessions/<date-slug>/                      one folder per open session
    session.md                               checkpoint (front matter) + story + Timeline
    evidence/<date-slug>.md                  one receipt per observation
    evidence/<date-slug>.png                 at most 5 small key screenshots per session
    agent/                                   agent working files, inbox/ included
  archive/sessions/<date-slug>.md            one story per finished session, kept forever
  archive/sessions/<date-slug>/              only when the session had screenshots
  archive/shelved/<id>/                      unreadable sessions, moved byte for byte
  archive/migrations/<date>-from-0-5.md      migration receipt
  bugs/<date-slug>.md                        one file per bug, append-only Log
  local/                                     Git-ignored: pointer, lock, logs/, external-artifacts.toml,
                                             identity.toml, gate lists, direct-write baselines
knowledge/
  index.md                                   reading guide, generated
  journal/<date-slug>.md                     short posts, written once, never rewritten by Chough
  journal/feed.xml                           Atom feed, generated
  lessons/<date-slug>.md                     lessons that span projects
  library/, briefs/                          pre-0.6.0 content, kept as is
```

Gone: `session.json`, `session.v1.json`, `transitions.jsonl`, `history/`,
`sessions/*.json`, `sessions/archive/`, `sessions/shelved/`, `evidence/*.json`,
`bugs/*.json`, `artifacts/` (except files an open bug names), `pruned.jsonl`, and in
`knowledge/`: `INDEX.md`, `README.md`, `rules.md`, `timeline.md`, `timeline/`,
`intent/`, `archives/`, `modules/`, `bugs/`, `sessions/`, `attachments/`.

## Record files

One reader and one writer, in a new `src/markdown.rs`, serve every record kind.

- Front matter is a strict YAML subset: scalars (plain or double-quoted), flow lists,
  block lists of scalars, block lists of flat maps. No new crate: the parser is about
  300 lines and refuses anything outside the subset with file, line and rule.
- Tolerant reading: unknown keys are kept in place on rewrite; missing optional keys
  take defaults; CRLF reads as LF.
- Chough owns the front matter and the list sections it appends to. Everything else
  in the body belongs to whoever wrote it, and a write never changes it. In practice a
  write is: replace the front matter, append list items to the end of a named section
  (creating the section at the end of the body if missing), or create a new file.
- Atomic replace under the project writer lock; create-new for new files.

### A session file

```markdown
---
kind: session
id: 2026-11-10-human-first-records-0-6-0
aliases: [human-first-records-0-6-0]
objective: Ship human-first Chough records as one change, Chough 0.6.0
stage: plan
stage_status: active
iteration: 1
native_change: chough-0-6-0-human-first-records-...
pr: chough#238
criteria: [DRIVE-24, DRIVE-25]
waiting_on_human:
  - gate: Scope approval of the Chough 0.6.0 native change
    holds: plan
    drives: approve
    opened: 2026-11-11T03:10:00Z
next: Wait for Leif's scope decision, then build
---
# Ship human-first Chough records as one change, Chough 0.6.0

## Goal
...written once from the objective; people may rewrite it...

## Where it stands
- Native change drafted with deltas for cli, history, reporting and records.

## Decisions and why
- Leif decided "Keep all (Recommended)" for archived stories ... [gate](#^t-3f9a2c)

## What was verified
- [Archive validity survives removing vault copies](evidence/2026-11-10-archive-validity-probe.md)

## Open threads
- Gate scoping (#237): option A or B?

## Timeline
- 2026-11-11T02:09:36Z · advance · design → plan · Leif approved the design ^t-8a01c4
  - by: claude-code (claude-opus-5-5, t370)
- 2026-11-11T03:10:00Z · gate-opened · plan → plan · Scope approval of the Chough 0.6.0 native change ^t-3f9a2c
  - holds: plan
  - drives: approve
```

The Timeline grammar is fixed so agents can parse it and people can still read it:
`- <ISO time> · <kind> · <from> → <to> · <text> ^<entry id>`, then indented
`- key: value` lines. The `^t-xxxxxx` suffix is an Obsidian block ID, so
`session.md#^t-3f9a2c` links to one entry in Obsidian, and on GitHub it reads as a
short tag at the end of the line.

### An evidence entry

As in the prototype: front matter with `claim`, `result`, `command`, `revision`,
`content`, `environment`, `fingerprint`, `raw_log`, `criteria`, `requirements` and
the rest (REQ-cli-017), then the claim in bold and one sentence. A failed run adds a
fenced block with at most 20 key failing lines (2 KiB), escape codes removed and any
leak redacted. The full log is in `.chough/local/logs/<id>.log` and in CI.

### A bug

As in the prototype: `# title`, What happened, Steps, Expected, Why it matters, Fix,
then `## Log`, which is append-only. Bug merge (the existing `chough bug merge`
driver) learns Markdown: field-wise front matter, union of Log lines, one-sided body
edits taken, two-sided edits left for a human.

## Names

`<YYYY-MM-DD>-<slug>`, UTC date of creation, slug normalized like today's bug slug,
whole name at most 80 bytes (keeps Windows paths short under WSL). A same-folder
clash at creation adds `-` and 6 random hex digits; a second clash is refused,
never retried. Old IDs move into `aliases`, and every lookup resolves `id` or alias.
Names never change after creation.

## Finishing a session

A session only ever closes itself (Q3, #207): at its own move into done, with
`chough session close --reason`, or by the explicit `chough doctor --fix`, which
`next` names for a finished session that is still open. `session new` and
`session use` never archive another session. The steps:

1. Build the story in memory: the session's front matter with `stage: done`,
   `closed`, `outcome`, and `receipts` (one map per evidence entry, carrying every
   field freshness needs); the body as written; a What was verified line for each
   receipt not already listed; the Timeline; a closing line.
2. Write `.chough/archive/sessions/<id>.md` by create-new and read it back.
3. Move screenshots beside the story.
4. Remove the session folder, each file only when its exact bytes are committed at
   HEAD. Anything else stays and is reported; the folder counts as archived because
   its story exists; `chough doctor --fix` removes it after the commit.
5. Write the journal post (create-new), then regenerate `index.md` and `feed.xml`.

Nothing in this sequence reads, moves or rewrites another session's folder or story.
That is the fix for issue 165: the 0.5.0 roll-up moved older archived sessions into
`knowledge/attachments/`, which looked like a deletion wherever `knowledge/` is not
committed. There is no roll-up in 0.6.0, and no count or age bound on the archive
(Leif: "Keep all").

In practice the move into done is followed by one commit (story, post, index) and a
second, tiny cleanup commit that removes the committed session folder. The skill's
closeout says so.

## Merging generated files

`knowledge/index.md` and `journal/feed.xml` are generated. A union merge of two Atom
feeds can leave invalid XML in a merge commit, so they get a `chough-generated`
merge driver (`chough library merge`) that keeps ours byte for byte and exits 0; the
after-merge `chough library index` step and the verify lane's `--check` rebuild and
check them from the merged sources. Journal posts, bugs and stories are separate
files, so they merge as files.

## Gates (issue 237)

The deltas take option A, the recommended default the coordinator planned (audit
Q5: a pending decision holds only its own step, and all are still listed). Leif can
still choose B at scope approval, which means rewriting REQ-cli-030 before approval
and nothing else.

- **Option A (recommended): a gate holds a stage.** `--awaiting-human "<gate>"
  --holds <stage>`, defaulting to the current stage, so today's habit (open a gate
  at the stage it blocks) behaves exactly as now. A gate for the crates.io publish
  holds `release`, so build, test and accept keep moving and only leaving release
  waits. Every open gate is still listed by `next`, `status` and `gate list`.
- **Option B: gates stay global, plus parking.** Any open gate still refuses every
  advance, and a new `--park` records a later decision that never blocks until it is
  unparked. Simpler to explain, but the parked decision is easy to forget, and
  someone still has to remember to park.

Recommendation: A, because a gate should block only the decision it is about, and
the stage is already the unit Chough gates on.

## A human's gate approval drives SpecSync as that human

Identity comes from configuration, never from code:

```toml
# ~/.config/chough/identity.toml (or .chough/local/identity.toml)
[[human]]
name = "Leif"            # how records show the person
aliases = ["0xLeif"]
specsync_actor = "0xLeif"
```

Each gate records `drives`: `approve` (default at plan), `review` (default at
accept) or `none`. Closing a driving gate with `--decided-by Leif --words ... --where
... --recorded-by ...` runs `specsync change approve <id> --actor 0xLeif --note "<words>
(<where>); relayed by <agent>; recorded with chough"` first, or for an acceptance
gate `specsync change review <id> --reviewer 0xLeif --verdict <pass|block>` with
the verdict given as `--verdict` (never read from the words), and only then records
the gate closure, the relay and the decision. A decision that is neither, such as
"change this first", is recorded with `--no-native`, which runs nothing. A refusal
passes through and records nothing. An unknown or ambiguous `--decided-by` is refused before
anything runs. The example above is the shape of a config file; no name is built in.

Only a decision made at that gate is in the human's name. When an agent goes ahead
on permission the human gave in advance, without the human answering that gate, the
SpecSync approval names the agent and quotes the human's earlier words and where
they said them (Leif: "The agent's (Recommended)", FLOW-24), so no record says a
person looked at something they did not. Leif's standing rule of 2026-11-10 sets the
form: the actor is `<agent>-<terminal>` (for example `kimi-code-t37b`), as `--actor`
on an approval and `--reviewer` on a review, and the note reads like "Approved under
Leif's standing bug-fix rule (coordinator chat, 2026-11-10): <its words>. Recorded by
<agent> (<model>, hc <terminal>)." If SpecSync or the project's configuration refuses
that actor, the agent stops and says so; it never retries as the human, and archived
approvals are not edited. The skill's plan, accept and lifecycle
references say which case applies.

## No leaks

`src/leaks.rs` holds the patterns (home and system paths, `~/`, host name,
`user@host`, emails, token and key forms, credential assignments). Every committed
write runs them over every value: refuse by default, `--redact` to replace. Automatic
key failing lines are redacted, never refused, so a panic path cannot block a test
record. `chough doctor` scans tracked files under `.chough/` and `knowledge/`; the
verify lane gains a `leaks` step that runs it. Messages name the field and kind,
never the text. Git history keeps whatever leaked before 0.6.0; the migration
cleans the current tree and the receipt says how many values it redacted.

## Proof files (Leif: "Split them")

- Logs: never committed; `.chough/local/logs/` locally, CI artifacts in CI.
- The record keeps the exact command, the output's SHA-256, and for a failure the key
  failing lines.
- Screenshots, proposed for Leif to confirm: PNG or JPEG, metadata chunks stripped,
  at most 512 KiB each, 5 per session, 2 MiB per session; committed beside the entry
  while open and beside the story when finished. Larger images stay external by
  fingerprint.
- Other external files: fingerprint only, path in `.chough/local/external-artifacts.toml`.

## Migration

`chough migrate` (preview, writes nothing, ends with the plan's digest) and
`chough migrate --apply <digest>`, which applies only the previewed plan:

1. Inventory every 0.5.x file with size and SHA-256. Refuse if any file it would
   remove is uncommitted or differs from HEAD, so Git holds every removed byte.
2. Build the whole 0.6.0 tree in memory: sessions (timelines from the selected
   history lineage, legacy log entries into the adopting session), archived stories
   (including sessions the old roll-up moved into `knowledge/attachments/`), evidence
   or receipts for every referenced observation, Markdown bugs, inbox entries, the
   old IDs as aliases, gates holding the stage their session was at, small
   screenshots kept beside their story. Run the leak guard with redaction.
3. Write every new file create-new; an existing different file stops the apply.
   Read each back.
4. Write the receipt: before and after (path, size, SHA-256), redactions, removed
   files with why and the commit that holds them, and the old-to-new ID map.
5. Read the native validity of every archived change whose acceptance manifest
   lists a file to remove (at most 8 at once), remove the 0.5.x files whose bytes
   still match, read validity again, and restore everything and refuse if any
   archive that was valid no longer is. The old trims' two safety rules, preview
   first and never weaken a SpecSync archive, live on here.

Every rewritten file passes the leak guard with redaction, so the home-folder paths
the audit found in 96 committed files do not survive; a file kept byte for byte that
holds one is reported as a leak finding. Git history is not rewritten (Leif: "No,
clean from 0.6.0 on"); the receipt says so. Fixtures from every release, 0.1.0 to
0.5.0, prove the path (#213).

A rerun after an interruption finishes the job; a rerun after success reports
nothing to do. IDs that were removed (unreferenced old observations) resolve as
`removed`, naming the receipt, never as passed.

Every other command refuses on an unmigrated or mixed tree with `refused:unmigrated`
and names `chough migrate`. `chough doctor` reports what is left. An older Chough on
a migrated tree finds no checkpoint; the CHANGELOG tells every agent to upgrade
before a project migrates, and `doctor` catches a stray 0.5.x file written later.

## Code layout

New, owned by the `records` module: `src/markdown.rs` (front matter and sections),
`src/leaks.rs`, `src/local.rs` (local store, external artifacts, logs, screenshots),
`src/humans.rs` (human identity configuration and the actor),
`src/migrate.rs`, `src/journal.rs`; `src/identity.rs` (today's slugs and random
draws) moves from `cli` to `records` and becomes the date and slug naming with
clash suffixes and aliases. Tests: `tests/records.rs`, `tests/migrate.rs`,
`tests/leaks.rs`, `tests/journal.rs`, `tests/approval.rs`, and `tests/ids.rs`
moves with `identity.rs`.

Rewritten on the new format: `checkpoint.rs`, `sessions.rs`, `history.rs` (now the
timeline), `stage.rs`, `gates.rs`, `evidence.rs`, `cases.rs` (recording only),
`bugs.rs`, `records.rs`, `library.rs` and `vault.rs` (much smaller), `timing.rs`
(reads timelines), `doctor.rs`, `context.rs`, `main.rs`.

Deleted code paths: history events and cursors, legacy adoption, history prune,
evidence prune and the ledger, library archive and snapshot, the post-archive trim,
attachments, roll-up, session notes, archive notes, module/intent/rules/timeline
notes, checkpoint merge, schema 1 migration.

## Bug fixes

testing.md lists every open Chough bug with its fix and regression case. The
design choices among them: gates hold a stage (#237, above); gate summaries count
from the merge base with the default branch and leave out Chough's records, the
knowledge directory and SpecSync bookkeeping (#190); `next` gains the init, test and
acceptance guidance and `status` leads with the `next` summary (#76, BUG-0023); a
`toolchain` Fledge task checks local tool versions against the CI pins (BUG-0022);
the delta guard follows SpecSync's effective approval (BUG-0020); the timing budget
starts at the first native read; the writer lock records its holder's process ID so
a busy refusal can name it (#132).

## Commands that go away

`chough checkpoint --merge`, `chough history adopt`, `chough history prune`,
`chough evidence prune`, `chough library archive`, `chough library snapshot`,
`chough session adopt`, `chough session shelve --legacy`, and the schema-1 meaning
of `chough migrate`. New: `chough correct`, `chough journal add`,
`chough evidence add --screenshot / --failing-lines`, `--holds` and `--drives` on
gates, `--verdict` and `--no-native` when closing one, `--redact` on writes,
`chough doctor --fix --redact`.
