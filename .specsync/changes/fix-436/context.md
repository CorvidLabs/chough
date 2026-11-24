# Context — fix-436

## Problem
When a gate closes with a relayed human decision, `decision_text`
(`src/checkpoint.rs`, `decision_text`, ~line 1176) renders the raw
`--decided-by` string the caller typed, e.g.
`0xLeif decided "…gate…": "…" (…); recorded by …`. But REQ-records-008
defines `name` as "the display name records show": identities are
resolved through `humans::resolve` (which maps names *and aliases* to a
configured `Human`) before anything runs, and the resolved human's
`name` field is what records are supposed to show. A decision closed
with an alias (or differently-cased text) currently renders the alias,
not the configured display name.

## Evidence
- `src/checkpoint.rs` `decision_text`: formats `relay.decided_by`
  directly.
- `src/humans.rs`: `Human.name` is documented "The display name records
  show"; `resolve` returns the configured `Human`, so the display name
  is available wherever the relay is built.
- The close path already holds the resolved `Human` when the gate
  drives a native step; `--no-native` closes resolve too
  (REQ-records-008 requires resolution before anything runs or is
  written).

## Affected requirement
REQ-records-008 (held by the `records` spec): "the display name records
show". No contract change: decisions already name the human; this only
makes the rendered name the configured one.

## Constraints consulted
- Decisions are leak-checked before writing; the configured display
  name SHALL NOT count as a user name (REQ-records-008 / REQ-records-006
  interplay, already implemented in the leak checker).
- The relay's `decided_by` field itself stays as recorded (audit trail
  of what was typed); only the rendered decision text uses the display
  name.
- docs/cli.md documents the decision format — update the wording.
## REQ-cli-050 template tension (GLM second review, 2026-11-24)
REQ-cli-050's decision template reads `<decided_by> decided "<gate>"…`,
normatively, with `<decided_by>` defined as the flag value. This change
renders the resolved display name there instead. Interpretation on
record: REQ-records-008 defines `name` as "the display name records
show" and the decision entry is a record; REQ-cli-050's acceptance
criterion that the timeline entry carries the four relay values byte
for byte still holds (the relay keeps the raw value). Two SHALLs
conflict on the alias case; the GLM review recommends settling it with
a small delta amending REQ-cli-050's template. Left flagged for the
implementation review — a second spec change needs Leif's own
authorization, not the orchestrator's.
