//! Authenticated archive reuse and batched stage advance (REQ-cli-035).

mod common;

use common::Fixture;
use serde_json::Value;
use std::fs;

const ID: &str = "old-change";
const ARCHIVE: &str = ".specsync/archive/changes/2026-11-01-old-change";

fn transitions(fixture: &Fixture) -> usize {
    fixture.read(".chough/transitions.jsonl").lines().count()
}

fn stage(fixture: &Fixture) -> String {
    fixture.session()["stage"]["current"]
        .as_str()
        .unwrap()
        .to_string()
}

fn cache_entries(fixture: &Fixture) -> usize {
    fs::read_dir(fixture.root().join(".git/chough/archive-auth"))
        .map_or(0, |entries| entries.count())
}

/// An archived native change whose committed archive holds an accepted-state snapshot
/// and an acceptance manifest, in a Git repository, at stage test, with calls counted.
#[cfg(unix)]
fn archived() -> Fixture {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("archived");
    fixture.init();
    fixture.write(
        &format!("{ARCHIVE}/state.json"),
        "{\"id\":\"old-change\",\"state\":\"archived\"}\n",
    );
    fixture.write(
        &format!("{ARCHIVE}/accepted-state.json"),
        "{\"id\":\"old-change\",\"state\":\"accepted\"}\n",
    );
    fixture.write(
        &format!("{ARCHIVE}/verification.json"),
        "{\"passed\":true,\"acceptance_manifest\":{\"schema_version\":1,\"entries\":[]}}\n",
    );
    fixture.ok(&["checkpoint", "--native-change", ID]);
    fixture.commit("archive");
    fixture.write(".stub-count", "");
    fixture
}

#[cfg(unix)]
fn reset_to_test(fixture: &Fixture) -> Value {
    fixture.ok(&[
        "stage",
        "reset",
        "--to",
        "test",
        "--reason",
        "Archived change",
    ])
}

#[cfg(unix)]
#[test]
fn an_authenticated_archive_is_read_once_and_reused_while_unchanged() {
    let fixture = archived();
    reset_to_test(&fixture);
    assert_eq!(fixture.native_calls(), 1);
    let log = fixture.read(".chough/transitions.jsonl");
    assert!(log.contains("\"native:archived\""), "{log}");
    assert_eq!(
        cache_entries(&fixture),
        1,
        "the authenticated read was not recorded"
    );

    let next = fixture.ok(&["next", "--json"]);
    assert_eq!(
        fixture.native_calls(),
        0,
        "next re-read an unchanged archive"
    );
    assert_eq!(
        next["gate"]["label"], "native:archived-authenticated-cached",
        "{next:#}"
    );
    let detail = next["gate"]["detail"].as_str().unwrap();
    assert!(detail.contains("authenticated-history"), "{detail}");
    assert!(detail.contains("specsync"), "{detail}");
    assert!(detail.contains("sha256:"), "{detail}");

    let status = fixture.ok(&["status", "--json"]);
    assert_eq!(
        fixture.native_calls(),
        0,
        "status re-read an unchanged archive"
    );
    assert_eq!(
        status["checkpoint"]["gate"]["label"],
        "native:archived-authenticated-cached"
    );

    let timing = fixture.ok(&["timing", "--json"]);
    assert_eq!(
        fixture.native_calls(),
        0,
        "timing re-read an unchanged archive"
    );
    assert!(timing.to_string().contains("cached"), "{timing:#}");

    let advanced = fixture.ok(&["stage", "advance"]);
    assert_eq!(
        fixture.native_calls(),
        0,
        "a stage move re-read an unchanged archive"
    );
    assert_eq!(
        advanced["applied"]["gate"],
        "native:archived-authenticated-cached"
    );
    assert_eq!(stage(&fixture), "accept");
    assert!(
        fixture
            .read(".chough/transitions.jsonl")
            .contains("native:archived-authenticated-cached")
    );
}

#[cfg(unix)]
#[test]
fn any_changed_input_reads_live_and_a_refused_read_still_refuses() {
    let fixture = archived();
    reset_to_test(&fixture);
    fixture.native_calls();
    let live = |fixture: &Fixture, why: &str| {
        let next = fixture.ok(&["next", "--json"]);
        assert_eq!(fixture.native_calls(), 1, "{why}: no live read");
        assert_eq!(next["gate"]["label"], "native:archived", "{why}: {next:#}");
    };

    // An archive byte that differs from the authenticated one, committed or not.
    fixture.write(
        &format!("{ARCHIVE}/state.json"),
        "{\"id\":\"old-change\",\"state\":\"archived\" }\n",
    );
    live(&fixture, "uncommitted archive change");
    live(&fixture, "uncommitted archive change, second read");
    fixture.commit("edit archive");
    live(&fixture, "changed archive bytes");
    fixture.ok(&["next", "--json"]);
    assert_eq!(
        fixture.native_calls(),
        0,
        "the new authentication was not recorded"
    );

    // The native policy, the baseline and the SpecSync binary are part of the key.
    fixture.write(".specsync/sdd.json", "{\"enabled\": true }\n");
    live(&fixture, "changed policy");
    fixture.write(".specsync/archive/legacy-baseline.json", "{}\n");
    live(&fixture, "new legacy baseline");
    let stub = fixture.read("bin/specsync");
    fixture.write("bin/specsync", &format!("{stub}# changed\n"));
    fixture.executable("bin/specsync");
    live(&fixture, "changed SpecSync binary");

    // A corrupt or mismatched entry is ignored.
    let directory = fixture.root().join(".git/chough/archive-auth");
    for entry in fs::read_dir(&directory).unwrap() {
        fs::write(entry.unwrap().path(), "{not json").unwrap();
    }
    live(&fixture, "corrupt entry");
    for entry in fs::read_dir(&directory).unwrap() {
        let path = entry.unwrap().path();
        let mut record: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        record["digest"] = Value::from("sha256:0000");
        fs::write(&path, record.to_string()).unwrap();
    }
    live(&fixture, "mismatched entry");
    fixture.ok(&["next", "--json"]);
    fixture.native_calls();

    // An active workspace of the same ID is never served from the archive.
    fixture.write(&format!(".specsync/changes/{ID}/state.json"), "{}\n");
    live(&fixture, "active workspace");
    live(&fixture, "active workspace, second read");
    fs::remove_dir_all(fixture.root().join(".specsync/changes")).unwrap();
    fixture.ok(&["next", "--json"]);
    fixture.native_calls();

    // A commit that is no longer reachable from HEAD.
    fixture.git(&["checkout", "-q", "--orphan", "rewritten"]);
    fixture.commit("rewritten history");
    live(&fixture, "unreachable recorded commit");
    fixture.native_calls();

    // A failed live read is still refused, never served from a stale entry.
    fixture.write(&format!("{ARCHIVE}/extra.json"), "{}\n");
    fixture.commit("another archive byte");
    fixture.write(".stub-fail", "");
    let refused = fixture.fails(&["stage", "advance"]);
    assert!(refused.contains("refused:native-unknown"), "{refused}");
}

