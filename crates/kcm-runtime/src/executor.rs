use kcm_core::types::*;
use rayon::ThreadPool;

pub struct Executor {
    thread_pool: ThreadPool,
}

impl Executor {
    pub fn new(num_threads: usize) -> Result<Self, KcmError> {
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads)
            .build()
            .map_err(|e| KcmError::Io(format!("Failed to build thread pool: {}", e)))?;

        Ok(Executor { thread_pool })
    }

    pub fn with_num_cpus() -> Result<Self, KcmError> {
        let num_cpus = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);
        Self::new(num_cpus)
    }

    pub fn num_threads(&self) -> usize {
        self.thread_pool.current_num_threads()
    }

    pub fn parallel_map<T, F, R>(&self, items: Vec<T>, f: F) -> Vec<R>
    where
        T: Send,
        F: Fn(T) -> R + Send + Sync,
        R: Send,
    {
        self.thread_pool.install(|| {
            use rayon::prelude::*;
            items.into_par_iter().map(f).collect()
        })
    }

    pub fn parallel_filter<T, F>(&self, items: Vec<T>, f: F) -> Vec<T>
    where
        T: Send,
        F: Fn(&T) -> bool + Send + Sync,
    {
        self.thread_pool.install(|| {
            use rayon::prelude::*;
            items.into_par_iter().filter(f).collect()
        })
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::with_num_cpus().unwrap()
    }
}
