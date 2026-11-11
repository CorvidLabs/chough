//! #167 in the 0.5.x layout: there is no append-only correction. Red on 0.5.0; the
//! 0.6.0 tests in this file replace it (REQ-history-015).

mod common;

use common::Fixture;

#[test]
fn a_correction_appends_one_linked_entry_and_changes_no_earlier_byte() {
    let fixture = Fixture::new();
    fixture.init();
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "reset", "--to", "intent", "--reason", "Start the paging work"]);
    let output = fixture.run(&["correct", "^t-000001", "--reason", "Start the search work"]);
    assert!(
        output.status.success(),
        "no correction can be recorded: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
