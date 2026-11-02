---
change: case-recording-order-the-shipped-skill-keeps-running-and-recording-chough-cases-out-of-tasks-md-checkboxes-and-runs-the
artifact: design
---

# Design

Skill text only, plus the earlier digests that let unedited host copies update. No
helper behavior, output or record format changes.

## references/plan.md

The lessons paragraph after "One approval" becomes (proposed text):

```markdown
Lessons learned from rehearsing SpecSync 6.1: the delta is frozen once approved and
`change check` refuses an edited delta; `change check` also refuses while `tasks.md`
has unchecked boxes, so list human gates, and running or recording Chough cases, as
plain text rather than checkboxes. A requirement the delta adds reaches the
companion only when `change check` materializes it, and `chough cases check` refuses
a case linking it until then, so a box for those cases would block the very check
that unblocks them.
```

## references/testing.md

After the `chough cases check` paragraph in "Definitions are not observations", one
new paragraph (proposed text):

```markdown
A requirement an approved native change adds is not in the companion until the
scoped `specsync change check <id>` materializes it, so `cases check` refuses a case
that links it before then. Finish the implementation tasks, run that check, then add,
run and record those cases. Running or recording cases is never a `tasks.md`
checkbox: `change check` refuses while any box is unchecked.
```

## references/accept.md

Step 1 of "Prepare the review" keeps its text and gains one sentence (proposed):

```markdown
1. When the delta adds or modifies requirements, run `specsync change check <id>`
   first, so the requirement text that cases link is materialized. It refuses while
   `tasks.md` has an unchecked box, so the cases in step 2 are never a box there.
```

## Installed copies and earlier digests

`chough skill update` refreshes `.agents/` and `.claude/`. KNOWN_PREVIOUS in
`src/skill.rs` gains the main 825a380 digests of the three references:

- references/plan.md `1022db57d0c114effc8669ede9ebb453f932109f608f883712725e57ba53551d`
- references/testing.md `426c2df68e8d25bfbdee4bf81b4deaa06eac4525b6dfad3c771311e0d6a4ac99`
- references/accept.md `80f71366ad81e1720bdc5f91d51c5428cd6a60729c9d1675977e284f7b08e398`

The entries are appended as one commented block, separate from PR #37's added
checkpoint.md entry, so the two merge in either order. `tests/package.rs` keeps the
825a380 bytes inline as consts, as earlier changes did, because a fixture directory
would sit outside the affected paths.

## Not changed

`src/cases.rs`, REQ-cli-005 and the `cases check` hint. Chough still reads no native
delta, keeps no lifecycle state of its own and does not ask SpecSync to change.

## Estimate

Small: three short skill paragraphs, three digests and two package tests. Main
uncertainty: the inline 825a380 reference consts in `tests/package.rs` and a textual
merge with PR #37 in `src/skill.rs`. `chough timing` on this repository (estimates
from recorded history, elapsed wall-clock, not targets): build median 26m 37s
(12 samples, 1s to 1h 30m 36s), test median 48s (11 samples), accept median 20m 37s
(12 samples). Human gates are not included.
