// The harness's own acceptance test for the host-matrix sample (REQ-cli-066). The
// harness copies it into a clone of the finished sample and runs it, so a host's
// build is judged by behaviour it did not write. Fees are in cents (CHECKOUT-1, CHECKOUT-2).
use shipping::shipping_fee;

#[test]
fn under_fifty_dollars_pays_the_standard_fee() {
    assert_eq!(shipping_fee(4_999), 499);
}

#[test]
fn exactly_fifty_dollars_ships_free() {
    assert_eq!(shipping_fee(5_000), 0);
}

#[test]
fn over_fifty_dollars_ships_free() {
    assert_eq!(shipping_fee(5_001), 0);
}
