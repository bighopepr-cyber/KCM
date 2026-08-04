# KCM C SDK

C FFI bindings for KCM. Already implemented in `kcm-interface`.

## Status: Stable

## API

18 FFI functions defined in `kcm-interface/src/lib.rs`:

```c
KCM_Database* KCM_DatabaseNew(const char* path);
void KCM_DatabaseFree(KCM_Database* db);
KCM_Error KCM_DatabaseInsert(KCM_Database* db, const KCM_Fact* fact);
KCM_Error KCM_DatabaseDelete(KCM_Database* db, uint64_t row_id);
KCM_Error KCM_DatabaseUpdate(KCM_Database* db, const KCM_Fact* fact);
uint64_t KCM_DatabaseFactCount(KCM_Database* db);
uint64_t KCM_DatabaseActiveCount(KCM_Database* db);
KCM_Query* KCM_DatabaseQuery(KCM_Database* db, const char* query);
KCM_Fact* KCM_QueryNext(KCM_Query* query);
void KCM_QueryFree(KCM_Query* query);
KCM_Error KCM_DatabaseSave(KCM_Database* db, const char* path);
KCM_Error KCM_DatabaseLoad(KCM_Database* db, const char* path);
KCM_Error KCM_DatabaseVerify(KCM_Database* db);
KCM_Transaction* KCM_DatabaseBeginTransaction(KCM_Database* db);
KCM_Error KCM_TransactionCommit(KCM_Transaction* txn);
KCM_Error KCM_TransactionRollback(KCM_Transaction* txn);
void KCM_TransactionFree(KCM_Transaction* txn);
const char* KCM_ErrorMessage(KCM_Error error);
```

## Usage

```c
#include <kcm/interface.h>

KCM_Database* db = KCM_DatabaseNew("my_knowledge.db");
KCM_Fact fact = { .subject = 1, .predicate = 1, .object = 1, .confidence = 0.99 };
KCM_DatabaseInsert(db, &fact);
printf("Fact count: %lu\n", KCM_DatabaseFactCount(db));
KCM_DatabaseFree(db);
```
