---
change: readiness-report-before-change-approval
artifact: tasks
---

# Tasks

- [x] Add `ChangeCommand::Ready` and `src/ready.rs`, and the tests in
  `tests/ready.rs` that fail without it.
- [x] Add `.chough/cases/cli.change-ready.cases.toml` for REQ-cli-067.
- [x] Document `chough change ready` in `docs/cli.md` and the README helper
  table, and add the Unreleased changelog bullet.
- [x] Name the command on the fast path and in the plan reference, in both skill
  copies, and refresh the skill-release manifest digest and `MANIFEST_SHA256`.
- [x] `fledge run fmt`, `fledge run test`, `fledge run lint` and
  `fledge lanes run verify` pass, and `chough change ready` on this change
  exits 0.

Human gates, as plain text. Scope approval of
readiness-report-before-change-approval was recorded and ran SpecSync's approve.
Implementation review comes later, after the checks, and drives the review.
The review has not been decided. The boxes above are the build. Each one is marked
done before `chough change check`, which refuses an open box. Running and
recording the cases is part of that build, not a box left open for a person.
