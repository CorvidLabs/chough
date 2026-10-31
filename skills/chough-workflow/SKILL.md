---
name: chough-workflow
description: Carry a product request or bug through HI intent, scoped requirements, native SpecSync changes, product verification, delivery evidence, and focused session resumption. Use for Chough-guided development, not unrelated single-file edits.
---

# Chough workflow

Keep human intent legible throughout delivery. HI owns permanent human criterion IDs;
SpecSync owns module contracts, companions, change gates, reviews, and archives;
Fledge runs project tasks and work flows. Chough supplies focused context and bound
observations. Use the native record for each responsibility rather than copying its
state into a new checklist.

## Start or resume

Read applicable `AGENTS.md` instructions. Prefer Fledge for tasks it supports;
inspect installed help when a command or lifecycle is uncertain. Missing native
tools or instructions are gaps to report, not reasons to invent their output.

For a new workspace, first run
`chough init --objective "<current outcome>" --intent-scope <HI-family> --module <module>`;
it preserves existing files. Then run `chough context` and
`chough status --environment <actual-environment>`. Edit `.chough/session.json` to maintain the objective,
selected intent scope/modules, decisions, open questions, and next action. Repeat
`--module` for multiple affected modules. This record is a handoff, not a lifecycle
ledger. Read any context omissions/truncation explicitly reported by the helper
before making a decision that depends on them.

Without the helper, read the same sources directly: relevant HI export; affected
module contract, requirements, tasks, testing and context; native change status;
objective, decisions and open questions. If a module has `<module>.3md`, read its
context and lesson planes as the native lesson source. Do not load unrelated history.

## Shape the request

Capture what the human wants before selecting an implementation. Use `hi export
<scope>` to find existing criteria before creating IDs. Preserve the human's
meaning and distinguish quoted intent, agent interpretation, proposed additions,
and unresolved decisions. Ask a short product question only where its answer
changes the outcome or scope; keep independent authorized work moving.

Use `hi <ID> "<human criterion>"` for agreed criteria. Never recycle or repurpose an
ID; retire it through HI when its meaning is withdrawn. Human criteria remain
durable sentences even when unbuilt or temporarily failing.

Synthesize an appropriately sized functional requirement from the relevant intent.
Put stable `REQ-*` requirements and acceptance criteria in native module
`requirements.md`; an FRD may summarize a cross-module feature and link to them.
FRD synthesis is this workflow's job, not an HI command. Keep each requirement small
enough to verify independently. Link each requirement to HI IDs and its contract;
describe an observable success condition, boundary cases, and relevant environment.
Mark agent-proposed scope as proposed until a human decision supports it.

## Deliver a scoped change

Read [the native lifecycle reference](references/lifecycle.md) for feature work,
contract changes, or a fix. Read [the bug reference](references/bugs.md) when asked
to reproduce, classify, deduplicate, report, or repair a defect.

Implementation and product checks follow the approved contract and applicable
native gates. A structural SpecSync pass does not execute arbitrary product tests.
Record observed product behavior separately from structure, CI, review, merge,
release, deployment, and verification of the delivered target.

The helper's `evidence add` records an observation; it does not run a test or prove
who performed it. Supply real criterion/requirement/contract links, exact
environment and a concise result, plus saved artifacts where available. Inspect
`chough evidence add --help` for flags. Use `product-test`, `structural`,
`human-observation`, or `delivery-observation` according to what actually happened.
Never turn an actor name, agent prose, a settled process, or green structural check
into authentic human approval or a deployment claim. Treat missing/stale evidence
as missing/stale, including after a revision or relevant environment changes.

## Leave the next session useful

Update native tasks and testing records with completed work, remaining debt, and
evidence links when that is within the requested scope. For an investigation that
must preserve native records, leave the findings and source links in its local
report and session instead. Keep the Chough session focused on the next objective and unresolved
decisions; `next_action` is a saved suggestion, while native status determines gate
eligibility.

Record lessons in existing native module sources: `<module>.3md` when present,
otherwise `context.md`. Each lesson needs a source (bug, test, review, or decision),
scope where it applies, and a concrete future use. Revise or retire a lesson when
later evidence contradicts it; retain its source and reason. Do not turn a
one-off implementation detail into a universal rule.

## Host use

Chough runs in the current agent. `init` installs this bundle in
`.agents/skills/chough-workflow/`; supported hosts can discover that project skill.
Use `$chough-workflow` where skill invocation is supported, or ask the host to read
this file. For another host, copy the complete bundle to its documented skill
location or use its supported skill installer. Map a slash command to this same
skill only when that host supports it; Chough does not install universal slash
commands. Rune session execution and other adapters remain optional until their
implementation and evidence establish support.
