#!/bin/sh
# Handoff cold-start fixture: builds a disposable Git repository for the fresh-agent
# resume test (TC-cli-0133) from static content embedded in this script.
#
# The seeded project, csvsum, is a small Python 3 command line tool (standard library
# only) that summarizes one column of a bank-export CSV. Its Chough records hold a
# goal, two human decisions (one rejected approach with its reason), one failed
# experiment that mixed a real defect with a case binding error, the verified bug and
# its labelled lesson, and an unfinished next task.
#
# Nothing is downloaded and nothing is read from the Chough repository. No Chough
# binary runs here: the records below were captured from chough 0.1.0 (main fb6e487)
# running the cases in a scratch copy, then written as static files with fixed times,
# the commit hashes filled in from this build, and the scratch path in one test log
# replaced by /Users/robin/src/csvsum, so the same tree is built whichever binary later
# reads it. Every digest in them matches the files written here. Commits use a fixed identity and fixed dates, so two runs produce the same
# commit hashes. The run harness installs the skill and binary under test afterwards
# (for example `chough init --host openai`, which keeps the existing checkpoint).
#
# Usage: sh seed.sh <empty-or-missing-target-directory>
# Needs: sh, git. The seeded project needs python3; its cases need chough and hi.
set -eu

fail() { printf 'seed failed: %s\n' "$*" >&2; exit 1; }

target=${1:-}
[ -n "$target" ] || fail "usage: sh seed.sh <empty-or-missing-target-directory>"
if [ -e "$target" ]; then
    [ -d "$target" ] || fail "$target is not a directory"
    [ -z "$(ls -A "$target")" ] || fail "$target is not empty"
fi
command -v git >/dev/null 2>&1 || fail "missing required tool: git"
mkdir -p "$target"
cd "$target"

# Reproducible Git: no user or system configuration, fixed identity, dates and zone.
GIT_CONFIG_GLOBAL=/dev/null
GIT_CONFIG_NOSYSTEM=1
TZ=UTC
LC_ALL=C
export GIT_CONFIG_GLOBAL GIT_CONFIG_NOSYSTEM TZ LC_ALL
umask 022
git -c init.defaultBranch=main init -q
git config user.name "Robin Ash"
git config user.email robin.ash@example.invalid
git config commit.gpgsign false
git config core.autocrlf false
git config core.fileMode true

commit() {
    git add -A
    GIT_AUTHOR_DATE="$1" GIT_COMMITTER_DATE="$1" git commit -q -m "$2"
}

# ---------------------------------------------------------------------------
# Commit 1 (2026-09-01): the project, its intent, contract, tests and cases.
# summarize.py splits lines on "," (the real defect) and TC-csvsum-0003 expects
# "Ran 3 tests" from a module with 2 tests (the case binding error).
# ---------------------------------------------------------------------------
mkdir -p csvsum tests samples hi specs/csvsum .chough/cases .chough/evidence \
    .chough/artifacts .chough/bugs

cat >.gitignore <<'EOF'
__pycache__/
*.pyc
EOF

cat >README.md <<'EOF'
# csvsum

Summarize one numeric column of a bank-export CSV: row count, total and mean, exact
to the cent. Python 3 standard library only.

```sh
python3 -m csvsum summarize samples/expenses.csv --column amount
python3 -m unittest
```
EOF

cat >AGENTS.md <<'EOF'
# csvsum

A small Python 3 command line tool; standard library only, no pip installs. Use the
Chough workflow skill in `.agents/skills/chough-workflow/` or
`.claude/skills/chough-workflow/`, and start with `chough next --environment <env>`.
Human intent is in `hi/`, the module contract and its lessons are in `specs/csvsum/`,
case definitions are in `.chough/cases/csvsum.cases.toml`, and checks run with
`python3 -m unittest`. Never record a human approval or review on anyone's behalf.
EOF

cat >INTENT.md <<'EOF'
# csvsum

A household-accounts helper. Its human intent is Robin Ash's.

## Features

<!-- hi:index -->
- [summary](hi/summary.md): SUMMARY (4 criteria)
<!-- /hi:index -->
EOF

cat >hi/summary.md <<'EOF'
---
hi: 1
families: [SUMMARY]
---

# Summary

## Intent

Robin Ash, who keeps the household accounts: "Point it at the bank's CSV export, name the amount column, and tell me how many rows, the total and the average, to the cent. It has to run on the plain python3 that is already on the laptop."

## Criteria

- **SUMMARY-1**  `python3 -m csvsum summarize <file> --column <name>` prints the row count, total and mean of that column.
- **SUMMARY-2**  Totals and means are exact to the cent, with no floating-point rounding.
- **SUMMARY-3**  A quoted field is one field, including an amount with a thousands separator such as "1,250.00" and a payee that contains a comma.
- **SUMMARY-4**  A semicolon-delimited export is summarized when the user names its delimiter with `--delimiter ";"`.
EOF