#[cfg(unix)]
#[test]
fn only_self_contained_authenticated_archives_are_recorded() {
    for (setup, why) in [
        (".stub-validity", "corrupt-history"),
        (".stub-validity", "exact"),
        (".stub-state", "verifying"),
        ("snapshot", "no accepted-state snapshot"),
        ("manifest", "no acceptance manifest"),
        ("nogit", "no Git directory"),
    ] {
        let fixture = archived();
        match setup {
            "snapshot" => {
                fs::remove_file(
                    fixture
                        .root()
                        .join(format!("{ARCHIVE}/accepted-state.json")),
                )
                .unwrap();
                fixture.commit("legacy");
            }
            "manifest" => {
                fixture.write(
                    &format!("{ARCHIVE}/verification.json"),
                    "{\"passed\":true}\n",
                );
                fixture.commit("legacy");
            }
            "nogit" => fs::remove_dir_all(fixture.root().join(".git")).unwrap(),
            file => fixture.write(file, &format!("{why}\n")),
        }
        for _ in 0..2 {
            fixture.ok(&["next", "--json"]);
            assert_eq!(fixture.native_calls(), 1, "{why}: not read live");
        }
        assert_eq!(cache_entries(&fixture), 0, "{why}: recorded");
    }
}

#[cfg(unix)]
#[test]
fn a_batched_advance_reads_native_status_once() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    fixture.no_spec_approval("add-thing");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    assert_eq!(stage(&fixture), "plan");
    fixture.write(".stub-count", "");
    let before = transitions(&fixture);
    let batch = fixture.ok(&["stage", "advance", "--to", "accept", "--note", "Batch"]);
    assert_eq!(fixture.native_calls(), 1, "{batch:#}");
    assert_eq!(stage(&fixture), "accept");
    assert_eq!(transitions(&fixture), before + 3);
    let steps = batch["applied"]["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 3);
    assert_eq!(steps[0]["to"], "build");
    assert_eq!(steps[0]["gate"], "native:approved");
    assert_eq!(steps[2]["to"], "accept");

    // The target must lie ahead on the main path.
    assert!(
        fixture
            .fails(&["stage", "advance", "--to", "plan"])
            .contains("ahead")
    );
    assert!(
        fixture
            .fails(&["stage", "advance", "--to", "explore"])
            .contains("main stage")
    );
    fixture.native_calls();
}

#[cfg(unix)]
#[test]
fn a_refused_step_refuses_the_whole_batch_with_nothing_recorded() {
    let fixture = Fixture::new();
    fixture.stubs();
    fixture.native("approved");
    fixture.init();
    fixture.ok(&["checkpoint", "--native-change", "add-thing"]);
    fixture.no_spec_approval("add-thing");
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&["stage", "advance"]);
    fixture.ok(&[
        "bug",
        "add",
        "--title",
        "Totals are wrong",
        "--symptom",
        "Total is off by one",
        "--trigger",
        "Any order",
        "--expected",
        "Exact total",
        "--actual",
        "Total plus one",
        "--environment",
        "test-local",
        "--requirement",
        "REQ-cli-001",
        "--scope",
        "blocking",
    ]);
    fixture.write(".stub-count", "");
    let session = fixture.read(".chough/session.json");
    let log = fixture.read(".chough/transitions.jsonl");
    let refused = fixture.fails(&["stage", "advance", "--to", "accept"]);
    assert!(refused.contains("refused:blocking-bugs"), "{refused}");
    assert_eq!(fixture.native_calls(), 1);
    assert_eq!(fixture.read(".chough/session.json"), session);
    assert_eq!(fixture.read(".chough/transitions.jsonl"), log);

    // A refused native gate on the first step refuses too.
    fixture.write(".stub-state", "draft\n");
    let refused = fixture.fails(&["stage", "advance", "--to", "test"]);
    assert!(refused.contains("refused:native-draft"), "{refused}");
    assert_eq!(fixture.read(".chough/transitions.jsonl"), log);
}
