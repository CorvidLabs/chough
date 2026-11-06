# Lesson bundle: one-librarian-default-in-the-requirements-req-cli-013-defers-to-req-cli-015-on-124-d8

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: One librarian default in the requirements: REQ-cli-013 defers to REQ-cli-015, on (124, D8)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: tests/optional.rs, CHANGELOG.md
- **Acceptance**: REQ-cli-013 no longer gives the librarian an off default and defers to REQ-cli-015 (on); no requirement sentence names the librarian, a default and off without saying it is on by default; chough pref show still reports librarian on with origin default; the runtime is unchanged. The new test fails first and fledge lanes run verify passes on Linux.

## Evidence

- Verification commit: `e89be5d75f6e31d49592687cbb8e2980de0bca07`
- Base commit: `8e0ab4392279cbbb1a4db08044305c450ae641b7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

Issue #124, audit finding D8. The audit was of main `700dad96` and published 0.3.0.

Two current requirements disagree on the librarian's default:
- REQ-cli-013 says preferences default to "ask, ask, on-request, capture-only and off", which makes the librarian off.
- REQ-cli-015 says "The librarian SHALL be on by default".

`chough pref show` reports `on`, with origin `default`, in a fresh directory. That matches REQ-cli-015 and the walkthrough, and `standing_preferences_need_a_source_and_stay_separate_from_consent` asserts it. No passing test can satisfy both requirements.

## Leif's plan

The runtime default stays on (95% plan, item C2, 2026-11-05). This is a contract repair. It is not authorization to change behavior.

## From the change's testing.md

# Testing

- tests/optional.rs `the_requirements_give_the_librarian_one_default_and_it_is_on`: any sentence in specs/cli/requirements.md that names the librarian, a default and `off` must say it is on by default. It fails first, on REQ-cli-013's sentence ending "...capture-only and off respectively".
- `standing_preferences_need_a_source_and_stay_separate_from_consent` still asserts that the runtime reports `librarian: on`, with origin `default`.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-013 | tests/optional.rs `the_requirements_give_the_librarian_one_default_and_it_is_on`, `standing_preferences_need_a_source_and_stay_separate_from_consent` |

## Lanes

`fledge lanes run verify` passes on Linux.

## Where these lessons go

- `specs/cli/context.md`
