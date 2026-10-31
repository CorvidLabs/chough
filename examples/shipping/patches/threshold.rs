use shipping::{FREE_SHIPPING_FROM_CENTS, STANDARD_FEE_CENTS, shipping_fee};

#[test]
fn free_shipping_at_threshold() {
    assert_eq!(shipping_fee(FREE_SHIPPING_FROM_CENTS), 0);
}

#[test]
fn standard_fee_below_threshold() {
    assert_eq!(shipping_fee(FREE_SHIPPING_FROM_CENTS - 1), STANDARD_FEE_CENTS);
}

#[test]
fn free_shipping_above_threshold() {
    assert_eq!(shipping_fee(FREE_SHIPPING_FROM_CENTS + 1), 0);
}
