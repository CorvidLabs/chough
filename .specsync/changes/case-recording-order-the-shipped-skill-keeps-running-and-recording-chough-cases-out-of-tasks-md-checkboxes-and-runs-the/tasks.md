---
change: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the
artifact: tasks
---

# Tasks

Human gate before implementation (not a task): 0xLeif's native scope approval of
this definition.

Implementation:

- [x] Record the BUG-0017 reproduction on Linux (shipping sample, SpecSync 6.1.2) as an observation linked to the bug: the open case-recording box refuses both commands, and the passing order without it.
- [x] Add failing tests in tests/package.rs: unedited 825a380 copies of the three references read known-outdated and update, an edited copy stays edited, and the canonical references carry the ordering guidance; record the failing run.
- [x] Edit skills/chough-workflow/references/plan.md, testing.md and accept.md as the design shows.
- [x] Append the three 825a380 digests to KNOWN_PREVIOUS in src/skill.rs.
- [x] Refresh the .agents/ and .claude/ copies with chough skill update and confirm byte identity.
- [x] Add the CHANGELOG.md Unreleased entry and mark the BUG-0017 ordering half delivered in docs/ROADMAP.md.
- [x] Closeout inside this change: a labelled lesson in specs/cli/context.md if one was learned, one completed checkpoint entry, a summary of at most 12 lines.

Steps after the tasks (not tasks), in order: the scoped `specsync change check` that
materializes the delta; adding `TC-cli-skill-orders-the-change-check-before-new-requirement-cases`
and rerunning TC-cli-0094; `fledge lanes run verify`; commit with `.chough/`;
`specsync change check --commit`; push and observe hosted CI; 0xLeif's implementation
review; native review and finalize by one session only; merge by 0xLeif.
