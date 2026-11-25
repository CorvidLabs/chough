---
id: chough-change-approve-relays-a-human-s-approval-of-a-change-definition-to-specsync-with-a-required-citation-req-cli-071
state: verifying
type: feature
base_commit: 61479feb1e88a57bc45716ff4e42ce84fb2b463b
---

# Chough change approve relays a human's approval of a change definition to SpecSync with a required citation (REQ-cli-071, DRIVE-20)

## Intent

chough change approve relays a human's approval of a change definition to SpecSync with a required citation (REQ-cli-071, DRIVE-20)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough change approve refuses before SpecSync runs without --actor, --words or --where, or with one of them blank. With all three, it runs specsync change approve <id> --actor <actor> --note <note>, where the note carries relayed=pi chat, decided-by, words, where and recorded-by joined by '; ', and --note appends its value; without --recorded-by the note names chough <version>. SpecSync's standard output, standard error and exit code pass through unchanged.

## No-spec Rationale

Not applicable
