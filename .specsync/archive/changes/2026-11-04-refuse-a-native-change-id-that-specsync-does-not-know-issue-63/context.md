---
change: refuse-a-native-change-id-that-specsync-does-not-know-issue-63
artifact: context
---

# Context

Kimi's second lab review (2026-11-03, Chough 7bd1b62, SpecSync 6.1.2, round 2 item 2)
ran `chough checkpoint --native-change does-not-exist`: exit 0, `saved: true`, and the
checkpoint held the bad ID. Later reads then failed from that bad state (`next`
refused as `refused:native-unknown`, `status` showed the native read as failed).
Reproduced on main 4daf861 and filed as https://github.com/CorvidLabs/chough/issues/63.

`checkpoint_command` only checked the ID's format (`valid_change_id`). SpecSync 6.1.2
exits 1 with the error "change `<id>` was not found" for an unknown change.

Ruled out: changing what stage gates, `next` or `status` do with native state; clearing
a recorded change (unchanged); projects without SpecSync (nothing to ask, unchanged).
Related round-2 findings filed separately: #62 (concurrent writers) and #64 (orphaned
temp files).
