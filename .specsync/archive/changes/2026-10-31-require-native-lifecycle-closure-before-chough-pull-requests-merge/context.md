---
change: require-native-lifecycle-closure-before-chough-pull-requests-merge
artifact: context
---

# Context

The human requested complete, useful Chough GitHub Actions checks, specifically a SpecSync Archive check that prevents merging any in-progress native change, including a scope-review draft. Product tests alone cannot establish lifecycle closure. The human explicitly authorized implementing this separate CI change and reviewable draft PR while the runtime review stays read-only. When drafted, the committed policy was optional; PR #2 has since enabled it on main (`enabled` and `require_change_for_meaningful_files` true). Implementation proceeds within the original authorization without changing that policy or inventing an approval record. This execution request does not authenticate human approval of this native definition digest.

Drafted from main `57aa169588522ef44438236b9f03f409dc1254c2` (the recorded base_commit), independently of the then-draft runtime PR #2. On 2026-10-31 PR #2 and PR #4 merged, and this branch was rebased onto main `2d782c4`; main has since advanced to `f19823d` with a checkpoint-only commit. No runtime source, approval, review, or archive is changed here.

The current workflow runs Rust tests and release builds on Ubuntu, macOS, and Windows. Its native job runs Fledge, HI, strict 100% SpecSync structural coverage, formatting, and Clippy. It has no archive check. Main's SDD policy is now enabled, so `specsync change audit` (run by the Fledge job) refuses this change's meaningful paths until its definition is approved: both `Fledge / HI / SpecSync` and `SpecSync Archive` are expected red while it is draft.

The merged runtime work added a native audit task and full-history checkout. The rebase kept both (Fledge `audit`, `cases`, `exercise` tasks; `fetch-depth: 0`) alongside this change's pinned actions. Native audit remains different from archive closure; both stay.

Applicable intent: LIFE-6 keeps native tools authoritative; LIFE-7 distinguishes structural tests, product verification, human acceptance, and delivery. Applicable repository policy: later scoped changes use the full native lifecycle with genuine human decisions, and agents must not create human approval/review records from actor labels.

This package records the concrete technical scope. Implementation and isolated verification proceed under the explicit CI request and current optional policy. Genuine native approval/review/finalization and merge are not claimed. No approval record is present; Archive remains expected red while this scope is active.
