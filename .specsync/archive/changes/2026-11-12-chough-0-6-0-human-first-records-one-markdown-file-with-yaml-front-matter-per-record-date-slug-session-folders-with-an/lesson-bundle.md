# Lesson bundle: chough-0-6-0-human-first-records-one-markdown-file-with-yaml-front-matter-per-record-date-slug-session-folders-with-an

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Chough 0.6.0 human-first records: one Markdown file with YAML front matter per record, date+slug session folders with an append-only timeline that fold into one archived story at finish, a knowledge vault that links and never copies, no local facts or credentials in committed records, a human's gate approval drives the SpecSync approval as that human, and chough migrate converts 0.5.x projects with a digest receipt
- **Kind**: Feature
- **Specs**: cli, history, reporting, records
- **Paths**: src/, tests/, skills/, specs/, docs/, .chough/cases/, .chough/README.md, knowledge/, .github/, .agents/, examples/, hi/, README.md, CHANGELOG.md, AGENTS.md, CLAUDE.md, INTENT.md, .gitattributes, .gitignore, Cargo.toml, Cargo.lock, fledge.toml, .specsync/sdd.json, .specsync/config.toml
- **Acceptance**: Records (DRIVE-25, DRIVE-26): every session, evidence entry and bug Chough writes is one Markdown file with YAML front matter, decisions are entries in their session's file, and no JSON twin exists. After chough migrate --apply on a 0.5.x fixture and on this repository, .chough/ holds no session.json, transitions.jsonl, history/, sessions/*.json, evidence/*.json or bugs/*.json, and every command works from the Markdown files alone
- **Acceptance**: Sessions (DRIVE-24, DRIVE-25): each session is .chough/sessions/<date-slug>/session.md, whose front matter carries the checkpoint and whose append-only Timeline section carries every stage move, gate and correction, with an agent/ folder for working files. Two branches that advance different sessions merge with no conflict, and a session only ever closes itself, never on another session's switch (#207). At finish the session becomes one .chough/archive/sessions/<date-slug>.md story with its evidence receipts folded into What was verified, and its folder, agent/ included, leaves the tree
- **Acceptance**: Names (DRIVE-27): new records are named YYYY-MM-DD-<slug>, never by a sequence number; a same-day clash gets a short suffix at creation; every 0.5.x ID keeps resolving as an alias in links, bug commands, observation, case and checkpoint references
- **Acceptance**: Guides (DRIVE-28): .chough/README.md and knowledge/index.md orient a new reader, and case definitions stay TOML with their format unchanged
- **Acceptance**: Vault (KNOW-30 to KNOW-33): knowledge/ holds only index.md, journal/ posts with a generated feed.xml, lessons/ for lessons that span projects, and pre-existing library/ and briefs/ content as is. knowledge/intent/, knowledge/archives/, attachments and generated session, bug and module notes are gone, no Chough step copies or moves a tracked record into the vault, the move into done writes a short journal post that links the archived story, and library index --check passes
- **Acceptance**: No leaks (LIFE-26, FLOW-22, issues 234 and 208): a write whose text holds a home path, the local user or host name, a token, key or credential is refused, or redacted with --redact, naming the field and the kind but never the value. External evidence is recorded by a neutral id and its fingerprint, with its path only in the git-ignored .chough/local/. chough doctor and the verify lane scan committed records and fail on any finding
- **Acceptance**: Approval as the human (FLOW-23, FLOW-24, issue 236): closing a scope or acceptance gate with a human's decision records the matching SpecSync approval or review as that human, whose identity comes from configuration or the approving channel, with their exact words, where and when, and the relaying agent; with no identity configured for that human it refuses rather than guess; an approval on authority the human gave in advance names the agent and quotes the human's earlier words and where
- **Acceptance**: Who and corrections (LIFE-17, LIFE-18, PRs 233 and 210): each timeline entry, evidence entry and bug records the acting agent, model and hc terminal when known and never fails when they are unknown; chough correct appends a correction linked to the original entry, also on an archived story, and next and status show the corrected text
- **Acceptance**: Data loss regression (issue 165): with earlier archived sessions present and knowledge/ not committed, advancing accept to done leaves every earlier archived session file byte-identical in place and adds only the new story. The 0.5.x reproduction is recorded failing first and linked to a local bug
- **Acceptance**: Migration (DRIVE-27, LIFE-20): chough migrate previews by default, writes nothing and prints the plan's digest, and --apply runs only that previewed plan, only with the human's consent and never while a native change is open (KNOW-10 and KNOW-11, carried forward); --apply writes the 0.6.0 tree create-new only, never replacing a file, removes the 0.5.x files only after every new file is written and read back, writes a receipt with the digest of every file before and after, redacts home-folder paths from every file it rewrites, never weakens a SpecSync archive, and is tested on copies from every release 0.1.0 to 0.5.0. A second apply reports nothing to do, the 0.6.0 reader refuses an unmigrated tree with refused:unmigrated naming chough migrate, and chough doctor lists what is left
- **Acceptance**: Gates (issue 237): an open human gate holds only the stage it names, so a gate for a later decision such as a release no longer refuses unrelated stage moves, and chough next still lists every open gate
- **Acceptance**: Bugs: every open Chough bug is fixed with a regression case, including Chough's own records staying out of SpecSync's verification (issue 12), or closed with a link when it was already fixed elsewhere (BUG-0002 and BUG-0003 as fixed upstream), and the criteria #212 marks superseded are retired with hi retire, with KNOW-24 reworded, in the commits that remove what implemented them
- **Acceptance**: Dogfood: this whole change runs on the 0.5.0 build; after it is archived, with no change open and Leif's consent, this repository is migrated with the 0.6.0 build, and the next change on this branch, the 0.6.0 release preparation, runs intent to done in the new format; fmt, lint, the full tests, the cases this change adds and fledge lanes run verify pass on macOS and Linux; no other repository changes; no 0.6.0 tag or release until that dogfood is complete and no Chough bug is open, and the release still waits for Leif's go

