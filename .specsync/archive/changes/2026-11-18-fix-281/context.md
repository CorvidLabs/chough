---
change: fix-281
artifact: context
---

# Context

GitHub #281, from CorvidLabs/jackdaw#150, a planning PR. Its Chough session
recorded hi criteria, docs and issues and needed no SpecSync change. Once the
work was finished there was no honest way to archive it. With SpecSync
configured, every gated stage move refuses as `refused:no-native-change`, and
`chough session close` is for abandoned work: it withdraws open human gates and
archives with outcome `closed-without-done`. So an agent had to open a SpecSync
change it did not need, or record finished work as abandoned.

Leif's decision, in his comment on #281 on 2026-11-18 from his planning
interview: "done without a change. A session can record that it needs no native
change, with a reason, and then may advance to done and archive with outcome
`done`. Its human gates still have to be closed by a person, as in any other
session."

The record is a new checkpoint flag, `--no-native-change <reason>`, kept as
`no_native_change` in the session's front matter. No command is added. It is
never held together with a native change. A gate that drives a native approval
or review still refuses in such a session, since there is nothing to approve.
