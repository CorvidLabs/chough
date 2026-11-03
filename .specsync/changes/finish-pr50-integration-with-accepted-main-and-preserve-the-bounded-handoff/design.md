# Integration design

## Approach

No visual or interaction redesign. Merge accepted CLI fixes with accepted report Reads, identity test hooks and bounded two-worker archive execution. Preserve current main canonical25 and all parent requirement bodies.

## Data and ownership

The only live Chough checkpoint remains .chough/session.json. The indexed artifact is immutable historical provenance, with exact raw inputs, literal old-note texts, indices and hashes; it is not another current ledger. SpecSync remains the only lifecycle owner. Select one coherent current-cycle group before the helper merges ordered collections.

## Bounds and safety

Keep65536bytes and200observation IDs. Preserve all existing IDs and24record pins. New observations use exact record-file pins; this protects record and artifacts without changing their revision, source identity, result, method or freshness. All other list/output/history bounds remain unchanged.

## Alternatives

Raw union is rejected because it resurrects24intentional removals. Decision-only cleanup is rejected because it leaves insufficient headroom. Capacity raise is a separate PR51 choice. Old evidence relabelling and archive edits are rejected.

## Constraints consulted

Accepted contracts and source, direct-pin protection, independent readonly prototype58567bytes and the current source-content fingerprint rules.
