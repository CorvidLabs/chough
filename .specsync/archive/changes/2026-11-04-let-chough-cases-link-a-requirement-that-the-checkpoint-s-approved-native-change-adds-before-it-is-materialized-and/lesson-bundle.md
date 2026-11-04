# Lesson bundle: let-chough-cases-link-a-requirement-that-the-checkpoint-s-approved-native-change-adds-before-it-is-materialized-and

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Let Chough cases link a requirement that the checkpoint's approved native change adds before it is materialized, and document how the test stage links failing-first evidence (DRIVE-6, Raven BUG-0004)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/cases.rs, src/evidence.rs, src/stage.rs, tests/cases.rs, tests/cli.rs, docs/cli.md, docs/lifecycle.md, skills/chough-workflow/references/testing.md, skills/chough-workflow/references/plan.md, skills/chough-workflow/references/lifecycle.md, skills/chough-workflow/references/accept.md, CHANGELOG.md, tests/package.rs, .agents/skills/chough-workflow/references/lifecycle.md, .agents/skills/chough-workflow/references/plan.md, .agents/skills/chough-workflow/references/accept.md, .agents/skills/chough-workflow/references/testing.md
- **Acceptance**: Source: Leif's brief t811u item 6, "Want: either the test stage links failing-first evidence (the commit that adds failing tests, the receipt showing them fail on the base, and the passing run) and cases accept REQ ids from the active approved change, or test folds into build with clear guidance. Pick one and document it.", and Raven BUG-0004 ("chough cases check rejects a requirement that exists only in an approved but not yet materialized SpecSync delta"). The parent relayed the 6:03 PM MST decision to take the first option. Outcomes: (1) a case requirement absent from the module's requirements.md is accepted by cases check, run and record when the checkpoint's native change is approved, implementing or verifying with approval_valid true, correction_valid not false and its approved deltas unchanged (REQ-cli-022), and that change's delta for the case file's module adds or modifies the ID; (2) such a link is reported in cases check output under approved_delta_requirements with the change and module; (3) at most one native status read per command, and none when every link resolves in a companion; (4) a typo, another module's delta, an edited delta, an invalid approval or correction, a draft or archived change, no checkpoint change, no native configuration or a failed native read refuses as before, and the message names why the delta did not resolve it when a change was consulted; (5) evidence add and every stage gate are unchanged, Chough decides no approval and records no human approval or review; (6) the plan, testing, accept and lifecycle references and docs say cases may link approved delta requirements before change check and describe how the test stage links failing-first evidence: the commit that adds the failing tests, a failed observation on the base linked to a bug, and the passing run after the fix; (7) the new tests fail before the fix; fledge lanes run verify passes

## Evidence

- Verification commit: `d2fb69715cf75c104becc8094235988f3e24db44`
- Base commit: `d673efc9fadd00953cf9e42dad96e08f3c76103c`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

On Raven on 2026-11-03, `chough cases check` refused a case linking a requirement that existed only in the approved, not yet materialized SpecSync delta (Raven BUG-0004). SpecSync materializes a delta only in `specsync change check`, which refuses while `tasks.md` has an unchecked box, so cases could be linked only after the implementation was finished. Failing-first evidence therefore never went through `chough cases`, and the test stage passed in 3 seconds with no cases recorded (Leif's brief t811u item 6).

Leif's brief offers two options; the parent relayed the 6:03 PM MST decision to make the test stage real: cases accept requirement IDs from the active approved change, and the test stage documents how failing-first evidence is linked. Stage gates do not change.

## From the change's design.md

# Design

`cases::check_links` keeps the canonical companion lookup. On a miss it asks a per-command resolver, created once per `cases check`, `run` or `record`. The resolver reads the checkpoint's `native_change` and, only when one is recorded and SpecSync is configured, makes one `specsync change status <id> --json` read, then reuses that result for every case in the command. It accepts the ID only when the state is approved, implementing or verifying, `approval_valid` is true, `correction_valid` is not false, `stage::approved_deltas` reports the deltas unchanged, and `.specsync/changes/<id>/deltas/<module>.md` holds `### REQUIREMENT <ID>` under `## ADDED` or `## MODIFIED`. Any other outcome keeps the existing refusal and appends the reason. `evidence add` keeps its canonical-only lookup.

## From the change's testing.md

# Testing

`tests/cases.rs` with the SpecSync stub and an approved `cli` delta adding REQ-cli-090: `cases check`, `run` and `record` accept a case linking it on one native read and `cases check` lists it under `approved_delta_requirements`; an edited delta, approval_valid false, correction_valid false, draft and archived states, a delta of another module, no checkpoint change, no native configuration and a failed read refuse with the reason; a typo still refuses. The tests fail before the fix. `fledge lanes run verify` runs the full suite.

## Requirement evidence

| Requirement | Test |
|---|---|
| REQ-cli-009 | `tests/cases.rs`: `a_requirement_the_approved_delta_adds_is_linkable_before_materialization` and `an_unapproved_or_changed_delta_does_not_resolve_a_requirement`; the existing `an_unresolved_requirement_is_refused_with_the_check_first_hint` still passes unchanged. |

## Where these lessons go

- `specs/cli/context.md`
