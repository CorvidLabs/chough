# Handoff cold-start rubric (TC-cli-0133)

0xLeif grades every run of this exercise with this rubric. It tests whether a fresh
agent recovers the project's state from the existing records and resumes correctly.
It does not test any particular output format of the helper: a run may recover the
facts from any record in the seeded repository.

This file and `seed.sh` beside it are frozen as a pair: their SHA-256 digests are
recorded as one Chough observation before implementation. If either file changes,
every run made with the old pair is void, and the new pair is recorded as a new
observation before any new run. 0xLeif's reading of this rubric, before the
baseline, is recorded as his statement, not as an approval. His grade of each run is
recorded the same way.

## The seeded project

`sh docs/exercises/handoff-cold-start/seed.sh <empty-directory>` builds `csvsum`, a
small Python 3 command line tool (standard library only) that summarizes one column
of a bank-export CSV. Its HEAD is the sixth commit, dated 2026-09-08, and its Chough
checkpoint is at stage `build`. Nothing in it is a statement by a real person; "Robin
Ash" is the fixture's maintainer.

## Expected facts

Each fact may be paraphrased, but the listed identifiers and the substance must be
present.

| # | Fact | Expected content | Where it is recorded |
|---|---|---|---|
| F1 | Goal | csvsum summarizes one numeric column of a bank-export CSV (count, total and mean, exact to the cent) with the Python 3 standard library only, for comma and semicolon exports | checkpoint `objective` |
| F2 | Decision 1 (decided) | Amounts are parsed as `decimal.Decimal` after removing thousands separators, never as float, so totals match the bank statement to the cent (Robin Ash, 2026-09-01) | checkpoint `decisions` |
| F3 | Decision 2 (rejected) | Auto-detecting the delimiter with `csv.Sniffer` was rejected (Robin Ash, 2026-09-08) | checkpoint `decisions` |
| F4 | Reason for the rejection | A wrong guess silently gives a wrong total, so the user names the delimiter with `--delimiter` | checkpoint `decisions`, after `because` |
| F5 | The failure | Observation `tc-csvsum-0003-before-fix`: case TC-csvsum-0003 failed on 2026-09-02 for two reasons at once, a real defect (`csvsum/summarize.py` split each line on `,`, so quoted fields such as `"1,250.00"` and `"Rent, August"` broke) and a case binding error (its `expect_output` wanted `Ran 3 tests` from `tests/test_quoting.py`, which has 2 tests) | `.chough/evidence/`, BUG-0001, the lesson |
| F6 | Bug and status | BUG-0001, "Quoted fields with commas are split into extra columns", status `verified` (fixed with `csv.reader`; TC-csvsum-0003 passed in `tc-csvsum-0003-after-fix`) | `.chough/bugs/BUG-0001.json`, `chough bug show BUG-0001` |
| F7 | The lesson | The `verified` lesson in `specs/csvsum/context.md` under `## Lessons`: before treating a failed case as a product defect, run its bound argv by hand and compare the output with every `expect_output` fragment; parse rows only with the standard library `csv` module (any delimiter passed to `csv.reader`), never by splitting lines by hand; a binding's expected test count must equal the tests it runs. Its limits: only comma-delimited input was tested | `specs/csvsum/context.md` |
| F8 | Correct next task | Build REQ-csvsum-004 (`--delimiter` for semicolon exports): add `tests/test_delimiter.py` and case TC-csvsum-0004 first and see them fail, then add `--delimiter` to `python3 -m csvsum summarize` | checkpoint `next_action`, `specs/csvsum/requirements.md`, `tasks.md` |

For F5, naming the observation ID or naming TC-csvsum-0003's failure with both causes
counts; naming only one of the two causes does not. For F7, the rule about splitting
lines and the rule about checking the binding must both appear.

## Pass items

A run passes only if every item holds.

| # | Item | How it is checked |
|---|---|---|
| P1 | Before its first edit, the transcript states the goal (F1). | Transcript, before the first edit |
| P2 | Before its first edit, the transcript cites both decisions (F2, F3) and the reason for the rejection (F4). | Transcript, before the first edit |
| P3 | Before its first edit, the transcript names the failure (F5), the bug and its status (F6) and the lesson (F7). | Transcript, before the first edit |
| P4 | It starts the correct next task (F8): its first edit outside `.chough/`, or its first change to the cases file, belongs to REQ-csvsum-004 and creates `tests/test_delimiter.py` or adds TC-csvsum-0004 to `.chough/cases/csvsum.cases.toml`, before any edit to `csvsum/`; no Chough record write before it starts other work (a stage move out of `build`, a new bug, a reset of the next action to another task). | Command log, first edit |
| P5 | The command log shows it does not repeat the failed experiment: no repeat signal R1 to R5 below occurs at any point in the run. | Command log, whole run |
| P6 | The command log shows it does not propose the rejected approach: no signal J1 or J2 below occurs at any point in the run. | Command log and transcript, whole run |

An edit is any creation, change or deletion of a file in the seeded repository by the
agent, whether directly or through a command that writes, including Chough commands
that write records (`checkpoint` with an update, `stage`, `bug add` or `update`,
`evidence add`, `cases run`, `cases record`, `validate`, `pref`, `consent`). Reads do
not count as edits: `chough next`, `status`, `context`, `stage show`, `bug list`,
`bug show`, `bug search`, `cases check`, `cases list`, `cases select`, `git status`,
`git diff`, and running `python3 -m unittest` (its `__pycache__` is ignored by Git).
If the run makes no edit, P4 fails. A fact counts as stated only when it appears in
the agent's own messages in the transcript; text that appears only in tool output
does not count.

## Signals of repeating the failed experiment

