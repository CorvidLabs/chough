---
change: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
artifact: testing
---

# Testing

`cargo test --test github_labels` covers REQ-cli-070.

The stage-name test reads `const ORDER` in `src/stage.rs` and compares it to the labels the workflow creates and the stages the script accepts.

The script test builds a temporary repository, puts a stand-in `gh` on `PATH`, and checks four pull requests: one introduced session at `build` gains `chough-stage:build`; a session at `implement` gains nothing and an existing `chough-stage:implement` label is removed; two sessions at different stages gain nothing; a pull request that introduces no session gains nothing.

`chough init` in a fresh fixture must not write `.github/workflows/chough-labels.yml`. A scan of `src/` must not find `gh label` or `chough-labels.yml`.

The dogfood copies must match the template bytes. `integrations/README.md` must still name every hc template, so REQ-cli-065 stays intact.
