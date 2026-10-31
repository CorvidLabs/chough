# Checkout summary prototype companion

Canonical prototype: `index.html`, `styles.css` and the pinned `tokens.css` in this
folder. Earlier versions are in Git history; there is no other current version.

Status: **awaiting human design approval.** This sample never simulates that
approval; agent review below is not approval.

## Mappings

| Screen or state | HI | Requirement | Case |
|---|---|---|---|
| Below threshold: fee line `$4.99`, hint "Add $7.50 more for free shipping." | CHECKOUT-2 | REQ-checkout-ui-001 | TC-checkout-ui-0001 |
| Free shipping: fee line reads "Free shipping", hint "Your order ships free." | CHECKOUT-1 | REQ-checkout-ui-001 | TC-checkout-ui-0001 |
| Loading, empty cart, error with retry | CHECKOUT-1, CHECKOUT-2 | REQ-checkout-ui-002 | TC-checkout-ui-0002 |
| Submitting: Pay button disabled with "Processing payment…" | CHECKOUT-1 | REQ-checkout-ui-002 | TC-checkout-ui-0002 |

## Decisions

- The fee line says "Free shipping" rather than `$0.00`, because a zero amount reads
  like an error and the human wording is "ships free".
- Desktop uses a cart and summary side by side; below 720 px they stack, summary
  last, so the total stays next to the Pay button.
- Colors come only from the CorvidLabs tokens; the accent marks success, never
  decoration.

## Notes and questions against acceptance criteria

- CHECKOUT-1: should the free-shipping hint also appear in the cart header? Left
  out to keep one message per state; question for the human design review.
- CHECKOUT-2: the "add $X more" hint is a proposed addition, not stated by the
  human; it is easy to remove.

## Iterations

1. First version with six states at two viewports.
2. Rendered review showed the desktop loading state with an empty summary column,
   so the layout would jump when the summary arrived. The loading state now keeps a
   summary skeleton. Found only by looking at the render, not by source checks.

## Rendered review

Recorded by the visual exercise in `docs/exercises/visual-design.md`, with the
screenshots it inspected and the checks it could not perform.
