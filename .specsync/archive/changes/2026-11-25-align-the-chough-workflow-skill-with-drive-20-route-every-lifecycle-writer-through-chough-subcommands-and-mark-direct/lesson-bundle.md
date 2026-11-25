# Lesson bundle: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Align the chough-workflow skill with DRIVE-20: route every lifecycle writer through chough subcommands and mark direct specsync change invocation as a recorded direct-native-write violation
- **Kind**: Feature
- **Specs**: cli
- **Paths**: skills/chough-workflow/SKILL.md, skills/chough-workflow/references/lifecycle.md, skills/chough-workflow/references/plan.md, skills/chough-workflow/references/checkpoint.md, skills/chough-workflow/references/accept.md, skills/chough-workflow/references/stages.md, skills/chough-workflow/references/validate.md, skills/chough-workflow/references/testing.md, docs/AGENTS.md
- **Acceptance**: The chough-workflow bundle routes every lifecycle writer through chough: the skill's Drive-the-workflow section states chough-first (chough change new/answer/check/approve/ship, chough stage, chough cases, chough evidence, chough gate) and names direct specsync change calls as recorded direct-native-write violations except for the steps chough does not wrap (review on standing authority, adopt, finalize, ship-status), each shown with its Timeline tracking; the SKILL.md line 'Run HI, SpecSync and Fledge yourself' becomes 'Run HI and Fledge yourself; chough drives SpecSync'; lifecycle.md and plan.md example commands use chough change forms; a repo check keeps the wrapper-only rule discoverable

## Evidence

- Verification commit: `fb0562045e8412aae2130e056dc90fbeb7ab5302`
- Base commit: `050e2c35b5f4f1f7c16d45e66ca697597803fb80`
- Verified by: `specsync check --spec cli`

## From the change's context.md

## Context

The incident (PR tob#38, 2026-11-25): the orchestrator ran specsync's change steps
directly (answer, check, review, ship) and the native change archived cleanly — but
the chough session trail never left intent. Chough's stage gates then refused to
advance (`refused:native-archived`) and its ship relay (REQ-cli-059) could not run
retroactively, so the session could not finish done. Leif's correction: "you need to
drive chough and then chough does all of it" — and asked how to make that structural.

The skill itself invites the mistake: SKILL.md line 17 says "Run HI, SpecSync and
Fledge yourself", and several reference files show direct `specsync change …` commands
as the agent's steps. That contradicted DRIVE-20 ("Agents call only chough to move a
change through SpecSync") from the day chough gained the change wrappers
(REQ-cli-057/059/061/067/071).

## From the change's testing.md

## Testing

- The check script runs in CI (docs lane) and locally; its failure output names the
  offending file + line.
- Human pass over the four reference files: every `specsync change` fenced command is
  either one of the four labeled exceptions or a chough form.
- No product code changes; `cargo test` unaffected but run once as regression.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-072 | scripts/check-skill-chough-first.sh passes (OK on the changed bundle), verified to FAIL on a planted violation (testing.md fixture, exit 1, then reverted); SKILL.md Drive-the-workflow now states the chough-first order citing DRIVE-20; lifecycle.md/plan.md/accept.md fenced commands labeled unwrapped-step or swapped to chough forms; fledge lanes ci+verify gained the skill-chough-first step; docs/lifecycle.md states the rule. |

## Where these lessons go

- `specs/cli/cli.3md`
