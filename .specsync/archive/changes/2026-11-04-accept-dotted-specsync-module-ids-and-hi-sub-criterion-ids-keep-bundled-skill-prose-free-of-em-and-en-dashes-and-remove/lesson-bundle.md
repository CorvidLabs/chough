# Lesson bundle: accept-dotted-specsync-module-ids-and-hi-sub-criterion-ids-keep-bundled-skill-prose-free-of-em-and-en-dashes-and-remove

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Accept dotted SpecSync module IDs and HI sub-criterion IDs, keep bundled skill prose free of em and en dashes, and remove a mistaken completed entry (DRIVE-1, DRIVE-7, DRIVE-8; issues 56 and 57)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/records.rs, src/checkpoint.rs, src/main.rs, src/bugs.rs, src/evidence.rs, src/cases.rs, tests/ids.rs, tests/prose.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: chough init, checkpoint, next, status, context and timing accept and keep SpecSync module IDs with dots such as chat.service, room.service, linked-wallet.service and identity-wipe.service, cases files and case IDs for such a module are read and checked, and requirement links such as REQ-chat.service-1 are kept; checkpoint --criterion and bug add --criterion accept every criterion ID HI accepts, including sub-criterion IDs such as UX-12.a, CHAT-34.a and SEND-1.a.1.b, with no length cap of its own; plain criterion IDs Chough accepted before, such as UX-012 and ux-1, stay accepted so existing checkpoints and bugs stay readable (review 5761422487); module IDs SpecSync refuses, such as a/b, .., CON and trailing., dotted criterion IDs HI refuses, such as UX-1.A and UX-1..a, and requirement or case IDs with no letter or digit are refused with nothing saved; a test fails if any bundled skill file contains U+2013 or U+2014; checkpoint --remove-completed removes one exact completed entry and refuses an entry that is not there with nothing saved; the new tests fail on main 8298e5a where they cover changed behavior; docs/cli.md states the accepted forms and the flag; fledge lanes run verify passes

## Evidence

- Verification commit: `f97323b77dfa8cc67b9a064f2db9c7df08e07774`
- Base commit: `d673efc9fadd00953cf9e42dad96e08f3c76103c`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Leif's 2026-11-03 5:03 PM MST message (t811u) lists what stopped Chough from driving Raven's work that day. Item 1: Raven's checkpoint names SpecSync modules `chat.service`, `room.service` and `linked-wallet.service`, which Chough refused ("invalid session module name"), so `chough next` refused the checkpoint and focused context dropped Raven's core services; `chough checkpoint --criterion UX-12.a` failed with "invalid criterion ID" although hi accepts `CHAT-34.a` and `BOTS-4.g`. Issues 56 and 57 report the same. Item 7 asks for no em dashes in bundled skill templates with a check (Raven BUG-0001, seen with 0.1.0) and a way to correct a `checkpoint --completed` entry without editing session.json by hand.

Chough used one `valid_name` rule (ASCII letters, digits, `-`, `_`) for every ID. The bundled templates on main 8298e5a already contain no U+2013 or U+2014, so the dash part only adds the check that keeps them out.

Intent: hi/drive.md DRIVE-1, DRIVE-7 and DRIVE-8, quoting t811u. Authority: Leif's 7:49 AM MST standing Chough maintainer instruction and his t811u request; corvid-agent is the acting account.

## From the change's design.md

# Design

- `records::valid_module_name` ports SpecSync 6.1.2 `validate_module_name`: one portable path segment, no separator, Windows-invalid or control character, no `.`/`..`, no edge space, no trailing dot, no reserved device basename, at most 247 bytes. Used for checkpoint and init modules, case file stems and the owning-module hint.
- `records::valid_criterion_id` ports hi 0.8.0 `Id::parse`: family `A-Z` then `A-Z0-9_`, then dot-separated levels alternating number and lowercase letters, starting with a number, no leading zero, 32-bit numbers. Used for checkpoint criteria and bug criteria.
- `records::valid_reference_id` is `valid_name` plus single inner dots, for requirement and case IDs such as `REQ-chat.service-1` and `TC-chat.service-sends-hybrid`. Observation, bug, run and consent IDs keep `valid_name`.
- `checkpoint --remove-completed <text>` (repeatable) removes one exact entry before additions are applied, so remove plus `--completed` corrects a typo in one call; an entry that is not present refuses the whole call and nothing is saved.
- `tests/prose.rs` walks `skills/chough-workflow` and fails on U+2013 or U+2014.

## From the change's testing.md

# Testing

- `tests/ids.rs`: dotted modules through init, checkpoint, context, next, status and timing; a checkpoint holding `chat.service`, `room.service`, `linked-wallet.service`, `UX-12.a`, `CHAT-34.a` and `BOTS-4.g` read by every view; bug links to `UX-12.a` and `REQ-chat.service-1`; a `chat.service.cases.toml` listed and checked; IDs neither tool accepts still refused with nothing saved.
- `tests/cli.rs` or `tests/ids.rs`: `--remove-completed` removes an exact entry, corrects with `--completed` in one call, refuses a missing entry with the session byte-identical.
- `tests/prose.rs`: no U+2013 or U+2014 in any bundled skill file.
- Failing first: the ID and remove tests fail with `src/` at main 8298e5a; the prose test passes there because the dashes were already removed.
- Proof on Raven at 74e43005: `chough next`, `status`, `timing` read Raven's real checkpoint instead of refusing it.
- `fledge lanes run verify` passes.

## Where these lessons go

- `specs/cli/context.md`
