---
change: fix-245-chough-library-index-links-no-source-git-ignores-so-a-fresh-clone-s-index-check-passes
artifact: testing
---

# Testing

`tests/vault.rs`: `git init`, a `.gitignore` that ignores `.grok`, a commit,
`chough init --host grok`, another commit, `git clone`, then `chough library index
--check` in the clone. The test also expects the working tree's index to link nothing
under `.grok/`, and `--check` to pass there with the ignored copy present and with it
removed. Red at the test commit, with the issue's exact error. Green at the fix. The
full local suite runs at the final head.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-cli-043 | `tests/vault.rs`: the index links no source Git ignores, and a fresh clone's check passes |
