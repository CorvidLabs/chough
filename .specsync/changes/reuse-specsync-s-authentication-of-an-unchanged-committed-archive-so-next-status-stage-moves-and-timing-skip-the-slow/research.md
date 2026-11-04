---
change: reuse-specsync-s-authentication-of-an-unchanged-committed-archive-so-next-status-stage-moves-and-timing-skip-the-slow
artifact: research
---

# Research

- SpecSync 6.1.2 `validate_archived_integrity_inner` (src/change/consider_accepted_evidence_anchor.rs) checks the accepted-state snapshot, passed verification, the definition approval, contract and execution digests, the acceptance manifest digest, semantic succession and finalization evidence, and reconstructs legacy archives from `.specsync/archive/legacy-baseline.json`. It returns authenticated-history or corrupt-history.
- Porting that check would duplicate SpecSync's lifecycle logic in Chough, which REQ-cli-028 and the hard rule forbid. Replaying SpecSync's own verdict for byte-identical inputs keeps SpecSync the authority.
- `.github/scripts/check-specsync-archive.py` authenticates the same way, by requiring change and summary state archived and validity authenticated-history from `specsync change status <id> --json`.
- Archives without an accepted-state snapshot or an acceptance manifest are judged partly from files outside the archive (legacy reconstruction), so they are never recorded.
