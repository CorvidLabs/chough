# Lesson bundle: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Keep hidden state panels hidden in the prototype template whatever display rules a project adds (#58)
- **Kind**: BugFix
- **Specs**: cli
- **Paths**: skills/chough-workflow/templates/prototype/styles.css, .agents/skills/chough-workflow/templates/prototype/styles.css, src/skill.rs, tests/package.rs, CHANGELOG.md, knowledge/
- **Acceptance**: tests/package.rs fails on main f41e7ab4 and passes after: the shipped templates/prototype/styles.css has a [hidden] rule with display: none !important, so a project's display rules on the state sections cannot show every state at once; an unedited styles.css from releases 0.1.0 to 0.4.0 reads known-outdated and skill update replaces it, while an edited copy stays edited and is kept; the canonical and .agents copies are identical. No helper behavior or requirement text changes, and fledge lanes run verify passes locally.

## Evidence

- Verification commit: `95e9aeecda2c2c8657c49b6f90330335a1e30392`
- Base commit: `f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8`
- Verified by: `specsync check --spec cli`

## From the change's context.md

# Context

The design stage starts from `skills/chough-workflow/templates/prototype/`. Its
`index.html` switches between the ready, loading, empty and error states by setting
`panel.hidden` on each `section[data-state-panel]` (the loading, empty and error
sections start `hidden`). Its `styles.css` has no `[hidden]` rule, so hiding relies on
the browser's own `display: none` for `[hidden]`. Any project rule that sets `display`
on those sections, for example `.surface section { display: grid }` or a token or base
stylesheet, wins over it and every state renders at once. Leif hit this in his Raven
session (2026-11-03, finding 6), filed as #58 and assigned by orc in the 2026-11-06
burn-down plan. FLOW-4 covers it: Leif approves the rendered prototype, so the
template's states must render one at a time.

`styles.css` has not changed since #2, so releases 0.1.0 to 0.4.0 and main f41e7ab4
all ship the same bytes (ce8f6374...). Changing it means that digest joins
`KNOWN_PREVIOUS` in `src/skill.rs` (REQ-cli-019), or every existing install would
read `edited` and never update.

Constraints: failing test first; the `KNOWN_PREVIOUS` row is appended at the end; no em
or en dash in the bundle; the canonical and `.agents` copies stay identical; no helper
behavior or requirement text changes.

## From the change's design.md

# Design

- `templates/prototype/styles.css` (canonical and `.agents` copy): add
  `[hidden] { display: none !important; }` after the `* { box-sizing }` rule, with a
  one-line comment saying why. `!important` is what keeps a later or more specific
  author rule from showing a hidden panel.
- `src/skill.rs`: append one `KNOWN_PREVIOUS` row for `templates/prototype/styles.css`
  at ce8f63741058cf79afa275e01fb7c6ab0c38dc242c35c87a8628c808bf386239 ("Releases 0.1.0
  to 0.4.0, unchanged since #2 through main f41e7ab4; superseded by the `[hidden]` guard
  (#58, FLOW-4)").
- `CHANGELOG.md`: one Unreleased bullet.
- Out of scope: `references/design.md` wording (the issue's optional QA note), other
  template changes, and #102's missing digests for other files.

## From the change's testing.md

# Testing

- tests/package.rs::the_prototype_template_keeps_hidden_state_panels_hidden: index.html
  still switches panels with `hidden`, and styles.css has a top-level `[hidden]` rule
  whose declarations include `display: none !important`. It fails on main f41e7ab4.
- tests/package.rs::the_prototype_styles_from_releases_0_1_0_to_0_4_0_are_known_outdated_and_edited_copies_stay:
  the release bytes (digest checked) read `known-outdated`, `skill update` makes every
  bundle file current, and an edited copy reads `edited` and is kept. It fails on main
  because those bytes are still the current file.
- Bound as TC-cli-prototype-hidden-panels-stay-hidden and
  TC-cli-prototype-styles-0-4-0-known-outdated (REQ-cli-019, FLOW-4). The full suite,
  `fledge lanes run verify`, pre-push and `specsync change audit --strict` pass before
  the push.

## Where these lessons go

- `specs/cli/context.md`
