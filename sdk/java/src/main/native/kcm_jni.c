#include <jni.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#include "kcm.h"

#define JNI_FN(name) Java_io_kcm_KcmNative_##name

static KCM_Fact make_fact(JNIEnv *env, jint subject, jbyte predicate, jint object,
                           jdouble confidence, jbyte evidence, jlong timestamp,
                           jbyte context, jint version, jbyte priority, jshort owner) {
    KCM_Fact f;
    f.subject = (uint32_t)subject;
    f.predicate = (uint8_t)predicate;
    f.object = (uint32_t)object;
    f.confidence = (double)confidence;
    f.evidence = (uint8_t)evidence;
    f.timestamp = (int64_t)timestamp;
    f.context = (uint8_t)context;
    f.version = (int32_t)version;
    f.priority = (int8_t)priority;
    f.owner = (uint16_t)owner;
    return f;
}

static jobject fact_to_java(JNIEnv *env, const KCM_Fact *fact) {
    jclass cls = (*env)->FindClass(env, "io/kcm/Fact");
    if (cls == NULL) return NULL;
    jmethodID mid = (*env)->GetMethodID(env, cls, "<init>", "(IBIDBJBIBS)V");
    if (mid == NULL) return NULL;
    return (*env)->NewObject(env, cls, mid,
        (jint)fact->subject, (jbyte)fact->predicate, (jint)fact->object,
        (jdouble)fact->confidence, (jbyte)fact->evidence, (jlong)fact->timestamp,
        (jbyte)fact->context, (jint)fact->version, (jbyte)fact->priority, (jshort)fact->owner);
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseNew)(JNIEnv *env, jclass cls, jlongArray dbOut) {
    KCM_Database *db = NULL;
    KCM_Error err = KCM_DatabaseNew(&db);
    if (err == KCM_OK && db != NULL) {
        (*env)->SetLongArrayRegion(env, dbOut, 0, 1, (const jlong *)&db);
    }
    return (jint)err;
}

JNIEXPORT void JNICALL JNI_FN(nativeDatabaseFree)(JNIEnv *env, jclass cls, jlong db) {
    if (db != 0) {
        KCM_DatabaseFree((KCM_Database *)db);
    }
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseInsert)(JNIEnv *env, jclass cls, jlong db,
    jint subject, jbyte predicate, jint object, jdouble confidence,
    jbyte evidence, jlong timestamp, jbyte context,
    jint version, jbyte priority, jshort owner) {
    if (db == 0) return KCM_ERR_INVALID_ARGUMENT;
    KCM_Fact fact = make_fact(env, subject, predicate, object, confidence,
                              evidence, timestamp, context, version, priority, owner);
    return (jint)KCM_DatabaseInsert((KCM_Database *)db, &fact);
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseUpdate)(JNIEnv *env, jclass cls, jlong db, jlong rowId,
    jint subject, jbyte predicate, jint object, jdouble confidence,
    jbyte evidence, jlong timestamp, jbyte context,
    jint version, jbyte priority, jshort owner) {
    if (db == 0) return KCM_ERR_INVALID_ARGUMENT;
    KCM_Fact fact = make_fact(env, subject, predicate, object, confidence,
                              evidence, timestamp, context, version, priority, owner);
    return (jint)KCM_DatabaseUpdate((KCM_Database *)db, (uint64_t)rowId, &fact);
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseDelete)(JNIEnv *env, jclass cls, jlong db, jlong rowId) {
    if (db == 0) return KCM_ERR_INVALID_ARGUMENT;
    return (jint)KCM_DatabaseDelete((KCM_Database *)db, (uint64_t)rowId);
}

JNIEXPORT jlong JNICALL JNI_FN(nativeDatabaseFactCount)(JNIEnv *env, jclass cls, jlong db) {
    if (db == 0) return 0;
    return (jlong)KCM_DatabaseFactCount((KCM_Database *)db);
}

JNIEXPORT jlong JNICALL JNI_FN(nativeDatabaseActiveCount)(JNIEnv *env, jclass cls, jlong db) {
    if (db == 0) return 0;
    return (jlong)KCM_DatabaseActiveCount((KCM_Database *)db);
}

