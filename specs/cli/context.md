# CLI context

## Decisions

2026-10-31: The human selected Chough, a private CorvidLabs repository, Rust helper
plus portable skills, and current-agent-first v1. Rune coordination is deferred.
HI captures intent; SpecSync owns module contracts and lifecycle; Fledge owns task
execution. The helper records observations and focused session context only.

Bootstrap implementation and initial private push are explicitly authorized. Native
SDD is optional by upstream policy and remains disabled for this baseline. Structural
SpecSync dogfooding is strict. No scope approval, human implementation review,
archive, merge, release or deployment is implied by the bootstrap.

## Working files

- src/main.rs: private binary implementation.
- tests/: isolated integration fixtures.
- skills/chough-workflow/: canonical portable skill bundle embedded in the binary.
- .chough/session.json: focused session, edited explicitly as plain JSON.
- .chough/evidence/: immutable-by-command observation records, not authenticated proof.

## Lessons

2026-10-31, Chough CI setup: the Fledge README's `@v1` Action ref was absent, and
the v1.7.2 release tag existed without action.yml. Source: initial CI job
https://github.com/CorvidLabs/chough/actions/runs/41218532137/job/124588047159
and exact-ref repository reads. Scope: this project's Fledge setup Action. Future
use: keep the reviewed wrapper commit pin separate from its binary version pin;
check the actual Action file and binary assets when upgrading. Revise/retire this
lesson when a supported immutable Action release replaces the bootstrap pin.

Add further lessons only from observed work, with source, applicable scope,
concrete future use and revision/retirement guidance. A future native cli.3md would
become the lesson source in place of this section.
