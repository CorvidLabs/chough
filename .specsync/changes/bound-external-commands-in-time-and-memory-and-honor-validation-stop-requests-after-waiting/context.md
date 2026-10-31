---
change: bound-external-commands-in-time-and-memory-and-honor-validation-stop-requests-after-waiting
artifact: context
---

# Context

Three execution defects were reproduced on main `f19823d` (macOS arm64) in disposable
copies of `examples/shipping`, each recorded with a failed observation and a log under
`.chough/artifacts/`:

- BUG-0009: `validate.rs` checks the stop file before sleeping, not after; a stop 1.5 s
  into a 4 s interval still allowed a second attempt.
- BUG-0010: `cases.rs` reads each pipe with `read_to_end` and truncates afterwards, and
  on timeout kills only the direct child, then joins readers that wait for EOF. A 1 s
  deadline returned after 20 s and a 3 s deadline after 21 s (recorded passed) while a
  `sleep` descendant held the pipes; 300 MB of output peaked at 578 MiB.
- BUG-0011: `records.rs` native reads use `Command::output()`: unlimited stdout and
  stderr, no deadline. 400 MB from a stand-in peaked at 774 MiB before `oversized`; a
  stand-in that never exits kept `chough next` waiting past 30 s.

BUG-0004's archived fix (parse complete output up to 4 MiB, display 16 KiB with a
note, reject oversized and undecodable output) stays as it is; this change bounds the
capture that feeds it. Gate rules are a separate change (BUG-0006 to BUG-0008).
