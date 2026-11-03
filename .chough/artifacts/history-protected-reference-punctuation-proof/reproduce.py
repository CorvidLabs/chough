import hashlib,json,pathlib,subprocess,tempfile,time
REPO=pathlib.Path('/Users/leif/Development/_CorvidLabs/_worktrees/chough-restate');BINARY=REPO/'target/debug/chough'
OUT=pathlib.Path(tempfile.mkdtemp(prefix='chough-post-prune-punctuation-'))
source=(REPO/'src/history.rs').read_bytes();(OUT/'history-source.rs').write_bytes(source)
def identity():return {'head':subprocess.check_output(['git','-C',str(REPO),'rev-parse','HEAD'],text=True).strip(),'source_sha256':hashlib.sha256((REPO/'src/history.rs').read_bytes()).hexdigest(),'binary_path':str(BINARY),'binary_sha256':hashlib.sha256(BINARY.read_bytes()).hexdigest()}
initial=identity();results=[]
for name,decorate in [('terminal-period',lambda p:p+'.'),('parenthesized',lambda p:'('+p+')')]:
 out=OUT/name;out.mkdir();root=out/'fixture';root.mkdir();commands=[]
 def save(n,v):(out/n).write_text(json.dumps(v,indent=2)+'\n')
 def execute(cmd):
  p=subprocess.run(cmd,cwd=root,capture_output=True,text=True);commands.append({'command':cmd,'exit_code':p.returncode,'stdout':p.stdout,'stderr':p.stderr});save('commands.json',commands)
  if p.returncode:raise RuntimeError(p.stderr)
  return p
 def run(*args):return json.loads(execute([str(BINARY),'--root',str(root),*args]).stdout)
 def git(*args):return execute(['git','-c','user.name=Disposable peer review','-c','user.email=peer@example.invalid',*args])
 def inventory():return {str(p.relative_to(root)):{'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()} for p in sorted((root/'.chough').rglob('*')) if p.is_file()}
 run('init','--objective','Disposable post-prune punctuation reference','--module','cli');run('stage','iterate','--note','initial')
 for n in range(6):run('stage','reset','--to','intent','--reason',f'close {n}')
 paths=[];cursor=json.loads((root/'.chough/session.json').read_text())['history_cursor']
 while cursor and cursor.get('head'):
  p=root/f".chough/history/cycles/{cursor['cycle']}/{cursor['head']}.json";v=json.loads(p.read_text());paths.append(p);cursor=v['previous_cycle'] if v['parent'] is None else dict(cursor,head=v['parent'])
 for n,p in enumerate(reversed(paths)):
  v=json.loads(p.read_text());v['transition']['at_unix_seconds']=int(time.time())-30*86400+n;p.write_text(json.dumps(v))
 preview=run('history','prune');save('baseline-preview.json',preview);target=preview['eligible'][0]['files'][0]['path'];cycle=target.split('/')[3];directory=target.rsplit('/',1)[0];original=(root/target).read_bytes();(out/'target-original-event.json').write_bytes(original)
 (root/'.gitattributes').write_text('.chough/session.json merge=chough-session\n');git('init');git('config','merge.chough-session.driver',f"'{BINARY}' checkpoint --merge %O %A %B");git('checkout','-b','baseline');git('add','.');git('commit','-m','shared pre-prune history');save('baseline-inventory.json',inventory())
 git('checkout','-b','prune');run('consent','grant','retention','--source','Disposable fixture authorization only');apply=run('history','prune','--apply');save('branch-a-apply.json',apply);assert target in apply['removed'];git('add','.chough');git('commit','-m','prune eligible old cycles');save('branch-a-inventory.json',inventory())
 git('checkout','-b','reference','baseline');artifact='.chough/artifacts/post-prune-punctuation.log';ap=root/artifact;ap.parent.mkdir(parents=True);text='Retain '+decorate(directory)+'\n';ap.write_text(text);git('add',artifact);git('commit','-m','reference old cycle directory in frozen text');save('branch-b-inventory.json',inventory());merge=git('merge','prune','--no-edit');save('merge-result.json',{'exit_code':merge.returncode,'stdout':merge.stdout,'stderr':merge.stderr});save('after-merge-inventory.json',inventory())
 show=run('history','show');save('after-merge-history-show.json',show);prune=run('history','prune');save('after-merge-prune-preview.json',prune)
 row=next(c for c in show['cycles'] if c['id']==cycle)
 ap.write_text('Retain '+directory+'\n');control=run('history','show');save('plain-directory-control-show.json',control);ap.write_text(text)
 summary={'variant':name,'input_reference':text,'target':target,'target_exists':(root/target).exists(),'merge_exit_code':merge.returncode,'complete':show['complete'],'partial':show.get('partial'),'target_cycle':row,'plain_control_complete':control['complete'],'plain_control_partial':control.get('partial'),'fixture':str(root),'output':str(out)};save('summary.json',summary);results.append(summary)
metadata={'before':initial,'after':identity(),'output':str(OUT),'results':results};(OUT/'summary.json').write_text(json.dumps(metadata,indent=2)+'\n');print(json.dumps(metadata,indent=2))
