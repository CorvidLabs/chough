# Lesson bundle: fix-281

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 281
- **Kind**: BugFix
- **Specs**: cli, records
- **Paths**: src/stage.rs, src/checkpoint.rs, src/session_md.rs, tests/stages.rs, docs/cli.md, specs/cli/requirements.md, specs/records/records.spec.md, .chough/cases/cli.no-native-change-required.cases.toml, changelog.d/2026-11-18-fix-281.md
- **Acceptance**: With SpecSync configured and no native change recorded, chough checkpoint --no-native-change <reason> records the reason in the session; its gated stage moves are then allowed and labelled ungated:no-native-change-required, so it reaches done and is archived with outcome done, the story keeping the reason; an open human gate still refuses the move; a blank reason is refused; the flag is refused while a native change is recorded, and recording one removes the reason.

## Evidence

- Verification commit: `40e9fe53c527b96718013c2e5be5e1e07edfe04c`
- Base commit: `66efea4dd777077ef9bbd412cf0fac8f7ea93b41`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

GitHub #281, from CorvidLabs/jackdaw#150, a planning PR. Its Chough session
recorded hi criteria, docs and issues and needed no SpecSync change. Once the
work was finished there was no honest way to archive it. With SpecSync
configured, every gated stage move refuses as `refused:no-native-change`, and
`chough session close` is for abandoned work: it withdraws open human gates and
archives with outcome `closed-without-done`. So an agent had to open a SpecSync
change it did not need, or record finished work as abandoned.

Leif's decision, in his comment on #281 on 2026-11-18 from his planning
interview: "done without a change. A session can record that it needs no native
change, with a reason, and then may advance to done and archive with outcome
`done`. Its human gates still have to be closed by a person, as in any other
session."

The record is a new checkpoint flag, `--no-native-change <reason>`, kept as
`no_native_change` in the session's front matter. No command is added. It is
never held together with a native change. A gate that drives a native approval
or review still refuses in such a session, since there is nothing to approve.

## From the change's design.md

# Design

Nothing visual changes. One new flag on an existing command:

```sh
chough checkpoint --no-native-change "Planning only: hi criteria, docs and issues"
```

The session's front matter gains `no_native_change: <reason>`. A stage move's
report and its Timeline entry give the gate as `ungated:no-native-change-required`.

## From the change's testing.md

# Testing

`tests/stages.rs::a_session_that_needs_no_native_change_reaches_done_and_archives_as_done`,
with SpecSync configured and no native change recorded:

- a stage advance refuses as `refused:no-native-change`;
- a blank reason is refused;
- after `chough checkpoint --no-native-change "<reason>"` the session holds the
  reason, in its JSON and in its front matter;
- an open human gate still refuses the advance as `refused:awaiting-human`;
- with the gate withdrawn, four advances, each labelled
  `ungated:no-native-change-required`, reach done; the session is archived with
  `outcome: done` and its story keeps the reason;
- recording a native change removes the reason, and the flag is then refused,
  naming the change.

- a reason holding a credential or an email address is refused as `refused:leak`
  with the session unchanged, and `--redact` stores it redacted (added after the
  tester's finding on #348; it fails without the leak guard).

It fails on main, which has no such flag.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-007 | the test above and the case in `.chough/cases/cli.no-native-change-required.cases.toml` |

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
