# Rendered review note: checkout summary prototype

Reviewer: Claude (Opus 5.5), a vision-capable agent. This is not a human review and
not design approval.

Inspected renders (headless Chromium, 1x): desktop 1280x800 below-threshold, free
shipping and loading (before and after iteration 2); mobile 390x844 below-threshold,
error, submitting and empty. The other renders were covered only by the deterministic
checks of TC-checkout-ui-0001. Non-loading states were inspected at iteration 1;
iteration 2 only added a summary skeleton that is hidden outside the loading state,
and the deterministic checks were rerun on iteration 2.

1. Pass. Fee line reads $4.99 with "Add $7.50 more for free shipping." below the
   threshold, and "Free shipping" with "Your order ships free." at $50.00, on both widths.
2. Pass. Loading shows skeletons only (iteration 2 keeps the summary column), empty
   shows one message and a link, error shows one message with Try again, submitting
   disables Pay with "Processing payment…".
3. Pass. At 390 px the summary stacks under the cart with the total directly above Pay.
4. Pass. The accent appears only on the free-shipping fee and hint; the error border
   uses the danger token.

Finding fixed during review: at iteration 1 the desktop loading state left the summary
column empty, which would shift the layout when the summary loaded.

Limits: brand fonts (Schibsted Grotesk, Spline Sans Mono) were not loaded, so system
fallbacks rendered; keyboard focus order and screen-reader output were not exercised;
no dark-mode render; the human copy review TC-checkout-ui-0003 was not performed.
