---
change: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
artifact: testing
---

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
