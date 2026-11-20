---
change: specsync-6-4-0-change-ready-asks-the-approval-dry-run-and-spec-history-is-recorded-in-fragments
artifact: research
---

# Research

Measured with SpecSync built from tag v6.4.0-rc.1 (commit 3d57fa13; it reports
`specsync 6.4.0`), in throwaway copies of this repository and a fresh project.

- `specsync --json change approve <id> --dry-run` prints one document, `id`, `dry_run`,
  `would_approve` and `checks`: thirteen checks in approval order, each `passed`,
  `refused` with a `message`, or `skipped` with `skipped_because`, and a `fix` on a
  refused `definition` or `ownership` check whose message names the `specsync change
  answer` command. It exits 0 when no check refuses and 1 when any does, and writes no
  file.
- The same draft gives the same document with and without `--actor`, so ready passes
  none.
- A draft missing its `history` delta and holding an extra `bogus` one gets one `deltas`
  refusal naming both: "semantic delta modules must exactly match affected specs
  (missing: history; extra: bogus)". Chough's own delta-set rule names the same two.
- Each refused check gives its first problem only: with unanswered questions and four
  TODO artifacts, `definition` names one question, where Chough's interview and artifact
  rules name every question and artifact.
- In a fresh project that never ran adoption, `change new` and the dry run leave
  untracked `.specsync/change.lock` and `.specsync/workflow-v2-baseline.json`.
- Every 6.4.0 command run here prints a note that `.specsync/sdd.json`'s `specs/` entry in
  `ignored_paths` is inert and should be removed.
- spec-sync #1089 (rebasing a stale MODIFIED delta) is an open pull request, not in the
  candidate.
