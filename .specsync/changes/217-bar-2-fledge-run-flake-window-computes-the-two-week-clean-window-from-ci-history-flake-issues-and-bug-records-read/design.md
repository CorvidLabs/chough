---
change: 217-bar-2-fledge-run-flake-window-computes-the-two-week-clean-window-from-ci-history-flake-issues-and-bug-records-read
artifact: design
---

# Design

`.github/scripts/flake-window.py` uses only Python's standard library, like
`check-specsync-archive.py`, and works in three parts:
- **fetch:** `gh api` reads the CI workflow's runs one UTC day at a time over a
  45-day lookback, because a run can be rerun for up to 30 days. It also reads the
  jobs of every main run created within the last 15 days, plus main runs after each
  closed flake until a green one is found, and every issue labelled `flake`, with
  pull requests left out.
- **read:** the front matter and text of every `.chough/bugs/*.md` record.
- **compute:** a pure function that turns those inputs and `now` into the receipt.

`--input <file>` replaces the fetch with saved input, and `--save-input <file>`
keeps the fetched input, so a receipt can be recomputed. The receipt carries the
input's SHA-256.

A main run is one on branch main whose event is push, schedule or workflow_dispatch.
A run is green for an OS when its `Product` job for that OS succeeded. A main run
whose conclusion is not success, or that has a job that did not succeed, is a red
main run. The DRIVE-19 restart and block read every bug record that names DRIVE-19
in its front matter or text. A record counts as closed when its status is verified,
duplicate, wont-fix or fixed-upstream (`src/bugs.rs` CLOSED). Exit codes: 0 with a
receipt, 1 under `--require-clean` unless the verdict is clean, 2 when GitHub's data
is unavailable or incomplete.
