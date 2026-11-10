# Technical design and plan

SpecSync drives this stage with its own adaptive artifacts. Read source
instructions, existing architecture and the native lessons first; `specsync change
new` prints the lessons of the modules you declare.

1. Create the change with installed help as the authority, for example
   `chough change new "<one outcome>" --spec <module> --path <path>`.
   Declare every path you expect to touch: scope freezes at approval.
2. Answer the deterministic interview with `chough change answer <id> <question>
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

Prepare one coherent concrete package: outcome, contract delta, plan, verification
and consequential choices. Apply the main skill's authorization rule before asking
for a decision. If an actual prior or standing human instruction covers this scope,
that authorization already exists: record its source, scope and limits in the
checkpoint and native context, then run the native operation yourself.

```sh
specsync change approve <id> --actor "<agent-account>" --note "<human authority source, covered scope and limits>"
chough checkpoint --clear-awaiting-human
```

The acting agent account is the actor; the human instruction supplies the authority.
Do not use the human's name merely because they authorized the agent's work, and do
not ask the operator to execute this command or repeat covered permission.

When a consequential decision or authority is actually missing, ask one concise
question and record that real gate with `chough checkpoint --awaiting-human
"Scope decision of <id>"`. Keep doing independent preparation while it is pending.
After the missing decision arrives, record it on the human's behalf and perform the
native steps as the agent:

```sh
chough checkpoint --close-gate "Scope decision of <id>" --decided-by "<who>" \
  --words "<their exact words>" --where "<where and when, and who relayed it>" \
  --recorded-by "<agent-account>"
```

That closes only that gate and keeps a relayed claim on its gate record and in the
decisions; it does not approve anything natively. An actor label alone never
supplies human authority, and neither Chough nor SpecSync authenticates who decided.

Lessons learned from rehearsing SpecSync 6.1: the delta is frozen once approved and
`change check` refuses an edited delta; `change check` also refuses while `tasks.md`
has unchecked boxes, so list human gates, and running or recording Chough cases, as
plain text rather than checkboxes. A requirement the delta adds reaches the
companion only when `change check` materializes it; until then Chough cases may link
it only while the checkpoint records the approved change and its delta is
unchanged, and a box for those cases would still block that check.

## What needs reapproval

A change to approved intent, contract, acceptance criteria or affected scope needs
native reapproval. Routine implementation, tests, evidence, delta materialization
and lifecycle metadata do not.
