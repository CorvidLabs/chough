# Technical design and plan

SpecSync drives this stage with its own adaptive artifacts. Read source
instructions, existing architecture and the native lessons first; `specsync change
new` prints the lessons of the modules you declare.

1. Create the change with installed help as the authority, for example
   `specsync change new "<one outcome>" --spec <module> --path <path> --json`.
   Declare every path you expect to touch: scope freezes at approval.
2. Answer the deterministic interview with `specsync change answer <id> <question>
   "<answer>"`.
3. Complete the selected artifacts: requirements, research, design, plan (approach,
   out of scope, steps, risks, constraints consulted), tasks, testing, docs, and a
   semantic delta per affected module.
4. Estimate meaningful increments with relative size and the main uncertainty. Do not
   invent precise hours. Run `chough timing` and use its history ranges (per stage, bug
   fix and human gate) as the basis: they are elapsed wall-clock time, not effort, and
   estimates from recorded history, not targets. When it reports too few samples, say
   that history is too thin and estimate by relative size alone; never present a range
   as a deadline.
5. If several agents will build, assign file or module ownership in the plan.
6. Record the change: `chough checkpoint --native-change <id>`.

## One approval

Present one coherent package: what will change, the contract delta, the plan, the
verification approach, and any consequential open choice with your recommendation.
Ask one concise question. Record `chough checkpoint --awaiting-human "Scope approval
of <id>"`, then keep doing independent preparation.

Only after the human actually approves, run `specsync change approve <id> --actor
"<their name>"` and clear the gate with `chough checkpoint --clear-awaiting-human`.
The actor is a label, not authenticated identity.

Lessons learned from rehearsing SpecSync 6.1: the delta is frozen once approved and
`change check` refuses an edited delta; `change check` also refuses while `tasks.md`
has unchecked boxes, so list human gates as plain text rather than checkboxes.

## What needs reapproval

A change to approved intent, contract, acceptance criteria or affected scope needs
native reapproval. Routine implementation, tests, evidence, delta materialization
and lifecycle metadata do not.
