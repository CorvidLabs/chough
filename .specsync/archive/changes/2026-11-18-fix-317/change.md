---
id: fix-317
state: archived
type: bug_fix
base_commit: 66543c2bf509627ec2a165414acae0bc8ab59c4a
---

# Fix 317

## Intent

fix 317

## Affected Canonical Specs

- `cli`
- `history`

## Acceptance Criteria

- evidence add, cases run and cases record take the project's writer lock before they read session.md to add their observation reference, and save under it, so an update another writer saves while one of them waits is kept: with the lock held while evidence add waits and the checkpoint's next action changed meanwhile, the change survives the save. bug add and bug update with --observation do the same when they add the bug to the active session, so an update saved while bug add waits survives too. Ten checkpoint --decision and ten evidence add commands run in parallel keep all ten decisions and all ten observation references. cases run still runs its case without holding the lock.

## No-spec Rationale

REQ-cli-040 already requires evidence add, cases record, bug add and bug update to hold the writer lock while they read what they change, and a command that first runs a case to re-read under the lock for its save; only the evidence save, the bug commands' session save and their tests change.
