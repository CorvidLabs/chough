---
id: make-native-workflow-stage-references-and-installed-copies-consistent
state: implementing
type: documentation
base_commit: 7cd739219ff667dcf6d153a6f439d6c3661b1fce
---

# Make native workflow stage references and installed copies consistent

## Intent

Make native workflow stage references and installed copies consistent

## Affected Canonical Specs

- None

## Acceptance Criteria

- Plan applies existing explicit authority before asking any missing decision, native writer examples use the acting agent account with actual human authority as source, accept preserves the actual implementation acceptance boundary, managed repository host copies match canonical guidance without overwriting edits, and cross-bundle actor/copy consistency plus safe predecessor upgrades are tested.

## No-spec Rationale

Complete the existing REQ-cli-029 agent-operated authority guidance across plan/accept references and installed host copies; no additional canonical requirement, helper command or lifecycle state is introduced. The primary workflow change owns its contract and helper/test support.
