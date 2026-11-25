---
change: fix-441
artifact: context
---

# Context

The maintainer confirmed this outcome in the interview: “Human gates: if one of two decisions fails, both gates stay pending (#438)”. The checkpoint records the same wording. FLOW-20 governs grouped replies. REQ-cli-061 already specifies that all named gates close or none do. The bug fix restores that contract. The draft has no scope approval yet.

On Chough 0.6.3 at `0d5988f8`, disposable public CLI runs reproduced a partial close at the 4,096-entry Timeline limit and at the 1,048,576-byte session limit. At each limit the group returned an error after persisting its first close. A small control group closed both gates. These runs used synthetic identities and gates that drive `none`.

The gate lists cover every open session. A reply that names a different session is refused by the current CLI. The proposal preserves that restriction and tests the two session records together. SpecSync remains the authority for native approval and review.
