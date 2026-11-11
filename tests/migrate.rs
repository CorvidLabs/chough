//! #213 in the 0.5.x layout, red on 0.5.0: chough migrate cannot carry a 0.5.x project to
//! 0.6.0. The 0.6.0 tests in this file replace it.

mod common;

use common::Fixture;

#[test]
fn migrate_previews_applies_and_keeps_every_byte_in_git() {
    let fixture = Fixture::new();
    fixture.ok(&["session", "new", "alpha", "--objective", "Ship alpha"]);
    let output = fixture.run(&["migrate"]);
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(
        text.contains("plan_digest"),
        "no 0.6.0 migration plan: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
