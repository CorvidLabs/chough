---
id: control-and-echo
state: implementing
type: feature
base_commit: 9a81c859ae449f9da541cab3538d318de411a1ee
---

# Control and echo

## Intent

control and echo

## Affected Canonical Specs

- `records`
- `cli`
- `history`
- `reporting`

## Acceptance Criteria

- A control character (C0, DEL, C1, a bidi override or isolate) in a value a person gives is refused at write as refused:control-character, naming the field and code point, except a line feed where the field takes a line break, with CRLF stored as LF; text printed from a record or given is escaped as \u and four hex digits, text and JSON alike, with a line feed shown as \n on a one-line slot; YAML reserved scalars are written quoted (#366). checkpoint --pr takes only a number or an http(s) URL, an empty value removes it, and next, status, context and the checkpoint view show it; the journal post links only those forms (#353). The remaining error and output sites name the field and what it holds, never the value, proven by a black-box sweep (part of #367). A file: URL is checked like the path it carries, and USERNAME counts like USER and LOGNAME (REQ-records-006).

## No-spec Rationale

Not applicable
