---
change: control-and-echo
artifact: context
---

# Context

Leif's decisions on the issues: #366 "both. New values holding control characters are refused at write (except line breaks where a field allows them), and stored text is shown escaped when printed, as status and context already do", and his widening to C0/C1 controls, DEL and bidi overrides in every field; #353 (a number or URL, shown in next, status, context and the checkpoint view; `--pr ""` clears it); #367 "approved. Errors name the field and the kind, never the offending text". #367's sites left after L1 are listed on the issue; the `file:` URL came from tester2's sweep of L1.

Measured on #407's head (9a81c85): every field but `--title` and `--pr` takes a line break today; `--pr` accepts `0`, `#12`, `src/pr.txt`, an ESC sequence, `null`, `true` and `ftp://`, and `--pr ""` stores `pr: ""`; the PR shows in none of the four views; `status` and `context` escape only what serde_json escapes (C0), so DEL, C1 and U+202E reach the terminal raw from them as from `next`; eleven read commands print stored control characters raw, `timing` among them.

Stacked on #407 (leaks-and-output), whose REQ-records-006 text the delta builds on; after #407 merges the branch is rebased onto main. It modifies no requirement that D1 (doctor-fix-safety: REQ-cli-021, REQ-history-006, REQ-history-012, REQ-records-001, REQ-records-014) or #311 (REQ-cli-067) modifies. The `change.rs` sites are G1's.
