# Shipping requirements

### REQ-shipping-001

The module SHALL charge the standard fee for an order under the free-shipping threshold. Trace: CHECKOUT-2.

Acceptance Criteria
- tests/threshold.rs asserts the standard fee just below the threshold.
