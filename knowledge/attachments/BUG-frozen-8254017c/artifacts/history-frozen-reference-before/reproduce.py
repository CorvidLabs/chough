import hashlib,json,pathlib,subprocess,tempfile,time
REPO=pathlib.Path('/Users/leif/Development/_CorvidLabs/_worktrees/chough-restate')
BINARY=REPO/'target/debug/chough'
OUT=pathlib.Path(tempfile.mkdtemp(prefix='chough-peer-retention-alias-'))
ROOT=OUT/'fixture';ROOT.mkdir()
commands=[]
def save(name,value):
 (OUT/name).write_text(json.dumps(value,indent=2)+'\n')
def run(*args):
 cmd=[str(BINARY),'--root',str(ROOT),*args]
 p=subprocess.run(cmd,capture_output=True,text=True)
 commands.append({'command':cmd,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr})
 save('commands.json',commands)
 if p.returncode:raise RuntimeError(p.stderr)
 return json.loads(p.stdout)
def inventory():
 return {str(p.relative_to(ROOT)):{'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted(ROOT.rglob('*')) if p.is_file()}
metadata={'repo':str(REPO),'head':subprocess.check_output(['git','-C',str(REPO),'rev-parse','HEAD'],text=True).strip(),'binary':str(BINARY),'binary_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest(),'fixture':str(ROOT),'output':str(OUT)}
save('metadata.json',metadata)
run('init','--objective','Disposable peer review','--module','cli')
run('stage','iterate','--note','first')
for n in range(6):run('stage','reset','--to','intent','--reason',f'close {n}')
paths=[];cursor=json.loads((ROOT/'.chough/session.json').read_text())['history_cursor']
while cursor and cursor.get('head'):
 p=ROOT/f".chough/history/cycles/{cursor['cycle']}/{cursor['head']}.json";v=json.loads(p.read_text());paths.append(p)
 cursor=v['previous_cycle'] if v['parent'] is None else dict(cursor,head=v['parent'])
for n,p in enumerate(reversed(paths)):
 v=json.loads(p.read_text());v['transition']['at_unix_seconds']=int(time.time())-30*86400+n;p.write_text(json.dumps(v))
artifact=ROOT/'.chough/artifacts/frozen-path-reference.log';artifact.parent.mkdir(parents=True)
artifact.write_text('`.chough/history/cycles`\n')
slash=run('history','prune');save('slash-preview.json',slash)
artifact.write_text('`.CHOUGH\\history\\CYCLES`\n')
alias=run('history','prune');save('alias-preview.json',alias)
run('consent','grant','retention','--source','Disposable reviewer fixture only')
save('before-apply-inventory.json',inventory())
applied=run('history','prune','--apply');save('alias-apply.json',applied)
save('after-apply-inventory.json',inventory())
summary={**metadata,'slash_eligible':len(slash['eligible']),'alias_eligible':len(alias['eligible']),'removed_count':len(applied['removed']),'complete':applied['complete'],'artifact_content':artifact.read_text(),'binary_sha256_after':hashlib.sha256(BINARY.read_bytes()).hexdigest()}
save('summary.json',summary)
print(json.dumps(summary,indent=2))