JNIEXPORT jlong JNICALL JNI_FN(nativeDatabaseQuery)(JNIEnv *env, jclass cls, jlong db, jstring query) {
    if (db == 0 || query == NULL) return 0;
    const char *qstr = (*env)->GetStringUTFChars(env, query, NULL);
    if (qstr == NULL) return 0;
    KCM_Query *q = KCM_DatabaseQuery((KCM_Database *)db, qstr);
    (*env)->ReleaseStringUTFChars(env, query, qstr);
    return (jlong)q;
}

JNIEXPORT jobject JNICALL JNI_FN(nativeQueryNext)(JNIEnv *env, jclass cls, jlong query) {
    if (query == 0) return NULL;
    KCM_Fact *fact = KCM_QueryNext((KCM_Query *)query);
    if (fact == NULL) return NULL;
    return fact_to_java(env, fact);
}

JNIEXPORT void JNICALL JNI_FN(nativeQueryFree)(JNIEnv *env, jclass cls, jlong query) {
    if (query != 0) {
        KCM_QueryFree((KCM_Query *)query);
    }
}

JNIEXPORT jlong JNICALL JNI_FN(nativeDatabaseBeginTransaction)(JNIEnv *env, jclass cls, jlong db) {
    if (db == 0) return 0;
    return (jlong)KCM_DatabaseBeginTransaction((KCM_Database *)db);
}

JNIEXPORT jint JNICALL JNI_FN(nativeTransactionCommit)(JNIEnv *env, jclass cls, jlong txn) {
    if (txn == 0) return KCM_ERR_INVALID_ARGUMENT;
    return (jint)KCM_TransactionCommit((KCM_Transaction *)txn);
}

JNIEXPORT jint JNICALL JNI_FN(nativeTransactionRollback)(JNIEnv *env, jclass cls, jlong txn) {
    if (txn == 0) return KCM_ERR_INVALID_ARGUMENT;
    return (jint)KCM_TransactionRollback((KCM_Transaction *)txn);
}

JNIEXPORT void JNICALL JNI_FN(nativeTransactionFree)(JNIEnv *env, jclass cls, jlong txn) {
    if (txn != 0) {
        KCM_TransactionFree((KCM_Transaction *)txn);
    }
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseSave)(JNIEnv *env, jclass cls, jlong db, jstring path) {
    if (db == 0 || path == NULL) return KCM_ERR_INVALID_ARGUMENT;
    const char *pstr = (*env)->GetStringUTFChars(env, path, NULL);
    if (pstr == NULL) return KCM_ERR_OUT_OF_MEMORY;
    KCM_Error err = KCM_DatabaseSave((KCM_Database *)db, pstr);
    (*env)->ReleaseStringUTFChars(env, path, pstr);
    return (jint)err;
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseLoad)(JNIEnv *env, jclass cls, jlong db, jstring path) {
    if (db == 0 || path == NULL) return KCM_ERR_INVALID_ARGUMENT;
    const char *pstr = (*env)->GetStringUTFChars(env, path, NULL);
    if (pstr == NULL) return KCM_ERR_OUT_OF_MEMORY;
    KCM_Error err = KCM_DatabaseLoad((KCM_Database *)db, pstr);
    (*env)->ReleaseStringUTFChars(env, path, pstr);
    return (jint)err;
}

JNIEXPORT jint JNICALL JNI_FN(nativeDatabaseVerify)(JNIEnv *env, jclass cls, jstring path) {
    if (path == NULL) return KCM_ERR_INVALID_ARGUMENT;
    const char *pstr = (*env)->GetStringUTFChars(env, path, NULL);
    if (pstr == NULL) return KCM_ERR_OUT_OF_MEMORY;
    KCM_Error err = KCM_DatabaseVerify(pstr);
    (*env)->ReleaseStringUTFChars(env, path, pstr);
    return (jint)err;
}

JNIEXPORT jstring JNICALL JNI_FN(nativeErrorMessage)(JNIEnv *env, jclass cls, jint err) {
    const char *msg = KCM_ErrorMessage((KCM_Error)err);
    if (msg == NULL) return (*env)->NewStringUTF(env, "Unknown error");
    return (*env)->NewStringUTF(env, msg);
}
