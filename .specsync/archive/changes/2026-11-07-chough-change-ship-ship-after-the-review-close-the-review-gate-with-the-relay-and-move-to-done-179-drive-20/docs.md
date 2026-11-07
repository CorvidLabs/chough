---
change: chough-change-ship-ship-after-the-review-close-the-review-gate-with-the-relay-and-move-to-done-179-drive-20
artifact: docs
---

# Docs

- `docs/walkthrough.md`:
  - `specsync change new` becomes `chough change new`;
  - `specsync change check … --commit` becomes `chough change check …`;
  - the closeout runs `specsync change review` and then `chough change ship <id> --close-gate "Implementation review of <id>" …`.
- `skills/chough-workflow/references/accept.md`, `lifecycle.md` and `stages.md` (both copies): the closeout names `chough change ship` with the relay; review stays `specsync change review`.
- `docs/lifecycle.md`: the closeout order names `chough change ship`.
- `docs/cli.md`: the `change` row and the "SpecSync change steps" section add `ship`.
- `README.md`: the helper-table row adds `ship`.
- `CHANGELOG.md`: one Unreleased entry citing #179 and DRIVE-20.
