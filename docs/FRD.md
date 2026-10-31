# Functional requirements: foundation and agreed evolution

These are agent-derived functional requirements for the authorized first slice,
traced to the human's durable [LIFE criteria](../hi/life.md). The criteria describe
the intended product. This document refines implementation scope; evidence records
describe what has actually been observed. Neither is a list of completed work.

The human selected **current-agent-first v1**. The product consists primarily of a
portable workflow skill, templates, and Markdown conventions, supported by a small
Rust CLI. Named Rune session coordination is deferred.

## Current CLI scope

The following existing IDs retain their bootstrap meaning and native contract links.
They do not imply that the broader stage workflow below has been implemented.

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

## Skill-led behavior in this slice

The skill supplies the reusable development procedure: capture actual wants before
choosing a solution; propose appropriately sized FRD requirements; use native SpecSync
records; run real product checks; reproduce and deduplicate bugs; verify the actual
delivery target; carry relevant decisions and scoped lessons into future work.

This procedure is not an automated proof that every LIFE criterion is fulfilled.
Bug issue creation, human approval, GitHub merge, release, deployment, and external
tool adapters retain their own authorization and verification requirements.

## Agreed evolution: design requirements, not implemented scope

The human direction below refines LIFE-1 through LIFE-7. It is product design for
the next evolution, not a newly approved native change definition, new permanent
HI/REQ identities, or passing acceptance evidence. Preserve existing IDs and size
new native requirements in the actual scoped preparation loop.

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

## Outside the current implementation

No hosted service, model-provider client, custom agent runtime, automatic agent
coordination, universal slash-command parser, replacement native change state,
automatic human authentication, or deployment engine is included. The
[roadmap](ROADMAP.md) records proposed extensions separately.
