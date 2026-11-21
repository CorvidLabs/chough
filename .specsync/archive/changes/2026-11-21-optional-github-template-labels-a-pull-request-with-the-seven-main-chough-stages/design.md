---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: design
---

# Design

No product screen changes, so there is no HTML prototype. The visible result is GitHub's own label chips.

Seven labels, left to right. GitHub lists labels alphabetically, so the color and the description carry the order.

| Order | Label | Color | Description |
|---|---|---|---|
| 1 | `chough-stage:intent` | `#FF0000` | Chough lifecycle label, step 1 of 7 |
| 2 | `chough-stage:design` | `#FF4D00` | Chough lifecycle label, step 2 of 7 |
| 3 | `chough-stage:plan` | `#FF9900` | Chough lifecycle label, step 3 of 7 |
| 4 | `chough-stage:build` | `#FFEE00` | Chough lifecycle label, step 4 of 7 |
| 5 | `chough-stage:test` | `#00CC44` | Chough lifecycle label, step 5 of 7 |
| 6 | `chough-stage:accept` | `#00BBFF` | Chough lifecycle label, step 6 of 7 |
| 7 | `chough-stage:done` | `#8C00FF` | Chough lifecycle label, step 7 of 7 |

A project turns this on by copying two files to two fixed paths: `integrations/github/chough-labels.yml` to `.github/workflows/chough-labels.yml`, and `integrations/github/chough-label-prs.sh` to `tools/chough-label-prs.sh`. Chough's own repository keeps those copies so the template is the one this repository runs. The workflow's `push` trigger names `main`. A project whose default branch has another name changes that one line.

The script asks `gh` for the pull request's base branch, diffs the session files introduced since the merge base, and applies one label only when those files share one main stage. `implement` and `verify` are deleted if an older copy created them, and they are never applied.
