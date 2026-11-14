# Lesson bundle: 217-bar-2-ci-runs-once-a-day-on-main-so-the-flake-window-sees-macos-and-linux-every-day-and-the-specsync-archive-jobs

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: #217 bar 2: CI runs once a day on main so the flake window sees macOS and Linux every day, and the SpecSync Archive jobs take the run's own commit as base when the event has none
- **Kind**: Feature
- **Specs**: cli
- **Paths**: .github/workflows/ci.yml, tests/package.rs, docs/ci.md, CHANGELOG.md
- **Acceptance**: ci.yml runs on a daily schedule as well as on pushes to main and pull requests; the SpecSync Archive base falls back to github.sha, so a scheduled run gives check-specsync-archive.py a real base; tests/package.rs fails naming the missing schedule or fallback (red then green)

## Evidence

- Verification commit: `c63620259d73895ff467b842fe09323d2cb663ae`
- Base commit: `38fe24c063e3afde49a5be5093cd69791bd06126`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

On 2026-11-13 at 03:30Z Leif answered ch10-flake's (claude-code, hc t38r) design
question Q3, "Run CI once a day on main so a quiet week still counts?", with "Yes,
macOS + Linux (Recommended)". The answer was relayed by t370.

The flake window (REQ-cli-062, PR #252) restarts after any full UTC day with no green
main run on both platforms. Its first live run found two such days in the last 14,
2026-10-30 and 2026-11-09, when nothing merged.

A scheduled event has no pull request base and no `before` commit. Today the SpecSync
Archive step passes `github.event.pull_request.base.sha || github.event.before`, and
check-specsync-archive.py refuses an empty base ("A real trusted base revision is
required"). Every scheduled run would therefore be red and restart the window. The
run's own commit is a real base: the inspector then walks that one revision and still
authenticates every archive.

## From the change's testing.md

# Testing

`tests/package.rs` reads `.github/workflows/ci.yml`. It checks for one daily
`cron` under `schedule:` beside the `push` and `pull_request` triggers, and for
`|| github.sha` ending the SpecSync Archive base. It is red at the test commit and
green at the fix. The first scheduled run after the merge is observed on GitHub and
recorded.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-064 | `tests/package.rs`: ci_runs_once_a_day_on_main_with_a_real_archive_base |

## Where these lessons go

- `specs/cli/cli.3md`
