# Native lifecycle and evidence

Use this reference for a scoped feature, contract change, or bug fix. Installed
`specsync change status <id> --json`, `ship-status`, and command help are authoritative
for the current state and recovery path. This example follows the verified 6.x
workflow; do not hardcode it over a different version's actual output.

## Define before implementing

Read relevant HI criteria and native module companions. Use the proposed FRD and
requirements to make the scope concrete: desired behavior, affected modules/paths,
acceptance checks, non-goals and unresolved product decisions. Requirements retain
their identity; current achievement belongs in evidence.

Create the native change, for example:

```bash
specsync change new "Describe one scoped outcome" --spec auth --path src/auth.rs --json
```

The returned ID and deterministic interview belong to SpecSync. Answer its actual
question IDs with `specsync change answer <id> <question-id> "<answer>" --json` and
complete its selected artifacts/deltas. Link HI IDs and `REQ-*` IDs in the native
requirements and change artifacts. Do not create a second Chough change state.

Present a concrete scope to the human when native policy requires human scope
approval and it has not already been given for that scope. Record
`specsync change approve <id> --actor <human>` only after a real, relevant human
decision. A generic instruction to keep working does not establish approval of a
new, materially different definition. Actor text is a portable claim, not identity
authentication. Do not substitute an agent review or a fabricated human label.

Bootstrap or draft work may be permitted by actual repository/native policy.
Label it honestly; do not claim that optional or unperformed lifecycle gates passed.

## Implement and verify

Follow the approved contract and keep paths, public API tables and companions
current. Run the project's meaningful product checks through Fledge tasks/lanes.
Use an isolated fixture for preservation, failure and session-resumption behavior
where those properties matter. A check that merely matches the implementation's
wording is not a product test.

Record each observation with:

- HI criterion IDs, requirement IDs and contract paths;
- actual revision/worktree and environment;
- command or procedure, observed result and supporting artifacts;
- limits, missing checks and any human decision provenance that actually exists.

`chough evidence add` can bind a local observation to its Git/worktree fingerprint;
it does not execute the procedure, authenticate a witness, verify a remote artifact,
or replace native SpecSync verification/review records.

When native status permits, use scoped verification:

```bash
specsync change check <id> --commit
```

This materializes approved deltas and records structural verification. Stage new
source files first when required by the native tool. Do not infer product-test
success from this command. Fix genuine failures before continuing; if evidence is
stale, follow the state-specific recovery action returned by native status.

## Review, archive, merge and verify delivery

Push only after the applicable local gates. Observe required hosted CI at the
actual product tip. Obtain the implementation review required by the repository
and record a native scoped human review only when that decision was actually made:

```bash
specsync change review <id> --reviewer <human>
specsync change ship <id>
```

The reviewer may be the scope approver where policy permits it. Review binds the
current workspace: do not commit between review and ship/finalize. Follow status
if prerequisites are incomplete. `finalize` is the explicit native archive command;
`ship` preflights and finalizes when eligible.

Archive on the same PR before merging. Commit/push the archive tip, observe its
required CI, and merge only after all active changes on that PR are archived.
Review and finalize sequentially when several changes share a PR; another archive
can stale accepted evidence. Do not assume that an already merged active change
can recover through a generic re-check; use the exact native recovery output.

Merge, package publication, release and deployment are separate observations.
Verify the actual requested delivery target and revision, then run the relevant
product/acceptance check there. Link its environment and result. If no deployment
was requested or performed, say that explicitly. Human acceptance remains a real
decision over the presented behavior, not a successful command exit.

## Compound without duplicating records

Native tasks capture remaining work; testing companions link observed checks;
context or existing `.3md` planes hold decisions and useful lessons. A lesson
should name its source, applicable scope, and the future action it changes. Mark
contradicted lessons revised/retired with the reason. The Chough session points
the next agent at these sources and the native next action instead of retaining an
ever-growing transcript.
