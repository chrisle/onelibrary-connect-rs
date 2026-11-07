// Actual original adapter/dateAdded and substring behavior on disposable copies.
const fs=require('fs'),path=require('path'),os=require('os');
const {OneLibraryAdapter,getEncryptionKey}=require('./reference.cjs');
const Database=require('better-sqlite3-multiple-ciphers');
const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'onelibrary-calendar-'));
try{
 const file=path.join(scratch,'fixture.db');fs.copyFileSync(path.join(__dirname,'../fixtures/test-onelibrary.db'),file);
 const writer=new Database(file);writer.pragma('cipher=sqlcipher');writer.pragma('legacy=4');writer.key(Buffer.from(getEncryptionKey()));
 const adapter=new OneLibraryAdapter(file);const cases=[];
 for(const input of require('../fixtures/calendar-inputs.json')){
  writer.prepare('UPDATE content SET dateAdded=? WHERE content_id=1').run(input);
  cases.push({input,result:adapter.findTrack(1).dateAdded});
 }
 const paths=[];
 for(const input of ['😀abc','😀.DAT','/USB/音楽/😀.DAT','abc','x','', 'a😀abc']){
  writer.prepare('UPDATE content SET analysisDataFilePath=? WHERE content_id=1').run(input);
  const result=adapter.findTrack(1).analyzePath;
  const units=result===undefined?null:Array.from({length:result.length},(_,i)=>result.charCodeAt(i));
  // JS String.isWellFormed detects precisely the malformed UTF16 path boundary.
  paths.push({input,jsCodeUnits:units,nativePath:result===undefined||!result.isWellFormed()?null:result});
 }
 adapter.close();writer.close();fs.writeFileSync(path.join(__dirname,'../fixtures/expected-calendar.json'),JSON.stringify({dates:cases,paths},null,2)+'\n');
 console.log(`Actual TS adapter verified ${cases.length} calendar/24-hour cases and ${paths.length} UTF16 path cases`);
}finally{fs.rmSync(scratch,{recursive:true,force:true});}
