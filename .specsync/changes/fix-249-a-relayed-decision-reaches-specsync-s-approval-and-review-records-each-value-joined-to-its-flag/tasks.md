---
change: fix-249-a-relayed-decision-reaches-specsync-s-approval-and-review-records-each-value-joined-to-its-flag
artifact: tasks
---

# Tasks

- [x] `Step::arguments` appends the relay as `--words`, `--where`, `--recorded-by`, each value joined to its flag; `checkpoint` and `change ship` pass it through.
- [x] Tests: approve close, review close and ship's review carry the relay (#261); hyphen-leading relay through ship and through `checkpoint --close-gate` (corvid-agent's review); exact-argument asserts in `tests/change.rs`, `tests/humans.rs`, `tests/relay.rs`.
- [x] `docs/cli.md` and CHANGELOG; cases in `.chough/cases/cli.ship-relay.cases.toml`.

Human gates (not checkboxes): scope approval, implementation review, merge.
