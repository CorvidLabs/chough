# Lesson bundle: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Chough change approve relays a human's approval of a change definition to SpecSync with a required citation (REQ-cli-071, DRIVE-20)
- **Kind**: Feature
- **Specs**: cli
- **Paths**: src/change.rs, src/main.rs, tests/change.rs, docs/cli.md, README.md, specs/cli/requirements.md, changelog.d/2026-11-25-approve-passthrough.toml, .chough/cases/cli.approve-passthrough.cases.toml, .gitattributes, changelog.d/2026-11-25-approve-passthrough.md
- **Acceptance**: chough change approve refuses before SpecSync runs without --actor, --words or --where, or with one of them blank. With all three, it runs specsync change approve <id> --actor <actor> --note <note>, where the note carries relayed=pi chat, decided-by, words, where and recorded-by joined by '; ', and --note appends its value; without --recorded-by the note names chough <version>. SpecSync's standard output, standard error and exit code pass through unchanged.

## Evidence

- Verification commit: `84d0871f96a824d0c04c09744066bca77cfe5887`
- Base commit: `61479feb1e88a57bc45716ff4e42ce84fb2b463b`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A 2026-11-25 standing rule says an agent records a human's approval of a SpecSync
change through Chough, with who decided, the exact words, a citation of where they
were said and the recording agent; a relay without a citation is exactly what the
rule exists to prevent, and a record whose citation does not resolve is void. Gate
closings already relay approvals and reviews through Chough (`checkpoint
--close-gate`, `gate record`, `change ship`), but a change definition approval
relayed from a chat had no Chough path: the agent either called `specsync change
approve` directly or left the approval unrecorded.

The user approved building the passthrough (quest q-14-1, on council counsel from
Astra and Fable) before any direct approve call. It extends REQ-cli-057's pattern:
`chough change` runs a SpecSync step and passes its output, refusals and exit code
through unchanged.

Ruled out: reusing the gate relay, which records a decision at a gate rather than a
change definition approval; letting agents keep calling `specsync change approve`
directly.

## From the change's design.md

# Design

No product screen changes, so there is no HTML prototype. The visible result is
the command line: `chough change approve` prints nothing of its own on success
but SpecSync's output, and its refusals are plain messages that name the missing
relay field and the command to run again.

## From the change's testing.md

# Testing

`cargo test --test change` covers REQ-cli-071 in `tests/change.rs`, on the
stand-in `specsync` that records its arguments and prints chosen bytes:

- The exact arguments: `specsync change approve <id> --actor <actor> --note
  <note>`, run in `--root`, with the note's five fields in order, joined by
  `; `, and `--note` appended as `note=<extra>`.
- The default: without `--recorded-by`, the note names `chough <version>`,
  the version the binary reports.
- The refusals: no `--actor`, no `--words`, no `--where`, a blank `--words`,
  and a present but blank `--recorded-by` each exit non-zero, name the flag and
  the command to run again, and run no `specsync`.
- The passthrough: SpecSync's stdout and stderr, including bytes no text
  handling keeps as they are, and its exit code, pass through unchanged.
- The leak checks: an id holding a credential is refused as
  `refused:unshowable-argument`, an actor or note holding a credential or a
  control character is refused before `specsync` runs, with the value never
  shown, with or without `--redact`.
- The records: no tracked file under `.chough/` changes, whatever SpecSync's
  exit code.

The same cases run as `TC-cli-approve-passthrough*` through
`.chough/cases/cli.approve-passthrough.cases.toml` with `chough cases run`.

## Where these lessons go

- `specs/cli/cli.3md`
