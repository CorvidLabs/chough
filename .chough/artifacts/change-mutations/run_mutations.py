import subprocess, sys, pathlib, json
root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(exist_ok=True)
src = root / "src/change.rs"
original = src.read_text()
mutants = [
 ("drop-commit", 'vec!["change".into(), "check".into(), id, "--commit".into()]', 'vec!["change".into(), "check".into(), id]'),
 ("status-drops-json", 'arguments.extend(id);\n            arguments.push("--json".into());', 'arguments.extend(id);'),
 ("swap-streams", "out.write_all(stdout)?;", "out.write_all(stderr)?;"),
 ("exit-zero", "Ok(exit_code(output.status.code(), signal(&output.status)))", "Ok(0)"),
 ("bound-applies-to-check", '"check",\n            vec!["change".into(), "check".into(), id, "--commit".into()],\n            false,', '"check",\n            vec!["change".into(), "check".into(), id, "--commit".into()],\n            true,'),
 ("status-unbounded", "        deadline,\n    };", "        deadline: Duration::from_secs(3_600),\n    };"),
 ("no-working-directory", "command.args(arguments).current_dir(root);", "command.args(arguments);"),
 ("invalid-bound-defaults", "Err(got) => bail!(", "Err(_) if true => NATIVE_DEADLINE,\n        Err(got) => bail!("),
]
results = []
try:
    for name, old, new in [m for m in mutants if m[0] in sys.argv[3:]] or mutants:
        text = original
        if text.count(old) != 1:
            results.append((name, "NOT-APPLIED")); continue
        src.write_text(text.replace(old, new))
        r = subprocess.run(["cargo", "test", "--locked", "--test", "change"], cwd=root, capture_output=True, text=True)
        (logs / f"{name}.log").write_text(r.stdout + r.stderr)
        failed = [l.split()[1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compiled = "error[" not in r.stderr
        results.append((name, "caught" if r.returncode != 0 and compiled else ("COMPILE-ERROR" if not compiled else "SURVIVED"), failed))
finally:
    src.write_text(original)
print(json.dumps(results, indent=1))
