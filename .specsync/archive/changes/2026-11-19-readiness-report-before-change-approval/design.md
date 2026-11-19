---
change: readiness-report-before-change-approval
artifact: design
---

# Design

Nothing visual changes. The command prints a text report to standard output and
a short refusal to standard error only when the change cannot be read. There is
no prompt, no colour contract and no second screen.

One finding is one paragraph or one bullet. It names what is missing, the
module or path, and one exact command or write that fixes it. The real change
id is filled in. The report ends with a count of findings, or with a line that
nothing is missing. Exit 0 means the count is zero.

`new`, `answer`, `check`, `status` and `ship` keep the output they have today.
`ready` is not one of those passthroughs.
