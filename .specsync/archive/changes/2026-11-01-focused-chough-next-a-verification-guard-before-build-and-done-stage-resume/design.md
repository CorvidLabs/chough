---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: design
---

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