## Evidence

- Verification commit: `8421201d62da1748427a4e863d67e5ba01d41201`
- Base commit: `50eb4f5f5c2d571c1b4465292b36449baf3a10e0`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

## Proposal, in plain words

Chough 0.6.0 makes Chough's own records readable by people, not only by agents.
Every piece of work, bug and test result becomes one Markdown file, with a short
header that agents read and plain sentences that people read, named by its date and
a short title instead of a number, so parallel branches stop fighting over IDs. Each
piece of work gets its own folder while it is in progress, and when it finishes it
becomes one short story file that stays in the archive for good. The knowledge
folder stops copying things and only links to them, plus a short journal post and a
feed for each finished piece of work. Committed records never hold local paths,
user names or credentials: logs go to CI, the record keeps the command, the key
failing lines and a fingerprint, and a few small screenshots can be committed beside
a story. When Leif approves or accepts at a Chough gate, Chough records the matching
SpecSync approval in Leif's name, from an identity in configuration rather than one
built into the code. A one-time `chough migrate` converts an existing project, keeps
a receipt of every file before and after, and removes nothing Git does not still
hold. It also closes a real data-loss bug where finishing one session deleted an
older archived one. Everything ships together as Chough 0.6.0 and is tried on the
chough repository before any other project changes.

