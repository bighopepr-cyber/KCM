use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

pub const PRED_TREATS: u8 = 0;
pub const PRED_SIDE_EFFECT: u8 = 1;
pub const PRED_CONTRAINDICATION: u8 = 2;

pub const ENTITY_DRUG: u32 = 0;
pub const ENTITY_DISEASE: u32 = 100_000;
pub const ENTITY_SYMPTOM: u32 = 200_000;

pub fn build_medical_kb() -> Result<KnowledgeDatabase, KcmError> {
    let kb = KnowledgeDatabase::new()?;

    let aspirin = SubjectID(ENTITY_DRUG + 1);
    let heart_disease = ObjectID(ENTITY_DISEASE + 1);
    kb.insert(&Fact::new(
        aspirin,
        PredicateID(PRED_TREATS),
        heart_disease,
        0.95,
    )?)?;

    let nausea = ObjectID(ENTITY_SYMPTOM + 100);
    kb.insert(&Fact::new(
        aspirin,
        PredicateID(PRED_SIDE_EFFECT),
        nausea,
        0.30,
    )?)?;

    let ibuprofen = SubjectID(ENTITY_DRUG + 2);
    kb.insert(&Fact::new(
        ibuprofen,
        PredicateID(PRED_CONTRAINDICATION),
        ObjectID(ENTITY_DRUG + 1),
        0.90,
    )?)?;

    Ok(kb)
}

pub fn find_treatments(kb: &KnowledgeDatabase, disease_id: u32) -> Result<Vec<Fact>, KcmError> {
    kb.query()
        .with_object(ObjectID(ENTITY_DISEASE + disease_id))
        .with_predicate(PredicateID(PRED_TREATS))
        .with_confidence(0.7)
        .execute()
}

pub fn check_contraindications(
    kb: &KnowledgeDatabase,
    drug1: u32,
    drug2: u32,
) -> Result<Option<Fact>, KcmError> {
    let results = kb
        .query()
        .with_subject(SubjectID(ENTITY_DRUG + drug1))
        .with_object(ObjectID(ENTITY_DRUG + drug2))
        .with_predicate(PredicateID(PRED_CONTRAINDICATION))
        .execute()?;
    Ok(results.first().cloned())
}
