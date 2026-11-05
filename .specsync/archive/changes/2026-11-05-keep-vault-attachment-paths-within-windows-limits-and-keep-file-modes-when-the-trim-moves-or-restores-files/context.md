---
change: keep-vault-attachment-paths-within-windows-limits-and-keep-file-modes-when-the-trim-moves-or-restores-files
artifact: context
---

# Context

Three defects in #109, found by CI and by corvid-agent's review at 6e0a835:

1. **Windows checkout.** The closeout trim moved 165 key items to `knowledge/attachments/<archive id>/<path under .chough/>`. This repository's archive IDs run to about 120 characters, so attachment paths reached 246, and Product / windows-latest failed in checkout ("unable to create file knowledge/attachments/claude-host-installs-…"). A tracked path the tool generates must stay short enough to clone.
2. **File modes.** A refused apply restores each removed file with create-new, which gives it the default mode. The refused first closeout trim (76fb917) turned two executable scripts into 0644. The vault copies of key items use `fs::write` and lose the mode the same way. KNOW-10 says removed files are restored, and a restore that changes the mode is not faithful.
3. **CI identity.** tests/reduce.rs `commit_aged` runs `git commit` without the identity the fixture's other Git calls pass, so it fails on CI runners with no global Git identity.

Leif chose "Yes, record and merge" for #109 and its fixes, and "Widen KNOW-19" for corvid-agent's wording point (2026-11-05). This branch's trim was undone in 8a91c5c, so the next move into done reruns it under the fixed layout.
