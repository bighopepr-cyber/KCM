import { Database, FactData } from '../src/index';

const db = new Database();

const fact1: FactData = {
    subject: 1,
    predicate: 0,
    object: 2,
    confidence: 0.95,
    evidence: 1,
    timestamp: Date.now(),
    context: 1,
    version: 1,
    priority: 0,
    owner: 1,
};

const fact2: FactData = {
    subject: 2,
    predicate: 1,
    object: 3,
    confidence: 0.85,
    evidence: 2,
    timestamp: Date.now(),
    context: 1,
    version: 1,
    priority: 1,
    owner: 2,
};

const id1 = db.insert(fact1);
console.log(`Inserted fact 1: row_id=${id1}`);

const id2 = db.insert(fact2);
console.log(`Inserted fact 2: row_id=${id2}`);

console.log(`Total facts: ${db.factCount()}`);
console.log(`Active facts: ${db.activeFactCount()}`);

const allFacts = db.queryAll();
console.log(`QueryAll returned ${allFacts.length} facts`);

const queryResult = db.query('SELECT * FROM facts');
console.log(`Query returned ${queryResult.count} facts`);

db.update(id1, {
    ...fact1,
    confidence: 0.99,
    version: 2,
});
console.log(`Updated fact ${id1}`);

db.delete(id2);
console.log(`Deleted fact ${id2}`);
console.log(`Active facts after delete: ${db.activeFactCount()}`);

const txn = db.beginTransaction();
console.log('Transaction started');

txn.commit();
console.log('Transaction committed');

const txn2 = db.beginTransaction();
txn2.rollback();
console.log('Transaction rolled back');

db.save('/tmp/kcm_example.json');
console.log('Saved to /tmp/kcm_example.json');

Database.verify('/tmp/kcm_example.json');
console.log('File verified');

db.close();
console.log('Database closed');
