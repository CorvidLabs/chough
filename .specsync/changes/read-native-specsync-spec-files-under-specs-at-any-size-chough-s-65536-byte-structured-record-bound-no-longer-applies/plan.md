---
change: read-native-specsync-spec-files-under-specs-at-any-size-chough-s-65536-byte-structured-record-bound-no-longer-applies
artifact: plan
---

# Plan

## Approach

Add `records::read_spec_file`, an unbounded reader for native spec files, and use it
for the two `requirements.md` reads in `src/evidence.rs`. `chough next`'s
`context.md` search and `chough library snapshot`'s spec-file refusal stay as
they are, named in REQ-cli-034 as known exceptions (review 5757495438), and a later
native change on a follow-up PR lifts them, because src/main.rs and src/library.rs
are outside this change's paths. The unstarted dependent change for `next` was
removed from this pull request (review 5757495438).
`chough context` keeps its display budgets for spec text. Every Chough record keeps
`read_record` and its bounds. Add advisory composability guidance to the workflow
skill's build reference and register main's previous build reference digest so
installed copies update.

## Out of scope

- Changing RECORD_LIMIT, SESSION_LIMIT, case, output, context or history bounds.
- Any size check, warning gate or automatic split for specs or source files.
- SpecSync, existing archives, HI criteria (a new criterion needs Leif's
  confirmation) and #55's behavior.

## Steps

1. Failing test first: a requirements companion over 4 MiB is refused today.
2. `read_spec_file` in src/records.rs; use it in `owning_module` and
   `validate_requirements`.
3. Guidance in skills/chough-workflow/references/build.md and the installed copy;
   KNOWN_PREVIOUS entry for main's build.md, with a test.
4. REQ-cli-034 delta, docs/cli.md note, CHANGELOG, a case for each new test, and a
   lesson in specs/cli/context.md.
5. fmt, lint, full tests, the cases, `fledge lanes run verify`, then
   `specsync change check --commit`.

## Risks

- An unbounded read of a very large spec costs memory; specs are repository files
  the project chose to keep, and Leif accepted any size.
- A bounded record could be routed through the new reader by mistake; the existing
  65,536-byte record test and the checkpoint tests still run.

## Constraints consulted

AGENTS.md; REQ-cli-032 (unchanged); Leif's 10:05 words; the parent's 10:06
instruction.
