---
id: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs
state: implementing
type: feature
base_commit: 38fe24c063e3afde49a5be5093cd69791bd06126
---

# #217 bar 2: CI runs once a day on main so the flake window sees macOS and Linux every day, and the SpecSync Archive jobs take the run's own commit as base when the event has none

## Intent

#217 bar 2: CI runs once a day on main so the flake window sees macOS and Linux every day, and the SpecSync Archive jobs take the run's own commit as base when the event has none

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- ci.yml runs on a daily schedule as well as on pushes to main and pull requests; the SpecSync Archive base falls back to github.sha, so a scheduled run gives check-specsync-archive.py a real base; tests/package.rs fails naming the missing schedule or fallback (red then green)

## No-spec Rationale

Not applicable
