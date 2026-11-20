---
change: fix-g1-gates-and-pins-fail-closed
artifact: design
---

# Design

Nothing visual changes. New and changed text a person sees:

- A session whose `no_native_change` is blank, a placeholder, or held with a native
  change is refused by every reader, through the existing "cannot be read" wrapper,
  with a line naming the field and the edit.
- `chough session use` and `chough session new`, given a `--session` or
  `CHOUGH_SESSION` that names another open session, add one line on standard error;
  standard output stays the same JSON.
- `chough change ship` can refuse as `refused:history-unchecked`.
- `chough change new|answer|check|status|ready|ship` refuse an argument holding a leak
  or a control character before SpecSync runs.
