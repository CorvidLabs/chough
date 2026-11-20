---
change: control-and-echo
artifact: plan
---

# Plan

## Approach

One check at the guard every written value passes, one escaping helper used by `print_json` and each text view, the `--pr` forms in `checkpoint`, and `shown` at each remaining echo site; three black-box sweeps prove the write, print and echo rules across commands.

## Out of scope

- The `change.rs` echo sites (G1).
- Rewriting stored records that already hold a control character: they are escaped on print, and a save that does not change them goes through.
- Masking a leak match in output other than a value given on the command line.

## Steps

1. Failing tests: the three sweeps and the per-module tests.
2. The guard, the YAML quoting, and `printable` with `print_json`.
3. The text views, the `PR:` line and the `pr` fields.
4. The `--pr` forms and the journal link.
5. The echo sites.
6. Docs, changelog, cases, verify.

## Risks

- A later change to #407's REQ-records-006 before it merges makes this delta stale; it is then rebuilt on the new text and asked again.
- JSON output bytes change for DEL, C1 and bidi characters, while the parsed values stay equal.
- Telling a stored value from a given one: the guard counts a value as given unless the stored record already holds it, as the redaction count does since #407.

## Constraints consulted

Declare every owning module and one cases file per module; `pub(super)` helpers; no new dependency; a sentence change after approval needs Leif's words.
