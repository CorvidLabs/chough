---
change: fix-351
artifact: testing
---

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
