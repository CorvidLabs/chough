# Lesson bundle: integrate-the-approved-case-ordering-and-native-authority-workflow-changes

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Integrate the approved case ordering and native authority workflow changes
- **Kind**: Refactor
- **Paths**: skills/chough-workflow/references/plan.md, skills/chough-workflow/references/accept.md, skills/chough-workflow/references/testing.md, .agents/skills/chough-workflow/, .claude/skills/chough-workflow/, src/skill.rs, tests/package.rs, tests/workflow_authority.rs, specs/cli/cli.spec.md, specs/cli/requirements.md, specs/cli/testing.md, specs/cli/context.md, CHANGELOG.md, docs/ROADMAP.md, .chough/cases/cli.cases.toml
- **Acceptance**: Merge main8fb0ea7 into PR44 without rebase or force, preserve case-ordering and acting-agent/human-source guidance across canonical and both host bundles, retain all existing predecessor identities/tests and both approved requirement effects with monotonic canonical version, select the complete PR44 checkpoint current-cycle group while preserving additive collections and every transition occurrence, leave existing archives/cold-start/observations/artifacts byte-identical, and pass all required Fledge steps, relevant bound cases and complete Rust1.95/1.88 tests before normal push. Native review/ship remains with the root closing session.

## Evidence

- Verification commit: `26a5acd72dc7b901d6f44e42f56bc1c1ab232d60`
- Base commit: `6fde285473f3fa3b6efd45bc0582b3826ec8775d`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

PR44's three native scopes are archived at branch head
6fde285473f3fa3b6efd45bc0582b3826ec8775d. PR41 merged to main as
8fb0ea7d1fd49580cfcb94dd4d4feab326ab53b7. Their shared base is825a380.
Both branches changed skill ordering/authority references, bundled predecessor
metadata and canonical version17. This scope performs bounded integration of
already approved meanings, not a new product requirement or helper behavior.

Actual human authority is0xLeif's standing2026-11-02 chat instruction authorizing
needed repository changes through PRs, relayed at
https://github.com/CorvidLabs/chough/issues/11#issuecomment-6581702299 , together
with his later request, "Can you finish up all these?", after the existing PR
status and closeout gates were presented. The acting project account is
corvid-agent. The covered work is this PR44 main integration and qualification.
Merge, release, deployment, live cleanup and fabricated acceptance stay excluded.

Root owns native review/ship. The integration worker may approve this concrete
scope under the actual standing authority, run checks and push normally. Existing
approval events and all frozen proof remain unchanged. Root selected the complete
PR44 current-cycle checkpoint group; that is branch ownership, not gate closure.
Original merge inputs and the selection proof are retained under/tmp.

The latest prior failure is the timing budget regression recording no initial
stub call. Its bug remains open with cause unknown. Any recurrence is preserved,
not hidden by weakened assertions or excluded tests.

## From the change's design.md

# Integration design

Canonical references combine independent approved sections rather than selecting
one branch wholesale. The package remains driven by one canonical skill bundle;
tracked openai and claude copies receive identical integrated reference bytes.
Predecessor identity entries are additive, with identical duplicates represented
once. Existing frozen fixtures and digest assertions remain unchanged.

Canonical version advances from17 on each side to18 for the combined approved
content. Requirement009's ordering clause and requirement029's authority contract
remain present. Changelog/roadmap retain both delivered descriptions.

The checkpoint retains one current-cycle group. Original base/ours/theirs files
are immutable diagnostics. If the native helper rejects divergent groups, a
separate selected-cycle operand explicitly carries root's PR44 choice; the helper
still merges additive collections. Transition JSONL retains the base prefix and
both branch suffixes, including identical independent occurrences.

## From the change's testing.md

# Verification

Before merge, record refs and all tracked archive, cold-start, observation and
artifact Git blob identities from both parents. After resolution every existing
blob must match its corresponding parent and all additive checkpoint collections
and transition occurrences must survive. No protected conflict is resolved by
rewriting its content.

Run the workflow authority/host-copy and shipped case-order regressions, relevant
bound cases, the full nine-step Fledge verify lane and all tests under both
Rust1.95 and Rust1.88. Do not exclude the prior flaky timing regression. A failure
is captured before corrective source edits or reruns, with its limits retained.

Cases execute only after required canonical materialization. No task checkbox
requires running/recording Chough cases before that check. Qualification outcomes
are observations, separate from native structural verification, human acceptance,
hosted CI, merge, release or deployment. No cold-start run or grade is added.

## Where these lessons go

This change declared no affected specs, so there is no module context to fold into.
