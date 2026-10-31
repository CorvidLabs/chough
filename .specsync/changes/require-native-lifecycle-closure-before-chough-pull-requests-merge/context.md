---
change: require-native-lifecycle-closure-before-chough-pull-requests-merge
artifact: context
---

# Context

The human requested complete, useful Chough GitHub Actions checks, specifically a SpecSync Archive check that prevents merging any in-progress native change, including a scope-review draft. Product tests alone cannot establish lifecycle closure. The human explicitly authorized implementing this separate CI change and reviewable draft PR while the runtime review stays read-only. The current committed policy is optional; implementation proceeds within that authorization without enabling policy or inventing an approval record. This execution request does not authenticate human approval of this native definition digest.

Remote main was verified as `57aa169588522ef44438236b9f03f409dc1254c2` on 2026-10-31. This branch starts there and is independent of draft runtime PR #2 (`feat/resumable-stage-workflow`). No Claude checkout, runtime source, PR branch, approval, review, or archive is changed here.

The current workflow runs Rust tests and release builds on Ubuntu, macOS, and Windows. Its native job runs Fledge, HI, strict 100% SpecSync structural coverage, formatting, and Clippy. It has no archive check. Main's SDD policy is explicitly disabled by bootstrap policy, and its version metadata is historical; do not represent bootstrap as an approved native lifecycle.

The unpublished runtime work has independently added a native audit task and full-history checkout. Native audit remains different from archive closure, so eventual integration must preserve both without duplicating incompatible workflow edits.

Applicable intent: LIFE-6 keeps native tools authoritative; LIFE-7 distinguishes structural tests, product verification, human acceptance, and delivery. Applicable repository policy: later scoped changes use the full native lifecycle with genuine human decisions, and agents must not create human approval/review records from actor labels.

This package records the concrete technical scope. Implementation and isolated verification proceed under the explicit CI request and current optional policy. Genuine native approval/review/finalization and merge are not claimed. No approval record is present; Archive remains expected red while this scope is active.
