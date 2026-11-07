---
change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
artifact: design
---

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
