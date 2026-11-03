---
id: cli-cleanups-for-issue-49-hyphen-values-in-bug-add-no-repeated-native-next-action-in-the-next-main-line-iso-times-in
state: implementing
type: bug_fix
base_commit: a788869b32b8442cbc8d7be2bf19c19a922954a8
---

# CLI cleanups for issue 49: hyphen values in bug add, no repeated native next action in the next main line, ISO times in next text, help text, cli.md drift and exercise temp dir cleanup

## Intent

CLI cleanups for issue 49: hyphen values in bug add, no repeated native next action in the next main line, ISO times in next text, help text, cli.md drift and exercise temp dir cleanup

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- bug add accepts free-text values that begin with -- (for example --title "--name prints nothing") and records them verbatim
- A gate refusal names the native status command once, even when the native next action already names it; the next view's main Next line no longer repeats the native next action, so a long native next action does not cut it
- The next text view shows the last failure's time as UTC ISO 8601; the JSON keeps observed_at_unix_seconds
- Every bug add and cases run option has a help description
- docs/cli.md lists history show|adopt|prune in the Commands table and describes SpecSync 6.1.1 versus 6.1.2 accurately in the approved-delta guard intro
- examples/feature-bug-loop.sh removes the temporary directory it created after a passing full run, and keeps a passed-in directory, a prepare-cold-start directory and a failed run's directory
- No approval or actor wording changes (#44); fledge lanes run verify passes

## No-spec Rationale

Polish within REQ-cli-008, REQ-cli-011, REQ-cli-012 and REQ-cli-021 as written: no requirement or criterion text changes; JSON fields are unchanged
