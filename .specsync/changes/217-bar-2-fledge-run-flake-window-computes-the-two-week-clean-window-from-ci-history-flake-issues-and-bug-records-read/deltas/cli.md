## ADDED

### REQUIREMENT REQ-cli-062

Compute the 1.0 flake window. The repository SHALL provide `fledge run flake-window`, which runs `.github/scripts/flake-window.py`: a read-only check that reads the CI workflow's run history and the issues labelled `flake` from GitHub and the bug records in `.chough/bugs/`, writes nothing, and prints one JSON receipt with a verdict of `clean`, `running` or `blocked`. Each of these SHALL restart the window at its time:
- a completed CI run on main in which any job did not succeed (failed, cancelled, timed out or skipped);
- a rerun, meaning a run attempt after the first, of any CI run on any branch;
- a filed flake: a `flake` issue opened, or a bug record created that names DRIVE-19;
- each of the last 14 full UTC days with no CI run on main whose `Product / ubuntu-latest` and `Product / macos-latest` jobs both succeeded, at the end of that day.

The verdict SHALL be `blocked` while a `flake` issue is open, while a bug record that names DRIVE-19 is not closed, while a closed `flake` issue has had no fully green CI run on main since it closed, or while a `flake` issue has no bug record that names it as `GitHub #<n>`. Otherwise it SHALL be `clean` when no restart falls within the last 14 days, and `running` when one does, naming the latest restart and when the window can first be clean. GitHub data that cannot be read in full SHALL make the check exit 2 without a verdict. `--require-clean` SHALL exit 1 for any verdict other than `clean`. The check records no human decision (FLOW-10). Trace: LIFE-21, DRIVE-19.

Acceptance Criteria
- Fourteen full days of green main runs on macOS and Linux, with no rerun and no open flake, give `clean`.
- A red main run, a main run with a skipped job, a rerun on a pull request branch, a newly filed `flake` issue, a new bug record naming DRIVE-19, and a day without a green main run on one OS each give `running`, naming that restart and when the window can first be clean.
- An open `flake` issue, an open bug record naming DRIVE-19, a closed `flake` issue with no green main run since it closed, and a `flake` issue that no bug record names each give `blocked`.
- A failed or incomplete GitHub read exits 2 with no verdict; `--require-clean` exits 1 on `running` and `blocked`; the check leaves the project tree unchanged.
- `fledge run flake-window` and `fledge run ci-flake-window-fixtures` exist, and the `CI gate fixtures` job runs the fixtures.
