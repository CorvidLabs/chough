import hashlib,json,pathlib,subprocess,tempfile,time
REPO=pathlib.Path('/Users/leif/Development/_CorvidLabs/_worktrees/chough-restate')
BINARY=REPO/'target/debug/chough'
OUT=pathlib.Path(tempfile.mkdtemp(prefix='chough-post-prune-reference-'))
ROOT=OUT/'fixture';ROOT.mkdir();commands=[]
def save(name,value):(OUT/name).write_text(json.dumps(value,indent=2)+'\n')
def exec_cmd(cmd,expected=0):
 p=subprocess.run(cmd,cwd=ROOT,capture_output=True,text=True)
 commands.append({'command':cmd,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr});save('commands.json',commands)
 if expected is not None and p.returncode!=expected:raise RuntimeError(f'{cmd}: {p.stderr}')
 return p
def run(*args):return json.loads(exec_cmd([str(BINARY),'--root',str(ROOT),*args]).stdout)
def git(*args):return exec_cmd(['git','-c','user.name=Disposable peer review','-c','user.email=peer@example.invalid',*args])
def inventory():return {str(p.relative_to(ROOT)):{'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted((ROOT/'.chough').rglob('*')) if p.is_file()}
def repoid():return {'head':subprocess.check_output(['git','-C',str(REPO),'rev-parse','HEAD'],text=True).strip(),'history_source_sha256':hashlib.sha256((REPO/'src/history.rs').read_bytes()).hexdigest(),'binary':str(BINARY),'binary_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest()}
metadata={'repo':str(REPO),'before':repoid(),'fixture':str(ROOT),'output':str(OUT)};save('metadata.json',metadata)
run('init','--objective','Disposable post-prune branch reference','--module','cli')
run('stage','iterate','--note','initial')
for n in range(6):run('stage','reset','--to','intent','--reason',f'close {n}')
paths=[];cursor=json.loads((ROOT/'.chough/session.json').read_text())['history_cursor']
while cursor and cursor.get('head'):
 p=ROOT/f".chough/history/cycles/{cursor['cycle']}/{cursor['head']}.json";v=json.loads(p.read_text());paths.append(p)
 cursor=v['previous_cycle'] if v['parent'] is None else dict(cursor,head=v['parent'])
for n,p in enumerate(reversed(paths)):
 v=json.loads(p.read_text());v['transition']['at_unix_seconds']=int(time.time())-30*86400+n;p.write_text(json.dumps(v))
preview=run('history','prune');save('baseline-preview.json',preview)
target=preview['eligible'][0]['files'][0]['path'];target_bytes=(ROOT/target).read_bytes();(OUT/'target-original-event.json').write_bytes(target_bytes)
(ROOT/'.gitattributes').write_text('.chough/session.json merge=chough-session\n')
git('init');git('config','merge.chough-session.driver',f"'{BINARY}' checkpoint --merge %O %A %B")
git('checkout','-b','baseline');git('add','.');git('commit','-m','shared pre-prune history')
save('baseline-inventory.json',inventory())
git('checkout','-b','branch-a-prune');run('consent','grant','retention','--source','Disposable fixture authorization only')
prune=run('history','prune','--apply');save('branch-a-apply.json',prune)
assert target in prune['removed']
git('add','.chough');git('commit','-m','prune eligible old cycles')
save('branch-a-after-prune-inventory.json',inventory())
git('checkout','-b','branch-b-reference','baseline')
assert (ROOT/target).read_bytes()==target_bytes
checkpoint=run('checkpoint','--file',target);save('branch-b-checkpoint.json',checkpoint)
git('add','.chough/session.json');git('commit','-m','reference retained pre-prune event')
save('branch-b-before-merge-inventory.json',inventory())
merge=git('merge','branch-a-prune','--no-edit');save('merge-result.json',{'exit_code':merge.returncode,'stdout':merge.stdout,'stderr':merge.stderr})
save('after-merge-inventory.json',inventory())
show=run('history','show');save('after-merge-history-show.json',show)
timing=run('timing','--cycle','all','--json');save('after-merge-timing.json',timing)
prune_preview=run('history','prune');save('after-merge-prune-preview.json',prune_preview)
cycle=target.split('/')[3];entry=next(c for c in show['cycles'] if c['id']==cycle)
summary={'metadata':metadata,'after':repoid(),'target':target,'target_sha256':hashlib.sha256(target_bytes).hexdigest(),'target_exists_after_merge':(ROOT/target).exists(),'checkpoint_references_target':target in json.loads((ROOT/'.chough/session.json').read_text())['references']['files'],'merge_exit_code':merge.returncode,'history_show_complete':show['complete'],'target_cycle_show':entry,'timing_partial':timing['partial']}
save('summary.json',summary);print(json.dumps(summary,indent=2))
