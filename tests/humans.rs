//! #236 in the 0.5.x layout: closing a scope gate with a human's relayed decision runs
//! no native approval as that human. Red on 0.5.0; the 0.6.0 tests of the same names
//! replace these (REQ-records-008).

mod common;

use common::Fixture;

#[cfg(unix)]
#[test]
fn a_scope_gate_runs_the_approval_once_as_the_configured_human() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "human-change"]);
    fixture.write(
        ".chough/local/identity.toml",
        "[[human]]\nname = \"SYNTHETIC Person\"\naliases = [\"SYNTHETIC-human\"]\nspecsync_actor = \"SYNTHETIC-actor\"\n",
    );
    fixture.ok(&["checkpoint", "--awaiting-human", "Scope approval"]);
    fixture.write(".stub-count", "");
    fixture.ok(&[
        "checkpoint",
        "--close-gate",
        "Scope approval",
        "--decided-by",
        "SYNTHETIC-human",
        "--words",
        "yes, go",
        "--where",
        "SYNTHETIC chat",
        "--recorded-by",
        "SYNTHETIC-agent",
    ]);
    let calls = std::fs::read_to_string(fixture.root().join(".stub-calls")).unwrap_or_default();
    assert!(
        calls
            .lines()
            .any(|call| call.starts_with("change approve human-change --actor SYNTHETIC-actor")),
        "no approval ran as the configured human; SpecSync calls: {calls:?}"
    );
}
