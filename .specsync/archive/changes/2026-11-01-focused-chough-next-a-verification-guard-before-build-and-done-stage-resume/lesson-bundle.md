# Lesson bundle — focused-chough-next-a-verification-guard-before-build-and-done-stage-resume

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Focused chough next, a verification guard before build, and done-stage resume
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/main.rs, src/stage.rs, src/skill.rs, src/records.rs, tests/, docs/, README.md, examples/feature-bug-loop.sh, examples/visual-design.sh, skills/chough-workflow/, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/
- **Acceptance**: Focused next: `chough next` stops sharing the status report and prints a compact action view: the objective; the current stage with its status, iteration and side-stage callers; the recorded native change with the native read state, summary state, approval_valid, correction_valid, terminal evidence validity, native next action and the approved-delta result (REQ-cli-022); the next valid action with its reason and, when one applies, the command to run; the gate for the next main stage with its label; open awaiting-human gates; checkpoint blockers; open acceptance-blocking bug IDs and the count of other open bugs; and only the observations that are referenced and not current (historical, pruned or unavailable), each with its ID, label and at most three referencing records, at most 10 listed beside their total, plus the counts of referenced current observations and of unreferenced observations. It prints no raw native output, no observation records, no snapshot and no full reference list, and names `chough status` as the full view. Text is the default and `--json` prints the same fields as compact JSON with schema_version 1 and view `next`. Each form is at most 16384 bytes: every list shows at most 10 entries with its total, every string taken from a record is cut to at most 512 bytes with a visible marker, and if the result would still exceed the cap, list entries are dropped until it fits and `truncated` is true. `chough status` stays the full view with unchanged fields; its `--help` says it always prints JSON. Both commands compute next_valid_action with one shared function, so they report the same action for the same records, and each makes at most one `specsync change status` read and evaluates the stage gate on that same response. A fixture with 500 referenced stale observations, 40 awaiting-human gates and 40 blockers of 4 KiB each, and a native response near the parse bound keeps both forms within 16384 bytes with correct totals; a counting stub shows one native read per command; the pull request reports both forms' byte counts on this repository (LIFE-3, FLOW-13, LIFE-7, REQ-cli-003, REQ-cli-021).
- **Acceptance**: Done-stage resume: in stage done, active and with no awaiting-human gate, when the recorded native change is archived with authenticated terminal evidence or no native change is recorded, the next valid action no longer echoes the checkpoint's next_action. It says the cycle is closed and recommends a new one: `chough stage reset --to intent --reason "<why>"` (or `--to plan` when the approved intent already covers the new work), then `specsync change new "<outcome>" --spec <module> --path <path>`, then `chough checkpoint --native-change <id> --next-action "<next step>"`; when native status names a next action, such as merging a pull request that is still open, the view shows it as the thing to check first. Every reset command Chough suggests carries `--reason`. An archive whose terminal evidence does not authenticate keeps today's gate refusal. `chough stage reset` replaces the checkpoint's next_action with `New cycle from <stage> after reset: <reason>. Create or record the native change (specsync change new, then chough checkpoint --native-change <id>) and set the next action.`, so a stale action is not carried into the new cycle; `chough stage back` leaves next_action unchanged, and the transition records the reason as before. The canonical skill's references/checkpoint.md gives the same steps with `--reason`. Tests drive a done fixture with an archived stub change: the action text and reason, that the suggested reset with its placeholder filled succeeds, and the replaced next_action afterwards (FLOW-13, LIFE-3, FLOW-2, REQ-cli-021).
- **Acceptance**: Approved-delta guard: when a move to build, test or accept (by advance, skip, back or reset) meets a native change in state approved, implementing or verifying whose status reports approval_valid true, Chough also checks that the semantic deltas are the ones approved instead of trusting approval_valid alone (BUG-0002: SpecSync 6.1.1 keeps it true after an approved delta is edited). It reads `.specsync/changes/<id>/approvals.json` read-only, bounded at 4 MiB (SpecSync 6.1.1's own change-artifact bound; the record bound of 64 KiB is too small for ledgers that SpecSync itself has grown past 270 KB), takes the last definition approval as SpecSync does, and compares its approved_delta_digests with digests it computes with SpecSync 6.1.1's framing (SHA-256 over the length-framed domain `specsync.approved-delta.v1`, the module name and the delta body with CRLF normalized to LF) for every module in the status response's change.affected_specs, each delta read under the same 4 MiB bound. A module whose digest differs, whose delta is missing, or that is affected but absent from the approval refuses the move with `refused:native-delta-changed`, naming the modules and the next native actions: restore the approved delta, or get a new human scope approval (`specsync change approve <id> --actor "<their name>"`) and then run `specsync change check <id>`. When the comparison cannot be made (the ledger is missing, unreadable, over the bound or malformed; there is no definition approval; a change with deltas has an approval without approved_delta_digests; status reports no affected_specs; a delta cannot be read) the move is refused with `refused:native-delta-unverifiable`, the reason and the next native action `specsync change status <id>`. A no_spec_change approval with an empty digest map passes. The guard starts no process: native reads stay the one bounded `specsync change status` read, no gate runs `specsync change check`, and archived changes keep the terminal-evidence rule. The gate and native summary in `chough next` and `chough status` show the result. Tests: digests computed for this repository's archived changes match their recorded approvals; an edited delta, an added module, a deleted delta, a missing ledger, an approval without digests and an oversized ledger each give the stated label, and the refused move leaves the session and transitions byte-identical; a CRLF-only edit and an unchanged approval pass (FLOW-5, LIFE-7, LIFE-1, REQ-cli-008, REQ-cli-022).
- **Acceptance**: Host wording: the canonical SKILL.md host section, README.md and docs/integrations.md say that `--host openai`, the default, writes `.agents/skills`, a shared skills directory, and that Google Antigravity read the skill from that directory in one observed run reported narratively in closed PR #20 and not independently verified; that `--host gemini` writes `.gemini/skills`, which no run has exercised; and nothing about Gemini CLI. The host map in src/skill.rs and `chough skill status` are unchanged: status reports `.agents/skills` once, as the openai directory, and does not count it for gemini. KNOWN_PREVIOUS lists the origin/main digests of each bundle file this change edits (SKILL.md and references/checkpoint.md), so unedited earlier copies read known-outdated and update while edited copies stay edited; Chough's own .agents/ and .claude/ copies are refreshed and read current, and no bundle file gains U+2013 or U+2014 (FLOW-1, LIFE-4, REQ-cli-016, REQ-cli-019).
- **Acceptance**: Cold-start record: docs/exercises/cold-start.md gains a section dated 2026-11-01 for Google Antigravity, marked unverified and narrative only, whose assessment table cites the closed PR #20 log (`.chough/artifacts/gemini-cold-start.log` at commit 53b60aa on branch gemini/feat/gemini-host-cold-start) as its only source and says what that source cannot show: it was written by the agent itself with no event log or transcript, it names the model inconsistently (Gemini 3.8 Flash High in the log, Gemini 2.5 Pro in the PR's own document), and its change was closed unmerged. Its done-stage friction is cited as the finding this change fixes. docs/integrations.md gets an Antigravity row with the same caveat. The earlier Codex record is not rewritten (FLOW-13, LIFE-4).
- **Acceptance**: Guidance and verification: docs/cli.md documents the next view (fields, text default and `--json`, the 16384-byte cap, one native read), status as the full view, both refusal labels with their next actions and the reset next_action replacement; docs/architecture.md says the guard reads the native approval ledger read-only and copies nothing; the README quick start shows `chough next`; examples/feature-bug-loop.sh and examples/visual-design.sh read `chough next --json`; existing tests that read full status fields through `next --json` read `status` instead. Cases TC-cli-0120 to TC-cli-0124 cover the cap, done-stage resume, the reset next_action, a changed delta and an unverifiable delta; they link REQ-cli-021 and REQ-cli-022, so they are added and run after the scoped `change check` materializes them. BUG-0002 stays open with a note that Chough now guards it; it closes when CI pins a SpecSync release that reports an edited approved delta. The full suite, `fledge lanes run verify` (including the feature-and-bug exercise) and native structural checks pass on Ubuntu, macOS and Windows CI. No new integration, model client, storage or record kind (LIFE-6, LIFE-7, FLOW-7).

## Evidence

- Verification commit: `2058870c298d6ff8be582c7cd182ac1479a63028`
- Base commit: `ee6e3d2fd69e1fb979873676b7b02d8d86a9074d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

0xLeif named three decisions as blocking for 0.1.0, after independent reviewers
measured them:

1. `chough next` and `chough status` share one code path (`src/main.rs`, the
   `Commands::Status { .. } | Commands::Next { .. }` arm) and print the same report;
   `--json` is ignored by both. On this repository the report is about 410 KB, almost
   all of it observation records and raw SpecSync output, which a resuming agent
   must load to find one next action.
2. BUG-0002 (open, out of scope for Chough's code until now): SpecSync 6.1.1 keeps
   `summary.approval_valid` true after an approved delta is edited, and Chough's build
   gate trusts it. Chough should guard this itself before admitting build, test or
   accept.
3. Closed PR #20 (a Gemini or Antigravity cold start) found two real things and got
   them partly wrong: from stage done, `chough next` echoed the old review action;
   its suggested `chough stage reset --to plan` omitted the required `--reason`; and
   its host wording claimed that Gemini CLI and Antigravity read both `.agents/skills`
   and `.gemini/skills`, which no record shows.

Constraints a resuming session needs: draft only until the human approves the scope;
`.chough/` is deliberately not an affected path; no new integration, model client or
storage; open PR #22 (0.1.0 release prep) modifies REQ-cli-008, REQ-cli-016 and
REQ-cli-019 and edits SKILL.md, references/checkpoint.md and KNOWN_PREVIOUS, so this
change adds REQ-cli-021 and REQ-cli-022 instead of modifying those requirements, and
whichever pull request merges second rebases its KNOWN_PREVIOUS lines and bundle
edits. Case IDs TC-cli-0120 to TC-cli-0124 are reserved here; they are free on
origin/main (highest 0104) and on PR #22 (0110 to 0114).

## From the change's design.md

# Design

## One assessment, two views (REQ-cli-021, REQ-cli-003)

`src/main.rs` stops sending `Status` and `Next` through one arm. A shared
`assess(root, environment)` reads the session, makes the one native read, evaluates
the gate for the next main stage on that same response, and computes the next valid
action with its reason and command. `status` serializes its existing full report from
the assessment (same fields as today), and `next` renders the compact view from it.

To make one native read, `stage::native_state` gains a variant that takes an
already parsed `NativeJson` (the response `native_status` in main.rs already has),
and `destination_gate` gets a sibling that takes the resulting `NativeState`.
Stage commands keep their single read. Today `status` at plan or accept reads
native status twice (`native_status`, then `build_gate` or `accept_gate`); on this
repository each read takes about 12.7 seconds.

### Next valid action (shared)

Rules in order; the first that applies wins:

1. Schema 1 session: run `chough migrate`.
2. Paused: check native state and freshness, then `chough stage resume`.
3. Awaiting human: name the gates; do not record them on the human's behalf.
4. Open blocking bugs at test or accept: go back to build.
5. New: stage done (active). The done prerequisite is evaluated on the same
   response:
   - archived with authenticated terminal evidence, or no native change recorded:
     action `Start a new cycle: chough stage reset --to intent --reason "<why>"
     (or --to plan when the approved intent already covers the new work), then
     specsync change new "<outcome>" --spec <module> --path <path>, then chough
     checkpoint --native-change <id> --next-action "<next step>".` Reason: `The cycle
     for <id> is closed: native state archived, terminal evidence <validity>.` plus,
     when native status reports a next action, `Native next action, check it first:
     <text>.` (on this repository: `merge the PR on GitHub if it is still open`).
     Command: `chough stage reset --to intent --reason "<why>"`.
   - native SpecSync not configured or SDD disabled: the same reset, without the
     `specsync change new` step.
   - archived whose terminal evidence does not authenticate, or not archived: the
     existing gate refusal text and label, unchanged.
6. Gate for the next main stage not open: the gate label and detail.
7. Otherwise: `Continue <stage>: <checkpoint next_action>`.

Every reset Chough suggests anywhere includes `--reason`; `valid_transitions`
already does.

### Reset replaces the stale next action

`stage reset` (any destination) sets the checkpoint's `next_action` to `New cycle
from <stage> after reset: <reason>. Create or record the native change (specsync
change new, then chough checkpoint --native-change <id>) and set the next action.`
A reset always starts a new cycle (the timing report already treats it so), so the
old action is never right afterwards. `back` keeps `next_action`, since it continues
the same cycle. The reset transition is recorded as before. `native_change` is kept:
the next view then shows the archived change as closed and asks for the new one, and
the plan to build gate refuses `native:archived` until it is recorded (open question
3).

### The compact view

JSON (`--json`), compact serialization:

```json
{"schema_version":1,"view":"next","objective":"...",
 "stage":{"current":"done","status":"active","iteration":1,"callers":[]},
 "native":{"change":"<id>","read":"observed","state":"archived","approval_valid":null,
   "correction_valid":null,"terminal_evidence":"exact","next_action":"...",
   "approved_deltas":"not checked: archived"},
 "next_valid_action":{"action":"...","reason":"...","command":"..."},
 "gate":{"to":"build","allowed":false,"label":"...","detail":"..."},
 "awaiting_human":{"total":0,"items":[]},
 "blockers":{"total":0,"items":[]},
 "blocking_bugs":{"total":0,"items":[]},"other_open_bugs":4,
 "observations":{"needs_attention_total":61,"items":[{"id":"...","label":"historical",
   "referenced_by":["checkpoint","BUG-0004 verified"]}],
   "referenced_current":0,"unreferenced":143},
 "environment":null,"full_view":"chough status","truncated":false}
```

`native.read` is the native section state (`observed`, `unavailable`, `timed-out`,
`invalid-output`, `not-configured` and so on, as status reports it); unknown values
stay null, never false or true. `approved_deltas` is the REQ-cli-022 result
(`unchanged: cli`, `changed: cli`, `unverifiable: <reason>`, or `not checked: <why>`).
`gate` is null when the next main stage has no prerequisite.

Text (default) carries the same fields as short lines:

```text
Objective: ...
Stage: done (active, iteration 1)
Native change: <id>: archived, terminal evidence exact (read observed)
  Native next action: merge the PR on GitHub if it is still open
Next: Start a new cycle: chough stage reset --to intent --reason "<why>" ...
  Why: The cycle for <id> is closed: ...
  Run: chough stage reset --to intent --reason "<why>"
Human gates: none
Blockers: none
Blocking bugs: none (4 other open; chough bug list)
Observations needing attention: 61 referenced, not current (10 shown)
  bug-0004-fixed-1 historical (checkpoint, BUG-0004 verified)
  ...
  51 more; 0 referenced current, 143 unreferenced. Full view: chough status
```

Bounds: `NEXT_LIMIT = 16_384` bytes per form, equal to the existing context bound
`records::LIMIT`. Every list shows at most 10 entries with its total; at most 3
`referenced_by` entries per observation; every string from a record (objective,
gates, blockers, native next action, gate detail) is cut at a character boundary to
at most 512 bytes, ending in ` [cut]`. After rendering, while the output is over the
limit, the last entry of the longest list is dropped and `truncated` becomes true
(text adds `Output truncated at 16384 bytes; run chough status.`). The view never
includes `native_specsync.content`, `observations` records, `snapshot` or the full
`observation_references`.

Observation counts come from `evidence::reference_labels` (already computed for
status) and the count of observation records present; unreferenced ones are counted,
not loaded into the view.

`status` keeps its JSON report and its fields. Its `--json` stays accepted, and its
help says status always prints JSON (open question 2).

## Approved-delta guard (REQ-cli-022, BUG-0002)

`native_gate` (src/stage.rs) gains the project root and, after its existing
approval_valid and correction_valid checks pass for a state in `BUILD_STATES`
(approved, implementing, verifying), calls `approved_deltas(root, id, value)`.
Archived changes keep the terminal-evidence rule; draft and other states are refused
before this point as today. So the guard covers every route to build, test and
accept: advance, skip, back and reset all use `destination_gate`.

`approved_deltas` reads, read-only:

1. `.specsync/changes/<id>/approvals.json` with a new bounded reader in
   src/records.rs (`read_native_record(path, NATIVE_RECORD_LIMIT)`, 4 MiB, SpecSync
   6.1.1's MAX_CHANGE_ARTIFACT_BYTES; the 64 KiB RECORD_LIMIT is too small, since
   spec-sync's own archive has a 273,639-byte ledger). The id is the recorded native
   change, already validated by the checkpoint; the path goes through `local_path`.
2. The last entry of `approvals` with `gate == "definition"` (SpecSync's
   `effective_definition_approval` takes the same entry), and its
   `approved_delta_digests`, an object of module to 64 lowercase hex characters.
3. `change.affected_specs` and `change.no_spec_change` from the status response
   already read. Each module must be a valid module name (`valid_name`).
4. For each module, `.specsync/changes/<id>/deltas/<module>.md` under the same
   bound, CRLF replaced by LF, digested as SHA-256 over: frame(`domain`,
   `specsync.approved-delta.v1`), frame(`module`, name), frame(`body`, text), where
   frame(tag, value) is the 8-byte big-endian length of tag, tag, the 8-byte
   big-endian length of value, value. This is SpecSync 6.1.1's `FramedDigest`.

Result:

- Every approved module matches and no affected module is unapproved: allowed;
  the gate detail appends `approved deltas unchanged (<modules>)`.
- `no_spec_change` true, no affected deltas, and an empty or absent digest map:
  allowed, `no semantic deltas (no_spec_change)`.
- Any approved module whose digest differs or whose delta is missing, or an affected
  module absent from the approval: `refused:native-delta-changed`, detail `Reaching
  <stage> needs the approved semantic deltas; the delta for <modules> changed after
  the definition approval, while native status still reports approval_valid true
  (SpecSync 6.1.1, BUG-0002). Restore the approved delta, or ask the human for a new
  scope approval (specsync change approve <id> --actor "<their name>") and then run
  specsync change check <id>. Digests use SpecSync 6.1.1's specsync.approved-delta.v1
  framing.`
- Anything that prevents the comparison: ledger missing, unreadable, over the
  bound, not JSON or the wrong shape; no definition approval; a change with deltas
  whose approval has no `approved_delta_digests`; a malformed digest; no
  `change.affected_specs` in status; an invalid module name; a delta over the bound
  or unreadable for a reason other than absence: `refused:native-delta-unverifiable`,
  detail naming the reason and `Native next action: run specsync change status <id>;
  Chough cannot confirm that the approved deltas are unchanged.`

The guard starts no process and never runs `specsync change check`, which
research shows mutates canonical specs and native records, refuses whenever tasks
are unchecked, and runs verification commands. It copies nothing into Chough's
records. Native reads stay the one bounded status read.

## Host wording (REQ-cli-016 and REQ-cli-019 unchanged)

No code change to `HOSTS` or `skill status`. Counting `.agents/skills` for gemini
would report one directory twice and assert a Gemini mapping no run supports. The
canonical SKILL.md host paragraph becomes:

> Chough runs in the current agent; no provider API or runtime is required. `chough
> init --host <openai|claude|codex|cursor|gemini|grok>` copies this bundle into that
> host's project skill directory: openai, the default, writes `.agents/skills`, a
> shared directory that more than one host may read, and gemini writes
> `.gemini/skills`. `chough skill status` shows whether a copy is current, edited or
> outdated. Which directory a host reads is the host's behavior: Claude Code was
> observed reading `.claude/skills`, and one unverified run reported Google
> Antigravity reading `.agents/skills`. Confirm that the host actually lists the
> skill; otherwise give it this file directly. Subagents are optional:
> [roles.md](references/roles.md) also works as sequential passes.

references/checkpoint.md, Resume cold, step 4 gains: `In stage done with the native
change archived, the cycle is closed: chough next recommends chough stage reset --to
intent --reason "<why>" (or --to plan), then specsync change new and chough
checkpoint --native-change <id> --next-action "<next step>". A reset replaces the
checkpoint's next action.`

`KNOWN_PREVIOUS` gains the origin/main ee6e3d2 digests of the two edited files:
SKILL.md `d350cc91271e6d942bef516a2b2ea8a6009697a2e0d034b9ff48c8591be2ee5c` and
references/checkpoint.md
`20daaaa0cec972169abcedd508cd9ad18ff93ef7ec9d1f4e7330fcd18ca80ac0`. If PR #22 merges
first, its versions of those files are the previous ones and their digests are added
too.

README.md (host list) and docs/integrations.md (host table) say the same: an
Antigravity row, `.agents/skills/chough-workflow/` (openai mapping), one run on
2026-11-01, self-reported in closed PR #20, not independently verified; the Gemini row
becomes `Gemini CLI` with `.gemini/skills`, not exercised.

## Cold-start record

A new section in docs/exercises/cold-start.md, after the Codex record, which is not
rewritten:

```markdown
# Exercise: Google Antigravity on CorvidLabs/chough (unverified, narrative only)

Requirement links: FLOW-13, LIFE-4. Reported 2026-11-01 on macOS arm64.

Source: closed PR #20, `.chough/artifacts/gemini-cold-start.log` at 53b60aa on branch
gemini/feat/gemini-host-cold-start. The agent wrote this log itself; no event log or
transcript was kept, and the model is named two ways (Gemini 3.8 Flash High in the
log, Gemini 2.5 Pro in the PR's document). Nothing below was observed independently.

| Expectation | Reported result | Status |
|---|---|---|
| Found the workflow skill without coaching | Listed from `.agents/skills/chough-workflow/` | Reported, unverified |
| Read the checkpoint | `chough next` from done echoed the archived change's review step | Reported; reproduced on main ee6e3d2, fixed by this change |
| Started a new cycle correctly | Suggested `chough stage reset --to plan` without the required `--reason` | Reported; corrected in guidance by this change |
| Honored human gates | Stopped for scope approval before build | Reported, unverified |
| Outcome | Change archived as no_spec_change; PR closed unmerged | From the PR record |
```

## Out of the design

No change to the host map or skill status, no new record kind or store, no network
or model client, no `specsync change check` from any gate, no copy of native state
into Chough's records, no modification of REQ-cli-008, 016 or 019 (open PR #22), and
no ROADMAP edit.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-021 | `tests/cli.rs`: next text and `--json` forms carry the listed fields and no `native_specsync`, `observations` or `snapshot`; the cap fixture (500 referenced stale observations, 40 gates and 40 blockers of 4 KiB, a native response near the parse bound) keeps each form at most 16384 bytes with totals and `truncated`; strings cut at 512 bytes with ` [cut]`; a counting `specsync` stub records one call for `next` and one for `status` at plan, build and accept; `next` and `status` agree on `next_valid_action` across the fixtures |
| REQ-cli-021 | `tests/stages.rs`: done with an archived authenticated stub recommends reset with `--reason` and `specsync change new` and shows the native next action; the suggested reset with the placeholder filled succeeds; afterwards `next_action` is the new-cycle text; `back` leaves it unchanged; a non-authenticating archive keeps its refusal label; no native change and SDD disabled recommend the reset |
| REQ-cli-022 | `tests/stages.rs`: digests for the 7 archived approvals of this repository (fixture copies of their deltas and ledgers) match; plan to build, build to test and test to accept refuse with `refused:native-delta-changed` for an edited delta, an added module and a deleted delta, and with `refused:native-delta-unverifiable` for a missing ledger, a ledger over 4 MiB, malformed JSON, no definition approval, an approval without digests and status without affected_specs; each refused move leaves session.json and transitions.jsonl byte-identical; a CRLF-only edit, an unchanged approval and a no_spec_change approval pass; back and reset to build use the same guard |
| REQ-cli-016, REQ-cli-019 | `tests/package.rs`: the origin/main SKILL.md and references/checkpoint.md read known-outdated and update; an edited copy stays edited; no bundle file contains U+2013 or U+2014 |

Existing tests that read full-report fields through `next --json` (tests/cli.rs,
tests/bugs.rs, tests/stages.rs, tests/timing.rs, tests/evidence.rs) read them through
`status`, and keep asserting the decision fields through `next --json`.

## Cases

Reserved: TC-cli-0120 (next cap), TC-cli-0121 (done-stage resume), TC-cli-0122 (reset
next_action), TC-cli-0123 (changed delta refused), TC-cli-0124 (unverifiable delta
refused). They link REQ-cli-021 or REQ-cli-022, so they are added to
.chough/cases/cli.cases.toml and run only after the scoped `change check` materializes
those IDs.

## Real behavior

On this repository, built binary: `chough next` and `chough next --json` byte counts
and wall time against the measured 410,822 bytes and 15 seconds; the done-stage action
names the reset. In a disposable copy of examples/shipping: approve with a SIMULATED
actor, append a requirement to the delta, and `chough stage advance` from plan refuses
with `refused:native-delta-changed` while `specsync change status` still reports
approval_valid true.

Regression: full `cargo test`, `fledge lanes run verify` (including the
feature-and-bug exercise) and native structural checks on all three CI platforms.

## Where these lessons go

- `specs/cli/context.md`
