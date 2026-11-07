---
id: keep-hidden-state-panels-hidden-in-the-prototype-template-whatever-display-rules-a-project-adds-58
state: implementing
type: bug_fix
base_commit: f41e7ab4c6380698783c1e98a4c6cf2fa96c1ff8
---

# Keep hidden state panels hidden in the prototype template whatever display rules a project adds (#58)

## Intent

Keep hidden state panels hidden in the prototype template whatever display rules a project adds (#58)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- tests/package.rs fails on main f41e7ab4 and passes after: the shipped templates/prototype/styles.css has a [hidden] rule with display: none !important, so a project's display rules on the state sections cannot show every state at once; an unedited styles.css from releases 0.1.0 to 0.4.0 reads known-outdated and skill update replaces it, while an edited copy stays edited and is kept; the canonical and .agents copies are identical. No helper behavior or requirement text changes, and fledge lanes run verify passes locally.

## No-spec Rationale

No requirement text changes: the shipped prototype template's styles.css gains a [hidden] guard so its state switcher shows one state panel, as FLOW-4's rendered review needs, and the earlier styles.css digest joins KNOWN_PREVIOUS under REQ-cli-019's existing rule that copies from earlier builds are recognized and updated. No helper behavior changes.
