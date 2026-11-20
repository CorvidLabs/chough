---
change: control-and-echo
artifact: context
---

# Context

Leif's decisions on the issues: #366 "both. New values holding control characters are refused at write (except line breaks where a field allows them), and stored text is shown escaped when printed, as status and context already do", and his widening to C0/C1 controls, DEL and bidi overrides in every field; #353 (a number or URL, shown in next, status, context and the checkpoint view; `--pr ""` clears it); #367 "approved. Errors name the field and the kind, never the offending text". #367's sites left after L1 are listed on the issue; the `file:` URL came from tester2's sweep of L1.

Measured on #407's head (9a81c85): every field but `--title` and `--pr` takes a line break today; `--pr` accepts `0`, `#12`, `src/pr.txt`, an ESC sequence, `null`, `true` and `ftp://`, and `--pr ""` stores `pr: ""`; the PR shows in none of the four views; `status` and `context --json` escape only what serde_json escapes (C0), so DEL, C1 and U+202E reach the terminal raw from them as from `next`. `context`'s text form escapes less still: on main 955edbc the tester saw it print a raw ESC, a C0 control, from a stored `no_native_change`; eleven read commands print stored control characters raw, `timing` among them.

Stacked on #407 (leaks-and-output), whose REQ-records-006 text the delta builds on; after #407 merges the branch is rebased onto main. It modifies no requirement that D1 (doctor-fix-safety: REQ-cli-021, REQ-history-006, REQ-history-012, REQ-records-001, REQ-records-014) or #311 (REQ-cli-067) modifies. The `change.rs` sites are G1's.

The tester's survey of the real records of 22 CorvidLabs repositories (1,375 files, chough-swarm-0611/prs/l2survey/notes.md): no record, log or JSON string holds a tab or any other control character; every stored `pr` is a quoted string, 104 empty, 6 positive numbers and 2 `https` URLs, so the new rules reject nothing that exists; no unquoted `yes`, `no`, `null` or `true` exists; all 17 existing feeds parse. On that evidence the tab is refused like any other control character, as Leif's words read, and no exception is asked.
