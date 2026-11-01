# Functional requirements: foundation and agreed evolution

These are agent-derived functional requirements traced to the human's durable
[LIFE](../hi/life.md) and [FLOW](../hi/flow.md) criteria. REQ-cli-001 to REQ-cli-006
come from the authorized first slice; REQ-cli-007 to REQ-cli-017 come from the
approved native change `evolve-chough-into-a-resumable-stage-workflow`. The criteria describe
the intended product. This document refines implementation scope; evidence records
describe what has actually been observed. Neither is a list of completed work.

The human selected **current-agent-first v1**. The product consists primarily of a
portable workflow skill, templates, and Markdown conventions, supported by a small
Rust CLI. Named Rune session coordination is deferred.

## Foundation scope (REQ-cli-001 to REQ-cli-006)

The following IDs keep their bootstrap meaning and native contract links. The stage
workflow is specified separately below by REQ-cli-007 to REQ-cli-017.

## REQ-cli-001: Preserve projects during initialization

The helper SHALL initialize its focused session and discoverable workflow skill
without overwriting user-edited files. Repeating initialization SHALL preserve the
existing session and skill.

Acceptance: initialize an isolated repository, edit both generated files, initialize
again, and observe byte-for-byte preservation. Unsafe destinations fail visibly.

Trace: LIFE-4, LIFE-6. Contract: [CLI](../specs/cli/cli.spec.md).

## REQ-cli-002: Resume from focused context

The helper SHALL assemble deterministic context from applicable instructions,
the scoped HI export, the selected modules' contracts and companions, and the
session objective, decisions, open questions, and next action. Missing, oversized,
or unreadable sources SHALL be explicit. It SHALL prefer the native module `.3md`
lesson source where present and identify limits on interpreting that source.

Acceptance: assemble context twice from unchanged inputs and obtain the same output;
edit the session and observe the resumed objective and next action. Unselected module
history does not enter the packet.

Trace: LIFE-3, LIFE-6. Contract: [CLI](../specs/cli/cli.spec.md).

## REQ-cli-003: Delegate native state and next actions

The helper SHALL report the native SpecSync status response when available and
SHALL preserve unknown or failed tool states. It SHALL report its session guidance
and evidence limitations without manufacturing a second SpecSync state machine.

Acceptance: an unavailable tool, absent configuration, and an actual native response
remain distinguishable. No inferred human approval, merge, release, or deployment
appears as established fact.

Trace: LIFE-1, LIFE-6, LIFE-7. Contract: [CLI](../specs/cli/cli.spec.md).

## REQ-cli-004: Bind observations to their scope

The helper SHALL bind each observation to a Git revision and worktree fingerprint,
an explicit environment, a kind, an outcome, and supporting artifacts. Status SHALL
distinguish current observations from stale observations and environment mismatches.
An observation is an assertion by its recorder, not authenticated human acceptance.

Acceptance: a recorded observation is current for unchanged inputs in the same
environment; changing code or the requested environment makes that limitation visible.
Writing Chough's own records without a new commit does not itself invalidate earlier
observations. Every new HEAD, including a metadata-only commit, remains stale under
the current implementation.

Trace: LIFE-2, LIFE-7. Contract: [CLI](../specs/cli/cli.spec.md).

## REQ-cli-005: Preserve traceability

An observation SHALL retain links to durable HI criterion IDs, derived requirement
IDs, affected contract paths, and supporting artifacts. Invalid records SHALL fail
validation rather than appear as trustworthy evidence.

Acceptance: recorded links survive serialization and resumed status. A malformed or
unsupported record produces an actionable diagnostic. The helper does not change HI
criteria when evidence changes.

Trace: LIFE-1, LIFE-2, LIFE-5. Contract: [CLI](../specs/cli/cli.spec.md).

## REQ-cli-006: Keep local context bounded and safe

The helper SHALL constrain its managed records and selected project sources to
validated locations, reject path traversal and symlink escapes, bound loaded content,
and reject unsupported schema versions. It SHALL pass tool arguments directly rather
than evaluating user strings as shell code.

Acceptance: isolated fixtures exercise traversal and symlink attempts, unsupported
records, bounded context, and direct argument handling.

Trace: LIFE-3, LIFE-4. Contract: [CLI](../specs/cli/cli.spec.md).

## Resumable stage workflow (REQ-cli-007 to REQ-cli-017)

The canonical text of each requirement is in
[specs/cli/requirements.md](../specs/cli/requirements.md); this section gives the
cross-module view and its traceability without restating it.

