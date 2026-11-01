---
change: make-checkpoint-merges-safe-across-parallel-changes
artifact: docs
---

# Docs

- `.gitattributes`: add `.chough/session.json merge=chough-session`. Keep this driver name identical in every example and test.
- `docs/cli.md`: add `checkpoint --merge <base> <ours> <theirs>` to the command table and a checkpoint-merge section. Document schema 2 only, the ordered collection algorithm and one-side removals, the indivisible current-cycle group, complete run-consent selection, existing 65536-byte and 200-item bounds, and fail-closed handling. State that success atomically replaces only `ours`, every refusal preserves all input bytes and writes no Chough record, and schema 1 still requires explicit migration for live writes. Give the explicit command as the fallback when no driver is configured.
- `docs/cli.md` and `README.md`: give the per-clone setup, not an init-time or global mutation:

  ```sh
  git config --local merge.chough-session.name "Chough schema 2 checkpoint"
  git config --local merge.chough-session.driver "chough checkpoint --merge %O %A %B"
  ```

  Explain that Git supplies base, ours and theirs as `%O`, `%A` and `%B`; a nonzero conflict needs human resolution, and `chough init` never sets either value. Keep the README command summary and link to the CLI section rather than duplicating the full algorithm.
- `docs/architecture.md`: extend Checkpoint authority with the bounded merge boundary. One-owner live writes and `.chough/session.json` as the sole canonical checkpoint remain unchanged. Git invokes the same explicit command for committed parallel edits; there is no distributed lock, second store, runtime, service or copied SpecSync state. A current-cycle conflict remains unresolved instead of manufacturing a combined objective, stage, consent or human gate.
- `docs/FRD.md`: update the REQ-cli-007 row and core checkpoint summary with safe schema 2 three-way merges and unresolved current-cycle conflicts. Keep FLOW-13 and LIFE-3 traceability and the explicit schema 1 migration statement unchanged.
- `skills/chough-workflow/references/checkpoint.md`: expand Schema and concurrency with the attributes declaration, the two per-clone config commands above, the explicit fallback and the one-owner live-write rule. Tell agents not to hand-resolve a current-cycle conflict by combining fields and not to treat a merge as approval, consent or gate closure.
- `.agents/skills/chough-workflow/references/checkpoint.md` and `.claude/skills/chough-workflow/references/checkpoint.md`: refresh both installed copies byte-identically from the canonical skill reference after its edit. Do not edit either installed copy independently.
- `src/skill.rs`: append `references/checkpoint.md` with its pre-change LF SHA-256 digest `28eccd903a89ff815e71e0cc3f81250e6c63bb986d93b0ee76b323180c4d92bf` to `KNOWN_PREVIOUS`. Retain every earlier digest so older unedited installs remain upgradable, while edited copies stay preserved.
- `tests/package.rs`: retain the pre-change reference bytes in the existing test structure and add the known-outdated, update-to-current and edited-copy-preservation regression. Verify the previous digest independently of the new constant without adding a fixture outside the approved paths.
- `specs/cli/requirements.md` and `specs/cli/cli.spec.md`: materialize only the approved REQ-cli-007 delta through the scoped SpecSync flow. Add the merge command to the observable CLI contract, the single-checkpoint invariant, compatible and conflicting behavioral examples, and fail-closed error cases. Do not manually rewrite canonical requirements or frozen archives.
- `specs/cli/testing.md`: map the merge checks to REQ-cli-007 without allocating a case number. After they run, name only the actual `checkpoint-merge-real-git-pre-fix` failure and strictly later `checkpoint-merge-real-git-post-fix` pass, their environment, retained artifacts, linked bug and limitations.
- `specs/cli/tasks.md`: after completion, summarize the implementation, real Git evidence, documentation and CI-matching checks with links to both observations and the local bug. Keep human review and native closeout separate from completed tasks.

No roadmap, release, deployment, adapter or unrelated ID-scheme documentation is introduced. Existing schema 1 migration docs, historical exercises and archived records keep their meaning and bytes.
