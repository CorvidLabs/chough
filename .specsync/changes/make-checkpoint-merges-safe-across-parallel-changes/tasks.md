---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: tasks
---

# Tasks

- [ ] Add the focused checkpoint merge and real Git regressions. Run the compatible-change Git merge at the unmodified tip, retain failed observation `checkpoint-merge-real-git-pre-fix` and its log, then search, create and link a local bug before any `src/` implementation edit. Use no new numeric case.
- [ ] Add bounded `chough checkpoint --merge <base> <ours> <theirs>` for schema 2 only. Validate every input and the complete result before writing; on success atomically replace only `ours`, and on every refusal leave all three inputs byte-identical and create no Chough record.
- [ ] Merge exact-string history and reference collections by ordered three-way membership: surviving base entries in base order, then ours-only additions in ours order, then theirs-only additions in theirs order. Deduplicate identical additions and preserve one-side removals. Cover criteria, requirements, cases, design, files, decisions, open questions, completed work, observations and bugs.
- [ ] Treat objective, intent scope, modules, stage, native change, next action, run consents, blockers and awaiting-human gates as one current-cycle group. Select the whole group when exactly one side changed it or both changed it identically; otherwise refuse with a current-cycle conflict reason. Do not merge run consents item by item or emit gate, transition or consent side effects.
- [ ] Fail closed for schema 1 in any position, mixed or unsupported versions, malformed JSON, unknown fields at any nesting level, oversized inputs and any merged result beyond existing record or collection bounds. Preserve schema 1 reading and explicit migration, one-owner live writes and native SpecSync authority.
- [ ] Declare `.chough/session.json merge=chough-session` in `.gitattributes`. Document the per-clone `merge.chough-session.driver` command and the explicit merge fallback, without making `chough init` read or mutate Git configuration.
- [ ] Apply the CLI, architecture, FRD and checkpoint-skill documentation updates. Refresh the `.agents/` and `.claude/` checkpoint references, retain the previous canonical reference digest in `src/skill.rs::KNOWN_PREVIOUS`, and cover known-outdated updates without replacing edited copies.
- [ ] Run the focused and full checks with CI-matching tools. Rerun the unchanged real Git regression, retain passed observation `checkpoint-merge-real-git-post-fix`, link it to the same bug and verify the bug only from the strictly later pass. Add only observed outcomes to the testing companion.

Human gates (not tasks): native scope approval, human implementation review, native finalization and archive, and the merge decision remain separate human decisions. No checked task, structural pass or agent label approves, finalizes, merges, releases or deploys this change.
