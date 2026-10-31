//! Sample shipping fees used by the Chough feature-and-bug exercise.

/// Standard shipping fee in cents.
pub const STANDARD_FEE_CENTS: u64 = 499;

/// Order total in cents from which shipping is free.
pub const FREE_SHIPPING_FROM_CENTS: u64 = 5_000;

/// Shipping fee in cents for an order total in cents.
pub fn shipping_fee(total_cents: u64) -> u64 {
    // Deliberate sample defect: the boundary itself should already ship free.
    if total_cents > FREE_SHIPPING_FROM_CENTS {
        0
    } else {
        STANDARD_FEE_CENTS
    }
}
