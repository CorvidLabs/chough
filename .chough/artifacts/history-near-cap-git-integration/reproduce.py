import hashlib,json,os,pathlib,platform,shlex,shutil,subprocess,tarfile,tempfile,time
REPO=pathlib.Path('/Users/leif/Development/_CorvidLabs/_worktrees/chough-restate')
OUT=pathlib.Path(tempfile.mkdtemp(prefix='chough-cap-fork-proof-'))
ROOT=OUT/'fixture';ROOT.mkdir();(OUT/'raw').mkdir();(OUT/'inventories').mkdir();(OUT/'inputs').mkdir();(OUT/'bin').mkdir()
BINARY=OUT/'bin/chough';shutil.copy2(REPO/'target/debug/chough',BINARY)
COMMANDS=[]
def save(name,value):
 p=OUT/name;p.parent.mkdir(parents=True,exist_ok=True);p.write_text(json.dumps(value,indent=2,sort_keys=True)+'\n')
def sha(data):return hashlib.sha256(data).hexdigest()
def execute(args,label,expected=0,timeout=60):
 index=len(COMMANDS);stem=f'{index:03d}-{label}';stdout=OUT/'raw'/f'{stem}.stdout';stderr=OUT/'raw'/f'{stem}.stderr';start=time.monotonic()
 with stdout.open('wb') as out,stderr.open('wb') as err:
  try:p=subprocess.run(args,cwd=ROOT,stdout=out,stderr=err,timeout=timeout);code=p.returncode
  except subprocess.TimeoutExpired:code='timeout'
 record={'command':list(map(str,args)),'cwd':str(ROOT),'exit_code':code,'elapsed_seconds':time.monotonic()-start,'timeout_seconds':timeout,'stdout':str(stdout.relative_to(OUT)),'stderr':str(stderr.relative_to(OUT))};COMMANDS.append(record);save('commands.json',COMMANDS)
 if expected is not None and code!=expected:raise RuntimeError(record)
 return code,stdout,stderr
def git(*args,label='git',expected=0):
 return execute(['git','-c','user.name=Disposable cap fixture','-c','user.email=cap@example.invalid','-c','commit.gpgsign=false','-c','core.hooksPath=/dev/null',*args],label,expected,60)
def cli(label,*args,expected=0):
 return execute([str(BINARY),'--root',str(ROOT),*args],label,expected,30)
def read_json(output):return json.loads(output.read_text())
def inventory():
 return {str(p.relative_to(ROOT)):{'bytes':p.stat().st_size,'sha256':sha(p.read_bytes())} for p in sorted((ROOT/'.chough').rglob('*')) if p.is_file()}
def event_inventory():
 return {k:v for k,v in inventory().items() if k.startswith('.chough/history/cycles/')}
def checkpoint():return json.loads((ROOT/'.chough/session.json').read_text())
def write_checkpoint(value):(ROOT/'.chough/session.json').write_text(json.dumps(value,separators=(',',':'),sort_keys=True)+'\n')
def event(identifier,parent,reason,ordinal):
 return {'schema_version':1,'cycle':CYCLE,'id':identifier,'parent':parent,'previous_cycle':None,'native_change':None,'transition':{'schema_version':1,'at_unix_seconds':1700000000+ordinal,'kind':'iterate','from':'intent','to':'intent','reason':reason,'gate':None,'revision':None,'refresh':[]}}
def publish(identifier,parent,reason,ordinal):
 payload=event(identifier,parent,reason,ordinal);data=json.dumps(payload,separators=(',',':'),sort_keys=True).encode();p=ROOT/f'.chough/history/cycles/{CYCLE}/{identifier}.json';p.parent.mkdir(parents=True,exist_ok=True)
 with p.open('xb') as f:f.write(data)
 assert len(data)<=65536
 return identifier
