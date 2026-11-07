// Execute the pinned TS adapter on a disposable encrypted extension-column fixture.
require('./verify-reference.cjs');
const fs=require('fs'),path=require('path'),os=require('os'),assert=require('node:assert/strict');
require('esbuild').buildSync({entryPoints:[path.join(__dirname,'reference/entry.ts')],bundle:true,platform:'node',format:'cjs',packages:'external',outfile:path.join(__dirname,'reference.cjs')});
const {OneLibraryAdapter,getEncryptionKey}=require('./reference.cjs');
const Database=require('better-sqlite3-multiple-ciphers');
const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'onelibrary-extensions-'));
try {
 const file=path.join(scratch,'fixture.db');fs.copyFileSync(path.join(__dirname,'../fixtures/test-onelibrary.db'),file);
 const writer=new Database(file);writer.pragma('cipher=sqlcipher');writer.pragma('legacy=4');writer.key(Buffer.from(getEncryptionKey()));
 writer.exec(fs.readFileSync(path.join(__dirname,'../fixtures/extensions.sql'),'utf8'));writer.close();
 const db=new OneLibraryAdapter(file);
 const queries=JSON.parse(fs.readFileSync(path.join(__dirname,'../fixtures/queries.json'),'utf8'));
 const actual=queries.map(({method,args})=>({method,args,result:db[method](...args)}));db.close();
 assert.deepStrictEqual(JSON.parse(JSON.stringify(actual)),JSON.parse(fs.readFileSync(path.join(__dirname,'../fixtures/expected.json'),'utf8')));
 console.log('Actual pinned TS encrypted extension-column oracle: all81 queries unchanged');
} finally {fs.rmSync(scratch,{recursive:true,force:true});}
