---
change: fix-298
artifact: context
---

# Context

GitHub #298, from CorvidLabs/starling.
- **Symptom:** `chough change ship` ran SpecSync's review and ship, both of which succeeded, and then the move into done was refused with `refused:native-archive-corrupt-history`. The ship had worked but reported an error, and the session was left at accept with its change archived.
- **Cause:** a merge of main into a build branch kept the branch's copy of a squash-merged change under `.specsync/changes/<id>/` while also adding main's archived copy under `.specsync/archive/changes/<date>-<id>/`. SpecSync reads the full history, so every change whose history holds that commit reports `terminal_evidence.validity: corrupt-history`.

Leif's decisions (2026-11-18, relayed by hc t4c):
- His decision comment on #298, "check before shipping": `change ship` first checks whether the archive's terminal evidence would authenticate, and refuses before SpecSync's review or ship if not, naming why and how to recover.
- "Both: Chough check now + SpecSync": SpecSync reports validity only after archiving, and `specsync change ship --dry-run` stops before the review, so Chough checks the change's own history now. t4c filed the SpecSync preflight as CorvidLabs/spec-sync#1082.

The Chough check is interim until that preflight exists, and the code, docs and REQ-cli-059 say so.
