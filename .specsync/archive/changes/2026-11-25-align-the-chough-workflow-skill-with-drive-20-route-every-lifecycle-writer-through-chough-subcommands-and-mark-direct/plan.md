## Plan

## Approach
Docs-only change in the bundled skill + one house-rules file. Edit SKILL.md's
Drive-the-workflow opening; edit lifecycle.md, plan.md, checkpoint.md, accept.md to
show `chough change` forms for wrapped steps and keep the four unwrapped-step
`specsync change` commands explicitly labeled "unwrapped — records as
direct-native-write (DRIVE-21)". Add a small check script
(`scripts/check-skill-chough-first.sh`) that greps the bundle for `specsync change`
outside the allowed contexts and fails CI otherwise; wire it into the existing
skill/docs check lane if one exists, else document its manual run in docs.

## Out of scope
No Rust source changes; no new chough subcommands (a code-level refusal was offered
and not chosen — the skill + tracking is the adopted enforcement); no hi or fledge
changes.

## Steps
1. SKILL.md: rewrite the Drive-the-workflow opening + resume step 2's specsync
   pointer to `chough change status`.
2. lifecycle.md / plan.md / checkpoint.md / accept.md: swap example commands to
   chough forms; label the four unwrapped exceptions.
3. stages.md / validate.md / testing.md: same sweep for any agent-facing
   `specsync change` instruction.
4. Add scripts/check-skill-chough-first.sh + wire into CI docs lane.
5. docs/AGENTS.md (chough repo): one line stating the chough-first rule.

## Risks
- The check may flag prose mentions of specsync commands; mitigate by anchoring it to
  fenced code blocks only.
- Agents with older installed skill copies keep the old wording; `chough skill
  status` already surfaces stale copies — mention it in the skill text.

## Constraints consulted

DRIVE-20 and DRIVE-21 (hi/process intent): chough-first routing with direct writes
tracked, never forbidden outright. DRIVE-7: no em dashes in bundled skill files.
REQ-cli-057/059/067/071 define the existing wrappers; REQ-cli-059's ship relay is the
step that closes the review gate and moves the session to done, so the skill must send
agents there, not to specsync ship.
