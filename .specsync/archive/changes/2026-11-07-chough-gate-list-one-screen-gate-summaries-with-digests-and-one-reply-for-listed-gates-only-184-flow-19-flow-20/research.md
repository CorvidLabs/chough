---
change: chough-gate-list-one-screen-gate-summaries-with-digests-and-one-reply-for-listed-gates-only-184-flow-19-flow-20
artifact: research
---

# Research

- **`RelayedBy` is `#[serde(deny_unknown_fields)]`** (`src/stage.rs`). New optional fields therefore make a history with them unreadable to earlier builds. That is the same as #183's new kind, and is why the change log must name the oldest reader.
- **No native read.** A change's `state.json` holds `title`, `state` and `base_commit` (seen in every archived change), so the summary needs no `specsync change status`.
- **The verification window.** As #179 established, nothing may write `.chough/` between `change check --commit` and finalize. So a review gate on a verifying change closes through `chough change ship`, not through `gate record`.
- **The untracked-file pattern.** `.chough/local/` is ignored through the `.gitignore` the checkpoint writer creates. The direct-write baseline in #183 is written only once that exists, and the gate list follows the same rule.
