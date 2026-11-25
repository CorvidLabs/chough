## ADDED

### REQUIREMENT REQ-cli-072

Chough-first skill routing. The chough-workflow skill SHALL route every lifecycle
writer through chough subcommands: the Drive-the-workflow section of SKILL.md SHALL
state that agents drive changes with `chough change` (new, answer, ready, approve,
check, ship), `chough stage`, `chough cases`, `chough evidence` and `chough gate`,
and SHALL present direct `specsync change` invocation for a wrapped step as a
violation recorded as a `direct-native-write` Timeline entry (DRIVE-21). The four
steps chough does not wrap — `specsync change review` on standing authority,
`adopt`, `finalize`, `ship-status` — SHALL be shown only in their unwrapped contexts,
each labeled as recording a `direct-native-write` entry. SKILL.md SHALL replace the
"Run HI, SpecSync and Fledge yourself" instruction with one that sends HI and Fledge
to the agent and SpecSync to chough, citing DRIVE-20. Example command blocks in
lifecycle.md and plan.md SHALL use the `chough change` forms. A repository check
SHALL fail when a fenced code block in the bundled skill shows `specsync change`
outside the four unwrapped contexts. Satisfies DRIVE-20.

Acceptance Criteria:
- A fresh reading of SKILL.md alone tells an agent to drive wrapped steps only through
  chough and names DRIVE-20.
- The check script passes on the changed bundle and fails when a fenced `specsync
  change` command is added outside the four unwrapped contexts.
- lifecycle.md and plan.md contain no unlabeled fenced `specsync change` command for a
  wrapped step.
