---
change: read-requirements-with-crlf-line-endings-in-the-librarian-default-test-131-windows
artifact: tasks
---

# Tasks

- [x] Make the test check a CRLF copy of the requirements as well. It fails first on Linux, the same way it failed on Windows.
- [ ] Normalize `\r\n` to `\n` before joining the requirement lines.
- [ ] Run the optional tests and verify on Linux; the hosted Windows job must pass.
