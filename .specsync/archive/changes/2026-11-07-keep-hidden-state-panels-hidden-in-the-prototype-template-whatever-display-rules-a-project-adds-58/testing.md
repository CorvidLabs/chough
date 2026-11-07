---
change: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
artifact: testing
---

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
