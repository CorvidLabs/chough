---
id: let-chough-drive-native-tools-and-reuse-existing-human-authorization
state: implementing
type: feature
base_commit: 825a3805530456005e361dfa6d96d93eee52cd09
---

# Let Chough drive native tools and reuse existing human authorization

## Intent

Let Chough drive native tools and reuse existing human authorization

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- The current agent executes HI, SpecSync and Fledge internally, records concrete native scope from applicable explicit prior or standing human authorization with its source and limits, and asks no repeated tool-specific approval; ordinary keep-working instructions and agent labels alone still cannot create human authority; actual checks, task ordering, separate implementation acceptance and delivery boundaries remain authoritative; the user-facing README explains goal/authorization to PR flow; exact shipped predecessor copies upgrade safely and edited copies are preserved.

## No-spec Rationale

Not applicable
