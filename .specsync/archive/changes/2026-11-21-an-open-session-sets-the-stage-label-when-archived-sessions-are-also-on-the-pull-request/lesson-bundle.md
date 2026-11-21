# Lesson bundle: an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: An open session sets the stage label when archived sessions are also on the pull request
- **Kind**: Feature
- **Specs**: cli
- **Paths**: integrations/github/chough-label-prs.sh, tools/chough-label-prs.sh, integrations/github/README.md, tests/github_labels.rs, .chough/cases/cli.open-session-sets-the-stage-label.cases.toml, changelog.d/2026-11-21-open-session-sets-the-stage-label.md, specs/cli/requirements.md, specs/cli/history/an-open-session-sets-the-stage-label-when-archived-sessions-are-also-on-the-pull-request.md, hi/life.md, INTENT.md
- **Acceptance**: A pull request that introduces an archived session at done and an open session at intent is labeled chough-stage:intent, and the done label is removed. A pull request whose only introduced session is an archived story at done is labeled chough-stage:done. Two open sessions at different stages still get no stage label. The two script copies stay byte-identical.

## Evidence

- Verification commit: `95605d888bdc48c4e3255ef8e23b3913a12c93ff`
- Base commit: `6bc60913b028761b20832a6906dc8e31d17e70df`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

A pull request that archives a finished session and starts a new one currently contributes both stages. The label script treats that as several stages and applies none. The user asked that the new change's stage be the one shown.

LIFE-32 is the criterion. REQ-cli-070 changes so an open session wins, and an archived story is used only when no open session remains. The keep-label change stays open at its own review and is not edited. Archived records are not edited. This change is not merged from here.

## Lessons

- 2026-11-21, verified: counting an archived session and a new open session as one set clears the stage label, so the script chooses the open session first. Source: the first run of `the_script_follows_an_open_session_ahead_of_an_archived_one`, which reported multiple distinct stages `done intent`. Limits: the optional GitHub label script. Two open sessions at different stages still apply no label.

## From the change's design.md

# Design

Nothing visual changes. There is no HTML mockup. The label is a GitHub label name taken from the session `stage:` field.

When a pull request introduces an open session, that session's stage is the label and archived stories on the same pull request are ignored. When the pull request introduces no open session, the archived stories supply the stage. One distinct main stage among the chosen sessions is applied. Several stages among them, or a stage outside the main line, clears every `chough-stage:*` label.

## From the change's testing.md

# Testing

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-070 | `tests/github_labels.rs`: `the_script_follows_an_open_session_ahead_of_an_archived_one` |

The test uses the same stand-in `gh` as the other script tests. One run introduces an archived story at `done` and an open session at `intent`, with `chough-stage:done` already applied, and expects the label to become `intent`. Another introduces only the archived story and expects `done`.

`TC-cli-open-session-sets-the-stage-label` binds that test. The earlier label cases stay in their own files.

## Where these lessons go

- `specs/cli/cli.3md`
