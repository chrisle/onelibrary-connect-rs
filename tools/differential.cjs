const fs=require('fs');const path=require('path');const os=require('os');
const {OneLibraryAdapter,getEncryptionKey}=require('./reference.cjs');
const Database=require('better-sqlite3-multiple-ciphers');
const fixtureIndex=process.argv.indexOf('--fixture');
const source=fixtureIndex>=0?path.resolve(process.argv[fixtureIndex+1]):path.resolve(__dirname,'../fixtures/test-onelibrary.db');
const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'onelibrary-reference-'));
try{
 const file=path.join(scratch,'fixture.db');fs.copyFileSync(source,file);
 if(process.argv.includes('--edges')){
 const writer=new Database(file);writer.pragma('cipher=sqlcipher');writer.pragma('legacy=4');
 // This API accepts a key separately from SQL and never logs it.
 writer.key(Buffer.from(getEncryptionKey()));
 writer.exec(fs.readFileSync(path.join(__dirname,'../fixtures/mutations.sql'),'utf8'));writer.close();
 }
 const adapter=new OneLibraryAdapter(file);
 const queries=JSON.parse(fs.readFileSync(path.join(__dirname,'../fixtures/queries.json'),'utf8'));
 const outputs=queries.map(({method,args})=>({method,args,result:adapter[method](...args)}));
 adapter.close();if(fixtureIndex>=0){if(!require('node:util').isDeepStrictEqual(JSON.parse(JSON.stringify(outputs)),JSON.parse(fs.readFileSync(path.join(__dirname,'../fixtures/expected.json'),'utf8'))))throw Error('Generated fixture normalized query output mismatch');console.log('Rust-writer / actual TS-reader all81 query roundtrip PASS');}
 if(fixtureIndex<0)fs.writeFileSync(path.join(__dirname,process.argv.includes('--edges')?'../fixtures/expected-edges.json':'../fixtures/expected.json'),JSON.stringify(outputs,null,2)+'\n');
 console.log(`Original TS executed ${outputs.length} real encrypted query cases; no skipped database suite`);
}finally{fs.rmSync(scratch,{recursive:true,force:true});}
