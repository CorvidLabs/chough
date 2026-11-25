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
