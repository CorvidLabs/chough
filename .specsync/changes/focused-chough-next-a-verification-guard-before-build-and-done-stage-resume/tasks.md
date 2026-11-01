---
change: focused-chough-next-a-verification-guard-before-build-and-done-stage-resume
artifact: tasks
---

# Tasks

- [x] Make one native status read serve the gate in `status` and `next`, with a counting-stub test.
- [x] Build `chough next` as the compact view (text default, `--json`), bounded at 16384 bytes with list totals, string cuts and the drop loop; keep `status` fields unchanged and say in its help that it always prints JSON.
- [x] Move tests that read full-report fields through `next --json` to `status`, and add the cap fixture test.
- [x] Add the done-stage new-cycle action (archived and authenticated, no native change, ungated) with `--reason` in every suggested reset.
- [x] Make `chough stage reset` replace the checkpoint's next_action; leave `back` unchanged; add tests.
- [x] Add the bounded native record reader (4 MiB) in src/records.rs.
- [x] Add the approved-delta guard to the build, test and accept gate with both refusal labels and the archived-approval digest vectors.
- [x] Update SKILL.md (host paragraph) and references/checkpoint.md (done-stage resume), add their origin/main digests to KNOWN_PREVIOUS, and reinstall the .agents/ and .claude/ copies.
- [x] Update docs/cli.md, docs/architecture.md, docs/integrations.md, docs/exercises/cold-start.md and README.md.
- [x] Switch examples/feature-bug-loop.sh and examples/visual-design.sh to `chough next --json`.
- [x] Measure `chough next` (text and JSON) and `chough status` sizes and times on this repository for the pull request.

Human gates (not tasks): scope approval of this change, including the open questions
in plan.md; the human implementation review; native review and ship.

Post-verification steps (not tasks): after the scoped `change check` materializes
REQ-cli-021 and REQ-cli-022, add TC-cli-0120 to TC-cli-0124 and record their runs;
note on BUG-0002 that Chough now guards it, leaving it open; push and CI on Ubuntu,
macOS and Windows; then re-run `change check`.
