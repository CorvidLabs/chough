---
change: fix-314-folders
artifact: testing
---

# Testing

`cargo test --locked --test migrate migrate_skips_macos_metadata_beside_0_5_records` now also adds `.chough/history/._metadata-folder/empty-child`, an empty folder inside a `._*` folder. It requires the same plan digest and both folders still there after the apply. It fails before this change. Case `TC-records-migrate-skips-macos-metadata` binds it.
