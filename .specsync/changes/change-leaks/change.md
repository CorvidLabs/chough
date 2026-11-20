---
id: change-leaks
state: draft
type: bug_fix
base_commit: 7bf110dcca70bdd3565007086aee40a286f40eff
---

# Change leaks

## Intent

change leaks

## Affected Canonical Specs

- `cli`
- `records`

## Acceptance Criteria

- A description, a --rationale and an answer holding a leak are refused by chough change new and chough change answer before SpecSync runs, naming the argument and the kind, with nothing under .specsync/ changed; with --redact the change's records hold <redacted:<kind>> and one line says how many were replaced; a control character is refused under --redact too (#412). chough doctor --leaks also scans the open changes under .specsync/changes/ and reports a leak there with the file, line and kind, which --fix --redact skips naming the way out; .specsync/archive/ is not scanned. change ship --close-gate, change new's description, change answer's answer and an explicit evidence ID are named by what they hold in any error or output (rest of #367).

## No-spec Rationale

Not applicable
