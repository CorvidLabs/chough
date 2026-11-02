# TC-cli-0132 reviewer note

Reviewer: Claude (the authoring agent that orchestrated this change), not an
independent reviewer. Commit read: 8b5b4cd (after merging main 66bef80). Environment: repository-text.

| Expected | Where | Result |
|---|---|---|
| Three certainty labels, source and limits, decision outranks lesson and never changes it | references/accept.md, Compounding | present |
| decided:, rejected:, failed: with a because reason, best-effort, what next shows | references/checkpoint.md | present |
| Same-PR fix still gets a bug record or a lesson entry | references/bugs.md, after-fix paragraph | present |
| Closeout inside the open change before change check --commit, at most 12 summary lines, no spec edits after archive, works with the librarian off, brief only when on | references/librarian.md, Closeout | present |
| SpecSync 6.1.2 ships verification_ignored_paths, not adopted here, and why | references/lifecycle.md and accept.md | present; matches the SpecSync 6.1.2 CHANGELOG ("at most 8 project-relative directory prefixes") |
| Both host copies current | chough skill status | openai and claude copies all current |

Limitation: this checks the wording is present and consistent, not that agents follow
it; TC-cli-0133 is the behavioral check.
