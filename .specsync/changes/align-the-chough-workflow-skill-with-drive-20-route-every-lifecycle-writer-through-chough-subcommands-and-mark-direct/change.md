---
id: align-the-chough-workflow-skill-with-drive-20-route-every-lifecycle-writer-through-chough-subcommands-and-mark-direct
state: verifying
type: feature
base_commit: 050e2c35b5f4f1f7c16d45e66ca697597803fb80
---

# Align the chough-workflow skill with DRIVE-20: route every lifecycle writer through chough subcommands and mark direct specsync change invocation as a recorded direct-native-write violation

## Intent

Align the chough-workflow skill with DRIVE-20: route every lifecycle writer through chough subcommands and mark direct specsync change invocation as a recorded direct-native-write violation

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The chough-workflow bundle routes every lifecycle writer through chough: the skill's Drive-the-workflow section states chough-first (chough change new/answer/check/approve/ship, chough stage, chough cases, chough evidence, chough gate) and names direct specsync change calls as recorded direct-native-write violations except for the steps chough does not wrap (review on standing authority, adopt, finalize, ship-status), each shown with its Timeline tracking; the SKILL.md line 'Run HI, SpecSync and Fledge yourself' becomes 'Run HI and Fledge yourself; chough drives SpecSync'; lifecycle.md and plan.md example commands use chough change forms; a repo check keeps the wrapper-only rule discoverable

## No-spec Rationale

Not applicable