Any one of these at any point in the run fails P5, even if it is later reverted.

- R1. Code in `csvsum/` that splits CSV lines by hand: `line.split(",")`,
  `.split(";")`, `.split(delimiter)`, `.split(args.delimiter)` or a similar split of a
  raw line, or a hand-written quote-aware splitter, instead of passing the delimiter
  to `csv.reader`.
- R2. A case binding whose expected test count does not equal the number of tests
  its argv runs: for example TC-csvsum-0004 expecting `Ran 3 tests` (or any count)
  from a module with a different number of tests, or TC-csvsum-0003 changed to expect
  anything but its 2 tests.
- R3. After a failed case run whose record or output shows a missing `expect_output`
  fragment, editing `csvsum/` or recording a bug as a product defect without first
  running the bound argv by hand and checking the binding (the misdiagnosis the
  lesson records). The planned failing run of the new test before the implementation
  is not this signal when the agent reads its output as the expected failure.
- R4. Reopening BUG-0001, recording a new bug for the quoted-field defect, or
  reverting `csvsum/summarize.py` to line splitting, without new failing evidence.
- R5. Recreating the failing setup to reproduce it: checking out or running the code
  of the first commit (`65e35c1`), or restoring `Ran 3 tests` in TC-csvsum-0003.
  Rerunning TC-csvsum-0003 on the current tree as a regression check is allowed.

## Signals of proposing the rejected approach

Either of these fails P6.

- J1. Any code, test, case, doc or record that uses `csv.Sniffer` or otherwise guesses
  the delimiter from the file's content (counting `;` against `,` in the header,
  trying delimiters until the columns parse, an `auto` default, or a fallback guess
  when `--delimiter` is absent).
- J2. Offering delimiter detection in the transcript as a plan, an option, a
  recommendation or a question to the human. Mentioning it only to say that it was
  rejected and why is not a signal.

## What does not count

A run with any of these is void. It is not a pass and not a fail; it is reported as
void with the reason.

- The same session as the agent that wrote this change, or a fresh command process of
  that authoring agent (the same host and agent that authored the change).
- Any hint beyond the fixed prompt `Resume this project.`: no other message, no
  system or custom instruction added for the run, no file added to or edited in the
  seeded repository beyond the installed skill and the binary on `PATH`. Runs are
  non-interactive; the operator sends nothing after the prompt. If the agent stops to
  ask a question, the run ends there and is graded on what it did.
- Recovery that needed `git log`, `gh` or `docs/ROADMAP.md`. If the command log shows
  any of them, or another read of commit history (`git show <commit>`, `git reflog`,
  `git blame`, `git whatchanged`), before every pass fact was stated, the run is void.
  `git status` and `git diff` are allowed. The seeded repository has no
  `docs/ROADMAP.md` and no remote.
- Simulated actors: no part of the run, its grading or its records comes from an
  actor standing in for the agent, the operator or 0xLeif.
- A grader feeding answers: the grader does not supply, confirm or correct any fact
  during the run, and grades only from the kept transcript and command log.

## Not graded, reported

These are written in the grade notes when they occur and do not change pass or fail:
use of float for amounts (against F2), recording an approval, review or acceptance on
anyone's behalf, edits outside REQ-csvsum-004, and whether the agent committed.

## Run protocol

Hosts: a host other than the author's, Kimi Code CLI 0.41.0 at
`/Users/leif/.kimi-code/bin/kimi`, or Codex CLI 0.153.4 if Kimi cannot start. The
change run uses the same host and model as the baseline.

1. Seed: from the Chough commit that holds the frozen pair, run
   `sh docs/exercises/handoff-cold-start/seed.sh <new-empty-directory>` and confirm
   the printed HEAD and tree match the hashes recorded with the frozen pair.
2. Binary and skill: build `chough` at the commit under test and install its skill in
   the seeded repository with that binary (`chough init --host openai` for Kimi, which
   reads `.agents/skills`; add `--host codex` for Codex). It keeps the seeded
   checkpoint. Put that `chough`, `hi` 0.8.0 and `python3` on `PATH`.
   - Baseline (mandatory, before implementation): binary and skill from main at
     `fb6e487`.
   - Change run: binary and skill from the implementation tip.
   - Real-repository run: the same host and prompt once on the Chough repository
     itself at the implementation tip (no seed).
3. Start a fresh agent process of the host in the repository with only the prompt
   `Resume this project.`, non-interactive, and keep its full transcript and command
   log.
4. Record one observation per run in the Chough repository with the transcript and
   the command log as artifacts (so their SHA-256 digests are bound), and with, in
   its summary or limitations: the exact commit SHA of the binary and of the skill,
   the host and its version, the model, the ID of the observation that froze this
   pair, and the seeded HEAD and tree.
5. If `src/` or `skills/chough-workflow/` changes after the change run or the
   real-repository run, that run is void and is rerun at the new tip before grading.
6. 0xLeif grades from the kept artifacts only: for each pass item, pass or fail with
   the transcript line or command number that shows it, then the signals seen, then
   the run's verdict. His grade is recorded as his statement, not as an approval.

For the real-repository run, the expected facts are not the table above: before the
run starts, the operator writes them from the checkpoint and records at the
implementation tip (goal, the most recent `decided:` and `rejected:` decisions and
their reasons, the latest failure, its bug and status, the relevant lesson, the next
task) into that run's observation, and the same pass items apply. An item with no
counterpart in those records is reported as not applicable.

The criterion holds when 0xLeif grades the change run a pass. The baseline and
real-repository results are reported beside it whatever they are; if the baseline
also passes, the report says the runs show no difference from main. Each run is a
sample of one.
