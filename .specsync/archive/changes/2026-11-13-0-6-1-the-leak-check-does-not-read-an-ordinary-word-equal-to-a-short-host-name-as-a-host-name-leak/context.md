---
change: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak
artifact: context
---

# Context

Chough 0.6.0's leak check reads the local host name as a leak wherever it appears. On a
machine whose short host name is an ordinary English word, a lesson that used the word
was reported by `chough doctor`, and a relayed `--where` quoting the human's words was
refused as `refused:leak`, so the human's quote had to be paraphrased. The bug is
recorded as `.chough/bugs/2026-11-12-the-leak-check-reads-an-ordinary-word-equal-to-a-short-host.md`,
deferred to 0.6.1 by Leif. On 2026-11-13T02:03Z he chose "Yes, start (Recommended)" for
0.6.1 as six small PRs, this one the leak check.

A short all-letter host name stays a leak inside a path or a URL or as `<user>@<host>`,
where it identifies the machine; a longer or hyphenated host name stays a leak
anywhere. The code stays in `src/leaks.rs`, which `chough doctor` shares.
