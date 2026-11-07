---
change: return-the-observation-id-from-cases-record-and-evidence-add-issue-60
artifact: design
---

# Design

## Approach

In `record_manual`, add the existing `id` as `case` and the allocated or explicit
`observation` as `observation` to the JSON value it already returns. In
`evidence::add`, add the stored record's existing `id` as `observation` to its
JSON output. Do not add a shared envelope type or alter any writer, ID allocator,
record schema, exit path or unrelated command.

Add two integration regressions in `tests/cases.rs`: one for manual case
recording and one for direct evidence addition. Each parses the returned JSON,
checks the additive keys, and proves the observation ID equals both the
`recorded` path's file stem and the stored record's `id`. The manual case also
checks `case` and preserves assertions for the existing keys and outcome.

Bind each regression in `.chough/cases/cli.observation-output.cases.toml` with
criteria FLOW-7 and LIFE-4 and the existing requirement links. Update the CLI
reference and add one Unreleased changelog bullet.

## Out of scope

Bug add/update/show output normalization, structured stage refusals, `next`
listing unreferenced observations, stored-record changes, ID allocation changes,
and any release or deployment step.

## Size and risk

Small: two additive response fields at two existing return sites plus focused
regressions and documentation. The main uncertainty is reusing the existing
fixtures without weakening their current assertions. Recorded history has 53 bug
fix times with a 10m47s median and a 45s to 37h30m25s range, and too few human
gate samples; these elapsed wall-clock figures are context, not an estimate or
deadline for this change. A consumer that rejects unknown JSON keys could notice
the additive fields; that is the explicitly requested public-contract correction,
not a reason to change existing keys.

Only k-ch-docs owns this lane and its listed files. No parallel implementation
or file handoff is planned.
