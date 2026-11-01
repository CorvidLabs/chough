# The development loop

Chough's skill guides this loop; HI, SpecSync, Fledge, the repository host and the
real environment keep their responsibilities. Current tool help and `specsync change
status` take precedence over any cached sequence.

## Stages

![Stages, gates, backtracking and side stages](diagrams/stages.svg)

```mermaid
flowchart TB
    intent["intent: HI criteria, FRD, one intent decision"] --> design["design: only for visual changes"]
    intent -. "skip design with a reason" .-> plan
    design --> plan["plan: native change, one scope approval"]
    plan -->|"gate: native definition approved"| build["build: approved tasks"]
    build -->|"gate: native approval valid or archived"| test["test: focused cases, three tracks"]
    test -->|"gate: no open blocking bug; native approval valid or archived"| accept["accept: evidence for human review"]
    accept -->|"gate: native change archived, terminal evidence authenticates"| done["done"]
    done -. "only on request or standing setting" .-> release["release: optional, real commands"]
    release -->|"same archive gate"| done
    test -. "back: blocking bug" .-> build
    plan -. "back" .-> intent
    any(["any stage"]) -. "enter, then return to caller" .-> side["side stages: bugs, validate, librarian"]
    any -. "every transition" .-> checkpoint[("checkpoint and transition log")]
```

Every stage shares one contract (purpose, inputs, outputs and owners, roles,
completion evidence, approval policy, questions, checkpoint content and transitions),
documented in [references/stages.md](../skills/chough-workflow/references/stages.md).
An active stage can iterate or pause, and a paused one can resume; advance, skip,
back, iterate, pause, enter and return all need an active stage. Reset is the one move
that also works from a paused stage or a side stage, and it leaves the stage active.
Every main stage before done, and release, can be skipped with a reason; skip moves
forward and meets the same gates as advance. Any main stage after intent, and release,
can go back to an earlier main stage. Back, reset and return also meet the native gate
of the stage they land on. A side stage returns to its caller before it can advance,
skip or go back.
History is append-only; going back lists what to refresh and, after native
approval, points to native reapproval for changes to intent, contract, acceptance
criteria or scope.

Human decisions sit at natural points, not at every step: the intent as a whole,
the rendered design with its companion, one native scope approval, and the
implementation review at acceptance. Agents synthesize review findings before
asking, and keep doing independent work while a gate is pending.

## Testing in three tracks

Functional checks always run for the affected criteria and impacted behavior.
Copy and localization checks run when visible text or locale behavior changes;
design QA runs when visuals or interactions change. `chough cases select` returns
the focused set and names every omission. Definitions and observations are separate
records, and a failing case stays failed even when others pass. See
[the testing reference](../skills/chough-workflow/references/testing.md).

## Acceptance and native closeout

![Acceptance before native closeout](diagrams/lifecycle.svg)

```mermaid
flowchart TB
    tests["Focused cases pass; blocking bugs closed"] --> gate["Record the awaiting-human gate; commit source with .chough/"]
    gate --> check["Native scoped check, committed"]
    check --> push["Push branch; hosted CI at the real tip"]
    push --> present["Present evidence, open bugs and limits"]
    present --> human{"Human implementation review"}
    human -->|"changes requested"| build["Back to build"]
    build --> tests
    human -->|"approved"| review["specsync change review"]
    review -->|"no commit in between"| ship["specsync change ship or finalize (same PR)"]
    ship --> archive["Push archive commit; CI"]
    archive --> merge["Merge through the normal process"]
    merge -. "only if authorized" .-> release["Optional release with real commands"]
```

The order before the review is: when the delta adds or modifies requirements, run
`specsync change check <id>` first to materialize them; `chough cases run`; record the
gate with `chough checkpoint --awaiting-human`; commit the source together with
`.chough/`; `specsync change check <id> --commit`; push and observe hosted CI; present
the work. After the human decision come `specsync change review`, then `ship` or
`finalize`, and only then clearing the gate and `chough stage advance`. Between the
check and finalize, nothing writes to `.chough/` and nothing is committed.

This order is an interim measure. Chough's observations stay current across
commits of its own records and SpecSync's bookkeeping, because freshness compares
content and leaves those records out. SpecSync's verification digest, however, covers
every non-ignored file, including `.chough/`, and has no exclusion setting; until
SpecSync offers an opt-in verification exclusion (the proposed upstream
`verification_ignored_paths`), any Chough write in that window forces another
`change check`.

`specsync change check <id> --commit` materializes the approved delta and records
targeted structural verification; it does not run product tests and refuses while
tasks are unchecked. The human implementation review happens after verification and
before archive; the native `review` record follows a real decision only. Then `ship`
or `finalize` archives on the same pull request without an intervening commit.
Every move that lands on done needs native state `archived` with authenticated
terminal evidence: `chough stage advance` or `stage skip` from accept, `stage advance`
or `stage skip` from release, `stage reset --to done`, and `stage return` to a caller
that was done. Build needs a valid definition approval, and test and accept need that
approval or such an archive. Unknown or unavailable native state, an invalid approval
and an invalid SDD policy refuse these moves, and so does SpecSync configured with SDD
enabled but no native change recorded in the checkpoint (`refused:no-native-change`;
record it with `chough checkpoint --native-change <id>`). No SpecSync configuration or
a valid disabled policy leaves them ungated.

## Release, monitoring and learning

Finishing never enters release. Release, publication, installation, deployment and
live health are separate observations, entered only on request or under a standing
project setting. Optional repeated validation and monitoring follow run consent or
standing preferences and keep findings locally. Lessons can be recorded at any stage
in the native module source (`<module>.3md` lesson plane or `context.md`) with their
source, scope and future use.

## Adoption in this repository

The bootstrap used native structural checks with SDD disabled and claimed no
approval. Chough then adopted the workflow-v2 lifecycle (commit c582a10). The first
fully native change, `evolve-chough-into-a-resumable-stage-workflow`, was approved by
the human before implementation, reviewed by the human at acceptance and archived on
2026-10-31; later changes are archived the same way in `.specsync/archive/changes/`.
