//! #237 in the 0.5.x layout: a gate opened for a later stage still holds every move.
//! Red on 0.5.0; the 0.6.0 test of the same name replaces it (REQ-cli-030).

mod common;

use common::Fixture;

/// #237: a gate meant for release, opened at build, must not stop build, test or
/// accept from advancing. On 0.5.0 every open gate refuses every advance.
#[test]
fn a_gate_holds_only_the_stage_it_names() {
    let fixture = Fixture::new();
    fixture.init();
    for _ in 0..3 {
        fixture.ok(&["stage", "advance"]);
    }
    fixture.ok(&["checkpoint", "--awaiting-human", "Publish the release"]);
    let output = fixture.run(&["stage", "advance"]);
    assert!(
        output.status.success(),
        "a gate for a later stage stopped build from advancing: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
