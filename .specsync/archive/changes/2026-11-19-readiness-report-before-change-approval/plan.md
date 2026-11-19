---
change: readiness-report-before-change-approval
artifact: plan
---

# Plan

## Approach

Add `ChangeCommand::Ready` in `src/change.rs` and implement the report in
`src/ready.rs`. New items stay `pub(super)` or private. `ready` does not call
`specsync change approve` or `specsync change check`. It reads the change with
`specsync change show <id> --json` for the interview, and `specsync check --spec
<module> --format json` for exports, saving and restoring `.specsync/hashes.json`
around that check. When `specsync` cannot be started, one finding names
`cargo install --locked specsync@6.3.0`.

Stale canonical text follows SpecSync's post-approval base binding. Before the
first definition approval there is no recorded base, so that class is empty.
After approval, a `## MODIFIED` or `## REMOVED` item is a finding only when the
delta body is unchanged since that approval, the canonical block now differs
from the recorded base, and the approved body does not already carry every edit
the current text made to that base.

Path coverage is Chough's own check: files changed since the change's
`base_commit`, including unstaged, staged and untracked files Git does not
ignore, minus `affected_paths`, `verification_ignored_paths` and
`.specsync/changes/<id>/`. A draft's fix is one `chough change answer` of the
full path list. Any other state names `specsync change send-back`.

The fast path in `skills/chough-workflow/references/stages.md` names
`chough change ready <id>` after the delta and before "scope approval belongs
to the human", inside the existing 32-line budget and without reordering the
phrases `tests/package.rs` already requires. `references/plan.md` tells the
agent to run that command before the scope approval. The same two files are
copied under `.agents/skills/chough-workflow/references/`. Their digests update
`tests/fixtures/skill-releases/manifest.json` `current_bundle_sha256`, and the
manifest's own digest updates `MANIFEST_SHA256` in `tests/skill_releases.rs`.

`chough timing --cycle all` on 2026-11-14 skipped 182 older archived changes
and read at most 10, and this cycle has only the current plan visit, so a
per-stage range is too thin to treat as a deadline. Relative size: one new
read-only subcommand, its tests, two command-table rows and a one-line skill
edit. Larger than the summary-cut session of 2026-11-14 (about 43 minutes of
known stage time, most of it a human wait). Smaller than the session that added
the `chough change` passthrough (about 2 hours 23 minutes of known stage time).
The main uncertainty is the stale-base comparison, including the already-carried
exception, reporting a finding only when SpecSync itself would refuse the
approval.

One agent owns every file in this change.

## Out of scope

Short change ids, `chough next` printing a filled-in command, and `chough ticket`
are later changes. `new`, `answer`, `check`, `status` and `ship` stay
passthroughs. No model client, runtime or second lifecycle record. Human gates
in `tasks.md` stay plain text, because `change check` refuses an open checkbox.
`.chough/sessions/`, `.chough/archive/` and `.chough/bugs/` are not declared
paths.

## Steps

1. Dispatch `ready` from `chough change` and implement the six finding classes
   in `src/ready.rs`, with `tests/ready.rs` failing first.
2. Add `.chough/cases/cli.change-ready.cases.toml` for REQ-cli-067. Do not put
   the cases in the shared module file.
3. Document the command in `docs/cli.md` in its own section after the
   passthrough table, and add `ready` to that table's `change` row and to
   README's helper row. Leave the sentence that the passthrough steps add
   nothing. Add an Unreleased changelog bullet.
4. Name `chough change ready <id>` on the fast path and in the plan reference,
   in both skill copies, and refresh the skill-release manifest digest and
   `MANIFEST_SHA256`. Assert the new phrase in `tests/package.rs` between the
   delta phrase and "scope approval belongs to the human".
5. Run `fledge run fmt`, `fledge run test`, `fledge run lint` and
   `fledge lanes run verify` with Fledge 1.8.0 first on `PATH`. Run
   `chough change ready` on this change until it exits 0, then
   `chough change check`.

## Risks

A stale-base check that ignores the already-carried exception will fail a
change SpecSync would approve, after main moves a requirement the delta already
contains. An export check that does not restore `.specsync/hashes.json` writes
a gitignored file and breaks the read-only promise. A `pub(crate)` item is
itself an undocumented export. Declaring `.chough/sessions` would freeze session
writes into the scope.

## Constraints consulted

The cli lessons recorded on this change: `change check` refuses while
`tasks.md` has unchecked boxes, so human gates stay plain text; an approved
delta is frozen; `pub(crate)` is reported as an undocumented export, so new
items stay `pub(super)`; do not list `.chough/sessions` in affected paths.
AGENTS.md: small change, Fledge first, no second lifecycle, no human decision
recorded except from that human. DRIVE-29 is the intent. DRIVE-20 keeps the
existing change steps as chough passthroughs.