| Requirement | Observable behavior | Criteria | Verification |
|---|---|---|---|
| REQ-cli-007 | One schema 2 checkpoint; explicit migration keeps schema 1 bytes | FLOW-13, LIFE-3 | tests/stages.rs; cold-start exercise |
| REQ-cli-008 | Stage routing with append-only history; build and closing gated by native state | FLOW-2, FLOW-5, FLOW-10, LIFE-1 | tests/stages.rs; feature-and-bug exercise |
| REQ-cli-009 | `chough-testing/1` definitions with stable IDs and position-free digests | FLOW-7, LIFE-2 | tests/cases.rs |
| REQ-cli-010 | Focused selection with tracks and stated omissions | FLOW-6 | tests/cases.rs; both exercises |
| REQ-cli-011 | Explicit case execution; vacuous and environment failures are not-run | FLOW-7, FLOW-9, LIFE-7 | tests/cases.rs, tests/optional.rs |
| REQ-cli-012 | Local backlog with duplicate search and regression baselines | FLOW-8, LIFE-5 | tests/bugs.rs; feature-and-bug exercise |
| REQ-cli-013 | Standing preferences separate from run consent | FLOW-9, FLOW-11 | tests/optional.rs, tests/stages.rs |
| REQ-cli-014 | Release only when authorized; distinct delivery and dry-run kinds | FLOW-11, LIFE-7 | tests/stages.rs, tests/cases.rs |
| REQ-cli-015 | Optional librarian with source-linked snapshots and index | FLOW-12 | tests/optional.rs |
| REQ-cli-016 | Per-host skill install; updates never replace edits | FLOW-1, LIFE-4 | tests/optional.rs; host discovery checks |
| REQ-cli-017 | Observation schema 2; schema 1 still read | LIFE-2, LIFE-7, FLOW-7 | tests/cases.rs |
| REQ-cli-020 | Read-only timing report from existing records, with timestamped awaiting-human gate records; elapsed wall-clock labels, unknowns never zero, ranges that are estimates and not targets | LIFE-10, LIFE-3, LIFE-6, LIFE-7 | tests/timing.rs |

Skill-led behavior with no helper requirement, verified by exercises rather than
helper tests: intent interviewing and role synthesis (FLOW-3,
[intent exercise](exercises/intent.md)), the design stage (FLOW-4,
[visual exercise](exercises/visual-design.md)), and the one scope approval (FLOW-5,
this change's own native record).

Excluded: APS as a checkpoint backend (qualified and deferred, see D-010), Magpie,
Rune, Trust, Augur and Attest integration, real releases or live monitoring,
publishing issues, and reuse of observations after metadata-only commits.

## Skill-led behavior in the first slice

The skill supplies the reusable development procedure: capture actual wants before
choosing a solution; propose appropriately sized FRD requirements; use native SpecSync
records; run real product checks; reproduce and deduplicate bugs; verify the actual
delivery target; carry relevant decisions and scoped lessons into future work.

This procedure is not an automated proof that every LIFE criterion is fulfilled.
Bug issue creation, human approval, GitHub merge, release, deployment, and external
tool adapters retain their own authorization and verification requirements.

## Agreed evolution: design provenance

This table was written (PR #1) before the evolution was scoped, as product design
rather than approved scope. It has since been captured as FLOW-1 to FLOW-13 and
REQ-cli-007 to REQ-cli-017 in the approved native change
`evolve-chough-into-a-resumable-stage-workflow`; the section above is authoritative
for requirements, and this table is kept as the design record they came from. Two
candidates it names were resolved there: APS was qualified and deferred (D-010) and
a TOML profile was chosen over 3md (D-011).

| Area | Agreed behavior and acceptance target |
|---|---|
| Intent and review | HI wording/IDs → appropriately sized FRD/AC links; relevant product/QA/design/accessibility/engineering/architecture role review, synthesized consequential questions, and explicit intent-stage approval. Conventional choices proceed; alignment percentages are judgment, not per-item scoring gates. |
| Conditional design | One canonical evolving local HTML/CSS prototype, interactions/states/viewports as relevant, companion Markdown with HI/AC/FRD mappings, actual rendered/source review and human approval of prototype plus notes. Nonvisual work skips with a reason. |
| Technical planning/build | SpecSync adaptive design/constraints/architecture/estimates/tasks/orchestration, one coherent initial human scope approval, then scoped implementation through Fledge. Required human implementation review occurs at acceptance before routine native archival. |
| Focused verification | Affected AC/FRD and relevant regressions; functional/basic always, copy/localization and design QA when applicable. Stable test-case IDs, explicit expected outcomes/methods/environments/artifacts; definitions separate from execution results. 3md profile is a candidate to qualify. |
| Local bug loop | Persistent reproduction/expected/actual/environment/revision/evidence/status; local dedup and live backlog. Blocking bugs repair/retest; out-of-scope findings queue without scope expansion or automatic external posting. |
| Optional validation | Routine checks continue naturally. Optional extended/background checks respect explicit run-only versus standing project preferences and pause/stop. Findings persist locally; auto-investigation policy remains open, default local capture. Retry/watch is not soak/live health. |
| Native closeout / optional release | Genuine acceptance/review before eligible same-PR native archive, archive CI and merge. Release requires request or explicit setting; publication/deployment/health remain separate observations. |
| Optional librarian | Project-local, callable from any stage and returns to its caller; source-linked history/snapshots, current index, focused briefs and visible unresolved bugs. Canonical living sources stay with native owners. |
| Core checkpoint/cold resume | Save objective/stage/scope refs/files/IDs/decisions/questions/preferences/work/tests/bugs/blockers/next action independently of the librarian. Fresh agent with no original conversation verifies real repo/native state and resumes the same scope/gates; demonstrate with a real cold-start exercise. |

Every stage may iterate/backtrack/reset/pause/resume or skip when inapplicable;
preserve history and refresh affected evidence through its owner. Current-agent-first
remains agent-neutral, with sequential role passes where subagents are unavailable.
APS qualification is optional and must avoid a second writable session authority.
See [lifecycle design](lifecycle.md), [architecture](architecture.md), and
[integration boundaries](integrations.md) for detail and remaining choices.

## Outside the current scope

No hosted service, model-provider client, custom agent runtime, automatic agent
coordination, universal slash-command parser, replacement native change state,
automatic human authentication, or deployment engine is included. The
[roadmap](ROADMAP.md) records proposed extensions separately.
