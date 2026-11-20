# Lesson bundle: change-leaks

Material for folding this change's lessons into each affected module's lesson file.
Synthesise from what actually happened below; do not restate the change description.

## What this change was

- **Title**: Change leaks
- **Kind**: BugFix
- **Specs**: cli, records
- **Paths**: src/change.rs, src/doctor.rs, src/evidence.rs, src/leaks.rs, tests/change.rs, tests/leaks.rs, tests/output_safety.rs, docs/cli.md, changelog.d/2026-11-20-change-leaks.md, .chough/cases/cli.change-leaks.cases.toml, .chough/cases/records.change-leaks.cases.toml
- **Acceptance**: A description, a --rationale and an answer holding a leak are refused by chough change new and chough change answer before SpecSync runs, naming the argument and the kind, with nothing under .specsync/ changed; with --redact the change's records hold <redacted:<kind>> and one line says how many were replaced; a control character is refused under --redact too (#412). chough doctor --leaks also scans the open changes under .specsync/changes/ and reports a leak there with the file, line and kind, which --fix --redact skips naming the way out; .specsync/archive/ is not scanned. change ship --close-gate, change new's description, change answer's answer and an explicit evidence ID are named by what they hold in any error or output (rest of #367).

## Evidence

- Verification commit: `7ab2ca20f460d5c9187e3dd8ffab3d64e0809901`
- Base commit: `7bf110dcca70bdd3565007086aee40a286f40eff`
- Verified by: `specsync check --spec cli --spec records`

## From the change's context.md

# Context

Issue #412 and the sites of #367 that L2 (#418, merged at 643420a9) left to it.

- `chough change new`'s description and `--rationale`, and `chough change answer`'s answer, reach committed files under `.specsync/changes/<id>/` (state.json, change.md) with no leak check (src/change.rs, the New and Answer arms). The IDs (kind, spec, path, id, question) are already refused by #413.
- `chough doctor --leaks`, CI's leak step, scans only files Git tracks under `.chough/` and the knowledge folder (src/doctor.rs, `tracked`).
- The echo sites left to #412 (tests/output_safety.rs's FOR_412): `change new`'s description, `change answer`'s answer, and `change ship --close-gate` in the relay refusal (src/change.rs, "closing gate"). `evidence.rs:798` says a bare "invalid evidence ID" for an explicit ID, while evidence.rs:922 names what it holds.

Measured on main with a throwaway build that adds SpecSync's folders to doctor's scan (scratchpad r412, one-lane):
- The 22 real repositories: scanning `.specsync/changes/` adds 2 findings in 2 repositories, both in artifacts and neither holding a user or host name:
  - Pix: `research.md`, a tilde path to a tool's folder (home-path);
  - site: `testing.md`, a temporary screenshot path (system-path).

  Those two repositories' leak scans would then exit 1. Scanning `.specsync/archive/` as well adds 279 findings in 7 repositories, among them 116 in Chough's own archive, which CI would then fail on for good. No other finding changed in any repository.
- The (A) replay: of every description and answer already in the 22 repositories, the leak check would have refused 4 values (`change.md` and `state.json`, all in archives: chough, jackdaw, spec-sync). No open change holds one.

Based on main 7bf110dc: #419 changed REQ-cli-067 only.

## From the change's design.md

# Design

- `src/change.rs` (cli): before SpecSync runs, the description, `--rationale` and answer are checked as a written value is (`leaks::guard`); a leak refuses as `refused:leak` naming the argument and kind, or under `--redact` is replaced with `<redacted:<kind>>` before SpecSync gets it, with one count line on standard error; a control character refuses under `--redact` too. The `--close-gate` text in the relay refusal is named by what it holds (`leaks::shown`).
- `src/doctor.rs` (cli): `tracked` adds `.specsync/changes/` except each change's `deltas/`, which hold requirement text as `specs/` does (a delta that quotes REQ-records-006's own example paths would otherwise be a finding: measured, this draft's own records delta gives 5); `--fix --redact` skips a leak finding there, naming `chough change answer` or a send-back. `.specsync/archive/` is not read.
- `src/evidence.rs` (cli): an explicit evidence ID that is not a valid name says what it holds, as line 922 does.
- `src/leaks.rs` (records): only a shared helper, such as the count line, if one is needed.

## From the change's testing.md

# Testing

Failing tests first, each red on main:
- `tests/change.rs`: each leak kind in a description, a `--rationale` and an answer is refused before SpecSync runs (the stub records no argv), naming the argument and kind, with nothing under `.specsync/` changed; with `--redact` the stub receives `<redacted:<kind>>` and the count line is printed; a control character is refused under `--redact` too.
- `tests/leaks.rs`: a token committed in an open change's `context.md` gives a `leak` finding with file, line and kind and exit 1; `--fix --redact` leaves it byte-identical and names the way out; a home path in an open change's delta and a leak under `.specsync/archive/` give no finding.
- `tests/output_safety.rs`: FOR_412 is removed, so the sweep covers `change new`'s description, `change answer`'s answer and `change ship --close-gate`; an explicit evidence ID holding a token is named by what it holds.

Cases: `.chough/cases/cli.change-leaks.cases.toml` (REQ-cli-057) and `.chough/cases/records.change-leaks.cases.toml` (REQ-records-006).

Sweep at main 7bf110dc: no active case is made false; no test pins REQ-cli-057 or REQ-records-006 (tests/spec_split.rs pins REQ-cli-044, REQ-cli-012 and REQ-cli-059 phrases and moved requirements); no test passes `change new` or `change answer` a value the check would refuse. A throwaway build scanning `.specsync/changes/` by default passed the full suite, 985 of 985, so the wider scan changes no existing test.

The run on the real repositories (rule 3, a new refusal): on fresh copies of the 22, every new `leak` finding under `.specsync/changes/` by repository, file and kind, read as true or false (measured on main: 2, Pix and site); none under `.specsync/archive/`; no other finding changed. Each repository's existing descriptions and answers replayed through the check (measured: 4 archived values, none open). A refused description given again with `--redact` makes a change SpecSync reads, and `change ready` and `change check` work on it. The tester's l2.py rows L2-change-ship-close-gate and L2-366-change-new-description, and i412.py's rows, pass.

## Where these lessons go

- `specs/cli/cli.3md`
- `specs/records/context.md`
