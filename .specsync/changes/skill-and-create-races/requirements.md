---
change: skill-and-create-races
artifact: requirements
---

# Requirements

- REQ-cli-016: a CRLF copy counts as the bundle file it equals and is updated with CRLF line breaks (#373); plain `skill status` and `skill update` skip a host folder that cannot be inspected, report it relative to the project and update the rest, exiting 0, while `--host` for it refuses (#403); parallel `skill update` runs all succeed, and a file it cannot write is refused naming it (#386, #373).
- REQ-cli-001: parallel `chough init` runs each succeed or refuse only as a run on its own would; an error names its file (#386).
- REQ-history-016 (added): `chough doctor` reads every record under `.chough/` that Git does not track yet and does not ignore, and reports one its reader refuses as `unreadable` with the file and the reason (#408).
