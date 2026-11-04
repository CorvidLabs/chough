---
change: stop-what-chough-started-when-chough-is-stopped-issue-75
artifact: context
---

# Context

Kimi's field lab round 3 (2026-11-03, Chough 4daf861) is an agent lab review, not a
human pass. Its finding N2, reproduced by corvid-agent on v0.2.0 (edbc3c1), is
https://github.com/CorvidLabs/chough/issues/75: when Chough is stopped during
`chough cases run` or a native SpecSync read, the command it started keeps running.
`run_bounded` in src/records.rs starts each command in its own process group so the
deadline can stop the whole tree, but nothing stops that group when Chough itself is
signalled, and the separate group also keeps a terminal Ctrl-C from reaching it.

Leif confirmed the criterion at 7:07 PM MT on 2026-11-03 (card t819s9328): "Yes, when I
stop Chough everything it started stops too and nothing partial is recorded", answering
"When you stop Chough (Ctrl-C or a normal kill), everything it started stops too, and
nothing half-done gets saved." It is captured as FLOW-14. A SIGKILL watchdog was offered
and not chosen, so a hard kill of Chough is out of scope.

Authority: Leif's 2026-11-03 7:49 AM MT standing Chough maintainer authority for bounded
fixes to Chough bug tickets, his 6:52 PM MT answer "File all three as Chough issues and
fix the clear bugs as small PRs", and the FLOW-14 confirmation above.

Out of scope: SIGKILL, Windows console events (behavior unchanged), and Chough's own
short `git` reads, which stay in Chough's process group and so already receive a
terminal Ctrl-C.
