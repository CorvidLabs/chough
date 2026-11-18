---
change: fix-339
artifact: context
---

# Context

GitHub #339 collects follow-ups from the tester's check of #336 (#265). Leif decided each one (2026-11-18), relayed by hc t4c as the scope approval:
1. **Code fences, exempt them:** a record that quotes an example conflict, or a lone `<<<<<<<` line of tool output, inside a fenced code block was refused.
2. **A stray `=======`** left by a half-resolved conflict, with no `<<<<<<<` or `>>>>>>>` around it, was not caught.
3. **Case TOML:** markers in a committed `.chough/cases/*.cases.toml` got no `doctor` finding, and `cases check` only said "invalid chough-testing definition".
4. **Migrate message:** the preview said nothing, and the `--apply` refusal named the 0.6 target, not the 0.5.x source that holds the markers.

The local team's attempt (chough-nemo-265) caught 2 and 3, but it refused plain heading underlines and wrote partially in migrate, so its rules are not copied.

For item 2, a heading underline and a stray separator look alike. Leif chose "Warn only" (relayed by hc t4c, 2026-11-18). A lone line of exactly seven `=` with no `<<<<<<<` or `>>>>>>>` around it is reported by `doctor` as "possible leftover conflict separator at line N", and no write is ever refused for it, since a valid heading underline may be followed directly by text. Doctor has no warning level, so the finding is marked advisory and never sets its exit status.
