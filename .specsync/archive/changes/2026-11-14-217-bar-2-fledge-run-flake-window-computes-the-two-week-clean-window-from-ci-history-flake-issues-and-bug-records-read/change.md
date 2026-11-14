---
id: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
state: archived
type: feature
base_commit: 4a7a273b3ff6d617ef7475b8a19f28ff8ac22e4c
---

# #217 bar 2: fledge run flake-window computes the two-week clean window from CI history, flake issues and bug records, read-only, as a receipt with a clean, running or blocked verdict

## Intent

#217 bar 2: fledge run flake-window computes the two-week clean window from CI history, flake issues and bug records, read-only, as a receipt with a clean, running or blocked verdict

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- fledge run flake-window prints one JSON receipt and writes nothing: clean after 14 days of green main CI on macOS and Linux with no rerun and no open flake; running, naming the latest restart and when it can first be clean, after a red or skipped main run, a rerun on any branch, a filed flake or a day without a green main run on either OS; blocked while a flake issue or DRIVE-19 bug is open, a closed flake has no green main run since, or a flake issue has no bug record; exit 2 and never clean when GitHub cannot be read in full (red then green in .github/scripts/test-flake-window.py)

## No-spec Rationale

Not applicable
