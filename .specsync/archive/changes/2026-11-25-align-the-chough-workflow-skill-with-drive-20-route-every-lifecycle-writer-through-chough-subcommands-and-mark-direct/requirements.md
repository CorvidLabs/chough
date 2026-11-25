## Requirements

REQ-cli-072 — The chough-workflow skill routes every lifecycle writer through
chough. The Drive-the-workflow section states the chough-first order (chough
session → chough change new/answer/ready/approve → chough stage advance while the
native change is live → chough change check/ship, chough cases, chough evidence,
chough gate) and marks direct `specsync change` invocation a violation that lands as
a `direct-native-write` Timeline entry (DRIVE-21), legitimate only for the steps
chough does not wrap: `specsync change review` on standing authority (lifecycle.md
already shows it as the exception), `adopt`, `finalize`, `ship-status` — each shown
with its tracking note. SKILL.md line 17 becomes "Run HI and Fledge yourself; chough
drives SpecSync (DRIVE-20)". Example command blocks in lifecycle.md and plan.md use
the `chough change` forms. Acceptance: a repo check (script or grep in CI) lists
every `specsync change` occurrence in the bundle and each is either inside one of the
four unwrapped-step contexts or fails the check; a fresh reading of SKILL.md alone
tells an agent never to call specsync change directly for wrapped steps.