readonly_env=os.environ.copy();readonly_env['GIT_OPTIONAL_LOCKS']='0'
def repo_git(*args):return subprocess.check_output(['git',*args],cwd=REPO,env=readonly_env,text=True).strip()
qualified=repo_git('rev-parse','998fcd5^{commit}');head=repo_git('rev-parse','HEAD');diff=repo_git('diff','--name-only',qualified,'HEAD','--','src','Cargo.toml','Cargo.lock','skills')
if diff:raise RuntimeError('Qualified runtime source changed: '+diff)
assert sha((REPO/'src/history.rs').read_bytes())=='e4deda0384980e54ca1defaf032b1b759c4a1afbadf39290131892aee60059c8'
assert sha((REPO/'tests/history.rs').read_bytes())=='2e13f852eee408e3691d31c1c4bf496cd1aad5376f1f0e482043d7d00105cd97'
metadata={'qualified_source_commit':qualified,'observed_worktree_head':head,'runtime_source_changes_since_qualified':diff,'binary_original':str(REPO/'target/debug/chough'),'binary_frozen':str(BINARY),'binary_sha256':sha(BINARY.read_bytes()),'history_source_sha256':sha((REPO/'src/history.rs').read_bytes()),'platform':platform.platform(),'machine':platform.machine(),'construction':'Synthetic deterministic valid event-envelope files, like tests/history.rs::chain; real Git integration and real CLI reads/refusal/reset. This is not 8190 real CLI appends.','output':str(OUT),'fixture':str(ROOT),'started_at_unix_seconds':int(time.time())}
save('metadata.json',metadata);shutil.copy2(__file__,OUT/'reproduce.py');print(str(OUT),flush=True)
(ROOT/'AGENTS.md').write_text('Disposable capacity integration fixture.\n');(ROOT/'INTENT.md').write_text('Preserve both branch histories at the writer cap.\n');(ROOT/'specs/cli').mkdir(parents=True);(ROOT/'specs/cli/cli.spec.md').write_text('# Disposable CLI contract\n')
cli('version','--version');cli('init','init','--objective','Synthetic valid-chain cap integration','--module','cli');assert not (ROOT/'.specsync/config.toml').exists() and not (ROOT/'.specsync/config.json').exists()
CYCLE=checkpoint()['history_cursor']['cycle'];BASE='EVT-'+format(0,'024x');publish(BASE,None,'synthetic shared base',0);s=checkpoint();s['history_cursor']['head']=BASE;s['stage']['iteration']=2;write_checkpoint(s)
(ROOT/'.gitattributes').write_text('.chough/session.json merge=chough-session\n');git('init','-b','base',label='init-git');git('config','core.autocrlf','false',label='local-line-endings');git('config','merge.chough-session.driver',f'{shlex.quote(str(BINARY))} checkpoint --merge %O %A %B',label='local-merge-driver');git('add','.',label='stage-base');git('commit','-m','shared one-event synthetic base',label='commit-base')
branch_inventories={};branch_checkpoints={}
for name,offset in (('left',0),('right',0x100000)):
 git('checkout','-b',name,'base',label='checkout-'+name);parent=BASE
 for index in range(1,4096):parent=publish('EVT-'+format(offset+index,'024x'),parent,'synthetic '+name+' branch',index)
 s=checkpoint();s['history_cursor']['head']=parent;s['stage']['iteration']=4097;s['next_action']='Synthetic '+name+' lineage is selected only by its complete checkpoint';write_checkpoint(s)
 inv=event_inventory();assert len(inv)==4096 and sum(v['bytes'] for v in inv.values())<=16*1048576
 branch_inventories[name]=inv;branch_checkpoints[name]=(ROOT/'.chough/session.json').read_bytes();save(f'inventories/{name}-4096.json',inv);(OUT/'inputs'/f'{name}-checkpoint.json').write_bytes(branch_checkpoints[name])
 before=inventory();_,show,_=cli(name+'-show','history','show');report=read_json(show);assert report['complete'] is True and report['cycles'][0]['events']==4096
 _,timing,_=cli(name+'-timing-current','timing','--cycle','current','--json');timing=read_json(timing);assert timing['transitions']['records']==4096 and timing['partial']==[];assert inventory()==before
 git('add','.chough',label='stage-'+name);git('commit','-m','synthetic '+name+' reaches exactly 4096 events',label='commit-'+name)
 print(name+' branch exactly4096 validated',flush=True)
union=dict(branch_inventories['left'])
for path,value in branch_inventories['right'].items():
 if path in union:assert union[path]==value
 union[path]=value
