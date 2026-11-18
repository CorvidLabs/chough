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

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-021 | the test above and the case in `.chough/cases/cli.next-offers-no-native-change.cases.toml` |
