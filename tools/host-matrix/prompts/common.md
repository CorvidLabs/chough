This is an automated end-to-end test of the Chough workflow (the Chough host-matrix
harness). You are the agent working in this repository. Nobody will answer questions
during this run.

- Follow AGENTS.md and the Chough workflow skill it points to. Run Chough, HI,
  SpecSync and cargo yourself.
- Wherever Chough or a case asks for an environment, use `{{ENVIRONMENT}}`.
- Never approve, review or decide anything for a human, and never write a person's
  name into a record. Between your runs the harness stands in for the human at each
  gate; every such decision is labelled SIMULATED and is not a human decision.
- When you reach a human gate, record it with `chough checkpoint --awaiting-human
  "<gate>"` and stop there.
- Commit all your work with git before you stop. Never push.
- End your final reply with exactly these two lines:
  STAGE: <the Chough stage the work is at now>
  NEXT: <the next action>

