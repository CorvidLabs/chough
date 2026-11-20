# Lesson bundle: control-and-echo

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Control and echo
- **Kind**: Feature
- **Specs**: records, cli, history, reporting
- **Paths**: src/leaks.rs, src/markdown.rs, src/journal.rs, src/identity.rs, src/main.rs, src/context.rs, src/checkpoint.rs, src/records.rs, src/prefs.rs, src/skill.rs, src/stage.rs, src/validate.rs, src/library.rs, src/cases.rs, src/bugs.rs, src/history.rs, src/sessions.rs, src/timing.rs, tests/output_safety.rs, tests/leaks.rs, tests/next.rs, tests/checkpoint.rs, tests/history.rs, tests/timing.rs, tests/journal.rs, docs/cli.md, changelog.d/2026-11-19-control-and-echo.md, .chough/cases/records.control-and-echo.cases.toml, .chough/cases/cli.control-and-echo.cases.toml, .chough/cases/history.control-and-echo.cases.toml, .chough/cases/reporting.control-and-echo.cases.toml, src/evidence.rs, src/gates.rs, tests/front_matter_writes.rs, .chough/cases/records.test-stage.cases.toml, tests/spec_split.rs
- **Acceptance**: A control character (C0, DEL, C1, a bidi override or isolate) in a value a person gives is refused at write as refused:control-character, naming the field and code point, except a line feed where the field takes a line break, with CRLF stored as LF; text printed from a record or given is escaped as \u and four hex digits, text and JSON alike, with a line feed shown as \n on a one-line slot; YAML reserved scalars are written quoted (#366). checkpoint --pr takes only a number or an http(s) URL, an empty value removes it, and next, status, context and the checkpoint view show it; the journal post links only those forms (#353). The remaining error and output sites name the field and what it holds, never the value, proven by a black-box sweep (part of #367). A file: URL is checked like the path it carries, and USERNAME counts like USER and LOGNAME (REQ-records-006).

## Evidence

- Verification commit: `7c1b9f03cb0f8bc52d3402732cf11095ad9810cb`
- Base commit: `9a81c859ae449f9da541cab3538d318de411a1ee`
- Verified by: `specsync check --spec cli --spec history --spec records --spec reporting`

## From the change's context.md

# Context

Leif's decisions on the issues: #366 "both. New values holding control characters are refused at write (except line breaks where a field allows them), and stored text is shown escaped when printed, as status and context already do", and his widening to C0/C1 controls, DEL and bidi overrides in every field; #353 (a number or URL, shown in next, status, context and the checkpoint view; `--pr ""` clears it); #367 "approved. Errors name the field and the kind, never the offending text". #367's sites left after L1 are listed on the issue; the `file:` URL came from tester2's sweep of L1.

Measured on #407's head (9a81c85): every field but `--title` and `--pr` takes a line break today; `--pr` accepts `0`, `#12`, `src/pr.txt`, an ESC sequence, `null`, `true` and `ftp://`, and `--pr ""` stores `pr: ""`; the PR shows in none of the four views; `status` and `context --json` escape only what serde_json escapes (C0), so DEL, C1 and U+202E reach the terminal raw from them as from `next`. `context`'s text form escapes less still: on main 955edbc the tester saw it print a raw ESC, a C0 control, from a stored `no_native_change`; eleven read commands print stored control characters raw, `timing` among them.

Stacked on #407 (leaks-and-output), whose REQ-records-006 text the delta builds on; after #407, #413 and #414 merged, main 2968f06 was merged into the branch. It modifies no requirement that D1 (doctor-fix-safety: REQ-cli-021, REQ-history-006, REQ-history-012, REQ-records-001, REQ-records-014) or #311 (REQ-cli-067) modifies. G1 (#413) fixed the six change arguments. The two remaining `change.rs` sites, the `change new` description SpecSync echoes and `change ship`'s `--close-gate` text, are #412's, and so is `evidence.rs:790`.

The tester's survey of the real records of 22 CorvidLabs repositories (1,375 files, chough-swarm-0611/prs/l2survey/notes.md): no record, log or JSON string holds a tab or any other control character; every stored `pr` is a quoted string, 104 empty, 6 positive numbers and 2 `https` URLs, so the new rules reject nothing that exists; no unquoted `yes`, `no`, `null` or `true` exists; all 17 existing feeds parse. On that evidence the tab is refused like any other control character, as Leif's words read, and no exception is asked.

## From the change's design.md

# Design

- `src/leaks.rs` (records): the guard every written value passes refuses a control character as `refused:control-character`, naming the field and the code point, before the leak match and under `--redact` too, in a value the stored record does not already hold, so a value written by hand or by an earlier release never blocks a save and is escaped on print; it stores a CRLF as LF; the one-line checks of `--title` and `--pr` stay where they are. A `printable(text, one_line)` helper escapes control characters as `\u` and four hex digits, and a line feed as `\n` on one line. `in_absolute_path` reads a `file:` URL's path. The `USERNAME` variable is already read; the sentence now says so.
- `src/markdown.rs` (records): the scalar writer double-quotes `null`, `~`, `true`, `false`, `yes`, `no`, `on` and `off` in any letter case.
- `src/journal.rs` (records): the post links the PR only when it is a number or an `http(s)` URL; the feed writes each post's text in the printed form, so it stays well-formed XML.
- `src/library.rs` (cli) and `src/sessions.rs` (history): `knowledge/index.md` and the archived story write stored text in the printed form.
- `src/identity.rs` (records): the explicit evidence ID error uses `shown`.
- `src/records.rs` (cli): `print_json` also writes DEL, C1 controls and the bidi characters as `\u` escapes, which serde_json leaves raw, so every JSON view prints none raw and parses back the same.
- `src/main.rs`, `src/context.rs`, `src/checkpoint.rs`, `src/bugs.rs` (cli), `src/sessions.rs`, `src/history.rs` (history), `src/timing.rs` (reporting): each text view that prints stored text, `next`, `status`, `context`, the checkpoint view, `session list`, `inbox list`, `history show`, `bug list`, `bug show` and `timing`, goes through `printable`; `next` gains a `PR:` line, and `status`, `context` and the checkpoint view a `pr` field.
- `src/checkpoint.rs`: `--pr` accepts a positive whole number with no sign or leading zero, after an optional `#` it drops, or an `http://` or `https://` URL with a host and none of whitespace, `()[]<>`, quotes, a backtick or a backslash; `--pr ""` removes the key; `pr` is written double-quoted, and an empty stored `pr` reads as none in every view and in the journal post.
- `src/prefs.rs`, `src/skill.rs`, `src/stage.rs`, `src/validate.rs`, `src/library.rs`, `src/cases.rs` (cli), `src/history.rs`: the remaining echo sites, and the `--environment` and `cases select` output, use `shown`.

## From the change's testing.md

# Testing

Failing tests first, each red on the base first:
- `tests/output_safety.rs` (new): (1) the write sweep, ESC, BEL, DEL, U+009B, U+202E and U+2066 in each checked field, refused as `refused:control-character` with every file byte-identical, `--redact` too, a line feed accepted where a field takes one and a tab refused everywhere, CRLF stored as LF; (2) the print sweep, a fixture holding each control character by hand in every record kind, then every command in `chough --help`'s tree, walked from the help output so a new command joins by itself, in text and JSON with the fixture's IDs, with no raw control character and JSON that parses; (3) the echo sweep, every value slot of every command given a token and an escape sequence, with neither in any output (#367).
- `tests/leaks.rs`: `file:///opt/<user>/x` refused, `https://…/<user>/…` written; `USER`, `LOGNAME` and `USERNAME` set to three names.
- `tests/checkpoint.rs`: each `--pr` form accepted or refused as REQ-cli-007 lists; `--pr 42` written as `pr: "42"`; `--pr ""` removes `pr`; a stored `pr: ""` shows no PR in any view; YAML reserved scalars read back as text.
- `tests/next.rs`: the `PR:` line and the JSON field; a `--no-native-change` reason with a line feed shown as `\n`.
- `tests/journal.rs`: a stored `pr` that is neither form is left out of the post's links; a post and a session from an earlier release holding each control character give a feed that parses as XML, an index and a story with none raw.
- `tests/history.rs`: `correct` with a token or an escape sequence names what it holds.
- `tests/timing.rs`: a reason holding ESC and U+202E printed escaped, text and JSON.

One case file per module: `.chough/cases/{records,cli,history,reporting}.control-and-echo.cases.toml`, each case linking its own module's requirement.

## Where these lessons go

- `specs/records/context.md`
- `specs/cli/cli.3md`
- `specs/history/context.md`
- `specs/reporting/context.md`
