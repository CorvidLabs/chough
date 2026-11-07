import subprocess, sys, pathlib, json
root = pathlib.Path(sys.argv[1]); logs = pathlib.Path(sys.argv[2]); logs.mkdir(exist_ok=True)
src = root / "src/change.rs"
original = src.read_text()
ship_call = 'let output =\n        records::run_owned(&mut command, None).map_err(|error| not_started("ship", &error))?;'
mutants = [
 ("skip-stage-check", 'if stage != "accept" {', 'if false && stage != "accept" {'),
 ("skip-gate-check", "if let Some(other) = open.iter().find(|gate| Some(gate.as_str()) != named) {", "if let Some(other) = open.iter().find(|_| false) {"),
 ("close-gate-before-ship", 'let mut command = Command::new(SPECSYNC);\n    command.args(["change", "ship", id])',
  'if let Some((gate, relay, _)) = &relayed {\n        let _ = Command::new(std::env::current_exe()?).arg("--root").arg(root).args(["checkpoint", "--close-gate", gate, "--decided-by", &relay.decided_by, "--words", &relay.words, "--where", &relay.place, "--recorded-by", &relay.recorded_by]).status();\n    }\n    let mut command = Command::new(SPECSYNC);\n    command.args(["change", "ship", id])'),
 ("skip-close", "if let Some((gate, relay, _)) = relayed {", "if let Some((gate, relay, _)) = relayed.filter(|_| false) {"),
 ("skip-done-move", '["stage", "advance", "--to", "done"]', '["stage", "show"]'),
 ("exit-zero-after-refused-step", "if !output.status.success() {", "if false && !output.status.success() {"),
 ("bound-applies-to-ship", ship_call,
  'let output = records::run_bounded(root, &["specsync", "change", "ship", id.as_str()], &[], &Limits { stdout_cap: usize::MAX, stderr_cap: usize::MAX, deadline: native_deadline(std::env::var_os(NATIVE_TIMEOUT_VARIABLE).as_deref()).unwrap_or(NATIVE_DEADLINE) }).map(|captured| std::process::Output { status: std::os::unix::process::ExitStatusExt::from_raw(captured.exit_code.unwrap_or(1) << 8), stdout: captured.stdout, stderr: captured.stderr }).map_err(|error| not_started("ship", &error))?;'),
]
results = []
try:
    for name, old, new in [m for m in mutants if m[0] in sys.argv[3:]] or mutants:
        if original.count(old) != 1:
            results.append((name, "NOT-APPLIED")); continue
        src.write_text(original.replace(old, new))
        r = subprocess.run(["cargo", "test", "--locked", "--test", "change"], cwd=root, capture_output=True, text=True)
        (logs / f"{name}.log").write_text(r.stdout + r.stderr)
        failed = [l.split()[1] for l in r.stdout.splitlines() if l.startswith("test ") and l.endswith("FAILED")]
        compiled = "error[" not in r.stderr and "error: could not compile" not in r.stderr
        results.append((name, "caught" if r.returncode != 0 and compiled else ("COMPILE-ERROR" if not compiled else "SURVIVED"), failed))
finally:
    src.write_text(original)
print(json.dumps(results, indent=1))
