---
change: control-and-echo
artifact: requirements
---

# Requirements

- REQ-records-016 (added): control characters (C0, DEL, C1, bidi overrides and isolates) are refused at write in every checked field, a line feed and a tab only where the field takes a line break and CRLF stored as LF; text printed from a record or given is escaped as `\u` and four hex digits, text and JSON alike, a line feed and a tab as `\n` and `\t` on a one-line slot; YAML's reserved scalars, `yes`, `no`, `on` and `off` among them, are written quoted (#366).
- REQ-records-006: a `file:` URL is checked like the path it carries; `USERNAME` counts as `USER` and `LOGNAME` do; output that repeats a value given on the command line follows the error rule; a sweep proves no echo (#367).
- REQ-cli-007: `--pr` takes only a pull request number, `#12` stored as `12`, or an `http(s)` URL, an empty value removes it, and `next`, `status`, `context` and the checkpoint view show it; the journal post links only those forms (#353).
- REQ-history-015: `correct` names a given entry only as REQ-records-006 allows (#367).
- REQ-reporting-001: `timing` prints stored text as REQ-records-016 says (#366).
