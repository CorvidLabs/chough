# Checkpoint and cold resume

Essential state must never live only in model context. `.chough/session.json` is the
single writable checkpoint. It works without the librarian and without any other
storage adapter.

## What it holds

| Field | Content |
|---|---|
| `objective`, `stage` | Current outcome, stage, status, side-stage callers and iteration |
| `native_change` | The SpecSync change ID; native status decides its state |
| `references` | Criteria, requirements, cases, design sources and files in scope |
| `decisions`, `open_questions` | Decisions with their reason in the sentence; questions that matter |
| `completed`, `observations`, `bugs`, `blockers` | Work done and the records that show it |
| `awaiting_human` | Real gates still waiting for a person |
| `run_consents` | Consent for this objective only |
| `next_action` | Suggested next step; never overrides native state |

Standing preferences are in `.chough/preferences.json`, transitions in
`.chough/transitions.jsonl`, observations in `.chough/evidence/`, bugs in
`.chough/bugs/`, case definitions in `.chough/cases/` and retention removals in
`.chough/pruned.jsonl`.

## When to save

Save after each meaningful increment, before handing off, before context is cleared,
and whenever a human gate opens or closes:

```sh
chough checkpoint --completed "<what was done>" --observation <id> \
  --decision "<decision, because reason>" --next-action "<next step>"
```

Opening a gate with `--awaiting-human "<gate>"` appends one `gate-opened` record to
`.chough/transitions.jsonl` (only when that gate is not already open), and
`--clear-awaiting-human` appends one `gate-closed` record per gate it clears. They
record when a gate waited, never who decided or what, and `chough timing` reads them
to show waiting-on-human time. Open and clear gates through these flags rather than by
editing `session.json`, or the waiting time is lost.

`--clear-awaiting-human` clears every open gate, not just one. To close some gates
and keep the others, pass the gates still waiting with `--awaiting-human` in the same
call; they are logged closed and opened again, so their waiting time restarts there.

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
   covers the new work), then `specsync change new "<outcome>" --spec <module> --path
   <path>` and `chough checkpoint --native-change <id> --next-action "<next step>"`.
   The same holds at an earlier stage when the native change is already archived:
   `chough next` names the archive and the new cycle instead of continuing, after
   `chough stage advance` to done at accept. A reset replaces the checkpoint's next
   action.

## Schema and concurrency

A schema 1 session keeps working for `context` and `status`. `chough migrate`
converts it after saving the original bytes to `.chough/session.v1.json`. Hand edits
are allowed and validated on every read. With several agents, one agent owns the
checkpoint; others report to it. APS was evaluated as a backend and deferred: it lost
concurrent read-modify-write updates without error and accepts unknown fields.
