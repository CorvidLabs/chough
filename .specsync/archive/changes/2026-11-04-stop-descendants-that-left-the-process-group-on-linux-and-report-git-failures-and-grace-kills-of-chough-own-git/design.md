---
change: stop-descendants-that-left-the-process-group-on-linux-and-report-git-failures-and-grace-kills-of-chough-own-git
artifact: design
---

# Design

- src/records.rs `stop_tree` (Linux): before signalling the group, collect the child's
  descendants by walking `/proc/<pid>/stat` parent links, so processes that left the
  group with `setsid` are found; then SIGKILL the group and every collected process.
  When the binary has made Chough a child subreaper (below), orphans of the tree are
  re-parented to Chough, so Chough's own other children are collected too.
- src/main.rs (Linux): at startup the binary calls `prctl(PR_SET_CHILD_SUBREAPER, 1)`,
  so a double-forked descendant whose parent exits is re-parented to Chough instead of
  init and is found by the walk. Unit tests in the crate do not set it, so a test never
  stops another test's processes.
- `run_owned` prints `chough: stopped a process that kept Git's output open <n> s after
  Git exited` on stderr when its grace kill fires.
- `head_revision` prints `chough: could not read the Git revision (<reason>); this
  transition records no revision` on stderr when `git rev-parse HEAD` fails for a reason
  other than "not a git repository" or a HEAD with no commit, and still returns None.
