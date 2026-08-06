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
        pub fn new() -> PyResult<Self> {
            let kb = KnowledgeDatabase::new()
                .map_err(|_| pyo3::exceptions::PyException::new_err("Failed to create database"))?;
            Ok(PyKnowledgeBase {
                kb: Arc::new(Mutex::new(kb)),
            })
        }

        pub fn insert(
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
            .map_err(|_| pyo3::exceptions::PyValueError::new_err("Invalid fact parameters"))?;

            self.kb
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(&fact)
                .map_err(|e| match e {
                    KcmError::InvalidArgument(msg) => {
                        pyo3::exceptions::PyValueError::new_err(msg)
                    }
                    KcmError::NotFound(msg) => {
                        pyo3::exceptions::PyKeyError::new_err(msg)
                    }
                    _ => pyo3::exceptions::PyException::new_err("Internal error"),
                })?;
            Ok(())
        }

        pub fn query_all(&self) -> PyResult<Vec<(u32, u8, u32, f64)>> {
            let kb = self.kb.lock().unwrap_or_else(|e| e.into_inner());
            let facts = kb
                .query()
                .execute()
                .map_err(|_| pyo3::exceptions::PyException::new_err("Query execution failed"))?;
            Ok(facts
                .iter()
                .map(|f| (f.subject.0, f.predicate.0, f.object.0, f.confidence))
                .collect())
        }

        pub fn fact_count(&self) -> usize {
            self.kb
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .fact_count()
        }

        pub fn active_count(&self) -> usize {
            self.kb
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .active_fact_count()
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
