---
change: read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows
artifact: context
---

# Context

PR #131 (C2, #124) was reviewed (Leif, "Record and merge", 19:50 MST) and closed out. Its hosted `Product / windows-latest` job then failed in `the_requirements_give_the_librarian_one_default_and_it_is_on`, at run 42153579452 on head `217680e2`.

Windows checks files out with CRLF line endings. The test replaced only `\n` before splitting sentences on ". ", so sentences ending ".\r" were never split. The resulting chunks named the librarian, a default and "off" together, and the check failed on them. Linux and macOS passed. This is a defect in the test, not in the requirement or the product.
