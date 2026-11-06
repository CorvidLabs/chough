---
change: support-macos-and-linux-with-windows-through-wsl-and-drop-the-native-windows-ci-job-flow-17
artifact: context
---

# Context

## The decision

Leif, in the orc tab on 2026-11-05 at 8:17 PM MST, chose "Mac + Linux; Windows via WSL", relayed by orc (hc t18). The context he chose from, as orc relayed it: match SpecSync 6.0 (no native Windows); drop native Windows CI from Chough; document and test WSL; keep tests CRLF-safe; Raven Desktop keeps Windows (not this repo). It is captured as FLOW-17. It was first drafted as FLOW-16 and renumbered before any push, because the branch of chough#123 (ch-d7) holds FLOW-16; orc's ID registry assigns FLOW-16 to ch-d7. Orc asked for this change to merge early because other Chough pull requests wait on the Windows job.

## What exists today

- `.github/workflows/ci.yml` runs `Product / windows-latest` (cargo test and a release build) beside Ubuntu and macOS. Branch protection on main requires no status check (read 2026-11-05: empty contexts and checks, rulesets `[]`), so removing the job blocks no merge.
- README "Tested platforms" lists "Hosted CI: Ubuntu, macOS, Windows", and Known limitations has a Windows `taskkill` bullet. docs/ci.md says CI builds on Windows and recommends `Product / windows-latest` as a required check. specs/cli/testing.md says Windows CI exercises `/`-separated keys and CRLF front matter.
- SpecSync 6.2.0 publishes Linux and macOS assets only, and its README says "Windows is not a supported target as of 6.0" and points to WSL.

## Requirements that name Windows

No HI criterion and no requirement promises that Chough runs on native Windows. The only requirement text that names Windows is REQ-cli-046 (#109): attachment folders are short and paths stop at 220 characters "so attachment paths stay short enough to check out on Windows". That is a rule about the tracked tree a repository holds, which any Git client may clone, so it stays, and so does its file-mode rule (a Unix guarantee). Module IDs that refuse Windows reserved names and `<>:"|?*`, and case env names that are equal ignoring ASCII case, stay for the same reason: they keep files portable and match what SpecSync accepts.

## Approval

Leif: "Approve, A on D1-D5", relayed by orc (hc t18) on 2026-11-05, answering the intent and scope gate for this change while it was still numbered FLOW-16. D1 to D5 are the five choices in that gate: no requirement delta; keep the `cfg(windows)` code, documented as unsupported and untested; the Ubuntu jobs cover WSL 2, with no WSL runner; close no issues; leave the `ci-windows` labels on the earlier case definitions.

## Issues

No open issue is about Windows only. #30 mentions a Windows `\\?\` path failure that was fixed long ago; its subject (parallel-safe identities) stays open. #109 is a merged pull request whose Windows items were fixed by the short-folder rule, which stays. Nothing is proposed for closing.
