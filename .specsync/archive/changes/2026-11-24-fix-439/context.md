# Context — fix-439

## Problem
`chough next` and `chough status` render a note that "SpecSync keeps no
reason" when the native check has not passed, telling the user to run
`specsync change check` to see why. But since fix-437's era the refusal
reason *is* recorded: `.specsync/changes/<id>/verification-attempts.json`
writes a `"passed": false` attempt with a `"refusal"` field (and docs/
cli.md documents the field). The CLI just never reads it back.

## Evidence
- `src/native_check.rs` ~line 62: the `(Some("approved" | "implementing"), _)`
  arm emits the "SpecSync keeps no reason when it refuses before
  verifying" note without consulting the attempts file.
- `docs/cli.md`: documents that `verification-attempts.json` records
  `"refusal"` for pre-verification refusals, while `next`/`status` say
  "not shown yet" in the same table row.

## Affected requirement
REQ-cli-040 (native readiness surfaced in `next`/`status`) — held by
the `cli` spec. No contract change: the note and the recorded reason
describe the same refusal; this change only makes the CLI prefer the
recorded reason when one exists.

## Constraints consulted
- `chough change check` refuses while tasks.md has unchecked boxes
  (vault lesson, 2026-10-31) — testing.md keeps plain text, no boxes.
- The attempts file is the same one `change check` appends to; reading
  it must never write it.
- docs test pins `.gitattributes` merge-driver lines exactly — do not
  touch `.gitattributes`.
## Re-scope correction (2026-11-24)
The change was created with `--no-spec-change`, but the GLM second
review correctly found the rendered-note change alters what REQ-cli-039
SHALL says. Leif authorized adding the delta ("Yes", pi chat t14z).
`specsync change send-back` put the change back to draft; the interview
was re-answered with `public_contract yes`; the flag correction in
`state.json` records what the interview would have said (no requirement
text was hand-edited — the amendment lives in `deltas/cli.md`).
