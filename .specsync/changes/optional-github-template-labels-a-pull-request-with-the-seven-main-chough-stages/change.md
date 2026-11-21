---
id: optional-github-template-labels-a-pull-request-with-the-seven-main-chough-stages
state: implementing
type: feature
base_commit: b5627125d1df640ae03882588e48b27f5f84f0c6
---

# Optional GitHub template labels a pull request with the seven main Chough stages

## Intent

Optional GitHub template labels a pull request with the seven main Chough stages

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- A project that copies the template gets seven chough-stage labels, one per main stage, in rainbow order. A pull request whose introduced sessions share one main stage gets that label. Any other stage, or more than one stage, gets no stage label. chough init does not install the workflow, and src/ does not call GitHub.

## No-spec Rationale

Not applicable
