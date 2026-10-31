# Acceptance and native closeout

Acceptance is where the human decides whether the implementation is what they
wanted. Agent review, a passing command, scope approval or an actor label never
stands in for that decision.

## Prepare the review

1. Run the project lanes and the native scoped check when native status allows it:
   `specsync change check <id> --commit` (stage new source files first).
2. Push the branch, open or update the pull request, and observe hosted CI at the
   real tip. Report product tests, structural checks, native lifecycle state,
   provenance and platforms separately.
3. Present to the human: what changed, the current verification, the relevant case
   observations with freshness, open bugs and limits, and what was not checked.
4. Record the gate: `chough checkpoint --awaiting-human "Implementation review of <id>"`.

If native archive checks are red because review has not happened, say exactly that.
Do not claim all checks are green and do not bypass the gate.

## After the human decides

Only after a real review decision, record it and close natively, following native
status rather than a remembered sequence:

```sh
specsync change review <id> --reviewer "<the human>"
specsync change ship <id>        # or: specsync change finalize <id>
```

Do not commit between review and finalize. Push the archive commit on the same pull
request, wait for required checks, then merge through the repository's normal
process. If several changes share a pull request, review and finalize them one at a
time. If evidence went stale, use the recovery action that `specsync change status`
names.

Then `chough stage advance` moves accept to done; it is refused until native state is
`archived`. Finishing never enters release.

## Compounding

Lessons can be recorded at any stage. Put them in the module's native lesson source
(`<module>.3md` later plane when present, otherwise `context.md`) with the source,
the scope where it applies and the future action it changes. A lesson that changes
behavior or policy may need its own native change. Archival does not mean no bug
remains; open bugs stay in the backlog.
