# Lesson bundle: fix-320

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 320
- **Kind**: BugFix
- **Specs**: cli, history
- **Paths**: src/checkpoint.rs, src/doctor.rs, tests/leaks.rs, docs/cli.md, .chough/cases/cli.leaking-id-recovery.cases.toml, changelog.d/2026-11-18-fix-320.md
- **Acceptance**: A save refused because a criterion, requirement, case, observation, bug or native change reference holds a leak names the recovery, chough session shelve <session id>, beside removing the value. A committed session record whose case reference holds a token gives a leak finding naming chough session shelve <id>, and chough doctor --fix --redact leaves that ID unchanged with the finding skipped, so the session still reads, while it still redacts a leak on an ordinary line of the same record.

## Evidence

- Verification commit: `1d47c4ff4c49da677507d55633178cdcb2ec6d42`
- Base commit: `cef86e2622a2a908856b0c2cf11b29d3c1d3e472`
- Verified by: `specsync check --spec cli --spec history`

## From the change's context.md

# Context

GitHub #320, a follow-up from the tester's check of #315. Since #315, a session that an older Chough let hold a token in an ID reference (criterion, requirement, case, observation, bug or native change) refuses every save, even with `--redact`, saying "remove it". No command removes such a reference, and no message names the way out: `chough session shelve <id>`, or a hand edit. On main too, `chough doctor --fix --redact` rewrites such an ID to `x-<redacted:token>`, which leaves the session unreadable (macOS swarm s6-F2/F8).

Leif's decision (2026-11-18, planning interview, recorded on #320 and relayed by hc t4c as the scope approval): approved as a fix. The refusal names the recovery (`chough session shelve <id>`, or a command that removes the reference), and `doctor --fix --redact` never rewrites an ID.

The refusal's new text fits REQ-records-006's refusal shape, so that part needs no wording. The doctor part changes REQ-history-012's `leak` fix, which said every match is redacted in place, and REQ-cli-055's `--fix`, since the doctor code is `cli`'s. Not in scope, and left on #320: `session new --module` still commits a leaking module name.

## From the change's design.md

# Design

- `checkpoint::check_id` takes the session's ID. Its refusal adds the recovery: remove the value from the command, or, when the session already holds it, `chough session shelve <id>` and continue in `chough session new <key>`.
- `doctor`: `id_lines(text)` returns the front matter lines whose key gives an ID: `id`, `aliases`, `native_change`, `criteria`, `requirements`, `cases`, `observations`, `bugs`, `evidence`, `related`, `duplicate_of`, `case`, a flow list on the key's own line, or a block list under it.
  - A `leak` finding on such a line names the recovery as its fix and is blocked.
  - The `--fix --redact` fix redacts the body as a whole and each other front matter line, leaves every ID line byte for byte, and refuses (so the finding is `skipped`) when its own line is an ID line.

## From the change's testing.md

# Testing

`tests/leaks.rs`:
- `a_leaking_id_reference_names_the_recovery`: a session whose committed `cases` reference holds a token refuses `checkpoint --decision`, naming `chough session shelve <id>`. Fails on main.
- `doctor_redact_never_rewrites_an_id`: the same committed session, with an email in a body line, gives two `leak` findings, and the ID's names `chough session shelve <id>`. `doctor --fix --redact` redacts the email, leaves the `cases` line byte for byte with its finding `skipped`, and the session still reads (`chough status` succeeds). Fails on main.

Case `TC-cli-doctor-redact-never-rewrites-an-id` binds the second.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/history/context.md`
