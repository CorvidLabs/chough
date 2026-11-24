# Plan — fix-436

## Approach
Where the close path builds the `RelayedBy` (after `humans::resolve`
has produced the configured `Human`), carry the resolved display name
alongside the raw `decided_by` — either as a new field on `RelayedBy`
used only for rendering, or by passing the display name into
`decision_text` as a parameter. `decision_text` renders the display
name; every other consumer of `relay.decided_by` (native step args,
records) keeps the raw value. Choose the smallest diff; no public
format change beyond the rendered name.

## Out of scope
- Changing what `--decided-by` accepts or how `humans::resolve` works.
- Changing the relay's stored `decided_by` value.
- Native step arguments (already use `specsync_actor`).

## Steps
1. Thread the resolved display name into `decision_text` (parameter or
   `RelayedBy` field — smallest diff).
2. Render it in place of the raw text in the decision sentence.
3. Tests: alias renders display name; exact-name renders unchanged;
   existing decision tests updated where they pin the old text.
4. docs/cli.md decision-format wording.

## Risks
- Existing tests pin decision text — update them deliberately; the
  pinned sentences change only where they used a non-display name.
- The leak checker: display names are already exempt
  (REQ-records-008); no new interaction.

## Constraints consulted
- REQ-records-008 wording; FLOW-16 (relay remains unauthenticated
  claim — unchanged); audit trail untouched.