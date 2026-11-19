# Lesson bundle: fix-351

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Fix 351
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: src/stage.rs, src/skill.rs, src/checkpoint.rs, src/main.rs, tests/stages.rs, specs/cli/requirements.md, specs/cli/cli.spec.md, skills/chough-workflow/, .agents/skills/chough-workflow/, tests/fixtures/skill-releases/manifest.json, tests/skill_releases.rs, .chough/cases/cli.next-offers-no-native-change.cases.toml, changelog.d/2026-11-18-fix-351.md
- **Acceptance**: With SpecSync configured and no native change recorded, a refused stage move and chough next, as JSON and as text, name both chough checkpoint --native-change <id> and chough checkpoint --no-native-change "<reason>"; next's command stays specsync change new; the workflow skill's lifecycle reference explains the flag in the canonical and the installed copy.

## Evidence

- Verification commit: `a09610ef11d27f072acf2aa1da7b5182a4018f53`
- Base commit: `235c3bfb063e835f67c4aae12f873ceac4074be7`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

GitHub #351, a follow-up to #348 (#281). #348 added `chough checkpoint
--no-native-change "<reason>"` for work that needs no SpecSync change. Nothing
told an agent the flag exists: the workflow skill did not mention it, and when a
move was refused as `refused:no-native-change`, the refusal and `chough next`
named only `chough checkpoint --native-change <id>` and `specsync change new`.

Leif's decision, in #351 on 2026-11-18: "yes, both": the skill explains the flag,
and when a gated move is refused for lacking a native change, `chough next`
offers both paths.

The host templates under `integrations/` do not cover stages or the native
change, so they are not changed.

## From the change's design.md

# Design

Nothing visual changes. The refusal and `chough next` read:

```text
Gate not open (refused:no-native-change): Reaching build needs the native change: record it with `chough checkpoint --native-change <id>`, or, when this work needs none (planning, docs or issues, say), record why with `chough checkpoint --no-native-change "<reason>"`.
```

## From the change's testing.md

# Testing

`tests/stages.rs::a_move_refused_for_lacking_a_change_offers_both_paths`, with
SpecSync configured and no native change recorded:

- the refused `stage advance` names both `chough checkpoint --native-change <id>`
  and `chough checkpoint --no-native-change "<reason>"`;
- `chough next`, as JSON and as text, names the flag in its action, and its
  command stays `specsync change new …`;
- `references/lifecycle.md` holds the flag in the canonical skill and in the
  installed `.agents` copy.

It fails on main, where the refusal names only `--native-change`. The skill
release manifest digest and its pinned hash are refreshed, and the skill, package,
docs and next tests pass.

Added after the tester's findings on #359:

- `skill::released_digests::every_released_file_the_bundle_replaces_is_known_previous`:
  every skill file as 0.6.1 shipped it is still current or a known previous digest, so
  a 0.6.1 install updates. It fails without the 0.6.1 `lifecycle.md` digest.
- the stages test also refuses the literal `<reason>` placeholder and expects
  `chough next` to say `Native change: none needed: <reason>`, with
  `no_native_change` in its JSON.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-021 | the test above and the case in `.chough/cases/cli.next-offers-no-native-change.cases.toml` |

## Where these lessons go

- `specs/cli/cli.3md`
