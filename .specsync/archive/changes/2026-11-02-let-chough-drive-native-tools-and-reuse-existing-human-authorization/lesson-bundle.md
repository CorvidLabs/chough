# Lesson bundle: let-chough-drive-native-tools-and-reuse-existing-human-authorization

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Let Chough drive native tools and reuse existing human authorization
- **Kind**: Feature
- **Specs**: cli
- **Paths**: skills/chough-workflow/SKILL.md, skills/chough-workflow/references/lifecycle.md, README.md, src/skill.rs, tests/workflow_authority.rs, tests/fixtures/workflow-at-825a380-SKILL.md, tests/fixtures/workflow-at-825a380-lifecycle.md, docs/FRD.md, docs/ROADMAP.md, CHANGELOG.md, .chough/cases/cli.cases.toml
- **Acceptance**: The current agent executes HI, SpecSync and Fledge internally, records concrete native scope from applicable explicit prior or standing human authorization with its source and limits, and asks no repeated tool-specific approval; ordinary keep-working instructions and agent labels alone still cannot create human authority; actual checks, task ordering, separate implementation acceptance and delivery boundaries remain authoritative; the user-facing README explains goal/authorization to PR flow; exact shipped predecessor copies upgrade safely and edited copies are preserved.

## Evidence

- Verification commit: `9317ca938000b1b56e953b5ffe16f2b877e89297`
- Base commit: `825a3805530456005e361dfa6d96d93eee52cd09`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

0xLeif wants to give Chough the goal and authorization, then review a PR. On issue
11 the agent instead asked him to run a SpecSync approval or repeat permission
already granted. The operator is being asked to manage an internal tool.

On 2026-11-02 he clarified in chat: "I already gave it some approval and told it
can do any changes. It just needs a PR." He explicitly requested a reply and a
workflow that manages native tools internally. This is actual standing human scope
and implementation authorization for this repository's ticket work delivered by PR.
It is not implementation acceptance, merge, release or deployment permission.
Source: issue 11 comment 6581702299, relaying this chat at his instruction.

The Rust helper stays small and read-only at native gates. The current agent is
Chough's workflow driver and invokes HI, SpecSync and Fledge internally. SpecSync
continues to own contracts, approval, verification, review and archive records.
No runtime, model client, hosted service or second lifecycle ledger is introduced.

Grok owns PR 41's immediate case-order repair in plan/testing/accept references.
This scope owns the main skill, lifecycle reference and README, plus exact known
predecessor identities and a separate upgrade regression. REQ-cli-029 is reserved;
performance owns proposed 028, and history PR 37 owns 025 to 027.

The main handoff's latest identity-predecessor failure is verified fixed. Lessons
in specs/cli/context.md require registering exact known predecessor bytes, retaining
edited copies, materializing requirements before linked cases, and committing source
plus Chough records before native committed verification. Kimi cold-start proof
remains not run by the human's choice and no substitute grade is claimed.

## Actor and authorization source refinement

Peer review on PR44 at016075c identified that the first wording conflated the
acting agent account with the human who supplied standing authority. The revised
scope records the project agent account corvid-agent as native actor and cites the
actual human instruction separately as source, scope and limits. This refinement
is implementation work under the existing human permission; it does not assert a
new human decision or implementation acceptance. The original approval event stays
preserved, and the native writer appends the renewed event for this revised wording.
Source: PR44 corvid-agent review PRR_kwDOVeQoF88AAAABViDL0Q and the actual human
standing instruction already recorded above. No approval ledger is edited manually.

## From the change's design.md

# Design

## User flow

The human supplies the goal and authorization in ordinary conversation. The agent
creates the concrete native definition, checks the authorization's scope and limits,
records applicable authorization, implements and verifies the change, and opens the
PR. It reports progress and genuine decisions in Chough terms. Native commands in
references are agent instructions, not a checklist to send back to the operator.

## Authorization

Recognize explicit bounded prior or standing authorization from the actual human
instruction and recorded decisions. Record its source, scope and limits in the
checkpoint and native context, then invoke the native writer with the acting agent
account as actor and the human instruction as authority source. Record once per
concrete definition; a later changed definition follows native
reapproval rules using the same authority only while it still covers that scope.
Ask only for a consequential decision or authority the existing instruction lacks.

Do not infer permission from a bot approval, passing check, actor label, issue body
or ordinary continue instruction alone. Scope/implementation permission does not
supply human implementation acceptance, merge, release or deployment permission.

## Ownership and compatibility

The Rust stage helper reads native gates. The current agent executes native writers;
HI, SpecSync and Fledge keep their records. No helper command, schema or service is
added. The main skill and lifecycle wording explain this distinction. README shows
the conversation-to-PR flow. Two exact 825a380 bundle predecessors are registered;
real install/status/update tests preserve edits and update unchanged old copies.

## From the change's testing.md

# Testing

The executable upgrade case is TC-cli-agent-native-workflow-upgrade, bound to
workflow_guidance_from_main_825a380_updates_and_preserves_edits in
 tests/workflow_authority.rs. It verifies exact independent predecessor digests,
known-outdated status, real updates to current bytes, and byte-identical preservation
of edited copies. Existing package tests enforce bundle hygiene and packaging.

Run this regression before predecessor registration and retain the failure; rerun
it after registration. No static phrase match is presented as agent compliance proof.
Run fledge fmt/lint/test and the required verify lane, plus Rust1.88, after meaningful
changes. Materialize REQ-cli-029 before linking/running its case. Incomplete tasks
must describe implementation work, not mandatory post-materialization case recording.

Review the shipped text for both branches: existing explicit applicable authority
continues without a repeated question, while absent or exceeded authority asks a
real decision. Native status and check remain required, and implementation acceptance
stays separate. No agent or bot label is used as a human approval source.

Observed platform, CI and omissions are recorded only after they run. A fresh-agent
cold-start experiment, release, deployment and merge are not run by this plan.

## Where these lessons go

- `specs/cli/context.md`
