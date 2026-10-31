# Reproduce, classify, deduplicate and report a bug

Use this procedure for a suspected defect, including a failing acceptance check.
The report can remain a local draft; a request to investigate does not by itself
authorize posting an issue, comment, or message to another person.

## Establish the failure

Find the applicable HI criterion and module contract/requirement. If intended
behavior is unclear, preserve the observation and ask the smallest product question
that distinguishes a bug from a new request. Do not silently rewrite intent to
match the implementation.

Record the exact revision, dirty worktree state, tool/runtime versions and relevant
environment. Reduce the reproduction to the smallest safe fixture, data and steps
that still show the failure. Save actual expected/observed behavior and output;
label an unexecuted reproduction as unverified. Avoid real credentials, private
customer data or destructive production actions when a local fixture suffices.

Classify based on evidence, keeping confidence visible:

| Class | Evidence needed |
| --- | --- |
| Product defect | Observed behavior contradicts an applicable criterion/contract. |
| Regression | It fails now and a demonstrated earlier revision passes the same case. |
| Environment/tooling | Failure is isolated to an identified environment/tool input. |
| Requirement gap | Behavior is observed, but the intended outcome is unresolved. |
| Evidence gap | Existing claims cannot be reproduced or their evidence is missing/stale. |

State impact separately from severity/priority. A plausible cause is a hypothesis
until a test or investigation supports it. Do not call something a regression
solely because the reporter remembers that it used to work.

## Search before filing

Search local bug records, native tasks and relevant requirements using `rg`. Use
Fledge's GitHub issue list/view commands when that connected repository is in
scope, for example:

```bash
fledge github issues --state all --limit 100 --json
fledge github issues view 42 --comments --json
```

The list is bounded; if it cannot establish adequate coverage, report that limit
and use an available supported search interface. Compare symptom, trigger, affected
criterion and likely cause, not title alone. A match can be related without being
the same defect. Link a verified duplicate rather than creating a second issue;
preserve any new reproduction/evidence for an authorized update. Do not close or
comment on an issue merely because it looks related.

## Draft a useful report

Keep the report alongside the repository's existing bug records or attach it to
the relevant native change. Use this shape, replacing the example field guidance
with actual findings and stating unknowns explicitly:

```markdown
# Observed symptom and trigger

Classification and confidence: what the evidence establishes
Impact: affected user action and practical consequence
Traceability: HI IDs; REQ IDs; contract paths; related issue/change URLs

## Reproduction
Revision/worktree: exact commit and relevant local changes
Environment: OS/runtime/tool versions and relevant configuration
Fixture: path or sanitized input
Steps: smallest executed sequence
Expected: behavior supported by intent/contract, or unresolved question
Observed: actual behavior and error/output
Frequency: observed runs and outcome, or unknown

## Evidence and deduplication
Artifacts: logs/screenshots/test result paths and any Chough evidence IDs
Search scope: local sources, issue query/list limit, date
Existing matches: duplicate/related/unrelated with reasons, or none found
Limits: missing observations and unverified hypotheses

## Proposed next action
Smallest test, product decision, or native change needed
```

When issue creation is authorized, use the available GitHub connector or Fledge
issue command after reviewing the exact report text; preserve real newlines and
quote it safely. Return the resulting URL. Without that authorization, deliver the
draft and its location. Do not post messages to people as an implicit part of a
bug investigation.

## Repair and close the loop

Use the native SpecSync bug-fix change lifecycle in
[lifecycle.md](lifecycle.md), with the failing behavior as an acceptance case. Show
the reproduction fail before the fix where practical, then pass afterward; run
relevant adjacent product checks. Link the new evidence to the same durable HI
and requirement IDs. An untested fix remains an unverified hypothesis.

Verify the delivered target if delivery is in scope. Resolve an issue only when
the authorized workflow and actual evidence support it; passing a local test does
not establish a production fix. Record a focused lesson in native context/`.3md`
only when the source, scope and future use are clear, and update the session with
the actual next action.
