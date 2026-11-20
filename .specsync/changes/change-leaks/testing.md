---
change: change-leaks
artifact: testing
---

# Testing

Failing tests first, each red on main:
- `tests/change.rs`: each leak kind in a description, a `--rationale` and an answer is refused before SpecSync runs (the stub records no argv), naming the argument and kind, with nothing under `.specsync/` changed; with `--redact` the stub receives `<redacted:<kind>>` and the count line is printed; a control character is refused under `--redact` too.
- `tests/leaks.rs`: a token committed in an open change's `context.md` gives a `leak` finding with file, line and kind and exit 1; `--fix --redact` leaves it byte-identical and names the way out; a home path in an open change's delta and a leak under `.specsync/archive/` give no finding.
- `tests/output_safety.rs`: FOR_412 is removed, so the sweep covers `change new`'s description, `change answer`'s answer and `change ship --close-gate`; an explicit evidence ID holding a token is named by what it holds.

Cases: `.chough/cases/cli.change-leaks.cases.toml` (REQ-cli-057) and `.chough/cases/records.change-leaks.cases.toml` (REQ-records-006).

Sweep at main 7bf110dc: no active case is made false; no test pins REQ-cli-057 or REQ-records-006 (tests/spec_split.rs pins REQ-cli-044, REQ-cli-012 and REQ-cli-059 phrases and moved requirements); no test passes `change new` or `change answer` a value the check would refuse. A throwaway build scanning `.specsync/changes/` by default passed the full suite, 985 of 985, so the wider scan changes no existing test.

The run on the real repositories (rule 3, a new refusal): on fresh copies of the 22, every new `leak` finding under `.specsync/changes/` by repository, file and kind, read as true or false (measured on main: 2, Pix and site); none under `.specsync/archive/`; no other finding changed. Each repository's existing descriptions and answers replayed through the check (measured: 4 archived values, none open). A refused description given again with `--redact` makes a change SpecSync reads, and `change ready` and `change check` work on it. The tester's l2.py rows L2-change-ship-close-gate and L2-366-change-new-description, and i412.py's rows, pass.
