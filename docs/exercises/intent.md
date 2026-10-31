# Exercise: intent stage on the shipping sample

Requirement links: FLOW-3, LIFE-1. A worked example produced by the agent following
`skills/chough-workflow/references/intent.md`, with role passes run sequentially
(no subagents). The "human" statement is sample input written for the exercise; this
record shows the procedure, not a real person's decision.

## Wording kept apart

| Kind | Text |
|---|---|
| Direct statement (sample input) | "Orders of fifty dollars or more should ship free, and everyone else pays the normal fee." |
| Interpretation (agent) | The threshold is inclusive: an order of exactly 50.00 ships free. |
| Proposed addition (agent) | Show how much more is needed for free shipping below the threshold. |
| Unresolved choice | Does the threshold use the subtotal before or after discounts? |

Captured criteria (sample `hi/checkout.md`): CHECKOUT-1 "An order of fifty dollars or
more ships free." and CHECKOUT-2 "An order under fifty dollars pays the standard
shipping fee." `hi export CHECKOUT` was read first; no ID was reused.

## FRD (from templates/frd.md)

| Requirement | Summary | HI | Native |
|---|---|---|---|
| FR-1 | Zero fee from 5000 cents inclusive | CHECKOUT-1 | REQ-shipping-002 (added by the native delta) |
| FR-2 | Standard fee below 5000 cents | CHECKOUT-2 | REQ-shipping-001 (existing) |

Out: discounts, regional fees, checkout wording. Assumption: amounts are integer
cents; if fractional currency appears, FR-1 needs a rounding rule.

## Role passes (from templates/review-findings.md)

| # | Role | Reference | Finding | Resolution |
|---|---|---|---|---|
| 1 | QA | CHECKOUT-1, FR-1 | "Fifty dollars or more" is untestable without a case at exactly 5000 | Added TC-shipping-0001 at the boundary |
| 2 | Product | Unresolved choice | Subtotal before or after discounts changes who ships free | Human decision |
| 3 | Engineering | FR-1 | Floating-point dollars risk boundary errors | Integer cents, within scope |
| 4 | Accessibility | — | No interface in this change | Discarded; no concrete reference |

## One decision for the human

"Approve the checkout intent as CHECKOUT-1 and CHECKOUT-2 with an inclusive
threshold, measured on the subtotal before discounts unless you say otherwise?"
One question for the whole stage; there is no per-criterion approval.

## In this repository

The Chough evolution itself was approved as one coherent decision covering both
intent (FLOW-1 to FLOW-13) and the native scope, because the brief already settled
the product direction and asked for a single scope approval. That combined decision
is recorded natively as approval of `evolve-chough-into-a-resumable-stage-workflow`.
