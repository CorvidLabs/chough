---
change: keep-evidence-current-across-commits-and-specsync-lifecycle-bookkeeping-and-verify-bugs-only-by-a-pass-observed-after
artifact: tasks
---

# Tasks

- [ ] Add case definitions for BUG-0014, BUG-0015 and BUG-0016, and record failing reproductions at the unfixed tip.
- [ ] Add the content digest, the bookkeeping key helper, the legacy fallback and record validation in src/evidence.rs.
- [ ] Add tests for a commit of only `.chough/`, an empty commit, an already-present edit, a committed deletion, a source edit, an untracked file, a delta, requirements and artifact edit, a bookkeeping write, a `change.md` state change, an archive move, and legacy records.
- [ ] Order bug verification and the verified rule by observation time in src/bugs.rs, with tests.
- [ ] Normalize the canonical REQ-cli-004 heading to `### REQ-cli-004`.
- [ ] Rewrite the accept order in accept.md and lifecycle.md, add the bugs.md line, copy all three to both host directories, and add the KNOWN_PREVIOUS digests.
- [ ] Update docs/cli.md, docs/architecture.md, docs/walkthrough.md, docs/FRD.md, docs/lifecycle.md, docs/NEXT_CHANGE.md, specs/cli/cli.spec.md invariant 4, specs/cli/testing.md, and the specs/cli/context.md lesson.

Human gates and post-verification steps (not tasks): scope approval of this change. After approval: run the scoped check to materialize REQ-cli-004; rerun the cases (passing) and verify BUG-0014 to BUG-0016; run the disposable end-to-end exercise of the documented order; record the human gate; commit the source with `.chough/`; run `specsync change check --commit`; run `fledge lanes run pre-push`; push and hosted CI on Ubuntu, macOS and Windows; the human implementation review; native review and ship. File the upstream SpecSync issue for an opt-in verification exclusion when the human agrees.
