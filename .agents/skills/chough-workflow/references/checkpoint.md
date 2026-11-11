# Checkpoint and cold resume

Essential state must never live only in model context. The checkpoint is the single
writable record of where work stands: the front matter of the active session's
`.chough/sessions/<id>/session.md` (see [Sessions](#sessions)). It works without the
librarian and without any other storage adapter.

## What it holds

| Field | Content |
|---|---|
| `objective`, `stage`, `stage_status`, `iteration`, `callers` | Current outcome, stage, status, side-stage callers and iteration |
| `native_change` | The SpecSync change ID; native status decides its state |
| `criteria`, `requirements`, `cases`, `design`, `files` | What is in scope |
| `observations`, `bugs`, `blockers` | The records that show the work, and what blocks it |
| `waiting_on_human` | Real gates still waiting for a person, each with the stage it holds |
| `run_consents` | Consent for this objective only |
| `next` | Suggested next step; never overrides native state |

These are the front matter of `.chough/sessions/<id>/session.md`. Decisions with
their reasons, completed work and open questions live in its Decisions and why, Where
it stands and Open threads sections, and transitions in its Timeline. Standing
preferences are in `.chough/preferences.json`. `.chough/local/active-session` names the
active session; `.chough/local/` stays in each worktree and is never committed.
Observations are in `.chough/evidence/`, bugs in
`.chough/bugs/`, case definitions in `.chough/cases/` and retention removals in
`.chough/pruned.jsonl`.

## When to save

Save after each meaningful increment, before handing off, before context is cleared,
and whenever a human gate opens or closes:

```sh
chough checkpoint --completed "<what was done>" --observation <id> \
  --decision "<decision, because reason>" --next-action "<next step>"
```

Opening a gate with `--awaiting-human "<gate>"` appends one `gate-opened` entry to the
session's Timeline, only when that gate is not already open; `--close-gate` and `--clear-awaiting-human` append one `gate-closed` record per
gate they close. They record when a gate waited, and `chough timing` reads them to
show waiting-on-human time. Open and close gates through these flags rather than by
editing the checkpoint file, or the waiting time is lost.

When the human decides a gate, record their decision on their behalf with
`chough checkpoint --close-gate "<gate>" --decided-by "<who>" --words "<their exact
words>" --where "<where and when, and who relayed it>" --recorded-by "<agent>"`. It
closes only that gate, puts `relayed_by` on its `gate-closed` record and adds one
decision. It is a relayed claim, not authenticated, and never a native approval or
review. Each refusal saves nothing and names its recovery.

`--clear-awaiting-human` withdraws every open gate and records no decision: use it
for gates no human decision closes. To withdraw some gates and keep the others, pass
the gates still waiting with `--awaiting-human` in the same call; they are logged
closed and opened again, so their waiting time restarts there.

Write decisions with a prefix and a reason, so a fresh agent can tell what was chosen,
what was ruled out and what already failed:

```text
decided: <what> because <reason>
rejected: <approach> because <reason>
failed: <experiment> because <reason>
```

This is a convention, read best-effort: the prefix starts the entry, lowercase, with a
colon, and the reason is the text after the last ` because `. Any other entry, such as
older free text or a dated note, is still kept and shown whole. Decisions stay plain
strings in the checkpoint; nothing validates the convention. Record a human's decision
only in their words, and label an agent's own assumption as the agent's, without a
`decided:` prefix.

Write decision summaries and observations, not private reasoning transcripts.
SpecSync's `Handoff:` line says whether its own records are safe to clear; follow it
too.

## Resume cold

A fresh agent with none of the earlier conversation:

1. Reads `AGENTS.md`, then this skill.
2. Runs `chough next --environment <env>` (or reads the files above directly); it is
   the focused view, and `chough status` is the full one. Besides the stage, native
   state, next action and gates, it shows the 5 newest decisions with their kind and
   reason, the latest failed observation by recorded time with the status of each bug
   that links it, and where each module's lessons live (`<module>.3md`, or the
   `## Lessons` line of `context.md`) with the newest archived SpecSync lesson
   bundles. It points to lessons and never prints them; read the source it names.
3. Checks native status for `native_change`, freshness of the referenced observations,
   open blocking bugs and `awaiting_human`. A referenced observation is `current`,
   `historical` (present but stale), `pruned` (removed by retention, in the ledger) or
   `unavailable` (missing, not in the ledger); only a current one counts, so rerun the
   others rather than treat them as passed.
4. Continues with `next_valid_action` under the same scope, preferences and gates. It
   never records a pending human decision itself. In stage done with the native
   change archived, the cycle is closed: `chough next` recommends `chough stage reset
   --to intent --reason "<why>"` (or `--to plan` when the approved intent already
   covers the new work), then `chough change new "<outcome>" --spec <module> --path
   <path>` and `chough checkpoint --native-change <id> --next-action "<next step>"`.
   The same holds at an earlier stage when the native change is already archived:
   `chough next` names the archive and the new cycle instead of continuing, after
   `chough stage advance` to done at accept. A reset replaces the checkpoint's next
   action.

## Schema and concurrency

The checkpoint is the front matter of the active session's `session.md`; the story
sections and the Timeline are in the same file. Hand edits are allowed: Chough keeps
any key it does not know and every line a person wrote outside the sections it
appends to, and refuses front matter outside its strict YAML subset with the file,
line and rule. With several agents, one agent owns live checkpoint writes; others
report through the inbox. A 0.5.x project (`.chough/session.json`, `history/`,
`transitions.jsonl`) is refused as unmigrated until `chough migrate` converts it.

There is no checkpoint merge. Each session is its own folder, so two branches that
work on different sessions merge with no conflict. Two branches that change the same
session conflict in Git like any file; resolve by keeping both Timelines' entries
and one coherent checkpoint, never by inventing a decision.

## Sessions

Each piece of work is a session folder, `.chough/sessions/<date>-<slug>/`, holding
`session.md`, `evidence/` and `agent/`. Start one with `chough session new <key>`; the
key is kept as an alias, and `chough session use <id, alias or slug>` switches. A
session only ever closes itself: at its own move into done it writes one story,
`.chough/archive/sessions/<id>.md`, removes each file of its folder whose bytes are
committed at HEAD, keeps and reports the rest, and clears the active pointer; then
start the next one with `chough session new`. It stays open while its inbox has
unfolded entries or release is authorized (ask for release before done). Starting or
switching work never archives, closes or changes another session, even a finished
one; `chough session list`, `chough next` and `chough doctor` name each finished open
session with its fix, `chough doctor --fix`. Close abandoned work with `chough session
close <key> --reason <text>`. After the move into done, commit the story, then run
`chough doctor --fix` to remove the committed folder and commit that. When a fresh
worktree refuses to enter Chough, run `chough doctor` to see the debt and
`chough doctor --fix` to clear it; decisions also need `--reason`. Subagents do not
edit the session: they run `chough inbox add --session <key> --from <name> --kind
observation|decision|completed|question --text <text>`, and the agent that owns the
session runs `chough inbox fold`. Inbox entries never approve, review, consent, clear a
gate or move a stage.

## The Timeline

Every transition is one entry of the session's `## Timeline`, `- <time> · <kind> ·
<from> → <to> · <text> ^t-xxxxxx`, with its fields as indented lines, written in the
same save as the checkpoint change it goes with. Chough only appends entries and
never edits one; `chough history show` lists them. Recent guidance reads at most the
newest 128 entries. A timeline holds at most 4,096 entries of 64 KiB each; a refusal
at that bound leaves the session unchanged and names `chough session close` and
`chough session new`. An entry that is not a valid transition is refused, never
shown as history.
