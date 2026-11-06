---
change: name-stage-advance-to-in-the-workflow-skill-s-transition-table-drive-9-req-cli-052-follow-up
artifact: context
---

# Context

#147 (REQ-cli-052, DRIVE-9) added `chough stage advance --to <stage>`, but the chough-workflow skill's transition table in `references/stages.md` does not name it. Agents learn their stage commands from that table, so they keep running `stage advance` once per step. The orc assigned this follow-up after #147 merged (t18 relay, 2026-11-05 late evening MST). It is a separate small change and PR. kimi-perf32 (#32) is not touching the skill. The pgrep flake is #144.

`stages.md` has been byte-identical since #22, so every installed copy from 0.1.0 to 0.4.0 has this one digest, 658ce798826c4d685753af0735e5fc6a53685f1205b9b9902d625c63c71d277f at main 01f1d023. It must join `KNOWN_PREVIOUS` in `src/skill.rs`. Otherwise `chough skill status` would call those copies `edited` rather than `known-outdated`, and `skill update` would leave them stale.
