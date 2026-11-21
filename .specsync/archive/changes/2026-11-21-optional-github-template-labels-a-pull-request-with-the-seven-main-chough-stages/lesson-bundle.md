# Lesson bundle: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Optional GitHub template labels a pull request with the seven main Chough stages
- **Kind**: Feature
- **Specs**: cli
- **Paths**: integrations/github, integrations/README.md, docs/integrations.md, tests/github_labels.rs, .github/workflows/chough-labels.yml, tools/chough-label-prs.sh, hi/life.md, changelog.d/2026-11-21-optional-github-stage-labels.md, .chough/cases/cli.optional-github-stage-labels.cases.toml, INTENT.md, specs/cli/requirements.md, specs/cli/history/optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages.md
- **Acceptance**: A project that copies the template gets seven chough-stage labels, one per main stage, in rainbow order. A pull request whose introduced sessions share one main stage gets that label. Any other stage, or more than one stage, gets no stage label. chough init does not install the workflow, and src/ does not call GitHub.

## Evidence

- Verification commit: `9f265d9fbb3a914863f1c70fbad6a82ff86a516b`
- Base commit: `b5627125d1df640ae03882588e48b27f5f84f0c6`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Jackdaw labeled pull requests from Chough session files with a workflow and a script that lived only in that repository. The label list was handwritten, so it grew `implement` and `verify`, which are SpecSync change statuses, not Chough stages. The seven main stages are `intent`, `design`, `plan`, `build`, `test`, `accept`, `done`, in that order, from `const ORDER` in `src/stage.rs`.

The user asked, on 2026-11-21, to add this to Chough after agreeing the shape: an optional template a project copies in, `chough init` unchanged, and the `chough` binary does not call GitHub. Labels stay a view of `stage:` in the session files the pull request introduces. SpecSync still decides whether a stage move is allowed.

Ruled out: installing the workflow from `chough init`, and putting GitHub calls in `src/`. Release and the side stages (`bugs`, `validate`, `librarian`) stay off the progress strip.

## From the change's design.md

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

## From the change's testing.md

# Testing

`cargo test --test github_labels` covers REQ-cli-070.

The stage-name test reads `const ORDER` in `src/stage.rs` and compares it to the labels the workflow creates and the stages the script accepts.

The script test builds a temporary repository, puts a stand-in `gh` on `PATH`, and checks four pull requests: one introduced session at `build` gains `chough-stage:build`; a session at `implement` gains nothing and an existing `chough-stage:implement` label is removed; two sessions at different stages gain nothing; a pull request that introduces no session gains nothing.

`chough init` in a fresh fixture must not write `.github/workflows/chough-labels.yml`. A scan of `src/` must not find `gh label` or `chough-labels.yml`.

The dogfood copies must match the template bytes. `integrations/README.md` must still name every hc template, so REQ-cli-065 stays intact.

## Where these lessons go

- `specs/cli/cli.3md`