assert len(union)==8191
git('checkout','left',label='checkout-selected-left');merge_code,_,merge_error=git('merge','right','--no-edit',label='real-fork-merge',expected=1)
_,conflict_file,_=git('diff','--name-only','--diff-filter=U',label='conflict-paths');assert conflict_file.read_text().splitlines()==['.chough/session.json'];assert 'current-cycle conflict' in merge_error.read_text();git('ls-files','-u',label='whole-checkpoint-conflict-stages');git('checkout','--ours','--','.chough/session.json',label='select-whole-left-checkpoint');assert (ROOT/'.chough/session.json').read_bytes()==branch_checkpoints['left'];git('add','.chough/session.json',label='stage-selected-checkpoint');git('commit','-m','retain both histories and select coherent left checkpoint',label='commit-real-merge')
merged=event_inventory();assert merged==union;save('inventories/merged-8191.json',merged);save('inputs/merged-checkpoint.json',checkpoint())
read_before=inventory();_,show_file,_=cli('merged-show','history','show');show=read_json(show_file);assert show['cycles'][0]['events']==8191 and show['cycles'][0]['unselected_tips'];_,preview_file,_=cli('merged-prune-preview','history','prune');preview=read_json(preview_file);assert preview['eligible']==[];_,timing_file,_=cli('merged-timing-all','timing','--cycle','all','--json');timing=read_json(timing_file);assert timing['transitions']['records']<=4096 and timing['partial'];assert inventory()==read_before
append_before=inventory();code,_,append_error=cli('append-refuses-at-merged-cap','stage','iterate','--note','real CLI append must refuse over-cap cycle',expected=None);assert code not in (0,'timeout') and ('bound' in append_error.read_text() or 'capacity' in append_error.read_text());append_unchanged=inventory()==append_before;assert append_unchanged;save('inventories/after-refused-append.json',event_inventory());save('inputs/checkpoint-after-refused-append.json',checkpoint())
old_cursor=checkpoint()['history_cursor'];cli('reasoned-reset','stage','reset','--to','intent','--reason','Start a fresh bounded cycle while retaining all merged fork proof');new_cursor=checkpoint()['history_cursor'];assert new_cursor['cycle']!=CYCLE
reset_inventory=event_inventory();old_after={k:v for k,v in reset_inventory.items() if k.startswith(f'.chough/history/cycles/{CYCLE}/')};assert old_after==union and len(reset_inventory)==8192;save('inventories/after-reset-8192.json',reset_inventory);save('inputs/checkpoint-after-reset.json',checkpoint());new_events={k:v for k,v in reset_inventory.items() if k not in union};assert len(new_events)==1
for path in new_events:(OUT/'inputs'/'new-reset-event.json').write_bytes((ROOT/path).read_bytes())
post_reset_before=inventory();_,post_show_file,_=cli('after-reset-show','history','show');post_show=read_json(post_show_file);_,post_preview_file,_=cli('after-reset-prune-preview','history','prune');post_preview=read_json(post_preview_file);_,post_timing_file,_=cli('after-reset-timing-all','timing','--cycle','all','--json');post_timing=read_json(post_timing_file);assert post_timing['transitions']['records']<=4096 and post_timing['partial'];assert inventory()==post_reset_before
git('bundle','create',str(OUT/'fixture-commits.bundle'),'--all',label='fixture-git-bundle');git('log','--all','--graph','--oneline','--decorate',label='real-git-graph');git('diff','--exit-code','HEAD','--',f'.chough/history/cycles/{CYCLE}',label='old-cycle-no-byte-diff-after-reset')
summary={'construction':metadata['construction'],'qualified_source_commit':qualified,'observed_worktree_head':head,'binary_sha256':metadata['binary_sha256'],'cycle':CYCLE,'each_branch_events':{k:len(v) for k,v in branch_inventories.items()},'shared_base_events':1,'distinct_generated_branch_events':8190,'merged_event_files':len(merged),'merged_sha_inventory_exact_union':merged==union,'checkpoint_selected_whole_left':True,'merged_show_complete':show['complete'],'merged_show_events':show['cycles'][0]['events'],'merged_unselected_tips':show['cycles'][0]['unselected_tips'],'merged_preview_complete':preview['complete'],'merged_preview_eligible':preview['eligible'],'merged_timing_records':timing['transitions']['records'],'merged_timing_partial':timing['partial'],'ordinary_append_exit_code':code,'ordinary_append_no_checkpoint_or_file_mutation':append_unchanged,'reasoned_reset_fresh_cycle':new_cursor['cycle']!=CYCLE,'old_8191_events_byte_identical_after_reset':old_after==union,'total_events_after_reset':len(reset_inventory),'after_reset_show_complete':post_show['complete'],'after_reset_preview_complete':post_preview['complete'],'after_reset_timing_records':post_timing['transitions']['records'],'after_reset_timing_partial':post_timing['partial'],'notes':['Inventory complete:true is truthful when all8191files were enumerated below the16384-source maintenance budget; unselected tip is explicit.','Timing stays at4096records and explicitly reports omitted/unknown lineages.','No native configuration, retention apply, repository source write or lifecycle record was used.','Fixture Git commits only; no real project commit or push.'],'finished_at_unix_seconds':int(time.time())}
save('summary.json',summary)
metadata['command_count']=len(COMMANDS);metadata['frozen_binary_sha256_after']=sha(BINARY.read_bytes());assert metadata['frozen_binary_sha256_after']==metadata['binary_sha256'];save('metadata.json',metadata)
archive=pathlib.Path(str(OUT)+'.tar.gz')
with tarfile.open(archive,'w:gz') as tar:
 for path in sorted(OUT.iterdir()):
  if path.name not in ('fixture','bin'):tar.add(path,arcname=OUT.name+'/'+path.name)
print(json.dumps({'output':str(OUT),'archive':str(archive),'archive_bytes':archive.stat().st_size,'summary':summary},indent=2),flush=True)
