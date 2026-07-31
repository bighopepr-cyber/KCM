#[cfg(feature = "python")]
pub mod bindings {
    use kcm_core::types::*;
    use kcm_runtime::database::KnowledgeDatabase;
    use pyo3::prelude::*;
    use std::sync::{Arc, Mutex};

    #[pyclass]
    pub struct PyKnowledgeBase {
        kb: Arc<Mutex<KnowledgeDatabase>>,
    }

    #[pymethods]
    impl PyKnowledgeBase {
        #[new]
        fn new() -> PyResult<Self> {
            let kb = KnowledgeDatabase::new()
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
            Ok(PyKnowledgeBase {
                kb: Arc::new(Mutex::new(kb)),
            })
        }

        fn insert(
            &self,
            subject: u32,
            predicate: u8,
            object: u32,
            confidence: f64,
        ) -> PyResult<()> {
            let fact = Fact::new(
                SubjectID(subject),
                PredicateID(predicate),
                ObjectID(object),
                confidence,
            )
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))?;

            self.kb
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(&fact)
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
            Ok(())
        }

        fn query_all(&self) -> PyResult<Vec<(u32, u8, u32, f64)>> {
            let kb = self.kb.lock().unwrap_or_else(|e| e.into_inner());
            let facts = kb
                .query()
                .execute()
                .map_err(|e| pyo3::exceptions::PyException::new_err(e.to_string()))?;
            Ok(facts
                .iter()
                .map(|f| (f.subject.0, f.predicate.0, f.object.0, f.confidence))
                .collect())
        }

        fn fact_count(&self) -> usize {
            self.kb
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .fact_count()
        }
    }

    #[pymodule]
    pub fn kcm(_py: Python, m: &PyModule) -> PyResult<()> {
        m.add_class::<PyKnowledgeBase>()?;
        Ok(())
    }
}

#[cfg(not(feature = "python"))]
pub mod bindings {}
