---
change: a-project-in-a-subdirectory-of-its-git-repository-records-evidence-against-its-own-files-170-drive-18
artifact: context
---

# Context

Issue #170, found by the independent auditor in `_audits/specsync-chough-interim-20261106`, using the `subdirectory-ts-v2` fixture with the project in `app/`.
- `chough cases run` in `app/` refuses: "--root must be the Git repository root for evidence".
- The advice, `--root <git root>`, fails: "run chough init first", because the checkpoint is in `app/`.
- `status` reports the snapshot as unknown, and `evidence add` and `cases record` refuse the same way.

This is the Chough counterpart of SpecSync's D2 (spec-sync #879, fixed in #890). `docs/cli.md` documented it as a limitation. Leif captured DRIVE-18 ("Capture both").
