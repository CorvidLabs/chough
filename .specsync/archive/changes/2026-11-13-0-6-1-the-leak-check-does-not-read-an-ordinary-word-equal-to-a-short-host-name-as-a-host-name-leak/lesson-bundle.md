# Lesson bundle: 0-6-1-the-leak-check-does-not-read-an-ordinary-word-equal-to-a-short-host-name-as-a-host-name-leak

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: 0.6.1: the leak check does not read an ordinary word equal to a short host name as a host-name leak
- **Kind**: BugFix
- **Specs**: records
- **Paths**: src/leaks.rs, tests/leaks.rs, docs/cli.md, CHANGELOG.md
- **Acceptance**: With a short all-letter host name, a record using it as an ordinary word is written and chough doctor reports nothing, while the same name inside a path, a URL or as <user>@<host> is still refused; a host name of 8 or more characters, or one holding a digit, hyphen or dot, is still refused anywhere (red then green)

## Evidence

- Verification commit: `4ca672d671b04ff14fb033ed13ed7ffd709e95d9`
- Base commit: `9d53604071df50b537adccad0ccf5cac9a2fc32b`
- Verified by: `specsync check --spec records`

## From the change's context.md

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

## From the change's testing.md

# Testing

A unit test in `src/leaks.rs` with a synthetic short host name through `Local::new`, never this machine's: a record using
it as a word is written and `chough doctor` finds nothing; the same name inside a path,
a URL or as `<user>@<host>` is refused; a long or hyphenated host name is refused
anywhere. Red at the test commit, green at the fix.

## Requirement evidence

| Requirement | Evidence |
|---|---|
| REQ-records-006 | `src/leaks.rs` (`leaks::tests::a_short_all_letter_host_name_is_a_leak_only_in_a_path_a_url_or_user_at_host`): a short all-letter host name is a leak only inside a path, a URL or `<user>@<host>` |

## Where these lessons go

- `specs/records/context.md`
