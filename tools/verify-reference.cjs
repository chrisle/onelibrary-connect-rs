const fs=require('fs'),path=require('path'),crypto=require('crypto');
const provenance=require('../fixtures/PROVENANCE.json');
for(const [name,hash] of Object.entries(provenance.source_sha256)){
 const actual=crypto.createHash('sha256').update(fs.readFileSync(path.join(__dirname,'reference',name))).digest('hex');
 if(actual!==hash)throw Error('Pinned reference source checksum mismatch');
}
if(crypto.createHash('sha256').update(fs.readFileSync(path.join(__dirname,'../fixtures/test-onelibrary.db'))).digest('hex')!==provenance.fixture_sha256)throw Error('Pinned fixture checksum mismatch');
console.log('All pinned TS source and encrypted fixture hashes match');
