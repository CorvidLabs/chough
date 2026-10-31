//! Sample shipping fees used by the Chough feature-and-bug exercise.

/// Standard shipping fee in cents.
pub const STANDARD_FEE_CENTS: u64 = 499;

/// Shipping fee in cents for an order total in cents.
pub fn shipping_fee(_total_cents: u64) -> u64 {
    STANDARD_FEE_CENTS
}
