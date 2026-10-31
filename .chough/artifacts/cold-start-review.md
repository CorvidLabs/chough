# Cold-start review (TC-cli-0102)

Reviewer: Claude (Opus 5.5), agent-assisted. Full record: docs/exercises/cold-start.md.
Sample HEAD before the session: 654c140 (prepared with examples/feature-bug-loop.sh
prepare-cold-start; scaffolding with the fix removed). Chough binary built from the
branch after 97410b8; Rust sources unchanged since.

1. Pass. The session read the installed skill, then ran `chough next` first.
2. Pass. `chough next` reported native state and observation freshness; it then ran
   the native scoped check.
3. Pass. It went back to build for BUG-0001, fixed `>` to `>=`, reran the three
   focused cases, verified the bug, and stopped at accept with the human
   implementation review recorded as awaiting a human.
4. Pass. It never ran `chough validate` (standing preference off) and recorded no
   approval, review or finalize.

Limits: one run, one agent (Codex), one sample; the first attempt failed on an
unsupported configured model and is not counted; the sandbox blocked commits.
