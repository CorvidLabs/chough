## MODIFIED

### REQUIREMENT REQ-cli-070

Optional GitHub stage labels. The repository SHALL ship an optional template at `integrations/github/` that a project copies in to label pull requests with the seven main Chough stages. `chough init` SHALL NOT install that template. No program under `src/` SHALL call GitHub or create those labels.

The template SHALL contain `integrations/github/README.md`, `integrations/github/chough-labels.yml` and `integrations/github/chough-label-prs.sh`. The README SHALL say the template is optional, that a project copies `chough-labels.yml` to `.github/workflows/chough-labels.yml` and `chough-label-prs.sh` to `tools/chough-label-prs.sh`, and that Chough does not call GitHub.

The workflow and the script SHALL name the main stages in the order of `const ORDER` in `src/stage.rs`: `intent`, `design`, `plan`, `build`, `test`, `accept`, `done`. The workflow SHALL create `chough-stage:<stage>` for each, with these colors, in that order: intent `FF0000`, design `FF4D00`, plan `FF9900`, build `FFEE00`, test `00CC44`, accept `00BBFF`, done `8C00FF`. It SHALL delete `chough-stage:implement` and `chough-stage:verify` when they exist. The label job SHALL NOT run for a pull request whose head repository is not this repository. The workflow SHALL create a missing `chough-stage:*` label without `--force`, and SHALL pass `--force` only when an existing label's color or description differs from this table. It SHALL NOT delete `chough-stage:implement` or `chough-stage:verify` when that label is absent. It SHALL pin `actions/checkout` to the same commit `.github/workflows/ci.yml` uses.

`.github/workflows/chough-labels.yml` and `tools/chough-label-prs.sh` SHALL be byte-identical to the template files.

The script SHALL read `stage:` from the session files the pull request introduces (`.chough/sessions/*/session.md` and `.chough/archive/sessions/*.md` that differ from the merge base with the pull request's base branch). It SHALL NOT read a session path that is a symlink. When those sessions share exactly one main stage, it SHALL apply `chough-stage:<stage>` and remove every other `chough-stage:*` label. When they share none, more than one, or a stage that is not a main stage, it SHALL remove every `chough-stage:*` label and apply none. It SHALL NOT use a session's `pr:` field to choose the pull request.

`docs/integrations.md` and `integrations/README.md` SHALL say the template is optional and that copying it is what turns the labels on. Trace: LIFE-31.

Acceptance Criteria
- The workflow's created labels, in order, are the seven names in `const ORDER`, with the colors above, and the script accepts only those names.
- Chough's installed workflow and script match the template bytes.
- A pull request that introduces one session at `build` is labeled `chough-stage:build`. One at `implement`, two sessions at different stages, and a pull request that introduces no session each end with no `chough-stage:*` label.
- `chough init` in a fresh project does not write `.github/workflows/chough-labels.yml`.
- `src/` contains no `gh label` and no `chough-labels.yml`.
- The label job does not run for a pull request whose head repository is not this repository.
- A symlinked session file is not read.
- When the seven labels already match the table, the workflow does not create them again. `--force` is used only when an existing label's color or description differs. `implement` and `verify` are deleted only when they exist.
