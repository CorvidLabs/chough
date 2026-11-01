---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: tasks
---

# Tasks

- [ ] Add case definitions for BUG-0014, BUG-0015 and BUG-0016, record failing reproductions at the unfixed tip, and link each to its bug.
- [ ] Add the content digest, the bookkeeping key helper (including `lesson-bundle.md` and the front-matter-only `state:` rule), the legacy fallback and record validation in src/evidence.rs.
- [ ] Add tests for a commit of only `.chough/`, an empty commit, an already-present edit, a committed deletion, a source edit, an untracked file, a unix-only exec-bit change, a delta, requirements and artifact edit, each bookkeeping write, a `change.md` front-matter state change, a `state:` line in the `change.md` body, an archive move, and legacy and malformed records.
- [ ] Order bug verification and the verified rule by observation time in src/bugs.rs, with tests that write evidence records with chosen `observed_at_unix_seconds` values.
- [ ] Normalize the canonical REQ-cli-004 heading to `### REQ-cli-004`, exactly as requirements.md shows, before the scoped check.
- [ ] Rewrite the accept order in accept.md and lifecycle.md (including its "Evidence records" paragraph), add the bugs.md line, copy all three to both host directories, and add the KNOWN_PREVIOUS digests for all three.
- [ ] Update docs/cli.md, docs/architecture.md, docs/walkthrough.md, docs/FRD.md (REQ-cli-004 section and excluded list), docs/lifecycle.md, docs/NEXT_CHANGE.md, README.md, the dated note in docs/exercises/cold-start.md, specs/cli/cli.spec.md invariant 4, specs/cli/testing.md, and the specs/cli/context.md lesson.
- [ ] Add the decision that supersedes D-016 to docs/DECISIONS.md, worded as the human approved it, and mark D-016 superseded.

Human gates and post-verification steps (not tasks): scope approval of this change, including the answers to the open questions in plan.md. After approval: run the scoped check to materialize REQ-cli-004, REQ-cli-012 and REQ-cli-017; rerun the cases (passing) and verify BUG-0014 to BUG-0016; run the disposable end-to-end exercise of the documented order; record the human gate; commit the source with `.chough/`; run `specsync change check --commit`; run `fledge lanes run pre-push`; push and hosted CI on Ubuntu, macOS and Windows; the human implementation review; native review and ship. File the upstream SpecSync issue for an opt-in verification exclusion when the human agrees.
