import hashlib, json, pathlib, shutil, subprocess, tempfile, time
repo=pathlib.Path('/Users/leif/Development/_CorvidLabs/_worktrees/chough-restate')
binary=repo/'target/debug/chough'
record=pathlib.Path(tempfile.mkdtemp(prefix='chough-idle-cycle-age-before-'))
fixture=record/'fixture'
fixture.mkdir()
commands=[]
def run(*args):
 command=[str(binary),'--root',str(fixture),*args]
 p=subprocess.run(command,text=True,capture_output=True)
 commands.append({'command':command,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 if p.returncode: raise RuntimeError(commands[-1])
 return json.loads(p.stdout)
meta={'revision':subprocess.check_output(['git','rev-parse','HEAD'],cwd=repo,text=True).strip(),'binary':str(binary),'binary_sha256':hashlib.sha256(binary.read_bytes()).hexdigest(),'recorded_at_unix_seconds':int(time.time()),'fixture':str(fixture)}
(record/'source.diff').write_bytes(subprocess.check_output(['git','diff','--binary','HEAD'],cwd=repo))
(record/'metadata.json').write_text(json.dumps(meta,indent=2)+'\n')
run('init','--objective','Idle cycle age reproduction','--module','cli')
run('stage','iterate','--note','event before a long idle interval')
checkpoint=json.loads((fixture/'.chough/session.json').read_text())
cursor=checkpoint['history_cursor']
old_cycle=cursor['cycle']
old_path=fixture/f'.chough/history/cycles/{old_cycle}/{cursor["head"]}.json'
event=json.loads(old_path.read_text())
event['transition']['at_unix_seconds']=int(time.time())-30*86400
old_path.write_text(json.dumps(event,separators=(',',':')))
for index in range(4): run('stage','reset','--to','intent','--reason',f'new cycle {index}')
shutil.copytree(fixture/'.chough',record/'input-before-preview')
preview=run('history','prune')
(record/'preview.json').write_text(json.dumps(preview,indent=2)+'\n')
run('consent','grant','retention','--source','fixture retention authorization')
shutil.copytree(fixture/'.chough',record/'input-before-apply')
applied=run('history','prune','--apply')
(record/'apply.json').write_text(json.dumps(applied,indent=2)+'\n')
(record/'commands.json').write_text(json.dumps(commands,indent=2)+'\n')
meta.update({'old_cycle':old_cycle,'old_event_path':str(old_path),'old_cycle_eligible':any(c['cycle']==old_cycle for c in preview['eligible']),'old_event_exists_after_apply':old_path.exists(),'removed':applied['removed'],'apply_complete':applied['complete']})
(record/'metadata.json').write_text(json.dumps(meta,indent=2)+'\n')
print(json.dumps({'record':str(record),**meta},indent=2))
