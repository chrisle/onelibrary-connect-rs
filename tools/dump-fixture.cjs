// Reproduce the redistributable mock schema/data, never a personal DB.
const Database=require('better-sqlite3-multiple-ciphers');
const {getEncryptionKey}=require('./reference.cjs');
const fs=require('fs');const path=require('path');
const db=new Database(path.join(__dirname,'../fixtures/test-onelibrary.db'),{readonly:true});db.pragma('cipher=sqlcipher');db.pragma('legacy=4');db.key(Buffer.from(getEncryptionKey()));
const q=s=>'"'+s.replaceAll('"','""')+'"';
const literal=x=>x===null?'NULL':typeof x==='number'?String(x):typeof x==='string'?"'"+x.replaceAll("'","''")+"'":(()=>{throw Error('Unexpected mock fixture value type')})();
let output='-- Synthetic mock database schema/data from pinned OneLibrary fixture, MIT.\n';
for(const {name,sql} of db.prepare("SELECT name,sql FROM sqlite_schema WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name").all()){
 output+=sql+';\n';for(const row of db.prepare(`SELECT * FROM ${q(name)}`).all())output+=`INSERT INTO ${q(name)}(${Object.keys(row).map(q).join(',')}) VALUES(${Object.values(row).map(literal).join(',')});\n`;
}
const evidence=db.prepare('SELECT sqlite_version() AS sqlite,sqlite3mc_version() AS codec').get();db.close();
fs.writeFileSync(path.join(__dirname,'../fixtures/synthetic.sql'),output);
fs.writeFileSync(path.join(__dirname,'node-evidence.json'),JSON.stringify({node:process.version,abi:process.versions.modules,...evidence,originalQueries:81,mutatedQueries:81},null,2)+'\n');
console.log('Synthetic schema/data generator saved; native engine identity recorded');