cat >specs/csvsum/csvsum.spec.md <<'EOF'
---
module: csvsum
version: 1
status: active
files:
  - csvsum/summarize.py
  - csvsum/__main__.py

db_tables: []
depends_on: []
---

# csvsum

## Purpose

Summarize one numeric column of a bank-export CSV: count, total and mean, exact to
the cent.

## Public API

| Name | Description |
|------|-------------|
| `python3 -m csvsum summarize <path> --column <name>` | Prints `count <n>`, `total <amount>` and `mean <amount>` |
| `summarize(lines, column)` | Count, total and mean of `column` over CSV lines, header first |
| `parse_amount(text)` | An amount such as `1,250.00` as a `Decimal` |

## Invariants

1. Amounts are `decimal.Decimal`, never `float`.
2. Only the Python 3 standard library is used.

## Behavioral Examples

```gherkin
Given a CSV export with a header row and an amount column
When csvsum summarizes the amount column
Then it prints the row count, the total and the mean to the cent
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Unknown column | The header has no such column | Exits with an error naming the column |

## Dependencies

None beyond the Python 3 standard library.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-09-01 | Count, total and mean; quoted fields; semicolon exports planned |
EOF

cat >specs/csvsum/requirements.md <<'EOF'
# csvsum requirements

### REQ-csvsum-001

`python3 -m csvsum summarize <path> --column <name>` SHALL print `count <n>`, `total <amount>` and `mean <amount>` for the named column. Trace: SUMMARY-1.

Acceptance Criteria
- tests/test_summary.py asserts the count, total and mean of a three-row export.

### REQ-csvsum-002

Amounts SHALL be parsed as `decimal.Decimal` after removing thousands separators, never as `float`; the total and the mean SHALL be shown to the cent, the mean rounded half to even. Trace: SUMMARY-2.

Acceptance Criteria
- tests/test_summary.py asserts that three amounts of 0.10 total exactly 0.30.

### REQ-csvsum-003

Rows SHALL be parsed so that a double-quoted field is one field, including an amount with a thousands separator and a payee that contains a comma. Trace: SUMMARY-3.

Acceptance Criteria
- tests/test_quoting.py asserts the total of samples like samples/quoted.csv.

### REQ-csvsum-004

`summarize` SHALL accept `--delimiter <one character>`, default `,`, and parse every row with that delimiter, so a semicolon export gives the same count, total and mean as the equivalent comma export. Trace: SUMMARY-4.

Acceptance Criteria
- tests/test_delimiter.py asserts that samples/expenses-semicolon.csv summarized with `;` matches samples/expenses.csv summarized with `,`.
EOF

cat >specs/csvsum/testing.md <<'EOF'
# csvsum testing

Product checks run with `python3 -m unittest`; case definitions live in
`.chough/cases/csvsum.cases.toml` and run with `chough cases run <id> --environment
local-macos`.

| Requirement | Case | Test |
|---|---|---|
| REQ-csvsum-001 | TC-csvsum-0001 | tests/test_summary.py |
| REQ-csvsum-002 | TC-csvsum-0002 | tests/test_summary.py |
| REQ-csvsum-003 | TC-csvsum-0003 | tests/test_quoting.py |
| REQ-csvsum-004 | TC-csvsum-0004 (to add) | tests/test_delimiter.py (to add) |
EOF

cat >specs/csvsum/tasks.md <<'EOF'
# csvsum tasks

- [x] Count, total and mean of one column (REQ-csvsum-001, REQ-csvsum-002).
- [x] Quoted fields (REQ-csvsum-003).
- [ ] `--delimiter` for semicolon exports (REQ-csvsum-004).
EOF

cat >specs/csvsum/context.md <<'EOF'
# csvsum context

csvsum summarizes one numeric column of a bank-export CSV. It is one Python 3
package with no third-party dependencies: run it with `python3 -m csvsum` and test it
with `python3 -m unittest`.

Exports seen so far: comma-delimited with double-quoted fields
(samples/quoted.csv), and from one bank semicolon-delimited with dot decimals
(samples/expenses-semicolon.csv).
EOF

cat >samples/expenses.csv <<'EOF'
date,payee,amount
2026-08-01,Grocer,40.00
2026-08-02,Bakery,12.50
2026-08-03,Cafe,7.50
EOF

cat >samples/expenses-semicolon.csv <<'EOF'
date;payee;amount
2026-08-01;Grocer;40.00
2026-08-02;Bakery;12.50
2026-08-03;Cafe;7.50
EOF

cat >samples/quoted.csv <<'EOF'
date,payee,amount
2026-08-01,"Rent, August","1,250.00"
2026-08-02,Grocer,50.50
2026-08-03,"Smith, J.",20.00
EOF

: >csvsum/__init__.py
: >tests/__init__.py

cat >csvsum/__main__.py <<'EOF'
"""Command line: python3 -m csvsum summarize <path> --column <name>."""
import argparse
import sys

from csvsum.summarize import summarize


def main(argv=None):
    parser = argparse.ArgumentParser(prog="csvsum")
    commands = parser.add_subparsers(dest="command", required=True)
    summary = commands.add_parser("summarize", help="count, total and mean of one column")
    summary.add_argument("path")
    summary.add_argument("--column", required=True)
    args = parser.parse_args(argv)
    with open(args.path, newline="", encoding="utf-8") as handle:
        try:
            result = summarize(handle, args.column)
        except ValueError as error:
            parser.exit(2, f"csvsum: {error}\n")
    print(f"count {result['count']}")
    print(f"total {result['total']}")
    print(f"mean {result['mean']}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
EOF

cat >csvsum/summarize.py <<'EOF'
"""Summarize one numeric column of a bank-export CSV, exact to the cent."""
from decimal import ROUND_HALF_EVEN, Decimal

CENT = Decimal("0.01")


def parse_amount(text):
    """An amount such as 1,250.00 or "1,250.00" as a Decimal."""
    return Decimal(text.strip().strip('"').replace(",", ""))


def summarize(lines, column):
    """Count, total and mean of `column` over CSV `lines`, header first."""
    rows = [line.rstrip("\r\n").split(",") for line in lines if line.strip()]
    header, data = rows[0], rows[1:]
    if column not in header:
        raise ValueError(f"no column named {column!r}")
    index = header.index(column)
    amounts = [parse_amount(row[index]) for row in data]
    total = sum(amounts, Decimal("0")).quantize(CENT)
    mean = (total / len(amounts)).quantize(CENT, ROUND_HALF_EVEN) if amounts else Decimal("0.00")
    return {"count": len(amounts), "total": total, "mean": mean}
EOF

cat >tests/test_summary.py <<'EOF'
import unittest
from decimal import Decimal

from csvsum.summarize import summarize


class SummaryTest(unittest.TestCase):
    def test_count_total_mean(self):
        lines = [
            "date,payee,amount\n",
            "2026-08-01,Grocer,40.00\n",
            "2026-08-02,Bakery,12.50\n",
            "2026-08-03,Cafe,7.50\n",
        ]
        self.assertEqual(
            summarize(lines, "amount"),
            {"count": 3, "total": Decimal("60.00"), "mean": Decimal("20.00")},
        )

    def test_total_is_exact_to_the_cent(self):
        lines = ["date,payee,amount\n"] + [f"2026-08-0{day},Bus,0.10\n" for day in (1, 2, 3)]
        result = summarize(lines, "amount")
        self.assertEqual(result["total"], Decimal("0.30"))
        self.assertEqual(result["mean"], Decimal("0.10"))


if __name__ == "__main__":
    unittest.main()
EOF

cat >tests/test_quoting.py <<'EOF'
import unittest
from decimal import Decimal

from csvsum.summarize import summarize


class QuotingTest(unittest.TestCase):
    def test_quoted_amount_with_thousands_separator(self):
        lines = [
            "date,payee,amount\n",
            '2026-08-01,"Rent, August","1,250.00"\n',
            "2026-08-02,Grocer,50.50\n",
        ]
        self.assertEqual(summarize(lines, "amount")["total"], Decimal("1300.50"))

    def test_quoted_payee_does_not_shift_columns(self):
        lines = ["date,payee,amount\n", '2026-08-03,"Smith, J.",20.00\n']
        self.assertEqual(summarize(lines, "amount")["total"], Decimal("20.00"))


if __name__ == "__main__":
    unittest.main()
EOF

write_cases() {
    quoted_count=$1
    cat >.chough/cases/csvsum.cases.toml <<EOF
# csvsum case definitions (chough-testing/1). Observations live in .chough/evidence/.
profile = "chough-testing/1"
module = "csvsum"

[[case]]
id = "TC-csvsum-0001"
title = "Count, total and mean of one column"
status = "active"
track = "functional"
method = "automated"
criteria = ["SUMMARY-1"]
requirements = ["REQ-csvsum-001"]
paths = ["csvsum/", "tests/test_summary.py"]
environments = ["local-macos"]
artifacts = ["test-log"]
expected = ["tests.test_summary.SummaryTest.test_count_total_mean passes"]

[case.bind]
adapter = "command"
argv = ["python3", "-m", "unittest", "tests.test_summary.SummaryTest.test_count_total_mean"]
expect_output = ["Ran 1 test", "OK"]

[[case]]
id = "TC-csvsum-0002"
title = "Totals are exact to the cent"
status = "active"
track = "functional"
method = "automated"
criteria = ["SUMMARY-2"]
requirements = ["REQ-csvsum-002"]
paths = ["csvsum/", "tests/test_summary.py"]
environments = ["local-macos"]
artifacts = ["test-log"]
expected = ["tests.test_summary.SummaryTest.test_total_is_exact_to_the_cent passes"]

[case.bind]
adapter = "command"
argv = ["python3", "-m", "unittest", "tests.test_summary.SummaryTest.test_total_is_exact_to_the_cent"]
expect_output = ["Ran 1 test", "OK"]

[[case]]
id = "TC-csvsum-0003"
title = "Quoted fields are single fields"
status = "active"
track = "functional"
method = "automated"
criteria = ["SUMMARY-3"]
requirements = ["REQ-csvsum-003"]
paths = ["csvsum/", "tests/test_quoting.py"]
environments = ["local-macos"]
artifacts = ["test-log"]
expected = ["Both tests in tests/test_quoting.py pass"]

[case.bind]
adapter = "command"
argv = ["python3", "-m", "unittest", "tests.test_quoting"]
expect_output = ["Ran ${quoted_count} tests", "OK"]
EOF
}
write_cases 3

cat >.chough/session.json <<'EOF'
{
  "schema_version": 2,
  "objective": "csvsum: summarize one numeric column of a bank-export CSV (count, total and mean, exact to the cent) with the Python 3 standard library only, for comma and semicolon exports",
  "intent_scope": "SUMMARY",
  "modules": [
    "csvsum"
  ],
  "stage": {
    "current": "intent",
    "status": "active",
    "callers": [],
    "iteration": 1
  },
  "native_change": null,
  "references": {
    "criteria": [
      "SUMMARY-1",
      "SUMMARY-2",
      "SUMMARY-3",
      "SUMMARY-4"
    ],
    "requirements": [
      "REQ-csvsum-001",
      "REQ-csvsum-002",
      "REQ-csvsum-003",
      "REQ-csvsum-004"
    ],
    "cases": [
      "TC-csvsum-0001",
      "TC-csvsum-0002",
      "TC-csvsum-0003"
    ],
    "design": [],
    "files": []
  },
  "decisions": [
    "decided: amounts are parsed as decimal.Decimal after removing thousands separators, never as float, so totals match the bank statement to the cent (Robin Ash, 2026-09-01)"
  ],
  "open_questions": [],
  "completed": [],
  "observations": [],
  "bugs": [],
  "blockers": [],
  "awaiting_human": [],
  "run_consents": [],
  "next_action": "Confirm SUMMARY-1 to SUMMARY-4 with Robin, then build and test REQ-csvsum-001 to REQ-csvsum-003 before REQ-csvsum-004."
}
EOF

commit "2026-09-01T09:00:00Z" "Start csvsum: count, total and mean of one CSV column"
rev1=$(git rev-parse HEAD)

# ---------------------------------------------------------------------------
# Commit 2 (2026-09-02): the failed run of TC-csvsum-0003 at commit 1 and BUG-0001.
# ---------------------------------------------------------------------------
cat >.chough/artifacts/tc-csvsum-0003-before-fix.log <<'EOF'
--- stdout ---

--- stderr ---
EE
======================================================================
ERROR: test_quoted_amount_with_thousands_separator (tests.test_quoting.QuotingTest.test_quoted_amount_with_thousands_separator)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/Users/robin/src/csvsum/tests/test_quoting.py", line 14, in test_quoted_amount_with_thousands_separator
    self.assertEqual(summarize(lines, "amount")["total"], Decimal("1300.50"))
                     ~~~~~~~~~^^^^^^^^^^^^^^^^^
  File "/Users/robin/src/csvsum/csvsum/summarize.py", line 19, in summarize
    amounts = [parse_amount(row[index]) for row in data]
               ~~~~~~~~~~~~^^^^^^^^^^^^
  File "/Users/robin/src/csvsum/csvsum/summarize.py", line 9, in parse_amount
    return Decimal(text.strip().strip('"').replace(",", ""))
decimal.InvalidOperation: [<class 'decimal.ConversionSyntax'>]

======================================================================
ERROR: test_quoted_payee_does_not_shift_columns (tests.test_quoting.QuotingTest.test_quoted_payee_does_not_shift_columns)
----------------------------------------------------------------------
Traceback (most recent call last):
  File "/Users/robin/src/csvsum/tests/test_quoting.py", line 18, in test_quoted_payee_does_not_shift_columns
    self.assertEqual(summarize(lines, "amount")["total"], Decimal("20.00"))
                     ~~~~~~~~~^^^^^^^^^^^^^^^^^
  File "/Users/robin/src/csvsum/csvsum/summarize.py", line 19, in summarize
    amounts = [parse_amount(row[index]) for row in data]
               ~~~~~~~~~~~~^^^^^^^^^^^^
  File "/Users/robin/src/csvsum/csvsum/summarize.py", line 9, in parse_amount
    return Decimal(text.strip().strip('"').replace(",", ""))
decimal.InvalidOperation: [<class 'decimal.ConversionSyntax'>]

----------------------------------------------------------------------
Ran 2 tests in 0.001s

FAILED (errors=2)
EOF

cat >.chough/evidence/tc-csvsum-0003-before-fix.json <<EOF
{
  "schema_version": 2,
  "id": "tc-csvsum-0003-before-fix",
  "criteria": [
    "SUMMARY-3"
  ],
  "requirements": [
    "REQ-csvsum-003"
  ],
  "contracts": [
    "specs/csvsum/csvsum.spec.md"
  ],
  "kind": "product-test",
  "outcome": "failed",
  "environment": "local-macos",
  "summary": "Ran python3 -m unittest tests.test_quoting for TC-csvsum-0003: failed (exit 1).",
  "artifacts": [
    ".chough/artifacts/tc-csvsum-0003-before-fix.log"
  ],
  "artifact_fingerprints": {
    ".chough/artifacts/tc-csvsum-0003-before-fix.log": "36a8115e131af617265fa74721004f4802c8e47e146fa280d30b6068bd6a59ad"
  },
  "observed_at_unix_seconds": 1788356400,
  "snapshot": {
    "revision": "${rev1}",
    "dirty": false,
    "fingerprint": "f7c43a8563fbb7638004e04a8b6ad13918abdd42715e6cb078276cb42fa53456",
    "content": "0aacd39c0e3fb39c121b4938a54ada316e7348287824cb4c5a39524b6b4503eb"
  },
  "provenance": "unauthenticated observation; not proof of human approval, test execution, or deployment",
  "cases": [
    {
      "id": "TC-csvsum-0003",
      "digest": "a546b4fec76fd575bdcefd7fef9b245a0831cb65ff6f9596f4db133b8e224c4b"
    }
  ],
  "method": "automated",
  "track": "functional",
  "argv": [
    "python3",
    "-m",
    "unittest",
    "tests.test_quoting"
  ],
  "exit_code": 1,
  "limitations": [
    "expected output missing: \"Ran 3 tests\", \"OK\"",
    "outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected",
    "inherited environment variables were not recorded"
  ],
  "duration_seconds": 0
}
EOF

cat >.chough/bugs/BUG-0001.json <<EOF
{
  "schema_version": 1,
  "id": "BUG-0001",
  "title": "Quoted fields with commas are split into extra columns",
  "symptom": "summarize raises decimal.InvalidOperation for samples/quoted.csv",
  "trigger": "A double-quoted field that contains a comma, such as the amount \"1,250.00\" or the payee \"Rent, August\"",
  "expected": "Each quoted field is one field; samples/quoted.csv totals 1320.50 (SUMMARY-3)",
  "actual": "csvsum/summarize.py splits each line on \",\", so the amount column holds part of the payee and Decimal() fails",
  "criteria": [
    "SUMMARY-3"
  ],
  "requirements": [
    "REQ-csvsum-003"
  ],
  "cases": [
    "TC-csvsum-0003"
  ],
  "environment": "local-macos",
  "revision": "${rev1}",
  "steps": [
    "python3 -m unittest tests.test_quoting",
    "python3 -m csvsum summarize samples/quoted.csv --column amount"
  ],
  "fixture": null,
  "observations": [
    "tc-csvsum-0003-before-fix"
  ],
  "impact": "Any export with a quoted amount or payee cannot be summarized",
  "classification": "product-defect",
  "scope": "blocking",
  "status": "open",
  "regression_baseline": null,
  "duplicate_of": null,
  "related": [],
  "source": "manual report",
  "notes": [
    {
      "at_unix_seconds": 1788356760,
      "text": "TC-csvsum-0003 also had a case binding error in the same run: expect_output wanted \"Ran 3 tests\" but tests/test_quoting.py has 2 tests. That is not part of this defect; it is corrected in .chough/cases/csvsum.cases.toml, not in the code. Both causes must be fixed for the case to pass.",
      "observations": []
    }
  ],
  "created_at_unix_seconds": 1788356700,
  "updated_at_unix_seconds": 1788356760
}
EOF

cat >.chough/transitions.jsonl <<EOF
{"schema_version":1,"at_unix_seconds":1788253800,"kind":"advance","from":"intent","to":"design","reason":"Robin confirmed SUMMARY-1 to SUMMARY-4","gate":null,"revision":"${rev1}","refresh":[]}
{"schema_version":1,"at_unix_seconds":1788253860,"kind":"skip","from":"design","to":"plan","reason":"Command line output only; nothing visual changes","gate":null,"revision":"${rev1}","refresh":[]}
{"schema_version":1,"at_unix_seconds":1788253920,"kind":"advance","from":"plan","to":"build","reason":null,"gate":"ungated:not-configured","revision":"${rev1}","refresh":[]}
{"schema_version":1,"at_unix_seconds":1788355800,"kind":"advance","from":"build","to":"test","reason":null,"gate":"ungated:not-configured","revision":"${rev1}","refresh":[]}
{"schema_version":1,"at_unix_seconds":1788356820,"kind":"back","from":"test","to":"build","reason":"BUG-0001 blocks REQ-csvsum-003","gate":"ungated:not-configured","revision":"${rev1}","refresh":["build: implementation against the current definition","test: rerun affected cases; earlier observations may be stale"]}
EOF

cat >.chough/session.json <<'EOF'
{
  "schema_version": 2,
  "objective": "csvsum: summarize one numeric column of a bank-export CSV (count, total and mean, exact to the cent) with the Python 3 standard library only, for comma and semicolon exports",
  "intent_scope": "SUMMARY",
  "modules": [
    "csvsum"
  ],
  "stage": {
    "current": "build",
    "status": "active",
    "callers": [],
    "iteration": 1
  },
  "native_change": null,
  "references": {
    "criteria": [
      "SUMMARY-1",
      "SUMMARY-2",
      "SUMMARY-3",
      "SUMMARY-4"
    ],
    "requirements": [
      "REQ-csvsum-001",
      "REQ-csvsum-002",
      "REQ-csvsum-003",
      "REQ-csvsum-004"
    ],
    "cases": [
      "TC-csvsum-0001",
      "TC-csvsum-0002",
      "TC-csvsum-0003"
    ],
    "design": [],
    "files": []
  },
  "decisions": [
    "decided: amounts are parsed as decimal.Decimal after removing thousands separators, never as float, so totals match the bank statement to the cent (Robin Ash, 2026-09-01)"
  ],
  "open_questions": [],
  "completed": [
    "REQ-csvsum-001 and REQ-csvsum-002 built: count, total and mean with Decimal amounts."
  ],
  "observations": [
    "tc-csvsum-0003-before-fix"
  ],
  "bugs": [
    "BUG-0001"
  ],
  "blockers": [],
  "awaiting_human": [],
  "run_consents": [],
  "next_action": "Fix BUG-0001 in csvsum/summarize.py, then rerun TC-csvsum-0003."
}
EOF

commit "2026-09-02T14:00:00Z" "Record the failed TC-csvsum-0003 run and BUG-0001"

# ---------------------------------------------------------------------------
# Commit 3 (2026-09-03): the fix (csv.reader) and the corrected case binding.
# ---------------------------------------------------------------------------
cat >csvsum/summarize.py <<'EOF'
"""Summarize one numeric column of a bank-export CSV, exact to the cent."""
import csv
from decimal import ROUND_HALF_EVEN, Decimal

CENT = Decimal("0.01")


def parse_amount(text):
    """An amount such as 1,250.00 as a Decimal; csv.reader has removed any quotes."""
    return Decimal(text.strip().replace(",", ""))


def summarize(lines, column):
    """Count, total and mean of `column` over CSV `lines`, header first."""
    rows = [row for row in csv.reader(lines) if row]
    header, data = rows[0], rows[1:]
    if column not in header:
        raise ValueError(f"no column named {column!r}")
    index = header.index(column)
    amounts = [parse_amount(row[index]) for row in data]
    total = sum(amounts, Decimal("0")).quantize(CENT)
    mean = (total / len(amounts)).quantize(CENT, ROUND_HALF_EVEN) if amounts else Decimal("0.00")
    return {"count": len(amounts), "total": total, "mean": mean}
EOF
write_cases 2

commit "2026-09-03T10:00:00Z" "Parse rows with csv.reader (BUG-0001); TC-csvsum-0003 expects its 2 tests"
rev3=$(git rev-parse HEAD)

# ---------------------------------------------------------------------------
# Commit 4 (2026-09-03): the verified lesson in the module's Lessons section.
# ---------------------------------------------------------------------------
cat >>specs/csvsum/context.md <<EOF

## Lessons

### A failing case can mix a binding error with a real defect (verified)

- Certainty: verified.
- Lesson: before treating a failed case as a product defect, run its bound argv by
  hand and compare the output with every expect_output fragment. Observation
  tc-csvsum-0003-before-fix failed for two reasons at once: a real defect
  (csvsum/summarize.py split each line on ",", so the quoted amount "1,250.00" and
  the payee "Rent, August" became extra fields and the tests errored) and a case
  binding error (TC-csvsum-0003 expected "Ran 3 tests" from tests/test_quoting.py,
  which has 2 tests). Fixing only the code would still have failed the case.
- Rule kept: parse rows only with the standard library csv module (csv.reader, with
  any delimiter passed to it), never by splitting lines by hand; a binding's expected
  test count must equal the number of tests it runs.
- Source: BUG-0001 (verified); failed observation tc-csvsum-0003-before-fix; fix
  commit ${rev3}.
- Evidence: observation tc-csvsum-0003-after-fix, TC-csvsum-0003 passed with the
  corrected binding at the commit that added this lesson.
- Limits: only comma-delimited input with double-quoted fields was tested; other
  delimiters, newlines inside quoted fields and escaped quotes ("") were not.
EOF

commit "2026-09-03T10:30:00Z" "Record the verified lesson from BUG-0001"
rev4=$(git rev-parse HEAD)

# ---------------------------------------------------------------------------
# Commit 5 (2026-09-03): the passing rerun at commit 4 and BUG-0001 verified.
# ---------------------------------------------------------------------------
cat >.chough/artifacts/tc-csvsum-0003-after-fix.log <<'EOF'
--- stdout ---

--- stderr ---
..
----------------------------------------------------------------------
Ran 2 tests in 0.001s

OK
EOF

cat >.chough/evidence/tc-csvsum-0003-after-fix.json <<EOF
{
  "schema_version": 2,
  "id": "tc-csvsum-0003-after-fix",
  "criteria": [
    "SUMMARY-3"
  ],
  "requirements": [
    "REQ-csvsum-003"
  ],
  "contracts": [
    "specs/csvsum/csvsum.spec.md"
  ],
  "kind": "product-test",
  "outcome": "passed",
  "environment": "local-macos",
  "summary": "Ran python3 -m unittest tests.test_quoting for TC-csvsum-0003: passed (exit 0).",
  "artifacts": [
    ".chough/artifacts/tc-csvsum-0003-after-fix.log"
  ],
  "artifact_fingerprints": {
    ".chough/artifacts/tc-csvsum-0003-after-fix.log": "c6d3bba8555b25f42fe1fb1e9156231418c7ba1af81d3c30b59c231ce1021396"
  },
  "observed_at_unix_seconds": 1788432300,
  "snapshot": {
    "revision": "${rev4}",
    "dirty": false,
    "fingerprint": "8c177005e44a00ba8d94201c3093e4ae5f5e4532f1c7118d9d1988e25958a9b5",
    "content": "5a56328c906bfaa0e56828abd24a707f9e023897acd8eb76d1c0fb29badee043"
  },
  "provenance": "unauthenticated observation; not proof of human approval, test execution, or deployment",
  "cases": [
    {
      "id": "TC-csvsum-0003",
      "digest": "8ddc8f2f47d49e27da7d6213114a3979139dcdcb1f66181be77df17c2412c5ff"
    }
  ],
  "method": "automated",
  "track": "functional",
  "argv": [
    "python3",
    "-m",
    "unittest",
    "tests.test_quoting"
  ],
  "exit_code": 0,
  "limitations": [
    "outcome rests on the exit code and the declared output fragments; the command's own assertions are not inspected",
    "inherited environment variables were not recorded"
  ],
  "duration_seconds": 0
}
EOF

cat >.chough/bugs/BUG-0001.json <<EOF
{
  "schema_version": 1,
  "id": "BUG-0001",
  "title": "Quoted fields with commas are split into extra columns",
  "symptom": "summarize raises decimal.InvalidOperation for samples/quoted.csv",
  "trigger": "A double-quoted field that contains a comma, such as the amount \"1,250.00\" or the payee \"Rent, August\"",
  "expected": "Each quoted field is one field; samples/quoted.csv totals 1320.50 (SUMMARY-3)",
  "actual": "csvsum/summarize.py splits each line on \",\", so the amount column holds part of the payee and Decimal() fails",
  "criteria": [
    "SUMMARY-3"
  ],
  "requirements": [
    "REQ-csvsum-003"
  ],
  "cases": [
    "TC-csvsum-0003"
  ],
  "environment": "local-macos",
  "revision": "${rev1}",
  "steps": [
    "python3 -m unittest tests.test_quoting",
    "python3 -m csvsum summarize samples/quoted.csv --column amount"
  ],
  "fixture": null,
  "observations": [
    "tc-csvsum-0003-before-fix",
    "tc-csvsum-0003-after-fix"
  ],
  "impact": "Any export with a quoted amount or payee cannot be summarized",
  "classification": "product-defect",
  "scope": "blocking",
  "status": "verified",
  "regression_baseline": null,
  "duplicate_of": null,
  "related": [],
  "source": "manual report",
  "notes": [
    {
      "at_unix_seconds": 1788356760,
      "text": "TC-csvsum-0003 also had a case binding error in the same run: expect_output wanted \"Ran 3 tests\" but tests/test_quoting.py has 2 tests. That is not part of this defect; it is corrected in .chough/cases/csvsum.cases.toml, not in the code. Both causes must be fixed for the case to pass.",
      "observations": []
    },
    {
      "at_unix_seconds": 1788432600,
      "text": "status: open -> verified. Fixed in csvsum/summarize.py by parsing rows with csv.reader instead of splitting lines on commas; the TC-csvsum-0003 binding now expects its 2 tests. TC-csvsum-0003 passed in tc-csvsum-0003-after-fix. Lesson recorded in specs/csvsum/context.md under Lessons.",
      "observations": [
        "tc-csvsum-0003-after-fix"
      ]
    }
  ],
  "created_at_unix_seconds": 1788356700,
  "updated_at_unix_seconds": 1788432600
}
EOF

cat >.chough/session.json <<'EOF'
{
  "schema_version": 2,
  "objective": "csvsum: summarize one numeric column of a bank-export CSV (count, total and mean, exact to the cent) with the Python 3 standard library only, for comma and semicolon exports",
  "intent_scope": "SUMMARY",
  "modules": [
    "csvsum"
  ],
  "stage": {
    "current": "build",
    "status": "active",
    "callers": [],
    "iteration": 1
  },
  "native_change": null,
  "references": {
    "criteria": [
      "SUMMARY-1",
      "SUMMARY-2",
      "SUMMARY-3",
      "SUMMARY-4"
    ],
    "requirements": [
      "REQ-csvsum-001",
      "REQ-csvsum-002",
      "REQ-csvsum-003",
      "REQ-csvsum-004"
    ],
    "cases": [
      "TC-csvsum-0001",
      "TC-csvsum-0002",
      "TC-csvsum-0003"
    ],
    "design": [],
    "files": []
  },
  "decisions": [
    "decided: amounts are parsed as decimal.Decimal after removing thousands separators, never as float, so totals match the bank statement to the cent (Robin Ash, 2026-09-01)"
  ],
  "open_questions": [],
  "completed": [
    "REQ-csvsum-001 and REQ-csvsum-002 built: count, total and mean with Decimal amounts.",
    "BUG-0001 fixed with csv.reader and verified: TC-csvsum-0003 passed in tc-csvsum-0003-after-fix; lesson recorded in specs/csvsum/context.md."
  ],
  "observations": [
    "tc-csvsum-0003-after-fix"
  ],
  "bugs": [
    "BUG-0001"
  ],
  "blockers": [],
  "awaiting_human": [],
  "run_consents": [],
  "next_action": "Ask Robin how semicolon exports should be recognized, then build REQ-csvsum-004."
}
EOF

commit "2026-09-03T11:00:00Z" "Verify BUG-0001: TC-csvsum-0003 passes after the fix"

# ---------------------------------------------------------------------------
# Commit 6 (2026-09-08): Robin's decision on delimiters and the next task.
# ---------------------------------------------------------------------------
cat >.chough/session.json <<'EOF'
{
  "schema_version": 2,
  "objective": "csvsum: summarize one numeric column of a bank-export CSV (count, total and mean, exact to the cent) with the Python 3 standard library only, for comma and semicolon exports",
  "intent_scope": "SUMMARY",
  "modules": [
    "csvsum"
  ],
  "stage": {
    "current": "build",
    "status": "active",
    "callers": [],
    "iteration": 1
  },
  "native_change": null,
  "references": {
    "criteria": [
      "SUMMARY-1",
      "SUMMARY-2",
      "SUMMARY-3",
      "SUMMARY-4"
    ],
    "requirements": [
      "REQ-csvsum-001",
      "REQ-csvsum-002",
      "REQ-csvsum-003",
      "REQ-csvsum-004"
    ],
    "cases": [
      "TC-csvsum-0001",
      "TC-csvsum-0002",
      "TC-csvsum-0003"
    ],
    "design": [],
    "files": []
  },
  "decisions": [
    "decided: amounts are parsed as decimal.Decimal after removing thousands separators, never as float, so totals match the bank statement to the cent (Robin Ash, 2026-09-01)",
    "rejected: auto-detecting the delimiter with csv.Sniffer because a wrong guess silently gives a wrong total, so the user names the delimiter with --delimiter (Robin Ash, 2026-09-08)"
  ],
  "open_questions": [],
  "completed": [
    "REQ-csvsum-001 and REQ-csvsum-002 built: count, total and mean with Decimal amounts.",
    "BUG-0001 fixed with csv.reader and verified: TC-csvsum-0003 passed in tc-csvsum-0003-after-fix; lesson recorded in specs/csvsum/context.md."
  ],
  "observations": [
    "tc-csvsum-0003-after-fix"
  ],
  "bugs": [
    "BUG-0001"
  ],
  "blockers": [],
  "awaiting_human": [],
  "run_consents": [],
  "next_action": "Build REQ-csvsum-004: add tests/test_delimiter.py and case TC-csvsum-0004 first and see them fail, then add --delimiter to python3 -m csvsum summarize."
}
EOF

commit "2026-09-08T16:00:00Z" "Checkpoint: name the delimiter explicitly; next, REQ-csvsum-004"

printf 'Seeded csvsum at %s (HEAD %s, tree %s)\n' "$(pwd)" \
    "$(git rev-parse HEAD)" "$(git rev-parse 'HEAD^{tree}')"
