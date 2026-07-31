use kcm_core::types::*;
use kcm_runtime::database::KnowledgeDatabase;
use std::sync::Arc;

pub mod knowledge_service {
    tonic::include_proto!("kcm");
}

use knowledge_service::knowledge_service_server::KnowledgeService;
use knowledge_service::*;

pub struct KcmGrpcService {
    pub db: Arc<KnowledgeDatabase>,
}

#[tonic::async_trait]
impl KnowledgeService for KcmGrpcService {
    async fn insert_fact(
        &self,
        request: tonic::Request<InsertFactRequest>,
    ) -> Result<tonic::Response<InsertFactResponse>, tonic::Status> {
        let req = request.into_inner();
        let fact = Fact::new(
            SubjectID(req.subject),
            PredicateID(req.predicate as u8),
            ObjectID(req.object),
            req.confidence,
        )
        .map_err(tonic::Status::invalid_argument)?;

        match self.db.insert(&fact) {
            Ok(row_id) => Ok(tonic::Response::new(InsertFactResponse {
                row_id: row_id.0,
                status: "OK".to_string(),
            })),
            Err(e) => Err(tonic::Status::internal(e.to_string())),
        }
    }

    async fn query_facts(
        &self,
        request: tonic::Request<QueryRequest>,
    ) -> Result<tonic::Response<QueryResponse>, tonic::Status> {
        let req = request.into_inner();
        let mut query = self.db.query();

        if let Some(s) = req.subject {
            query = query.with_subject(SubjectID(s));
        }
        if let Some(p) = req.predicate {
            query = query.with_predicate(PredicateID(p as u8));
        }
        if let Some(o) = req.object {
            query = query.with_object(ObjectID(o));
        }
        if let Some(c) = req.confidence_min {
            query = query.with_confidence(c);
        }

        match query.execute() {
            Ok(facts) => {
                let total = facts.len() as u32;
                let fact_data: Vec<FactData> = facts
                    .iter()
                    .map(|f| FactData {
                        subject: f.subject.0,
                        predicate: f.predicate.0 as u32,
                        object: f.object.0,
                        confidence: f.confidence,
                        timestamp: f.timestamp,
                        context: f.context.0 as u32,
                    })
                    .collect();
                Ok(tonic::Response::new(QueryResponse {
                    facts: fact_data,
                    total_count: total,
                }))
            }
            Err(e) => Err(tonic::Status::internal(e.to_string())),
        }
    }

    async fn get_fact(
        &self,
        request: tonic::Request<GetFactRequest>,
    ) -> Result<tonic::Response<FactData>, tonic::Status> {
        let req = request.into_inner();
        match self.db.get_fact(RowID(req.row_id)) {
            Ok(Some(fact)) => Ok(tonic::Response::new(FactData {
                subject: fact.subject.0,
                predicate: fact.predicate.0 as u32,
                object: fact.object.0,
                confidence: fact.confidence,
                timestamp: fact.timestamp,
                context: fact.context.0 as u32,
            })),
            Ok(None) => Err(tonic::Status::not_found("Fact not found")),
            Err(e) => Err(tonic::Status::internal(e.to_string())),
        }
    }

    async fn get_stats(
        &self,
        _request: tonic::Request<GetStatsRequest>,
    ) -> Result<tonic::Response<StatsResponse>, tonic::Status> {
        let fact_count = self.db.fact_count() as u64;
        let avg_confidence = if fact_count > 0 {
            let mut total = 0.0;
            let mut count = 0u64;
            for i in 0..fact_count as usize {
                if let Ok(Some(f)) = self.db.get_fact(RowID(i as u64)) {
                    total += f.confidence;
                    count += 1;
                }
            }
            if count > 0 {
                total / count as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        Ok(tonic::Response::new(StatsResponse {
            fact_count,
            memory_bytes: fact_count * 34,
            avg_confidence,
        }))
    }
}
