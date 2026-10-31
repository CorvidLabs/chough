# Functional requirements

These are agent-derived functional requirements for the authorized first slice,
traced to the human's durable [LIFE criteria](../hi/life.md). The criteria describe
the intended product. This document refines implementation scope; evidence records
describe what has actually been observed. Neither is a list of completed work.

The human selected **current-agent-first v1**. The product consists primarily of a
portable workflow skill, templates, and Markdown conventions, supported by a small
Rust CLI. Named Rune session coordination is deferred.

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
Adding Chough's own records does not itself invalidate earlier observations.

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

## Outside this slice

No hosted service, model-provider client, custom agent runtime, automatic agent
coordination, universal slash-command parser, replacement native change state,
automatic human authentication, or deployment engine is included. The
[roadmap](ROADMAP.md) records proposed extensions separately.
