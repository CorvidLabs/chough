---
change: recognize-pristine-skills-from-pinned-released-bundles-without-replacing-edits
artifact: design
---

# Design

A data-only predecessor catalog correction, not a UI/architecture redesign. file_state continues to compare current bytes first, then an exact (filename,SHA256) predecessor pair; skill update adds missing/current-pristine files and preserves every edited file. Only six catalog rows are added. Standalone tests consume fixed local bytes and cannot execute downloaded scripts or network lookups. Manifest binds5tag commits,24files each and51 exact SHA-named blobs. All production methods, data schemas, host routing and legacy policy stay unchanged.

The complete implementation plan below is deliberately included in this selected artifact so its scope is digest-bound even though the native interview did not select standalone plan.md after architecture_risk=no. Standalone plan.md remains identical supporting preparation, not an inferred completed gate.


# Plan

## Approach

Treat published source bytes as exact predecessor data. Add only the six proven missing filename/SHA256 pairs to KNOWN_PREVIOUS; never change file_state/update/host/legacy policy. Commit the offline five-release manifest and51 deduplicated source-byte blobs with a new standalone runtime test. REQ-cli-016 keeps all existing wording and gains explicit pinned-release coverage/edit-preservation acceptance criteria. No new IDs or public command shapes.

## Out of scope

Future/moving release recognition, blanket overwrite, hash ignore/normalization, byte-newline conversion, replacing edited bytes/modes, changing host routes or legacy removal, init/adopt/migration, other production/test files, inherited lifecycle records, network-executed fixtures, actor/approval policy, release/install/hosted actions.

## Steps

1. Freeze complete preparation/digest and hand sole metadata writer to root for actual scope record.
2. After root receipt and a separate focused build lease, commit exact offline fixtures and discriminating standalone tests; execute real compiled RED on unchanged production. Keep setup/compile failures separate.
3. Add only six KNOWN_PREVIOUS rows; SAME tests GREEN. Exercise all pinned files and edited/mode/name-digest/current/missing/host/legacy boundaries; run existing focused controls/fmt/lint only as leased.
4. Finish actual owned case/evidence/local bug/companion/one-change-log/index records; ordinary own native strict materialization/current freeze, then other-author review/human gates remain separate. No inherited native refresh by this owner.

## Risks

Incomplete predecessor catalogs silently preserve stale pristine content; overly broad matching overwrites real edits. Assert exact path+digest and offline immutable corpus identity. Six-host fixtures are convention/routing checks, not proof that each host discovers the skill. Existing frozen runtime relevant-input equivalence is not a fresh full-main build. New lesson file must preserve old context.md and all inherited sessions; no automatic archive drift may survive.

## Constraints consulted

Actual AGENTS.md and workflow skill; cli contract/REQ-cli-016 and companions; HI FLOW-1/LIFE-4/LIFE-12..14; root external preparation allocation; source archives/tag commits and323-receipt discovery packet; current main4bbd; own inherited typed snapshot. Fledge first, Rust/HI/Fledge/SpecSync pins follow CI only after lease. Other agents own other branches and all original seals remain unchanged.
