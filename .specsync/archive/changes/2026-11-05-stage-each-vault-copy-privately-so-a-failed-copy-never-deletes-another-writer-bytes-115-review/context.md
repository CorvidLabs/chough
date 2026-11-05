---
change: stage-each-vault-copy-privately-so-a-failed-copy-never-deletes-another-writer-bytes-115-review
artifact: context
---

# Context

Leif's agent review of #115 at 628f974b (blocking P1, thread on src/evidence.rs:2700), upheld by corvid-agent's re-review at a6cfd329: after `create_new` made a vault copy, a failing `write_all` or `set_permissions` led to an unconditional `remove_file` on the destination. If another writer had replaced the attachment in between, that removed their bytes. The copy was not yet in `copies`, so the rollback could not recover them. This is the same foreign-byte class as #110. It was reproduced with a syscall fault shim on macOS: the shim rewrote the attachment, then `set_permissions` returned EIO. Leif chose "Record and merge" for #115 (2026-11-05 10:18 MST).
