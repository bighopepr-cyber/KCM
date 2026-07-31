use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;

pub const PRED_HAS_CATEGORY: u8 = 0;
pub const PRED_IN_STOCK: u8 = 1;
pub const PRED_USER_VIEWED: u8 = 2;
pub const PRED_USER_PURCHASED: u8 = 3;
pub const PRED_SIMILAR_TO: u8 = 4;

pub fn build_engine() -> Result<KnowledgeDatabase, KcmError> {
    let kb = KnowledgeDatabase::new()?;

    for product_id in 0u32..1000 {
        let fact = Fact::new(
            SubjectID(product_id),
            PredicateID(PRED_HAS_CATEGORY),
            ObjectID(product_id % 10),
            1.0,
        )?;
        kb.insert(&fact)?;
    }

    for user_id in 0u32..100 {
        for product in (0u32..1000).step_by(50) {
            let fact = Fact::new(
                SubjectID(100_000 + user_id),
                PredicateID(PRED_USER_PURCHASED),
                ObjectID(product),
                (0.5 + user_id as f64 * 0.001).min(1.0),
            )?;
            kb.insert(&fact)?;
        }
    }

    Ok(kb)
}

pub fn find_purchases(kb: &KnowledgeDatabase, user_id: u32) -> Result<Vec<Fact>, KcmError> {
    kb.query()
        .with_subject(SubjectID(100_000 + user_id))
        .with_predicate(PredicateID(PRED_USER_PURCHASED))
        .execute()
}
