---
id: accept-dotted-specsync-module-ids-and-hi-sub-criterion-ids-keep-bundled-skill-prose-free-of-em-and-en-dashes-and-remove
state: implementing
type: bug_fix
base_commit: d673efc9fadd00953cf9e42dad96e08f3c76103c
---

# Accept dotted SpecSync module IDs and HI sub-criterion IDs, keep bundled skill prose free of em and en dashes, and remove a mistaken completed entry (DRIVE-1, DRIVE-7, DRIVE-8; issues 56 and 57)

## Intent

Accept dotted SpecSync module IDs and HI sub-criterion IDs, keep bundled skill prose free of em and en dashes, and remove a mistaken completed entry (DRIVE-1, DRIVE-7, DRIVE-8; issues 56 and 57)

## Affected Canonical Specs

- `cli`

## Acceptance Criteria

- chough init, checkpoint, next, status, context and timing accept and keep SpecSync module IDs with dots such as chat.service, room.service, linked-wallet.service and identity-wipe.service, cases files and case IDs for such a module are read and checked, and requirement links such as REQ-chat.service-1 are kept; checkpoint --criterion and bug add --criterion accept HI sub-criterion IDs such as UX-12.a, CHAT-34.a and SEND-1.a.1.b; IDs SpecSync or HI refuse, such as a/b, .., CON, trailing., UX-012, ux-1, UX-1.A and UX-1..a, are still refused with nothing saved; a test fails if any bundled skill file contains U+2013 or U+2014; checkpoint --remove-completed removes one exact completed entry and refuses an entry that is not there with nothing saved; the new tests fail on main 8298e5a where they cover changed behavior; docs/cli.md states the accepted forms and the flag; fledge lanes run verify passes

## No-spec Rationale

ID acceptance within REQ-cli traceability and checkpoint requirements as written: the requirements already defer criterion IDs to HI and module and requirement IDs to SpecSync, so accepting exactly the IDs those tools accept changes no requirement or criterion text; removing a mistaken completed entry is a checkpoint update under the existing canonical checkpoint requirement; docs/cli.md states the accepted forms and the new flag
