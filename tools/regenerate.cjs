// Fail on any addon error; no native ABI skips. Uses disposable copies only.
const {spawnSync}=require('child_process'),fs=require('fs'),path=require('path'),os=require('os');
function run(exe,args){const r=spawnSync(exe,args,{stdio:'inherit',env:{...process.env,TZ:'UTC'}});if(r.status!==0)throw Error('Fixture subprocess failed with status '+r.status);}
require('./verify-reference.cjs');
require('esbuild').buildSync({entryPoints:[path.join(__dirname,'reference/entry.ts')],bundle:true,platform:'node',format:'cjs',packages:'external',outfile:path.join(__dirname,'reference.cjs')});
run(process.execPath,[path.join(__dirname,'differential.cjs')]);
run(process.execPath,[path.join(__dirname,'differential.cjs'),'--edges']);
run(process.execPath,[path.join(__dirname,'dump-fixture.cjs')]);
run(process.execPath,[path.join(__dirname,'calendar-reference.cjs')]);
run(process.execPath,[path.join(__dirname,'extension-reference.cjs')]);
const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'onelibrary-rust-writer-'));
try{
 const file=path.join(scratch,'synthetic.db');run('cargo',['run','--locked','--example','generate_fixture','--',file]);
 run(process.execPath,[path.join(__dirname,'differential.cjs'),'--fixture',file]);
}finally{fs.rmSync(scratch,{recursive:true,force:true});}
