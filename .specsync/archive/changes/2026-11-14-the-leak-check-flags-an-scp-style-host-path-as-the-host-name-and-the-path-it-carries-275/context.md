---
change: the-leak-check-flags-an-scp-style-host-path-as-the-host-name-and-the-path-it-carries-275
artifact: context
---

# Context

Issue #275: the leak check for committed records (REQ-records-006) flagged a home or system path, the local host name in a path or as `<user>@<host>`, emails and credentials, but not an scp-style remote path `host:/path` such as `buildbox:/Users/name/project`. Found by t384 in the post-merge review of #241.

The code change and its test were written by Flint 40B (local, Ollama) through Pi in a rerun of the local-model bake-off, from the workflow skill's new fast path (#285); Claude Code brought the commit onto main, opened this session and native change, and moved the case into its own file.
