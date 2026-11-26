---
change: fix-435
artifact: context
---

# Context

Found by the 0.6.2 documentation audit and filed as chough#435.
`chough inbox add` still publishes each entry with `fs::hard_link`
(src/sessions.rs `publish_entry`, about line 627). On a volume without
hard links, such as exFAT, it fails the way `chough init` did before
#424. #260 and #424 removed the hard links from new bugs, journal posts
and `.chough/local/.gitignore`; the docs say only those work there.

The existing whole-or-not-at-all writer is `records::write_new_bytes`
(src/records.rs, with `write_aside_then_place` and an
`Unsupported` fallback to create-new in place). `tests/no_hard_links.rs`
mounts an exFAT image with hdiutil (ignored by default) and is the way
this was checked for init/bug add/journal add; it gains inbox add.