The same release fixes every open Chough bug: a pending human decision no longer
holds up unrelated steps, Chough's records and SpecSync's checks stop making each
other look out of date, review summaries count only the change's own files, `chough
next` guides a new agent from start to acceptance and `chough status` leads with a
short summary, version notes and local tools are checked against CI, a leftover
empty folder no longer looks like open work, and the intermittent test failures get
fixes or, where they will not reproduce, the diagnostics to catch them. Two bugs that
were really SpecSync's are already fixed there, and are closed as fixed upstream.

## What led here

- Leif's 2026-11-10 interview and design review (issues #235, #234, #236, #214),
  captured as hi criteria in #212: DRIVE-24 to DRIVE-28, KNOW-30 to KNOW-33, FLOW-21
  to FLOW-23, LIFE-17 to LIFE-26. Intent approved by Leif ("Approve it"); the design
  prototype approved by Leif ("Approve the design"). Both are recorded on this
  session as relayed decisions.
- Three safeguards of those nine criteria are kept here as spec rules, as the
  coordinator asked: only committed bytes ever leave the tree (REQ-records-012, from
  KNOW-7), `knowledge/index.md` links every lesson, per module and cross-project
  (REQ-cli-043, from KNOW-25), and the journal and feed are the timeline of
  archived changes (REQ-records-011, REQ-cli-043, from KNOW-28).
- Leif widened 0.6.0 to fix every open Chough bug (coordinator chat, 2026-11-10: he
  picked all four groups offered and added "all bugs?"). testing.md lists each with
  its fix and regression case. BUG-0002 and BUG-0003 describe SpecSync's behaviour; a
  spec-sync worker found neither reproduces on spec-sync main 46f55652 (6.2.0), so
  0.6.0 closes both as fixed upstream, citing that check.
- #212 marks the old cleanup and vault-copy criteria as superseded but does not retire
  them, because live requirements and cases still depend on them. This change retires
  them all with `hi retire`, rewords KNOW-24, and removes or rewrites what implements
  them; plan.md maps every affected requirement, criterion and case. As merged
  (88e53eba), #212 marks 18 superseded and carries KNOW-10 and KNOW-11 forward as the
  upgrade's rules: never weaken a SpecSync archive; preview first, apply only with
  consent and never while a change is open. Those two stay and trace to the migration.
- Leif's two later answers, relayed by the coordinator on 2026-11-10 and captured
  in #212 as KNOW-34 and KNOW-35: "Keep all (Recommended)" for finished-session stories
  (nothing trims the archive) and "Split them (Recommended)" for proof files (logs to
  CI; the record keeps key failing lines, the command and a fingerprint; a few small
  key screenshots are committed beside the story with a size cap).
- Shipping decision (Leif, 2026-11-10): one native change, released as 0.6.0,
  dogfooded on the chough repository only; other repositories update only after
  0.6.0 is released at 95 percent confidence or better.
- Windows is best-effort through WSL (FLOW-17 read with FLOW-21).

## Folded in

- PR #233 (actor identity, LIFE-17): REQ-records-007. Its REQ-cli-063 never landed;
  the PR closes as folded in when this change lands.
- PR #210 (append-only corrections, LIFE-18): REQ-history-015. Its REQ-history-015
  draft never landed; this change takes the ID. The PR closes as folded in.
- PR #208 (external artifacts by fingerprint, FLOW-22): REQ-records-005, changed so
  the committed record holds a neutral ID and fingerprint, never the absolute path
  (#234). The PR closes as folded in.
- PR #211 (#165 copy-never-move): closes as superseded by KNOW-30 when this change
  lands. It is not closed now.
- Issue #165 has a new real repro (algochat, 2026-11-10): accept to done deleted an
  earlier archived session with no prune record. REQ-history-006 carries the
  regression for that exact path, recorded failing first on 0.5.0.
- Issue #237 (a gate blocks every advance): decided at scope approval, see below.
- Issue #12 has two new real repros, relayed by the coordinator: an untracked Chough
  evidence receipt present at `check --commit` gave a reviewer's clean checkout stale
  verification (spec-sync #1045, 2026-11-11), and checkpoints written after `check
  --commit` staled verification repeatedly on #212. REQ-records-014 keeps Chough's
  records in three folders a project lists in SpecSync's documented exclusion, with a
  regression case for each repro (DRIVE-4).

## Leif's decisions, and the defaults he can still change

Answered by Leif on 2026-11-10 (coordinator chat, relayed; each is on the session):

- Fix every open Chough bug in 0.6.0: "all bugs?".
- Retire the rest of the old cleanup rules and fix the wording where the approved
  design and criteria disagree, with today's answers captured (audit Q1 and Q4):
  "Yes, make it match". t37d puts that wording into #212.
- Whose name goes on an approval given in advance (Q2): "The agent's
  (Recommended)" (FLOW-24). Leif's standing rule, relayed the same day, sets the
  form: approvals and reviews on standing permission name the agent as
  `<agent>-<terminal>`, with a note naming the rule, where it was given and the
  recording agent, model and terminal; only a decision Leif made at that gate is in
  Leif's name; archived records are not edited; a refused agent actor is reported,
  never retried as Leif.
- Rewrite Git history to purge leaked paths: "No, clean from 0.6.0 on
  (Recommended)".
- Release condition: "yes.. but we dog food and fix all bugs before tagging and
  releasing" (plan.md, Release gate).
- On 2026-11-11: "we need to make sure we use chough to fix and close all the bugs and
  tickets for 0.6.0 before tagging 0.6.0.. we need to dogfood and make sure chough is
  perfect". plan.md carries the ticket ledger and the release gate the coordinator set
  from it: every item in the Chough 0.6.0 milestone closed through Chough, the dogfood
  complete with every bug it finds fixed the same way, then Leif's go.

Planned as the audit's recommended defaults, which Leif can still change at scope
approval:

- A pending decision holds only its own step, and every one is still listed (Q5,
  #237; design.md has both options).
- A session only ever closes itself; starting or switching work never archives
  another one, and `chough doctor` lists finished sessions with the fix (Q3, #207).
- A gate summary's "what changed" counts only the change's own files, from where the
  branch left main, leaving out Chough's records (Q6, #190).
- The upgrade is tested on copies from every earlier release, 0.1.0 to 0.5.0 (Q7).
- The fresh-agent guidance wording as proposed in REQ-cli-021 (Q9, #76).

Shown for Leif to confirm at scope approval: the screenshot cap, PNG or JPEG with
metadata removed, at most 512 KB each, 5 per piece of work and 2 MB in total, kept
in a folder beside its story; anything bigger stays on the machine that took it,
recorded by fingerprint (Q8, REQ-records-013).

The independent alignment audit (chough-060-audit, 629 items) is folded in: its eight
disagreements with the first draft of this plan, its other findings and its per-item
verdicts for every requirement, case file, doc and skill file this change touches.

## For a session picking this up

- Read the deltas first; they are frozen at approval. design.md explains them.
- The chough repository's own records stay 0.5.x for this whole change; drive it with
  the 0.5.0 build. The repository migrates only after this change is archived
  (KNOW-11), and the dogfood is the next change on this branch (plan.md, Dogfood).
- `tests/spec_split.rs` pins the bodies of REQ-history-001 to 004 and
  REQ-reporting-001 to 003. This change modifies or removes REQ-history-001, 002,
  003, 004 and REQ-reporting-001, so those pins are updated naming this change.
- Ruled out: a YAML crate (strict subset instead, minimal dependencies); a sidecar
  JSON index for speed (a twin); one file per decision (the approved design keeps
  decisions in the session story); keeping the event-file history beside the
  timeline (two sources); trimming archived stories (Leif: keep all).

## From the change's design.md

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
verify lane gains a `leaks` step that runs it and fails on a leak or an oversized record, so CI refuses a receipt over its bound (#215). Messages name the field and kind,
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
`chough migrate --apply <digest>`, which applies only the previewed plan, only with
the human's consent (`--consent <who, words, where>` or a standing `migration`
preference) and never while a native change is open:

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
   archive that was valid no longer is. The old trims' safety rules live on here,
   as #212 carries them forward: preview first, apply only with consent and never
   while a change is open (KNOW-11), and never weaken a SpecSync archive (KNOW-10).

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

## Staying out of SpecSync's verification (#12)

SpecSync 6.1.2 and later leave the directory prefixes listed in
`verification_ignored_paths` (`.specsync/sdd.json`) out of the verification digest,
and nothing else. In 0.5.x that could not be used here, because `session.json` sat
beside `cases/`. In 0.6.0 everything Chough writes while work is in progress lives in
three folders, `.chough/sessions/`, `.chough/archive/` and `.chough/bugs/`, and the
inputs a change's verification needs (`cases/`, `preferences.json`, `README.md`,
`knowledge/`) stay outside them. A project lists the three folders, and then a
checkpoint, an evidence entry or a bug written after `check --commit`, untracked or
committed, leaves the verification current, including in a reviewer's clean
checkout. Chough only reads SpecSync's policy: `doctor` reports
`records_in_verification` while an entry is missing, `init` and the migration say
the same, and `next` says when writes are safe. This change adds the entries to this
repository's policy during build. SpecSync refuses the list while an open change's
affected paths overlap it, so this change names `.chough/cases/`, `.chough/README.md`
and `.specsync/sdd.json` instead of all of `.chough/`; archived changes are exempt.

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

## From the change's testing.md

# Testing

## Requirement evidence

Each case binds one cargo test with the `cargo-test` adapter, in a case file of its
own per module, `.chough/cases/<module>.human-first-0-6-0.cases.toml`. A case may
link a requirement this change adds or modifies only after approval, while its delta
is unchanged; until `change check` materializes the delta, those cases are defined
in build and recorded in test. Cases for every acceptance criterion of each
requirement are inside the named test, one assertion block per criterion.

| Requirement | Case | Test | Shows |
|---|---|---|---|
| REQ-records-001 | TC-records-files-are-markdown-and-tolerant | tests/records.rs `every_record_is_markdown_and_keeps_human_edits` | front matter subset, unknown keys kept, prose kept, refusals with file and line, bounds |
| REQ-records-002 | TC-records-tree-and-readme | tests/records.rs `the_tree_holds_only_the_listed_paths` | init and a full cycle write only the tree's paths; README create-only |
| REQ-records-003 | TC-records-date-slug-names-and-aliases | src/identity.rs `tests::dated_names_are_bounded_slugs_without_counters`, `tests::a_clash_gets_one_suffix_and_a_second_clash_is_refused_without_another_draw`; tests/records.rs `names_are_date_and_slug_with_aliases` | slug rules, the 80-byte bound, clash suffix and refused second clash with no second draw (identity); each record kind's date-slug name, no count-based name, alias resolution by bug show, bug update and checkpoint --bug, a shared alias refused (records); merges of two branches are proven by tests/bugs.rs `bug_merge_unions_logs_and_takes_one_sided_edits`, tests/sessions.rs `branches_with_different_sessions_merge_without_conflict` and tests/cases.rs `independently_added_records_merge_without_conflict_or_replacement` |
| REQ-records-004 | TC-records-session-story-sections | tests/records.rs `the_session_record_is_checkpoint_and_story` | keys, sections, decisions and questions in the story only |
| REQ-records-005 | TC-records-proof-split-and-external | tests/records.rs `logs_stay_local_and_failures_keep_key_lines` | local logs, digest, key failing lines with redaction, external artifacts by ID and fingerprint |
| REQ-records-006 | TC-records-leak-guard | tests/leaks.rs `no_write_leaks_and_doctor_finds_old_leaks` | each pattern kind refused or redacted, value never echoed, doctor finding, verify lane fails |
| REQ-records-007 | TC-records-actor-recorded | tests/actor.rs, tests/bugs.rs `bugs_record_the_acting_agent` and the evidence actor test | override and detected values, none when unknown, unsafe terminal dropped |
| REQ-records-008 | TC-records-approval-as-the-human | tests/humans.rs, the five identity tests (`an_unknown_or_ambiguous_human_is_refused_before_anything_runs`, `no_source_names_a_person_as_the_default_human` and others) | approve and review run as the configured actor through a SpecSync stub; refusals; no default name in sources |
| REQ-records-009 | TC-records-migrate-from-0-5 | tests/migrate.rs `migrate_previews_applies_and_keeps_every_byte_in_git` | preview, refusal on uncommitted files, refusal while a change is open or without consent, mapping, receipt digests, idempotence, interrupted apply, removed IDs |
| REQ-records-009 | TC-records-migrate-evidence-entries-receipts, TC-records-migrate-evidence-v0-1-ids, TC-records-migrate-evidence-shared-observation, TC-records-migrate-evidence-key-screenshots, TC-records-migrate-evidence-validation-runs, TC-records-migrate-evidence-deterministic-names, TC-records-migrate-evidence-rolled-observations, TC-records-migrate-evidence-schema-1 | tests/migrate_evidence.rs `observations_become_entries_receipts_or_removals_and_every_old_id_resolves`, `v0_1_0_observation_ids_resolve_after_the_apply`, `one_observation_two_open_sessions_reference_is_one_entry_in_the_first`, `key_screenshots_are_stripped_bounded_and_kept_beside_their_session_or_story`, `validation_runs_carry_like_observations`, `names_are_deterministic_and_an_interrupted_apply_resumes`, `rolled_observations_in_the_vault_become_receipts_of_their_story`, `a_schema_1_observation_is_carried_without_content_and_stays_stale` | the evidence mapper: entries, receipts (REQ-history-006's fields) and removals with every old ID an alias, raw logs out with their digest as a sha256: fingerprint, in-project artifacts kept; v0.1.0 IDs; one entry for an observation two open sessions reference; stripped and bounded key screenshots; validation runs in validate run's format (REQ-cli-023); deterministic names and a resumed interrupted apply; rolled observations in the vault; schema 1 without content (REQ-cli-017) |
| REQ-records-009 (with REQ-cli-004, REQ-cli-006) | TC-cli-inventory-historical, TC-cli-inventory-advisory, TC-cli-inventory-strict, TC-cli-inventory-caps, TC-cli-inventory-change-key, TC-cli-inventory-legacy, TC-cli-inventory-aliases, TC-cli-inventory-bookkeeping-alias (ported) | tests/evidence_inventory_compatibility.rs, its eight tests (`inventory_exact_nine_originals_stay_historical_partial_and_byte_identical` and others) | the byte-pinned 0.5.x corpus and each inventory variant migrated into an open session, then read through live status with the freshness 0.5.x reported; an invalid inventory refuses the migration preview with nothing written |
| REQ-records-010 | TC-records-unmigrated-fails-closed | tests/migrate.rs `an_unmigrated_tree_is_refused_with_the_migrate_command` | every command refused, allowed commands run, mixed tree |
| REQ-records-011 | TC-records-journal-and-feed | tests/journal.rs `done_writes_a_short_post_and_the_feed_lists_it` | post at done, archive link, never overwritten, feed determinism, index names the timeline |
| REQ-records-012 | TC-records-only-committed-bytes-leave | tests/records.rs `nothing_leaves_the_tree_unless_head_holds_it`; tests/sessions.rs `an_uncommitted_file_keeps_the_folder_until_a_commit_and_doctor_fix`; tests/migrate.rs `migrate_previews_applies_and_keeps_every_byte_in_git` | finish keeps an uncommitted entry and reports it, 40 stories of every age untouched through done, session new, doctor --fix and library index (records); an uncommitted agent/ file kept until a commit and doctor --fix (sessions); migrate refuses a changed 0.5.x file and removes it once committed (migrate) |
| REQ-records-013 | TC-records-key-screenshots | tests/records.rs `small_key_screenshots_are_committed_beside_the_story` | metadata stripped, bounds and remedies, moved beside the story at finish |
| REQ-records-014 | TC-records-untracked-receipt-keeps-verification, TC-records-write-after-check-keeps-verification, TC-records-case-definitions-stay-verified, TC-records-doctor-names-missing-exclusions | tests/native.rs `an_untracked_receipt_at_check_commit_matches_a_clean_clone`, `chough_writes_after_check_commit_leave_verification_current`, `a_case_definition_change_still_stales_verification` (real SpecSync at the CI pin); tests/doctor.rs `doctor_names_the_missing_verification_exclusions` | the two #12 repros pass with the entries and fail without them; case definitions stay verified; doctor names missing entries and never writes `.specsync/sdd.json`; `next` says when writes are safe |
| REQ-history-002 | TC-history-append-only-timeline | tests/history.rs `transitions_are_append_only_timeline_entries` | grammar, unique IDs, atomic with the checkpoint, rewritten timeline found by doctor |
| REQ-history-003 | TC-history-timeline-bounds | tests/history.rs `the_timeline_is_bounded`; src/session_md.rs `tests::a_timeline_over_its_bound_is_refused` | entry and count bounds, next reads 128, timing partial; a timeline over its bound refused |
| REQ-history-005 | TC-history-backfill-on-entries | tests/stages.rs `a_backfilled_move_keeps_its_original_time_and_quoted_evidence` | a backfilled move keeps its original time and quoted evidence on its entry |
| REQ-history-006 | TC-history-165-earlier-archives-stay | tests/sessions.rs `done_leaves_earlier_archived_stories_in_place` | the #165 path: seven earlier stories, `knowledge/` untracked, nothing else changes |
| REQ-history-006 | TC-history-sessions-in-folders | tests/sessions.rs `sessions_live_in_folders_and_finish_into_one_story` | new, use, list, finish, kept_open cases, release, close, shelve |
| REQ-history-007 | TC-history-local-only-ignored | tests/sessions.rs `local_state_is_ignored_by_git` | every local file ignored |
| REQ-history-009 | TC-history-inbox-in-agent-folder | tests/sessions.rs `inbox_entries_live_in_the_agent_folder` | lock, whole publication, fold once |
| REQ-history-010 | TC-history-relay-fields-on-gate-entries | tests/history.rs `relay_fields_live_on_gate_closed_entries` | four values together, native step fields, refusals |
| REQ-history-012 | TC-history-doctor-findings-0-6 | tests/doctor.rs `doctor_reports_an_unmigrated_checkpoint_and_never_migrates`, `doctor_reports_a_committed_leak_and_redacts_it_only_with_redact`, `doctor_reports_a_rewritten_timeline_entry_and_fixes_nothing`, `doctor_reports_an_alias_held_by_two_bugs_and_fixes_nothing`, `doctor_reports_oversized_committed_records_and_the_leaks_step_fails`, `doctor_names_the_missing_verification_exclusions` | unmigrated, leak, unfinished archive, timeline rewritten, alias conflict, oversize (a committed receipt over its bound fails the verify lane's `leaks` step, #215), records in verification, existing findings |
| REQ-history-013 | TC-history-direct-write-entries | tests/history.rs `direct_write_entries_are_known_to_timing` | fields, refusal elsewhere, timing complete |
| REQ-history-014 | TC-history-relay-summary-digests | tests/gates.rs `relay_summary_fields_are_both_or_neither` | both or neither |
| REQ-history-015 | TC-history-append-only-corrections | tests/correct.rs, its four tests | correction entry, archived story append only, refusals, bounded list |
| REQ-cli-007 | TC-cli-checkpoint-in-session-front-matter | tests/cli.rs `the_checkpoint_is_the_session_front_matter` | round trip across processes, typed refusals, native-change refusals unchanged |
| REQ-cli-011 | TC-cli-case-run-writes-entry-and-local-log | tests/cases.rs `a_case_run_writes_an_entry_and_a_local_log` | existing execution assertions ported, log path, names |
| REQ-cli-012 | TC-cli-bugs-are-markdown | tests/bugs.rs `bugs_are_markdown_with_an_append_only_log` | existing bug assertions ported, Log append-only, aliases, receipts after archive |
| REQ-cli-014 | TC-cli-delivery-kinds-as-receipts | tests/stages.rs `delivery_kinds_survive_as_receipts` | kinds kept in the story |
| REQ-cli-015 | TC-cli-librarian-links-only | tests/knowledge_default.rs `the_librarian_copies_nothing` | removed commands, snapshots and briefs kept |
| REQ-cli-017 | TC-cli-evidence-entries | tests/evidence.rs `evidence_entries_carry_their_receipt` | keys, kinds, freshness parity for receipts, response keys |
| REQ-cli-019 | TC-cli-0110 and TC-cli-bug-0022-toolchain-matches-ci | tests/package.rs `versions_agree_across_readme_agents_ci_and_specsync_config`, `toolchain_task_names_a_version_that_differs` | the config comment joins the version check (BUG-0001); the toolchain task fails on a differing version (BUG-0022) |
| REQ-cli-021 | TC-cli-next-reads-the-story | tests/next.rs `next_reads_decisions_and_questions_from_the_story` | existing bounds ported, lesson bundles match the index |
| REQ-cli-022 | TC-cli-bug-0020-guard-uses-effective-approval and TC-cli-delta-guard-refusals-name-the-agent-actor (rewritten) | tests/native_check.rs `the_delta_guard_compares_with_the_effective_approval`; the ported guard tests | the effective, digest-matching approval (BUG-0020); the remedy names the scope gate, and the agent only for authority given in advance |
| REQ-cli-023 | TC-cli-names-and-draws | tests/ids.rs `new_records_get_date_slug_names_without_retry` | existing identity assertions ported to names |
| REQ-cli-029 | TC-cli-authority-before-approval (rewritten) | tests/workflow_authority.rs `authority_given_in_advance_names_the_agent_and_quotes_the_human` | a gate decision is the human's; authority given in advance names the agent as `<agent>-<terminal>`, for an approval or a review, with the quoted words (FLOW-24); a refused agent actor is reported, never retried as the human; the skill references agree |
| REQ-cli-030 | TC-cli-237-gate-holds-its-stage | tests/humans.rs `a_gate_holds_only_the_stage_it_names` | the #237 regression and the ported gate assertions |
| REQ-cli-032 | TC-cli-session-record-capacity | tests/cli.rs `the_session_record_is_bounded_at_one_mebibyte` | the 1 MiB and 400 bounds, timing at the cap |
| REQ-cli-034 | the ported REQ-cli-034 cases | tests/cli.rs, the existing size tests | spec files read at any size; no `library snapshot` exception; the 0.6.0 record bounds |
| REQ-cli-037 | TC-cli-active-session-folder | tests/sessions.rs `the_active_session_is_resolved_once` | pointer rules, mid-command switch, init with objective |
| REQ-cli-040 | TC-cli-one-writer-lock | tests/cli.rs `record_writers_share_one_lock` | busy refusal, two bug notes both kept |
| REQ-cli-042 | TC-cli-archived-finalization-over-record-bound (rewritten) | tests/cli.rs `an_archived_finalization_over_the_record_bound_still_tells_a_finished_session` | native sources read at full size by the finished-session check, timing and the index; no prune reader |
| REQ-cli-043 | TC-cli-index-links-every-lesson | tests/vault.rs `the_index_is_a_reading_guide_that_links_every_lesson` | index contents, refresh at done, check passes after done |
| REQ-cli-044 | TC-cli-vault-holds-only-its-own | tests/vault.rs `the_vault_holds_only_what_has_no_other_home` | allowed paths only, no copied text, links resolve |
| REQ-cli-047 | TC-cli-markdown-bug-merge | tests/bugs.rs `bug_merge_unions_logs_and_takes_one_sided_edits` | driver, attributes, index check |
| REQ-cli-050 | TC-cli-close-gate-drives-native-step | tests/humans.rs, the five driving-gate tests (`a_scope_gate_runs_the_approval_once_as_the_configured_human` and others) | existing relay assertions ported, drives none vs approve |
| REQ-cli-052 | TC-cli-advance-to-one-read | tests/advance_to.rs `advance_to_records_entries_on_one_read` | entries instead of events, one read into done |
| REQ-cli-059 | TC-cli-ship-reviews-as-human | tests/change.rs `ship_with_the_review_relay_ships_closes_the_gate_and_moves_to_done`, `ship_refuses_an_unknown_human_and_stops_on_a_refused_review` | review then ship through a stub, refusals |
| REQ-cli-061 | TC-cli-gate-record-one-driving-gate | tests/gates.rs `gate_record_runs_at_most_one_native_step` | the new refusal and stage column |
| REQ-reporting-001 | TC-reporting-timing-from-timelines | tests/timing.rs `timing_reads_timelines_and_receipts` | segments from open and archived timelines, partial on unreadable |

## Every open Chough bug, with its regression case

Leif widened 0.6.0 to fix every open Chough bug ("all bugs?", 2026-11-10). A row still
open is fixed in build and closed only with a passing observation of its case recorded
after a failing one, or, for a flake, with the stress result recorded honestly. A row
already fixed elsewhere is closed with a link to the fix, and its case stays as the
regression. The remaining Chough work is BUG-0020, BUG-0022, BUG-0023, the lock-timeout
and timing-budget flakes, #132, and the issues #12, #14 item 3, #76, #165, #167, #190,
#201, #234, #236 and #237.

| Bug | Fix in 0.6.0 | Requirement | Case | Failing first |
|---|---|---|---|---|
| #237 a gate blocks every advance | a gate holds only the stage it names (option A, pending Leif) | REQ-cli-030 | TC-cli-237-gate-holds-its-stage | a release gate refuses build to test on 0.5.0 |
| #165 done deletes an earlier archived session | no roll-up; nothing removes another session's story | REQ-history-006, REQ-records-012 | TC-history-165-earlier-archives-stay | the 0.5.x layout loses `release-a` on 0.5.0 |
| #12 Chough records and SpecSync verification stale each other | Chough's record folders hold only its records and are what a project lists in SpecSync's `verification_ignored_paths`; doctor and `next` say when they are missing and when writes are safe; this change lists them in this repository's policy. Two real repros since: an untracked evidence receipt at `check --commit` gave a reviewer's clean checkout stale verification (spec-sync #1045, 2026-11-11), and checkpoints after `check --commit` staled verification repeatedly on #212 | REQ-records-014, REQ-cli-004, REQ-cli-059 | TC-records-untracked-receipt-keeps-verification (the #1045 case), TC-records-write-after-check-keeps-verification (the #212 case), TC-records-case-definitions-stay-verified, and TC-records-untracked-receipt-keeps-verification, TC-records-write-after-check-keeps-verification (`tests/native.rs` `a_real_native_closeout_keeps_both_sides_current`), all with real SpecSync at the CI pin | both repro cases are written first in the 0.5.x layout without the entries and recorded failing on 0.5.0; if SpecSync at the pin still refuses the review after a committed Chough record, the case records that and that part is filed with spec-sync, with the documented order kept |
| #190 gate summaries count Chough's records and merged main | count from the merge base with the default branch, leaving out `.chough/`, the knowledge directory and SpecSync bookkeeping | REQ-cli-061 | TC-cli-190-gate-summary-counts-the-change (`tests/gates.rs` `gate_summary_counts_only_the_changes_own_files`) | the fixture counts 501 files on 0.5.0 |
| #76 next guidance for a fresh agent | items 2 and 5 are fixed on main (TC-cli-0121, TC-cli-0126 kept); items 1, 3 and 4 get the wording in REQ-cli-021 | REQ-cli-021 | TC-cli-76-next-guides-a-fresh-agent (`tests/next.rs` `next_guides_a_fresh_agent_from_init_to_accept`) | the init, test and accept actions differ on 0.5.0 |
| #14 env, colour codes, external artifacts, field errors | items 1, 2 and 4 shipped in #16 and stay proven; item 3 is REQ-records-005 | REQ-cli-009, REQ-cli-011, REQ-cli-054, REQ-records-005 | TC-cli-0097, TC-cli-0098, TC-cli-0099, TC-cli-0092 (ported) and TC-records-proof-split-and-external | item 3 refuses an absolute artifact on 0.5.0 |
| #132 fork and descriptor inheritance flakes | not reproduced since 2026-11-06. Stand-ins are already written through a temporary name and renamed; the lock refusal names its holder's process ID; the in-process lock test waits a bounded time for a forked child's inherited descriptor; a stress case records how often the lock and stand-in tests fail under a parallel suite | REQ-cli-040 | TC-cli-132-fork-inheritance-stress (`tests/cli.rs` `lock_and_stand_in_tests_survive_parallel_forks`) | the stress run on 0.5.0 is recorded as observed; if neither failure reproduces, the bug says so and stays open with the diagnostics in place |
| BUG-0001 the SpecSync config comment names stale versions | already fixed on main (the comment says 6.2.0): close the record with that link; the comment also joins the version agreement check so it cannot drift again | REQ-cli-019 | TC-cli-0110 (rewritten) | a stale comment fails the check |
| BUG-0020 the delta guard uses the latest approval | compare with the effective approval, the digest-matching one, for every workflow version | REQ-cli-022 | TC-cli-bug-0020-guard-uses-effective-approval (`tests/native_check.rs` `the_delta_guard_compares_with_the_effective_approval`) | three old-version approvals refuse wrongly on 0.5.0 |
| BUG-0022 local tools differ from the CI pins | a `toolchain` Fledge task, first in the verify lane, fails naming each version that differs, and evidence from a differing toolchain says so | REQ-cli-019 | TC-cli-bug-0022-toolchain-matches-ci (`tests/package.rs` `toolchain_task_names_a_version_that_differs`) | no such check exists on 0.5.0 |
| BUG-0023 cold-start output is hard to scan | `status` leads with the same short summary as `next` | REQ-cli-021 | TC-cli-bug-0023-status-leads-with-summary (`tests/next.rs`) | the summary is missing on 0.5.0 |
| BUG-…-empty-change-directory (#148) | fixed by #157 for the index: close the record with that link; 0.6.0 extends the rule to every reader of open changes | REQ-cli-043 | TC-cli-empty-change-dir-not-open (rewritten) | the case is rerun on 0.5.0 for `next`, `gate list` and `doctor` |
| BUG-…-history-writes-refuse-once-the-adopting-session-rolls-into-the-vault | fixed by #142: close the record with that link; the roll-up and the legacy anchor also go, and migration puts the legacy entries in the adopter's story | REQ-records-009 | TC-records-migrate-vaulted-adopter (`tests/migrate.rs` `a_vaulted_adopter_migrates_and_writes_work`) | the 0.5.x fixture is built at the release before #142, where the writes refuse |
| BUG-…-full-macos-lane-checkpoint-writer-lock-timeout | root cause unverified; same family as #132. Each writer child reports how long it waited, and the stress case runs the writer test under the full parallel suite on macOS | REQ-cli-040 | TC-cli-132-fork-inheritance-stress | as #132 |
| BUG-…-verify-lane-test-step-fails-intermittently-under-load | root cause unverified; same family as #132 and the lock timeout: the stress case runs the full suite in parallel under load and records each failure with its test name and environment, and the writer and lock diagnostics of #132 name what waited | REQ-cli-040 | TC-cli-132-fork-inheritance-stress | as #132; a flake that never reproduces goes to Leif, never closed to clear the gate |
| BUG-…-timing-native-budget-…-no-initial-stub-call | the budget clock starts at the first native read, as REQ-reporting-002 already bounds only time spent waiting on native reads | REQ-reporting-002 | TC-reporting-budget-starts-at-first-read (`tests/timing.rs` `a_slow_record_load_does_not_spend_the_native_budget`) | a record load slower than the budget starts no read on 0.5.0 |
| BUG-…-timing-trace-…-cross-a-second | fixed by #172 (TC-reporting-timing-trace-straddles-a-second): close the record with that link; the case also runs in the stress run | REQ-reporting-002 | TC-reporting-timing-trace-straddles-a-second | not needed: fixed before this change |

BUG-0002 and BUG-0003 describe SpecSync's own behaviour, fixed upstream in spec-sync
#823 and #824 on 2026-11-01. A spec-sync worker (t37b) checked both again on spec-sync
main 46f55652 (SpecSync 6.2.0) and neither reproduces: editing
an approved delta turns `approval_valid` false and `change check` refuses, and a
failing gate records `passed: false`. 0.6.0 closes both as `fixed-upstream`, citing
those PRs and that check, never as fixed or verified by Chough (REQ-cli-012). Chough ran no check of
its own for them.

## Failing first

- TC-history-165-earlier-archives-stay starts as a test in the 0.5.x layout on the
  0.5.0 code. It is recorded failing there (the earlier archived session leaves the
  tree) and linked to a local bug before any implementation, then rewritten for the
  0.6.0 layout and recorded passing.
- TC-cli-237-gate-holds-its-stage is recorded failing on 0.5.0 the same way (an
  advance at build is refused by a gate meant for release) and linked to a bug.
- Every other new test is written before its implementation and seen failing once.
  Ported tests keep their assertions; a test is deleted only with the requirement
  that retires its behaviour (REQ-cli-033, 041, 045, 046, 048, 049, 051 and
  REQ-history-001, 004, 008, 011), and tasks.md names each one.

## Test-stage regressions and the 6.3.0 send-back

Each test-stage bug has its own case, red before its fix and green after, in
`.chough/cases/*.test-stage*.cases.toml`, `cli.knowledge-test-fixes.cases.toml` and
`*.knowledge-send-back.cases.toml`. The send-back's code items are covered by
TC-cli-init-sets-up-merge-drivers, TC-cli-init-appends-only-missing-merge-lines,
TC-cli-doctor-merge-drivers and TC-history-doctor-merge-drivers (K1),
TC-cli-knowledge-index-bounded-open-bugs (K2), TC-records-journal-posts-record-time
(Q1), TC-records-the-standing-migration-preference-is-set-before-migrating (U2) and
TC-history-doctor-reports-an-unreadable-record (R9) and TC-cli-accept-ships-through-change-ship (N1). On SpecSync 6.3.0 the full suite
runs again, with each tester's SpecSync-dependent checks.

## Suites and platforms

- `fledge run fmt`, `fledge run lint`, `fledge run test` (the full suite) on every
  increment, macOS arm64.
- `fledge lanes run verify`, with the new `leaks` step and the existing `cases`,
  `knowledge`, `intent`, `spec` and `audit` steps, on macOS and in the Linux images
  (`fledge run linux-verify`) before accept.
- Windows is best-effort through WSL (FLOW-17, FLOW-21): no Windows run is claimed.

## Dogfood checks (recorded as evidence)

- `chough migrate` preview and `--apply` on a throwaway clone and then on this
  branch; receipt digests compared with the previous commit.
- The 0.6.0 build refuses an unmigrated clone of main with `refused:unmigrated`.
- This session's move into done in 0.6.0: its story, journal post, feed entry and an
  unchanged archive of earlier stories.
- CI's archive authentication over the migrated tree.
- The KNOW gap list (#216): computed from `hi/knowledge.md` and the recorded evidence,
  not from memory, it shows every live KNOW criterion with a passing observation and
  every retired one as retired.
- The ticket ledger in plan.md: every row in the Chough 0.6.0 milestone closed through
  Chough, each with its case's evidence.

Running and recording cases, the human scope approval and the human acceptance are
plain steps, not checkboxes.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
- `specs/reporting/context.md`
- `specs/records/context.md`
