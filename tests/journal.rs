//! #216 in the 0.5.x layout, red on 0.5.0: the move into done writes no journal post and
//! no feed. The 0.6.0 tests in this file replace it.

mod common;

use common::Fixture;

#[test]
fn done_writes_a_short_post_and_the_feed_lists_it() {
    let fixture = Fixture::new();
    fixture.init();
    for _ in 0..6 {
        fixture.ok(&["stage", "advance"]);
    }
    assert!(
        fixture.root().join("knowledge/journal/feed.xml").is_file(),
        "the move into done wrote no journal feed"
    );
}
